//! Java CST 遍历与 SQL 提取。

use std::collections::HashMap;

use tree_sitter::Node;

use super::types::JavaExtractConfig;
use crate::java::error::JavaError;
use crate::java::types::*;

const STRING_BUILDER_TYPES: &[&str] = &["StringBuilder", "StringBuffer"];

struct TrackedVar {
    sql: String,
    extraction_index: usize,
    is_string_builder: bool,
}

const SQL_ANNOTATIONS: &[&str] = &[
    "Query",
    "NamedQuery",
    "SqlUpdate",
    "SqlQuery",
    "Modifying",
    "Select",
    "Insert",
    "Update",
    "Delete",
    "NamedNativeQuery",
    "SqlBatch",
];

const SQL_METHOD_UNAMBIGUOUS: &[&str] = &[
    "createNativeQuery",
    "createQuery",
    "prepareStatement",
    "prepareCall",
    "executeQuery",
    "executeUpdate",
    "executeProcedure",
    "queryForObject",
    "queryForList",
    "queryForMap",
    "batchUpdate",
];

const SQL_METHOD_AMBIGUOUS: &[&str] = &["query", "update", "execute"];

const SQL_KEYWORDS: &[&str] = &[
    "SELECT ",
    "INSERT ",
    "UPDATE ",
    "DELETE ",
    "WITH ",
    "CREATE ",
    "ALTER ",
    "DROP ",
    "MERGE ",
    "TRUNCATE ",
    "CALL ",
    "{CALL ",
];

const SQL_NAME_PATTERN: &str = "SQL";

const SQL_STATEMENT_PREFIXES: &[&str] = &[
    "SELECT ",
    "INSERT ",
    "UPDATE ",
    "DELETE ",
    "WITH ",
    "CREATE ",
    "ALTER ",
    "DROP ",
    "MERGE ",
    "TRUNCATE ",
    "CALL ",
    "select ",
    "insert ",
    "update ",
    "delete ",
    "with ",
    "create ",
    "alter ",
    "drop ",
    "merge ",
    "truncate ",
    "call ",
];

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

struct ExtractContext<'a> {
    source: &'a str,
    file_path: &'a str,
    extractions: Vec<ExtractedSql>,
    errors: Vec<JavaError>,
    class_name: Option<String>,
    method_name: Option<String>,
    sql_vars: HashMap<String, TrackedVar>,
    var_types: HashMap<String, String>,
    extra_sql_methods: &'a [String],
}

