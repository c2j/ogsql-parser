use crate::ast::plpgsql::PlUsingMode;
use crate::ast::*;

pub struct SqlFormatter {
    indent: usize,
    uppercase_keywords: bool,
}

impl SqlFormatter {
    pub fn new() -> Self {
        Self {
            indent: 0,
            uppercase_keywords: true,
        }
    }

    pub fn format_statement(&self, stmt: &Statement) -> String {
        match stmt {
            Statement::Select(s) => self.format_select(s),
            Statement::Insert(s) => self.format_insert(s),
            Statement::InsertAll(s) => self.format_insert_all(s),
            Statement::InsertFirst(s) => self.format_insert_first(s),
            Statement::Update(s) => self.format_update(s),
            Statement::Delete(s) => self.format_delete(s),
            Statement::Merge(s) => self.format_merge(s),
            Statement::CreateTable(s) => self.format_create_table(s),
            Statement::AlterTable(s) => self.format_alter_table(s),
            Statement::Drop(s) => self.format_drop(s),
            Statement::Truncate(s) => self.format_truncate(s),
            Statement::CreateIndex(s) => self.format_create_index(s),
            Statement::CreateSchema(s) => self.format_create_schema(s),
            Statement::CreateDatabase(s) => self.format_create_database(s),
            Statement::CreateTablespace(s) => self.format_create_tablespace(s),
            Statement::CreateView(s) => self.format_create_view(s),
            Statement::CreateSequence(s) => self.format_create_sequence(s),
            Statement::Transaction(s) => self.format_transaction(s),
            Statement::Copy(s) => self.format_copy(s),
            Statement::Explain(s) => self.format_explain(s),
            Statement::VariableSet(s) => self.format_variable_set(s),
            Statement::VariableShow(s) => self.format_variable_show(s),
            Statement::VariableReset(s) => self.format_variable_reset(s),
            Statement::Discard(s) => self.format_discard(s),
            Statement::Call(s) => self.format_call(s),
            Statement::CreateFdw(s) => self.format_create_fdw(s),
            Statement::CreateForeignServer(s) => self.format_create_foreign_server(s),
            Statement::CreateForeignTable(s) => self.format_create_foreign_table(s),
            Statement::CreatePublication(s) => self.format_create_publication(s),
            Statement::CreateSubscription(s) => self.format_create_subscription(s),
            Statement::CreateNode(s) => self.format_create_node(s),
            Statement::CreateNodeGroup(s) => self.format_create_node_group(s),
            Statement::CreateResourcePool(s) => self.format_create_resource_pool(s),
            Statement::CreateWorkloadGroup(s) => self.format_create_workload_group(s),
            Statement::CreateAuditPolicy(s) => self.format_create_audit_policy(s),
            Statement::CreateMaskingPolicy(s) => self.format_create_masking_policy(s),
            Statement::CreateRlsPolicy(s) => self.format_create_rls_policy(s),
            Statement::Empty => String::new(),
            Statement::Checkpoint => self.kw("CHECKPOINT"),
            Statement::Grant(s) => self.format_grant(s),
            Statement::Revoke(s) => self.format_revoke(s),
            Statement::Vacuum(s) => self.format_vacuum(s),
            Statement::Analyze(s) => self.format_analyze(s),
            Statement::Do(s) => self.format_do(s),
            Statement::Prepare(s) => self.format_prepare(s),
            Statement::Execute(s) => self.format_execute(s),
            Statement::Deallocate(s) => self.format_deallocate(s),
            Statement::Comment(s) => self.format_comment(s),
            Statement::Lock(s) => self.format_lock(s),
            Statement::DeclareCursor(s) => self.format_declare_cursor(s),
            Statement::ClosePortal(s) => self.format_close_portal(s),
            Statement::Fetch(s) => self.format_fetch(s),
            Statement::Cluster(s) => self.format_cluster(s),
            Statement::Reindex(s) => self.format_reindex(s),
            Statement::Listen(s) => self.format_listen(s),
            Statement::Notify(s) => self.format_notify(s),
            Statement::Unlisten(s) => self.format_unlisten(s),
            Statement::Rule(s) => self.format_rule(s),
            Statement::CreateTrigger(s) => self.format_create_trigger(s),
            Statement::CreateMaterializedView(s) => self.format_create_materialized_view(s),
            Statement::RefreshMaterializedView(s) => self.format_refresh_matview(s),
            Statement::AlterDatabase(s) => self.format_alter_database(s),
            Statement::AlterSchema(s) => self.format_alter_schema(s),
            Statement::AlterSequence(s) => self.format_alter_sequence(s),
            Statement::AlterFunction(s) => self.format_alter_function(s),
            Statement::AlterProcedure(s) => self.format_alter_function_action(&s.action),
            Statement::AlterRole(s) => self.format_alter_role(s),
            Statement::AlterUser(s) => self.format_alter_user(s),
            Statement::AlterGlobalConfig(s) => self.format_alter_global_config(s),
            Statement::CreateFunction(s) => self.format_create_function(s),
            Statement::CreateProcedure(s) => self.format_create_procedure(s),
            Statement::CreatePackage(s) => self.format_create_package(s),
            Statement::CreatePackageBody(s) => self.format_create_package_body(s),
            Statement::CreateExtension(s) => self.format_create_extension(s),
            Statement::CreateRole(s) => self.format_create_role(s),
            Statement::CreateUser(s) => self.format_create_user(s),
            Statement::CreateGroup(s) => self.format_create_group(s),
            Statement::CreateCast(s) => self.format_create_cast(s),
            Statement::CreateDomain(s) => self.format_create_domain(s),
            Statement::GrantRole(s) => self.format_grant_role(s),
            Statement::RevokeRole(s) => self.format_revoke_role(s),
            Statement::AlterGroup(s) => self.format_alter_group(s),
            Statement::AlterCompositeType(s) => self.format_alter_composite_type(s),
            Statement::AlterView(s) => self.format_alter_view(s),
            Statement::AlterTrigger(s) => self.format_alter_trigger(s),
            Statement::AlterExtension(s) => self.format_alter_extension(s),
            Statement::AnonyBlock(s) => self.format_anon_block(s),
            _ => self.format_stub(stmt),
        }
    }

    fn kw(&self, s: &str) -> String {
        if self.uppercase_keywords {
            s.to_uppercase()
        } else {
            s.to_lowercase()
        }
    }

    fn format_stub(&self, stmt: &Statement) -> String {
        let name = match stmt {
            Statement::CreateConversion(_) => "CREATE CONVERSION",
            Statement::AlterDomain(_) => "ALTER DOMAIN",
            Statement::CreateSynonym(_) => "CREATE SYNONYM",
            Statement::CreateModel(_) => "CREATE MODEL",
            Statement::CreateAm(_) => "CREATE ACCESS METHOD",
            Statement::CreateDirectory(_) => "CREATE DIRECTORY",
            Statement::CreateDataSource(_) => "CREATE DATA SOURCE",
            Statement::CreateEvent(_) => "CREATE EVENT",
            Statement::CreateOpClass(_) => "CREATE OPERATOR CLASS",
            Statement::CreateOpFamily(_) => "CREATE OPERATOR FAMILY",
            Statement::CreateContQuery(_) => "CREATE CONTINUOUS QUERY",
            Statement::CreateStream(_) => "CREATE STREAM",
            Statement::CreateKey(_) => "CREATE KEY",
            Statement::AlterForeignTable(_) => "ALTER FOREIGN TABLE",
            Statement::AlterForeignServer(_) => "ALTER SERVER",
            Statement::AlterFdw(_) => "ALTER FOREIGN DATA WRAPPER",
            Statement::AlterPublication(_) => "ALTER PUBLICATION",
            Statement::AlterSubscription(_) => "ALTER SUBSCRIPTION",
            Statement::AlterNode(_) => "ALTER NODE",
            Statement::AlterNodeGroup(_) => "ALTER NODE GROUP",
            Statement::AlterResourcePool(_) => "ALTER RESOURCE POOL",
            Statement::AlterWorkloadGroup(_) => "ALTER WORKLOAD GROUP",
            Statement::AlterAuditPolicy(_) => "ALTER AUDIT POLICY",
            Statement::AlterMaskingPolicy(_) => "ALTER MASKING POLICY",
            Statement::AlterRlsPolicy(_) => "ALTER RLS POLICY",
            Statement::AlterDataSource(_) => "ALTER DATA SOURCE",
            Statement::AlterEvent(_) => "ALTER EVENT",
            Statement::AlterOpFamily(_) => "ALTER OPERATOR FAMILY",
            Statement::Shutdown(_) => "SHUTDOWN",
            Statement::Barrier(_) => "BARRIER",
            Statement::Purge(_) => "PURGE",
            Statement::TimeCapsule(_) => "TIME CAPSULE",
            Statement::Snapshot(_) => "SNAPSHOT",
            Statement::Shrink(_) => "SHRINK",
            Statement::Verify(_) => "VERIFY",
            Statement::CleanConn(_) => "CLEAN CONNECTION",
            Statement::Compile(_) => "COMPILE",
            Statement::GetDiag(_) => "GET DIAGNOSTICS",
            Statement::ShowEvent(_) => "SHOW EVENT",
            Statement::RemovePackage(_) => "REMOVE PACKAGE",
            Statement::SecLabel(_) => "SECURITY LABEL",
            Statement::CreateWeakPasswordDictionary => "CREATE WEAK PASSWORD DICTIONARY",
            Statement::DropWeakPasswordDictionary => "DROP WEAK PASSWORD DICTIONARY",
            Statement::CreatePolicyLabel(_) => "CREATE POLICY LABEL",
            Statement::AlterPolicyLabel(_) => "ALTER POLICY LABEL",
            Statement::DropPolicyLabel(_) => "DROP POLICY LABEL",
            _ => "UNKNOWN",
        };
        format!("/* stub: {} */", name)
    }

