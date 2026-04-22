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
        Expr::FunctionCall { args, .. } => {
            for arg in args {
                if walk_expr(visitor, arg) == VisitorResult::Stop {
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
        _ => {}
    }

    VisitorResult::Continue
}

fn walk_table_ref(visitor: &mut dyn Visitor, table_ref: &TableRef) -> VisitorResult {
    match table_ref {
        TableRef::Subquery { query, .. } => walk_select(visitor, query),
        _ => VisitorResult::Continue,
    }
}
