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
            Statement::CreateFunction(_) => "CREATE FUNCTION",
            Statement::CreateProcedure(_) => "CREATE PROCEDURE",
            Statement::CreatePackage(_) => "CREATE PACKAGE",
            Statement::CreateMaterializedView(_) => "CREATE MATERIALIZED VIEW",
            Statement::CreateTrigger(_) => "CREATE TRIGGER",
            Statement::CreateExtension(_) => "CREATE EXTENSION",
            Statement::CreateRole(_) => "CREATE ROLE",
            Statement::CreateUser(_) => "CREATE USER",
            Statement::CreateGroup(_) => "CREATE GROUP",
            Statement::Grant(_) => "GRANT",
            Statement::Revoke(_) => "REVOKE",
            Statement::Vacuum(_) => "VACUUM",
            Statement::Do(_) => "DO",
            Statement::Prepare(_) => "PREPARE",
            Statement::Execute(_) => "EXECUTE",
            Statement::Deallocate(_) => "DEALLOCATE",
            Statement::Comment(_) => "COMMENT",
            Statement::Lock(_) => "LOCK",
            Statement::DeclareCursor(_) => "DECLARE CURSOR",
            Statement::ClosePortal(_) => "CLOSE",
            Statement::Fetch(_) => "FETCH",
            Statement::Cluster(_) => "CLUSTER",
            Statement::Reindex(_) => "REINDEX",
            Statement::Listen(_) => "LISTEN",
            Statement::Notify(_) => "NOTIFY",
            Statement::Unlisten(_) => "UNLISTEN",
            Statement::Rule(_) => "CREATE RULE",
            Statement::DropRule(_) => "DROP RULE",
            Statement::CreateCast(_) => "CREATE CAST",
            Statement::CreateConversion(_) => "CREATE CONVERSION",
            Statement::CreateDomain(_) => "CREATE DOMAIN",
            Statement::AlterDomain(_) => "ALTER DOMAIN",
            Statement::CreateForeignTable(_) => "CREATE FOREIGN TABLE",
            Statement::CreateForeignServer(_) => "CREATE SERVER",
            Statement::CreateFdw(_) => "CREATE FOREIGN DATA WRAPPER",
            Statement::CreatePublication(_) => "CREATE PUBLICATION",
            Statement::CreateSubscription(_) => "CREATE SUBSCRIPTION",
            Statement::CreateSynonym(_) => "CREATE SYNONYM",
            Statement::CreateModel(_) => "CREATE MODEL",
            Statement::CreateAm(_) => "CREATE ACCESS METHOD",
            Statement::CreateDirectory(_) => "CREATE DIRECTORY",
            Statement::CreateNode(_) => "CREATE NODE",
            Statement::CreateNodeGroup(_) => "CREATE NODE GROUP",
            Statement::CreateResourcePool(_) => "CREATE RESOURCE POOL",
            Statement::CreateWorkloadGroup(_) => "CREATE WORKLOAD GROUP",
            Statement::CreateAuditPolicy(_) => "CREATE AUDIT POLICY",
            Statement::CreateMaskingPolicy(_) => "CREATE MASKING POLICY",
            Statement::CreateRlsPolicy(_) => "CREATE RLS POLICY",
            Statement::CreateDataSource(_) => "CREATE DATA SOURCE",
            Statement::CreateEvent(_) => "CREATE EVENT",
            Statement::CreateOpClass(_) => "CREATE OPERATOR CLASS",
            Statement::CreateOpFamily(_) => "CREATE OPERATOR FAMILY",
            Statement::CreateContQuery(_) => "CREATE CONTINUOUS QUERY",
            Statement::CreateStream(_) => "CREATE STREAM",
            Statement::CreateKey(_) => "CREATE KEY",
            Statement::CreatePackageBody(_) => "CREATE PACKAGE BODY",
            Statement::AlterFunction(_) => "ALTER FUNCTION",
            Statement::AlterProcedure(_) => "ALTER PROCEDURE",
            Statement::AlterSchema(_) => "ALTER SCHEMA",
            Statement::AlterDatabase(_) => "ALTER DATABASE",
            Statement::AlterRole(_) => "ALTER ROLE",
            Statement::AlterUser(_) => "ALTER USER",
            Statement::AlterGroup(_) => "ALTER GROUP",
            Statement::AlterSequence(_) => "ALTER SEQUENCE",
            Statement::AlterExtension(_) => "ALTER EXTENSION",
            Statement::AlterCompositeType(_) => "ALTER TYPE",
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
            Statement::AlterGlobalConfig(_) => "ALTER GLOBAL CONFIG",
            Statement::RefreshMaterializedView(_) => "REFRESH MATERIALIZED VIEW",
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
            Statement::AnonyBlock(_) => "ANONYMOUS BLOCK",
            Statement::RemovePackage(_) => "REMOVE PACKAGE",
            Statement::SecLabel(_) => "SECURITY LABEL",
            Statement::CreateWeakPasswordDictionary => "CREATE WEAK PASSWORD DICTIONARY",
            Statement::DropWeakPasswordDictionary => "DROP WEAK PASSWORD DICTIONARY",
            Statement::CreatePolicyLabel(_) => "CREATE POLICY LABEL",
            Statement::AlterPolicyLabel(_) => "ALTER POLICY LABEL",
            Statement::DropPolicyLabel(_) => "DROP POLICY LABEL",
            Statement::GrantRole(_) => "GRANT ROLE",
            Statement::RevokeRole(_) => "REVOKE ROLE",
            Statement::Analyze(_) => "ANALYZE",
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
        if stmt.distinct {
            select_parts.push(self.kw("DISTINCT"));
        }
        select_parts.push(self.format_select_targets(&stmt.targets));
        parts.push(select_parts.join(" "));

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

        if !stmt.group_by.is_empty() {
            parts.push(format!(
                "{} {}",
                self.kw("GROUP BY"),
                self.format_exprs(&stmt.group_by)
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
        let mut parts = vec![cte.name.clone()];
        if !cte.columns.is_empty() {
            parts.push(format!("({})", cte.columns.join(", ")));
        }
        parts.push(self.kw("AS"));
        if let Some(mat) = cte.materialized {
            if mat {
                parts.push(self.kw("MATERIALIZED"));
            } else {
                parts.push(self.kw("NOT MATERIALIZED"));
            }
        }
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
                let mut result = format!("{}(", self.format_object_name(name));
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
                format!("{}::{}", self.format_expr(expr), type_name)
            }
            Expr::Parameter(n) => format!("${}", n),
            Expr::Array(elements) => {
                format!("{}[{}]", self.kw("ARRAY"), self.format_exprs(elements))
            }
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
        let mut parts = vec![frame.mode.clone()];

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
        match bound.direction.as_str() {
            "UNBOUNDED PRECEDING" => self.kw("UNBOUNDED PRECEDING"),
            "UNBOUNDED FOLLOWING" => self.kw("UNBOUNDED FOLLOWING"),
            "CURRENT ROW" => self.kw("CURRENT ROW"),
            d if d.ends_with(" PRECEDING") || d.ends_with(" FOLLOWING") => {
                if let Some(offset) = bound.offset {
                    format!("{} {}", offset, d)
                } else {
                    d.to_string()
                }
            }
            _ => bound.direction.clone(),
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
            AlterTableAction::AddConstraint(constraint) => {
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
            s.push_str(&format!(" USING ({})", expr));
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
        let stmts = parser.parse().unwrap();
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
