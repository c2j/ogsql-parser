use ogsql_parser::{Parser, ParserError, Tokenizer};

fn test_parse(sql: &str, name: &str) {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    let errors = parser.errors();
    if errors.is_empty() {
        println!("OK: {} -> {:?}", name, stmts.first());
    } else {
        println!("ERR: {} -> errors: {:?}", name, errors);
    }
}

fn test_parse_with_warnings(sql: &str, name: &str, expect_warnings: &[&str]) {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let _ = parser.parse();
    let errors = parser.errors();
    let warnings: Vec<_> = errors.iter().filter(|e| matches!(e, ParserError::Warning { .. })).collect();
    let mut ok = true;

    for expected in expect_warnings {
        if !warnings.iter().any(|w| w.to_string().contains(expected)) {
            println!(
                "MISS: {} -> expected warning containing '{}', got: {:?}",
                name,
                expected,
                warnings.iter().map(|w| w.to_string()).collect::<Vec<_>>()
            );
            ok = false;
        }
    }

    let extra: Vec<_> =
        warnings.iter().filter(|w| !expect_warnings.iter().any(|e| w.to_string().contains(e))).collect();
    for w in &extra {
        println!("EXTRA: {} -> unexpected warning: {}", name, w);
        ok = false;
    }

    if ok {
        println!("OK: {} ({} warnings matched)", name, warnings.len());
    }
}

fn main() {
    test_parse("SELECT xmlagg(data order by id desc) FROM xmltest", "fix1: ORDER BY inside aggregate");
    test_parse("SELECT xmlexists('//town[text() = ''Toronto'']' PASSING BY REF '<towns>...')", "fix2: PASSING BY REF");
    test_parse("CURSOR xc WITH HOLD FOR SELECT * FROM testxmlschema.test1 ORDER BY 1, 2", "fix3: CURSOR declaration");
    test_parse("CREATE TABLE t2(a int, b int GENERATED ALWAYS AS (a + 1) STORED)", "fix4: GENERATED column");
    test_parse(
        "SELECT current_date + s.a AS dates FROM generate_series(0, 14, 7) AS s(a)",
        "fix5: function composite alias",
    );
    test_parse("SELECT * FROM DBE_HEAT_MAP.ROW_HEAT_MAP(owner => 'heat_map_data', segment_name => 'heat_map_table', partition_name => NULL, ctid => '(0,1)')", "fix6: named arg with NULL");
    test_parse("CLUSTER test_c1 USING idx_test_c1_id", "fix7: CLUSTER USING");
    test_parse("REINDEX TABLE CONCURRENTLY tpcds.customer_t1", "fix8: REINDEX CONCURRENTLY");
    test_parse("VACUUM (VERBOSE, ANALYZE) tpcds.reason", "fix9: VACUUM with options");
    test_parse("CLEAN CONNECTION TO ALL FOR DATABASE template1 TO USER jack", "fix10: CLEAN CONNECTION");
    test_parse("SELECT pro_variadic(var1 => 'hello', VARIADIC var4 => array[1,2,3,4])", "fix11: VARIADIC named param");

    test_parse("SELECT bit_and(c1) FROM t", "func: bit_and(c1) built-in OK");
    test_parse_with_warnings("SELECT bit_and(c1, c2) FROM t", "func: bit_and(c1,c2) built-in warns", &["bit_and"]);

    test_parse("SELECT dbe_raw.bit_and(r1, r2) FROM t", "func: dbe_raw.bit_and(r1,r2) correct arg count");
    test_parse("SELECT dbe_raw.bit_and(r1) FROM t", "func: dbe_raw.bit_and(r1) too few args");

    test_parse("SELECT regexp_substr('s', 'p') FROM t", "func: regexp_substr 2 args");
    test_parse("SELECT regexp_substr('s', 'p', 1, 1) FROM t", "func: regexp_substr 4 args");
    test_parse("SELECT regexp_substr('s', 'p', 1, 1, 'i') FROM t", "func: regexp_substr 5 args");
}
