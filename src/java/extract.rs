//! Java CST 遍历与 SQL 提取。

use std::collections::HashMap;

use tree_sitter::Node;

use super::types::{JavaExtractConfig, JdbcParamInfo, MethodPsBehavior};
use crate::java::error::JavaError;
use crate::java::method_call::PendingInjection;
use crate::java::types::*;



pub(super) struct TrackedVar {
    pub(super) sql: String,
    pub(super) extraction_index: usize,
    pub(super) is_string_builder: bool,
    pub(super) is_field_level: bool,
}

pub fn extract(
    source: &str,
    root: Node,
    file_path: &str,
    config: &JavaExtractConfig,
) -> JavaExtractResult {
    let mut sql_var_patterns_upper = vec!["SQL".to_string()];
    for p in &config.extra_sql_var_patterns {
        let upper = p.to_uppercase();
        if !sql_var_patterns_upper.contains(&upper) {
            sql_var_patterns_upper.push(upper);
        }
    }

    let mut ctx = ExtractContext {
        source,
        file_path,
        extractions: Vec::new(),
        errors: Vec::new(),
        class_name: None,
        method_name: None,
        sql_vars: HashMap::new(),
        var_types: HashMap::new(),
        jdbc_param_map: HashMap::new(),
        ps_var_to_extraction: HashMap::new(),
        extra_sql_methods: &config.extra_sql_methods,
        sql_var_patterns_upper,
        method_behaviors: HashMap::new(),
        pending_injections: Vec::new(),
        class_ps_to_extraction: HashMap::new(),
        dynamic_setters: Vec::new(),
    };
    ctx.visit(root);
    let extractions: Vec<ExtractedSql> = ctx
        .extractions
        .into_iter()
        .filter(|e| starts_with_sql_keyword(&e.sql))
        .collect();
    JavaExtractResult {
        file_path: file_path.to_string(),
        extractions,
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
    /// Tracks setXxx calls: (ps_var_name, param_index) → inferred info.
    pub(super) jdbc_param_map: HashMap<(String, usize), JdbcParamInfo>,
    /// Maps PreparedStatement variable name → extraction index for backfill.
    pub(super) ps_var_to_extraction: HashMap<String, usize>,
    pub(super) extra_sql_methods: &'a [String],
    pub(super) sql_var_patterns_upper: Vec<String>,
    pub(super) method_behaviors: HashMap<String, MethodPsBehavior>,
    pub(super) pending_injections: Vec<PendingInjection>,
    pub(super) class_ps_to_extraction: HashMap<String, usize>,
    pub(super) dynamic_setters: Vec<(String, String)>,
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
        self.apply_pending_injections();
        self.class_name = old_class;
    }

    pub(super) fn visit_method_declaration(&mut self, node: Node) {
        let old_method = self.method_name.clone();
        let field_sql_vars: HashMap<String, TrackedVar> = self
            .sql_vars
            .iter()
            .filter(|(_, v)| v.is_field_level)
            .map(|(k, v)| (k.clone(), TrackedVar {
                sql: v.sql.clone(),
                extraction_index: v.extraction_index,
                is_string_builder: v.is_string_builder,
                is_field_level: true,
            }))
            .collect();
        let old_sql_vars = std::mem::take(&mut self.sql_vars);
        for (k, v) in field_sql_vars {
            self.sql_vars.insert(k, v);
        }
        let old_var_types = std::mem::take(&mut self.var_types);
        let old_jdbc_param_map = std::mem::take(&mut self.jdbc_param_map);
        let old_ps_var_to_extraction = std::mem::take(&mut self.ps_var_to_extraction);
        let old_dynamic_setters = std::mem::take(&mut self.dynamic_setters);
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

        // Backfill JDBC parameter type/name inference from setXxx() calls
        self.backfill_jdbc_params();

        self.record_method_behavior(node);

        for (var, idx) in &self.ps_var_to_extraction {
            self.class_ps_to_extraction.insert(var.clone(), *idx);
        }

        self.method_name = old_method;
        self.sql_vars = old_sql_vars;
        self.var_types = old_var_types;
        self.jdbc_param_map = old_jdbc_param_map;
        self.ps_var_to_extraction = old_ps_var_to_extraction;
        self.dynamic_setters = old_dynamic_setters;
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
                None => {
                    let default_type = self
                        .infer_type_from_concat_context(node)
                        .unwrap_or_else(|| sanitized.clone());
                    if default_type != sanitized {
                        format!("__JAVA_VAR_{}_{}__", default_type, sanitized)
                    } else {
                        format!("__JAVA_VAR_{}__", sanitized)
                    }
                }
            },
        }
    }

    /// Check if an identifier is used in a string concatenation context (parent `+` binary
    /// expression where the other operand is a string literal or String-typed expression).
    fn infer_type_from_concat_context(&self, node: Node) -> Option<String> {
        let parent = node.parent()?;
        if parent.kind() != "binary_expression" {
            return None;
        }
        let op = parent.child_by_field_name("operator").map(|n| self.node_text(n));
        if op.as_deref() != Some("+") {
            return None;
        }
        // Check if the OTHER operand of this + is a String
        let left = parent.child_by_field_name("left")?;
        let right = parent.child_by_field_name("right")?;
        let other = if left.id() == node.id() { right } else { left };
        let other_type = self.infer_expression_type(other);
        if other_type.as_deref() == Some("String") {
            return Some("String".to_string());
        }
        None
    }

    /// If this method_invocation is the RHS of a variable declaration or assignment,
    /// return the LHS variable name by walking up the CST.
    pub(super) fn find_assignment_target(&self, node: Node) -> Option<String> {
        let mut current = node;
        loop {
            let parent = current.parent()?;
            match parent.kind() {
                "variable_declarator" => {
                    if let Some(name_node) = parent.child_by_field_name("name") {
                        return Some(self.node_text(name_node));
                    }
                    return None;
                }
                "assignment_expression" => {
                    let left = parent.child_by_field_name("left")?;
                    if left.kind() == "identifier" {
                        return Some(self.node_text(left));
                    }
                    return None;
                }
                "argument_list" | "field_access" | "method_invocation" => {
                    current = parent;
                }
                _ => return None,
            }
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

    /// Backfill a single PS variable's old binding before its mapping is overwritten.
    pub(super) fn backfill_for_ps_var(&mut self, ps_var: &str, ext_idx: usize) {
        let keys: Vec<(String, usize)> = self
            .jdbc_param_map
            .keys()
            .filter(|(v, _)| v == ps_var)
            .cloned()
            .collect();

        if keys.is_empty() {
            return;
        }

        let mut modified = false;
        for key in &keys {
            let info = match self.jdbc_param_map.get(key) {
                Some(i) => i,
                None => continue,
            };
            let old = format!("__JAVA_VAR_JDBC_PARAM_{}__", info.index);
            let new = match &info.var_name {
                Some(name) => format!(
                    "__JAVA_VAR_{}_{}__",
                    info.java_type,
                    sanitize_var_name(name)
                ),
                None => format!(
                    "__JAVA_VAR_{}_JDBC_PARAM_{}__",
                    info.java_type,
                    info.index
                ),
            };
            if let Some(ext) = self.extractions.get_mut(ext_idx) {
                let before = ext.sql.len();
                ext.sql = ext.sql.replace(&old, &new);
                modified |= ext.sql.len() != before;
            }
        }

        for key in keys {
            self.jdbc_param_map.remove(&key);
        }

        if modified {
            if let Some(ext) = self.extractions.get(ext_idx) {
                let sql = ext.sql.clone();
                let sql_kind = ext.sql_kind;
                let parse_result = self.try_parse_sql(&sql, sql_kind);
                if let Some(ext) = self.extractions.get_mut(ext_idx) {
                    ext.parse_result = parse_result;
                }
            }
        }
    }

    /// After collecting setXxx() info, replace __JAVA_VAR_JDBC_PARAM_N__
    /// with __JAVA_VAR_{Type}_{name}__ in the extracted SQL.
    pub(super) fn backfill_jdbc_params(&mut self) {
        let mut to_reparse: std::collections::HashSet<usize> = std::collections::HashSet::new();

        for ((ps_var, _param_idx), info) in &self.jdbc_param_map {
            let ext_idx = match self.ps_var_to_extraction.get(ps_var) {
                Some(&idx) => idx,
                None => continue,
            };
            let ext = match self.extractions.get_mut(ext_idx) {
                Some(e) => e,
                None => continue,
            };

            let old = format!("__JAVA_VAR_JDBC_PARAM_{}__", info.index);
            let new = match &info.var_name {
                Some(name) => format!("__JAVA_VAR_{}_{}__", info.java_type, sanitize_var_name(name)),
                None => format!("__JAVA_VAR_{}_JDBC_PARAM_{}__", info.java_type, info.index),
            };
            ext.sql = ext.sql.replace(&old, &new);
            to_reparse.insert(ext_idx);
        }

        // Re-parse modified extractions
        for idx in to_reparse {
            let (sql, sql_kind) = {
                let ext = &self.extractions[idx];
                (ext.sql.clone(), ext.sql_kind)
            };
            let parse_result = self.try_parse_sql(&sql, sql_kind);
            if let Some(ext) = self.extractions.get_mut(idx) {
                ext.parse_result = parse_result;
            }
        }
    }

    fn record_method_behavior(&mut self, node: Node) {
        let method_name_str = match &self.method_name {
            Some(n) => n.clone(),
            None => return,
        };

        let mut ps_param_name: Option<String> = None;
        let mut ps_param_index: Option<usize> = None;

        if let Some(params_node) = node.child_by_field_name("parameters") {
            let mut cursor = params_node.walk();
            let mut param_idx = 0usize;
            for child in params_node.children(&mut cursor) {
                if child.kind() == "formal_parameter" {
                    let mut pc = child.walk();
                    let mut type_name: Option<String> = None;
                    let mut var_name: Option<String> = None;
                    for p in child.children(&mut pc) {
                        match p.kind() {
                            "type_identifier" | "generic_type" => {
                                type_name = self.extract_type_name(p);
                            }
                            "identifier" => {
                                var_name = Some(self.node_text(p));
                            }
                            _ => {}
                        }
                    }
                    if let (Some(t), Some(v)) = (&type_name, &var_name) {
                        if t == "PreparedStatement" {
                            ps_param_name = Some(v.clone());
                            ps_param_index = Some(param_idx);
                        }
                    }
                    param_idx += 1;
                }
            }
        }

        let ps_var = match ps_param_name {
            Some(v) => v,
            None => return,
        };
        let ps_idx = match ps_param_index {
            Some(i) => i,
            None => return,
        };

        let mut setter_patterns: Vec<crate::java::types::SetterPattern> = Vec::new();
        for ((var_name, _), info) in &self.jdbc_param_map {
            if var_name == &ps_var {
                setter_patterns.push(crate::java::types::SetterPattern::Literal {
                    index: info.index,
                    java_type: info.java_type.clone(),
                    var_name: info.var_name.clone(),
                });
            }
        }

        let mut seen_dynamic_types: std::collections::HashSet<String> = std::collections::HashSet::new();
        for (var_name, java_type) in &self.dynamic_setters {
            if var_name == &ps_var && !seen_dynamic_types.contains(java_type) {
                seen_dynamic_types.insert(java_type.clone());
                setter_patterns.push(crate::java::types::SetterPattern::DynamicLoop {
                    java_type: java_type.clone(),
                });
            }
        }

        if setter_patterns.is_empty() {
            return;
        }

        setter_patterns.sort_by_key(|p| match p {
            crate::java::types::SetterPattern::Literal { index, .. } => *index,
            crate::java::types::SetterPattern::DynamicLoop { .. } => usize::MAX,
        });

        self.method_behaviors.insert(
            method_name_str,
            crate::java::types::MethodPsBehavior {
                ps_param_index: ps_idx,
                ps_param_name: ps_var,
                setter_patterns,
            },
        );
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

fn starts_with_sql_keyword(sql: &str) -> bool {
    let first_word = sql.trim().split_whitespace().next().unwrap_or("");
    let upper = first_word.to_uppercase();
    matches!(upper.as_str(), "SELECT" | "INSERT" | "UPDATE" | "DELETE" | "MERGE" | "WITH")
}
