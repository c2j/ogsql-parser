//! Java CST 遍历与 SQL 提取。

use std::collections::HashMap;

use tree_sitter::Node;

use super::types::JavaExtractConfig;
use crate::java::error::JavaError;
use crate::java::types::*;



pub(super) struct TrackedVar {
    pub(super) sql: String,
    pub(super) extraction_index: usize,
    pub(super) is_string_builder: bool,
}

pub fn extract(
    source: &str,
    root: Node,
    file_path: &str,
    config: &JavaExtractConfig,
) -> JavaExtractResult {
    let mut ctx = ExtractContext {
        source,
        file_path,
        extractions: Vec::new(),
        errors: Vec::new(),
        class_name: None,
        method_name: None,
        sql_vars: HashMap::new(),
        var_types: HashMap::new(),
        extra_sql_methods: &config.extra_sql_methods,
    };
    ctx.visit(root);
    JavaExtractResult {
        file_path: file_path.to_string(),
        extractions: ctx.extractions,
        errors: ctx.errors,
    }
}

pub(super) struct ExtractContext<'a> {
    pub(super) source: &'a str,
    pub(super) file_path: &'a str,
    pub(super) extractions: Vec<ExtractedSql>,
    pub(super) errors: Vec<JavaError>,
    pub(super) class_name: Option<String>,
    pub(super) method_name: Option<String>,
    pub(super) sql_vars: HashMap<String, TrackedVar>,
    pub(super) var_types: HashMap<String, String>,
    pub(super) extra_sql_methods: &'a [String],
}

