use super::*;

/// Result type for visitor operations.
/// Controls traversal behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitorResult {
    /// Continue visiting children
    Continue,
    /// Skip children of current node
    SkipChildren,
    /// Stop traversal entirely
    Stop,
}

/// AST Visitor trait for traversal and analysis.
/// Default implementations return Continue.
pub trait Visitor {
    fn visit_statement(&mut self, _stmt: &Statement) -> VisitorResult {
        VisitorResult::Continue
    }

    fn visit_expr(&mut self, _expr: &Expr) -> VisitorResult {
        VisitorResult::Continue
    }

    fn visit_select(&mut self, _select: &SelectStatement) -> VisitorResult {
        VisitorResult::Continue
    }

    fn visit_insert(&mut self, _insert: &InsertStatement) -> VisitorResult {
        VisitorResult::Continue
    }

    fn visit_update(&mut self, _update: &UpdateStatement) -> VisitorResult {
        VisitorResult::Continue
    }

    fn visit_delete(&mut self, _delete: &DeleteStatement) -> VisitorResult {
        VisitorResult::Continue
    }

    fn visit_create_table(&mut self, _table: &CreateTableStatement) -> VisitorResult {
        VisitorResult::Continue
    }

    fn visit_pl_block(&mut self, _block: &crate::ast::plpgsql::PlBlock) -> VisitorResult {
        VisitorResult::Continue
    }

    fn visit_pl_statement(&mut self, _stmt: &crate::ast::plpgsql::PlStatement) -> VisitorResult {
        VisitorResult::Continue
    }

    fn visit_pl_declaration(&mut self, _decl: &crate::ast::plpgsql::PlDeclaration) -> VisitorResult {
        VisitorResult::Continue
    }

    fn visit_pl_exception_handler(&mut self, _handler: &crate::ast::plpgsql::PlExceptionHandler) -> VisitorResult {
        VisitorResult::Continue
    }

    fn visit_call(&mut self, _call: &CallFuncStatement) -> VisitorResult {
        VisitorResult::Continue
    }

    fn visit_procedure_call(&mut self, _call: &crate::ast::plpgsql::PlProcedureCall) -> VisitorResult {
        VisitorResult::Continue
    }

    fn visit_table_ref(&mut self, _table_ref: &TableRef) -> VisitorResult {
        VisitorResult::Continue
    }
}