impl<'a> ExtractContext<'a> {
    fn visit(&mut self, node: Node) {
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

    fn visit_type_declaration(&mut self, node: Node) {
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

    fn visit_method_declaration(&mut self, node: Node) {
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

    // ── P0: Annotation SQL Extraction ──

    fn visit_annotation(&mut self, node: Node) {
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

    fn visit_named_queries(&mut self, node: Node) {
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

    fn visit_sql_annotation(&mut self, node: Node, annotation_name: &str) {
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

    fn find_sql_value_in_annotation(
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

    fn check_native_query_flag(&self, args_node: &Node) -> NativeQueryFlag {
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

    // ── P1: Method Call SQL Extraction ──

    fn visit_method_invocation(&mut self, node: Node) {
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
            let sql_converted = convert_placeholders(&sql_text);
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

    fn find_first_string_arg(&self, args_node: &Node) -> Option<(String, bool)> {
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

    // ── P2: Constant SQL Extraction ──

    fn visit_field_declaration(&mut self, node: Node) {
        self.check_string_declaration(node);
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.visit(child);
        }
    }

    fn visit_local_variable_declaration(&mut self, node: Node) {
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

    fn detect_local_var_type(&self, node: Node) -> Option<String> {
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

    fn check_string_builder_declaration(&mut self, node: Node) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "variable_declarator" {
                self.check_sb_declarator(child);
            }
        }
    }

    fn check_sb_declarator(&mut self, declarator: Node) {
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

    fn check_string_declaration(&mut self, node: Node) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "variable_declarator" {
                self.check_declarator(child);
            }
        }
    }

    fn check_declarator(&mut self, declarator: Node) {
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

    // ── StringBuilder Method Handling ──

    fn find_method_chain_root(&self, node: Node) -> Option<String> {
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

    fn handle_string_builder_call(&mut self, node: Node, root_var: &str, method_name: &str) {
        match method_name {
            "append" => self.handle_sb_append(node, root_var),
            "insert" => self.handle_sb_insert(node, root_var),
            "delete" => self.handle_sb_delete(node, root_var),
            _ => {}
        }
    }

    fn handle_sb_append(&mut self, node: Node, root_var: &str) {
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

    fn extract_single_arg_value(&self, args_node: &Node) -> Option<String> {
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

    fn handle_sb_insert(&mut self, node: Node, root_var: &str) {
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

    fn handle_sb_delete(&mut self, node: Node, root_var: &str) {
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

    fn parse_java_int(&self, text: &str) -> Option<usize> {
        let s = text.trim();
        let s = s.strip_suffix('L').or_else(|| s.strip_suffix('l')).unwrap_or(s);
        s.parse().ok()
    }

    // ── Cross-Statement Concatenation Tracking ──

    fn visit_assignment_expression(&mut self, node: Node) {
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

    fn extract_concat_string_parts(&self, node: Node) -> Option<Vec<(String, bool)>> {
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

    fn is_binary_left_identifier(&self, node: Node, var_name: &str) -> bool {
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

    fn append_to_tracked_var(&mut self, var_name: &str, parts: &[(String, bool)], node: Node) {
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

    fn reassign_tracked_var(&mut self, var_name: &str, rhs: Node, node: Node) {
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

    fn recurse(&mut self, node: Node) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.visit(child);
        }
    }

    // ── String Extraction Helpers ──

    fn extract_string_value(&self, node: Node) -> Option<(String, bool)> {
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

    fn collect_concat_parts(&self, node: Node) -> Vec<(String, bool)> {
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
                    let var_name = self.node_text(left);
                    parts.push((self.make_var_placeholder(&var_name), false));
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
                    let var_name = self.node_text(right);
                    parts.push((self.make_var_placeholder(&var_name), false));
                }
            }
        }

        parts
    }

    fn decode_java_string(&self, raw: &str, is_text_block: bool) -> String {
        if is_text_block {
            self.decode_text_block(raw)
        } else {
            self.decode_regular_string(raw)
        }
    }

    fn decode_regular_string(&self, raw: &str) -> String {
        let inner = raw.strip_prefix('"').and_then(|s| s.strip_suffix('"'));
        match inner {
            Some(s) => self.process_escape_sequences(s),
            None => raw.to_string(),
        }
    }

    fn decode_text_block(&self, raw: &str) -> String {
        let inner = raw
            .strip_prefix("\"\"\"")
            .and_then(|s| s.strip_suffix("\"\"\""));
        let inner = match inner {
            Some(s) => s,
            None => return raw.to_string(),
        };

        let lines: Vec<&str> = inner.lines().collect();
        if lines.is_empty() {
            return String::new();
        }

        let start = if lines.first().map(|l| l.trim().is_empty()).unwrap_or(false) {
            1
        } else {
            0
        };

        let effective_lines = &lines[start..];
        let min_indent = effective_lines
            .iter()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.chars().take_while(|c| *c == ' ' || *c == '\t').count())
            .min()
            .unwrap_or(0);

        let result: Vec<String> = effective_lines
            .iter()
            .map(|l| {
                if l.len() >= min_indent {
                    l[min_indent..].to_string()
                } else {
                    l.trim_end().to_string()
                }
            })
            .collect();

        let mut joined = result.join("\n");
        joined = joined.trim().to_string();
        self.process_escape_sequences(&joined)
    }

    fn process_escape_sequences(&self, s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '\\' && i + 1 < chars.len() {
                match chars[i + 1] {
                    'n' => {
                        result.push('\n');
                        i += 2;
                    }
                    't' => {
                        result.push('\t');
                        i += 2;
                    }
                    'r' => {
                        result.push('\r');
                        i += 2;
                    }
                    '"' => {
                        result.push('"');
                        i += 2;
                    }
                    '\'' => {
                        result.push('\'');
                        i += 2;
                    }
                    '\\' => {
                        result.push('\\');
                        i += 2;
                    }
                    _ => {
                        result.push(chars[i]);
                        i += 1;
                    }
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }
        result
    }

    fn try_parse_sql(&self, sql: &str, sql_kind: SqlKind) -> Option<SqlParseResult> {
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

    fn extract_type_name(&self, node: Node) -> Option<String> {
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

    fn make_var_placeholder(&self, var_name: &str) -> String {
        let sanitized: String = var_name
            .chars()
            .map(|c| {
                if c == '.' || c == '(' || c == ')' {
                    '_'
                } else {
                    c
                }
            })
            .collect();
        match self.var_types.get(var_name) {
            Some(type_name) => format!("__JAVA_VAR_{}_{}__", type_name, sanitized),
            None => format!("__JAVA_VAR_{}__", sanitized),
        }
    }

    fn node_text(&self, node: Node) -> String {
        self.source[node.byte_range()].to_string()
    }
}

enum NativeQueryFlag {
    True,
    False,
    NotPresent,
}

fn looks_like_sql(text: &str) -> bool {
    let upper = text.to_uppercase();
    SQL_KEYWORDS.iter().any(|kw| upper.contains(kw))
}

fn detect_sql_kind_from_content(sql: &str) -> SqlKind {
    let upper = sql.trim().to_uppercase();
    let prefix = upper.split_whitespace().next().unwrap_or("");
    match prefix {
        "CREATE" | "ALTER" | "DROP" | "TRUNCATE" => SqlKind::Ddl,
        _ => SqlKind::NativeSql,
    }
}

fn detect_parameter_style(sql: &str) -> ParameterStyle {
    let mut has_question = false;
    let mut has_numbered = false;
    let mut has_named_colon = false;
    let mut has_hash = false;

    let chars: Vec<char> = sql.chars().collect();
    let len = chars.len();
    let mut in_string = false;
    let mut i = 0;

    while i < len {
        if chars[i] == '\'' {
            in_string = !in_string;
            i += 1;
            continue;
        }
        if in_string {
            i += 1;
            continue;
        }
        if chars[i] == '?' && i + 1 < len && chars[i + 1].is_ascii_digit() {
            has_numbered = true;
        } else if chars[i] == '?' {
            has_question = true;
        } else if chars[i] == ':' && i + 1 < len && chars[i + 1].is_ascii_alphabetic() {
            has_named_colon = true;
        } else if chars[i] == '#' && i + 1 < len && chars[i + 1] == '{' {
            has_hash = true;
        }
        i += 1;
    }

    if has_hash {
        ParameterStyle::NamedHash
    } else if has_numbered {
        ParameterStyle::PositionalNumbered
    } else if has_named_colon {
        ParameterStyle::NamedColon
    } else if has_question {
        ParameterStyle::PositionalQuestion
    } else {
        ParameterStyle::None
    }
}

fn convert_placeholders(sql: &str) -> String {
    let chars: Vec<char> = sql.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(sql.len() + sql.len() / 4);
    let mut i = 0;
    let mut question_counter: usize = 0;

    while i < len {
        if chars[i] == '\'' {
            result.push(chars[i]);
            i += 1;
            while i < len {
                if chars[i] == '\'' {
                    result.push(chars[i]);
                    i += 1;
                    break;
                }
                result.push(chars[i]);
                i += 1;
            }
            continue;
        }

        if chars[i] == '?' {
            question_counter += 1;
            let digit_start = i + 1;
            let mut digit_end = digit_start;
            while digit_end < len && chars[digit_end].is_ascii_digit() {
                digit_end += 1;
            }
            let param_num = if digit_end > digit_start {
                let num_str: String = chars[digit_start..digit_end].iter().collect();
                num_str.parse::<usize>().unwrap_or(question_counter)
            } else {
                question_counter
            };
            result.push_str(&format!("__JAVA_VAR_JDBC_PARAM_{}__", param_num));
            i = digit_end;
            continue;
        }

        if chars[i] == ':'
            && i + 1 < len
            && (chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_')
        {
            let name_start = i + 1;
            let mut name_end = name_start;
            while name_end < len
                && (chars[name_end].is_ascii_alphanumeric() || chars[name_end] == '_')
            {
                name_end += 1;
            }
            let name: String = chars[name_start..name_end].iter().collect();
            result.push_str(&format!("__JAVA_VAR_{}__", name));
            i = name_end;
            continue;
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}
