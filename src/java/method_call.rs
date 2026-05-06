//! Method call SQL extraction for Java source files.

use tree_sitter::Node;

use super::constant::{JDBC_SETTER_TYPES, SQL_METHOD_AMBIGUOUS, SQL_METHOD_UNAMBIGUOUS};
use super::extract::ExtractContext;
use super::heuristics::{detect_parameter_style, looks_like_sql};
use super::types::JdbcParamInfo;
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

        // Intercept JDBC setter calls (setString, setInt, etc.)
        if method_name.starts_with("set") && method_name.len() > 3 {
            let next_char = method_name.chars().nth(3).unwrap();
            if next_char.is_ascii_uppercase() {
                if let Some(root_var) = self.find_method_chain_root(node) {
                    if self.ps_var_to_extraction.contains_key(&root_var) {
                        self.visit_setter_call(node, &method_name);
                        return;
                    }
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

        let mut pushed_extraction_idx: Option<usize> = None;

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
                    api_method_name: Some(method_name.clone()),
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
            pushed_extraction_idx = Some(self.extractions.len() - 1);
        }

        // Track PreparedStatement variable → extraction mapping
        if method_name == "prepareStatement" || method_name == "prepareCall" {
            if let Some(target_var) = self.find_assignment_target(node) {
                let extraction_idx = match pushed_extraction_idx {
                    Some(idx) => Some(idx),
                    None => {
                        // First arg may be a tracked SQL variable; resolve its extraction index
                        let mut cursor = args_node.walk();
                        let mut found = None;
                        for child in args_node.children(&mut cursor) {
                            if child.kind() == "identifier" {
                                let arg_name = self.node_text(child);
                                if let Some(tracked) = self.sql_vars.get(&arg_name) {
                                    found = Some(tracked.extraction_index);
                                    break;
                                }
                            }
                        }
                        found
                    }
                };
                if let Some(idx) = extraction_idx {
                    self.ps_var_to_extraction.insert(target_var, idx);
                }
            }
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

    /// Handle `ps.setXxx(index, value)` calls to infer `?` placeholder types.
    pub(super) fn visit_setter_call(&mut self, node: Node, method_name: &str) {
        // Determine Java type from setter method name
        let java_type = match JDBC_SETTER_TYPES.iter().find(|(m, _)| *m == method_name) {
            Some((_, t)) => t.to_string(),
            None => {
                if method_name == "setObject" {
                    "Object".to_string()
                } else {
                    return;
                }
            }
        };

        let args_node = match node.child_by_field_name("arguments") {
            Some(n) if n.kind() == "argument_list" => n,
            _ => return,
        };

        // Extract arguments: first is index (int literal), second is value
        let mut args = Vec::new();
        {
            let mut cursor = args_node.walk();
            for child in args_node.children(&mut cursor) {
                if child.kind() != "," && child.kind() != "(" && child.kind() != ")" {
                    args.push(child);
                }
            }
        }

        if args.len() < 2 {
            return;
        }

        // Parse parameter index
        let idx_text = self.node_text(args[0]);
        let param_index: usize = match idx_text.trim().parse() {
            Ok(n) => n,
            Err(_) => return,
        };

        // Extract variable name from value argument
        let value_node = args[1];
        let var_name = self.extract_setter_value_name(value_node);

        // Find the PreparedStatement variable name
        let ps_var = match self.find_method_chain_root(node) {
            Some(v) => v,
            None => return,
        };

        // Only track if this PS variable is known
        if !self.ps_var_to_extraction.contains_key(&ps_var) {
            return;
        }

        self.jdbc_param_map.insert(
            (ps_var, param_index),
            JdbcParamInfo {
                index: param_index,
                java_type,
                var_name,
            },
        );
    }

    /// Extract a variable name from the setter value argument.
    fn extract_setter_value_name(&self, node: Node) -> Option<String> {
        match node.kind() {
            "identifier" => Some(self.node_text(node)),
            "field_access" => {
                let text = self.node_text(node);
                let parts: Vec<&str> = text.rsplitn(2, '.').collect();
                Some(parts[0].to_string())
            }
            "method_invocation" => {
                node.child_by_field_name("name").map(|n| self.node_text(n))
            }
            _ => None,
        }
    }
}
