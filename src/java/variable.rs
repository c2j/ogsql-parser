//! Variable declaration, StringBuilder, and cross-statement concatenation tracking.

use tree_sitter::Node;

use super::constant::{SQL_NAME_PATTERN, SQL_STATEMENT_PREFIXES, STRING_BUILDER_TYPES};
use super::extract::{ExtractContext, TrackedVar};
use super::heuristics::{convert_placeholders, detect_parameter_style, detect_sql_kind_from_content, looks_like_sql};
use crate::java::types::*;

impl<'a> ExtractContext<'a> {
    pub(super) fn visit_field_declaration(&mut self, node: Node) {
        self.check_string_declaration(node);
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.visit(child);
        }
    }

    pub(super) fn visit_local_variable_declaration(&mut self, node: Node) {
        let type_name = self.detect_local_var_type(node);
        let is_sb = type_name
            .as_ref()
            .map_or(false, |t| STRING_BUILDER_TYPES.contains(&t.as_str()));

        if is_sb {
            self.check_string_builder_declaration(node);
        } else {
            self.check_string_declaration(node);
        }

        if let Some(t) = type_name {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "variable_declarator" {
                    if let Some(name_node) = child.child_by_field_name("name") {
                        self.var_types.insert(self.node_text(name_node), t.clone());
                    }
                }
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.visit(child);
        }
    }

    pub(super) fn detect_local_var_type(&self, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "type_identifier"
                | "primitive_type"
                | "integral_type"
                | "floating_point_type"
                | "boolean_type"
                | "generic_type"
                | "array_type" => {
                    return self.extract_type_name(child);
                }
                _ => {}
            }
        }
        None
    }

    pub(super) fn check_string_builder_declaration(&mut self, node: Node) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "variable_declarator" {
                self.check_sb_declarator(child);
            }
        }
    }

    pub(super) fn check_sb_declarator(&mut self, declarator: Node) {
        let name_node = match declarator.child_by_field_name("name") {
            Some(n) => n,
            None => return,
        };
        let var_name = self.node_text(name_node);

        let init_sql = match declarator.child_by_field_name("value") {
            Some(value) => {
                if value.kind() == "object_creation_expression" {
                    if let Some(args) = value.child_by_field_name("arguments") {
                        if let Some((sql_text, _)) = self.find_first_string_arg(&args) {
                            Some(sql_text)
                        } else {
                            Some(String::new())
                        }
                    } else {
                        Some(String::new())
                    }
                } else {
                    return;
                }
            }
            None => return,
        };

        let sql_text = match init_sql {
            Some(s) => s,
            None => return,
        };

        let var_name_upper = var_name.to_uppercase();
        let name_hints_sql = var_name_upper.contains(SQL_NAME_PATTERN);
        let content_hints_sql = looks_like_sql(&sql_text);

        if sql_text.is_empty() {
            if !name_hints_sql {
                return;
            }
        } else if !name_hints_sql && !content_hints_sql {
            return;
        }

        let sql_kind = detect_sql_kind_from_content(&sql_text);
        let param_style = detect_parameter_style(&sql_text);
        let sql_converted = convert_placeholders(&sql_text);
        let parse_result = if sql_converted.trim().is_empty() {
            None
        } else {
            self.try_parse_sql(&sql_converted, sql_kind)
        };

        self.extractions.push(ExtractedSql {
            sql: sql_converted.clone(),
            origin: SqlOrigin {
                method: ExtractionMethod::Constant,
                class_name: self.class_name.clone(),
                method_name: self.method_name.clone(),
                annotation_name: None,
                api_method_name: None,
                variable_name: Some(var_name.clone()),
                line: declarator.start_position().row + 1,
                column: declarator.start_position().column,
            },
            sql_kind,
            parameter_style: param_style,
            is_concatenated: false,
            is_text_block: false,
            parse_result,
        });

        self.sql_vars.insert(
            var_name,
            TrackedVar {
                sql: sql_converted,
                extraction_index: self.extractions.len() - 1,
                is_string_builder: true,
            },
        );
    }

    pub(super) fn check_string_declaration(&mut self, node: Node) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "variable_declarator" {
                self.check_declarator(child);
            }
        }
    }

    pub(super) fn check_declarator(&mut self, declarator: Node) {
        let name_node = match declarator.child_by_field_name("name") {
            Some(n) => n,
            None => return,
        };
        let value_node = match declarator.child_by_field_name("value") {
            Some(n) => n,
            None => return,
        };

        let var_name = self.node_text(name_node);
        let (sql_text, is_text_block) = match self.extract_string_value(value_node) {
            Some(v) => v,
            None => return,
        };

        let var_name_upper = var_name.to_uppercase();
        let name_looks_like_sql = var_name_upper.contains(SQL_NAME_PATTERN);

        let content_looks_like_sql = looks_like_sql(&sql_text);

        if !name_looks_like_sql && !content_looks_like_sql {
            return;
        }

        if name_looks_like_sql && !content_looks_like_sql {
            let trimmed = sql_text.trim();
            let starts_like_sql = SQL_STATEMENT_PREFIXES
                .iter()
                .any(|prefix| trimmed.starts_with(prefix));
            if !starts_like_sql {
                return;
            }
        }

        let sql_kind = detect_sql_kind_from_content(&sql_text);
        let param_style = detect_parameter_style(&sql_text);
        let sql_converted = convert_placeholders(&sql_text);
        let is_concatenated = value_node.kind() == "binary_expression";
        let parse_result = self.try_parse_sql(&sql_converted, sql_kind);

        self.extractions.push(ExtractedSql {
            sql: sql_converted.clone(),
            origin: SqlOrigin {
                method: ExtractionMethod::Constant,
                class_name: self.class_name.clone(),
                method_name: self.method_name.clone(),
                annotation_name: None,
                api_method_name: None,
                variable_name: Some(var_name.clone()),
                line: declarator.start_position().row + 1,
                column: declarator.start_position().column,
            },
            sql_kind,
            parameter_style: param_style,
            is_concatenated,
            is_text_block,
            parse_result,
        });

        self.sql_vars.insert(
            var_name,
            TrackedVar {
                sql: sql_converted,
                extraction_index: self.extractions.len() - 1,
                is_string_builder: false,
            },
        );
    }

    pub(super) fn find_method_chain_root(&self, node: Node) -> Option<String> {
        let mut current = node;
        loop {
            let object = match current.child_by_field_name("object") {
                Some(n) => n,
                None => return None,
            };
            match object.kind() {
                "identifier" => return Some(self.node_text(object)),
                "method_invocation" => current = object,
                _ => return None,
            }
        }
    }

    pub(super) fn handle_string_builder_call(&mut self, node: Node, root_var: &str, method_name: &str) {
        match method_name {
            "append" => self.handle_sb_append(node, root_var),
            "insert" => self.handle_sb_insert(node, root_var),
            "delete" => self.handle_sb_delete(node, root_var),
            _ => {}
        }
    }

    pub(super) fn handle_sb_append(&mut self, node: Node, root_var: &str) {
        let object = match node.child_by_field_name("object") {
            Some(n) => n,
            None => return,
        };
        if object.kind() == "method_invocation" {
            let inner_name = match object.child_by_field_name("name") {
                Some(n) => n,
                None => return,
            };
            if self.node_text(inner_name) == "append" {
                self.handle_sb_append(object, root_var);
            }
        }

        let args_node = match node.child_by_field_name("arguments") {
            Some(n) if n.kind() == "argument_list" => n,
            _ => return,
        };

        if let Some(text) = self.extract_single_arg_value(&args_node) {
            let tracked = match self.sql_vars.get_mut(root_var) {
                Some(t) => t,
                None => return,
            };
            tracked.sql.push_str(&text);
            let idx = tracked.extraction_index;
            let new_sql = tracked.sql.clone();

            let sql_kind = self.extractions[idx].sql_kind;
            let parse_result = self.try_parse_sql(&new_sql, sql_kind);

            let ext = match self.extractions.get_mut(idx) {
                Some(e) => e,
                None => return,
            };
            ext.sql = new_sql;
            ext.is_concatenated = true;
            ext.origin.line = node.start_position().row + 1;
            ext.origin.column = node.start_position().column;
            ext.parse_result = parse_result;
        }
    }

    pub(super) fn extract_single_arg_value(&self, args_node: &Node) -> Option<String> {
        let mut cursor = args_node.walk();
        for child in args_node.children(&mut cursor) {
            let kind = child.kind();
            if kind == "," || kind == "(" || kind == ")" {
                continue;
            }
            match kind {
                "string_literal" => {
                    let raw = self.node_text(child);
                    let is_tb = raw.starts_with("\"\"\"");
                    return Some(self.decode_java_string(&raw, is_tb));
                }
                _ => {
                    let var_name = self.node_text(child);
                    return Some(self.make_var_placeholder(&var_name));
                }
            }
        }
        None
    }

    pub(super) fn handle_sb_insert(&mut self, node: Node, root_var: &str) {
        let args_node = match node.child_by_field_name("arguments") {
            Some(n) if n.kind() == "argument_list" => n,
            _ => return,
        };

        let mut offset: Option<usize> = None;
        let mut value: Option<String> = None;
        let mut cursor = args_node.walk();
        for child in args_node.children(&mut cursor) {
            if child.kind() == "," {
                continue;
            }
            if offset.is_none() {
                let text = self.node_text(child);
                offset = self.parse_java_int(&text);
            } else if value.is_none() {
                if child.kind() == "string_literal" {
                    let raw = self.node_text(child);
                    let is_tb = raw.starts_with("\"\"\"");
                    value = Some(self.decode_java_string(&raw, is_tb));
                } else {
                    return;
                }
            }
        }

        if let (Some(off), Some(val)) = (offset, value) {
            let tracked = match self.sql_vars.get_mut(root_var) {
                Some(t) => t,
                None => return,
            };
            let byte_off = tracked
                .sql
                .char_indices()
                .nth(off)
                .map(|(i, _)| i)
                .unwrap_or(tracked.sql.len());
            tracked.sql.insert_str(byte_off, &val);

            let idx = tracked.extraction_index;
            let new_sql = tracked.sql.clone();
            let sql_kind = self.extractions[idx].sql_kind;
            let parse_result = self.try_parse_sql(&new_sql, sql_kind);

            let ext = match self.extractions.get_mut(idx) {
                Some(e) => e,
                None => return,
            };
            ext.sql = new_sql;
            ext.is_concatenated = true;
            ext.origin.line = node.start_position().row + 1;
            ext.origin.column = node.start_position().column;
            ext.parse_result = parse_result;
        }
    }

    pub(super) fn handle_sb_delete(&mut self, node: Node, root_var: &str) {
        let args_node = match node.child_by_field_name("arguments") {
            Some(n) if n.kind() == "argument_list" => n,
            _ => return,
        };

        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;
        let mut cursor = args_node.walk();
        for child in args_node.children(&mut cursor) {
            if child.kind() == "," {
                continue;
            }
            let text = self.node_text(child);
            if start.is_none() {
                start = self.parse_java_int(&text);
            } else if end.is_none() {
                end = self.parse_java_int(&text);
            }
        }

        if let (Some(s), Some(e)) = (start, end) {
            let tracked = match self.sql_vars.get_mut(root_var) {
                Some(t) => t,
                None => return,
            };

            let chars: Vec<char> = tracked.sql.chars().collect();
            if s < chars.len() && e <= chars.len() && s < e {
                let new_sql: String = chars[..s].iter().chain(chars[e..].iter()).collect();
                tracked.sql = new_sql;

                let idx = tracked.extraction_index;
                let new_sql = tracked.sql.clone();
                let sql_kind = self.extractions[idx].sql_kind;
                let parse_result = self.try_parse_sql(&new_sql, sql_kind);

                let ext = match self.extractions.get_mut(idx) {
                    Some(e) => e,
                    None => return,
                };
                ext.sql = new_sql;
                ext.is_concatenated = true;
                ext.origin.line = node.start_position().row + 1;
                ext.origin.column = node.start_position().column;
                ext.parse_result = parse_result;
            }
        }
    }

    pub(super) fn parse_java_int(&self, text: &str) -> Option<usize> {
        let s = text.trim();
        let s = s.strip_suffix('L').or_else(|| s.strip_suffix('l')).unwrap_or(s);
        s.parse().ok()
    }

    pub(super) fn visit_assignment_expression(&mut self, node: Node) {
        let left = match node.child_by_field_name("left") {
            Some(n) if n.kind() == "identifier" => n,
            _ => {
                self.recurse(node);
                return;
            }
        };
        let var_name = self.node_text(left);

        if !self.sql_vars.contains_key(&var_name) {
            self.recurse(node);
            return;
        }

        let operator = node
            .child_by_field_name("operator")
            .map(|n| self.node_text(n))
            .unwrap_or_default();

        let right = match node.child_by_field_name("right") {
            Some(n) => n,
            None => {
                self.recurse(node);
                return;
            }
        };

        match operator.as_str() {
            "+=" => {
                if let Some(parts) = self.extract_concat_string_parts(right) {
                    if !parts.is_empty() {
                        self.append_to_tracked_var(&var_name, &parts, node);
                    }
                }
            }
            "=" => {
                if right.kind() == "binary_expression"
                    && self.is_binary_left_identifier(right, &var_name)
                {
                    let parts = self.collect_concat_parts(right);
                    let append_parts: Vec<_> = parts.into_iter().skip(1).collect();
                    if !append_parts.is_empty() {
                        self.append_to_tracked_var(&var_name, &append_parts, node);
                    }
                } else {
                    self.reassign_tracked_var(&var_name, right, node);
                }
            }
            _ => {}
        }

        self.recurse(node);
    }

    pub(super) fn extract_concat_string_parts(&self, node: Node) -> Option<Vec<(String, bool)>> {
        match node.kind() {
            "string_literal" => {
                let raw = self.node_text(node);
                let is_tb = raw.starts_with("\"\"\"");
                Some(vec![(self.decode_java_string(&raw, is_tb), is_tb)])
            }
            "binary_expression" => {
                let parts = self.collect_concat_parts(node);
                if parts.is_empty() {
                    None
                } else {
                    Some(parts)
                }
            }
            _ => {
                let var_name = self.node_text(node);
                Some(vec![(self.make_var_placeholder(&var_name), false)])
            }
        }
    }

    pub(super) fn is_binary_left_identifier(&self, node: Node, var_name: &str) -> bool {
        let mut current = node;
        loop {
            let left = match current.child_by_field_name("left") {
                Some(n) => n,
                None => return false,
            };
            match left.kind() {
                "identifier" => return self.node_text(left) == var_name,
                "binary_expression" => current = left,
                _ => return false,
            }
        }
    }

    pub(super) fn append_to_tracked_var(&mut self, var_name: &str, parts: &[(String, bool)], node: Node) {
        let tracked = match self.sql_vars.get_mut(var_name) {
            Some(t) => t,
            None => return,
        };
        for (part, _) in parts {
            tracked.sql.push_str(part);
        }
        let idx = tracked.extraction_index;
        let new_sql = tracked.sql.clone();

        let sql_kind = self.extractions[idx].sql_kind;
        let parse_result = self.try_parse_sql(&new_sql, sql_kind);

        let ext = match self.extractions.get_mut(idx) {
            Some(e) => e,
            None => return,
        };
        ext.sql = new_sql;
        ext.is_concatenated = true;
        ext.origin.line = node.start_position().row + 1;
        ext.origin.column = node.start_position().column;
        ext.parse_result = parse_result;
    }

    pub(super) fn reassign_tracked_var(&mut self, var_name: &str, rhs: Node, node: Node) {
        let (sql_text, is_text_block) = match self.extract_string_value(rhs) {
            Some(v) => v,
            None => {
                self.sql_vars.remove(var_name);
                return;
            }
        };

        if !looks_like_sql(&sql_text) {
            self.sql_vars.remove(var_name);
            return;
        }

        let sql_kind = detect_sql_kind_from_content(&sql_text);
        let param_style = detect_parameter_style(&sql_text);
        let sql_converted = convert_placeholders(&sql_text);
        let is_concatenated = rhs.kind() == "binary_expression";
        let parse_result = self.try_parse_sql(&sql_converted, sql_kind);

        self.extractions.push(ExtractedSql {
            sql: sql_converted.clone(),
            origin: SqlOrigin {
                method: ExtractionMethod::Constant,
                class_name: self.class_name.clone(),
                method_name: self.method_name.clone(),
                annotation_name: None,
                api_method_name: None,
                variable_name: Some(var_name.to_string()),
                line: node.start_position().row + 1,
                column: node.start_position().column,
            },
            sql_kind,
            parameter_style: param_style,
            is_concatenated,
            is_text_block,
            parse_result,
        });

        let was_sb = self
            .sql_vars
            .get(var_name)
            .map_or(false, |v| v.is_string_builder);
        self.sql_vars.insert(
            var_name.to_string(),
            TrackedVar {
                sql: sql_converted,
                extraction_index: self.extractions.len() - 1,
                is_string_builder: was_sb,
            },
        );
    }
}
