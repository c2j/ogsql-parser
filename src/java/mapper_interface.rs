use std::collections::HashMap;
use tree_sitter::Node;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MapperInterfaceInfo {
    pub fqn: String,
    pub methods: HashMap<String, MapperMethodInfo>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MapperMethodInfo {
    pub return_type: Option<String>,
    pub params: Vec<MethodParam>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MethodParam {
    pub name: String,
    pub java_type: String,
    pub param_annotation: Option<String>,
}

pub fn parse_mapper_interface(source: &str) -> Option<MapperInterfaceInfo> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_java::LANGUAGE.into()).ok()?;
    let tree = parser.parse(source, None)?;
    let root = tree.root_node();
    let package_name = extract_package_name(&root, source);
    let interface_node = find_interface(&root)?;
    let class_name = interface_node
        .child_by_field_name("name")
        .map(|n| source[n.byte_range()].to_string())?;
    let fqn = match &package_name {
        Some(pkg) => format!("{}.{}", pkg, class_name),
        None => class_name,
    };
    let methods = extract_methods(&interface_node, source);
    Some(MapperInterfaceInfo { fqn, methods })
}

fn extract_package_name(root: &Node, source: &str) -> Option<String> {
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "package_declaration" {
            let mut cursor2 = child.walk();
            for c in child.children(&mut cursor2) {
                if c.kind() == "scoped_identifier" || c.kind() == "identifier" {
                    return Some(source[c.byte_range()].to_string());
                }
            }
        }
    }
    None
}

fn find_interface<'a>(root: &'a Node) -> Option<Node<'a>> {
    if root.kind() == "interface_declaration" {
        return Some(*root);
    }
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "interface_declaration" {
            return Some(child);
        }
    }
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "class_declaration" || child.kind() == "program" {
            let mut inner_cursor = child.walk();
            for inner in child.children(&mut inner_cursor) {
                if inner.kind() == "interface_declaration" {
                    return Some(inner);
                }
            }
        }
    }
    None
}

fn extract_methods(interface: &Node, source: &str) -> HashMap<String, MapperMethodInfo> {
    let mut methods = HashMap::new();
    let mut cursor = interface.walk();
    for child in interface.children(&mut cursor) {
        if child.kind() == "interface_body" {
            let mut body_cursor = child.walk();
            for member in child.children(&mut body_cursor) {
                if member.kind() == "method_declaration" || member.kind() == "abstract_method_declaration" {
                    if let Some(info) = parse_method(&member, source) {
                        methods.insert(info.0, info.1);
                    }
                }
            }
        }
    }
    methods
}

fn parse_method(node: &Node, source: &str) -> Option<(String, MapperMethodInfo)> {
    let name_node = node.child_by_field_name("name")?;
    let method_name = source[name_node.byte_range()].to_string();
    let return_type = node.child_by_field_name("type")
        .map(|n| extract_type_name_simple(&n, source));
    let params = node.child_by_field_name("parameters")
        .map(|params_node| extract_params(&params_node, source))
        .unwrap_or_default();
    Some((method_name, MapperMethodInfo { return_type, params }))
}

fn extract_params(params_node: &Node, source: &str) -> Vec<MethodParam> {
    let mut params = Vec::new();
    let mut cursor = params_node.walk();
    for child in params_node.children(&mut cursor) {
        if child.kind() == "formal_parameter" {
            if let Some(param) = parse_formal_parameter(&child, source) {
                params.push(param);
            }
        }
    }
    params
}

fn parse_formal_parameter(node: &Node, source: &str) -> Option<MethodParam> {
    let mut java_type: Option<String> = None;
    let mut var_name: Option<String> = None;
    let mut param_annotation: Option<String> = None;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "modifiers" => {
                let mut ann_cursor = child.walk();
                for ann in child.children(&mut ann_cursor) {
                    if ann.kind() == "annotation" {
                        param_annotation = extract_param_annotation(&ann, source);
                    }
                }
            }
            "type_identifier" | "primitive_type" | "integral_type"
            | "floating_point_type" | "boolean_type" | "generic_type" | "array_type" => {
                java_type = Some(extract_type_name_simple(&child, source));
            }
            "identifier" => {
                var_name = Some(source[child.byte_range()].to_string());
            }
            "annotation" => {
                param_annotation = extract_param_annotation(&child, source);
            }
            _ => {}
        }
    }
    let java_type = java_type?;
    let var_name = var_name?;
    let name = param_annotation.clone().unwrap_or_else(|| var_name.clone());
    Some(MethodParam { name, java_type, param_annotation })
}

fn extract_param_annotation(node: &Node, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
            let ann_name = source[child.byte_range()].to_string();
            if ann_name != "Param" { return None; }
        }
        if child.kind() == "annotation_argument_list" || child.kind() == "argument_list" {
            let mut arg_cursor = child.walk();
            for arg in child.children(&mut arg_cursor) {
                if arg.kind() == "string_literal" {
                    let raw = source[arg.byte_range()].to_string();
                    return Some(raw.trim_matches('"').to_string());
                }
            }
        }
    }
    None
}

fn extract_type_name_simple(node: &Node, source: &str) -> String {
    match node.kind() {
        "type_identifier" | "primitive_type" | "integral_type"
        | "floating_point_type" | "boolean_type" => source[node.byte_range()].to_string(),
        "generic_type" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "type_identifier" {
                    return source[child.byte_range()].to_string();
                }
            }
            source[node.byte_range()].to_string()
        }
        "array_type" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if matches!(child.kind(),
                    "type_identifier" | "primitive_type" | "integral_type"
                    | "floating_point_type" | "boolean_type"
                ) {
                    return source[child.byte_range()].to_string();
                }
            }
            source[node.byte_range()].to_string()
        }
        _ => source[node.byte_range()].to_string(),
    }
}
