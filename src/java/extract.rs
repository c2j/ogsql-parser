//! Java CST 遍历与 SQL 提取。

use std::collections::HashMap;

use tree_sitter::Node;

use crate::java::error::JavaError;
use crate::java::types::*;

struct TrackedVar {
    sql: String,
    extraction_index: usize,
}

const SQL_ANNOTATIONS: &[&str] = &["Query", "NamedQuery", "SqlUpdate", "SqlQuery", "Modifying"];

const SQL_METHOD_UNAMBIGUOUS: &[&str] = &[
    "createNativeQuery",
    "createQuery",
    "prepareStatement",
    "prepareCall",
    "executeQuery",
    "executeUpdate",
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
];

const SQL_NAME_KEYWORDS: &[&str] = &["SQL_", "_SQL", "SQLQUERY", "QUERY_"];

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

pub fn extract(source: &str, root: Node, file_path: &str) -> JavaExtractResult {
    let mut ctx = ExtractContext {
        source,
        file_path,
        extractions: Vec::new(),
        errors: Vec::new(),
        class_name: None,
        method_name: None,
        sql_vars: HashMap::new(),
        var_types: HashMap::new(),
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
    /// Maps variable name to Java type name (e.g., "tableName" to "String")
    var_types: HashMap<String, String>,
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

        // extract parameter types
        if let Some(params_node) = node.child_by_field_name("parameters") {
            let mut cursor = params_node.walk();
            for child in params_node.children(&mut cursor) {
                if child.kind() == "formal_parameter" {
                    let mut param_cursor = child.walk();
                    let mut type_name: Option<String> = None;
                    let mut var_name: Option<String> = None;
                    for pc in child.children(&mut param_cursor) {
                        match pc.kind() {
                            "type_identifier" | "primitive_type" | "generic_type"
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

        // process annotations — modifiers is not a named field, iterate children
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

        // recurse into body for P1/P2
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
                    "SqlUpdate" | "SqlQuery" => SqlKind::NativeSql,
                    "Query" => SqlKind::Jpql,
                    _ => SqlKind::NativeSql,
                },
            };

            let param_style = detect_parameter_style(&sql_text);
            let parse_result = self.try_parse_sql(&sql_text, sql_kind);

            self.extractions.push(ExtractedSql {
                sql: sql_text,
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
            "NamedQuery" => "query",
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

        // fallback for @Query("SELECT ...") without explicit value= key
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

        let is_unambiguous = SQL_METHOD_UNAMBIGUOUS.contains(&method_name.as_str());
        let is_ambiguous = SQL_METHOD_AMBIGUOUS.contains(&method_name.as_str());

        if !is_unambiguous && !is_ambiguous {
            // still recurse — method calls may be nested
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
            let parse_result = self.try_parse_sql(&sql_text, sql_kind);

            self.extractions.push(ExtractedSql {
                sql: sql_text,
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
        self.check_string_declaration(node);

        let mut cursor = node.walk();
        let mut type_name: Option<String> = None;
        for child in node.children(&mut cursor) {
            match child.kind() {
                "type_identifier" | "primitive_type" | "generic_type" | "array_type" => {
                    type_name = self.extract_type_name(child);
                }
                "variable_declarator" => {
                    if let Some(t) = &type_name {
                        if let Some(name_node) = child.child_by_field_name("name") {
                            self.var_types.insert(self.node_text(name_node), t.clone());
                        }
                    }
                }
                _ => {}
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.visit(child);
        }
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
        let name_looks_like_sql = SQL_NAME_KEYWORDS
            .iter()
            .any(|kw| var_name_upper.contains(kw));

        let content_looks_like_sql = looks_like_sql(&sql_text);

        if !name_looks_like_sql && !content_looks_like_sql {
            return;
        }

        // when only the variable name hints SQL (not the content), require
        // the content to at least start with a SQL statement prefix to avoid
        // false positives like "No SqlSessionFactory specified"
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
        let is_concatenated = value_node.kind() == "binary_expression";
        let parse_result = self.try_parse_sql(&sql_text, sql_kind);

        self.extractions.push(ExtractedSql {
            sql: sql_text.clone(),
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
                sql: sql_text,
                extraction_index: self.extractions.len() - 1,
            },
        );
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

        let append_parts = match operator.as_str() {
            "+=" => self.extract_concat_string_parts(right),
            "=" => self.extract_append_parts_for_var(right, &var_name),
            _ => None,
        };

        if let Some(parts) = append_parts {
            if !parts.is_empty() {
                self.append_to_tracked_var(&var_name, &parts, node);
            }
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

    fn extract_append_parts_for_var(
        &self,
        rhs: Node,
        var_name: &str,
    ) -> Option<Vec<(String, bool)>> {
        match rhs.kind() {
            "string_literal" => {
                let raw = self.node_text(rhs);
                let is_tb = raw.starts_with("\"\"\"");
                Some(vec![(self.decode_java_string(&raw, is_tb), is_tb)])
            }
            "binary_expression" => {
                let parts = self.collect_concat_parts(rhs);
                if parts.is_empty() {
                    return None;
                }
                if self.is_binary_left_identifier(rhs, var_name) {
                    Some(parts.into_iter().skip(1).collect())
                } else {
                    Some(parts)
                }
            }
            _ => None,
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

        // replace JDBC ? parameter markers with NULL so the parser can handle them
        let parseable = replace_jdbc_parameters(&flat_sql);
        let (stmts, errors) = crate::parser::Parser::parse_sql(&parseable);
        Some(SqlParseResult {
            statements: stmts,
            errors,
        })
    }

    fn extract_type_name(&self, node: Node) -> Option<String> {
        match node.kind() {
            "type_identifier" | "primitive_type" => Some(self.node_text(node)),
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
        match self.var_types.get(var_name) {
            Some(type_name) => format!("__JAVA_VAR_{}_{}__", type_name, var_name),
            None => format!("__JAVA_VAR_{}__", var_name),
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

/// Replace JDBC `?` parameter markers and `:name` named parameters with `NULL`
/// so the core SQL parser (which doesn't understand JDBC parameters) can parse the statement.
/// Also replaces `?1` numbered params. String literals are preserved.
fn replace_jdbc_parameters(sql: &str) -> String {
    let chars: Vec<char> = sql.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(sql.len());
    let mut in_string = false;
    let mut i = 0;

    while i < len {
        if chars[i] == '\'' {
            in_string = !in_string;
            result.push(chars[i]);
            i += 1;
            continue;
        }
        if in_string {
            result.push(chars[i]);
            i += 1;
            continue;
        }

        // `?` followed by digit → numbered param like ?1
        if chars[i] == '?' {
            result.push_str("NULL");
            i += 1;
            // skip trailing digits for ?1, ?2 etc
            while i < len && chars[i].is_ascii_digit() {
                i += 1;
            }
            continue;
        }

        // `:name` → named param, replace with NULL
        if chars[i] == ':'
            && i + 1 < len
            && (chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_')
        {
            result.push_str("NULL");
            i += 1;
            while i < len && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            continue;
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}
