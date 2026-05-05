use std::collections::HashMap;
use tree_sitter::Node;

pub fn parse_dto_fields(source: &str) -> HashMap<String, String> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_java::LANGUAGE.into())
        .expect("Failed to set Java language");
    let tree = match parser.parse(source, None) {
        Some(t) => t,
        None => return HashMap::new(),
    };
    let mut fields = HashMap::new();
    collect_fields(tree.root_node(), source, &mut fields);
    fields
}

fn collect_fields(node: Node, source: &str, fields: &mut HashMap<String, String>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "class_declaration" {
            if let Some(body) = child.child_by_field_name("body") {
                let mut body_cursor = body.walk();
                for member in body.children(&mut body_cursor) {
                    if member.kind() == "field_declaration" {
                        parse_field(&member, source, fields);
                    }
                }
            }
        }
    }
}

fn parse_field(node: &Node, source: &str, fields: &mut HashMap<String, String>) {
    let mut type_name: Option<String> = None;
    let mut var_name: Option<String> = None;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "type_identifier" | "primitive_type" | "integral_type"
            | "floating_point_type" | "boolean_type" | "generic_type" | "array_type" => {
                type_name = Some(extract_type(&child, source));
            }
            "variable_declarator" => {
                if let Some(name_node) = child.child_by_field_name("name") {
                    var_name = Some(source[name_node.byte_range()].to_string());
                }
            }
            "variable_declarator_list" => {
                let mut dc = child.walk();
                for dc_child in child.children(&mut dc) {
                    if dc_child.kind() == "variable_declarator" {
                        if let Some(name_node) = dc_child.child_by_field_name("name") {
                            var_name = Some(source[name_node.byte_range()].to_string());
                        }
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    if let (Some(t), Some(n)) = (type_name, var_name) {
        fields.insert(n, t);
    }
}

fn extract_type(node: &Node, source: &str) -> String {
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
        _ => source[node.byte_range()].to_string(),
    }
}