    fn format_select(&self, stmt: &SelectStatement) -> String {
        let mut parts = Vec::new();

        if let Some(with) = &stmt.with {
            parts.push(self.format_with(with));
        }

        let mut select_parts = vec![self.kw("SELECT")];
        if !stmt.hints.is_empty() {
            let hints: Vec<String> = stmt.hints.iter().map(|h| format!("/*+ {} */", h)).collect();
            select_parts.push(hints.join(" "));
        }
        if stmt.distinct {
            select_parts.push(self.kw("DISTINCT"));
        }
        select_parts.push(self.format_select_targets(&stmt.targets));
        parts.push(select_parts.join(" "));

        if let Some(into_targets) = &stmt.into_targets {
            parts.push(format!(
                "{} {}",
                self.kw("INTO"),
                self.format_select_targets(into_targets)
            ));
        }

        if !stmt.from.is_empty() {
            parts.push(format!(
                "{} {}",
                self.kw("FROM"),
                self.format_table_refs(&stmt.from)
            ));
        }

        if let Some(where_clause) = &stmt.where_clause {
            parts.push(format!(
                "{} {}",
                self.kw("WHERE"),
                self.format_expr(where_clause)
            ));
        }

        if let Some(cb) = &stmt.connect_by {
            if let Some(sw) = &cb.start_with {
                parts.push(format!(
                    "{} {}",
                    self.kw("START WITH"),
                    self.format_expr(sw)
                ));
            }
            let nocycle = if cb.nocycle { " NOCYCLE" } else { "" };
            parts.push(format!(
                "{}{} {}",
                self.kw("CONNECT BY"),
                nocycle,
                self.format_expr(&cb.condition)
            ));
        }

        if !stmt.group_by.is_empty() {
            parts.push(format!(
                "{} {}",
                self.kw("GROUP BY"),
                stmt.group_by
                    .iter()
                    .map(|item| self.format_group_by_item(item))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        if let Some(having) = &stmt.having {
            parts.push(format!(
                "{} {}",
                self.kw("HAVING"),
                self.format_expr(having)
            ));
        }

        let mut result = parts.join(" ");

        if let Some(set_op) = &stmt.set_operation {
            let (op_name, all, right) = match set_op {
                SetOperation::Union { all, right } => ("UNION", all, right),
                SetOperation::Intersect { all, right } => ("INTERSECT", all, right),
                SetOperation::Except { all, right } => ("EXCEPT", all, right),
            };
            let mut op_str = self.kw(op_name);
            if *all {
                op_str.push_str(" ALL");
            }
            result = format!("{} {} {}", result, op_str, self.format_select(right));
        }

        let mut final_parts = vec![result];

        if !stmt.order_by.is_empty() {
            final_parts.push(format!(
                "{} {}",
                self.kw("ORDER BY"),
                self.format_order_by(&stmt.order_by)
            ));
        }

        if let Some(limit) = &stmt.limit {
            final_parts.push(format!("{} {}", self.kw("LIMIT"), self.format_expr(limit)));
        }

        if let Some(offset) = &stmt.offset {
            final_parts.push(format!(
                "{} {}",
                self.kw("OFFSET"),
                self.format_expr(offset)
            ));
        }

        final_parts.join(" ")
    }

    fn format_with(&self, with: &WithClause) -> String {
        let mut parts = vec![self.kw("WITH")];
        if with.recursive {
            parts.push(self.kw("RECURSIVE"));
        }
        let ctes: Vec<String> = with.ctes.iter().map(|c| self.format_cte(c)).collect();
        parts.push(ctes.join(", "));
        parts.join(" ")
    }

    fn format_cte(&self, cte: &Cte) -> String {
        let mut parts = vec![self.quote_identifier(&cte.name)];
        if let Some(mat) = cte.materialized {
            if mat {
                parts.push(self.kw("MATERIALIZED"));
            } else {
                parts.push(self.kw("NOT MATERIALIZED"));
            }
        }
        if !cte.columns.is_empty() {
            let cols: Vec<String> = cte
                .columns
                .iter()
                .map(|c| self.quote_identifier(c))
                .collect();
            parts.push(format!("({})", cols.join(", ")));
        }
        parts.push(self.kw("AS").to_string());
        parts.push(format!("({})", self.format_select(&cte.query)));
        parts.join(" ")
    }

    fn format_select_targets(&self, targets: &[SelectTarget]) -> String {
        let formatted: Vec<String> = targets
            .iter()
            .map(|t| self.format_select_target(t))
            .collect();
        formatted.join(", ")
    }

    fn format_select_target(&self, target: &SelectTarget) -> String {
        match target {
            SelectTarget::Expr(expr, alias) => {
                let mut result = self.format_expr(expr);
                if let Some(a) = alias {
                    result = format!("{} AS {}", result, self.quote_identifier(a));
                }
                result
            }
            SelectTarget::Star(alias) => match alias {
                Some(a) => format!("{}.*", self.quote_identifier(a)),
                None => "*".to_string(),
            },
        }
    }

    fn format_table_refs(&self, refs: &[TableRef]) -> String {
        let formatted: Vec<String> = refs.iter().map(|r| self.format_table_ref(r)).collect();
        formatted.join(", ")
    }

    fn format_table_ref(&self, r: &TableRef) -> String {
        match r {
            TableRef::Table { name, alias } => {
                let mut result = self.format_object_name(name);
                if let Some(a) = alias {
                    result = format!("{} AS {}", result, self.quote_identifier(a));
                }
                result
            }
            TableRef::FunctionCall { name, args, alias } => {
                let args_str: Vec<String> = args.iter().map(|a| self.format_expr(a)).collect();
                let mut result =
                    format!("{}({})", self.format_object_name(name), args_str.join(", "));
                if let Some(a) = alias {
                    result = format!("{} AS {}", result, self.quote_identifier(a));
                }
                result
            }
            TableRef::Subquery { query, alias } => {
                let result = format!("({})", self.format_select(query));
                match alias {
                    Some(a) => format!("{} AS {}", result, self.quote_identifier(a)),
                    None => result,
                }
            }
            TableRef::Join {
                left,
                right,
                join_type,
                condition,
            } => {
                let left_str = self.format_table_ref(left);
                let join_kw = match join_type {
                    JoinType::Inner => self.kw("INNER JOIN"),
                    JoinType::Left => self.kw("LEFT JOIN"),
                    JoinType::Right => self.kw("RIGHT JOIN"),
                    JoinType::Full => self.kw("FULL JOIN"),
                    JoinType::Cross => self.kw("CROSS JOIN"),
                };
                let right_str = self.format_table_ref(right);
                let mut result = format!("{} {} {}", left_str, join_kw, right_str);
                if let Some(cond) = condition {
                    result = format!("{} {} {}", result, self.kw("ON"), self.format_expr(cond));
                }
                result
            }
        }
    }

    fn format_group_by_item(&self, item: &GroupByItem) -> String {
        match item {
            GroupByItem::Expr(e) => self.format_expr(e),
            GroupByItem::GroupingSets(sets) => {
                let inner: Vec<String> = sets
                    .iter()
                    .map(|s| {
                        if s.is_empty() {
                            "()".to_string()
                        } else {
                            format!(
                                "({})",
                                s.iter()
                                    .map(|e| self.format_expr(e))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            )
                        }
                    })
                    .collect();
                format!("{} ({})", self.kw("GROUPING SETS"), inner.join(", "))
            }
            GroupByItem::Rollup(cols) => {
                format!(
                    "{} ({})",
                    self.kw("ROLLUP"),
                    cols.iter()
                        .map(|e| self.format_expr(e))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            GroupByItem::Cube(cols) => {
                format!(
                    "{} ({})",
                    self.kw("CUBE"),
                    cols.iter()
                        .map(|e| self.format_expr(e))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }

    fn format_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(lit) => self.format_literal(lit),
            Expr::ColumnRef(name) => self.format_object_name(name),
            Expr::BinaryOp { left, op, right } => {
                format!(
                    "{} {} {}",
                    self.format_expr(left),
                    op.to_uppercase(),
                    self.format_expr(right)
                )
            }
            Expr::UnaryOp { op, expr } => {
                let op_upper = op.to_uppercase();
                if op == "-" || op == "@" {
                    format!("{}{}", op_upper, self.format_expr(expr))
                } else {
                    format!("{} {}", op_upper, self.format_expr(expr))
                }
            }
            Expr::FunctionCall {
                name,
                args,
                distinct,
                over,
            } => {
                let parts: Vec<String> = name
                    .iter()
                    .map(|s| self.quote_identifier_relaxed(s))
                    .collect();
                let mut result = format!("{}(", parts.join("."));
                if *distinct {
                    result.push_str(&format!("{} ", self.kw("DISTINCT")));
                }
                result.push_str(&self.format_exprs(args));
                result.push(')');
                if let Some(ws) = over {
                    result = format!("{} {}", result, self.format_window_spec(ws));
                }
                result
            }
            Expr::Case {
                operand,
                whens,
                else_expr,
            } => {
                let mut parts = vec![self.kw("CASE")];
                if let Some(op) = operand {
                    parts.push(self.format_expr(op));
                }
                for w in whens {
                    parts.push(format!(
                        "{} {} {}",
                        self.kw("WHEN"),
                        self.format_expr(&w.condition),
                        self.kw("THEN")
                    ));
                    parts.push(self.format_expr(&w.result));
                }
                if let Some(e) = else_expr {
                    parts.push(self.kw("ELSE"));
                    parts.push(self.format_expr(e));
                }
                parts.push(self.kw("END"));
                parts.join(" ")
            }
            Expr::Between {
                expr,
                low,
                high,
                negated,
            } => {
                let not_str = if *negated {
                    format!("{} ", self.kw("NOT"))
                } else {
                    String::new()
                };
                format!(
                    "{} {}{} {} {} {}",
                    self.format_expr(expr),
                    not_str,
                    self.kw("BETWEEN"),
                    self.format_expr(low),
                    self.kw("AND"),
                    self.format_expr(high)
                )
            }
            Expr::InList {
                expr,
                list,
                negated,
            } => {
                let not_str = if *negated {
                    format!("{} ", self.kw("NOT"))
                } else {
                    String::new()
                };
                format!(
                    "{} {}{} ({})",
                    self.format_expr(expr),
                    not_str,
                    self.kw("IN"),
                    self.format_exprs(list)
                )
            }
            Expr::InSubquery {
                expr,
                subquery,
                negated,
            } => {
                let not_str = if *negated {
                    format!("{} ", self.kw("NOT"))
                } else {
                    String::new()
                };
                format!(
                    "{} {}{} ({})",
                    self.format_expr(expr),
                    not_str,
                    self.kw("IN"),
                    self.format_select(subquery)
                )
            }
            Expr::Exists(subquery) => {
                format!("{} ({})", self.kw("EXISTS"), self.format_select(subquery))
            }
            Expr::Subquery(subquery) => {
                format!("({})", self.format_select(subquery))
            }
            Expr::IsNull { expr, negated } => {
                let not_str = if *negated {
                    format!(" {}", self.kw("NOT"))
                } else {
                    String::new()
                };
                format!(
                    "{} IS{} {}",
                    self.format_expr(expr),
                    not_str,
                    self.kw("NULL")
                )
            }
            Expr::TypeCast { expr, type_name } => {
                format!(
                    "{}::{}",
                    self.format_expr(expr),
                    self.format_data_type(type_name)
                )
            }
            Expr::Parameter(n) => format!("${}", n),
            Expr::Array(elements) => {
                format!("{}[{}]", self.kw("ARRAY"), self.format_exprs(elements))
            }
            Expr::Parenthesized(expr) => {
                format!("({})", self.format_expr(expr))
            }
            Expr::Prior(e) => format!("{} {}", self.kw("PRIOR"), self.format_expr(e)),
            Expr::Default => self.kw("DEFAULT").to_string(),
        }
    }

    fn format_exprs(&self, exprs: &[Expr]) -> String {
        let formatted: Vec<String> = exprs.iter().map(|e| self.format_expr(e)).collect();
        formatted.join(", ")
    }

    fn format_literal(&self, lit: &Literal) -> String {
        match lit {
            Literal::Integer(n) => n.to_string(),
            Literal::Float(s) => s.clone(),
            Literal::String(s) => self.quote_string(s),
            Literal::EscapeString(s) => {
                let escaped = s.replace('\\', "\\\\").replace("'", "''");
                format!("E'{}'", escaped)
            }
            Literal::BitString(s) => format!("B'{}'", s),
            Literal::HexString(s) => format!("X'{}'", s),
            Literal::NationalString(s) => format!("N'{}'", s),
            Literal::DollarString { tag, body } => match tag {
                Some(t) => format!("${t}${body}${t}$"),
                None => format!("$${body}$$"),
            },
            Literal::Boolean(b) => {
                if *b {
                    "TRUE".to_string()
                } else {
                    "FALSE".to_string()
                }
            }
            Literal::Null => "NULL".to_string(),
        }
    }

    fn quote_string(&self, s: &str) -> String {
        let escaped = s.replace("'", "''");
        format!("'{}'", escaped)
    }

    fn quote_identifier(&self, s: &str) -> String {
        if s == "*" {
            return s.to_string();
        }
        if s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
            && !s.is_empty()
            && !s.chars().next().unwrap().is_ascii_digit()
        {
            s.to_string()
        } else {
            format!("\"{}\"", s.replace("\"", "\"\""))
        }
    }

    fn quote_identifier_relaxed(&self, s: &str) -> String {
        if s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
            && !s.is_empty()
            && !s.chars().next().unwrap().is_ascii_digit()
        {
            s.to_string()
        } else {
            format!("\"{}\"", s.replace("\"", "\"\""))
        }
    }

    fn format_object_name(&self, name: &ObjectName) -> String {
        let parts: Vec<String> = name.iter().map(|s| self.quote_identifier(s)).collect();
        parts.join(".")
    }

    fn format_window_spec(&self, ws: &WindowSpec) -> String {
        let mut parts = vec![self.kw("OVER")];
        let mut inner = Vec::new();

        if let Some(name) = &ws.window_name {
            inner.push(self.quote_identifier(name));
        }

        if !ws.partition_by.is_empty() {
            inner.push(format!(
                "{} {}",
                self.kw("PARTITION BY"),
                self.format_exprs(&ws.partition_by)
            ));
        }

        if !ws.order_by.is_empty() {
            inner.push(format!(
                "{} {}",
                self.kw("ORDER BY"),
                self.format_order_by(&ws.order_by)
            ));
        }

        if let Some(frame) = &ws.frame {
            inner.push(self.format_window_frame(frame));
        }

        if inner.len() == 1 && ws.window_name.is_some() {
            parts.push(inner.pop().unwrap());
        } else {
            parts.push(format!("({})", inner.join(" ")));
        }

        parts.join(" ")
    }

    fn format_window_frame(&self, frame: &WindowFrame) -> String {
        let mode_str = match frame.mode {
            WindowFrameMode::Rows => self.kw("ROWS"),
            WindowFrameMode::Range => self.kw("RANGE"),
            WindowFrameMode::Groups => self.kw("GROUPS"),
        };
        let mut parts = vec![mode_str];

        if let Some(start) = &frame.start {
            if let Some(end) = &frame.end {
                parts.push(self.kw("BETWEEN"));
                parts.push(self.format_frame_bound(start));
                parts.push(self.kw("AND"));
                parts.push(self.format_frame_bound(end));
            } else {
                parts.push(self.format_frame_bound(start));
            }
        }

        parts.join(" ")
    }

    fn format_frame_bound(&self, bound: &WindowFrameBound) -> String {
        match &bound.direction {
            WindowFrameDirection::UnboundedPreceding => self.kw("UNBOUNDED PRECEDING"),
            WindowFrameDirection::UnboundedFollowing => self.kw("UNBOUNDED FOLLOWING"),
            WindowFrameDirection::CurrentRow => self.kw("CURRENT ROW"),
            WindowFrameDirection::Preceding(offset) => {
                format!("{} {}", offset, self.kw("PRECEDING"))
            }
            WindowFrameDirection::Following(offset) => {
                format!("{} {}", offset, self.kw("FOLLOWING"))
            }
        }
    }

    fn format_order_by(&self, items: &[OrderByItem]) -> String {
        let formatted: Vec<String> = items.iter().map(|i| self.format_order_by_item(i)).collect();
        formatted.join(", ")
    }

    fn format_order_by_item(&self, item: &OrderByItem) -> String {
        let mut result = self.format_expr(&item.expr);
        if let Some(asc) = item.asc {
            result = format!(
                "{} {}",
                result,
                if asc { self.kw("ASC") } else { self.kw("DESC") }
            );
        }
        if let Some(nf) = item.nulls_first {
            result = format!(
                "{} {} {}",
                result,
                self.kw("NULLS"),
                if nf {
                    self.kw("FIRST")
                } else {
                    self.kw("LAST")
                }
            );
        }
        result
    }

    fn format_insert(&self, stmt: &InsertStatement) -> String {
        let mut parts = vec![self.kw("INSERT INTO")];
        parts.push(self.format_object_name(&stmt.table));

        if !stmt.columns.is_empty() {
            parts.push(format!("({})", stmt.columns.join(", ")));
        }

        match &stmt.source {
            InsertSource::Values(rows) => {
                let rows_formatted: Vec<String> = rows
                    .iter()
                    .map(|row| format!("({})", self.format_exprs(row)))
                    .collect();
                parts.push(format!(
                    "{} {}",
                    self.kw("VALUES"),
                    rows_formatted.join(", ")
                ));
            }
            InsertSource::Select(sel) => {
                parts.push(self.format_select(sel));
            }
            InsertSource::DefaultValues => {
                parts.push(self.kw("DEFAULT VALUES"));
            }
        }

        if !stmt.returning.is_empty() {
            parts.push(format!(
                "{} {}",
                self.kw("RETURNING"),
                self.format_select_targets(&stmt.returning)
            ));
        }

        parts.join(" ")
    }

    fn format_insert_all_target(&self, target: &InsertAllTarget) -> String {
        let mut parts = vec![self.kw("INTO"), self.format_object_name(&target.table)];
        if !target.columns.is_empty() {
            parts.push(format!("({})", target.columns.join(", ")));
        }
        let rows: Vec<String> = target
            .values
            .iter()
            .map(|row| format!("({})", self.format_exprs(row)))
            .collect();
        parts.push(format!("{} {}", self.kw("VALUES"), rows.join(", ")));
        parts.join(" ")
    }

    fn format_insert_all(&self, stmt: &InsertAllStatement) -> String {
        let mut parts = vec![self.kw("INSERT ALL")];
        for target in &stmt.targets {
            parts.push(self.format_insert_all_target(target));
        }
        for cond in &stmt.conditions {
            parts.push(self.kw("WHEN"));
            parts.push(self.format_expr(&cond.condition));
            parts.push(self.kw("THEN"));
            for target in &cond.targets {
                parts.push(self.format_insert_all_target(target));
            }
        }
        if !stmt.else_targets.is_empty() {
            parts.push(self.kw("ELSE"));
            for target in &stmt.else_targets {
                parts.push(self.format_insert_all_target(target));
            }
        }
        parts.push(self.format_select(&stmt.source));
        parts.join(" ")
    }

    fn format_insert_first(&self, stmt: &InsertFirstStatement) -> String {
        let mut parts = vec![self.kw("INSERT FIRST")];
        for cond in &stmt.when_clauses {
            parts.push(self.kw("WHEN"));
            parts.push(self.format_expr(&cond.condition));
            parts.push(self.kw("THEN"));
            for target in &cond.targets {
                parts.push(self.format_insert_all_target(target));
            }
        }
        if !stmt.else_targets.is_empty() {
            parts.push(self.kw("ELSE"));
            for target in &stmt.else_targets {
                parts.push(self.format_insert_all_target(target));
            }
        }
        parts.push(self.format_select(&stmt.source));
        parts.join(" ")
    }

    fn format_update(&self, stmt: &UpdateStatement) -> String {
        let mut parts = vec![self.kw("UPDATE")];
        parts.push(self.format_table_refs(&stmt.tables));

        let assignments: Vec<String> = stmt
            .assignments
            .iter()
            .map(|a| {
                format!(
                    "{} = {}",
                    self.format_object_name(&a.column),
                    self.format_expr(&a.value)
                )
            })
            .collect();
        parts.push(format!("{} {}", self.kw("SET"), assignments.join(", ")));

        if !stmt.from.is_empty() {
            parts.push(format!(
                "{} {}",
                self.kw("FROM"),
                self.format_table_refs(&stmt.from)
            ));
        }

        if let Some(where_clause) = &stmt.where_clause {
            parts.push(format!(
                "{} {}",
                self.kw("WHERE"),
                self.format_expr(where_clause)
            ));
        }

        if !stmt.returning.is_empty() {
            parts.push(format!(
                "{} {}",
                self.kw("RETURNING"),
                self.format_select_targets(&stmt.returning)
            ));
        }

        parts.join(" ")
    }

    fn format_delete(&self, stmt: &DeleteStatement) -> String {
        let mut parts = vec![self.kw("DELETE FROM")];
        parts.push(self.format_table_refs(&stmt.tables));

        if !stmt.using.is_empty() {
            parts.push(format!(
                "{} {}",
                self.kw("USING"),
                self.format_table_refs(&stmt.using)
            ));
        }

        if let Some(where_clause) = &stmt.where_clause {
            parts.push(format!(
                "{} {}",
                self.kw("WHERE"),
                self.format_expr(where_clause)
            ));
        }

        if !stmt.returning.is_empty() {
            parts.push(format!(
                "{} {}",
                self.kw("RETURNING"),
                self.format_select_targets(&stmt.returning)
            ));
        }

        parts.join(" ")
    }

    fn format_merge(&self, stmt: &MergeStatement) -> String {
        let mut parts = vec![self.kw("MERGE INTO")];
        parts.push(self.format_table_ref(&stmt.target));
        parts.push(self.kw("USING"));
        parts.push(self.format_table_ref(&stmt.source));
        parts.push(self.kw("ON"));
        parts.push(self.format_expr(&stmt.on_condition));

        for wc in &stmt.when_clauses {
            let when_str = if wc.matched {
                format!(
                    "{} {}",
                    self.kw("WHEN MATCHED THEN"),
                    self.format_merge_action(&wc.action)
                )
            } else {
                format!(
                    "{} {}",
                    self.kw("WHEN NOT MATCHED THEN"),
                    self.format_merge_action(&wc.action)
                )
            };
            parts.push(when_str);
        }

        parts.join(" ")
    }

    fn format_merge_action(&self, action: &MergeAction) -> String {
        match action {
            MergeAction::Update(assignments) => {
                let assigns: Vec<String> = assignments
                    .iter()
                    .map(|a| {
                        format!(
                            "{} = {}",
                            self.format_object_name(&a.column),
                            self.format_expr(&a.value)
                        )
                    })
                    .collect();
                format!("{} {}", self.kw("UPDATE SET"), assigns.join(", "))
            }
            MergeAction::Delete => self.kw("DELETE").to_string(),
            MergeAction::Insert { columns, values } => {
                let mut parts = vec![self.kw("INSERT")];
                if !columns.is_empty() {
                    parts.push(format!("({})", columns.join(", ")));
                }
                parts.push(format!(
                    "{} ({})",
                    self.kw("VALUES"),
                    self.format_exprs(values)
                ));
                parts.join(" ")
            }
        }
    }

    fn format_create_table(&self, stmt: &CreateTableStatement) -> String {
        let mut parts = vec![self.kw("CREATE TABLE")];

        if stmt.if_not_exists {
            parts.push(self.kw("IF NOT EXISTS"));
        }

        parts.push(self.format_object_name(&stmt.name));

        let mut inner = Vec::new();
        for col in &stmt.columns {
            inner.push(self.format_column_def(col));
        }
        for constraint in &stmt.constraints {
            inner.push(self.format_table_constraint(constraint));
        }

        parts.push(format!("({})", inner.join(", ")));

        parts.join(" ")
    }

    fn format_column_def(&self, col: &ColumnDef) -> String {
        let mut parts = vec![self.quote_identifier(&col.name)];
        parts.push(self.format_data_type(&col.data_type));

        for constraint in &col.constraints {
            parts.push(self.format_column_constraint(constraint));
        }

        parts.join(" ")
    }

    fn format_data_type(&self, dt: &DataType) -> String {
        match dt {
            DataType::Boolean => "BOOLEAN".to_string(),
            DataType::SmallInt => "SMALLINT".to_string(),
            DataType::Integer => "INTEGER".to_string(),
            DataType::BigInt => "BIGINT".to_string(),
            DataType::Real => "REAL".to_string(),
            DataType::Double => "DOUBLE PRECISION".to_string(),
            DataType::Numeric(p, s) => match (p, s) {
                (None, None) => "NUMERIC".to_string(),
                (Some(p), None) => format!("NUMERIC({})", p),
                (Some(p), Some(s)) => format!("NUMERIC({}, {})", p, s),
                (None, Some(s)) => format!("NUMERIC({}, {})", 0, s),
            },
            DataType::Char(n) => match n {
                Some(len) => format!("CHAR({})", len),
                None => "CHAR".to_string(),
            },
            DataType::Varchar(n) => match n {
                Some(len) => format!("VARCHAR({})", len),
                None => "VARCHAR".to_string(),
            },
            DataType::Text => "TEXT".to_string(),
            DataType::Bytea => "BYTEA".to_string(),
            DataType::Timestamp(p, tz) => {
                let precision = match p {
                    Some(pr) => format!("({})", pr),
                    None => String::new(),
                };
                let tz_str = match tz {
                    Some(TimeZoneInfo::WithTimeZone) => " WITH TIME ZONE",
                    Some(TimeZoneInfo::WithoutTimeZone) => " WITHOUT TIME ZONE",
                    None => "",
                };
                format!("TIMESTAMP{}{}", precision, tz_str)
            }
            DataType::Timestamptz(p) => match p {
                Some(pr) => format!("TIMESTAMPTZ({})", pr),
                None => "TIMESTAMPTZ".to_string(),
            },
            DataType::Date => "DATE".to_string(),
            DataType::Time(p, tz) => {
                let precision = match p {
                    Some(pr) => format!("({})", pr),
                    None => String::new(),
                };
                let tz_str = match tz {
                    Some(TimeZoneInfo::WithTimeZone) => " WITH TIME ZONE",
                    Some(TimeZoneInfo::WithoutTimeZone) => " WITHOUT TIME ZONE",
                    None => "",
                };
                format!("TIME{}{}", precision, tz_str)
            }
            DataType::Interval => "INTERVAL".to_string(),
            DataType::Json => "JSON".to_string(),
            DataType::Jsonb => "JSONB".to_string(),
            DataType::Uuid => "UUID".to_string(),
            DataType::Bit(n) => match n {
                Some(len) => format!("BIT({})", len),
                None => "BIT".to_string(),
            },
            DataType::Varbit(n) => match n {
                Some(len) => format!("BIT VARYING({})", len),
                None => "BIT VARYING".to_string(),
            },
            DataType::Custom(name) => self.format_object_name(name),
        }
    }

    fn format_column_constraint(&self, c: &ColumnConstraint) -> String {
        match c {
            ColumnConstraint::NotNull => self.kw("NOT NULL"),
            ColumnConstraint::Null => self.kw("NULL"),
            ColumnConstraint::Default(expr) => {
                format!("{} {}", self.kw("DEFAULT"), self.format_expr(expr))
            }
            ColumnConstraint::Unique => self.kw("UNIQUE"),
            ColumnConstraint::PrimaryKey => self.kw("PRIMARY KEY"),
            ColumnConstraint::Check(expr) => {
                format!("{} ({})", self.kw("CHECK"), self.format_expr(expr))
            }
            ColumnConstraint::References(table, cols) => {
                let mut result = format!(
                    "{} {}",
                    self.kw("REFERENCES"),
                    self.format_object_name(table)
                );
                if !cols.is_empty() {
                    result = format!("{} ({})", result, cols.join(", "));
                }
                result
            }
        }
    }

    fn format_table_constraint(&self, c: &TableConstraint) -> String {
        match c {
            TableConstraint::PrimaryKey(cols) => {
                format!("{} ({})", self.kw("PRIMARY KEY"), cols.join(", "))
            }
            TableConstraint::Unique(cols) => {
                format!("{} ({})", self.kw("UNIQUE"), cols.join(", "))
            }
            TableConstraint::Check(expr) => {
                format!("{} ({})", self.kw("CHECK"), self.format_expr(expr))
            }
            TableConstraint::ForeignKey {
                columns,
                ref_table,
                ref_columns,
            } => {
                let mut result = format!(
                    "{} ({}) {} {}",
                    self.kw("FOREIGN KEY"),
                    columns.join(", "),
                    self.kw("REFERENCES"),
                    self.format_object_name(ref_table)
                );
                if !ref_columns.is_empty() {
                    result = format!("{} ({})", result, ref_columns.join(", "));
                }
                result
            }
        }
    }

    fn format_alter_table(&self, stmt: &AlterTableStatement) -> String {
        let mut parts = vec![self.kw("ALTER TABLE")];

        if stmt.if_exists {
            parts.push(self.kw("IF EXISTS"));
        }

        parts.push(self.format_object_name(&stmt.name));

        let actions: Vec<String> = stmt
            .actions
            .iter()
            .map(|a| self.format_alter_table_action(a))
            .collect();
        parts.push(actions.join(", "));

        parts.join(" ")
    }

    fn format_alter_table_action(&self, action: &AlterTableAction) -> String {
        match action {
            AlterTableAction::AddColumn(col) => {
                format!("{} {}", self.kw("ADD COLUMN"), self.format_column_def(col))
            }
            AlterTableAction::DropColumn {
                name,
                if_exists,
                cascade,
            } => {
                let mut parts = vec![self.kw("DROP COLUMN")];
                if *if_exists {
                    parts.push(self.kw("IF EXISTS"));
                }
                parts.push(self.quote_identifier(name));
                if *cascade {
                    parts.push(self.kw("CASCADE"));
                }
                parts.join(" ")
            }
            AlterTableAction::AlterColumn { name, action } => {
                format!(
                    "{} {} {}",
                    self.kw("ALTER COLUMN"),
                    self.quote_identifier(name),
                    self.format_alter_column_action(action)
                )
            }
            AlterTableAction::AddConstraint { constraint, .. } => {
                format!(
                    "{} {}",
                    self.kw("ADD"),
                    self.format_table_constraint(constraint)
                )
            }
            AlterTableAction::DropConstraint {
                name,
                if_exists,
                cascade,
            } => {
                let mut parts = vec![self.kw("DROP CONSTRAINT")];
                if *if_exists {
                    parts.push(self.kw("IF EXISTS"));
                }
                parts.push(self.quote_identifier(name));
                if *cascade {
                    parts.push(self.kw("CASCADE"));
                }
                parts.join(" ")
            }
            AlterTableAction::RenameColumn { old, new } => {
                format!(
                    "{} {} {} {}",
                    self.kw("RENAME COLUMN"),
                    self.quote_identifier(old),
                    self.kw("TO"),
                    self.quote_identifier(new)
                )
            }
            AlterTableAction::RenameTo { new_name } => {
                format!(
                    "{} {}",
                    self.kw("RENAME TO"),
                    self.quote_identifier(new_name)
                )
            }
            AlterTableAction::OwnerTo { owner } => {
                format!("{} {}", self.kw("OWNER TO"), self.quote_identifier(owner))
            }
            AlterTableAction::SetSchema { schema } => {
                format!(
                    "{} {}",
                    self.kw("SET SCHEMA"),
                    self.quote_identifier(schema)
                )
            }
        }
    }

    fn format_alter_column_action(&self, action: &AlterColumnAction) -> String {
        match action {
            AlterColumnAction::SetDataType(dt) => {
                format!("{} {}", self.kw("TYPE"), self.format_data_type(dt))
            }
            AlterColumnAction::SetDefault(expr) => {
                format!("{} {}", self.kw("SET DEFAULT"), self.format_expr(expr))
            }
            AlterColumnAction::DropDefault => self.kw("DROP DEFAULT"),
            AlterColumnAction::SetNotNull => self.kw("SET NOT NULL"),
            AlterColumnAction::DropNotNull => self.kw("DROP NOT NULL"),
        }
    }

    fn format_drop(&self, stmt: &DropStatement) -> String {
        let mut parts = vec![self.kw("DROP")];

        let obj_type = match stmt.object_type {
            ObjectType::Table => "TABLE",
            ObjectType::Index => "INDEX",
            ObjectType::Sequence => "SEQUENCE",
            ObjectType::View => "VIEW",
            ObjectType::Schema => "SCHEMA",
            ObjectType::Database => "DATABASE",
            ObjectType::Tablespace => "TABLESPACE",
            ObjectType::Function => "FUNCTION",
            ObjectType::Procedure => "PROCEDURE",
            ObjectType::Trigger => "TRIGGER",
            ObjectType::Extension => "EXTENSION",
            ObjectType::MaterializedView => "MATERIALIZED VIEW",
            ObjectType::ForeignTable => "FOREIGN TABLE",
            ObjectType::ForeignServer => "SERVER",
            ObjectType::Fdw => "FOREIGN DATA WRAPPER",
        };
        parts.push(self.kw(obj_type));

        if stmt.if_exists {
            parts.push(self.kw("IF EXISTS"));
        }

        let names: Vec<String> = stmt
            .names
            .iter()
            .map(|n| self.format_object_name(n))
            .collect();
        parts.push(names.join(", "));

        if stmt.cascade {
            parts.push(self.kw("CASCADE"));
        }

        if stmt.purge {
            parts.push(self.kw("PURGE"));
        }

        parts.join(" ")
    }

    fn format_truncate(&self, stmt: &TruncateStatement) -> String {
        let mut parts = vec![self.kw("TRUNCATE TABLE")];

        let tables: Vec<String> = stmt
            .tables
            .iter()
            .map(|t| self.format_object_name(t))
            .collect();
        parts.push(tables.join(", "));

        if stmt.restart_identity {
            parts.push(self.kw("RESTART IDENTITY"));
        }

        if stmt.cascade {
            parts.push(self.kw("CASCADE"));
        }

        parts.join(" ")
    }

    fn format_create_index(&self, stmt: &CreateIndexStatement) -> String {
        let mut parts = vec![self.kw("CREATE")];

        if stmt.unique {
            parts.push(self.kw("UNIQUE"));
        }

        parts.push(self.kw("INDEX"));

        if stmt.if_not_exists {
            parts.push(self.kw("IF NOT EXISTS"));
        }

        if let Some(name) = &stmt.name {
            parts.push(self.quote_identifier(name));
        }

        parts.push(self.kw("ON"));
        parts.push(self.format_object_name(&stmt.table));

        let cols: Vec<String> = stmt
            .columns
            .iter()
            .map(|c| {
                let mut result = self.quote_identifier(&c.name);
                if let Some(asc) = c.asc {
                    result = format!("{} {}", result, if asc { "ASC" } else { "DESC" });
                }
                result
            })
            .collect();
        parts.push(format!("({})", cols.join(", ")));

        if let Some(where_clause) = &stmt.where_clause {
            parts.push(format!(
                "{} {}",
                self.kw("WHERE"),
                self.format_expr(where_clause)
            ));
        }

        parts.join(" ")
    }

    fn format_create_view(&self, stmt: &CreateViewStatement) -> String {
        let mut parts = vec![self.kw("CREATE")];

        if stmt.replace {
            parts.push(self.kw("OR REPLACE"));
        }

        if stmt.temporary {
            parts.push(self.kw("TEMPORARY"));
        }

        if stmt.recursive {
            parts.push(self.kw("RECURSIVE"));
        }

        parts.push(self.kw("VIEW"));
        parts.push(self.format_object_name(&stmt.name));

        if !stmt.columns.is_empty() {
            parts.push(format!("({})", stmt.columns.join(", ")));
        }

        parts.push(self.kw("AS"));
        parts.push(self.format_select(&stmt.query));

        if let Some(check) = &stmt.check_option {
            parts.push(self.kw("WITH"));
            match check {
                CheckOption::Local => parts.push(self.kw("LOCAL CHECK OPTION")),
                CheckOption::Cascaded => parts.push(self.kw("CASCADED CHECK OPTION")),
            }
        }

        parts.join(" ")
    }

    fn format_create_sequence(&self, stmt: &CreateSequenceStatement) -> String {
        let mut parts = vec![self.kw("CREATE SEQUENCE")];

        if stmt.if_not_exists {
            parts.push(self.kw("IF NOT EXISTS"));
        }

        parts.push(self.format_object_name(&stmt.name));

        if let Some(start) = &stmt.start {
            parts.push(format!(
                "{} {}",
                self.kw("START WITH"),
                self.format_expr(start)
            ));
        }

        if let Some(incr) = &stmt.increment {
            parts.push(format!(
                "{} {}",
                self.kw("INCREMENT BY"),
                self.format_expr(incr)
            ));
        }

        if let Some(min) = &stmt.min_value {
            parts.push(format!("{} {}", self.kw("MINVALUE"), self.format_expr(min)));
        }

        if let Some(max) = &stmt.max_value {
            parts.push(format!("{} {}", self.kw("MAXVALUE"), self.format_expr(max)));
        }

        if let Some(cache) = &stmt.cache {
            parts.push(format!("{} {}", self.kw("CACHE"), self.format_expr(cache)));
        }

        if stmt.cycle {
            parts.push(self.kw("CYCLE"));
        }

        parts.join(" ")
    }

    fn format_create_schema(&self, stmt: &CreateSchemaStatement) -> String {
        let mut parts = vec![self.kw("CREATE SCHEMA")];

        if stmt.if_not_exists {
            parts.push(self.kw("IF NOT EXISTS"));
        }

        if let Some(name) = &stmt.name {
            parts.push(self.quote_identifier(name));
        }

        if let Some(auth) = &stmt.authorization {
            parts.push(self.kw("AUTHORIZATION"));
            parts.push(self.quote_identifier(auth));
        }

        parts.join(" ")
    }

    fn format_create_database(&self, stmt: &CreateDatabaseStatement) -> String {
        let mut parts = vec![self.kw("CREATE DATABASE")];
        parts.push(self.quote_identifier(&stmt.name));

        if !stmt.owner.is_none()
            || !stmt.template.is_none()
            || !stmt.encoding.is_none()
            || !stmt.locale.is_none()
            || !stmt.lc_collate.is_none()
            || !stmt.lc_ctype.is_none()
            || !stmt.tablespace.is_none()
            || !stmt.allow_connections.is_none()
            || !stmt.connection_limit.is_none()
            || !stmt.is_template.is_none()
        {
            parts.push(self.kw("WITH"));
        }

        if let Some(owner) = &stmt.owner {
            parts.push(format!(
                "{} = {}",
                self.kw("OWNER"),
                self.quote_identifier(owner)
            ));
        }

        if let Some(template) = &stmt.template {
            parts.push(format!(
                "{} = {}",
                self.kw("TEMPLATE"),
                self.quote_identifier(template)
            ));
        }

        if let Some(encoding) = &stmt.encoding {
            parts.push(format!(
                "{} = {}",
                self.kw("ENCODING"),
                self.quote_string(encoding)
            ));
        }

        if let Some(locale) = &stmt.locale {
            parts.push(format!(
                "{} = {}",
                self.kw("LOCALE"),
                self.quote_string(locale)
            ));
        }

        if let Some(lc_collate) = &stmt.lc_collate {
            parts.push(format!(
                "{} = {}",
                self.kw("LC_COLLATE"),
                self.quote_string(lc_collate)
            ));
        }

        if let Some(lc_ctype) = &stmt.lc_ctype {
            parts.push(format!(
                "{} = {}",
                self.kw("LC_CTYPE"),
                self.quote_string(lc_ctype)
            ));
        }

        if let Some(tablespace) = &stmt.tablespace {
            parts.push(format!(
                "{} = {}",
                self.kw("TABLESPACE"),
                self.quote_identifier(tablespace)
            ));
        }

        if let Some(allow_conn) = &stmt.allow_connections {
            parts.push(format!("{} = {}", self.kw("ALLOW_CONNECTIONS"), allow_conn));
        }

        if let Some(conn_limit) = &stmt.connection_limit {
            parts.push(format!("{} = {}", self.kw("CONNECTION LIMIT"), conn_limit));
        }

        if let Some(is_template) = &stmt.is_template {
            parts.push(format!("{} = {}", self.kw("IS_TEMPLATE"), is_template));
        }

        parts.join(" ")
    }

    fn format_create_tablespace(&self, stmt: &CreateTablespaceStatement) -> String {
        let mut parts = vec![self.kw("CREATE TABLESPACE")];
        parts.push(self.quote_identifier(&stmt.name));

        if let Some(owner) = &stmt.owner {
            parts.push(self.kw("OWNER"));
            parts.push(self.quote_identifier(owner));
        }

        parts.push(self.kw("LOCATION"));
        parts.push(self.quote_string(&stmt.location));

        parts.join(" ")
    }

    fn format_transaction(&self, stmt: &TransactionStatement) -> String {
        match &stmt.kind {
            TransactionKind::Begin => {
                let mut parts = vec![self.kw("BEGIN")];
                if !stmt.modes.is_empty() {
                    parts.push(self.format_transaction_modes(&stmt.modes));
                }
                parts.join(" ")
            }
            TransactionKind::Commit => self.kw("COMMIT"),
            TransactionKind::Rollback => self.kw("ROLLBACK"),
            TransactionKind::Savepoint => {
                format!(
                    "{} {}",
                    self.kw("SAVEPOINT"),
                    self.quote_identifier(stmt.savepoint_name.as_ref().unwrap())
                )
            }
            TransactionKind::ReleaseSavepoint => {
                format!(
                    "{} {} {}",
                    self.kw("RELEASE"),
                    self.kw("SAVEPOINT"),
                    self.quote_identifier(stmt.savepoint_name.as_ref().unwrap())
                )
            }
        }
    }

    fn format_transaction_modes(&self, modes: &[TransactionMode]) -> String {
        let formatted: Vec<String> = modes
            .iter()
            .map(|m| self.format_transaction_mode(m))
            .collect();
        formatted.join(", ")
    }

    fn format_transaction_mode(&self, mode: &TransactionMode) -> String {
        match mode {
            TransactionMode::IsolationLevel(level) => {
                let level_str = match level {
                    IsolationLevel::ReadUncommitted => "READ UNCOMMITTED",
                    IsolationLevel::ReadCommitted => "READ COMMITTED",
                    IsolationLevel::RepeatableRead => "REPEATABLE READ",
                    IsolationLevel::Serializable => "SERIALIZABLE",
                };
                format!("{} {}", self.kw("ISOLATION LEVEL"), self.kw(level_str))
            }
            TransactionMode::ReadOnly => self.kw("READ ONLY"),
            TransactionMode::ReadWrite => self.kw("READ WRITE"),
            TransactionMode::Deferrable => self.kw("DEFERRABLE"),
            TransactionMode::NotDeferrable => self.kw("NOT DEFERRABLE"),
        }
    }

    fn format_variable_set(&self, stmt: &VariableSetStatement) -> String {
        let mut parts = vec![self.kw("SET")];

        if stmt.session {
            parts.push(self.kw("SESSION"));
        } else if stmt.local {
            parts.push(self.kw("LOCAL"));
        }

        parts.push(stmt.name.clone());
        parts.push("=".to_string());

        if stmt.value.len() == 1 {
            parts.push(self.format_expr(&stmt.value[0]));
        } else {
            parts.push(format!("({})", self.format_exprs(&stmt.value)));
        }

        parts.join(" ")
    }

    fn format_variable_show(&self, stmt: &VariableShowStatement) -> String {
        format!("{} {}", self.kw("SHOW"), stmt.name)
    }

    fn format_variable_reset(&self, stmt: &VariableResetStatement) -> String {
        format!("{} {}", self.kw("RESET"), stmt.name)
    }

    fn format_discard(&self, stmt: &DiscardStatement) -> String {
        let target = match stmt.target {
            DiscardTarget::All => "ALL",
            DiscardTarget::Plans => "PLANS",
            DiscardTarget::Sequences => "SEQUENCES",
            DiscardTarget::Temp => "TEMP",
        };
        format!("{} {}", self.kw("DISCARD"), self.kw(target))
    }

    fn format_copy(&self, stmt: &CopyStatement) -> String {
        let mut parts = vec![self.kw("COPY")];

        if let Some(query) = &stmt.query {
            parts.push(format!("({})", self.format_select(query)));
        } else if let Some(relation) = &stmt.relation {
            parts.push(self.format_object_name(relation));
            if !stmt.columns.is_empty() {
                parts.push(format!("({})", stmt.columns.join(", ")));
            }
        }

        if stmt.is_from {
            parts.push(self.kw("FROM"));
        } else {
            parts.push(self.kw("TO"));
        }

        if let Some(filename) = &stmt.filename {
            parts.push(self.quote_string(filename));
        } else if stmt.is_from {
            parts.push(self.kw("STDIN"));
        } else {
            parts.push(self.kw("STDOUT"));
        }

        if !stmt.options.is_empty() {
            let opts: Vec<String> = stmt
                .options
                .iter()
                .map(|o| match &o.value {
                    Some(v) => format!("{} {}", o.name, v),
                    None => o.name.clone(),
                })
                .collect();
            parts.push(format!("{} ({})", self.kw("WITH"), opts.join(", ")));
        }

        parts.join(" ")
    }

    fn format_explain(&self, stmt: &ExplainStatement) -> String {
        let mut parts = vec![self.kw("EXPLAIN")];

        if stmt.analyze {
            parts.push(self.kw("ANALYZE"));
        }

        if stmt.verbose {
            parts.push(self.kw("VERBOSE"));
        }

        if stmt.performance {
            parts.push(self.kw("PERFORMANCE"));
        }

        if stmt.plan {
            parts.push(self.kw("PLAN"));
        }

        if !stmt.options.is_empty() {
            let opts: Vec<String> = stmt
                .options
                .iter()
                .map(|o| match &o.value {
                    Some(v) => format!("{} {}", o.name, v),
                    None => o.name.clone(),
                })
                .collect();
            parts.push(format!("({})", opts.join(", ")));
        }

        parts.push(self.format_statement(&stmt.query));

        parts.join(" ")
    }

    fn format_call(&self, stmt: &CallFuncStatement) -> String {
        let args: Vec<String> = stmt.args.iter().map(|a| self.format_call_arg(a)).collect();
        format!(
            "{} {}({})",
            self.kw("CALL"),
            self.format_object_name(&stmt.func_name),
            args.join(", ")
        )
    }

    fn format_call_arg(&self, arg: &CallArg) -> String {
        match arg {
            CallArg::Positional(expr) => self.format_expr(expr),
            CallArg::Named {
                name,
                arg,
                uses_arrow,
            } => {
                if *uses_arrow {
                    format!("{} => {}", name, self.format_expr(arg))
                } else {
                    format!("{} = {}", name, self.format_expr(arg))
                }
            }
        }
    }

    fn format_anon_block(&self, stmt: &AnonyBlockStatement) -> String {
        self.format_pl_block(&stmt.block)
    }

    fn format_pl_block(&self, block: &crate::ast::plpgsql::PlBlock) -> String {
        use crate::ast::plpgsql::*;
        let mut s = String::new();

        if let Some(ref label) = block.label {
            s.push_str(&format!("<<{}>> ", label));
        }

        if !block.declarations.is_empty() {
            s.push_str(&format!("{} ", self.kw("DECLARE")));
            for decl in &block.declarations {
                s.push_str(&self.format_pl_declaration(decl));
                s.push(' ');
            }
        }

        s.push_str(&self.kw("BEGIN"));
        for stmt in &block.body {
            s.push(' ');
            s.push_str(&self.format_pl_statement(stmt));
        }

        if let Some(ref exc) = block.exception_block {
            s.push(' ');
            s.push_str(&self.kw("EXCEPTION"));
            for handler in &exc.handlers {
                s.push_str(&format!(" {} ", self.kw("WHEN")));
                s.push_str(&handler.conditions.join(" OR "));
                s.push_str(&format!(" {} ", self.kw("THEN")));
                for stmt in &handler.statements {
                    s.push_str(&self.format_pl_statement(stmt));
                    s.push(' ');
                }
            }
        }

        s.push(' ');
        s.push_str(&self.kw("END"));

        if let Some(ref label) = block.end_label {
            s.push_str(&format!(" {}", label));
        }

        s
    }

    fn format_pl_declaration(&self, decl: &crate::ast::plpgsql::PlDeclaration) -> String {
        use crate::ast::plpgsql::*;
        match decl {
            PlDeclaration::Variable(v) => {
                let mut s = v.name.clone();
                if v.constant {
                    s.push_str(&format!(" {} ", self.kw("CONSTANT")));
                }
                s.push(' ');
                s.push_str(&self.format_pl_data_type(&v.data_type));
                if v.not_null {
                    s.push_str(&format!(" {} ", self.kw("NOT NULL")));
                }
                if let Some(ref default) = v.default {
                    s.push_str(&format!(" := {}", self.format_expr(default)));
                }
                s
            }
            PlDeclaration::Cursor(c) => {
                let mut s = format!("{} {} ", c.name, self.kw("CURSOR"));
                if !c.arguments.is_empty() {
                    s.push('(');
                    s.push_str(
                        &c.arguments
                            .iter()
                            .map(|a| {
                                format!("{} {}", a.name, self.format_pl_data_type(&a.data_type))
                            })
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                    s.push_str(") ");
                }
                if !c.query.is_empty() {
                    s.push_str(&format!("{} {}", self.kw("FOR"), c.query));
                }
                s
            }
            PlDeclaration::Record(r) => format!("{} {}", r.name, self.kw("RECORD")),
            PlDeclaration::Type(t) => {
                let fields = t
                    .fields
                    .iter()
                    .map(|f| format!("{} {}", f.name, self.format_pl_data_type(&f.data_type)))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} {} ({})", t.name, self.kw("TYPE"), fields)
            }
            PlDeclaration::NestedProcedure(p) => {
                let mut s = format!(
                    "{} {} ",
                    self.kw("PROCEDURE"),
                    self.format_object_name(&p.name)
                );
                if !p.parameters.is_empty() {
                    let params: Vec<String> = p
                        .parameters
                        .iter()
                        .map(|p| self.format_routine_param(p))
                        .collect();
                    s.push_str(&format!("({})", params.join(", ")));
                }
                if let Some(ref block) = p.block {
                    s.push_str(&format!(
                        " {} {}",
                        self.kw("AS"),
                        self.format_pl_block(block)
                    ));
                }
                s
            }
            PlDeclaration::NestedFunction(f) => {
                let mut s = format!(
                    "{} {} ",
                    self.kw("FUNCTION"),
                    self.format_object_name(&f.name)
                );
                if !f.parameters.is_empty() {
                    let params: Vec<String> = f
                        .parameters
                        .iter()
                        .map(|p| self.format_routine_param(p))
                        .collect();
                    s.push_str(&format!("({})", params.join(", ")));
                }
                if let Some(ref rt) = f.return_type {
                    s.push_str(&format!(" {} {}", self.kw("RETURN"), rt));
                }
                if let Some(ref block) = f.block {
                    s.push_str(&format!(
                        " {} {}",
                        self.kw("AS"),
                        self.format_pl_block(block)
                    ));
                }
                s
            }
            PlDeclaration::Pragma { name, arguments } => {
                if arguments.is_empty() {
                    format!("{} {}", self.kw("PRAGMA"), name)
                } else {
                    format!("{} {}({})", self.kw("PRAGMA"), name, arguments)
                }
            }
        }
    }

    fn format_pl_data_type(&self, dt: &crate::ast::plpgsql::PlDataType) -> String {
        use crate::ast::plpgsql::*;
        match dt {
            PlDataType::TypeName(name) => name.clone(),
            PlDataType::PercentType { table, column } => format!("{}%.{}%", table, column),
            PlDataType::PercentRowType(table) => format!("{}%ROWTYPE", table),
            PlDataType::Record => self.kw("RECORD").to_string(),
            PlDataType::Cursor => self.kw("CURSOR").to_string(),
            PlDataType::RefCursor => self.kw("REFCURSOR").to_string(),
        }
    }

    fn format_pl_statement(&self, stmt: &crate::ast::plpgsql::PlStatement) -> String {
        use crate::ast::plpgsql::*;
        match stmt {
            PlStatement::Block(b) => format!("{};", self.format_pl_block(b)),
            PlStatement::Assignment { target, expression } => {
                format!("{} := {};", target, self.format_expr(expression))
            }
            PlStatement::Null => format!("{};", self.kw("NULL")),
            PlStatement::If(i) => self.format_pl_if(i),
            PlStatement::Case(c) => self.format_pl_case(c),
            PlStatement::Loop(l) => self.format_pl_loop(l),
            PlStatement::While(w) => self.format_pl_while(w),
            PlStatement::For(f) => self.format_pl_for(f),
            PlStatement::ForEach(fe) => self.format_pl_foreach(fe),
            PlStatement::Exit { label, condition } => {
                let mut s = self.kw("EXIT").to_string();
                if let Some(ref lbl) = label {
                    s.push_str(&format!(" {}", lbl));
                }
                if let Some(ref cond) = condition {
                    s.push_str(&format!(" {} {}", self.kw("WHEN"), self.format_expr(cond)));
                }
                format!("{};", s)
            }
            PlStatement::Continue { label, condition } => {
                let mut s = self.kw("CONTINUE").to_string();
                if let Some(ref lbl) = label {
                    s.push_str(&format!(" {}", lbl));
                }
                if let Some(ref cond) = condition {
                    s.push_str(&format!(" {} {}", self.kw("WHEN"), self.format_expr(cond)));
                }
                format!("{};", s)
            }
            PlStatement::Return { expression } => {
                if let Some(ref expr) = expression {
                    format!("{} {};", self.kw("RETURN"), self.format_expr(expr))
                } else {
                    format!("{};", self.kw("RETURN"))
                }
            }
            PlStatement::Raise(r) => self.format_pl_raise(r),
            PlStatement::Execute(e) => {
                let mut s = if e.immediate {
                    format!("{} {}", self.kw("EXECUTE"), self.kw("IMMEDIATE"))
                } else {
                    self.kw("EXECUTE").to_string()
                };
                s.push_str(&format!(" {}", self.format_expr(&e.string_expr)));
                if !e.into_targets.is_empty() {
                    let targets: Vec<String> =
                        e.into_targets.iter().map(|t| self.format_expr(t)).collect();
                    s.push_str(&format!(" {} {}", self.kw("INTO"), targets.join(", ")));
                }
                if !e.using_args.is_empty() {
                    let args: Vec<String> = e
                        .using_args
                        .iter()
                        .map(|a| {
                            let mode = match a.mode {
                                PlUsingMode::In => format!("{} ", self.kw("IN")),
                                PlUsingMode::Out => format!("{} ", self.kw("OUT")),
                                PlUsingMode::InOut => {
                                    format!("{} {} ", self.kw("IN"), self.kw("OUT"))
                                }
                            };
                            format!("{}{}", mode, self.format_expr(&a.argument))
                        })
                        .collect();
                    s.push_str(&format!(" {} {}", self.kw("USING"), args.join(", ")));
                }
                format!("{};", s)
            }
            PlStatement::Perform { query, .. } => format!("{} {};", self.kw("PERFORM"), query),
            PlStatement::Open(o) => {
                let mut s = format!("{} {}", self.kw("OPEN"), o.cursor);
                match &o.kind {
                    PlOpenKind::Simple { arguments } => {
                        if !arguments.is_empty() {
                            let args: Vec<String> =
                                arguments.iter().map(|a| self.format_expr(a)).collect();
                            s.push_str(&format!("({})", args.join(", ")));
                        }
                    }
                    PlOpenKind::ForQuery { query, .. } => {
                        s.push_str(&format!(" {} {}", self.kw("FOR"), query));
                    }
                    PlOpenKind::ForUsing { expressions } => {
                        let exprs: Vec<String> =
                            expressions.iter().map(|e| self.format_expr(e)).collect();
                        s.push_str(&format!(" {} {}", self.kw("FOR USING"), exprs.join(", ")));
                    }
                }
                format!("{};", s)
            }
            PlStatement::Fetch(f) => {
                let mut s = self.kw("FETCH").to_string();
                if let Some(ref dir) = f.direction {
                    s.push_str(&format!(" {} ", dir));
                }
                s.push_str(&format!(
                    "{} {} {};",
                    f.cursor,
                    self.kw("INTO"),
                    self.format_expr(&f.into)
                ));
                s
            }
            PlStatement::Close { cursor } => format!("{} {};", self.kw("CLOSE"), cursor),
            PlStatement::Move { cursor, direction } => {
                let mut s = self.kw("MOVE").to_string();
                if let Some(ref dir) = direction {
                    s.push_str(&format!(" {} ", dir));
                }
                format!("{}{};", s, cursor)
            }
            PlStatement::GetDiagnostics(g) => {
                let mut s = self.kw("GET").to_string();
                if g.stacked {
                    s.push_str(&format!(" {}", self.kw("STACKED")));
                }
                s.push_str(&format!(" {} ", self.kw("DIAGNOSTICS")));
                s.push_str(
                    &g.items
                        .iter()
                        .map(|i| format!("{} = {}", i.target, i.item))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                format!("{};", s)
            }
            PlStatement::Commit => format!("{};", self.kw("COMMIT")),
            PlStatement::Rollback { to_savepoint } => {
                if let Some(sp) = to_savepoint {
                    format!("{} {} {};", self.kw("ROLLBACK"), self.kw("TO"), sp)
                } else {
                    format!("{};", self.kw("ROLLBACK"))
                }
            }
            PlStatement::Savepoint { name } => format!("{} {};", self.kw("SAVEPOINT"), name),
            PlStatement::Goto { label } => format!("{} {};", self.kw("GOTO"), label),
            PlStatement::ProcedureCall(call) => {
                let name = call.name.join(".");
                let args: Vec<String> =
                    call.arguments.iter().map(|a| self.format_expr(a)).collect();
                let args_str = args.join(", ");
                if args_str.is_empty() {
                    format!("{};", name)
                } else {
                    format!("{}({});", name, args_str)
                }
            }
            PlStatement::Sql(sql) => {
                if sql.is_empty() {
                    String::new()
                } else {
                    format!("{};", sql)
                }
            }
            PlStatement::SqlStatement {
                sql_text,
                statement,
            } => {
                let _ = sql_text;
                self.format_statement(&statement)
            }
            PlStatement::ForAll(f) => format!(
                "{} {} {} {}",
                self.kw("FORALL"),
                f.variable,
                f.bounds,
                f.body
            ),
            PlStatement::PipeRow { expression } => {
                format!("{}({});", self.kw("PIPE ROW"), self.format_expr(expression))
            }
        }
    }

    fn format_pl_if(&self, i: &crate::ast::plpgsql::PlIfStmt) -> String {
        let mut s = format!(
            "{} {} {} ",
            self.kw("IF"),
            self.format_expr(&i.condition),
            self.kw("THEN")
        );
        for stmt in &i.then_stmts {
            s.push_str(&self.format_pl_statement(stmt));
            s.push(' ');
        }
        for elsif in &i.elsifs {
            s.push_str(&format!(
                "{} {} {} ",
                self.kw("ELSIF"),
                self.format_expr(&elsif.condition),
                self.kw("THEN")
            ));
            for stmt in &elsif.stmts {
                s.push_str(&self.format_pl_statement(stmt));
                s.push(' ');
            }
        }
        if !i.else_stmts.is_empty() {
            s.push_str(&format!("{} ", self.kw("ELSE")));
            for stmt in &i.else_stmts {
                s.push_str(&self.format_pl_statement(stmt));
                s.push(' ');
            }
        }
        s.push_str(&format!("{};", self.kw("END IF")));
        s
    }

    fn format_pl_case(&self, c: &crate::ast::plpgsql::PlCaseStmt) -> String {
        let mut s = self.kw("CASE").to_string();
        if let Some(ref expr) = c.expression {
            s.push_str(&format!(" {}", self.format_expr(expr)));
        }
        for when in &c.whens {
            s.push_str(&format!(
                " {} {} {} ",
                self.kw("WHEN"),
                self.format_expr(&when.condition),
                self.kw("THEN")
            ));
            for stmt in &when.stmts {
                s.push_str(&self.format_pl_statement(stmt));
                s.push(' ');
            }
        }
        if !c.else_stmts.is_empty() {
            s.push_str(&format!("{} ", self.kw("ELSE")));
            for stmt in &c.else_stmts {
                s.push_str(&self.format_pl_statement(stmt));
                s.push(' ');
            }
        }
        s.push_str(&format!("{} {};", self.kw("END"), self.kw("CASE")));
        s
    }

    fn format_pl_loop(&self, l: &crate::ast::plpgsql::PlLoopStmt) -> String {
        let mut s = String::new();
        if let Some(ref label) = l.label {
            s.push_str(&format!("<<{}>> ", label));
        }
        s.push_str(&self.kw("LOOP"));
        for stmt in &l.body {
            s.push(' ');
            s.push_str(&self.format_pl_statement(stmt));
        }
        s.push_str(&format!(" {} {};", self.kw("END"), self.kw("LOOP")));
        if let Some(ref label) = l.end_label {
            s.push_str(&format!(" {}", label));
        }
        s
    }

    fn format_pl_while(&self, w: &crate::ast::plpgsql::PlWhileStmt) -> String {
        let mut s = String::new();
        if let Some(ref label) = w.label {
            s.push_str(&format!("<<{}>> ", label));
        }
        s.push_str(&format!(
            "{} {} {} ",
            self.kw("WHILE"),
            self.format_expr(&w.condition),
            self.kw("LOOP")
        ));
        for stmt in &w.body {
            s.push_str(&self.format_pl_statement(stmt));
            s.push(' ');
        }
        s.push_str(&format!("{} {};", self.kw("END"), self.kw("LOOP")));
        if let Some(ref label) = w.end_label {
            s.push_str(&format!(" {}", label));
        }
        s
    }

    fn format_pl_for(&self, f: &crate::ast::plpgsql::PlForStmt) -> String {
        use crate::ast::plpgsql::PlForKind;
        let mut s = String::new();
        if let Some(ref label) = f.label {
            s.push_str(&format!("<<{}>> ", label));
        }
        s.push_str(&format!(
            "{} {} {} ",
            self.kw("FOR"),
            f.variable,
            self.kw("IN")
        ));
        match &f.kind {
            PlForKind::Range {
                low,
                high,
                step,
                reverse,
            } => {
                if *reverse {
                    s.push_str(&format!("{} ", self.kw("REVERSE")));
                }
                s.push_str(&format!(
                    "{}..{}",
                    self.format_expr(low),
                    self.format_expr(high)
                ));
                if let Some(ref st) = step {
                    s.push_str(&format!(" {} {}", self.kw("BY"), self.format_expr(st)));
                }
            }
            PlForKind::Query { query, .. } => {
                s.push_str(query);
            }
            PlForKind::Cursor {
                cursor_name,
                arguments,
            } => {
                s.push_str(cursor_name);
                if !arguments.is_empty() {
                    let args: Vec<String> = arguments.iter().map(|a| self.format_expr(a)).collect();
                    s.push_str(&format!("({})", args.join(", ")));
                }
            }
        }
        s.push_str(&format!("{} ", self.kw("LOOP")));
        for stmt in &f.body {
            s.push_str(&self.format_pl_statement(stmt));
            s.push(' ');
        }
        s.push_str(&format!("{} {};", self.kw("END"), self.kw("LOOP")));
        if let Some(ref label) = f.end_label {
            s.push_str(&format!(" {}", label));
        }
        s
    }

    fn format_pl_foreach(&self, fe: &crate::ast::plpgsql::PlForEachStmt) -> String {
        let mut s = String::new();
        if let Some(ref label) = fe.label {
            s.push_str(&format!("<<{}>> ", label));
        }
        s.push_str(&format!(
            "{} {} {} {} {}",
            self.kw("FOREACH"),
            fe.variable,
            self.kw("IN ARRAY"),
            self.format_expr(&fe.expression),
            self.kw("LOOP")
        ));
        if let Some(slice) = fe.slice {
            s.push_str(&format!(" {} {}", self.kw("SLICE"), slice));
        }
        for stmt in &fe.body {
            s.push_str(&self.format_pl_statement(stmt));
            s.push(' ');
        }
        s.push_str(&format!("{} {};", self.kw("END"), self.kw("LOOP")));
        if let Some(ref label) = fe.end_label {
            s.push_str(&format!(" {}", label));
        }
        s
    }

    fn format_pl_raise(&self, r: &crate::ast::plpgsql::PlRaiseStmt) -> String {
        use crate::ast::plpgsql::RaiseLevel;
        let mut s = self.kw("RAISE").to_string();
        if let Some(ref level) = r.level {
            let level_str = match level {
                RaiseLevel::Debug => "DEBUG",
                RaiseLevel::Log => "LOG",
                RaiseLevel::Info => "INFO",
                RaiseLevel::Notice => "NOTICE",
                RaiseLevel::Warning => "WARNING",
                RaiseLevel::Exception => "EXCEPTION",
            };
            s.push_str(&format!(" {} ", level_str));
        }
        if let Some(ref msg) = r.message {
            s.push_str(msg);
        }
        format!("{};", s)
    }
}

impl Default for SqlFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", SqlFormatter::new().format_statement(self))
    }
}

impl SqlFormatter {
    fn format_options(&self, options: &[(String, String)]) -> String {
        if options.is_empty() {
            return String::new();
        }
        let pairs: Vec<String> = options
            .iter()
            .map(|(k, v)| format!("{} = '{}'", k, v))
            .collect();
        format!(" WITH ({})", pairs.join(", "))
    }

    fn format_create_fdw(&self, stmt: &CreateFdwStatement) -> String {
        let mut s = format!("CREATE FOREIGN DATA WRAPPER {}", stmt.name);
        if let Some(ref h) = stmt.handler {
            s.push_str(&format!(" HANDLER {}", h));
        }
        if let Some(ref v) = stmt.validator {
            s.push_str(&format!(" VALIDATOR {}", v));
        }
        s.push_str(&self.format_options(&stmt.options));
        s
    }

    fn format_create_foreign_server(&self, stmt: &CreateForeignServerStatement) -> String {
        let mut s = format!("CREATE SERVER {}", stmt.name);
        if let Some(ref t) = stmt.server_type {
            s.push_str(&format!(" TYPE '{}'", t));
        }
        if let Some(ref v) = stmt.version {
            s.push_str(&format!(" VERSION '{}'", v));
        }
        s.push_str(&format!(" FOREIGN DATA WRAPPER {}", stmt.fdw_name));
        s.push_str(&self.format_options(&stmt.options));
        s
    }

    fn format_create_foreign_table(&self, stmt: &CreateForeignTableStatement) -> String {
        let mut s = format!(
            "CREATE FOREIGN TABLE {}",
            self.format_object_name(&stmt.name)
        );
        if !stmt.columns.is_empty() {
            let cols: Vec<String> = stmt
                .columns
                .iter()
                .map(|c| self.format_column_def(c))
                .collect();
            s.push_str(&format!(" ({})", cols.join(", ")));
        }
        s.push_str(&format!(" SERVER {}", stmt.server_name));
        s.push_str(&self.format_options(&stmt.options));
        s
    }

    fn format_create_publication(&self, stmt: &CreatePublicationStatement) -> String {
        let mut s = format!("CREATE PUBLICATION {}", stmt.name);
        if stmt.all_tables {
            s.push_str(" FOR ALL TABLES");
        } else if !stmt.tables.is_empty() {
            let tables: Vec<String> = stmt
                .tables
                .iter()
                .map(|t| self.format_object_name(t))
                .collect();
            s.push_str(&format!(" FOR TABLE {}", tables.join(", ")));
        }
        s.push_str(&self.format_options(&stmt.options));
        s
    }

    fn format_create_subscription(&self, stmt: &CreateSubscriptionStatement) -> String {
        let mut s = format!("CREATE SUBSCRIPTION {}", stmt.name);
        s.push_str(&format!(" CONNECTION '{}'", stmt.connection));
        let pubs = stmt.publications.join(", ");
        s.push_str(&format!(" PUBLICATION {}", pubs));
        s.push_str(&self.format_options(&stmt.options));
        s
    }

    fn format_create_node(&self, stmt: &CreateNodeStatement) -> String {
        let mut s = format!("CREATE NODE {}", stmt.name);
        s.push_str(&self.format_options(&stmt.options));
        s
    }

    fn format_create_node_group(&self, stmt: &CreateNodeGroupStatement) -> String {
        let mut s = format!("CREATE NODE GROUP {}", stmt.name);
        if !stmt.nodes.is_empty() {
            s.push_str(&format!(" ({})", stmt.nodes.join(", ")));
        }
        s.push_str(&self.format_options(&stmt.options));
        s
    }

    fn format_create_resource_pool(&self, stmt: &CreateResourcePoolStatement) -> String {
        let mut s = format!("CREATE RESOURCE POOL {}", stmt.name);
        s.push_str(&self.format_options(&stmt.options));
        s
    }

    fn format_create_workload_group(&self, stmt: &CreateWorkloadGroupStatement) -> String {
        let mut s = format!("CREATE WORKLOAD GROUP {}", stmt.name);
        if let Some(ref p) = stmt.pool_name {
            s.push_str(&format!(" USING RESOURCE POOL {}", p));
        }
        s.push_str(&self.format_options(&stmt.options));
        s
    }

    fn format_create_audit_policy(&self, stmt: &CreateAuditPolicyStatement) -> String {
        let mut s = format!("CREATE AUDIT POLICY {}", stmt.name);
        s.push_str(&format!(" {}", stmt.policy_type));
        s.push_str(&self.format_options(&stmt.options));
        s
    }

    fn format_create_masking_policy(&self, stmt: &CreateMaskingPolicyStatement) -> String {
        let mut s = format!("CREATE MASKING POLICY {}", stmt.name);
        s.push_str(&self.format_options(&stmt.options));
        s
    }

    fn format_create_rls_policy(&self, stmt: &CreateRlsPolicyStatement) -> String {
        let mut s = format!(
            "CREATE POLICY {} ON {}",
            stmt.name,
            self.format_object_name(&stmt.table)
        );
        if !stmt.permissive {
            s.push_str(" AS RESTRICTIVE");
        }
        if let Some(ref expr) = stmt.using_expr {
            s.push_str(&format!(" USING ({})", self.format_expr(expr)));
        }
        s
    }

    fn format_privilege(&self, p: &Privilege) -> String {
        let name = match p {
            Privilege::All => "ALL PRIVILEGES",
            Privilege::Select => "SELECT",
            Privilege::Insert => "INSERT",
            Privilege::Update => "UPDATE",
            Privilege::Delete => "DELETE",
            Privilege::Usage => "USAGE",
            Privilege::Create => "CREATE",
            Privilege::Connect => "CONNECT",
            Privilege::Temporary => "TEMPORARY",
            Privilege::Execute => "EXECUTE",
            Privilege::Trigger => "TRIGGER",
            Privilege::References => "REFERENCES",
            Privilege::Alter => "ALTER",
            Privilege::Drop => "DROP",
            Privilege::Comment => "COMMENT",
            Privilege::Index => "INDEX",
            Privilege::Vacuum => "VACUUM",
        };
        self.kw(name)
    }

    fn format_grant(&self, stmt: &GrantStatement) -> String {
        let privs = stmt
            .privileges
            .iter()
            .map(|p| self.format_privilege(p))
            .collect::<Vec<_>>()
            .join(", ");
        let target = match &stmt.target {
            GrantTarget::Table(tables) => format!(
                "TABLE {}",
                tables
                    .iter()
                    .map(|t| self.format_object_name(t))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            GrantTarget::Schema(schemas) => format!("SCHEMA {}", schemas.join(", ")),
            GrantTarget::Database(dbs) => format!("DATABASE {}", dbs.join(", ")),
            GrantTarget::Function(funcs) => format!(
                "FUNCTION {}",
                funcs
                    .iter()
                    .map(|f| self.format_object_name(f))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            GrantTarget::Sequence(seqs) => format!(
                "SEQUENCE {}",
                seqs.iter()
                    .map(|s| self.format_object_name(s))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            GrantTarget::AllTablesInSchema(schemas) => {
                format!("ALL TABLES IN SCHEMA {}", schemas.join(", "))
            }
            GrantTarget::AllFunctionsInSchema(schemas) => {
                format!("ALL FUNCTIONS IN SCHEMA {}", schemas.join(", "))
            }
            GrantTarget::AllSequencesInSchema(schemas) => {
                format!("ALL SEQUENCES IN SCHEMA {}", schemas.join(", "))
            }
        };
        let mut s = format!(
            "{} {} ON {} TO {}",
            self.kw("GRANT"),
            privs,
            target,
            stmt.grantees.join(", ")
        );
        if stmt.with_grant_option {
            s.push_str(&format!(" {}", self.kw("WITH GRANT OPTION")));
        }
        if let Some(ref by) = stmt.granted_by {
            s.push_str(&format!(" {} {}", self.kw("GRANTED BY"), by));
        }
        s
    }

    fn format_revoke(&self, stmt: &RevokeStatement) -> String {
        let privs = stmt
            .privileges
            .iter()
            .map(|p| self.format_privilege(p))
            .collect::<Vec<_>>()
            .join(", ");
        let target = match &stmt.target {
            GrantTarget::Table(tables) => format!(
                "TABLE {}",
                tables
                    .iter()
                    .map(|t| self.format_object_name(t))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            GrantTarget::Schema(schemas) => format!("SCHEMA {}", schemas.join(", ")),
            GrantTarget::Database(dbs) => format!("DATABASE {}", dbs.join(", ")),
            GrantTarget::Function(funcs) => format!(
                "FUNCTION {}",
                funcs
                    .iter()
                    .map(|f| self.format_object_name(f))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            GrantTarget::Sequence(seqs) => format!(
                "SEQUENCE {}",
                seqs.iter()
                    .map(|s| self.format_object_name(s))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            GrantTarget::AllTablesInSchema(schemas) => {
                format!("ALL TABLES IN SCHEMA {}", schemas.join(", "))
            }
            GrantTarget::AllFunctionsInSchema(schemas) => {
                format!("ALL FUNCTIONS IN SCHEMA {}", schemas.join(", "))
            }
            GrantTarget::AllSequencesInSchema(schemas) => {
                format!("ALL SEQUENCES IN SCHEMA {}", schemas.join(", "))
            }
        };
        let mut s = format!(
            "{} {} ON {} FROM {}",
            self.kw("REVOKE"),
            privs,
            target,
            stmt.grantees.join(", ")
        );
        if stmt.cascade {
            s.push_str(&format!(" {}", self.kw("CASCADE")));
        }
        if let Some(ref by) = stmt.granted_by {
            s.push_str(&format!(" {} {}", self.kw("GRANTED BY"), by));
        }
        s
    }

    fn format_vacuum(&self, stmt: &VacuumStatement) -> String {
        let mut opts = Vec::new();
        if stmt.full {
            opts.push(self.kw("FULL"));
        }
        if stmt.verbose {
            opts.push(self.kw("VERBOSE"));
        }
        if stmt.analyze {
            opts.push(self.kw("ANALYZE"));
        }
        if stmt.freeze {
            opts.push(self.kw("FREEZE"));
        }
        let mut s = format!(
            "{}{}",
            self.kw("VACUUM"),
            if opts.is_empty() {
                String::new()
            } else {
                format!(" {}", opts.join(" "))
            }
        );
        if !stmt.tables.is_empty() {
            s.push(' ');
            s.push_str(
                &stmt
                    .tables
                    .iter()
                    .map(|t| {
                        let mut s = self.format_object_name(&t.name);
                        if !t.columns.is_empty() {
                            s.push_str(&format!("({})", t.columns.join(", ")));
                        }
                        s
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
        s
    }

    fn format_analyze(&self, stmt: &AnalyzeStatement) -> String {
        let mut s = self.kw("ANALYZE").to_string();
        if stmt.verbose {
            s.push(' ');
            s.push_str(&self.kw("VERBOSE"));
        }
        if !stmt.tables.is_empty() {
            s.push(' ');
            s.push_str(
                &stmt
                    .tables
                    .iter()
                    .map(|t| self.format_object_name(&t.name))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
        s
    }

    fn format_do(&self, stmt: &DoStatement) -> String {
        let mut s = self.kw("DO").to_string();
        if let Some(ref lang) = stmt.language {
            s.push_str(&format!(" {} {}", self.kw("LANGUAGE"), lang));
        }
        if let Some(ref block) = stmt.block {
            s.push_str(" $$ ");
            s.push_str(&self.format_pl_block(block));
            s.push_str(" $$");
        } else {
            s.push(' ');
            s.push_str(&stmt.code);
        }
        s
    }

    fn format_prepare(&self, stmt: &PrepareStatement) -> String {
        let mut s = format!("{} {}", self.kw("PREPARE"), stmt.name);
        if !stmt.data_types.is_empty() {
            s.push_str(&format!("({})", stmt.data_types.join(", ")));
        }
        s.push_str(&format!(" {} ", self.kw("AS")));
        if let Some(ref parsed) = stmt.parsed_statement {
            s.push_str(&self.format_statement(parsed));
        } else {
            s.push_str(&stmt.statement);
        }
        s
    }

    fn format_execute(&self, stmt: &ExecuteStatement) -> String {
        let mut s = format!("{} {}", self.kw("EXECUTE"), stmt.name);
        if !stmt.params.is_empty() {
            s.push_str(&format!(
                "({})",
                stmt.params
                    .iter()
                    .map(|p| self.format_expr(p))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        s
    }

    fn format_deallocate(&self, stmt: &DeallocateStatement) -> String {
        if stmt.all {
            self.kw("DEALLOCATE ALL").to_string()
        } else {
            format!(
                "{} {}",
                self.kw("DEALLOCATE PREPARE"),
                stmt.name.as_deref().unwrap_or("")
            )
        }
    }

    fn format_comment(&self, stmt: &CommentStatement) -> String {
        format!(
            "{} {} {} {} '{}'",
            self.kw("COMMENT ON"),
            stmt.object_type,
            self.format_object_name(&stmt.name),
            self.kw("IS"),
            stmt.comment
        )
    }

    fn format_lock(&self, stmt: &LockStatement) -> String {
        let mut s = format!(
            "{} {}",
            self.kw("LOCK TABLE"),
            stmt.tables
                .iter()
                .map(|t| self.format_object_name(t))
                .collect::<Vec<_>>()
                .join(", ")
        );
        if !stmt.mode.is_empty() {
            s.push_str(&format!(
                " {} {} {}",
                self.kw("IN"),
                stmt.mode,
                self.kw("MODE")
            ));
        }
        if stmt.nowait {
            s.push(' ');
            s.push_str(&self.kw("NOWAIT"));
        }
        s
    }

    fn format_declare_cursor(&self, stmt: &DeclareCursorStatement) -> String {
        let mut s = format!("{} {}", self.kw("DECLARE"), stmt.name);
        if stmt.binary {
            s.push(' ');
            s.push_str(&self.kw("BINARY"));
        }
        if stmt.scroll {
            s.push(' ');
            s.push_str(&self.kw("SCROLL"));
        }
        if stmt.hold {
            s.push_str(&format!(" {} {}", self.kw("WITH"), self.kw("HOLD")));
        }
        s.push_str(&format!(
            " {} {}",
            self.kw("CURSOR FOR"),
            self.format_select(&stmt.query)
        ));
        s
    }

    fn format_close_portal(&self, stmt: &ClosePortalStatement) -> String {
        format!("{} {}", self.kw("CLOSE"), stmt.name)
    }

    fn format_fetch(&self, stmt: &FetchStatement) -> String {
        let dir = match &stmt.direction {
            FetchDirection::Next => self.kw("NEXT").to_string(),
            FetchDirection::Prior => self.kw("PRIOR").to_string(),
            FetchDirection::First => self.kw("FIRST").to_string(),
            FetchDirection::Last => self.kw("LAST").to_string(),
            FetchDirection::Absolute(n) => format!("{} {}", self.kw("ABSOLUTE"), n),
            FetchDirection::Relative(n) => format!("{} {}", self.kw("RELATIVE"), n),
            FetchDirection::ForwardAll => format!("{} {}", self.kw("FORWARD"), self.kw("ALL")),
            FetchDirection::BackwardAll => format!("{} {}", self.kw("BACKWARD"), self.kw("ALL")),
            FetchDirection::Forward(n) => format!("{} {}", self.kw("FORWARD"), n),
            FetchDirection::Backward(n) => format!("{} {}", self.kw("BACKWARD"), n),
            FetchDirection::Count(n) => n.to_string(),
            FetchDirection::All => self.kw("ALL").to_string(),
        };
        format!("{} {} {}", dir, self.kw("FROM"), stmt.cursor_name)
    }

    fn format_cluster(&self, stmt: &ClusterStatement) -> String {
        let mut s = self.kw("CLUSTER").to_string();
        if stmt.verbose {
            s.push(' ');
            s.push_str(&self.kw("VERBOSE"));
        }
        if let Some(ref table) = stmt.table {
            s.push(' ');
            s.push_str(&self.format_object_name(table));
        }
        s
    }

    fn format_reindex(&self, stmt: &ReindexStatement) -> String {
        let target = match &stmt.target {
            ReindexTarget::Table(name) => {
                format!("{} {}", self.kw("TABLE"), self.format_object_name(name))
            }
            ReindexTarget::Index(name) => {
                format!("{} {}", self.kw("INDEX"), self.format_object_name(name))
            }
            ReindexTarget::Schema(name) => format!("{} {}", self.kw("SCHEMA"), name),
            ReindexTarget::Database(name) => format!("{} {}", self.kw("DATABASE"), name),
            ReindexTarget::System => self.kw("SYSTEM").to_string(),
        };
        let mut s = format!("{} {}", self.kw("REINDEX"), target);
        if stmt.concurrent {
            s.push(' ');
            s.push_str(&self.kw("CONCURRENTLY"));
        }
        if stmt.verbose {
            s.push(' ');
            s.push_str(&self.kw("VERBOSE"));
        }
        s
    }

    fn format_listen(&self, stmt: &ListenStatement) -> String {
        format!("{} {}", self.kw("LISTEN"), stmt.channel)
    }

    fn format_notify(&self, stmt: &NotifyStatement) -> String {
        let mut s = format!("{} {}", self.kw("NOTIFY"), stmt.channel);
        if let Some(ref payload) = stmt.payload {
            s.push_str(&format!(", '{}'", payload));
        }
        s
    }

    fn format_unlisten(&self, stmt: &UnlistenStatement) -> String {
        match &stmt.channel {
            Some(ch) => format!("{} {}", self.kw("UNLISTEN"), ch),
            None => self.kw("UNLISTEN").to_string(),
        }
    }

    fn format_rule(&self, stmt: &RuleStatement) -> String {
        let mut s = format!(
            "{} {} {} {} {} {}",
            self.kw("CREATE RULE"),
            stmt.name,
            self.kw("AS ON"),
            stmt.event,
            self.kw("TO"),
            self.format_object_name(&stmt.table)
        );
        if let Some(ref cond) = stmt.condition {
            s.push_str(&format!(" {} {}", self.kw("WHERE"), self.format_expr(cond)));
        }
        if stmt.instead {
            s.push_str(&self.kw(" DO INSTEAD"));
        } else {
            s.push_str(&self.kw(" DO"));
        }
        if stmt.actions.is_empty() || (stmt.actions.len() == 1 && stmt.actions[0] == "NOTHING") {
            s.push(' ');
            s.push_str(&self.kw("NOTHING"));
        }
        s
    }

    fn format_create_trigger(&self, stmt: &CreateTriggerStatement) -> String {
        let events: Vec<String> = stmt
            .events
            .iter()
            .map(|e| match e {
                TriggerEvent::Insert => "INSERT".to_string(),
                TriggerEvent::Update => "UPDATE".to_string(),
                TriggerEvent::UpdateOf(cols) => format!("UPDATE ({})", cols.join(", ")),
                TriggerEvent::Delete => "DELETE".to_string(),
                TriggerEvent::Truncate => "TRUNCATE".to_string(),
            })
            .collect();
        let mut s = format!(
            "{} {} {} {} {} {} {}",
            self.kw("CREATE TRIGGER"),
            stmt.name,
            self.kw("ON"),
            self.format_object_name(&stmt.table),
            self.kw("FOR EACH"),
            match stmt.for_each {
                TriggerForEach::Row => "ROW",
                TriggerForEach::Statement => "STATEMENT",
            },
            events.join(" OR ")
        );
        if let Some(ref w) = stmt.when {
            s.push_str(&format!(" {} ({})", self.kw("WHEN"), self.format_expr(w)));
        }
        s.push_str(&format!(
            " {} {} {}",
            self.kw("EXECUTE PROCEDURE"),
            self.format_object_name(&stmt.func_name),
            if stmt.func_args.is_empty() {
                String::new()
            } else {
                let args: Vec<String> =
                    stmt.func_args.iter().map(|a| self.format_expr(a)).collect();
                format!("({})", args.join(", "))
            }
        ));
        s
    }

    fn format_create_materialized_view(&self, stmt: &CreateMaterializedViewStatement) -> String {
        let mut s = format!(
            "{} {}",
            self.kw("CREATE MATERIALIZED VIEW"),
            self.format_object_name(&stmt.name)
        );
        if !stmt.columns.is_empty() {
            s.push_str(&format!("({})", stmt.columns.join(", ")));
        }
        s.push_str(&format!(
            " {} {}",
            self.kw("AS"),
            self.format_select(&stmt.query)
        ));
        if let Some(ref ts) = stmt.tablespace {
            s.push_str(&format!(" {} {}", self.kw("TABLESPACE"), ts));
        }
        if stmt.with_data {
            s.push_str(&format!(" {} {}", self.kw("WITH"), self.kw("DATA")));
        } else {
            s.push_str(&format!(
                " {} {} {}",
                self.kw("WITH"),
                self.kw("NO"),
                self.kw("DATA")
            ));
        }
        s
    }

    fn format_refresh_matview(&self, stmt: &RefreshMatViewStatement) -> String {
        let mut s = format!(
            "{} {}",
            self.kw("REFRESH MATERIALIZED VIEW"),
            self.format_object_name(&stmt.name)
        );
        if stmt.concurrent {
            s.push(' ');
            s.push_str(&self.kw("CONCURRENTLY"));
        }
        s
    }

    fn format_alter_database(&self, stmt: &AlterDatabaseStatement) -> String {
        let action = match &stmt.action {
            AlterDatabaseAction::Set { parameter, value } => format!(
                "{} {} {} {}",
                self.kw("SET"),
                parameter,
                self.kw("TO"),
                value
            ),
            AlterDatabaseAction::Reset { parameter } => {
                format!("{} {}", self.kw("RESET"), parameter)
            }
            AlterDatabaseAction::RenameTo { new_name } => {
                format!("{} {}", self.kw("RENAME TO"), new_name)
            }
            AlterDatabaseAction::OwnerTo { owner } => {
                format!("{} {}", self.kw("OWNER TO"), owner)
            }
        };
        format!("{} {} {}", self.kw("ALTER DATABASE"), stmt.name, action)
    }

    fn format_alter_schema(&self, stmt: &AlterSchemaStatement) -> String {
        let action = match &stmt.action {
            AlterSchemaAction::RenameTo { new_name } => {
                format!("{} {}", self.kw("RENAME TO"), new_name)
            }
            AlterSchemaAction::OwnerTo { owner } => format!("{} {}", self.kw("OWNER TO"), owner),
        };
        format!("{} {} {}", self.kw("ALTER SCHEMA"), stmt.name, action)
    }

    fn format_alter_sequence(&self, stmt: &AlterSequenceStatement) -> String {
        let opts: Vec<String> = stmt
            .options
            .iter()
            .map(|o| match o {
                SequenceOption::IncrementBy(n) => format!("{} {}", self.kw("INCREMENT BY"), n),
                SequenceOption::MinValue(Some(n)) => format!("{} {}", self.kw("MINVALUE"), n),
                SequenceOption::MinValue(None) => self.kw("NO MINVALUE").to_string(),
                SequenceOption::MaxValue(Some(n)) => format!("{} {}", self.kw("MAXVALUE"), n),
                SequenceOption::MaxValue(None) => self.kw("NO MAXVALUE").to_string(),
                SequenceOption::StartWith(n) => format!("{} {}", self.kw("START WITH"), n),
                SequenceOption::Restart(_) => self.kw("RESTART").to_string(),
                SequenceOption::Cache(n) => format!("{} {}", self.kw("CACHE"), n),
                SequenceOption::Cycle(true) => self.kw("CYCLE").to_string(),
                SequenceOption::Cycle(false) | SequenceOption::NoCycle => {
                    self.kw("NO CYCLE").to_string()
                }
                SequenceOption::OwnedBy { owner } => {
                    format!("{} {}", self.kw("OWNED BY"), self.format_object_name(owner))
                }
            })
            .collect();
        format!(
            "{} {} {}",
            self.kw("ALTER SEQUENCE"),
            self.format_object_name(&stmt.name),
            opts.join(" ")
        )
    }

    fn format_alter_function(&self, stmt: &AlterFunctionStatement) -> String {
        format!(
            "{} {} {}",
            self.kw("ALTER FUNCTION"),
            self.format_object_name(&stmt.name),
            self.format_alter_function_action(&stmt.action)
        )
    }

    fn format_alter_function_action(&self, action: &AlterFunctionAction) -> String {
        match action {
            AlterFunctionAction::RenameTo { new_name } => {
                format!("{} {}", self.kw("RENAME TO"), new_name)
            }
            AlterFunctionAction::OwnerTo { owner } => format!("{} {}", self.kw("OWNER TO"), owner),
            AlterFunctionAction::SetSchema { schema } => {
                format!("{} {}", self.kw("SET SCHEMA"), schema)
            }
            AlterFunctionAction::Set { parameter, value } => format!(
                "{} {} {} {}",
                self.kw("SET"),
                parameter,
                self.kw("TO"),
                value
            ),
            AlterFunctionAction::Reset { parameter } => {
                format!("{} {}", self.kw("RESET"), parameter)
            }
        }
    }

    fn format_alter_role(&self, stmt: &AlterRoleStatement) -> String {
        let opts: Vec<String> = stmt
            .options
            .iter()
            .map(|(k, v)| match v {
                Some(val) => format!("{} {}", k, val),
                None => k.clone(),
            })
            .collect();
        format!("{} {} {}", self.kw("ALTER ROLE"), stmt.name, opts.join(" "))
    }

    fn format_alter_user(&self, stmt: &AlterUserStatement) -> String {
        let opts: Vec<String> = stmt
            .options
            .iter()
            .map(|(k, v)| match v {
                Some(val) => format!("{} {}", k, val),
                None => k.clone(),
            })
            .collect();
        format!("{} {} {}", self.kw("ALTER USER"), stmt.name, opts.join(" "))
    }

    fn format_alter_global_config(&self, stmt: &AlterGlobalConfigStatement) -> String {
        match &stmt.action {
            AlterGlobalConfigAction::Set { parameter, value } => format!(
                "{} {} {} {}",
                self.kw("ALTER SYSTEM SET"),
                parameter,
                self.kw("="),
                value
            ),
            AlterGlobalConfigAction::Reset { parameter } => {
                format!("{} {}", self.kw("ALTER SYSTEM RESET"), parameter)
            }
        }
    }

    // ── Additional formatters ──

    fn format_routine_param(&self, param: &RoutineParam) -> String {
        let mut s = param.name.clone();
        if let Some(ref mode) = param.mode {
            s.push(' ');
            s.push_str(mode);
        }
        s.push(' ');
        s.push_str(&param.data_type);
        if let Some(ref default_val) = param.default_value {
            s.push_str(&format!(" {} {}", self.kw("DEFAULT"), default_val));
        }
        s
    }

    fn format_create_function(&self, stmt: &CreateFunctionStatement) -> String {
        let mut s = self.kw("CREATE").to_string();
        if stmt.replace {
            s.push_str(&format!(" {} {}", self.kw("OR"), self.kw("REPLACE")));
        }
        s.push_str(&format!(
            "{} {}",
            self.kw(" FUNCTION"),
            self.format_object_name(&stmt.name)
        ));
        let params: Vec<String> = stmt
            .parameters
            .iter()
            .map(|p| self.format_routine_param(p))
            .collect();
        s.push_str(&format!("({})", params.join(", ")));
        if let Some(rt) = &stmt.return_type {
            s.push_str(&format!(" {} {}", self.kw("RETURNS"), rt));
        }
        if let Some(block) = &stmt.block {
            s.push_str(&format!(
                " {} $$ {} $$",
                self.kw("AS"),
                self.format_pl_block(block)
            ));
        } else if !stmt.options.is_empty() {
            s.push_str(&format!(" {}", stmt.options));
        }
        s
    }

    fn format_create_procedure(&self, stmt: &CreateProcedureStatement) -> String {
        let mut s = self.kw("CREATE").to_string();
        if stmt.replace {
            s.push_str(&format!(" {} {}", self.kw("OR"), self.kw("REPLACE")));
        }
        s.push_str(&format!(
            "{} {}",
            self.kw(" PROCEDURE"),
            self.format_object_name(&stmt.name)
        ));
        let params: Vec<String> = stmt
            .parameters
            .iter()
            .map(|p| self.format_routine_param(p))
            .collect();
        s.push_str(&format!("({})", params.join(", ")));
        if let Some(block) = &stmt.block {
            s.push_str(&format!(
                " {} $$ {} $$",
                self.kw("AS"),
                self.format_pl_block(block)
            ));
        } else if !stmt.options.is_empty() {
            s.push_str(&format!(" {}", stmt.options));
        }
        s
    }

    fn format_create_package(&self, stmt: &CreatePackageStatement) -> String {
        let mut s = self.kw("CREATE").to_string();
        if stmt.replace {
            s.push_str(&format!(" {}", self.kw("OR REPLACE")));
        }
        s.push_str(&format!(
            " {} {}",
            self.kw("PACKAGE"),
            self.format_object_name(&stmt.name)
        ));
        if let Some(authid) = &stmt.authid {
            match authid {
                PackageAuthid::CurrentUser => {
                    s.push_str(&format!(" {}", self.kw("AUTHID CURRENT_USER")))
                }
                PackageAuthid::Definer => s.push_str(&format!(" {}", self.kw("AUTHID DEFINER"))),
            }
        }
        s.push_str(&format!(" {} {}", self.kw("AS"), stmt.body));
        s
    }

    fn format_create_package_body(&self, stmt: &CreatePackageBodyStatement) -> String {
        let mut s = self.kw("CREATE").to_string();
        if stmt.replace {
            s.push_str(&format!(" {}", self.kw("OR REPLACE")));
        }
        s.push_str(&format!(
            " {} {}",
            self.kw("PACKAGE BODY"),
            self.format_object_name(&stmt.name)
        ));
        s.push_str(&format!(" {} {}", self.kw("AS"), stmt.body));
        s
    }

    fn format_create_extension(&self, stmt: &CreateExtensionStatement) -> String {
        let mut s = self.kw("CREATE").to_string();
        if stmt.replace {
            s.push_str(&format!(" {}", self.kw("OR REPLACE")));
        }
        s.push_str(&format!(" {}", self.kw("EXTENSION")));
        if stmt.if_not_exists {
            s.push_str(&format!(" {}", self.kw("IF NOT EXISTS")));
        }
        s.push_str(&format!(" {}", stmt.name));
        if let Some(schema) = &stmt.schema {
            s.push_str(&format!(" {} {}", self.kw("SCHEMA"), schema));
        }
        if let Some(version) = &stmt.version {
            s.push_str(&format!(" {} {}", self.kw("VERSION"), version));
        }
        if stmt.cascade {
            s.push_str(&format!(" {}", self.kw("CASCADE")));
        }
        s
    }

    fn format_create_role(&self, stmt: &CreateRoleStatement) -> String {
        format!("{} {}", self.kw("CREATE ROLE"), stmt.name)
    }

    fn format_create_user(&self, stmt: &CreateUserStatement) -> String {
        format!("{} {}", self.kw("CREATE USER"), stmt.name)
    }

    fn format_create_group(&self, stmt: &CreateGroupStatement) -> String {
        format!("{} {}", self.kw("CREATE GROUP"), stmt.name)
    }

    fn format_grant_role(&self, stmt: &GrantRoleStatement) -> String {
        let mut s = format!("{} {}", self.kw("GRANT"), stmt.roles.join(", "));
        s.push_str(&format!(" {} {}", self.kw("TO"), stmt.grantees.join(", ")));
        if stmt.with_admin_option {
            s.push_str(&format!(" {}", self.kw("WITH ADMIN OPTION")));
        }
        if let Some(by) = &stmt.granted_by {
            s.push_str(&format!(" {} {}", self.kw("GRANTED BY"), by));
        }
        s
    }

    fn format_revoke_role(&self, stmt: &RevokeRoleStatement) -> String {
        let mut s = format!("{} {}", self.kw("REVOKE"), stmt.roles.join(", "));
        s.push_str(&format!(
            " {} {}",
            self.kw("FROM"),
            stmt.grantees.join(", ")
        ));
        if stmt.cascade {
            s.push_str(&format!(" {}", self.kw("CASCADE")));
        }
        if let Some(by) = &stmt.granted_by {
            s.push_str(&format!(" {} {}", self.kw("GRANTED BY"), by));
        }
        s
    }

    fn format_alter_group(&self, stmt: &AlterGroupStatement) -> String {
        let action = match &stmt.action {
            AlterGroupAction::AddUser(user) => format!("{} {}", self.kw("ADD USER"), user),
            AlterGroupAction::DropUser(user) => format!("{} {}", self.kw("DROP USER"), user),
        };
        format!("{} {} {}", self.kw("ALTER GROUP"), stmt.name, action)
    }

    fn format_alter_composite_type(&self, stmt: &AlterCompositeTypeStatement) -> String {
        let action = match &stmt.action {
            AlterTypeAction::AddAttribute {
                name,
                data_type,
                cascade,
            } => {
                let mut s = format!(
                    "{} {} {} {}",
                    self.kw("ADD ATTRIBUTE"),
                    name,
                    self.kw("TYPE"),
                    data_type
                );
                if *cascade {
                    s.push_str(&format!(" {}", self.kw("CASCADE")));
                }
                s
            }
            AlterTypeAction::DropAttribute {
                name,
                if_exists,
                cascade,
            } => {
                let mut s = format!("{} {}", self.kw("DROP ATTRIBUTE"), name);
                if *if_exists {
                    s.push_str(&format!(" {}", self.kw("IF EXISTS")));
                }
                if *cascade {
                    s.push_str(&format!(" {}", self.kw("CASCADE")));
                }
                s
            }
            AlterTypeAction::RenameAttribute {
                old_name,
                new_name,
                cascade,
            } => {
                let mut s = format!(
                    "{} {} {} {} {}",
                    self.kw("RENAME ATTRIBUTE"),
                    old_name,
                    self.kw("TO"),
                    new_name,
                    ""
                );
                if *cascade {
                    s.push_str(&format!(" {}", self.kw("CASCADE")));
                }
                s.trim_end().to_string()
            }
            AlterTypeAction::RenameTo(new_name) => format!("{} {}", self.kw("RENAME TO"), new_name),
            AlterTypeAction::SetSchema(schema) => format!("{} {}", self.kw("SET SCHEMA"), schema),
            AlterTypeAction::OwnerTo(owner) => format!("{} {}", self.kw("OWNER TO"), owner),
            AlterTypeAction::AddEnumValue {
                value,
                before,
                after,
            } => {
                let mut s = format!("{} {} {} {}", self.kw("ADD VALUE"), value, "", "");
                if let Some(b) = before {
                    s = format!(
                        "{} {} {} {}",
                        self.kw("ADD VALUE"),
                        value,
                        self.kw("BEFORE"),
                        b
                    );
                }
                if let Some(a) = after {
                    s = format!(
                        "{} {} {} {}",
                        self.kw("ADD VALUE"),
                        value,
                        self.kw("AFTER"),
                        a
                    );
                }
                s.trim().to_string()
            }
            AlterTypeAction::RenameEnumValue {
                old_value,
                new_value,
            } => {
                format!(
                    "{} {} {} {}",
                    self.kw("RENAME VALUE"),
                    old_value,
                    self.kw("TO"),
                    new_value
                )
            }
        };
        format!(
            "{} {} {}",
            self.kw("ALTER TYPE"),
            self.format_object_name(&stmt.name),
            action
        )
    }

    fn format_alter_view(&self, stmt: &AlterViewStatement) -> String {
        let action = match &stmt.action {
            AlterViewAction::RenameTo(new_name) => format!("{} {}", self.kw("RENAME TO"), new_name),
            AlterViewAction::Set(opts) => {
                format!("{} {}", self.kw("SET"), self.format_options(opts))
            }
            AlterViewAction::Reset(params) => {
                format!("{} {}", self.kw("RESET"), params.join(", "))
            }
            AlterViewAction::SetSchema(schema) => format!("{} {}", self.kw("SET SCHEMA"), schema),
            AlterViewAction::OwnerTo(owner) => format!("{} {}", self.kw("OWNER TO"), owner),
            AlterViewAction::AlterColumnDefault {
                column,
                set_default,
            } => match set_default {
                Some(val) => format!(
                    "{} {} {} {} {}",
                    self.kw("ALTER COLUMN"),
                    column,
                    self.kw("SET DEFAULT"),
                    "",
                    val
                ),
                None => format!(
                    "{} {} {}",
                    self.kw("ALTER COLUMN"),
                    column,
                    self.kw("DROP DEFAULT")
                ),
            },
        };
        format!(
            "{} {} {}",
            self.kw("ALTER VIEW"),
            self.format_object_name(&stmt.name),
            action
        )
    }

    fn format_alter_trigger(&self, stmt: &AlterTriggerStatement) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.kw("ALTER TRIGGER"),
            stmt.name,
            self.kw("ON"),
            self.format_object_name(&stmt.table),
            self.kw("RENAME TO"),
            stmt.new_name
        )
    }

    fn format_alter_extension(&self, stmt: &AlterExtensionStatement) -> String {
        format!(
            "{} {} {}",
            self.kw("ALTER EXTENSION"),
            stmt.name,
            stmt.action
        )
    }

    fn format_create_cast(&self, stmt: &CreateCastStatement) -> String {
        let mut s = format!(
            "{} ({}) {} ({})",
            self.kw("CREATE CAST"),
            self.format_data_type(&stmt.source_type),
            self.kw("AS"),
            self.format_data_type(&stmt.target_type)
        );
        match &stmt.method {
            CastMethod::WithFunction(func) => {
                s.push_str(&format!(" {} {}", self.kw("WITH FUNCTION"), func))
            }
            CastMethod::WithoutFunction => s.push_str(&format!(" {}", self.kw("WITHOUT FUNCTION"))),
            CastMethod::WithInout => s.push_str(&format!(" {}", self.kw("WITH INOUT"))),
        }
        if let Some(ctx) = &stmt.context {
            match ctx {
                CastContext::Implicit => s.push_str(&format!(" {}", self.kw("AS IMPLICIT"))),
                CastContext::Assignment => s.push_str(&format!(" {}", self.kw("AS ASSIGNMENT"))),
            }
        }
        s
    }

    fn format_create_domain(&self, stmt: &CreateDomainStatement) -> String {
        let mut s = format!(
            "{} {} {}",
            self.kw("CREATE DOMAIN"),
            self.format_object_name(&stmt.name),
            self.format_data_type(&stmt.data_type)
        );
        if let Some(def) = &stmt.default_value {
            s.push_str(&format!(
                " {} {}",
                self.kw("DEFAULT"),
                self.format_expr(def)
            ));
        }
        if stmt.not_null {
            s.push_str(&format!(" {}", self.kw("NOT NULL")));
        }
        if let Some(check) = &stmt.check {
            s.push_str(&format!(
                " {} ({})",
                self.kw("CHECK"),
                self.format_expr(check)
            ));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::token::tokenizer::Tokenizer;

    fn parse_one(sql: &str) -> Statement {
        let tokens = Tokenizer::new(sql).tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse();
        stmts.into_iter().next().unwrap()
    }

    #[test]
    fn test_format_select_simple() {
        let sql = "SELECT id, name FROM users";
        let stmt = parse_one(sql);
        let formatted = stmt.to_string();
        assert!(formatted.contains("SELECT"));
        assert!(formatted.contains("FROM"));
    }

    #[test]
    fn test_format_select_with_where() {
        let sql = "SELECT * FROM users WHERE id = 1";
        let stmt = parse_one(sql);
        let formatted = stmt.to_string();
        assert!(formatted.contains("WHERE"));
        assert!(formatted.contains("="));
    }

    #[test]
    fn test_format_insert_values() {
        let sql = "INSERT INTO users (id, name) VALUES (1, 'Alice')";
        let stmt = parse_one(sql);
        let formatted = stmt.to_string();
        assert!(formatted.contains("INSERT INTO"));
        assert!(formatted.contains("VALUES"));
    }

    #[test]
    fn test_format_update() {
        let sql = "UPDATE users SET name = 'Bob' WHERE id = 1";
        let stmt = parse_one(sql);
        let formatted = stmt.to_string();
        assert!(formatted.contains("UPDATE"));
        assert!(formatted.contains("SET"));
        assert!(formatted.contains("WHERE"));
    }

    #[test]
    fn test_format_delete() {
        let sql = "DELETE FROM users WHERE id = 1";
        let stmt = parse_one(sql);
        let formatted = stmt.to_string();
        assert!(formatted.contains("DELETE FROM"));
        assert!(formatted.contains("WHERE"));
    }

    #[test]
    fn test_format_create_table() {
        let sql = "CREATE TABLE users (id INTEGER PRIMARY KEY, name VARCHAR(100) NOT NULL)";
        let stmt = parse_one(sql);
        let formatted = stmt.to_string();
        assert!(formatted.contains("CREATE TABLE"));
        assert!(formatted.contains("INTEGER"));
        assert!(formatted.contains("VARCHAR"));
    }

    #[test]
    fn test_format_create_index() {
        let sql = "CREATE INDEX idx_name ON users (name)";
        let stmt = parse_one(sql);
        let formatted = stmt.to_string();
        assert!(formatted.contains("CREATE INDEX"));
        assert!(formatted.contains("ON"));
    }

    #[test]
    fn test_format_drop_table() {
        let sql = "DROP TABLE IF EXISTS users CASCADE";
        let stmt = parse_one(sql);
        let formatted = stmt.to_string();
        assert!(formatted.contains("DROP TABLE"));
        assert!(formatted.contains("IF EXISTS"));
        assert!(formatted.contains("CASCADE"));
    }

    #[test]
    fn test_format_transaction_begin() {
        let sql = "BEGIN";
        let stmt = parse_one(sql);
        let formatted = stmt.to_string();
        assert_eq!(formatted, "BEGIN");
    }

    #[test]
    fn test_format_string_escape() {
        let sql = "SELECT 'it''s a test'";
        let stmt = parse_one(sql);
        let formatted = stmt.to_string();
        assert!(formatted.contains("''"));
    }

    #[test]
    fn test_roundtrip_select() {
        let sql = "SELECT id, name FROM users WHERE status = 'active'";
        let stmt1 = parse_one(sql);
        let formatted = stmt1.to_string();
        let stmt2 = parse_one(&formatted);
        assert_eq!(stmt1, stmt2);
    }

    #[test]
    fn test_roundtrip_insert() {
        let sql = "INSERT INTO users (id, name) VALUES (1, 'Test')";
        let stmt1 = parse_one(sql);
        let formatted = stmt1.to_string();
        let stmt2 = parse_one(&formatted);
        assert_eq!(stmt1, stmt2);
    }

    #[test]
    fn test_format_truncate() {
        let sql = "TRUNCATE TABLE users CASCADE";
        let stmt = parse_one(sql);
        let formatted = stmt.to_string();
        assert!(formatted.contains("TRUNCATE TABLE"));
        assert!(formatted.contains("CASCADE"));
    }

    #[test]
    fn test_format_create_view() {
        let sql = "CREATE VIEW active_users AS SELECT * FROM users WHERE status = 'active'";
        let stmt = parse_one(sql);
        let formatted = stmt.to_string();
        assert!(formatted.contains("CREATE VIEW"));
        assert!(formatted.contains("AS SELECT"));
    }

    #[test]
    fn test_format_explain() {
        let sql = "EXPLAIN SELECT * FROM users";
        let stmt = parse_one(sql);
        let formatted = stmt.to_string();
        assert!(formatted.contains("EXPLAIN"));
    }
}
