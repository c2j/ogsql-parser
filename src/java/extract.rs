//! Java CST 遍历与 SQL 提取。

use std::collections::HashMap;

use tree_sitter::Node;

use super::types::{
    ExtractedSql, ExtractionMethod, JavaExtractConfig, JavaExtractResult, JdbcParamInfo, MethodPsBehavior, SqlKind,
    SqlOrigin, SqlParseResult,
};
use crate::java::error::JavaError;
use crate::java::method_call::PendingInjection;

/// Tracks how a Collection/List variable gets its values.
#[derive(Clone, Debug)]
pub(super) enum ListSource {
    /// Created via `new ArrayList<>()` — no known source yet.
    NewArrayList,
    /// Populated by iterating over a source set with a filter guard.
    Filtered { source_set_var: String },
}

pub(super) struct TrackedVar {
    pub(super) sql: String,
    pub(super) extraction_index: usize,
    pub(super) is_string_builder: bool,
    pub(super) is_field_level: bool,
}

pub fn extract(source: &str, root: Node, file_path: &str, config: &JavaExtractConfig) -> JavaExtractResult {
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
        string_constants: HashMap::new(),
        known_sets: HashMap::new(),
        list_sources: HashMap::new(),
        string_exprs: HashMap::new(),
        local_bool_vars: HashMap::new(),
        pending_string_vars: HashMap::new(),
    };
    ctx.visit(root);
    ctx.apply_pending_injections();
    let extractions: Vec<ExtractedSql> =
        ctx.extractions.into_iter().filter(|e| starts_with_sql_keyword(&e.sql)).collect();
    JavaExtractResult { file_path: file_path.to_string(), extractions, errors: ctx.errors }
}

