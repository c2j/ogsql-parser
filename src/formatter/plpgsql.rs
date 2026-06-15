use super::SqlFormatter;
// A3 exception: ast is a pure data module; this file references 180+ AST types.
use crate::ast::*;

impl SqlFormatter {
    pub(crate) fn format_anon_block(&self, stmt: &AnonyBlockStatement) -> String {
        self.format_pl_block(&stmt.block, 0)
    }

    pub(crate) fn pad(indent: usize) -> String {
        "  ".repeat(indent)
    }

    pub(crate) fn format_pl_block(&self, block: &crate::ast::plpgsql::PlBlock, indent: usize) -> String {
        self.format_pl_block_inner(block, indent, false)
    }

    pub(crate) fn format_pl_block_named(&self, block: &crate::ast::plpgsql::PlBlock, indent: usize) -> String {
        self.format_pl_block_inner(block, indent, true)
    }

    pub(crate) fn format_pl_block_inner(
        &self,
        block: &crate::ast::plpgsql::PlBlock,
        indent: usize,
        named: bool,
    ) -> String {
        let mut s = String::new();

        if let Some(ref label) = block.label {
            s.push_str(&format!("{}<<{}>> ", Self::pad(indent), label));
        }

        if !block.declarations.is_empty() {
            if !named {
                s.push('\n');
                s.push_str(&Self::pad(indent));
                s.push_str(&self.kw("DECLARE"));
            }
            for decl in &block.declarations {
                s.push('\n');
                s.push_str(&Self::pad(indent + 1));
                s.push_str(&self.format_pl_declaration(decl));
                s.push(';');
            }
        }

        s.push('\n');
        s.push_str(&Self::pad(indent));
        s.push_str(&self.kw("BEGIN"));
        for stmt in &block.body {
            s.push('\n');
            s.push_str(&Self::pad(indent + 1));
            s.push_str(&self.format_pl_statement(stmt, indent + 1));
        }

        if let Some(ref exc) = block.exception_block {
            s.push('\n');
            s.push_str(&Self::pad(indent));
            s.push_str(&self.kw("EXCEPTION"));
            for handler in &exc.handlers {
                s.push('\n');
                s.push_str(&Self::pad(indent + 1));
                s.push_str(&self.kw("WHEN"));
                s.push(' ');
                s.push_str(&handler.conditions.join(" OR "));
                s.push(' ');
                s.push_str(&self.kw("THEN"));
                for stmt in &handler.statements {
                    s.push('\n');
                    s.push_str(&Self::pad(indent + 2));
                    s.push_str(&self.format_pl_statement(stmt, indent + 2));
                }
            }
        }

        s.push('\n');
        s.push_str(&Self::pad(indent));
        s.push_str(&self.kw("END"));

        if let Some(ref label) = block.end_label {
            s.push_str(&format!(" {}", label));
        }

        s
    }

