use std::fs;
use std::path::Path;

fn main() {
    let regress_dir = "lib/openGauss-server/src/test/regress/sql";
    let args: Vec<String> = std::env::args().collect();

    let files: Vec<String> = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        let mut files: Vec<String> = fs::read_dir(regress_dir)
            .expect("Failed to read regress/sql directory")
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "sql")
                    .unwrap_or(false)
            })
            .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
            .collect();
        files.sort();
        files
    };

    let mut total = 0usize;
    let mut read_ok = 0usize;
    let mut read_err = 0usize;
    let mut tokenized_ok = 0usize;
    let mut tokenized_err = 0usize;
    let mut parsed_ok = 0usize;
    let mut parsed_err = 0usize;

    for file in &files {
        let path = Path::new(regress_dir).join(file);

        // Read as bytes first to handle different encodings
        let bytes = match fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("FAIL_READ  {} : {}", file, e);
                continue;
            }
        };

        // Decode with encoding detection
        let (sql, encoding) = match ogsql_parser::token::decode_sql_file(&bytes) {
            Ok((s, enc)) => {
                read_ok += 1;
                (s, enc)
            }
            Err(e) => {
                read_err += 1;
                eprintln!("FAIL_DECODE {} : {}", file, e);
                continue;
            }
        };

        total += 1;

        // Report if non-UTF-8 encoding was detected
        if encoding != "UTF-8" {
            eprintln!("ENCODING   {} : {}", file, encoding);
        }

        match ogsql_parser::Tokenizer::new(&sql).tokenize() {
            Ok(tokens) => {
                tokenized_ok += 1;
                match ogsql_parser::parser::Parser::new(tokens).parse() {
                    stmts => {
                        if stmts
                            .iter()
                            .any(|s| matches!(s, ogsql_parser::Statement::Empty))
                        {
                            parsed_err += 1;
                        } else {
                            parsed_ok += 1;
                        }
                    }
                }
            }
            Err(e) => {
                tokenized_err += 1;
                eprintln!("TOKEN_ERR  {} : {}", file, e);
            }
        }
    }

    println!("\n=== Regression Test Summary ===");
    println!("Files found:   {}", files.len());
    println!("Files tested:  {}", total);
    println!("Read OK:       {}", read_ok);
    println!("Read ERR:      {}", read_err);
    println!("Tokenized OK:  {}", tokenized_ok);
    println!("Tokenize ERR:  {}", tokenized_err);
    println!("Parsed OK:     {}", parsed_ok);
    println!("Parse ERR:     {}", parsed_err);

    if read_err > 0 || tokenized_err > 0 || parsed_err > 0 || total == 0 {
        std::process::exit(1);
    }
}
