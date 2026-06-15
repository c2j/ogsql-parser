//! Method call SQL extraction for Java source files.

use tree_sitter::Node;

use super::constant::{JDBC_SETTER_TYPES, SQL_METHOD_AMBIGUOUS, SQL_METHOD_UNAMBIGUOUS};
use super::extract::ExtractContext;
use super::heuristics::{detect_parameter_style, looks_like_sql};
use super::types::{ExtractedSql, ExtractionMethod, JdbcParamInfo, MethodPsBehavior, SqlKind, SqlOrigin};

pub(super) struct PendingInjection {
    pub(super) ps_var: String,
    pub(super) method_name: String,
    pub(super) arg_count: usize,
    pub(super) extraction_idx: Option<usize>,
    pub(super) call_arg_names: Vec<Option<String>>,
}

impl<'a> ExtractContext<'a> {
    pub(super) fn visit_method_invocation(&mut self, node: Node) {
        let name_node = match node.child_by_field_name("name") {
            Some(n) => n,
            None => return,
        };
        let method_name = self.node_text(name_node);

        if method_name == "append" || method_name == "insert" || method_name == "delete" || method_name == "replace" {
            if let Some(root_var) = self.find_method_chain_root(node) {
                if self.sql_vars.contains_key(&root_var)
                    && self.sql_vars.get(&root_var).is_some_and(|v| v.is_string_builder)
                {
                    self.handle_string_builder_call(node, &root_var, &method_name);
                    return;
                }
            }
        }

        // Intercept JDBC setter calls (setString, setInt, etc.)
        if method_name.starts_with("set") && method_name.len() > 3 {
            let next_char = method_name.chars().nth(3).expect("method_name.len() > 3 checked above");
            if next_char.is_ascii_uppercase() {
                if let Some(root_var) = self.find_method_chain_root(node) {
                    let is_ps = self.ps_var_to_extraction.contains_key(&root_var)
                        || self.var_types.get(&root_var).is_some_and(|t| t == "PreparedStatement");
                    if is_ps {
                        self.visit_setter_call(node, &method_name);
                        return;
                    }
                }
            }
        }

        let is_unambiguous = SQL_METHOD_UNAMBIGUOUS.contains(&method_name.as_str())
            || self.extra_sql_methods.iter().any(|m| m == &method_name);
        let is_ambiguous = SQL_METHOD_AMBIGUOUS.contains(&method_name.as_str());

        if let Some(args_node) = node.child_by_field_name("arguments") {
            if args_node.kind() == "argument_list" {
                let mut cursor = args_node.walk();
                let mut found_ps_var: Option<String> = None;
                let mut arg_count = 0usize;
                let mut call_arg_names: Vec<Option<String>> = Vec::new();
                for child in args_node.children(&mut cursor) {
                    if child.kind() == "," || child.kind() == "(" || child.kind() == ")" {
                        continue;
                    }
                    arg_count += 1;
                    let arg_name = match child.kind() {
                        "identifier" => Some(self.node_text(child)),
                        "field_access" => child.child_by_field_name("field").map(|n| self.node_text(n)),
                        _ => None,
                    };
                    if let Some(ref name) = arg_name {
                        if self.ps_var_to_extraction.contains_key(name)
                            || self.var_types.get(name).is_some_and(|t| t == "PreparedStatement")
                        {
                            found_ps_var = Some(name.clone());
                        }
                    }
                    call_arg_names.push(arg_name);
                }
                if let Some(ps_var) = found_ps_var {
                    let ext_idx = self.ps_var_to_extraction.get(&ps_var).copied();
                    self.pending_injections.push(PendingInjection {
                        ps_var,
                        method_name: method_name.clone(),
                        arg_count,
                        extraction_idx: ext_idx,
                        call_arg_names,
                    });
                }
            }
        }

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

        let tracked_sql_idx = {
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
        };

        let mut pushed_extraction_idx: Option<usize> = None;

        if let Some((sql_text, is_text_block)) = self.find_sql_arg(&args_node) {
            let is_sql = looks_like_sql(&sql_text);

            if !is_sql && tracked_sql_idx.is_some() {
            } else {
                if !is_unambiguous && !is_sql {
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
        }

        let backfill_target = pushed_extraction_idx.or(tracked_sql_idx);

        if let Some(array_elements) = self.find_array_creation_arg(&args_node) {
            if let Some(ext_idx) = backfill_target {
                self.backfill_array_params(ext_idx, &array_elements);
            }
        }

        // Track PreparedStatement variable → extraction mapping
        if method_name == "prepareStatement" || method_name == "prepareCall" {
            if let Some(target_var) = self.find_assignment_target(node) {
                let extraction_idx = match pushed_extraction_idx {
                    Some(idx) => Some(idx),
                    None => {
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
                            if child.kind() == "method_invocation" {
                                if let Some(name_n) = child.child_by_field_name("name") {
                                    if self.node_text(name_n) == "toString" {
                                        if let Some(obj) = child.child_by_field_name("object") {
                                            if obj.kind() == "identifier" {
                                                let arg_name = self.node_text(obj);
                                                if let Some(tracked) = self.sql_vars.get(&arg_name) {
                                                    found = Some(tracked.extraction_index);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        found
                    }
                };
                if let Some(idx) = extraction_idx {
                    if let Some(&old_idx) = self.ps_var_to_extraction.get(&target_var) {
                        self.backfill_for_ps_var(&target_var, old_idx);
                    }
                    if let Some(ext) = self.extractions.get_mut(idx) {
                        let mut param_num = 0usize;
                        let mut result = String::with_capacity(ext.sql.len() + 200);
                        let mut in_quote = false;
                        for c in ext.sql.chars() {
                            if c == '\'' {
                                in_quote = !in_quote;
                                result.push(c);
                            } else if c == '?' && !in_quote {
                                param_num += 1;
                                result.push_str(&format!("__JAVA_VAR_JDBC_PARAM_{}__", param_num));
                            } else {
                                result.push(c);
                            }
                        }
                        if param_num > 0 {
                            ext.sql = result;
                        }
                    }
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
                "identifier" => {
                    let name = self.node_text(child);
                    if let Some(val) = self.string_constants.get(&name) {
                        return Some((val.clone(), false));
                    }
                }
                "field_access" => {
                    let last_ident = child.child_by_field_name("field").or_else(|| {
                        let mut c = child.walk();
                        child.children(&mut c).filter(|n| n.kind() == "identifier").last()
                    });
                    if let Some(n) = last_ident {
                        let name = self.node_text(n);
                        if let Some(val) = self.string_constants.get(&name) {
                            return Some((val.clone(), false));
                        }
                    }
                }
                _ => continue,
            }
        }
        None
    }

    fn find_sql_arg(&self, args_node: &Node) -> Option<(String, bool)> {
        let mut first_string: Option<(String, bool)> = None;
        let mut cursor = args_node.walk();
        for child in args_node.children(&mut cursor) {
            match child.kind() {
                "string_literal" | "binary_expression" => {
                    if let Some((text, is_tb)) = self.extract_string_value(child) {
                        if looks_like_sql(&text) {
                            return Some((text, is_tb));
                        }
                        if first_string.is_none() {
                            first_string = Some((text, is_tb));
                        }
                    }
                }
                _ => continue,
            }
        }
        first_string
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

        let idx_text = self.node_text(args[0]);
        let param_index: usize = match idx_text.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                let ps_var = match self.find_method_chain_root(node) {
                    Some(v) => v,
                    None => return,
                };
                self.dynamic_setters.push((ps_var, java_type));
                return;
            }
        };

        let value_node = args[1];
        let var_name = self.extract_setter_value_name(value_node);

        let ps_var = match self.find_method_chain_root(node) {
            Some(v) => v,
            None => return,
        };

        self.jdbc_param_map.insert((ps_var, param_index), JdbcParamInfo { index: param_index, java_type, var_name });
    }

    fn extract_setter_value_name(&self, node: Node) -> Option<String> {
        match node.kind() {
            "identifier" => Some(self.node_text(node)),
            "field_access" => {
                let text = self.node_text(node);
                let parts: Vec<&str> = text.rsplitn(2, '.').collect();
                Some(parts[0].to_string())
            }
            "method_invocation" => {
                if let Some(obj) = node.child_by_field_name("object") {
                    match obj.kind() {
                        "identifier" => return Some(self.node_text(obj)),
                        "field_access" => {
                            if let Some(field) = obj.child_by_field_name("field") {
                                return Some(self.node_text(field));
                            }
                        }
                        _ => {}
                    }
                }
                node.child_by_field_name("name").map(|n| self.node_text(n))
            }
            "object_creation_expression" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "argument_list" {
                        let mut ac = child.walk();
                        for arg in child.children(&mut ac) {
                            if arg.kind() == "," || arg.kind() == "(" || arg.kind() == ")" {
                                continue;
                            }
                            return self.extract_setter_value_name(arg);
                        }
                    }
                }
                None
            }
            "binary_expression" => {
                let mut found = None;
                let mut stack = vec![node];
                while let Some(current) = stack.pop() {
                    let mut cursor = current.walk();
                    for child in current.children(&mut cursor) {
                        match child.kind() {
                            "identifier" => {
                                found = Some(self.node_text(child));
                                break;
                            }
                            "binary_expression" => stack.push(child),
                            _ => {}
                        }
                    }
                    if found.is_some() {
                        break;
                    }
                }
                found
            }
            _ => None,
        }
    }

    fn find_array_creation_arg(&self, args_node: &Node) -> Option<Vec<(String, Option<String>)>> {
        let mut cursor = args_node.walk();
        for child in args_node.children(&mut cursor) {
            if child.kind() == "array_creation_expression" {
                return Some(self.extract_array_element_names(child));
            }
        }
        None
    }

    fn extract_array_element_names(&self, node: Node) -> Vec<(String, Option<String>)> {
        let mut elements = Vec::new();
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "dimensions"
                || child.kind() == "type_identifier"
                || child.kind() == "integral_type"
                || child.kind() == "primitive_type"
                || child.kind() == "floating_point_type"
                || child.kind() == "boolean_type"
                || child.kind() == "generic_type"
                || child.kind() == "array_type"
            {
                continue;
            }
            if child.kind() == "dimensions" {
                continue;
            }
            if child.kind() == "element_list" || child.kind() == "array_initializer" {
                let mut el_cursor = child.walk();
                for el in child.children(&mut el_cursor) {
                    if el.kind() == "," || el.kind() == "{" || el.kind() == "}" {
                        continue;
                    }
                    match el.kind() {
                        "identifier" => {
                            let name = self.node_text(el);
                            let inferred = self.var_types.get(&name).cloned();
                            elements.push((name, inferred));
                        }
                        _ => {
                            let placeholder = self.make_placeholder_for_node(el);
                            elements.push((placeholder, None));
                        }
                    }
                }
            }
        }
        elements
    }

    fn backfill_array_params(&mut self, ext_idx: usize, elements: &[(String, Option<String>)]) {
        let mut modified = false;
        if let Some(ext) = self.extractions.get_mut(ext_idx) {
            for (i, (var_name, inferred_type)) in elements.iter().enumerate() {
                let param_num = i + 1;
                let old = format!("__JAVA_VAR_JDBC_PARAM_{}__", param_num);
                let java_type = inferred_type
                    .as_deref()
                    .or_else(|| self.var_types.get(var_name).map(|s| s.as_str()))
                    .unwrap_or("Object");
                let sanitized = var_name
                    .chars()
                    .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
                    .collect::<String>();
                let new = format!("__JAVA_VAR_{}_{}__", java_type, sanitized);
                let before = ext.sql.len();
                ext.sql = ext.sql.replace(&old, &new);
                modified |= ext.sql.len() != before;
            }
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

    #[allow(dead_code)]
    fn resolve_ps_arg_extraction(&self, args_node: &Node, behavior: &MethodPsBehavior) -> Option<usize> {
        let mut cursor = args_node.walk();
        let mut arg_idx = 0usize;
        for child in args_node.children(&mut cursor) {
            if child.kind() == "," || child.kind() == "(" || child.kind() == ")" {
                continue;
            }
            if arg_idx == behavior.ps_param_index {
                if child.kind() == "identifier" {
                    let ps_var = self.node_text(child);
                    return self.ps_var_to_extraction.get(&ps_var).copied();
                }
                return None;
            }
            arg_idx += 1;
        }
        None
    }

    fn apply_fallback_dynamic(&mut self, ext_idx: usize) {
        let max_param = {
            let ext = match self.extractions.get(ext_idx) {
                Some(e) => e,
                None => return,
            };
            let mut count = 0usize;
            let mut pos = 0;
            while let Some(i) = ext.sql[pos..].find("__JAVA_VAR_JDBC_PARAM_") {
                let abs_pos = pos + i;
                let rest = match ext.sql.get(abs_pos + 22..) {
                    Some(r) => r,
                    None => break,
                };
                if let Some(end_off) = rest.find("__") {
                    if let Ok(param_num) = rest[..end_off].parse::<usize>() {
                        count = count.max(param_num);
                    }
                }
                pos = abs_pos + 22;
            }
            count
        };
        if max_param == 0 {
            return;
        }
        let mut modified = false;
        for n in 1..=max_param {
            let old = format!("__JAVA_VAR_JDBC_PARAM_{}__", n);
            let new = format!("__JAVA_VAR_String_DYNAMIC_{}__", n);
            if let Some(ext) = self.extractions.get_mut(ext_idx) {
                let before = ext.sql.len();
                ext.sql = ext.sql.replace(&old, &new);
                modified |= ext.sql.len() != before;
            }
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

    pub(super) fn apply_pending_injections(&mut self) {
        let injections = std::mem::take(&mut self.pending_injections);
        for injection in injections {
            let ext_idx = match injection.extraction_idx {
                Some(idx) => idx,
                None => match self.class_ps_to_extraction.get(&injection.ps_var) {
                    Some(&idx) => idx,
                    None => continue,
                },
            };
            let behavior_key = format!("{}:{}", injection.method_name, injection.arg_count);
            let behavior = self
                .method_behaviors
                .get(&behavior_key)
                .cloned()
                .or_else(|| self.method_behaviors.get(&injection.method_name).cloned());
            let behavior = match behavior {
                Some(b) => b,
                None => {
                    self.apply_fallback_dynamic(ext_idx);
                    continue;
                }
            };

            let mut modified = false;
            for pattern in &behavior.setter_patterns {
                match pattern {
                    super::types::SetterPattern::Literal { index, java_type, var_name, param_index } => {
                        let old = format!("__JAVA_VAR_JDBC_PARAM_{}__", index);
                        let resolved_name = var_name.as_ref().and_then(|name| {
                            param_index.and_then(|pidx| {
                                injection
                                    .call_arg_names
                                    .get(pidx)
                                    .and_then(|n| n.as_ref().map(|s| s.as_str()))
                                    .filter(|actual| actual != &name.as_str())
                            })
                        });
                        let resolved_name = resolved_name.or_else(|| var_name.as_ref().map(|n| n.as_str()));
                        let new = match resolved_name {
                            Some(name) => {
                                let sanitized = name
                                    .chars()
                                    .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
                                    .collect::<String>();
                                format!("__JAVA_VAR_{}_{}__", java_type, sanitized)
                            }
                            None => format!("__JAVA_VAR_{}_JDBC_PARAM_{}__", java_type, index),
                        };
                        if let Some(ext) = self.extractions.get_mut(ext_idx) {
                            let before = ext.sql.len();
                            ext.sql = ext.sql.replace(&old, &new);
                            modified |= ext.sql.len() != before;
                        }
                    }
                    super::types::SetterPattern::DynamicLoop { java_type } => {
                        let max_param = {
                            let ext = match self.extractions.get(ext_idx) {
                                Some(e) => e,
                                None => continue,
                            };
                            let mut count = 0usize;
                            let mut pos = 0;
                            while let Some(i) = ext.sql[pos..].find("__JAVA_VAR_JDBC_PARAM_") {
                                let abs_pos = pos + i;
                                let rest = &ext.sql[abs_pos + 22..];
                                if let Some(end_off) = rest.find("__") {
                                    if let Ok(param_num) = rest[..end_off].parse::<usize>() {
                                        count = count.max(param_num);
                                    }
                                }
                                pos = abs_pos + 22;
                            }
                            count
                        };
                        for n in 1..=max_param {
                            let old = format!("__JAVA_VAR_JDBC_PARAM_{}__", n);
                            let new = format!("__JAVA_VAR_{}_DYNAMIC_{}__", java_type, n);
                            if let Some(ext) = self.extractions.get_mut(ext_idx) {
                                let before = ext.sql.len();
                                ext.sql = ext.sql.replace(&old, &new);
                                modified |= ext.sql.len() != before;
                            }
                        }
                    }
                }
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
    }
}