impl<'a> ExtractContext<'a> {
    pub(super) fn visit(&mut self, node: Node) {
        match node.kind() {
            "class_declaration" | "interface_declaration" | "enum_declaration" => {
                self.visit_type_declaration(node);
            }
            "method_declaration" => {
                self.visit_method_declaration(node);
            }
            "annotation" => {
                self.visit_annotation(node);
            }
            "method_invocation" => {
                self.visit_method_invocation(node);
            }
            "field_declaration" => {
                self.visit_field_declaration(node);
            }
            "local_variable_declaration" => {
                self.visit_local_variable_declaration(node);
            }
            "assignment_expression" => {
                self.visit_assignment_expression(node);
            }
            _ => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.visit(child);
                }
            }
        }
    }

    pub(super) fn visit_type_declaration(&mut self, node: Node) {
        let old_class = self.class_name.clone();
        if let Some(name_node) = node.child_by_field_name("name") {
            self.class_name = Some(self.node_text(name_node));
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.visit(child);
        }
        self.class_name = old_class;
    }

    pub(super) fn visit_method_declaration(&mut self, node: Node) {
        let old_method = self.method_name.clone();
        let old_sql_vars = std::mem::take(&mut self.sql_vars);
        let old_var_types = std::mem::take(&mut self.var_types);
        if let Some(name_node) = node.child_by_field_name("name") {
            self.method_name = Some(self.node_text(name_node));
        }

        if let Some(params_node) = node.child_by_field_name("parameters") {
            let mut cursor = params_node.walk();
            for child in params_node.children(&mut cursor) {
                if child.kind() == "formal_parameter" {
                    let mut param_cursor = child.walk();
                    let mut type_name: Option<String> = None;
                    let mut var_name: Option<String> = None;
                    for pc in child.children(&mut param_cursor) {
                        match pc.kind() {
                            "type_identifier"
                            | "primitive_type"
                            | "generic_type"
                            | "integral_type"
                            | "floating_point_type"
                            | "boolean_type"
                            | "array_type" => {
                                type_name = self.extract_type_name(pc);
                            }
                            "identifier" => {
                                var_name = Some(self.node_text(pc));
                            }
                            _ => {}
                        }
                    }
                    if let (Some(t), Some(v)) = (type_name, var_name) {
                        self.var_types.insert(v, t);
                    }
                }
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "modifiers" {
                let mut mod_cursor = child.walk();
                for mod_child in child.children(&mut mod_cursor) {
                    if mod_child.kind() == "annotation" {
                        self.visit_annotation(mod_child);
                    }
                }
            }
        }

        if let Some(body) = node.child_by_field_name("body") {
            let mut cursor = body.walk();
            for child in body.children(&mut cursor) {
                self.visit(child);
            }
        }

        self.method_name = old_method;
        self.sql_vars = old_sql_vars;
        self.var_types = old_var_types;
    }

    pub(super) fn recurse(&mut self, node: Node) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.visit(child);
        }
    }

    pub(super) fn extract_string_value(&self, node: Node) -> Option<(String, bool)> {
        match node.kind() {
            "string_literal" => {
                let raw = self.node_text(node);
                let is_text_block = raw.starts_with("\"\"\"");
                let content = self.decode_java_string(&raw, is_text_block);
                Some((content, is_text_block))
            }
            "binary_expression" => {
                let parts = self.collect_concat_parts(node);
                if parts.is_empty() {
                    return None;
                }
                let combined: String = parts.into_iter().map(|(s, _)| s).collect();
                Some((combined, false))
            }
            _ => None,
        }
    }

    pub(super) fn collect_concat_parts(&self, node: Node) -> Vec<(String, bool)> {
        let op = node
            .child_by_field_name("operator")
            .map(|n| self.node_text(n));
        if op.as_deref() != Some("+") {
            return vec![];
        }

        let mut parts = Vec::new();

        if let Some(left) = node.child_by_field_name("left") {
            match left.kind() {
                "string_literal" => {
                    let raw = self.node_text(left);
                    let is_tb = raw.starts_with("\"\"\"");
                    parts.push((self.decode_java_string(&raw, is_tb), is_tb));
                }
                "binary_expression" => {
                    parts.extend(self.collect_concat_parts(left));
                }
                _ => {
                    parts.push((self.make_placeholder_for_node(left), false));
                }
            }
        }

        if let Some(right) = node.child_by_field_name("right") {
            match right.kind() {
                "string_literal" => {
                    let raw = self.node_text(right);
                    let is_tb = raw.starts_with("\"\"\"");
                    parts.push((self.decode_java_string(&raw, is_tb), is_tb));
                }
                "binary_expression" => {
                    parts.extend(self.collect_concat_parts(right));
                }
                _ => {
                    parts.push((self.make_placeholder_for_node(right), false));
                }
            }
        }

        parts
    }

    pub(super) fn try_parse_sql(&self, sql: &str, sql_kind: SqlKind) -> Option<SqlParseResult> {
        if sql_kind == SqlKind::Jpql {
            return None;
        }
        let flat_sql = sql.trim().to_string();
        if flat_sql.is_empty() {
            return None;
        }

        let (stmts, errors) = crate::parser::Parser::parse_sql(&flat_sql);
        Some(SqlParseResult {
            statements: stmts,
            errors,
        })
    }

    pub(super) fn extract_type_name(&self, node: Node) -> Option<String> {
        match node.kind() {
            "type_identifier"
            | "primitive_type"
            | "integral_type"
            | "floating_point_type"
            | "boolean_type" => Some(self.node_text(node)),
            "generic_type" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "type_identifier" {
                        return Some(self.node_text(child));
                    }
                }
                None
            }
            "array_type" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    let kind = child.kind();
                    if kind == "type_identifier"
                        || kind == "primitive_type"
                        || kind == "integral_type"
                        || kind == "floating_point_type"
                        || kind == "boolean_type"
                        || kind == "generic_type"
                    {
                        return self.extract_type_name(child);
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub(super) fn make_var_placeholder(&self, var_name: &str) -> String {
        let sanitized = sanitize_var_name(var_name);
        let type_name = self.var_types.get(var_name)
            .or_else(|| {
                let trimmed = var_name.trim();
                if trimmed != var_name { self.var_types.get(trimmed) } else { None }
            });
        match type_name {
            Some(t) => format!("__JAVA_VAR_{}_{}__", t, sanitized),
            None => format!("__JAVA_VAR_{}__", sanitized),
        }
    }

    pub(super) fn make_placeholder_for_node(&self, node: Node) -> String {
        let var_name = self.node_text(node);
        let sanitized = sanitize_var_name(&var_name);
        let inferred_type = self.infer_expression_type(node);
        match inferred_type {
            Some(t) => format!("__JAVA_VAR_{}_{}__", t, sanitized),
            None => match self.var_types.get(&var_name) {
                Some(t) => format!("__JAVA_VAR_{}_{}__", t, sanitized),
                None => format!("__JAVA_VAR_{}__", sanitized),
            },
        }
    }

    fn infer_expression_type(&self, node: Node) -> Option<String> {
        match node.kind() {
            "identifier" => self.var_types.get(&self.node_text(node)).cloned(),
            "parenthesized_expression" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() != "(" && child.kind() != ")" {
                        return self.infer_expression_type(child);
                    }
                }
                None
            }
            "binary_expression" => {
                let op = node.child_by_field_name("operator")
                    .map(|n| self.node_text(n));
                if op.as_deref() == Some("+") {
                    let left_type = node.child_by_field_name("left")
                        .and_then(|n| self.infer_expression_type(n));
                    let right_type = node.child_by_field_name("right")
                        .and_then(|n| self.infer_expression_type(n));
                    if left_type.as_deref() == Some("String") || right_type.as_deref() == Some("String") {
                        return Some("String".to_string());
                    }
                    return left_type.or(right_type);
                }
                node.child_by_field_name("left").and_then(|n| self.infer_expression_type(n))
            }
            "method_invocation" => {
                let name = node.child_by_field_name("name")
                    .map(|n| self.node_text(n));
                match name.as_deref() {
                    Some("toString") | Some("valueOf") => Some("String".to_string()),
                    Some("length") | Some("intValue") | Some("longValue") => Some("int".to_string()),
                    _ => None,
                }
            }
            "string_literal" => Some("String".to_string()),
            _ => None,
        }
    }

    pub(super) fn node_text(&self, node: Node) -> String {
        self.source[node.byte_range()].to_string()
    }
}

fn sanitize_var_name(var_name: &str) -> String {
    var_name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
