//! Annotation-based SQL extraction for Java source files.

use tree_sitter::Node;

use super::constant::SQL_ANNOTATIONS;
use super::extract::ExtractContext;
use super::heuristics::{convert_placeholders, detect_parameter_style, NativeQueryFlag};
use crate::java::types::*;

impl<'a> ExtractContext<'a> {
    pub(super) fn visit_annotation(&mut self, node: Node) {
        let name_node = match node.child_by_field_name("name") {
            Some(n) => n,
            None => return,
        };
        let annotation_name = self.node_text(name_node);
        match annotation_name.as_str() {
            "NamedQueries" => self.visit_named_queries(node),
            name if SQL_ANNOTATIONS.contains(&name) => {
                self.visit_sql_annotation(node, &annotation_name);
            }
            _ => {}
        }
    }

    pub(super) fn visit_named_queries(&mut self, node: Node) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "annotation_argument_list" {
                let mut inner_cursor = child.walk();
                for inner_child in child.children(&mut inner_cursor) {
                    if inner_child.kind() == "annotation" {
                        self.visit_annotation(inner_child);
                    }
                }
            }
        }
    }

    pub(super) fn visit_sql_annotation(&mut self, node: Node, annotation_name: &str) {
        let args_node = match node.child_by_field_name("arguments") {
            Some(n) if n.kind() == "annotation_argument_list" => n,
            _ => return,
        };

        let native_flag = self.check_native_query_flag(&args_node);
        let sql_value = self.find_sql_value_in_annotation(&args_node, annotation_name);

        if let Some((sql_text, is_text_block)) = sql_value {
            let sql_kind = match native_flag {
                NativeQueryFlag::True => SqlKind::NativeSql,
                NativeQueryFlag::False => SqlKind::Jpql,
                NativeQueryFlag::NotPresent => match annotation_name {
                    "NamedQuery" | "Modifying" => SqlKind::Jpql,
                    "Query" => SqlKind::Jpql,
                    _ => SqlKind::NativeSql,
                },
            };

            let param_style = detect_parameter_style(&sql_text);
            let sql_converted = convert_placeholders(&sql_text);
            let parse_result = self.try_parse_sql(&sql_converted, sql_kind);

            self.extractions.push(ExtractedSql {
                sql: sql_converted,
                origin: SqlOrigin {
                    method: ExtractionMethod::Annotation,
                    class_name: self.class_name.clone(),
                    method_name: self.method_name.clone(),
                    annotation_name: Some(annotation_name.to_string()),
                    api_method_name: None,
                    variable_name: None,
                    line: node.start_position().row + 1,
                    column: node.start_position().column,
                },
                sql_kind,
                parameter_style: param_style,
                is_concatenated: false,
                is_text_block,
                parse_result,
            });
        }
    }

    pub(super) fn find_sql_value_in_annotation(
        &self,
        args_node: &Node,
        annotation_name: &str,
    ) -> Option<(String, bool)> {
        let target_key = match annotation_name {
            "NamedQuery" | "NamedNativeQuery" => "query",
            _ => "value",
        };

        let mut cursor = args_node.walk();
        for child in args_node.children(&mut cursor) {
            match child.kind() {
                "element_value_pair" => {
                    let key_node = child.child_by_field_name("key")?;
                    if self.node_text(key_node) == target_key {
                        let value_node = child.child_by_field_name("value")?;
                        return self.extract_string_value(value_node);
                    }
                }
                "string_literal" | "binary_expression" => {
                    return self.extract_string_value(child);
                }
                _ => {}
            }
        }

        if annotation_name == "Query" {
            let mut cursor = args_node.walk();
            for child in args_node.children(&mut cursor) {
                if child.kind() == "string_literal" {
                    return self.extract_string_value(child);
                }
            }
        }
        None
    }

    pub(super) fn check_native_query_flag(&self, args_node: &Node) -> NativeQueryFlag {
        let mut cursor = args_node.walk();
        for child in args_node.children(&mut cursor) {
            if child.kind() == "element_value_pair" {
                if let Some(key) = child.child_by_field_name("key") {
                    if self.node_text(key) == "nativeQuery" {
                        if let Some(value) = child.child_by_field_name("value") {
                            return if self.node_text(value) == "true" {
                                NativeQueryFlag::True
                            } else {
                                NativeQueryFlag::False
                            };
                        }
                    }
                }
            }
        }
        NativeQueryFlag::NotPresent
    }
}