pub(super) struct ExtractContext<'a> {
    pub(super) source: &'a str,
    #[allow(dead_code)]
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
    pub(super) string_constants: HashMap<String, String>,
    /// Known set literals from `Set.of("a", "b", ...)` declarations.
    pub(super) known_sets: HashMap<String, Vec<String>>,
    /// How each collection/list variable is populated.
    pub(super) list_sources: HashMap<String, ListSource>,
    /// Evaluation results of String-typed method invocations for cross-method tracking.
    pub(super) string_exprs: HashMap<String, String>,
    /// Tracks known boolean local variable values (e.g., `first = true`).
    pub(super) local_bool_vars: HashMap<String, bool>,
    /// Deferred accumulation pool for String variables not yet confirmed as SQL-related.
    /// Tracks `+=` and `=` construction for any local String variable. When such a variable
    /// is later appended to a tracked StringBuilder, its accumulated value is flushed into
    /// the SQL output instead of a generic placeholder.
    pub(super) pending_string_vars: HashMap<String, String>,
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
            "return_statement" => {
                self.visit_return_statement(node);
            }
            "switch_expression" => {
                self.visit_switch_expression(node);
            }
            "enhanced_for_statement" => {
                self.visit_enhanced_for_statement(node);
            }
            "resource" => {
                self.visit_resource(node);
            }
            "if_statement" => {
                self.visit_if_statement(node);
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

    pub(super) fn visit_switch_expression(&mut self, node: Node) {
        let body = match node.child_by_field_name("body") {
            Some(b) => b,
            None => {
                self.recurse(node);
                return;
            }
        };
        let mut cursor = body.walk();
        for child in body.children(&mut cursor) {
            if child.kind() == "switch_rule" {
                self.extract_switch_rule_sql(child);
            } else if child.kind() == "switch_block_statement_group" {
                self.recurse(child);
            }
        }
    }

    fn extract_switch_rule_sql(&mut self, rule: Node) {
        let mut cursor = rule.walk();
        for child in rule.children(&mut cursor) {
            match child.kind() {
                "expression_statement" => {
                    let expr = child.children(&mut child.walk()).find(|c| c.kind() != ";");
                    if let Some(expr_node) = expr {
                        self.extract_expression_sql(
                            expr_node,
                            child.start_position().row + 1,
                            child.start_position().column,
                        );
                    }
                }
                "block" => self.recurse(child),
                _ => {}
            }
        }
    }

    fn extract_expression_sql(&mut self, node: Node, line: usize, column: usize) {
        let (sql_text, is_text_block) = match self.extract_string_value(node) {
            Some(v) => v,
            None => return,
        };
        if !super::heuristics::looks_like_sql(&sql_text) {
            return;
        }
        let sql_kind = super::heuristics::detect_sql_kind_from_content(&sql_text);
        let param_style = super::heuristics::detect_parameter_style(&sql_text);
        let sql_converted = self.convert_placeholders(&sql_text);
        let is_concatenated = node.kind() == "binary_expression";
        let parse_result = self.try_parse_sql(&sql_converted, sql_kind);

        self.extractions.push(ExtractedSql {
            sql: sql_converted,
            origin: SqlOrigin {
                method: ExtractionMethod::Constant,
                class_name: self.class_name.clone(),
                method_name: self.method_name.clone(),
                annotation_name: None,
                api_method_name: None,
                variable_name: None,
                line,
                column,
            },
            sql_kind,
            parameter_style: param_style,
            is_concatenated,
            is_text_block,
            parse_result,
        });
    }

    pub(super) fn visit_return_statement(&mut self, node: Node) {
        let value_node = node.children(&mut node.walk()).find(|c| c.kind() != "return");
        if let Some(value) = value_node {
            self.extract_expression_sql(value, node.start_position().row + 1, node.start_position().column);
        }
        self.recurse(node);
    }

    pub(super) fn visit_method_declaration(&mut self, node: Node) {
        let old_method = self.method_name.clone();
        let field_sql_vars: HashMap<String, TrackedVar> = self
            .sql_vars
            .iter()
            .filter(|(_, v)| v.is_field_level)
            .map(|(k, v)| {
                (
                    k.clone(),
                    TrackedVar {
                        sql: v.sql.clone(),
                        extraction_index: v.extraction_index,
                        is_string_builder: v.is_string_builder,
                        is_field_level: true,
                    },
                )
            })
            .collect();
        let old_sql_vars = std::mem::take(&mut self.sql_vars);
        for (k, v) in field_sql_vars {
            self.sql_vars.insert(k, v);
        }
        let old_var_types = self.var_types.clone();
        let old_jdbc_param_map = std::mem::take(&mut self.jdbc_param_map);
        let old_ps_var_to_extraction = std::mem::take(&mut self.ps_var_to_extraction);
        let old_dynamic_setters = std::mem::take(&mut self.dynamic_setters);
        let old_local_bool_vars = std::mem::take(&mut self.local_bool_vars);
        let old_pending_string_vars = std::mem::take(&mut self.pending_string_vars);
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
                            | "generic_type"
                            | "scoped_type_identifier"
                            | "primitive_type"
                            | "integral_type"
                            | "floating_point_type"
                            | "boolean_type" => {
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

        self.track_return_sql(node);
        self.record_method_behavior(node);
        self.record_delegation_behavior(node);

        for (var, idx) in &self.ps_var_to_extraction {
            self.class_ps_to_extraction.insert(var.clone(), *idx);
        }

        self.method_name = old_method;
        self.sql_vars = old_sql_vars;
        for (k, v) in old_var_types {
            self.var_types.entry(k).or_insert(v);
        }
        self.jdbc_param_map = old_jdbc_param_map;
        self.ps_var_to_extraction = old_ps_var_to_extraction;
        self.dynamic_setters = old_dynamic_setters;
        self.local_bool_vars = old_local_bool_vars;
        self.pending_string_vars = old_pending_string_vars;
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
            "ternary_expression" => {
                let consequence = node.child_by_field_name("consequence");
                let alternative = node.child_by_field_name("alternative");
                let mut results = Vec::new();
                if let Some(cons) = consequence {
                    if let Some((sql, _)) = self.extract_string_value(cons) {
                        if super::heuristics::looks_like_sql(&sql) {
                            results.push(sql);
                        }
                    }
                }
                if let Some(alt) = alternative {
                    if let Some((sql, _)) = self.extract_string_value(alt) {
                        if super::heuristics::looks_like_sql(&sql) {
                            results.push(sql);
                        }
                    }
                }
                if results.is_empty() {
                    None
                } else {
                    Some((results.join(""), false))
                }
            }
            "field_access" => self.resolve_field_access_constant(node).map(|v| (v, false)),
            "identifier" => {
                let name = self.node_text(node);
                self.string_constants.get(&name).map(|v| (v.clone(), false))
            }
            _ => None,
        }
    }

    pub(super) fn collect_concat_parts(&self, node: Node) -> Vec<(String, bool)> {
        let op = node.child_by_field_name("operator").map(|n| self.node_text(n));
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
                "identifier" => {
                    let name = self.node_text(left);
                    if let Some(val) = self.string_constants.get(&name) {
                        parts.push((val.clone(), false));
                    } else if let Some(val) = self.string_exprs.get(&name) {
                        parts.push((val.clone(), false));
                    } else {
                        parts.push((self.make_placeholder_for_node(left), false));
                    }
                }
                "field_access" => {
                    if let Some(val) = self.resolve_field_access_constant(left) {
                        parts.push((val, false));
                    } else {
                        parts.push((self.make_placeholder_for_node(left), false));
                    }
                }
                "method_invocation" => {
                    if let Some(name_n) = left.child_by_field_name("name") {
                        let name = self.node_text(name_n);
                        if let Some(val) = self.string_constants.get(&name) {
                            parts.push((val.clone(), false));
                        } else {
                            parts.push((self.make_placeholder_for_node(left), false));
                        }
                    } else {
                        parts.push((self.make_placeholder_for_node(left), false));
                    }
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
                "identifier" => {
                    let name = self.node_text(right);
                    if let Some(val) = self.string_constants.get(&name) {
                        parts.push((val.clone(), false));
                    } else if let Some(val) = self.string_exprs.get(&name) {
                        parts.push((val.clone(), false));
                    } else {
                        parts.push((self.make_placeholder_for_node(right), false));
                    }
                }
                "field_access" => {
                    if let Some(val) = self.resolve_field_access_constant(right) {
                        parts.push((val, false));
                    } else {
                        parts.push((self.make_placeholder_for_node(right), false));
                    }
                }
                "method_invocation" => {
                    if let Some(name_n) = right.child_by_field_name("name") {
                        let name = self.node_text(name_n);
                        if let Some(val) = self.string_constants.get(&name) {
                            parts.push((val.clone(), false));
                        } else {
                            parts.push((self.make_placeholder_for_node(right), false));
                        }
                    } else {
                        parts.push((self.make_placeholder_for_node(right), false));
                    }
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
        Some(SqlParseResult { statements: stmts, errors })
    }

    pub(super) fn extract_type_name(&self, node: Node) -> Option<String> {
        match node.kind() {
            "type_identifier" | "primitive_type" | "integral_type" | "floating_point_type" | "boolean_type" => {
                Some(self.node_text(node))
            }
            "scoped_type_identifier" => {
                let text = self.node_text(node);
                let short = text.rsplit('.').next().unwrap_or(&text);
                Some(short.to_string())
            }
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

    #[allow(dead_code)]
    pub(super) fn make_var_placeholder(&self, var_name: &str) -> String {
        let sanitized = sanitize_var_name(var_name);
        let type_name = self.var_types.get(var_name).or_else(|| {
            let trimmed = var_name.trim();
            if trimmed != var_name {
                self.var_types.get(trimmed)
            } else {
                None
            }
        });
        match type_name {
            Some(t) => format!("__JAVA_RAW_{}_{}__", t, sanitized),
            None => format!("__JAVA_RAW_{}__", sanitized),
        }
    }

    pub(super) fn make_placeholder_for_node(&self, node: Node) -> String {
        let var_name = self.node_text(node);
        let display_name = match node.kind() {
            "field_access" => var_name.rsplit('.').next().unwrap_or(&var_name).to_string(),
            "method_invocation" => {
                node.child_by_field_name("name").map(|n| self.node_text(n)).unwrap_or_else(|| var_name.clone())
            }
            _ => var_name.clone(),
        };
        let sanitized = sanitize_var_name(&display_name);
        let inferred_type = self.infer_expression_type(node);
        match inferred_type {
            Some(t) => format!("__JAVA_RAW_{}_{}__", t, sanitized),
            None => match self.var_types.get(&display_name).or_else(|| self.var_types.get(&var_name)) {
                Some(t) => format!("__JAVA_RAW_{}_{}__", t, sanitized),
                None => {
                    let default_type = self.infer_type_from_concat_context(node).unwrap_or_else(|| sanitized.clone());
                    if default_type != sanitized {
                        format!("__JAVA_RAW_{}_{}__", default_type, sanitized)
                    } else {
                        format!("__JAVA_RAW_{}__", sanitized)
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
                "variable_declarator" | "resource" => {
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

    pub(super) fn visit_resource(&mut self, node: Node) {
        if let Some(type_node) = node.child_by_field_name("type") {
            if let Some(type_name) = self.extract_type_name(type_node) {
                if let Some(name_node) = node.child_by_field_name("name") {
                    self.var_types.insert(self.node_text(name_node), type_name);
                }
            }
        }
        self.recurse(node);
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
                let op = node.child_by_field_name("operator").map(|n| self.node_text(n));
                if op.as_deref() == Some("+") {
                    let left_type = node.child_by_field_name("left").and_then(|n| self.infer_expression_type(n));
                    let right_type = node.child_by_field_name("right").and_then(|n| self.infer_expression_type(n));
                    if left_type.as_deref() == Some("String") || right_type.as_deref() == Some("String") {
                        return Some("String".to_string());
                    }
                    return left_type.or(right_type);
                }
                node.child_by_field_name("left").and_then(|n| self.infer_expression_type(n))
            }
            "method_invocation" => {
                let name = node.child_by_field_name("name").map(|n| self.node_text(n));
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
        let keys: Vec<(String, usize)> = self.jdbc_param_map.keys().filter(|(v, _)| v == ps_var).cloned().collect();

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
                Some(name) => format!("__JAVA_VAR_{}_{}__", info.java_type, sanitize_var_name(name)),
                None => format!("__JAVA_VAR_{}_JDBC_PARAM_{}__", info.java_type, info.index),
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

    pub(super) fn backfill_jdbc_params(&mut self) {
        // Pre-scan: detect var_names that appear for multiple param indices in the same extraction.
        // e.g. `params` for indices 1,2,3 → need disambiguation.
        let mut dup_names: std::collections::HashMap<usize, std::collections::HashSet<String>> =
            std::collections::HashMap::new();
        {
            let mut name_to_indices: std::collections::HashMap<usize, std::collections::HashMap<String, Vec<usize>>> =
                std::collections::HashMap::new();
            for ((ps_var, _param_idx), info) in &self.jdbc_param_map {
                if let Some(&ext_idx) = self.ps_var_to_extraction.get(ps_var) {
                    if let Some(name) = &info.var_name {
                        name_to_indices.entry(ext_idx).or_default().entry(name.clone()).or_default().push(info.index);
                    }
                }
            }
            for (ext_idx, name_map) in &name_to_indices {
                for (name, indices) in name_map {
                    if indices.len() > 1 {
                        dup_names.entry(*ext_idx).or_default().insert(name.clone());
                    }
                }
            }
        }

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
                Some(name) => {
                    let sanitized = sanitize_var_name(name);
                    if dup_names.get(&ext_idx).is_some_and(|s| s.contains(name)) {
                        format!("__JAVA_VAR_{}_{}_{}__", info.java_type, sanitized, info.index)
                    } else {
                        format!("__JAVA_VAR_{}_{}__", info.java_type, sanitized)
                    }
                }
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
        let mut param_name_to_idx: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

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
                            "type_identifier" | "generic_type" | "scoped_type_identifier" => {
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
                        param_name_to_idx.insert(v.clone(), param_idx);
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
                let pidx = info.var_name.as_ref().and_then(|n| param_name_to_idx.get(n).copied());
                setter_patterns.push(crate::java::types::SetterPattern::Literal {
                    index: info.index,
                    java_type: info.java_type.clone(),
                    var_name: info.var_name.clone(),
                    param_index: pidx,
                });
            }
        }

        let mut seen_dynamic_types: std::collections::HashSet<String> = std::collections::HashSet::new();
        for (var_name, java_type) in &self.dynamic_setters {
            if var_name == &ps_var && !seen_dynamic_types.contains(java_type) {
                seen_dynamic_types.insert(java_type.clone());
                setter_patterns.push(crate::java::types::SetterPattern::DynamicLoop { java_type: java_type.clone() });
            }
        }

        if setter_patterns.is_empty() {
            return;
        }

        let total_params = {
            let mut count = 0usize;
            if let Some(params_node) = node.child_by_field_name("parameters") {
                let mut cursor = params_node.walk();
                for child in params_node.children(&mut cursor) {
                    if child.kind() == "formal_parameter" {
                        count += 1;
                    }
                }
            }
            count
        };

        setter_patterns.sort_by_key(|p| match p {
            crate::java::types::SetterPattern::Literal { index, .. } => *index,
            crate::java::types::SetterPattern::DynamicLoop { .. } => usize::MAX,
        });

        let key = format!("{}:{}", method_name_str, total_params);
        self.method_behaviors.insert(
            key,
            crate::java::types::MethodPsBehavior { ps_param_index: ps_idx, ps_param_name: ps_var, setter_patterns },
        );
    }

    fn track_return_sql(&mut self, node: Node) {
        let method_name_str = match &self.method_name {
            Some(n) => n.clone(),
            None => return,
        };
        let body = match node.child_by_field_name("body") {
            Some(b) => b,
            None => return,
        };
        let mut last_return: Option<Node> = None;
        let mut cursor = body.walk();
        for child in body.children(&mut cursor) {
            if child.kind() == "return_statement" {
                last_return = Some(child);
            }
        }
        let ret_stmt = match last_return {
            Some(n) => n,
            None => return,
        };
        let ret_value = match ret_stmt.children(&mut ret_stmt.walk()).find(|c| c.kind() != "return") {
            Some(n) => n,
            None => return,
        };
        if let Some((sql_text, _)) = self.extract_string_value(ret_value) {
            if super::heuristics::looks_like_sql(&sql_text) {
                self.string_constants.insert(method_name_str, sql_text);
            }
        }
    }

    fn record_delegation_behavior(&mut self, node: Node) {
        let method_name_str = match &self.method_name {
            Some(n) => n.clone(),
            None => return,
        };
        if self.method_behaviors.contains_key(&format!("{}:", method_name_str))
            || self.method_behaviors.contains_key(&method_name_str)
        {
            return;
        }

        let mut ps_param_index: Option<usize> = None;
        if let Some(params_node) = node.child_by_field_name("parameters") {
            let mut cursor = params_node.walk();
            let mut idx = 0usize;
            for child in params_node.children(&mut cursor) {
                if child.kind() == "formal_parameter" {
                    let mut pc = child.walk();
                    let mut type_name: Option<String> = None;
                    for p in child.children(&mut pc) {
                        if p.kind() == "type_identifier"
                            || p.kind() == "generic_type"
                            || p.kind() == "scoped_type_identifier"
                        {
                            type_name = self.extract_type_name(p);
                        }
                    }
                    if type_name.as_deref() == Some("PreparedStatement") {
                        ps_param_index = Some(idx);
                    }
                    idx += 1;
                }
            }
        }
        let ps_idx = match ps_param_index {
            Some(i) => i,
            None => return,
        };

        let total_params = {
            let mut count = 0usize;
            if let Some(params_node) = node.child_by_field_name("parameters") {
                let mut cursor = params_node.walk();
                for child in params_node.children(&mut cursor) {
                    if child.kind() == "formal_parameter" {
                        count += 1;
                    }
                }
            }
            count
        };

        for injection in &self.pending_injections {
            if injection.extraction_idx.is_some() {
                continue;
            }
            let behavior_key = format!("{}:{}", injection.method_name, injection.arg_count);
            let behavior =
                self.method_behaviors.get(&behavior_key).or_else(|| self.method_behaviors.get(&injection.method_name));
            if let Some(behavior) = behavior {
                if behavior.ps_param_index < injection.call_arg_names.len() {
                    self.method_behaviors.insert(
                        format!("{}:{}", method_name_str, total_params),
                        crate::java::types::MethodPsBehavior {
                            ps_param_index: ps_idx,
                            ps_param_name: String::new(),
                            setter_patterns: behavior.setter_patterns.clone(),
                        },
                    );
                }
                return;
            }
        }
    }

    pub(super) fn node_text(&self, node: Node) -> String {
        self.source[node.byte_range()].to_string()
    }

    fn resolve_field_access_constant(&self, node: Node) -> Option<String> {
        if node.kind() != "field_access" {
            return None;
        }
        let last_ident = node.child_by_field_name("field").or_else(|| {
            let mut cursor = node.walk();
            node.children(&mut cursor).filter(|c| c.kind() == "identifier").last()
        });
        match last_ident {
            Some(n) => {
                let name = self.node_text(n);
                self.string_constants.get(&name).cloned()
            }
            None => None,
        }
    }

    // ── Cross-method evaluation helpers ──

    /// Extract elements from `Set.of("a", "b", ...)` into `known_sets`.
    pub(super) fn try_extract_set_of(&mut self, value_node: Node, var_name: &str) {
        if value_node.kind() != "method_invocation" {
            return;
        }
        let method_name = match value_node.child_by_field_name("name") {
            Some(n) => self.node_text(n),
            None => return,
        };
        if method_name != "of" {
            return;
        }
        let args = match value_node.child_by_field_name("arguments") {
            Some(n) => n,
            None => return,
        };
        let mut elements = Vec::new();
        for i in 0..args.child_count() {
            if let Some(child) = args.child(i) {
                if child.kind() == "string_literal" {
                    let raw = self.node_text(child);
                    elements.push(self.decode_java_string(&raw, false));
                }
            }
        }
        if !elements.is_empty() {
            self.known_sets.insert(var_name.to_string(), elements);
        }
    }

    /// Track `new ArrayList<>()` declarations in `list_sources`.
    pub(super) fn try_track_list_declaration(&mut self, value_node: Node, var_name: &str) {
        if value_node.kind() != "object_creation_expression" {
            return;
        }
        let type_node = match value_node.child_by_field_name("type") {
            Some(n) => n,
            None => return,
        };
        let type_text = self.node_text(type_node);
        if type_text == "ArrayList" || type_text.starts_with("ArrayList<") {
            self.list_sources.insert(var_name.to_string(), ListSource::NewArrayList);
        }
    }

    /// Visit `for (T var : iterable) { ... }` — detect filter → add patterns.
    pub(super) fn visit_enhanced_for_statement(&mut self, node: Node) {
        let loop_var_name = {
            let mut cursor = node.walk();
            let mut found = None;
            for child in node.children(&mut cursor) {
                if child.kind() == "identifier" {
                    found = Some(self.node_text(child));
                    break;
                }
            }
            match found {
                Some(n) => n,
                None => {
                    self.recurse(node);
                    return;
                }
            }
        };

        let body = match node.child_by_field_name("body") {
            Some(n) => n,
            None => {
                self.recurse(node);
                return;
            }
        };

        let mut filter_set_var: Option<String> = None;
        let mut add_list_vars: Vec<String> = Vec::new();

        let mut cursor = body.walk();
        for child in body.children(&mut cursor) {
            match child.kind() {
                "if_statement" => {
                    if let Some(set_var) = self.detect_negated_contains_guard(child, &loop_var_name) {
                        filter_set_var = Some(set_var);
                    }
                }
                "expression_statement" => {
                    if let Some(list_var) = self.detect_list_add_call(child, &loop_var_name) {
                        add_list_vars.push(list_var);
                    }
                }
                "block" => {
                    let mut bc = child.walk();
                    for stmt in child.children(&mut bc) {
                        match stmt.kind() {
                            "if_statement" => {
                                if let Some(set_var) = self.detect_negated_contains_guard(stmt, &loop_var_name) {
                                    filter_set_var = Some(set_var);
                                }
                            }
                            "expression_statement" => {
                                if let Some(list_var) = self.detect_list_add_call(stmt, &loop_var_name) {
                                    add_list_vars.push(list_var);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if let Some(ref set_var) = filter_set_var {
            if self.known_sets.contains_key(set_var) {
                for list_var in &add_list_vars {
                    self.list_sources
                        .insert(list_var.clone(), ListSource::Filtered { source_set_var: set_var.clone() });
                }
            }
        }

        self.recurse(node);
    }

    /// Visit `if (condition) { ... }` with boolean tracking.
    /// When condition is `!var` and `var` is tracked as `true`, skip the consequence.
    pub(super) fn visit_if_statement(&mut self, node: Node) {
        let condition = node.child_by_field_name("condition");
        let consequence = node.child_by_field_name("consequence");
        let alternative = node.child_by_field_name("alternative");

        let mut should_visit_consequence = true;

        if let Some(cond) = condition {
            // Unwrap parenthesized_expression (tree-sitter includes `( )` in condition)
            let inner = if cond.kind() == "parenthesized_expression" { cond.named_child(0) } else { Some(cond) };
            if let Some(expr) = inner {
                if expr.kind() == "unary_expression" {
                    let has_bang =
                        expr.child_by_field_name("operator").map(|op| self.node_text(op) == "!").unwrap_or(false);
                    if has_bang {
                        if let Some(operand) = expr.child_by_field_name("operand") {
                            if operand.kind() == "identifier" {
                                let var_name = self.node_text(operand);
                                if let Some(&val) = self.local_bool_vars.get(&var_name) {
                                    should_visit_consequence = !val;
                                }
                            }
                        }
                    }
                } else if expr.kind() == "method_invocation" {
                    if let Some((var_name, suffix)) = self.try_extract_ends_with_pattern(expr) {
                        let skip =
                            self.pending_string_vars.get(&var_name).is_some_and(|v| !v.ends_with(suffix.as_str()));
                        if skip {
                            should_visit_consequence = false;
                        }
                    }
                }
            }
        }

        if should_visit_consequence {
            if let Some(cons) = consequence {
                self.visit(cons);
            }
        }
        if let Some(alt) = alternative {
            self.visit(alt);
        }
    }

    /// Check if an if_statement matches: `if (!setVar.contains(loopVar.method())) throw ...`
    fn detect_negated_contains_guard(&self, if_node: Node, loop_var: &str) -> Option<String> {
        let condition = if_node.child_by_field_name("condition")?;
        let consequence = if_node.child_by_field_name("consequence")?;

        // consequence must be a throw_statement (possibly wrapped in a block with braces)
        let is_throw = consequence.kind() == "throw_statement"
            || (consequence.kind() == "block"
                && consequence.named_child_count() == 1
                && consequence.named_child(0).is_some_and(|c| c.kind() == "throw_statement"));
        if !is_throw {
            return None;
        }

        // condition must be: !expr or expr is a negated expression
        let inner = if condition.kind() == "unary_expression" {
            let op = condition.child_by_field_name("operator")?;
            if self.node_text(op) != "!" {
                return None;
            }
            condition.child_by_field_name("operand")?
        } else if condition.kind() == "parenthesized_expression" {
            // Check inside parens for negation pattern
            let inner_expr = condition.named_child(0)?;
            if inner_expr.kind() == "unary_expression" {
                let op = inner_expr.child_by_field_name("operator")?;
                if self.node_text(op) != "!" {
                    return None;
                }
                inner_expr.child_by_field_name("operand")?
            } else {
                return None;
            }
        } else {
            return None;
        };

        if inner.kind() != "method_invocation" {
            return None;
        }
        let name = match inner.child_by_field_name("name") {
            Some(n) => self.node_text(n),
            None => return None,
        };
        if name != "contains" {
            return None;
        }
        let obj = inner.child_by_field_name("object")?;
        if obj.kind() != "identifier" {
            return None;
        }
        let set_var = self.node_text(obj);

        // Check first argument starts with loop_var (e.g., `entry.getKey()`)
        let args = inner.child_by_field_name("arguments")?;
        let first_arg = self.nth_arg_node(&args, 0)?;
        let arg_text = self.node_text(first_arg);
        if !arg_text.starts_with(loop_var) {
            return None;
        }

        Some(set_var)
    }

    /// Check if an expression_statement matches: `listVar.add(loopVar.method())`
    fn detect_list_add_call(&self, stmt: Node, loop_var: &str) -> Option<String> {
        if stmt.kind() != "expression_statement" {
            return None;
        }
        let expr = stmt.named_child(0)?;
        if expr.kind() != "method_invocation" {
            return None;
        }
        let name = match expr.child_by_field_name("name") {
            Some(n) => self.node_text(n),
            None => return None,
        };
        if name != "add" {
            return None;
        }
        let obj = expr.child_by_field_name("object")?;
        if obj.kind() != "identifier" {
            return None;
        }
        let list_var = self.node_text(obj);

        // Check first argument starts with loop_var
        let args = expr.child_by_field_name("arguments")?;
        let first_arg = self.nth_arg_node(&args, 0)?;
        let arg_text = self.node_text(first_arg);
        if !arg_text.starts_with(loop_var) {
            return None;
        }

        Some(list_var)
    }

    /// Extract the n-th named argument from an argument_list node.
    fn nth_arg_node<'b>(&self, args: &Node<'b>, index: u32) -> Option<Node<'b>> {
        let mut expr_idx = 0u32;
        for i in 0..args.child_count() {
            if let Some(child) = args.child(i) {
                if child.is_named() {
                    if expr_idx == index {
                        return Some(child);
                    }
                    expr_idx += 1;
                }
            }
        }
        None
    }

    // ── Expression evaluation ──

    /// Try to evaluate any node to a concrete string value.
    pub(super) fn try_evaluate_to_string(&self, node: Node) -> Option<String> {
        match node.kind() {
            "string_literal" => {
                let raw = self.node_text(node);
                Some(self.decode_java_string(&raw, raw.starts_with("\"\"\"")))
            }
            "identifier" => {
                let name = self.node_text(node);
                if let Some(val) = self.string_constants.get(&name) {
                    return Some(val.clone());
                }
                if let Some(val) = self.string_exprs.get(&name) {
                    return Some(val.clone());
                }
                None
            }
            "method_invocation" => self.try_evaluate_method_result(node),
            _ => None,
        }
    }

    /// Evaluate a method invocation to a string result.
    pub(super) fn try_evaluate_method_result(&self, node: Node) -> Option<String> {
        let name = match node.child_by_field_name("name") {
            Some(n) => self.node_text(n),
            None => return None,
        };
        match name.as_str() {
            "join" => self.evaluate_string_join(node),
            "nCopies" => self.evaluate_ncopies_string(node),
            _ => None,
        }
    }

    /// `String.join(delim, iterable)` → evaluate to joined string.
    fn evaluate_string_join(&self, node: Node) -> Option<String> {
        let args = node.child_by_field_name("arguments")?;
        let delim = self.try_eval_string_arg_node(&args, 0)?;
        let elements = self.try_evaluate_collection_from_arg(&args, 1)?;
        Some(elements.join(&delim))
    }

    /// `Collections.nCopies(count, element)` — evaluate to comma-separated string.
    fn evaluate_ncopies_string(&self, node: Node) -> Option<String> {
        let args = node.child_by_field_name("arguments")?;
        let count = self.resolve_int_arg_node(&args, 0)?;
        let element = self.try_eval_string_arg_node(&args, 1)?;
        let parts: Vec<&str> = std::iter::repeat(element.as_str()).take(count).collect();
        Some(parts.join(", "))
    }

    /// Try to evaluate a method argument to a Vec<String> (collection).
    fn try_evaluate_collection_from_arg(&self, args: &Node, arg_idx: u32) -> Option<Vec<String>> {
        let source = self.nth_arg_node(args, arg_idx)?;
        match source.kind() {
            "identifier" => {
                let name = self.node_text(source);
                if let Some(src) = self.list_sources.get(&name) {
                    match src {
                        ListSource::Filtered { source_set_var } => {
                            return self.known_sets.get(source_set_var).cloned();
                        }
                        ListSource::NewArrayList => return None,
                    }
                }
                None
            }
            "method_invocation" => {
                let method_name = match source.child_by_field_name("name") {
                    Some(n) => self.node_text(n),
                    None => return None,
                };
                if method_name == "nCopies" {
                    let inner_args = source.child_by_field_name("arguments")?;
                    let count = self.resolve_int_arg_node(&inner_args, 0)?;
                    let element = self.try_eval_string_arg_node(&inner_args, 1)?;
                    Some(vec![element; count])
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Resolve an argument to an integer value (literal or `var.size()` → known_set size).
    fn resolve_int_arg_node(&self, args: &Node, arg_idx: u32) -> Option<usize> {
        let node = self.nth_arg_node(args, arg_idx)?;
        match node.kind() {
            "decimal_integer_literal" => self.node_text(node).parse::<usize>().ok(),
            "method_invocation" => {
                let name = match node.child_by_field_name("name") {
                    Some(n) => self.node_text(n),
                    None => return None,
                };
                if name != "size" {
                    return None;
                }
                let obj = node.child_by_field_name("object")?;
                if obj.kind() != "identifier" {
                    return None;
                }
                let obj_name = self.node_text(obj);
                if let Some(src) = self.list_sources.get(&obj_name) {
                    match src {
                        ListSource::Filtered { source_set_var } => {
                            return self.known_sets.get(source_set_var).map(|s| s.len());
                        }
                        _ => return None,
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Extract a string literal argument value.
    fn try_eval_string_arg_node(&self, args: &Node, arg_idx: u32) -> Option<String> {
        let node = self.nth_arg_node(args, arg_idx)?;
        if node.kind() == "string_literal" {
            let raw = self.node_text(node);
            Some(self.decode_java_string(&raw, raw.starts_with("\"\"\"")))
        } else {
            self.try_evaluate_to_string(node)
        }
    }
}

pub(crate) fn starts_with_sql_keyword(sql: &str) -> bool {
    let first_word = sql.split_whitespace().next().unwrap_or("");
    let upper = first_word.to_uppercase();
    matches!(upper.as_str(), "SELECT" | "INSERT" | "UPDATE" | "DELETE" | "MERGE" | "WITH")
}

fn sanitize_var_name(var_name: &str) -> String {
    let mapped: String =
        var_name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' }).collect();
    let mut result = String::with_capacity(mapped.len());
    let mut prev_underscore = false;
    for c in mapped.chars() {
        if c == '_' {
            if !prev_underscore {
                result.push(c);
            }
            prev_underscore = true;
        } else {
            result.push(c);
            prev_underscore = false;
        }
    }
    while result.ends_with('_') {
        result.pop();
    }
    if result.is_empty() {
        "_".to_string()
    } else {
        result
    }
}
