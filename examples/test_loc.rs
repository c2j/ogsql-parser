fn main() {
    let sql = std::fs::read_to_string(
        "lib/openGauss-server/src/test/regress/sql/plpgsql/plpgsql_normal.sql",
    )
    .unwrap();
    let (infos, errors) = ogsql_parser::parser::Parser::parse_sql(&sql);
    println!("Parsed {} statements, {} errors", infos.len(), errors.len());
    let mut correct = 0;
    for (i, info) in infos.iter().enumerate() {
        let text_preview: String = info.sql_text.chars().take(80).collect();
        let trailing = if info.sql_text.len() > 80 { "..." } else { "" };
        // A statement is correct if sql_text is non-empty, doesn't start with ; or /,
        // and either has no embedded ; or is a legitimate block (contains BEGIN/END/IS/AS)
        let starts_ok = !info.sql_text.is_empty()
            && !info.sql_text.starts_with(';')
            && !info.sql_text.starts_with('/');
        let is_block_end = info.sql_text.starts_with("END ") || info.sql_text.starts_with("BEGIN");
        let has_bad_embed = text_preview.contains(";\n")
            && !text_preview.contains("$BODY$")
            && !text_preview.contains("$$")
            && !is_block_end
            && !text_preview.contains("FUNCTION")
            && !text_preview.contains("PACKAGE");
        let ok = starts_ok && !has_bad_embed;
        if ok {
            correct += 1;
        }
        println!(
            "{:3} {} {:?}{}",
            i,
            if ok { "✓" } else { "✗" },
            text_preview,
            trailing
        );
    }
    println!("\nCorrect: {}/{}", correct, infos.len());
}
