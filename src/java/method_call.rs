//! Method call SQL extraction for Java source files.

use tree_sitter::Node;

use super::constant::{SQL_METHOD_AMBIGUOUS, SQL_METHOD_UNAMBIGUOUS};
use super::extract::ExtractContext;
use super::heuristics::{detect_parameter_style, looks_like_sql};
use crate::java::types::*;

impl<'a> ExtractContext<'a> {
    pub(super) fn visit_method_invocation(&mut self, node: Node) {
        let name_node = match node.child_by_field_name("name") {
            Some(n) => n,
            None => return,
        };
        let method_name = self.node_text(name_node);

        if method_name == "append" || method_name == "insert" || method_name == "delete" {
            if let Some(root_var) = self.find_method_chain_root(node) {
                if self.sql_vars.contains_key(&root_var)
                    && self.sql_vars.get(&root_var).map_or(false, |v| v.is_string_builder)
                {
                    self.handle_string_builder_call(node, &root_var, &method_name);
                    return;
                }
            }
        }

        let is_unambiguous = SQL_METHOD_UNAMBIGUOUS.contains(&method_name.as_str())
            || self.extra_sql_methods.iter().any(|m| m == &method_name);
        let is_ambiguous = SQL_METHOD_AMBIGUOUS.contains(&method_name.as_str());

        if !is_unambiguous && !is_ambiguous {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                self.visit(child);
            }
            return;
        }

        let args_node = match node.child_by_field_name("arguments") {
            Some(n) if n.kind() == "argument_list" => n,
            _ => return,
        };

        if let Some((sql_text, is_text_block)) = self.find_first_string_arg(&args_node) {
            if !is_unambiguous && !looks_like_sql(&sql_text) {
                return;
            }

            let sql_kind = match method_name.as_str() {
                "createQuery" => SqlKind::Jpql,
                _ => SqlKind::NativeSql,
            };
            let param_style = detect_parameter_style(&sql_text);
            let sql_converted = self.convert_placeholders(&sql_text);
            let parse_result = self.try_parse_sql(&sql_converted, sql_kind);

            self.extractions.push(ExtractedSql {
                sql: sql_converted,
                origin: SqlOrigin {
                    method: ExtractionMethod::MethodCall,
                    class_name: self.class_name.clone(),
                    method_name: self.method_name.clone(),
                    annotation_name: None,
                    api_method_name: Some(method_name),
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

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.visit(child);
        }
    }

    pub(super) fn find_first_string_arg(&self, args_node: &Node) -> Option<(String, bool)> {
        let mut cursor = args_node.walk();
        for child in args_node.children(&mut cursor) {
            match child.kind() {
                "string_literal" | "binary_expression" => {
                    return self.extract_string_value(child);
                }
                _ => continue,
            }
        }
        None
    }
}