    pub(crate) fn format_pl_declaration(&self, decl: &crate::ast::plpgsql::PlDeclaration) -> String {
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
                let mut s = format!("{} {} ", self.kw("CURSOR"), c.name);
                if !c.arguments.is_empty() {
                    s.push('(');
                    s.push_str(
                        &c.arguments
                            .iter()
                            .map(|a| {
                                let mode_str = match a.mode {
                                    PlArgMode::In => self.kw("IN"),
                                    PlArgMode::Out => self.kw("OUT"),
                                    PlArgMode::InOut => format!("{} {}", self.kw("IN"), self.kw("OUT")),
                                };
                                format!("{} {} {}", a.name, mode_str, self.format_pl_data_type(&a.data_type))
                            })
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                    s.push_str(") ");
                }
                if !c.query.is_empty() {
                    s.push_str(&format!("{} {}", self.kw("IS"), c.query));
                }
                s
            }
            PlDeclaration::Record(r) => format!("{} {}", r.name, self.kw("RECORD")),
            PlDeclaration::Type(t) => match t {
                PlTypeDecl::Record { name, fields } => {
                    let fields_str = fields
                        .iter()
                        .map(|f| format!("{} {}", f.name, self.format_pl_data_type(&f.data_type)))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{} {} ({})", name, self.kw("TYPE IS RECORD"), fields_str)
                }
                PlTypeDecl::TableOf { name, elem_type, index_by } => {
                    let mut s = format!(
                        "{} {} {} {}",
                        name,
                        self.kw("TYPE IS TABLE OF"),
                        self.format_pl_data_type(elem_type),
                        ""
                    );
                    if let Some(idx) = index_by {
                        s = format!("{} {} {} {}", s.trim(), self.kw("INDEX BY"), self.format_pl_data_type(idx), "");
                    }
                    s.trim().to_string()
                }
                PlTypeDecl::VarrayOf { name, size, elem_type } => {
                    format!(
                        "{} {} ({}) {} {}",
                        name,
                        self.kw("TYPE IS VARRAY"),
                        self.format_expr(size),
                        self.kw("OF"),
                        self.format_pl_data_type(elem_type)
                    )
                }
                PlTypeDecl::RefCursor { name } => {
                    format!("{} {}", name, self.kw("TYPE IS REF CURSOR"))
                }
            },
            PlDeclaration::NestedProcedure(p) => {
                let mut s = format!("{} {} ", self.kw("PROCEDURE"), self.format_object_name(&p.name));
                if !p.parameters.is_empty() {
                    let params: Vec<String> = p.parameters.iter().map(|p| self.format_routine_param(p)).collect();
                    s.push_str(&format!("({})", params.join(", ")));
                }
                if let Some(ref block) = p.block {
                    s.push_str(&format!(" {} {}", self.kw("AS"), self.format_pl_block(block, 0)));
                }
                s
            }
            PlDeclaration::NestedFunction(f) => {
                let mut s = format!("{} {} ", self.kw("FUNCTION"), self.format_object_name(&f.name));
                if !f.parameters.is_empty() {
                    let params: Vec<String> = f.parameters.iter().map(|p| self.format_routine_param(p)).collect();
                    s.push_str(&format!("({})", params.join(", ")));
                }
                if let Some(ref rt) = f.return_type {
                    s.push_str(&format!(" {} {}", self.kw("RETURN"), rt));
                }
                if let Some(ref block) = f.block {
                    s.push_str(&format!(" {} {}", self.kw("AS"), self.format_pl_block(block, 0)));
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

    pub(crate) fn format_pl_data_type(&self, dt: &crate::ast::plpgsql::PlDataType) -> String {
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

    pub(crate) fn format_pl_statement(&self, stmt: &crate::ast::plpgsql::PlStatement, indent: usize) -> String {
        use crate::ast::plpgsql::*;
        match stmt {
            PlStatement::Block(b) => format!("{};", self.format_pl_block(b, indent)),
            PlStatement::Assignment { target, expression } => {
                format!("{} := {};", self.format_expr(target), self.format_expr(expression))
            }
            PlStatement::SetTransaction { isolation_level, read_only, deferrable } => {
                let mut s = format!("{} {}", self.kw("SET"), self.kw("TRANSACTION"));
                if let Some(ref il) = isolation_level {
                    s.push_str(&format!(" {} {}", self.kw("ISOLATION"), self.kw("LEVEL")));
                    match il {
                        PlIsolationLevel::ReadCommitted => {
                            s.push_str(&format!(" {} {}", self.kw("READ"), self.kw("COMMITTED")))
                        }
                        PlIsolationLevel::RepeatableRead => {
                            s.push_str(&format!(" {} {}", self.kw("REPEATABLE"), self.kw("READ")))
                        }
                        PlIsolationLevel::Serializable => s.push_str(&format!(" {}", self.kw("SERIALIZABLE"))),
                    }
                }
                if let Some(ro) = read_only {
                    if *ro {
                        s.push_str(&format!(" {} {}", self.kw("READ"), self.kw("ONLY")));
                    } else {
                        s.push_str(&format!(" {} {}", self.kw("READ"), self.kw("WRITE")));
                    }
                }
                if let Some(d) = deferrable {
                    if *d {
                        s.push_str(&format!(" {}", self.kw("DEFERRABLE")));
                    } else {
                        s.push_str(&format!(" {} {}", self.kw("NOT"), self.kw("DEFERRABLE")));
                    }
                }
                format!("{};", s)
            }
            PlStatement::Null => format!("{};", self.kw("NULL")),
            PlStatement::If(i) => self.format_pl_if(i, indent),
            PlStatement::Case(c) => self.format_pl_case(c, indent),
            PlStatement::Loop(l) => self.format_pl_loop(l, indent),
            PlStatement::While(w) => self.format_pl_while(w, indent),
            PlStatement::For(f) => self.format_pl_for(f, indent),
            PlStatement::ForEach(fe) => self.format_pl_foreach(fe, indent),
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
            PlStatement::ReturnNext { expression } => {
                format!("{} {} {};", self.kw("RETURN"), self.kw("NEXT"), self.format_expr(expression))
            }
            PlStatement::ReturnQuery(q) => {
                let mut s = format!("{} {}", self.kw("RETURN"), self.kw("QUERY"));
                if q.is_dynamic {
                    s.push_str(&format!(" {}", self.kw("EXECUTE")));
                    if let Some(ref expr) = q.dynamic_expr {
                        s.push_str(&format!(" {}", self.format_expr(expr)));
                    }
                    if !q.using_args.is_empty() {
                        let args: Vec<String> = q
                            .using_args
                            .iter()
                            .map(|a| {
                                let mode_str = match a.mode {
                                    PlUsingMode::In => self.kw("IN").to_string(),
                                    PlUsingMode::Out => self.kw("OUT").to_string(),
                                    PlUsingMode::InOut => {
                                        format!("{}{}", self.kw("IN"), self.kw("OUT"))
                                    }
                                };
                                format!("{} {}", mode_str, self.format_expr(&a.argument))
                            })
                            .collect();
                        s.push_str(&format!(" {} {}", self.kw("USING"), args.join(", ")));
                    }
                } else if !q.query.is_empty() {
                    s.push_str(&format!(" {}", q.query));
                } else if let Some(ref expr) = q.dynamic_expr {
                    s.push_str(&format!(" {}", self.format_expr(expr)));
                }
                format!("{};", s)
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
                    let targets: Vec<String> = e.into_targets.iter().map(|t| self.format_expr(t)).collect();
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
            PlStatement::Perform { query, parsed_expr, .. } => {
                if let Some(ref expr) = parsed_expr {
                    format!("{} {};", self.kw("PERFORM"), self.format_expr(expr))
                } else {
                    format!("{} {};", self.kw("PERFORM"), query)
                }
            }
            PlStatement::Open(o) => {
                let mut s = format!("{} {}", self.kw("OPEN"), self.format_expr(&o.cursor));
                match &o.kind {
                    PlOpenKind::Simple { arguments } => {
                        if !arguments.is_empty() {
                            let args: Vec<String> = arguments.iter().map(|a| self.format_expr(a)).collect();
                            s.push_str(&format!("({})", args.join(", ")));
                        }
                    }
                    PlOpenKind::ForQuery { scroll, query, .. } => {
                        if let Some(sc) = scroll {
                            if *sc {
                                s.push_str(&format!(" {}", self.kw("SCROLL")));
                            } else {
                                s.push_str(&format!(" {} {}", self.kw("NO"), self.kw("SCROLL")));
                            }
                        }
                        s.push_str(&format!(" {} {}", self.kw("FOR"), query));
                    }
                    PlOpenKind::ForExecute { query, using_args } => {
                        s.push_str(&format!(" {} {}", self.kw("FOR EXECUTE"), self.format_expr(query)));
                        if !using_args.is_empty() {
                            let args: Vec<String> = using_args.iter().map(|a| self.format_expr(a)).collect();
                            s.push_str(&format!(" {} {}", self.kw("USING"), args.join(", ")));
                        }
                    }
                    PlOpenKind::ForUsing { expressions } => {
                        let exprs: Vec<String> = expressions.iter().map(|e| self.format_expr(e)).collect();
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
                s.push_str(&self.format_expr(&f.cursor).to_string());
                if f.bulk_collect {
                    s.push_str(&format!(" {} {}", self.kw("BULK"), self.kw("COLLECT")));
                }
                s.push_str(&format!(
                    " {} {};",
                    self.kw("INTO"),
                    f.into.iter().map(|e| self.format_expr(e)).collect::<Vec<_>>().join(", ")
                ));
                s
            }
            PlStatement::Close { cursor } => format!("{} {};", self.kw("CLOSE"), self.format_expr(cursor)),
            PlStatement::Move { cursor, direction } => {
                let mut s = self.kw("MOVE").to_string();
                if let Some(ref dir) = direction {
                    s.push_str(&format!(" {} ", dir));
                }
                format!("{}{};", s, self.format_expr(cursor))
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
                        .map(|i| format!("{} = {}", self.format_expr(&i.target), i.item))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                format!("{};", s)
            }
            PlStatement::Commit { and_chain } => {
                if *and_chain {
                    format!("{} {} {};", self.kw("COMMIT"), self.kw("AND"), self.kw("CHAIN"))
                } else {
                    format!("{};", self.kw("COMMIT"))
                }
            }
            PlStatement::Rollback { to_savepoint, and_chain } => {
                let mut s = self.kw("ROLLBACK").to_string();
                if let Some(sp) = to_savepoint {
                    s.push_str(&format!(" {} {}", self.kw("TO"), sp));
                }
                if *and_chain {
                    s.push_str(&format!(" {} {}", self.kw("AND"), self.kw("CHAIN")));
                }
                format!("{};", s)
            }
            PlStatement::Savepoint { name } => format!("{} {};", self.kw("SAVEPOINT"), name),
            PlStatement::ReleaseSavepoint { name } => {
                format!("{} {} {};", self.kw("RELEASE"), self.kw("SAVEPOINT"), name)
            }
            PlStatement::SetTransaction { isolation_level, read_only, deferrable } => {
                let mut parts = vec![self.kw("SET"), self.kw("TRANSACTION")];
                if let Some(level) = isolation_level {
                    parts.push(self.kw("ISOLATION"));
                    parts.push(self.kw("LEVEL"));
                    parts.push(self.kw(match level {
                        crate::ast::plpgsql::PlIsolationLevel::ReadCommitted => "READ COMMITTED",
                        crate::ast::plpgsql::PlIsolationLevel::RepeatableRead => "REPEATABLE READ",
                        crate::ast::plpgsql::PlIsolationLevel::Serializable => "SERIALIZABLE",
                    }));
                }
                if let Some(ro) = read_only {
                    if *ro {
                        parts.push(self.kw("READ"));
                        parts.push(self.kw("ONLY"));
                    } else {
                        parts.push(self.kw("READ"));
                        parts.push(self.kw("WRITE"));
                    }
                }
                if let Some(d) = deferrable {
                    if *d {
                        parts.push(self.kw("DEFERRABLE"));
                    } else {
                        parts.push(self.kw("NOT"));
                        parts.push(self.kw("DEFERRABLE"));
                    }
                }
                format!("{};", parts.join(" "))
            }
            PlStatement::Goto { label } => format!("{} {};", self.kw("GOTO"), label),
            PlStatement::ProcedureCall(call) => {
                let name = call.name.join(".");
                let args: Vec<String> = call.arguments.iter().map(|a| self.format_expr(a)).collect();
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
            PlStatement::SqlStatement { sql_text, statement, .. } => {
                let _ = sql_text;
                let formatted = self.format_statement_flat(statement);
                let multiline = self.multiline_format_sql(&formatted, indent);
                format!("{};", multiline)
            }
            PlStatement::ForAll(f) => {
                let save_exceptions =
                    if f.save_exceptions { format!(" {}", self.kw("SAVE EXCEPTIONS")) } else { String::new() };
                format!("{} {} {}{} {}", self.kw("FORALL"), f.variable, f.bounds, save_exceptions, f.body)
            }
            PlStatement::PipeRow { expression } => {
                format!("{}({});", self.kw("PIPE ROW"), self.format_expr(expression))
            }
            PlStatement::VariableSet(set_stmt) => {
                format!("{};", self.format_variable_set(set_stmt))
            }
            PlStatement::VariableReset(reset_stmt) => {
                format!("{};", self.format_variable_reset(reset_stmt))
            }
        }
    }

    pub(crate) fn format_pl_if(&self, i: &crate::ast::plpgsql::PlIfStmt, indent: usize) -> String {
        let mut s = format!("{} {} {}", self.kw("IF"), self.format_expr(&i.condition), self.kw("THEN"));
        for stmt in &i.then_stmts {
            s.push('\n');
            s.push_str(&Self::pad(indent + 1));
            s.push_str(&self.format_pl_statement(stmt, indent + 1));
        }
        for elsif in &i.elsifs {
            s.push('\n');
            s.push_str(&Self::pad(indent));
            s.push_str(&format!("{} {} {}", self.kw("ELSIF"), self.format_expr(&elsif.condition), self.kw("THEN")));
            for stmt in &elsif.stmts {
                s.push('\n');
                s.push_str(&Self::pad(indent + 1));
                s.push_str(&self.format_pl_statement(stmt, indent + 1));
            }
        }
        if !i.else_stmts.is_empty() {
            s.push('\n');
            s.push_str(&Self::pad(indent));
            s.push_str(&self.kw("ELSE"));
            for stmt in &i.else_stmts {
                s.push('\n');
                s.push_str(&Self::pad(indent + 1));
                s.push_str(&self.format_pl_statement(stmt, indent + 1));
            }
        }
        s.push('\n');
        s.push_str(&Self::pad(indent));
        s.push_str(&format!("{};", self.kw("END IF")));
        s
    }

    pub(crate) fn format_pl_case(&self, c: &crate::ast::plpgsql::PlCaseStmt, indent: usize) -> String {
        let mut s = self.kw("CASE").to_string();
        if let Some(ref expr) = c.expression {
            s.push_str(&format!(" {}", self.format_expr(expr)));
        }
        for when in &c.whens {
            s.push('\n');
            s.push_str(&Self::pad(indent + 1));
            s.push_str(&format!("{} {} {}", self.kw("WHEN"), self.format_expr(&when.condition), self.kw("THEN")));
            for stmt in &when.stmts {
                s.push('\n');
                s.push_str(&Self::pad(indent + 2));
                s.push_str(&self.format_pl_statement(stmt, indent + 2));
            }
        }
        if !c.else_stmts.is_empty() {
            s.push('\n');
            s.push_str(&Self::pad(indent + 1));
            s.push_str(&self.kw("ELSE"));
            for stmt in &c.else_stmts {
                s.push('\n');
                s.push_str(&Self::pad(indent + 2));
                s.push_str(&self.format_pl_statement(stmt, indent + 2));
            }
        }
        s.push('\n');
        s.push_str(&Self::pad(indent));
        s.push_str(&format!("{} {};", self.kw("END"), self.kw("CASE")));
        s
    }

    pub(crate) fn format_pl_loop(&self, l: &crate::ast::plpgsql::PlLoopStmt, indent: usize) -> String {
        let mut s = String::new();
        if let Some(ref label) = l.label {
            s.push_str(&format!("{}<<{}>> ", Self::pad(indent), label));
        } else {
            s.push_str(&Self::pad(indent));
        }
        s.push_str(&self.kw("LOOP"));
        for stmt in &l.body {
            s.push('\n');
            s.push_str(&Self::pad(indent + 1));
            s.push_str(&self.format_pl_statement(stmt, indent + 1));
        }
        s.push('\n');
        s.push_str(&Self::pad(indent));
        s.push_str(&format!("{} {};", self.kw("END"), self.kw("LOOP")));
        if let Some(ref label) = l.end_label {
            s.push_str(&format!(" {}", label));
        }
        s
    }

    pub(crate) fn format_pl_while(&self, w: &crate::ast::plpgsql::PlWhileStmt, indent: usize) -> String {
        let mut s = String::new();
        if let Some(ref label) = w.label {
            s.push_str(&format!("{}<<{}>> ", Self::pad(indent), label));
        } else {
            s.push_str(&Self::pad(indent));
        }
        s.push_str(&format!("{} {} {}", self.kw("WHILE"), self.format_expr(&w.condition), self.kw("LOOP")));
        for stmt in &w.body {
            s.push('\n');
            s.push_str(&Self::pad(indent + 1));
            s.push_str(&self.format_pl_statement(stmt, indent + 1));
        }
        s.push('\n');
        s.push_str(&Self::pad(indent));
        s.push_str(&format!("{} {};", self.kw("END"), self.kw("LOOP")));
        if let Some(ref label) = w.end_label {
            s.push_str(&format!(" {}", label));
        }
        s
    }

    pub(crate) fn format_pl_for(&self, f: &crate::ast::plpgsql::PlForStmt, indent: usize) -> String {
        use crate::ast::plpgsql::PlForKind;
        let mut s = String::new();
        if let Some(ref label) = f.label {
            s.push_str(&format!("{}<<{}>> ", Self::pad(indent), label));
        } else {
            s.push_str(&Self::pad(indent));
        }
        s.push_str(&format!("{} {} {} ", self.kw("FOR"), f.variable, self.kw("IN")));
        match &f.kind {
            PlForKind::Range { low, high, step, reverse } => {
                if *reverse {
                    s.push_str(&format!("{} ", self.kw("REVERSE")));
                }
                s.push_str(&format!("{}..{}", self.format_expr(low), self.format_expr(high)));
                if let Some(ref st) = step {
                    s.push_str(&format!(" {} {}", self.kw("BY"), self.format_expr(st)));
                }
            }
            PlForKind::Query { query, .. } => {
                s.push_str(query);
            }
            PlForKind::Cursor { cursor_name, arguments } => {
                s.push_str(&self.format_expr(cursor_name));
                if !arguments.is_empty() {
                    let args: Vec<String> = arguments.iter().map(|a| self.format_expr(a)).collect();
                    s.push_str(&format!("({})", args.join(", ")));
                }
            }
        }
        s.push(' ');
        s.push_str(&self.kw("LOOP"));
        for stmt in &f.body {
            s.push('\n');
            s.push_str(&Self::pad(indent + 1));
            s.push_str(&self.format_pl_statement(stmt, indent + 1));
        }
        s.push('\n');
        s.push_str(&Self::pad(indent));
        s.push_str(&format!("{} {};", self.kw("END"), self.kw("LOOP")));
        if let Some(ref label) = f.end_label {
            s.push_str(&format!(" {}", label));
        }
        s
    }

    pub(crate) fn format_pl_foreach(&self, fe: &crate::ast::plpgsql::PlForEachStmt, indent: usize) -> String {
        let mut s = String::new();
        if let Some(ref label) = fe.label {
            s.push_str(&format!("{}<<{}>> ", Self::pad(indent), label));
        } else {
            s.push_str(&Self::pad(indent));
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
            s.push('\n');
            s.push_str(&Self::pad(indent + 1));
            s.push_str(&self.format_pl_statement(stmt, indent + 1));
        }
        s.push('\n');
        s.push_str(&Self::pad(indent));
        s.push_str(&format!("{} {};", self.kw("END"), self.kw("LOOP")));
        if let Some(ref label) = fe.end_label {
            s.push_str(&format!(" {}", label));
        }
        s
    }

    pub(crate) fn format_pl_raise(&self, r: &crate::ast::plpgsql::PlRaiseStmt) -> String {
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
            s.push_str(&format!(" {}", level_str));
        }
        if let Some(ref msg) = r.message {
            s.push_str(&format!(" '{}'", msg));
            if !r.params.is_empty() {
                let params: Vec<String> = r.params.iter().map(|p| self.format_expr(p)).collect();
                s.push_str(&format!(", {}", params.join(", ")));
            }
        } else if let Some(ref cond) = r.condname {
            s.push_str(&format!(" {}", cond));
        } else if let Some(ref state) = r.sqlstate {
            s.push_str(&format!(" SQLSTATE '{}'", state));
        }
        if !r.options.is_empty() {
            s.push_str(&format!(" {}", self.kw("USING")));
            let opts: Vec<String> =
                r.options.iter().map(|o| format!("{} = {}", o.name, self.format_expr(&o.value))).collect();
            s.push_str(&format!(" {}", opts.join(", ")));
        }
        format!("{};", s)
    }
}
