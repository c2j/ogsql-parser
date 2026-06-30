mod common;
use common::load_named;
use ogsql_parser::Parser;

fn assert_parse_all(sql: &str, label: &str) {
    let (infos, errors) = Parser::parse_sql(sql);
    let empty_count = infos.iter().filter(|s| matches!(s.statement, ogsql_parser::Statement::Empty)).count();
    assert!(
        errors.is_empty() && empty_count == 0,
        "回归守护: [{label}] 解析失败\n  errors: {errors:?}\n  empty statements: {empty_count}\n  SQL: {sql}"
    );
}

#[test]
fn slash_division_in_select() {
    let fixtures = load_named("slash_division_select", "parser");
    assert!(!fixtures.is_empty(), "回归守护: fixture 文件缺失");
    for f in &fixtures {
        for (i, stmt) in f.content.split(';').enumerate() {
            let stmt = stmt.trim();
            if stmt.is_empty() {
                continue;
            }
            let sql = format!("{};", stmt);
            assert_parse_all(&sql, &format!("{}/stmt{}", f.id, i));
        }
    }
}

#[test]
fn slash_division_in_ddl() {
    let fixtures = load_named("slash_division_ddl", "parser");
    assert!(!fixtures.is_empty(), "回归守护: fixture 文件缺失");
    for f in &fixtures {
        for (i, stmt) in f.content.split(';').enumerate() {
            let stmt = stmt.trim();
            if stmt.is_empty() {
                continue;
            }
            let sql = format!("{};", stmt);
            assert_parse_all(&sql, &format!("{}/stmt{}", f.id, i));
        }
    }
}

#[test]
fn slash_division_in_complex() {
    let fixtures = load_named("slash_division_complex", "parser");
    assert!(!fixtures.is_empty(), "回归守护: fixture 文件缺失");
    for f in &fixtures {
        for (i, stmt) in f.content.split(';').enumerate() {
            let stmt = stmt.trim();
            if stmt.is_empty() {
                continue;
            }
            let sql = format!("{};", stmt);
            assert_parse_all(&sql, &format!("{}/stmt{}", f.id, i));
        }
    }
}

#[test]
fn package_var_reference_in_body() {
    let fixtures = load_named("package_var_reference", "parser");
    assert!(!fixtures.is_empty(), "回归守护: fixture 文件缺失");
    for f in &fixtures {
        let blocks: Vec<&str> = f.content.split("\n\n").filter(|b| !b.trim().is_empty()).collect();
        for (i, block) in blocks.iter().enumerate() {
            assert_parse_all(block, &format!("{}/block{}", f.id, i));
        }
    }
}

#[test]
fn slash_division_in_plpgsql() {
    let fixtures = load_named("slash_division_plpgsql", "parser");
    assert!(!fixtures.is_empty(), "回归守护: fixture 文件缺失");
    for f in &fixtures {
        // PL/pgSQL blocks use $$ quoting — split by empty lines between blocks
        let blocks: Vec<&str> = f.content.split("\n\n").filter(|b| !b.trim().is_empty()).collect();
        for (i, block) in blocks.iter().enumerate() {
            assert_parse_all(block, &format!("{}/plblock{}", f.id, i));
        }
    }
}
