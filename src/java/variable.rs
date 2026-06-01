//! Variable declaration, StringBuilder, and cross-statement concatenation tracking.

use tree_sitter::Node;

use super::constant::{SQL_STATEMENT_PREFIXES, STRING_BUILDER_TYPES};
use super::extract::{ExtractContext, TrackedVar};
use super::heuristics::{detect_parameter_style, detect_sql_kind_from_content, looks_like_sql};
use crate::java::types::*;

impl<'a> ExtractContext<'a> {
    pub(super) fn visit_field_declaration(&mut self, node: Node) {
        let type_name = self.detect_local_var_type(node);
        if let Some(ref ts) = type_name {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "variable_declarator" {
                    if let Some(name_node) = child.child_by_field_name("name") {
                        self.var_types.insert(self.node_text(name_node), ts.clone());
                    }
                }
            }
            if ts == "StringBuilder" || ts == "StringBuffer" {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "variable_declarator" {
                        self.check_sb_declarator(child, true);
                    }
                }
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.visit(child);
                }
                return;
            }
        }
        if type_name.as_deref() == Some("String") {
            self.track_string_constant(node);
        }
        self.check_string_declaration(node, true);
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.visit(child);
        }

        // Track Set.of() declarations in fields for cross-method evaluation
        if let Some(ref ts) = type_name {
            if ts == "Set" || ts.starts_with("Set<") {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "variable_declarator" {
                        if let Some(value_node) = child.child_by_field_name("value") {
                            let var_name =
                                child.child_by_field_name("name").map(|n| self.node_text(n)).unwrap_or_default();
                            self.try_extract_set_of(value_node, &var_name);
                        }
                    }
                }
            }
        }
    }

    pub(super) fn visit_local_variable_declaration(&mut self, node: Node) {
        let type_name = self.detect_local_var_type(node);
        let is_sb = type_name.as_ref().is_some_and(|t| STRING_BUILDER_TYPES.contains(&t.as_str()));

        if is_sb {
            self.check_string_builder_declaration(node);
        } else {
            self.check_string_declaration(node, false);
        }

        let is_string = type_name.as_deref() == Some("String");

        if let Some(ref t) = type_name {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "variable_declarator" {
                    if let Some(name_node) = child.child_by_field_name("name") {
                        self.var_types.insert(self.node_text(name_node), t.clone());
                    }
                }
            }
        }

        if is_string {
            self.track_local_string_constant(node);
        }

        // Track Set.of() and List declarations for cross-method evaluation
        if let Some(ref t) = type_name {
            let is_set = t == "Set" || t.starts_with("Set<");
            let is_list = matches!(t.as_str(), "List" | "ArrayList" | "Collection")
                || t.starts_with("List<")
                || t.starts_with("ArrayList<")
                || t.starts_with("Collection<");
            if is_set || is_list {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "variable_declarator" {
                        let var_name = child.child_by_field_name("name").map(|n| self.node_text(n)).unwrap_or_default();
                        if is_set {
                            if let Some(value_node) = child.child_by_field_name("value") {
                                self.try_extract_set_of(value_node, &var_name);
                            }
                        }
                        if is_list {
                            if let Some(value_node) = child.child_by_field_name("value") {
                                self.try_track_list_declaration(value_node, &var_name);
                            }
                        }
                    }
                }
            }
        }

        // Track boolean variable initial values for if-statement handling
        if type_name.as_deref() == Some("boolean") {
            let mut bc = node.walk();
            for child in node.children(&mut bc) {
                if child.kind() == "variable_declarator" {
                    let var_name = child.child_by_field_name("name").map(|n| self.node_text(n)).unwrap_or_default();
                    if let Some(value_node) = child.child_by_field_name("value") {
                        let text = self.node_text(value_node);
                        if text == "true" {
                            self.local_bool_vars.insert(var_name, true);
                        } else if text == "false" {
                            self.local_bool_vars.insert(var_name, false);
                        }
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
                | "array_type"
                | "scoped_type_identifier" => {
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
                self.check_sb_declarator(child, false);
            }
        }
    }

    pub(super) fn check_sb_declarator(&mut self, declarator: Node, is_field_level: bool) {
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
        let name_hints_sql = self.sql_var_patterns_upper.iter().any(|p| var_name_upper.contains(p.as_str()));
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
        let sql_converted = self.convert_placeholders(&sql_text);
        let parse_result =
            if sql_converted.trim().is_empty() { None } else { self.try_parse_sql(&sql_converted, sql_kind) };

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
                is_field_level,
            },
        );
    }

    pub(super) fn check_string_declaration(&mut self, node: Node, is_field_level: bool) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "variable_declarator" {
                self.check_declarator(child, is_field_level);
            }
        }
    }

    pub(super) fn check_declarator(&mut self, declarator: Node, is_field_level: bool) {
        let name_node = match declarator.child_by_field_name("name") {
            Some(n) => n,
            None => return,
        };
        let var_name = self.node_text(name_node);
        let value_node = declarator.child_by_field_name("value");

        if value_node.is_none() {
            let var_name_upper = var_name.to_uppercase();
            let name_hints_sql = self.sql_var_patterns_upper.iter().any(|p| var_name_upper.contains(p.as_str()));
            if name_hints_sql {
                self.sql_vars.insert(
                    var_name,
                    TrackedVar {
                        sql: String::new(),
                        extraction_index: usize::MAX,
                        is_string_builder: false,
                        is_field_level,
                    },
                );
            }
            return;
        }
        let value_node = value_node.unwrap();

        let (sql_text, is_text_block) = match self.extract_string_value(value_node) {
            Some(v) => v,
            None => {
                // Eagerly evaluate String method chains (String.join, nCopies, etc.)
                if value_node.kind() == "method_invocation" {
                    if let Some(evaluated) = self.try_evaluate_method_result(value_node) {
                        self.string_exprs.insert(var_name.clone(), evaluated);
                    }
                }
                return;
            }
        };

        let var_name_upper = var_name.to_uppercase();
        let name_looks_like_sql = self.sql_var_patterns_upper.iter().any(|p| var_name_upper.contains(p.as_str()));

        let content_looks_like_sql = looks_like_sql(&sql_text);

        if !name_looks_like_sql && !content_looks_like_sql {
            return;
        }

        if name_looks_like_sql && !content_looks_like_sql {
            let trimmed = sql_text.trim();
            let starts_like_sql = SQL_STATEMENT_PREFIXES.iter().any(|prefix| trimmed.starts_with(prefix));
            if !starts_like_sql {
                return;
            }
        }

        let sql_kind = detect_sql_kind_from_content(&sql_text);
        let param_style = detect_parameter_style(&sql_text);
        let sql_converted = self.convert_placeholders(&sql_text);
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
                is_field_level,
            },
        );
    }

    pub(super) fn find_method_chain_root(&self, node: Node) -> Option<String> {
        let mut current = node;
        loop {
            let object = current.child_by_field_name("object")?;
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
            "replace" => self.handle_sb_replace(node, root_var),
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
            let inner_method = self.node_text(inner_name);
            match inner_method.as_str() {
                "append" | "insert" | "delete" | "replace" => {
                    self.handle_string_builder_call(object, root_var, &inner_method);
                }
                _ => {}
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
            let converted_sql = self.convert_placeholders(&new_sql);

            let sql_kind = self.extractions[idx].sql_kind;
            let parse_result = self.try_parse_sql(&converted_sql, sql_kind);

            let ext = match self.extractions.get_mut(idx) {
                Some(e) => e,
                None => return,
            };
            ext.sql = converted_sql;
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
                    return Some(self.make_placeholder_for_node(child));
                }
            }
        }
        None
    }

    pub(super) fn handle_sb_insert(&mut self, node: Node, root_var: &str) {
        let object = match node.child_by_field_name("object") {
            Some(n) => n,
            None => return,
        };
        if object.kind() == "method_invocation" {
            let inner_name = match object.child_by_field_name("name") {
                Some(n) => n,
                None => return,
            };
            let inner_method = self.node_text(inner_name);
            match inner_method.as_str() {
                "append" | "insert" | "delete" | "replace" => {
                    self.handle_string_builder_call(object, root_var, &inner_method);
                }
                _ => {}
            }
        }

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
            let byte_off = tracked.sql.char_indices().nth(off).map(|(i, _)| i).unwrap_or(tracked.sql.len());
            tracked.sql.insert_str(byte_off, &val);

            let idx = tracked.extraction_index;
            let new_sql = tracked.sql.clone();
            let converted_sql = self.convert_placeholders(&new_sql);
            let sql_kind = self.extractions[idx].sql_kind;
            let parse_result = self.try_parse_sql(&converted_sql, sql_kind);

            let ext = match self.extractions.get_mut(idx) {
                Some(e) => e,
                None => return,
            };
            ext.sql = converted_sql;
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
                let converted_sql = self.convert_placeholders(&new_sql);
                let sql_kind = self.extractions[idx].sql_kind;
                let parse_result = self.try_parse_sql(&converted_sql, sql_kind);

                let ext = match self.extractions.get_mut(idx) {
                    Some(e) => e,
                    None => return,
                };
                ext.sql = converted_sql;
                ext.is_concatenated = true;
                ext.origin.line = node.start_position().row + 1;
                ext.origin.column = node.start_position().column;
                ext.parse_result = parse_result;
            }
        }
    }

    pub(super) fn handle_sb_replace(&mut self, node: Node, root_var: &str) {
        let args_node = match node.child_by_field_name("arguments") {
            Some(n) if n.kind() == "argument_list" => n,
            _ => return,
        };

        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;
        let mut value: Option<String> = None;
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
            } else if value.is_none() {
                if child.kind() == "string_literal" {
                    let raw = self.node_text(child);
                    let is_tb = raw.starts_with("\"\"\"");
                    value = Some(self.decode_java_string(&raw, is_tb));
                } else {
                    value = Some(self.make_placeholder_for_node(child));
                }
            }
        }

        if let (Some(s), Some(e), Some(val)) = (start, end, value) {
            let tracked = match self.sql_vars.get_mut(root_var) {
                Some(t) => t,
                None => return,
            };

            let chars: Vec<char> = tracked.sql.chars().collect();
            if s < chars.len() && e <= chars.len() && s <= e {
                let new_sql: String =
                    chars[..s].iter().copied().chain(val.chars()).chain(chars[e..].iter().copied()).collect();
                tracked.sql = new_sql;

                let idx = tracked.extraction_index;
                let new_sql = tracked.sql.clone();
                let converted_sql = self.convert_placeholders(&new_sql);
                let sql_kind = self.extractions[idx].sql_kind;
                let parse_result = self.try_parse_sql(&converted_sql, sql_kind);

                let ext = match self.extractions.get_mut(idx) {
                    Some(e) => e,
                    None => return,
                };
                ext.sql = converted_sql;
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

        // Track boolean variable values (e.g., `first = false`) for if-statement handling
        self.track_bool_assignment(&var_name, &node);

        if !self.sql_vars.contains_key(&var_name) {
            self.recurse(node);
            return;
        }

        let operator = node.child_by_field_name("operator").map(|n| self.node_text(n)).unwrap_or_default();

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
                if right.kind() == "binary_expression" && self.is_binary_left_identifier(right, &var_name) {
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

    /// Track boolean assignments like `first = false` for if-statement handling.
    fn track_bool_assignment(&mut self, var_name: &str, node: &Node) {
        if let Some(right) = node.child_by_field_name("right") {
            let text = self.node_text(right);
            if text == "true" {
                self.local_bool_vars.insert(var_name.to_string(), true);
            } else if text == "false" {
                self.local_bool_vars.insert(var_name.to_string(), false);
            }
        }
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
            _ => Some(vec![(self.make_placeholder_for_node(node), false)]),
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
        let converted_sql = self.convert_placeholders(&new_sql);

        let sql_kind = self.extractions[idx].sql_kind;
        let parse_result = self.try_parse_sql(&converted_sql, sql_kind);

        let ext = match self.extractions.get_mut(idx) {
            Some(e) => e,
            None => return,
        };
        ext.sql = converted_sql;
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
        let sql_converted = self.convert_placeholders(&sql_text);
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

        let was_sb = self.sql_vars.get(var_name).is_some_and(|v| v.is_string_builder);
        self.sql_vars.insert(
            var_name.to_string(),
            TrackedVar {
                sql: sql_converted,
                extraction_index: self.extractions.len() - 1,
                is_string_builder: was_sb,
                is_field_level: false,
            },
        );
    }

    fn track_string_constant(&mut self, node: Node) {
        if !self.has_final_modifier(node) {
            return;
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "variable_declarator" {
                let name_node = match child.child_by_field_name("name") {
                    Some(n) => n,
                    None => continue,
                };
                let value_node = match child.child_by_field_name("value") {
                    Some(n) => n,
                    None => continue,
                };
                if value_node.kind() == "string_literal" {
                    let raw = self.node_text(value_node);
                    let is_tb = raw.starts_with("\"\"\"");
                    let val = self.decode_java_string(&raw, is_tb);
                    self.string_constants.insert(self.node_text(name_node), val);
                }
            }
        }
    }

    fn track_local_string_constant(&mut self, node: Node) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "variable_declarator" {
                let name_node = match child.child_by_field_name("name") {
                    Some(n) => n,
                    None => continue,
                };
                let value_node = match child.child_by_field_name("value") {
                    Some(n) => n,
                    None => continue,
                };
                if value_node.kind() == "string_literal" {
                    let raw = self.node_text(value_node);
                    let is_tb = raw.starts_with("\"\"\"");
                    let val = self.decode_java_string(&raw, is_tb);
                    self.string_constants.insert(self.node_text(name_node), val);
                }
            }
        }
    }

    fn has_final_modifier(&self, node: Node) -> bool {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "modifiers" {
                let mut mc = child.walk();
                for m in child.children(&mut mc) {
                    if self.node_text(m) == "final" {
                        return true;
                    }
                }
            }
        }
        false
    }
}