/// Walk a PL/pgSQL block.
pub fn walk_pl_block(visitor: &mut dyn Visitor, block: &crate::ast::plpgsql::PlBlock) -> VisitorResult {
    let result = visitor.visit_pl_block(block);
    match result {
        VisitorResult::Continue => {
            for decl in &block.declarations {
                if walk_pl_declaration(visitor, decl) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            for stmt in &block.body {
                if walk_pl_statement(visitor, stmt) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            if let Some(ref exception_block) = block.exception_block {
                for handler in &exception_block.handlers {
                    if visitor.visit_pl_exception_handler(handler) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                    for stmt in &handler.statements {
                        if walk_pl_statement(visitor, stmt) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                }
            }
            VisitorResult::Continue
        }
        VisitorResult::SkipChildren => VisitorResult::Continue,
        VisitorResult::Stop => VisitorResult::Stop,
    }
}

/// Walk a PL/pgSQL statement.
pub fn walk_pl_statement(visitor: &mut dyn Visitor, stmt: &crate::ast::plpgsql::PlStatement) -> VisitorResult {
    let result = visitor.visit_pl_statement(stmt);
    match result {
        VisitorResult::Continue => {
            match stmt {
                crate::ast::plpgsql::PlStatement::Block(block) => {
                    walk_pl_block(visitor, block)
                }
                crate::ast::plpgsql::PlStatement::Assignment { target, expression } => {
                    if walk_expr(visitor, target) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                    walk_expr(visitor, expression)
                }
                crate::ast::plpgsql::PlStatement::If(if_stmt) => {
                    if walk_expr(visitor, &if_stmt.condition) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                    for stmt in &if_stmt.then_stmts {
                        if walk_pl_statement(visitor, stmt) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    for elsif in &if_stmt.elsifs {
                        if walk_expr(visitor, &elsif.condition) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                        for stmt in &elsif.stmts {
                            if walk_pl_statement(visitor, stmt) == VisitorResult::Stop {
                                return VisitorResult::Stop;
                            }
                        }
                    }
                    for stmt in &if_stmt.else_stmts {
                        if walk_pl_statement(visitor, stmt) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::Case(case_stmt) => {
                    if let Some(ref expr) = case_stmt.expression {
                        if walk_expr(visitor, expr) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    for when in &case_stmt.whens {
                        if walk_expr(visitor, &when.condition) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                        for stmt in &when.stmts {
                            if walk_pl_statement(visitor, stmt) == VisitorResult::Stop {
                                return VisitorResult::Stop;
                            }
                        }
                    }
                    for stmt in &case_stmt.else_stmts {
                        if walk_pl_statement(visitor, stmt) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::Loop(loop_stmt) => {
                    for stmt in &loop_stmt.body {
                        if walk_pl_statement(visitor, stmt) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::While(while_stmt) => {
                    if walk_expr(visitor, &while_stmt.condition) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                    for stmt in &while_stmt.body {
                        if walk_pl_statement(visitor, stmt) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::For(for_stmt) => {
                    match &for_stmt.kind {
                        crate::ast::plpgsql::PlForKind::Range { low, high, step, .. } => {
                            if walk_expr(visitor, low) == VisitorResult::Stop {
                                return VisitorResult::Stop;
                            }
                            if walk_expr(visitor, high) == VisitorResult::Stop {
                                return VisitorResult::Stop;
                            }
                            if let Some(step) = step {
                                if walk_expr(visitor, step) == VisitorResult::Stop {
                                    return VisitorResult::Stop;
                                }
                            }
                        }
                        crate::ast::plpgsql::PlForKind::Query { parsed_query, using_args, .. } => {
                            if let Some(ref query) = parsed_query {
                                if walk_statement(visitor, query) == VisitorResult::Stop {
                                    return VisitorResult::Stop;
                                }
                            }
                            for arg in using_args {
                                if walk_expr(visitor, &arg.argument) == VisitorResult::Stop {
                                    return VisitorResult::Stop;
                                }
                            }
                        }
                        crate::ast::plpgsql::PlForKind::Cursor { cursor_name, arguments } => {
                            if walk_expr(visitor, cursor_name) == VisitorResult::Stop {
                                return VisitorResult::Stop;
                            }
                            for arg in arguments {
                                if walk_expr(visitor, arg) == VisitorResult::Stop {
                                    return VisitorResult::Stop;
                                }
                            }
                        }
                    }
                    for stmt in &for_stmt.body {
                        if walk_pl_statement(visitor, stmt) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::ForEach(foreach_stmt) => {
                    if walk_expr(visitor, &foreach_stmt.expression) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                    for stmt in &foreach_stmt.body {
                        if walk_pl_statement(visitor, stmt) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::Exit { condition, .. } |
                crate::ast::plpgsql::PlStatement::Continue { condition, .. } => {
                    if let Some(ref condition) = condition {
                        if walk_expr(visitor, condition) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::Return { expression } => {
                    if let Some(ref expression) = expression {
                        if walk_expr(visitor, expression) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::ReturnNext { expression } => {
                    walk_expr(visitor, expression)
                }
                crate::ast::plpgsql::PlStatement::ReturnQuery(return_query) => {
                    if let Some(ref dynamic_expr) = return_query.dynamic_expr {
                        if walk_expr(visitor, dynamic_expr) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    for arg in &return_query.using_args {
                        if walk_expr(visitor, &arg.argument) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::Raise(raise_stmt) => {
                    for param in &raise_stmt.params {
                        if walk_expr(visitor, param) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    for option in &raise_stmt.options {
                        if walk_expr(visitor, &option.value) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::Execute(execute_stmt) => {
                    if walk_expr(visitor, &execute_stmt.string_expr) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                    for target in &execute_stmt.into_targets {
                        if walk_expr(visitor, target) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    for arg in &execute_stmt.using_args {
                        if walk_expr(visitor, &arg.argument) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    if let Some(ref parsed_query) = execute_stmt.parsed_query {
                        if walk_statement(visitor, parsed_query) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::Perform { parsed_query, parsed_expr, .. } => {
                    if let Some(ref query) = parsed_query {
                        if walk_statement(visitor, query) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    if let Some(ref expr) = parsed_expr {
                        if walk_expr(visitor, expr) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::Open(open_stmt) => {
                    match &open_stmt.kind {
                        crate::ast::plpgsql::PlOpenKind::Simple { arguments } => {
                            for arg in arguments {
                                if walk_expr(visitor, arg) == VisitorResult::Stop {
                                    return VisitorResult::Stop;
                                }
                            }
                        }
                        crate::ast::plpgsql::PlOpenKind::ForQuery { parsed_query, .. } => {
                            if let Some(ref query) = parsed_query {
                                if walk_statement(visitor, query) == VisitorResult::Stop {
                                    return VisitorResult::Stop;
                                }
                            }
                        }
                        crate::ast::plpgsql::PlOpenKind::ForExecute { query, using_args } => {
                            if walk_expr(visitor, query) == VisitorResult::Stop {
                                return VisitorResult::Stop;
                            }
                            for arg in using_args {
                                if walk_expr(visitor, arg) == VisitorResult::Stop {
                                    return VisitorResult::Stop;
                                }
                            }
                        }
                        crate::ast::plpgsql::PlOpenKind::ForUsing { expressions } => {
                            for expr in expressions {
                                if walk_expr(visitor, expr) == VisitorResult::Stop {
                                    return VisitorResult::Stop;
                                }
                            }
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::Fetch(fetch_stmt) => {
                    for expr in &fetch_stmt.into {
                        if walk_expr(visitor, expr) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::ProcedureCall(proc_call) => {
                    if visitor.visit_procedure_call(proc_call) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                    for arg in &proc_call.arguments {
                        if walk_expr(visitor, arg) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlStatement::SqlStatement { statement, .. } => {
                    walk_statement(visitor, statement)
                }
                crate::ast::plpgsql::PlStatement::PipeRow { expression } => {
                    walk_expr(visitor, expression)
                }
                _ => VisitorResult::Continue,
            }
        }
        VisitorResult::SkipChildren => VisitorResult::Continue,
        VisitorResult::Stop => VisitorResult::Stop,
    }
}

/// Walk a PL/pgSQL declaration.
pub fn walk_pl_declaration(visitor: &mut dyn Visitor, decl: &crate::ast::plpgsql::PlDeclaration) -> VisitorResult {
    let result = visitor.visit_pl_declaration(decl);
    match result {
        VisitorResult::Continue => {
            match decl {
                crate::ast::plpgsql::PlDeclaration::Variable(var_decl) => {
                    if let Some(ref default) = var_decl.default {
                        if walk_expr(visitor, default) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlDeclaration::Cursor(cursor_decl) => {
                    if let Some(ref parsed_query) = cursor_decl.parsed_query {
                        if walk_statement(visitor, parsed_query) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                    VisitorResult::Continue
                }
                crate::ast::plpgsql::PlDeclaration::Type(crate::ast::plpgsql::PlTypeDecl::VarrayOf { size, .. }) => {
                    walk_expr(visitor, size)
                }
                crate::ast::plpgsql::PlDeclaration::NestedProcedure(proc) => {
                    if let Some(ref block) = proc.block {
                        walk_pl_block(visitor, block)
                    } else {
                        VisitorResult::Continue
                    }
                }
                crate::ast::plpgsql::PlDeclaration::NestedFunction(func) => {
                    if let Some(ref block) = func.block {
                        walk_pl_block(visitor, block)
                    } else {
                        VisitorResult::Continue
                    }
                }
                _ => VisitorResult::Continue,
            }
        }
        VisitorResult::SkipChildren => VisitorResult::Continue,
        VisitorResult::Stop => VisitorResult::Stop,
    }
}

/// Walk the AST using a visitor.
pub fn walk_statement(visitor: &mut dyn Visitor, stmt: &Statement) -> VisitorResult {
    let result = visitor.visit_statement(stmt);
    match result {
        VisitorResult::Continue => match stmt {
            Statement::Select(s) => walk_select(visitor, s),
            Statement::Insert(s) => walk_insert(visitor, s),
            Statement::Update(s) => walk_update(visitor, s),
            Statement::Delete(s) => walk_delete(visitor, s),
            Statement::CreateTable(s) => walk_create_table(visitor, s),
            Statement::CreateFunction(s) => {
                if let Some(ref block) = s.block {
                    if walk_pl_block(visitor, block) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                }
                VisitorResult::Continue
            }
            Statement::CreateProcedure(s) => {
                if let Some(ref block) = s.block {
                    if walk_pl_block(visitor, block) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                }
                VisitorResult::Continue
            }
            Statement::Do(s) => {
                if let Some(ref block) = s.block {
                    if walk_pl_block(visitor, block) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                }
                VisitorResult::Continue
            }
            Statement::AnonyBlock(s) => {
                if walk_pl_block(visitor, &s.block) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
                VisitorResult::Continue
            }
            Statement::CreatePackage(s) => {
                for item in &s.items {
                    match item {
                        crate::ast::PackageItem::Procedure(p) => {
                            if let Some(ref block) = p.block {
                                if walk_pl_block(visitor, block) == VisitorResult::Stop {
                                    return VisitorResult::Stop;
                                }
                            }
                        }
                        crate::ast::PackageItem::Function(f) => {
                            if let Some(ref block) = f.block {
                                if walk_pl_block(visitor, block) == VisitorResult::Stop {
                                    return VisitorResult::Stop;
                                }
                            }
                        }
                        crate::ast::PackageItem::Raw(_) => {}
                    }
                }
                VisitorResult::Continue
            }
            Statement::CreatePackageBody(s) => {
                for item in &s.items {
                    match item {
                        crate::ast::PackageItem::Procedure(p) => {
                            if let Some(ref block) = p.block {
                                if walk_pl_block(visitor, block) == VisitorResult::Stop {
                                    return VisitorResult::Stop;
                                }
                            }
                        }
                        crate::ast::PackageItem::Function(f) => {
                            if let Some(ref block) = f.block {
                                if walk_pl_block(visitor, block) == VisitorResult::Stop {
                                    return VisitorResult::Stop;
                                }
                            }
                        }
                        crate::ast::PackageItem::Raw(_) => {}
                    }
                }
                VisitorResult::Continue
            }
            Statement::Call(s) => {
                if visitor.visit_call(s) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
                VisitorResult::Continue
            }
            _ => VisitorResult::Continue,
        },
        VisitorResult::SkipChildren => VisitorResult::Continue,
        VisitorResult::Stop => VisitorResult::Stop,
    }
}

fn walk_select(visitor: &mut dyn Visitor, select: &SelectStatement) -> VisitorResult {
    let result = visitor.visit_select(select);
    if result == VisitorResult::Stop {
        return VisitorResult::Stop;
    }
    if result == VisitorResult::SkipChildren {
        return VisitorResult::Continue;
    }

    for target in &select.targets {
        match target {
            SelectTarget::Expr(expr, _) => {
                if walk_expr(visitor, expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            SelectTarget::Star(_) => {}
        }
    }

    if let Some(ref where_clause) = select.where_clause {
        if walk_expr(visitor, where_clause) == VisitorResult::Stop {
            return VisitorResult::Stop;
        }
    }

    for table_ref in &select.from {
        if walk_table_ref(visitor, table_ref) == VisitorResult::Stop {
            return VisitorResult::Stop;
        }
    }

    // with (CTEs)
    if let Some(ref with_clause) = select.with {
        for cte in &with_clause.ctes {
            if walk_select(visitor, &cte.query) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
    }

    // distinct_on
    for expr in &select.distinct_on {
        if walk_expr(visitor, expr) == VisitorResult::Stop {
            return VisitorResult::Stop;
        }
    }

    // connect_by
    if let Some(ref connect_by) = select.connect_by {
        if walk_expr(visitor, &connect_by.condition) == VisitorResult::Stop {
            return VisitorResult::Stop;
        }
        if let Some(ref start_with) = connect_by.start_with {
            if walk_expr(visitor, start_with) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
    }

    // group_by
    for item in &select.group_by {
        match item {
            crate::ast::GroupByItem::Expr(expr) => {
                if walk_expr(visitor, expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            crate::ast::GroupByItem::GroupingSets(sets) => {
                for set in sets {
                    for expr in set {
                        if walk_expr(visitor, expr) == VisitorResult::Stop {
                            return VisitorResult::Stop;
                        }
                    }
                }
            }
            crate::ast::GroupByItem::Rollup(exprs) | crate::ast::GroupByItem::Cube(exprs) => {
                for expr in exprs {
                    if walk_expr(visitor, expr) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                }
            }
        }
    }

    // having
    if let Some(ref having) = select.having {
        if walk_expr(visitor, having) == VisitorResult::Stop {
            return VisitorResult::Stop;
        }
    }

    // order_by
    for item in &select.order_by {
        if walk_expr(visitor, &item.expr) == VisitorResult::Stop {
            return VisitorResult::Stop;
        }
    }

    // limit
    if let Some(ref limit) = select.limit {
        if walk_expr(visitor, limit) == VisitorResult::Stop {
            return VisitorResult::Stop;
        }
    }

    // offset
    if let Some(ref offset) = select.offset {
        if walk_expr(visitor, offset) == VisitorResult::Stop {
            return VisitorResult::Stop;
        }
    }

    // fetch
    if let Some(ref fetch) = select.fetch {
        if let Some(ref count) = fetch.count {
            if walk_expr(visitor, count) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
    }

    // set_operation
    if let Some(ref set_op) = select.set_operation {
        match set_op {
            crate::ast::SetOperation::Union { right, .. } |
            crate::ast::SetOperation::Intersect { right, .. } |
            crate::ast::SetOperation::Except { right, .. } => {
                if walk_select(visitor, right) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
    }

    // window_clause
    for named_window in &select.window_clause {
        for expr in &named_window.spec.partition_by {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        for item in &named_window.spec.order_by {
            if walk_expr(visitor, &item.expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
    }

    // into_targets
    if let Some(ref targets) = select.into_targets {
        for target in targets {
            match target {
                crate::ast::SelectTarget::Expr(expr, _) => {
                    if walk_expr(visitor, expr) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                }
                crate::ast::SelectTarget::Star(_) => {}
            }
        }
    }

    VisitorResult::Continue
}

fn walk_insert(visitor: &mut dyn Visitor, insert: &InsertStatement) -> VisitorResult {
    let result = visitor.visit_insert(insert);
    if result != VisitorResult::Continue {
        return if result == VisitorResult::Stop {
            VisitorResult::Stop
        } else {
            VisitorResult::Continue
        };
    }

    match &insert.source {
        InsertSource::Values(values) => {
            for row in values {
                for expr in row {
                    if walk_expr(visitor, expr) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                }
            }
        }
        InsertSource::Select(select) => {
            if walk_select(visitor, select) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        InsertSource::DefaultValues => {}
        InsertSource::Set(assignments) => {
            for a in assignments {
                if walk_expr(visitor, &a.value) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
    }

    VisitorResult::Continue
}

fn walk_update(visitor: &mut dyn Visitor, update: &UpdateStatement) -> VisitorResult {
    let result = visitor.visit_update(update);
    if result != VisitorResult::Continue {
        return if result == VisitorResult::Stop {
            VisitorResult::Stop
        } else {
            VisitorResult::Continue
        };
    }

    for assignment in &update.assignments {
        if walk_expr(visitor, &assignment.value) == VisitorResult::Stop {
            return VisitorResult::Stop;
        }
    }

    if let Some(ref where_clause) = update.where_clause {
        if walk_expr(visitor, where_clause) == VisitorResult::Stop {
            return VisitorResult::Stop;
        }
    }

    VisitorResult::Continue
}

fn walk_delete(visitor: &mut dyn Visitor, delete: &DeleteStatement) -> VisitorResult {
    let result = visitor.visit_delete(delete);
    if result != VisitorResult::Continue {
        return if result == VisitorResult::Stop {
            VisitorResult::Stop
        } else {
            VisitorResult::Continue
        };
    }

    if let Some(ref where_clause) = delete.where_clause {
        if walk_expr(visitor, where_clause) == VisitorResult::Stop {
            return VisitorResult::Stop;
        }
    }

    VisitorResult::Continue
}

fn walk_create_table(visitor: &mut dyn Visitor, table: &CreateTableStatement) -> VisitorResult {
    let result = visitor.visit_create_table(table);
    if result != VisitorResult::Continue {
        return if result == VisitorResult::Stop {
            VisitorResult::Stop
        } else {
            VisitorResult::Continue
        };
    }

    VisitorResult::Continue
}

fn walk_expr(visitor: &mut dyn Visitor, expr: &Expr) -> VisitorResult {
    let result = visitor.visit_expr(expr);
    if result != VisitorResult::Continue {
        return if result == VisitorResult::Stop {
            VisitorResult::Stop
        } else {
            VisitorResult::Continue
        };
    }

    match expr {
        Expr::BinaryOp { left, right, .. } => {
            if walk_expr(visitor, left) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if walk_expr(visitor, right) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::UnaryOp { expr, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::FunctionCall { args, over, filter, within_group, separator, default, conversion_format, .. } => {
            for arg in args {
                if walk_expr(visitor, arg) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            if let Some(over) = over {
                for expr in &over.partition_by {
                    if walk_expr(visitor, expr) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                }
                for item in &over.order_by {
                    if walk_expr(visitor, &item.expr) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                }
            }
            if let Some(filter) = filter {
                if walk_expr(visitor, filter) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            for item in within_group {
                if walk_expr(visitor, &item.expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            if let Some(separator) = separator {
                if walk_expr(visitor, separator) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            if let Some(default) = default {
                if walk_expr(visitor, default) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            if let Some(conversion_format) = conversion_format {
                if walk_expr(visitor, conversion_format) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::Subquery(select) => {
            if walk_select(visitor, select) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::Case {
            operand,
            whens,
            else_expr,
        } => {
            if let Some(op) = operand {
                if walk_expr(visitor, op) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            for when in whens {
                if walk_expr(visitor, &when.condition) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
                if walk_expr(visitor, &when.result) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            if let Some(else_expr) = else_expr {
                if walk_expr(visitor, else_expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::XmlElement {
            evalname,
            attributes,
            content,
            ..
        } => {
            if let Some(expr) = evalname {
                if walk_expr(visitor, expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            if let Some(attrs) = attributes {
                for item in &attrs.items {
                    if walk_expr(visitor, &item.value) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                }
            }
            for item in content {
                if walk_expr(visitor, &item.expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::XmlConcat(exprs) => {
            for expr in exprs {
                if walk_expr(visitor, expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::XmlForest(items) => {
            for item in items {
                if walk_expr(visitor, &item.expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::XmlParse { expr, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::XmlPi { content, .. } => {
            if let Some(c) = content {
                if walk_expr(visitor, c) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::XmlRoot { expr, version, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if let Some(v) = version {
                if walk_expr(visitor, v) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::XmlSerialize { expr, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::PredictBy { features, .. } => {
            for f in features {
                if walk_expr(visitor, f) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::Like { expr, pattern, escape, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if walk_expr(visitor, pattern) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if let Some(escape) = escape {
                if walk_expr(visitor, escape) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::Between { expr, low, high, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if walk_expr(visitor, low) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if walk_expr(visitor, high) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::InList { expr, list, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            for item in list {
                if walk_expr(visitor, item) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::InSubquery { expr, subquery, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if walk_select(visitor, subquery) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::Exists(subquery) => {
            if walk_select(visitor, subquery) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::ScalarSublink { expr, subquery, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if walk_select(visitor, subquery) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::IsNull { expr, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::TypeCast { expr, default, format, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if let Some(default) = default {
                if walk_expr(visitor, default) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            if let Some(format) = format {
                if walk_expr(visitor, format) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::Treat { expr, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::Array(elems) => {
            for elem in elems {
                if walk_expr(visitor, elem) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::Subscript { object, index } => {
            if walk_expr(visitor, object) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if walk_expr(visitor, index) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::FieldAccess { object, .. } => {
            if walk_expr(visitor, object) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::Parenthesized(expr) => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::RowConstructor(exprs) => {
            for expr in exprs {
                if walk_expr(visitor, expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::CollationFor { expr } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::Prior(expr) => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::SpecialFunction { args, .. } => {
            for arg in args {
                if walk_expr(visitor, arg) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::PlVariable(_) => {}
        _ => {}
    }

    VisitorResult::Continue
}

fn walk_table_ref(visitor: &mut dyn Visitor, table_ref: &TableRef) -> VisitorResult {
    let result = visitor.visit_table_ref(table_ref);
    if result == VisitorResult::Stop {
        return VisitorResult::Stop;
    }
    if result == VisitorResult::SkipChildren {
        return VisitorResult::Continue;
    }

    match table_ref {
        TableRef::Table { timecapsule, .. } => {
            if let Some(ref timecapsule) = timecapsule {
                if walk_expr(visitor, timecapsule) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            VisitorResult::Continue
        }
        TableRef::FunctionCall { args, .. } => {
            for arg in args {
                if walk_expr(visitor, arg) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            VisitorResult::Continue
        }
        TableRef::Subquery { query, .. } => walk_select(visitor, query),
        TableRef::Values { values, .. } => {
            for row in &values.rows {
                for expr in row {
                    if walk_expr(visitor, expr) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                }
            }
            for item in &values.order_by {
                if walk_expr(visitor, &item.expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            if let Some(ref limit) = values.limit {
                if walk_expr(visitor, limit) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            if let Some(ref offset) = values.offset {
                if walk_expr(visitor, offset) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            VisitorResult::Continue
        }
        TableRef::Join { left, right, condition, .. } => {
            if walk_table_ref(visitor, left) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if walk_table_ref(visitor, right) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if let Some(condition) = condition {
                if walk_expr(visitor, condition) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            VisitorResult::Continue
        }
        TableRef::Pivot { source, pivot, .. } => {
            if walk_table_ref(visitor, source) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if walk_expr(visitor, &pivot.aggregate) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            for pv in &pivot.values {
                if walk_expr(visitor, &pv.value) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            VisitorResult::Continue
        }
        TableRef::Unpivot { source, .. } => {
            if walk_table_ref(visitor, source) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            VisitorResult::Continue
        }
    }
}

#[cfg(test)]
mod visitor_tests {
    use super::*;
    use crate::ast::plpgsql::{
        PlBlock, PlDeclaration, PlExceptionBlock, PlExceptionHandler, PlForKind, PlForStmt,
        PlIfStmt, PlLoopStmt, PlOpenKind, PlOpenStmt, PlProcedureCall, PlStatement, PlWhileStmt,
    };
    use crate::ast::{
        CallFuncStatement, CreateFunctionStatement, CreatePackageBodyStatement,
        CreatePackageStatement, CreateProcedureStatement, DoStatement, Expr, ObjectName,
        SelectStatement, Statement, TableRef,
    };
    use crate::parser::Parser;
    use crate::token::tokenizer::Tokenizer;

    fn parse(sql: &str) -> Vec<Statement> {
        let tokens = Tokenizer::new(sql).tokenize().unwrap();
        Parser::new(tokens).parse()
    }

    fn parse_single(sql: &str) -> Statement {
        let mut stmts = parse(sql);
        assert_eq!(stmts.len(), 1, "Expected exactly one statement");
        stmts.remove(0)
    }

    fn parse_expr(sql: &str) -> Expr {
        let stmt = parse_single(&format!("SELECT {}", sql));
        match stmt {
            Statement::Select(ref select) => {
                match &select.targets[0] {
                    crate::ast::SelectTarget::Expr(expr, _) => expr.clone(),
                    _ => panic!("Expected expression target"),
                }
            }
            _ => panic!("Expected SELECT statement"),
        }
    }

    #[derive(Debug, Default)]
    struct TestVisitor {
        statements: Vec<String>,
        exprs: Vec<String>,
        pl_blocks: usize,
        pl_statements: usize,
        pl_declarations: usize,
        pl_exception_handlers: usize,
        calls: Vec<ObjectName>,
        procedure_calls: Vec<ObjectName>,
        selects: usize,
        table_refs: Vec<String>,
    }

    impl Visitor for TestVisitor {
        fn visit_statement(&mut self, stmt: &Statement) -> VisitorResult {
            self.statements.push(format!("{:?}", std::mem::discriminant(stmt)));
            VisitorResult::Continue
        }

        fn visit_expr(&mut self, _expr: &Expr) -> VisitorResult {
            self.exprs.push("expr".to_string());
            VisitorResult::Continue
        }

        fn visit_pl_block(&mut self, _block: &PlBlock) -> VisitorResult {
            self.pl_blocks += 1;
            VisitorResult::Continue
        }

        fn visit_pl_statement(&mut self, _stmt: &PlStatement) -> VisitorResult {
            self.pl_statements += 1;
            VisitorResult::Continue
        }

        fn visit_pl_declaration(&mut self, _decl: &PlDeclaration) -> VisitorResult {
            self.pl_declarations += 1;
            VisitorResult::Continue
        }

        fn visit_pl_exception_handler(&mut self, _handler: &PlExceptionHandler) -> VisitorResult {
            self.pl_exception_handlers += 1;
            VisitorResult::Continue
        }

        fn visit_call(&mut self, call: &CallFuncStatement) -> VisitorResult {
            self.calls.push(call.func_name.clone());
            VisitorResult::Continue
        }

        fn visit_procedure_call(&mut self, call: &PlProcedureCall) -> VisitorResult {
            self.procedure_calls.push(call.name.clone());
            VisitorResult::Continue
        }

        fn visit_select(&mut self, _select: &SelectStatement) -> VisitorResult {
            self.selects += 1;
            VisitorResult::Continue
        }

        fn visit_table_ref(&mut self, table_ref: &TableRef) -> VisitorResult {
            let name = match table_ref {
                TableRef::Table { name, .. } => format!("Table({})", name.join(".")),
                TableRef::FunctionCall { name, .. } => format!("FunctionCall({})", name.join(".")),
                TableRef::Subquery { .. } => "Subquery".to_string(),
                TableRef::Values { .. } => "Values".to_string(),
                TableRef::Join { .. } => "Join".to_string(),
                TableRef::Pivot { .. } => "Pivot".to_string(),
                TableRef::Unpivot { .. } => "Unpivot".to_string(),
            };
            self.table_refs.push(name);
            VisitorResult::Continue
        }
    }

    #[test]
    fn test_pl_block_visitor_methods_exist() {
        let mut visitor = TestVisitor::default();
        let block = PlBlock {
            label: None,
            declarations: vec![],
            body: vec![],
            exception_block: None,
            end_label: None,
        };
        assert_eq!(visitor.visit_pl_block(&block), VisitorResult::Continue);
        assert_eq!(visitor.visit_pl_statement(&PlStatement::Null), VisitorResult::Continue);
        assert_eq!(visitor.visit_pl_declaration(&PlDeclaration::Record(crate::ast::plpgsql::PlRecordDecl { name: "x".to_string() })), VisitorResult::Continue);
        assert_eq!(visitor.visit_pl_exception_handler(&PlExceptionHandler { conditions: vec![], statements: vec![] }), VisitorResult::Continue);
    }

    #[test]
    fn test_walk_pl_block_basic() {
        let block = PlBlock {
            label: None,
            declarations: vec![
                PlDeclaration::Variable(crate::ast::plpgsql::PlVarDecl {
                    name: "x".to_string(),
                    data_type: crate::ast::plpgsql::PlDataType::TypeName("INTEGER".to_string()),
                    default: Some(Expr::Literal(crate::ast::Literal::Integer(42))),
                    constant: false,
                    not_null: false,
                    collate: None,
                }),
            ],
            body: vec![
                PlStatement::Assignment {
                    target: Expr::ColumnRef(crate::ast::ObjectName::from(vec!["x".to_string()])),
                    expression: Expr::Literal(crate::ast::Literal::Integer(1)),
                },
            ],
            exception_block: Some(PlExceptionBlock {
                handlers: vec![
                    PlExceptionHandler {
                        conditions: vec!["OTHERS".to_string()],
                        statements: vec![PlStatement::Null],
                    },
                ],
            }),
            end_label: None,
        };

        let mut visitor = TestVisitor::default();
        walk_pl_block(&mut visitor, &block);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_declarations, 1);
        assert_eq!(visitor.pl_statements, 2);
        assert_eq!(visitor.pl_exception_handlers, 1);
        assert_eq!(visitor.exprs.len(), 3);
    }

    #[test]
    fn test_walk_pl_statement_if() {
        let if_stmt = PlStatement::If(crate::ast::Spanned::new(PlIfStmt {
            condition: Expr::Literal(crate::ast::Literal::Boolean(true)),
            then_stmts: vec![PlStatement::Null],
            elsifs: vec![crate::ast::plpgsql::PlElsif {
                condition: Expr::Literal(crate::ast::Literal::Boolean(false)),
                stmts: vec![PlStatement::Null],
            }],
            else_stmts: vec![PlStatement::Null],
        }, None));

        let mut visitor = TestVisitor::default();
        walk_pl_statement(&mut visitor, &if_stmt);

        assert_eq!(visitor.pl_statements, 4);
        assert_eq!(visitor.exprs.len(), 2);
    }

    #[test]
    fn test_walk_pl_statement_loop() {
        let loop_stmt = PlStatement::Loop(crate::ast::Spanned::new(PlLoopStmt {
            label: None,
            body: vec![PlStatement::Null, PlStatement::Null],
            end_label: None,
        }, None));

        let mut visitor = TestVisitor::default();
        walk_pl_statement(&mut visitor, &loop_stmt);

        assert_eq!(visitor.pl_statements, 3);
    }

    #[test]
    fn test_walk_pl_statement_while() {
        let while_stmt = PlStatement::While(crate::ast::Spanned::new(PlWhileStmt {
            label: None,
            condition: Expr::Literal(crate::ast::Literal::Boolean(true)),
            body: vec![PlStatement::Null],
            end_label: None,
        }, None));

        let mut visitor = TestVisitor::default();
        walk_pl_statement(&mut visitor, &while_stmt);

        assert_eq!(visitor.pl_statements, 2);
        assert_eq!(visitor.exprs.len(), 1);
    }

    #[test]
    fn test_walk_pl_statement_for_range() {
        let for_stmt = PlStatement::For(crate::ast::Spanned::new(PlForStmt {
            label: None,
            variable: "i".to_string(),
            kind: PlForKind::Range {
                low: Expr::Literal(crate::ast::Literal::Integer(1)),
                high: Expr::Literal(crate::ast::Literal::Integer(10)),
                step: None,
                reverse: false,
            },
            body: vec![PlStatement::Null],
            end_label: None,
        }, None));

        let mut visitor = TestVisitor::default();
        walk_pl_statement(&mut visitor, &for_stmt);

        assert_eq!(visitor.pl_statements, 2);
        assert_eq!(visitor.exprs.len(), 2);
    }

    #[test]
    fn test_walk_pl_statement_procedure_call() {
        let proc_call = PlStatement::ProcedureCall(crate::ast::Spanned::new(PlProcedureCall {
            name: vec!["schema".to_string(), "proc".to_string()],
            arguments: vec![
                Expr::Literal(crate::ast::Literal::Integer(1)),
                Expr::Literal(crate::ast::Literal::Integer(2)),
            ],
        }, None));

        let mut visitor = TestVisitor::default();
        walk_pl_statement(&mut visitor, &proc_call);

        assert_eq!(visitor.pl_statements, 1);
        assert_eq!(visitor.procedure_calls.len(), 1);
        assert_eq!(visitor.procedure_calls[0], vec!["schema".to_string(), "proc".to_string()]);
        assert_eq!(visitor.exprs.len(), 2);
    }

    #[test]
    fn test_walk_pl_statement_exception() {
        let block = PlBlock {
            label: None,
            declarations: vec![],
            body: vec![PlStatement::Null],
            exception_block: Some(PlExceptionBlock {
                handlers: vec![
                    PlExceptionHandler {
                        conditions: vec!["NO_DATA_FOUND".to_string()],
                        statements: vec![PlStatement::Null],
                    },
                    PlExceptionHandler {
                        conditions: vec!["OTHERS".to_string()],
                        statements: vec![PlStatement::Null],
                    },
                ],
            }),
            end_label: None,
        };

        let mut visitor = TestVisitor::default();
        walk_pl_block(&mut visitor, &block);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_exception_handlers, 2);
        assert_eq!(visitor.pl_statements, 3);
    }

    #[test]
    fn test_walk_statement_create_function() {
        let sql = "CREATE FUNCTION foo() RETURNS INTEGER AS $$ BEGIN RETURN 1; END; $$ LANGUAGE plpgsql";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_statements, 1);
    }

    #[test]
    fn test_walk_statement_create_procedure() {
        let sql = "CREATE PROCEDURE bar() AS $$ BEGIN NULL; END; $$";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_statements, 1);
    }

    #[test]
    fn test_walk_statement_do() {
        let sql = "DO $$ BEGIN PERFORM 1; END $$";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_statements, 1);
    }

    #[test]
    fn test_walk_statement_anony_block() {
        let sql = "BEGIN PERFORM 1; END";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_statements, 1);
    }

    #[test]
    fn test_walk_statement_call() {
        let sql = "CALL schema.proc_name(1, 2)";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.calls.len(), 1);
        assert_eq!(visitor.calls[0], vec!["schema".to_string(), "proc_name".to_string()]);
    }

    #[test]
    fn test_walk_expr_like() {
        let expr = parse_expr("name LIKE '%test%'");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 2);
    }

    #[test]
    fn test_walk_expr_between() {
        let expr = parse_expr("x BETWEEN 1 AND 10");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 3);
    }

    #[test]
    fn test_walk_expr_in_list() {
        let expr = parse_expr("x IN (1, 2, 3)");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 4);
    }

    #[test]
    fn test_walk_expr_exists() {
        let expr = parse_expr("EXISTS (SELECT 1 FROM t)");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert_eq!(visitor.selects, 1);
    }

    #[test]
    fn test_walk_expr_typecast() {
        let expr = parse_expr("x::INTEGER");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 1);
    }

    #[test]
    fn test_walk_expr_array() {
        let expr = parse_expr("ARRAY[1, 2, 3]");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 3);
    }

    #[test]
    fn test_walk_expr_subscript() {
        let expr = parse_expr("arr[1]");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 2);
    }

    #[test]
    fn test_walk_expr_field_access() {
        let expr = parse_expr("rec.field");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 1);
    }

    #[test]
    fn test_walk_expr_parenthesized() {
        let expr = parse_expr("(1 + 2)");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 1);
    }

    #[test]
    fn test_walk_expr_row_constructor() {
        let expr = parse_expr("ROW(1, 2, 3)");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 3);
    }

    #[test]
    fn test_walk_select_with_cte() {
        let sql = "WITH cte AS (SELECT id FROM users) SELECT * FROM cte";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.selects, 2);
    }

    #[test]
    fn test_walk_select_group_by() {
        let sql = "SELECT dept, COUNT(*) FROM employees GROUP BY dept HAVING COUNT(*) > 5";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert!(visitor.exprs.len() >= 3);
    }

    #[test]
    fn test_walk_select_order_by() {
        let sql = "SELECT * FROM t ORDER BY x, y DESC";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert!(visitor.exprs.len() >= 2);
    }

    #[test]
    fn test_walk_select_limit_offset() {
        let sql = "SELECT * FROM t LIMIT 10 OFFSET 5";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert!(visitor.exprs.len() >= 2);
    }

    #[test]
    fn test_walk_select_union() {
        let sql = "SELECT 1 UNION ALL SELECT 2";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.selects, 2);
    }

    #[test]
    fn test_walk_table_ref_join() {
        let sql = "SELECT * FROM a JOIN b ON a.id = b.id";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert!(visitor.exprs.len() >= 2);
    }

    #[test]
    fn test_walk_table_ref_function_call() {
        let sql = "SELECT * FROM generate_series(1, 10)";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert!(visitor.exprs.len() >= 2);
    }

    #[derive(Debug, Default)]
    struct SkipChildrenVisitor {
        expr_count: usize,
    }

    impl Visitor for SkipChildrenVisitor {
        fn visit_expr(&mut self, _expr: &Expr) -> VisitorResult {
            self.expr_count += 1;
            VisitorResult::SkipChildren
        }
    }

    #[test]
    fn test_skip_children_expr() {
        let expr = parse_expr("1 + 2");
        let mut visitor = SkipChildrenVisitor::default();
        walk_expr(&mut visitor, &expr);

        assert_eq!(visitor.expr_count, 1);
    }

    #[derive(Debug, Default)]
    struct StopVisitor {
        expr_count: usize,
    }

    impl Visitor for StopVisitor {
        fn visit_expr(&mut self, _expr: &Expr) -> VisitorResult {
            self.expr_count += 1;
            if self.expr_count >= 2 {
                VisitorResult::Stop
            } else {
                VisitorResult::Continue
            }
        }
    }

    #[test]
    fn test_stop_expr() {
        let expr = parse_expr("1 + 2 + 3");
        let mut visitor = StopVisitor::default();
        let result = walk_expr(&mut visitor, &expr);

        assert_eq!(result, VisitorResult::Stop);
        assert_eq!(visitor.expr_count, 2);
    }

    #[test]
    fn test_complex_pl_block() {
        let sql = r#"
            CREATE FUNCTION complex_func(p_id INT) RETURNS INT AS $$
            DECLARE
                v_count INT := 0;
            BEGIN
                IF p_id > 0 THEN
                    PERFORM SELECT 1 FROM t;
                    v_count := v_count + 1;
                ELSE
                    RAISE NOTICE 'Invalid id';
                END IF;
                
                FOR i IN 1..10 LOOP
                    PERFORM SELECT 2 FROM t;
                END LOOP;
                
                RETURN v_count;
            EXCEPTION
                WHEN OTHERS THEN
                    PERFORM SELECT 3 FROM t;
                    RETURN -1;
            END;
            $$ LANGUAGE plpgsql
        "#;

        let stmt = parse_single(sql);
        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_declarations, 1);
        assert!(visitor.pl_statements >= 5);
        assert_eq!(visitor.pl_exception_handlers, 1);
    }

    #[test]
    fn test_nested_blocks() {
        let sql = r#"
            DO $$
            BEGIN
                BEGIN
                    PERFORM SELECT 1 FROM t;
                END;
                PERFORM SELECT 2 FROM t;
            END;
            $$
        "#;

        let stmt = parse_single(sql);
        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 2);
        assert!(visitor.pl_statements >= 2);
    }

    #[test]
    fn test_create_package_body() {
        let sql = r#"
            CREATE PACKAGE BODY pkg_api AS
                PROCEDURE inner_proc IS
                BEGIN
                    PERFORM SELECT 1 FROM t;
                END;
                
                FUNCTION get_val RETURN NUMBER IS
                BEGIN
                    RETURN 42;
                END;
            END pkg_api
        "#;

        let stmt = parse_single(sql);
        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 2);
    }

    #[test]
    fn test_visit_table_ref_function_call_in_from() {
        let sql = "SELECT * FROM generate_series(1, 10)";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.table_refs.len(), 1);
        assert!(visitor.table_refs[0].starts_with("FunctionCall("));
    }

    #[test]
    fn test_visit_table_ref_regular_table() {
        let sql = "SELECT * FROM users";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.table_refs.len(), 1);
        assert!(visitor.table_refs[0].starts_with("Table("));
    }

    #[test]
    fn test_visit_table_ref_join() {
        let sql = "SELECT * FROM a JOIN b ON a.id = b.id";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert!(visitor.table_refs.len() >= 2, "Join should recurse into left and right table refs, got: {:?}", visitor.table_refs);
    }

    #[test]
    fn test_visit_table_ref_in_perform() {
        let sql = "DO $$ BEGIN PERFORM SELECT 1 FROM pkg_audit.log_transfer(1, 2); END $$";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        let func_refs: Vec<&String> = visitor.table_refs.iter()
            .filter(|r| r.starts_with("FunctionCall("))
            .collect();
        assert_eq!(func_refs.len(), 1, "Expected one FunctionCall table ref from PERFORM, got: {:?}", visitor.table_refs);
        assert!(func_refs[0].contains("pkg_audit.log_transfer"));
    }

    #[test]
    fn test_visit_table_ref_skip_children() {
        #[derive(Debug, Default)]
        struct SkipTableRefVisitor {
            table_refs: usize,
            exprs: usize,
        }

        impl Visitor for SkipTableRefVisitor {
            fn visit_table_ref(&mut self, _table_ref: &TableRef) -> VisitorResult {
                self.table_refs += 1;
                VisitorResult::SkipChildren
            }

            fn visit_expr(&mut self, _expr: &Expr) -> VisitorResult {
                self.exprs += 1;
                VisitorResult::Continue
            }
        }

        let sql = "SELECT * FROM generate_series(1, 10)";
        let stmt = parse_single(sql);

        let mut visitor = SkipTableRefVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.table_refs, 1, "visit_table_ref should fire");
        assert_eq!(visitor.exprs, 0, "SkipChildren should skip args of FunctionCall");
    }

    #[test]
    fn test_visit_table_ref_stop() {
        #[derive(Debug, Default)]
        struct StopTableRefVisitor {
            table_refs: usize,
        }

        impl Visitor for StopTableRefVisitor {
            fn visit_table_ref(&mut self, _table_ref: &TableRef) -> VisitorResult {
                self.table_refs += 1;
                VisitorResult::Stop
            }
        }

        let sql = "SELECT * FROM generate_series(1, 10) JOIN users ON true";
        let stmt = parse_single(sql);

        let mut visitor = StopTableRefVisitor::default();
        let result = walk_statement(&mut visitor, &stmt);

        assert_eq!(result, VisitorResult::Stop);
        assert!(visitor.table_refs >= 1, "Should have stopped at the first table ref");
    }

    #[test]
    fn test_perform_bare_func_call_has_parsed_expr() {
        let sql = "DO $$ BEGIN PERFORM pkg_audit.log_transfer(1, 2); END $$";
        let stmt = parse_single(sql);

        let block = match &stmt {
            Statement::Do(s) => match &s.node.block {
                Some(b) => b,
                _ => panic!("Expected DO statement with block"),
            },
            _ => panic!("Expected DO statement with block"),
        };
        let (parsed_query, parsed_expr) = match &block.body[0] {
            PlStatement::Perform { parsed_query, parsed_expr, .. } => (parsed_query, parsed_expr),
            _ => panic!("Expected Perform statement"),
        };
        assert!(parsed_expr.is_some(), "PERFORM func(args) should have parsed_expr, got None");
        assert!(parsed_query.is_none(), "PERFORM func(args) should not have parsed_query");
    }

    #[test]
    fn test_visit_expr_in_perform_bare_func() {
        let sql = "DO $$ BEGIN PERFORM pkg_audit.log_transfer(1, 2); END $$";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert!(visitor.exprs.len() >= 2, "PERFORM func(args) should visit the function args as expressions, got {} exprs", visitor.exprs.len());
    }
}
