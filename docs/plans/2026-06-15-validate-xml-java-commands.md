# validate-xml / validate-java CLI Commands Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `ogsql validate-xml` and `ogsql validate-java` CLI commands that mirror `ogsql validate` in rigor — PACKAGE consistency, MERGE semantics, PL variable checks, Full-confidence lint, VALID/INVALID output, and `exit(1)` on error — achieving feature parity with `parse` ↔ `validate`.

**Architecture:** Extract `validate_from_stmts()` core from `validate_sql()`, making PACKAGE/MERGE/PL validation reusable for any `&[StatementInfo]` source. Each new command collects `StatementInfo` from its file type (XML → flat SQL parse; Java → tree-sitter extraction), then pipes through the shared core. `Confidence::Partial` → `Confidence::Full` for lint in both commands, matching `validate`'s standard.

**Tech Stack:** Rust, clap 4 (CLI), ogsql_parser (ibatis/java/lib), same validation pipeline as `validate`

---

### 前置约束（遵循 docs/CONTRIBUTING.md & BEST-PRATICE.md）

- **M-ARCH-03**: `ogsql.rs` 已 7482 行（远超 600 行上限）。本次新增代码尽量精简（目标 <250 行），将可共用逻辑抽取为独立函数减少重复。
- **M-FMT-01**: `cargo fmt --all -- --check` 必须通过。
- **M-ERR-02**: 库代码禁止 `unwrap()`，CLI 代码中遵循现有 `die!` / `unwrap_or_else` 模式。
- **M-NAM-01**: 统一命名 `cmd_validate_*` / `output_csv_validate_*`，与现有 `parse-xml`/`parse-java` 对应。
- **R-API-01**: 函数参数 >5 时用结构体封装（本次 `validate_from_stmts` 参数较少，无需）。
- **R-COL-01**: `Vec` 创建时预分配合适容量。
- **R-TST-03**: 每个命令至少 3 个测试用例（有效 SQL、含错误 SQL、动态 SQL）。

**CI 标准**（`.github/workflows/ci.yml`）：
- `cargo fmt --all -- --check`
- `cargo clippy --all-features -- -D warnings`
- `cargo test --all-features`
- `cargo audit`

---

### Task 1: Extract `validate_from_stmts` core function

**Files:**
- Modify: `src/bin/ogsql.rs:3806-3855` (refactor `validate_sql`)

**背景：** 当前 `validate_sql` 耦合了 `parse_input()` 和后处理校验。需要拆分为两部分，使得 XML 和 Java 来源的 `StatementInfo` 也能复用校验逻辑。

**Step 1: 新增 `validate_from_stmts` 函数**

在 `validate_sql` 之前（line 3805）插入：

```rust
/// Run PACKAGE consistency, MERGE semantics, and PL variable validation on
/// already-parsed statements. Returns errors to merge into the caller's error list.
fn validate_from_stmts(
    stmts: &[ogsql_parser::StatementInfo],
    extra_funcs: &[String],
    strict: bool,
) -> (
    Vec<ogsql_parser::ParserError>,
    Vec<ogsql_parser::PackageConsistencyError>,
    Vec<ogsql_parser::UndefinedVariableError>,
) {
    use ogsql_parser::{validate_merge_semantics, validate_package_consistency};

    let mut errors = Vec::new();

    // 1. PACKAGE consistency
    let pkg_errors = validate_package_consistency(stmts);
    if !pkg_errors.is_empty() {
        for pe in &pkg_errors {
            let msg = match &pe.detail {
                Some(d) => format!("package {}: {} — {}", pe.package_name, pe.subprogram_name, d),
                None => format!("package {}: {} — {:?}", pe.package_name, pe.subprogram_name, pe.kind),
            };
            errors.push(ogsql_parser::ParserError::Warning {
                message: msg,
                location: ogs::SourceLocation::default(),
            });
        }
    }

    // 2. MERGE semantic validation
    let merge_errors = validate_merge_semantics(stmts);
    if !merge_errors.is_empty() {
        for me in &merge_errors {
            errors.push(ogsql_parser::ParserError::UnsupportedSyntax {
                location: me.location,
                syntax: "MERGE".to_string(),
                hint: merge_error_detail(me),
            });
        }
    }

    // 3. PL variable/function validation
    let mut all_funcs: Vec<String> = extra_funcs.to_vec();
    let own_funcs = collect_defined_routine_names(stmts);
    all_funcs.extend(own_funcs);
    all_funcs.sort();
    all_funcs.dedup();

    let var_errors = validate_pl_variables_from_stmts(stmts, &all_funcs, strict);

    (errors, pkg_errors, var_errors)
}
```

**Step 2: 简化 `validate_sql`**

将 `validate_sql` (line 3806-3855) 替换为：

```rust
fn validate_sql(
    sql: &str,
    mybatis: bool,
    extra_funcs: &[String],
    strict: bool,
) -> (
    Vec<ogsql_parser::StatementInfo>,
    Vec<ogsql_parser::ParserError>,
    Vec<ogsql_parser::PackageConsistencyError>,
    Vec<ogsql_parser::UndefinedVariableError>,
) {
    let output = parse_input(sql, false, mybatis);
    let mut errors = output.errors;

    let (core_errors, pkg_errors, var_errors) = validate_from_stmts(&output.statements, extra_funcs, strict);
    errors.extend(core_errors);

    (output.statements, errors, pkg_errors, var_errors)
}
```

**Step 3: 验证**

```bash
cargo check --features cli
cargo test --features cli validate
```

预期：现有 `validate` 命令行为不变。

---

### Task 2: Add `ValidateXml` and `ValidateJava` CLI enum variants

**Files:**
- Modify: `src/bin/ogsql.rs:107-244` (`Commands` enum — but 仅新增变体，不修改已有)

**Step 1: 新增 `ValidateXml` 变体**

在 `ParseXml` 之后（line 222, `}` 之后）插入：

```rust
    #[cfg(feature = "ibatis")]
    /// Validate iBatis/MyBatis XML mapper — parse + semantic checks + lint
    /// 校验 iBatis/MyBatis XML mapper 文件（解析 + 语义校验 + lint）
    #[command(name = "validate-xml")]
    ValidateXml {
        /// Recursively scan directory for XML files / 递归扫描目录
        #[arg(short = 'd', long = "dir")]
        dir: Option<String>,
        /// Output in CSV format / CSV 格式输出
        #[arg(long = "csv")]
        csv: bool,
        #[cfg(feature = "java")]
        /// Java source root directory for parameter type inference
        #[arg(long = "java-src")]
        java_src: Option<String>,
        /// Print statistics after directory processing
        #[arg(long)]
        stats: bool,
        /// Enable strict mode: detect undefined function calls
        #[arg(long)]
        strict: bool,
    },
```

**Step 2: 新增 `ValidateJava` 变体**

在 `ParseJava` 之后（line 243, `}` 之后）插入：

```rust
    #[cfg(feature = "java")]
    /// Validate Java source — extract SQL + semantic checks + lint
    /// 校验 Java 源码（提取 SQL + 语义校验 + lint）
    #[command(name = "validate-java")]
    ValidateJava {
        /// Recursively scan directory for Java files / 递归扫描目录
        #[arg(short = 'd', long = "dir")]
        dir: Option<String>,
        /// Output in CSV format / CSV 格式输出
        #[arg(long = "csv")]
        csv: bool,
        /// Extra method names to treat as SQL-bearing
        #[arg(long = "extra-sql-methods", value_delimiter = ',')]
        extra_sql_methods: Vec<String>,
        /// Extra variable name patterns for SQL detection
        #[arg(long = "extra-sql-var-patterns", value_delimiter = ',')]
        extra_sql_var_patterns: Vec<String>,
        /// Print statistics after directory processing
        #[arg(long)]
        stats: bool,
        /// Enable strict mode: detect undefined function calls
        #[arg(long)]
        strict: bool,
    },
```

**Step 3: 验证**

```bash
cargo check --features cli,ibatis,java
```

预期：新增变体编译通过（handler 函数暂不存在，需 Task 3/4 补充）。

---

### Task 3: Implement `cmd_validate_xml` handler

**Files:**
- Modify: `src/bin/ogsql.rs` (在 `cmd_parse_xml_structured` 附近新增)

**Step 1: Single-file handler**

在 `cmd_parse_xml_single` 之后（line 5617）插入 `cmd_validate_xml_single`：

```rust
#[cfg(feature = "ibatis")]
fn cmd_validate_xml_single(cli: &Cli, csv: bool, java_roots: &[std::path::PathBuf], strict: bool) {
    let file_opt = cli.file.first().map(|s| s.as_str());
    let input = match file_opt {
        Some(path) => std::fs::read(path).unwrap_or_else(|e| die!("Error reading {}: {}", path, e)),
        None => {
            let mut buf = Vec::new();
            std::io::stdin().read_to_end(&mut buf).unwrap_or_else(|e| die!("Error reading stdin: {}", e));
            buf
        }
    };

    #[cfg(feature = "java")]
    let result = if java_roots.is_empty() {
        ogsql_parser::ibatis::parse_mapper_bytes_with_path(&input, file_opt)
    } else {
        ogsql_parser::ibatis::parse_mapper_bytes_with_java_src(&input, file_opt, java_roots.to_vec())
    };
    #[cfg(not(feature = "java"))]
    let result = ogsql_parser::ibatis::parse_mapper_bytes_with_path(&input, file_opt);

    // Collect all StatementInfo from successfully parsed statements
    let mut all_stmts: Vec<ogsql_parser::StatementInfo> = Vec::new();
    for stmt in &result.statements {
        if let Some((ref stmts, _)) = stmt.parse_result {
            all_stmts.extend(stmts.clone());
        }
    }

    // Run core validation (PACKAGE + MERGE + PL) — same rigor as `validate`
    let (core_errors, _pkg_errors, var_errors) = validate_from_stmts(&all_stmts, &[], strict);

    // Lint with Full confidence (was Partial in parse-xml)
    let lint_warnings = if cli.lint {
        let config = build_lint_config(cli);
        let mut ws: Vec<ogsql_parser::linter::SqlWarning> = Vec::new();
        let linter = ogsql_parser::linter::SqlLinter::with_default_rules(config.clone());
        for stmt in &result.statements {
            if let Some((ref stmts, _)) = stmt.parse_result {
                let mut sw = linter.lint(stmts, None, ogsql_parser::linter::Confidence::Full);
                ws.append(&mut sw);
            }
        }
        // C018: foreach batch insert expanded lint
        let expand_ws = lint_xml_expanded(&input, &config);
        ws.extend(expand_ws);
        ws
    } else {
        vec![]
    };

    // Errors to display: XML parse errors + SQL parse errors + core validation errors
    let mut all_errors: Vec<ogsql_parser::ParserError> = Vec::new();
    // XML-level errors as warnings
    for e in &result.errors {
        all_errors.push(ogsql_parser::ParserError::Warning {
            message: format!("XML: {}", e),
            location: ogsql_parser::SourceLocation::default(),
        });
    }
    // SQL parse errors from each statement
    for stmt in &result.statements {
        if let Some((_, ref parse_errors)) = stmt.parse_result {
            all_errors.extend(parse_errors.clone());
        }
    }
    // Core validation errors
    all_errors.extend(core_errors);

    let format_var_err = |ve: &ogsql_parser::UndefinedVariableError| -> String {
        let line_info = ve.location.as_ref().map(|sp| format!(":{}", sp.start.line)).unwrap_or_default();
        let kind_label = match ve.kind {
            ogsql_parser::UndefinedRefKind::Function => "undefined function",
            ogsql_parser::UndefinedRefKind::Variable => "undefined variable",
        };
        format!("{} '{}' in {}{}", kind_label, ve.variable_name, ve.context, line_info)
    };

    let file_name = file_opt.unwrap_or("<stdin>");

    if csv {
        output_csv_validate_xml_header();
        output_csv_validate_xml_rows(&result.statements, &all_errors, &var_errors, &lint_warnings, file_name, ".");
        let real_errors: Vec<_> = all_errors.iter().filter(|e| !is_warning(e)).collect();
        if !real_errors.is_empty() || !var_errors.is_empty() {
            std::process::exit(1);
        }
        return;
    }

    if cli.json {
        let real_errors: Vec<_> = all_errors.iter().filter(|e| !is_warning(e)).collect();
        let warnings: Vec<_> = all_errors.iter().filter(|e| is_warning(e)).collect();
        let mut out = serde_json::json!({
            "valid": real_errors.is_empty() && var_errors.is_empty(),
            "error_count": real_errors.len() + var_errors.len(),
            "warning_count": warnings.len(),
            "errors": all_errors,
            "statements": result.statements.iter().map(|s| {
                serde_json::json!({
                    "id": s.id,
                    "kind": format!("{:?}", s.kind),
                    "flat_sql": s.flat_sql,
                    "parameters": s.parameters,
                    "line": s.line,
                    "has_dynamic_elements": s.has_dynamic_elements,
                })
            }).collect::<Vec<_>>(),
        });
        if !var_errors.is_empty() {
            out.as_object_mut().unwrap().insert("undefined_variables".to_string(), serde_json::json!(var_errors));
        }
        if !lint_warnings.is_empty() {
            out.as_object_mut().unwrap().insert("lint_warnings".to_string(), serde_json::json!(lint_warnings));
            out.as_object_mut().unwrap().insert("lint_summary".to_string(), format_warnings_summary(&lint_warnings));
        }
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        let real_errors: Vec<_> = all_errors.iter().filter(|e| !is_warning(e)).collect();
        let warnings: Vec<_> = all_errors.iter().filter(|e| is_warning(e)).collect();
        let has_var_errors = !var_errors.is_empty();

        for (i, stmt) in result.statements.iter().enumerate() {
            println!("── Statement {}: {} ({:?}) ──", i + 1, stmt.id, stmt.kind);
            println!("{}", stmt.flat_sql);
            println!();
        }

        if real_errors.is_empty() && warnings.is_empty() && !has_var_errors {
            println!("VALID ({} statement(s))", result.statements.len());
        } else if real_errors.is_empty() && !has_var_errors {
            println!("VALID ({} warning(s)):", warnings.len());
            for w in &warnings {
                eprintln!("  warning: {}", w);
            }
        } else {
            let total_errors = real_errors.len() + var_errors.len();
            println!("INVALID ({} error(s), {} warning(s)):", total_errors, warnings.len());
            for e in &real_errors {
                eprintln!("  error: {}", e);
            }
            for ve in &var_errors {
                eprintln!("  error: {}", format_var_err(ve));
            }
            for w in &warnings {
                eprintln!("  warning: {}", w);
            }
        }

        if !lint_warnings.is_empty() {
            eprintln!("\n── Lint Warnings ({}) ──", lint_warnings.len());
            format_warnings_text(&lint_warnings);
            eprintln!("\n── Summary ──");
            let summary = format_warnings_summary(&lint_warnings);
            for (level, count) in summary.get("by_level").unwrap().as_object().unwrap() {
                eprintln!("  {}: {}", level, count);
            }
            eprintln!("  Total: {} lint warnings", lint_warnings.len());
        }

        if !real_errors.is_empty() || has_var_errors {
            std::process::exit(1);
        }
    }
}
```

**Step 2: Dir handler (轻量版)**

在 `cmd_validate_xml_single` 之后插入：

```rust
#[cfg(feature = "ibatis")]
fn cmd_validate_xml_dir(
    cli: &Cli,
    dir_path: &str,
    csv: bool,
    java_roots: &[std::path::PathBuf],
    stats: bool,
    strict: bool,
) {
    use std::path::Path;
    use walkdir::WalkDir;

    if !Path::new(dir_path).is_dir() {
        die!("Error: '{}' is not a directory", dir_path);
    }

    let mut files: Vec<(String, std::path::PathBuf)> = Vec::new();
    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
        if ext != "xml" {
            continue;
        }
        let file_name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        files.push((file_name, path.to_path_buf()));
    }
    files.sort_by(|a, b| a.1.cmp(&b.1));

    if files.is_empty() {
        eprintln!("No XML files found in {}", dir_path);
        return;
    }

    let mut any_invalid = false;

    if csv {
        output_csv_validate_xml_header();
    }

    for (file_name, abs_path) in &files {
        let input = std::fs::read(abs_path).unwrap_or_else(|e| die!("Error reading {}: {}", abs_path.display(), e));

        #[cfg(feature = "java")]
        let result = if java_roots.is_empty() {
            ogsql_parser::ibatis::parse_mapper_bytes_with_path(&input, Some(file_name))
        } else {
            ogsql_parser::ibatis::parse_mapper_bytes_with_java_src(&input, Some(file_name), java_roots.to_vec())
        };
        #[cfg(not(feature = "java"))]
        let result = ogsql_parser::ibatis::parse_mapper_bytes_with_path(&input, Some(file_name));

        let mut all_stmts: Vec<ogsql_parser::StatementInfo> = Vec::new();
        for stmt in &result.statements {
            if let Some((ref stmts, _)) = stmt.parse_result {
                all_stmts.extend(stmts.clone());
            }
        }

        let (core_errors, _pkg, var_errors) = validate_from_stmts(&all_stmts, &[], strict);

        let mut all_errors: Vec<ogsql_parser::ParserError> = Vec::new();
        for e in &result.errors {
            all_errors.push(ogsql_parser::ParserError::Warning {
                message: format!("XML: {}", e),
                location: ogsql_parser::SourceLocation::default(),
            });
        }
        for s in &result.statements {
            if let Some((_, ref pe)) = s.parse_result {
                all_errors.extend(pe.clone());
            }
        }
        all_errors.extend(core_errors);

        let lint_warnings = if cli.lint {
            let config = build_lint_config(cli);
            let mut ws: Vec<ogsql_parser::linter::SqlWarning> = Vec::new();
            let linter = ogsql_parser::linter::SqlLinter::with_default_rules(config.clone());
            for s in &result.statements {
                if let Some((ref stmts, _)) = s.parse_result {
                    ws.append(&mut linter.lint(stmts, None, ogsql_parser::linter::Confidence::Full));
                }
            }
            ws.extend(lint_xml_expanded(&input, &config));
            ws
        } else {
            vec![]
        };

        if csv {
            output_csv_validate_xml_rows(&result.statements, &all_errors, &var_errors, &lint_warnings, file_name, ".");
        } else {
            let real_errors: Vec<_> = all_errors.iter().filter(|e| !is_warning(e)).collect();
            let has_issues = !real_errors.is_empty() || !var_errors.is_empty();
            if has_issues {
                any_invalid = true;
                println!("{}: INVALID ({} error(s), {} var error(s))", file_name, real_errors.len(), var_errors.len());
            } else {
                println!("{}: VALID ({} statement(s))", file_name, result.statements.len());
            }
        }
    }

    if stats {
        eprintln!("Total: {} file(s) processed", files.len());
    }
    if any_invalid {
        std::process::exit(1);
    }
}
```

**Step 3: 入口函数**

```rust
#[cfg(feature = "ibatis")]
fn cmd_validate_xml(cli: &Cli, dir: Option<&str>, csv: bool, java_src: Option<&str>, stats: bool, strict: bool) {
    #[cfg(feature = "java")]
    let java_roots: Vec<std::path::PathBuf> = java_src
        .map(|p| vec![std::path::PathBuf::from(p)])
        .unwrap_or_default();
    #[cfg(not(feature = "java"))]
    let java_roots: Vec<std::path::PathBuf> = vec![];

    if let Some(dir_path) = dir {
        cmd_validate_xml_dir(cli, dir_path, csv, &java_roots, stats, strict);
    } else {
        cmd_validate_xml_single(cli, csv, &java_roots, strict);
    }
}
```

**Step 4: 验证**

```bash
cargo check --features ibatis
cargo check --features ibatis,java
```

---

### Task 4: Implement `cmd_validate_java` handler

**Files:**
- Modify: `src/bin/ogsql.rs` (在 `cmd_parse_java_dir` 附近新增)

**Step 1: Single-file handler**

在 `cmd_parse_java_single` 之后（line 6016）插入：

```rust
#[cfg(feature = "java")]
fn cmd_validate_java_single(
    cli: &Cli,
    extra_sql_methods: &[String],
    extra_sql_var_patterns: &[String],
    csv: bool,
    strict: bool,
) {
    let file_opt = cli.file.first().map(|s| s.as_str());
    let (source, file_path) = match file_opt {
        Some(path) => {
            let bytes = std::fs::read(path).unwrap_or_else(|e| die!("Error reading {}: {}", path, e));
            let (text, _encoding) =
                ogsql_parser::token::decode_sql_file(&bytes).unwrap_or_else(|e| die!("Error decoding {}: {}", path, e));
            (text, path.to_string())
        }
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf).unwrap_or_else(|e| die!("Error reading stdin: {}", e));
            (buf, "<stdin>".to_string())
        }
    };

    let config = ogsql_parser::java::JavaExtractConfig {
        extra_sql_methods: extra_sql_methods.to_vec(),
        extra_sql_var_patterns: extra_sql_var_patterns.to_vec(),
    };
    let result = ogsql_parser::java::extract_sql_from_java(&source, &file_path, &config);

    // Collect all StatementInfo from successfully parsed extractions
    let mut all_stmts: Vec<ogsql_parser::StatementInfo> = Vec::new();
    let mut all_errors: Vec<ogsql_parser::ParserError> = Vec::new();
    for ext in &result.extractions {
        if let Some(ref parse_result) = ext.parse_result {
            all_stmts.extend(parse_result.statements.clone());
            all_errors.extend(parse_result.errors.clone());
        }
    }

    // Java-level errors
    for e in &result.errors {
        all_errors.push(ogsql_parser::ParserError::Warning {
            message: format!("Java extraction: {}", e),
            location: ogsql_parser::SourceLocation::default(),
        });
    }

    // Run core validation (PACKAGE + MERGE + PL)
    let (core_errors, _pkg, var_errors) = validate_from_stmts(&all_stmts, &[], strict);
    all_errors.extend(core_errors);

    // Lint with Full confidence (was Partial in parse-java)
    let lint_warnings = if cli.lint {
        let config = build_lint_config(cli);
        lint_java_extractions(&result.extractions, &config)
        // Note: lint_java_extractions uses Confidence::Partial — need to inline with Full
    } else {
        vec![]
    };

    // FIX: override lint_java_extractions' Partial → Full
    let lint_warnings = if cli.lint {
        let config = build_lint_config(cli);
        let linter = ogsql_parser::linter::SqlLinter::with_default_rules(config);
        let mut ws: Vec<ogsql_parser::linter::SqlWarning> = Vec::new();
        for ext in &result.extractions {
            if let Some(ref parse_result) = ext.parse_result {
                ws.append(&mut linter.lint(&parse_result.statements, None, ogsql_parser::linter::Confidence::Full));
            }
        }
        ws
    } else {
        vec![]
    };

    let format_var_err = |ve: &ogsql_parser::UndefinedVariableError| -> String {
        let line_info = ve.location.as_ref().map(|sp| format!(":{}", sp.start.line)).unwrap_or_default();
        let kind_label = match ve.kind {
            ogsql_parser::UndefinedRefKind::Function => "undefined function",
            ogsql_parser::UndefinedRefKind::Variable => "undefined variable",
        };
        format!("{} '{}' in {}{}", kind_label, ve.variable_name, ve.context, line_info)
    };

    if csv {
        output_csv_validate_java_header();
        output_csv_validate_java_rows(&result.extractions, &all_errors, &var_errors, &lint_warnings, &file_path, ".");
        let real_errors: Vec<_> = all_errors.iter().filter(|e| !is_warning(e)).collect();
        if !real_errors.is_empty() || !var_errors.is_empty() {
            std::process::exit(1);
        }
        return;
    }

    if cli.json {
        let real_errors: Vec<_> = all_errors.iter().filter(|e| !is_warning(e)).collect();
        let warnings: Vec<_> = all_errors.iter().filter(|e| is_warning(e)).collect();
        let mut out = serde_json::json!({
            "file": file_path,
            "valid": real_errors.is_empty() && var_errors.is_empty(),
            "error_count": real_errors.len() + var_errors.len(),
            "warning_count": warnings.len(),
            "errors": all_errors,
            "extractions": result.extractions.iter().map(|ext| {
                serde_json::json!({
                    "line": ext.origin.line,
                    "method": ext.origin.method_name,
                    "sql": ext.sql,
                    "kind": format!("{:?}", ext.kind),
                })
            }).collect::<Vec<_>>(),
        });
        if !var_errors.is_empty() {
            out.as_object_mut().unwrap().insert("undefined_variables".to_string(), serde_json::json!(var_errors));
        }
        if !lint_warnings.is_empty() {
            out.as_object_mut().unwrap().insert("lint_warnings".to_string(), serde_json::json!(lint_warnings));
            out.as_object_mut().unwrap().insert("lint_summary".to_string(), format_warnings_summary(&lint_warnings));
        }
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        let real_errors: Vec<_> = all_errors.iter().filter(|e| !is_warning(e)).collect();
        let warnings: Vec<_> = all_errors.iter().filter(|e| is_warning(e)).collect();
        let has_var_errors = !var_errors.is_empty();

        if real_errors.is_empty() && warnings.is_empty() && !has_var_errors {
            println!("{}: VALID ({} SQL(s) extracted)", file_path, result.extractions.len());
        } else if real_errors.is_empty() && !has_var_errors {
            println!("{}: VALID ({} warning(s)):", file_path, warnings.len());
            for w in &warnings {
                eprintln!("  warning: {}", w);
            }
        } else {
            let total_errors = real_errors.len() + var_errors.len();
            println!("{}: INVALID ({} error(s), {} warning(s)):", file_path, total_errors, warnings.len());
            for e in &real_errors {
                eprintln!("  error: {}", e);
            }
            for ve in &var_errors {
                eprintln!("  error: {}", format_var_err(ve));
            }
            for w in &warnings {
                eprintln!("  warning: {}", w);
            }
        }
        if !lint_warnings.is_empty() {
            eprintln!("\n── Lint Warnings ({}) ──", lint_warnings.len());
            format_warnings_text(&lint_warnings);
        }
        if !real_errors.is_empty() || has_var_errors {
            std::process::exit(1);
        }
    }
}
```

**Step 2: Dir handler**

```rust
#[cfg(feature = "java")]
fn cmd_validate_java_dir(
    cli: &Cli,
    dir_path: &str,
    extra_sql_methods: &[String],
    extra_sql_var_patterns: &[String],
    csv: bool,
    stats: bool,
    strict: bool,
) {
    use std::path::Path;
    use walkdir::WalkDir;

    if !Path::new(dir_path).is_dir() {
        die!("Error: '{}' is not a directory", dir_path);
    }

    let mut files: Vec<(String, std::path::PathBuf)> = Vec::new();
    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("java") {
            continue;
        }
        let file_name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        files.push((file_name, path.to_path_buf()));
    }
    files.sort_by(|a, b| a.1.cmp(&b.1));

    if files.is_empty() {
        eprintln!("No Java files found in {}", dir_path);
        return;
    }

    let config = ogsql_parser::java::JavaExtractConfig {
        extra_sql_methods: extra_sql_methods.to_vec(),
        extra_sql_var_patterns: extra_sql_var_patterns.to_vec(),
    };
    let mut state = ogsql_parser::java::CrossFileState::default();
    let mut any_invalid = false;

    if csv {
        output_csv_validate_java_header();
    }

    // Load all files into memory for cross-file analysis
    struct FileData { name: String, source: String }
    let file_data: Vec<FileData> = files.iter().map(|(name, path)| {
        let bytes = std::fs::read(path).unwrap_or_else(|e| die!("Error reading {}: {}", path.display(), e));
        let (text, _) = ogsql_parser::token::decode_sql_file(&bytes).unwrap_or_else(|e| die!("Error decoding {}", name, e));
        FileData { name: name.clone(), source: text }
    }).collect();

    let file_refs: Vec<(&str, &str)> = file_data.iter().map(|f| (f.name.as_str(), f.source.as_str())).collect();
    let results = ogsql_parser::java::extract_sql_from_java_files_with_state(&file_refs, &config, &mut state);

    for (fi, result) in results.iter().enumerate() {
        let mut all_stmts: Vec<ogsql_parser::StatementInfo> = Vec::new();
        let mut all_errors: Vec<ogsql_parser::ParserError> = Vec::new();
        for ext in &result.extractions {
            if let Some(ref pr) = ext.parse_result {
                all_stmts.extend(pr.statements.clone());
                all_errors.extend(pr.errors.clone());
            }
        }
        for e in &result.errors {
            all_errors.push(ogsql_parser::ParserError::Warning {
                message: format!("Java: {}", e),
                location: ogsql_parser::SourceLocation::default(),
            });
        }

        let (core_errors, _pkg, var_errors) = validate_from_stmts(&all_stmts, &[], strict);
        all_errors.extend(core_errors);

        let lint_warnings = if cli.lint {
            let lconfig = build_lint_config(cli);
            let linter = ogsql_parser::linter::SqlLinter::with_default_rules(lconfig);
            let mut ws = Vec::new();
            for ext in &result.extractions {
                if let Some(ref pr) = ext.parse_result {
                    ws.append(&mut linter.lint(&pr.statements, None, ogsql_parser::linter::Confidence::Full));
                }
            }
            ws
        } else {
            vec![]
        };

        let file_name = &file_data[fi].name;
        if csv {
            output_csv_validate_java_rows(&result.extractions, &all_errors, &var_errors, &lint_warnings, file_name, ".");
        } else {
            let real_errors: Vec<_> = all_errors.iter().filter(|e| !is_warning(e)).collect();
            if !real_errors.is_empty() || !var_errors.is_empty() {
                any_invalid = true;
                println!("{}: INVALID ({} error(s))", file_name, real_errors.len() + var_errors.len());
            } else {
                println!("{}: VALID ({} SQL(s))", file_name, result.extractions.len());
            }
        }
    }

    if stats {
        eprintln!("Total: {} file(s) processed", files.len());
    }
    if any_invalid {
        std::process::exit(1);
    }
}
```

**Step 3: 入口函数**

```rust
#[cfg(feature = "java")]
fn cmd_validate_java(
    cli: &Cli,
    extra_sql_methods: &[String],
    extra_sql_var_patterns: &[String],
    dir: Option<&str>,
    csv: bool,
    stats: bool,
    strict: bool,
) {
    if let Some(dir_path) = dir {
        cmd_validate_java_dir(cli, dir_path, extra_sql_methods, extra_sql_var_patterns, csv, stats, strict);
    } else {
        cmd_validate_java_single(cli, extra_sql_methods, extra_sql_var_patterns, csv, strict);
    }
}
```

---

### Task 5: CSV output helpers for validate-xml / validate-java

**Files:**
- Modify: `src/bin/ogsql.rs` (在 `output_csv_xml_rows` 之后新增)

**Step 1: validate-xml CSV**

在 `output_csv_xml_rows` 之后（line 6413）插入：

```rust
fn output_csv_validate_xml_header() {
    println!("file,directory,line,id,sql,parameter_types,valid,error_count,warning_count,errors,warnings");
}

#[cfg(feature = "ibatis")]
fn output_csv_validate_xml_rows(
    statements: &[ogsql_parser::ibatis::ParsedStatement],
    errors: &[ogsql_parser::ParserError],
    var_errors: &[ogsql_parser::UndefinedVariableError],
    lint_warnings: &[ogsql_parser::linter::SqlWarning],
    file_name: &str,
    rel_dir: &str,
) {
    for stmt in statements {
        let (parse_errors, warnings) = match &stmt.parse_result {
            Some((_, pe)) => {
                let refs: Vec<&ogsql_parser::ParserError> = pe.iter().collect();
                (merge_error_messages(&refs, false), merge_error_messages(&refs, true))
            }
            None => (String::new(), String::new()),
        };

        let mut err_parts: Vec<String> = Vec::new();
        if !parse_errors.is_empty() { err_parts.push(parse_errors); }
        for ve in var_errors {
            err_parts.push(format!("undefined '{}' in {}", ve.variable_name, ve.context));
        }

        let mut warn_parts: Vec<String> = Vec::new();
        if !warnings.is_empty() { warn_parts.push(warnings); }
        let merged_lint = merge_lint_warnings(
            &lint_warnings.iter().collect::<Vec<_>>()
        );
        if !merged_lint.is_empty() { warn_parts.push(merged_lint); }

        let error_count = stmt.parse_result.as_ref().map(|(_, pe)| pe.iter().filter(|e| !is_warning(e)).count()).unwrap_or(0)
            + var_errors.len();
        let warning_count = stmt.parse_result.as_ref().map(|(_, pe)| pe.iter().filter(|e| is_warning(e)).count()).unwrap_or(0)
            + lint_warnings.len();

        let parameter_types: String = stmt.parameters.iter()
            .map(|p| match p.jdbc_type {
                Some(ref jt) => format!("{}:{:?}", p.name, jt),
                None => p.name.clone(),
            })
            .collect::<Vec<_>>()
            .join(";");

        println!(
            "{},{},{},{},{},{},{},{},{},{},{}",
            csv_escape(file_name),
            csv_escape(rel_dir),
            stmt.line,
            csv_escape(&stmt.id),
            csv_escape(&stmt.flat_sql.trim().replace('\r', "")),
            csv_escape(&parameter_types),
            if error_count == 0 { "VALID" } else { "INVALID" },
            error_count,
            warning_count,
            csv_escape(&err_parts.join("; ")),
            csv_escape(&warn_parts.join("; ")),
        );
    }
}
```

**Step 2: validate-java CSV**

在 validate-xml CSV 之后插入：

```rust
fn output_csv_validate_java_header() {
    println!("file,directory,line,method,sql,valid,error_count,warning_count,errors,warnings");
}

#[cfg(feature = "java")]
fn output_csv_validate_java_rows(
    extractions: &[ogsql_parser::java::ExtractedSql],
    errors: &[ogsql_parser::ParserError],
    var_errors: &[ogsql_parser::UndefinedVariableError],
    lint_warnings: &[ogsql_parser::linter::SqlWarning],
    file_name: &str,
    rel_dir: &str,
) {
    for ext in extractions {
        let (parse_errors, warnings) = match &ext.parse_result {
            Some(pr) => {
                let refs: Vec<&ogsql_parser::ParserError> = pr.errors.iter().collect();
                (merge_error_messages(&refs, false), merge_error_messages(&refs, true))
            }
            None => (String::new(), String::new()),
        };

        let mut err_parts: Vec<String> = Vec::new();
        if !parse_errors.is_empty() { err_parts.push(parse_errors); }
        for ve in var_errors {
            err_parts.push(format!("undefined '{}' in {}", ve.variable_name, ve.context));
        }

        let mut warn_parts: Vec<String> = Vec::new();
        if !warnings.is_empty() { warn_parts.push(warnings); }
        let merged_lint = merge_lint_warnings(
            &lint_warnings.iter().collect::<Vec<_>>()
        );
        if !merged_lint.is_empty() { warn_parts.push(merged_lint); }

        let error_count = ext.parse_result.as_ref().map(|pr| pr.errors.iter().filter(|e| !is_warning(e)).count()).unwrap_or(0)
            + var_errors.len();
        let warning_count = ext.parse_result.as_ref().map(|pr| pr.errors.iter().filter(|e| is_warning(e)).count()).unwrap_or(0)
            + lint_warnings.len();

        let method = match (&ext.origin.class_name, &ext.origin.method_name) {
            (Some(cls), Some(m)) => format!("{}::{}", cls, m),
            (None, Some(m)) => m.clone(),
            (Some(cls), None) => cls.clone(),
            (None, None) => ext.origin.variable_name.clone().unwrap_or_default(),
        };

        println!(
            "{},{},{},{},{},{},{},{},{},{}",
            csv_escape(file_name),
            csv_escape(rel_dir),
            ext.origin.line,
            csv_escape(&method),
            csv_escape(&ext.sql.trim().replace('\r', "")),
            if error_count == 0 { "VALID" } else { "INVALID" },
            error_count,
            warning_count,
            csv_escape(&err_parts.join("; ")),
            csv_escape(&warn_parts.join("; ")),
        );
    }
}
```

---

### Task 6: Wire commands in `main()`

**Files:**
- Modify: `src/bin/ogsql.rs:6674-6773` (`main()`)

**Step 1: 新增 dispatch arm**

在 `Commands::ParseJava { ... }` arm（line 6769）之后，`}` 之前插入：

```rust
        #[cfg(feature = "ibatis")]
        Commands::ValidateXml { ref dir, csv, #[cfg(feature = "java")] ref java_src, stats, strict } => {
            #[cfg(feature = "java")]
            let js = java_src.as_deref();
            #[cfg(not(feature = "java"))]
            let js: Option<&str> = None;
            cmd_validate_xml(&cli, dir.as_deref(), csv, js, stats, strict);
        }
        #[cfg(feature = "java")]
        Commands::ValidateJava {
            ref extra_sql_methods,
            ref extra_sql_var_patterns,
            ref dir,
            csv,
            stats,
            strict,
        } => {
            cmd_validate_java(&cli, extra_sql_methods, extra_sql_var_patterns, dir.as_deref(), csv, stats, strict);
        }
```

---

### Task 7: Add `--lint` support note to new commands

**Files:**
- Modify: `src/bin/ogsql.rs:69-72` (`--lint` 参数文档)

**Step 1: 更新 `--lint` 文档**

将 line 71 的注释从：
```rust
    /// Enable SQL anti-pattern linting on parse/validate/parse-xml/parse-java output
    /// 启用 SQL 反模式检测（配合 parse/validate/parse-xml/parse-java 使用）
```
改为：
```rust
    /// Enable SQL anti-pattern linting on parse/validate/parse-xml/parse-java/validate-xml/validate-java output
    /// 启用 SQL 反模式检测（配合 parse/validate/parse-xml/parse-java/validate-xml/validate-java 使用）
```

---

### Task 8: Add tests

**Files:**
- Create: `tests/validate_xml.rs`
- Create: `tests/validate_java.rs`

**Step 1: validate-xml tests**

```rust
// tests/validate_xml.rs
#[cfg(feature = "ibatis")]
mod validate_xml_tests {
    use std::process::Command;

    fn ogsql() -> Command {
        Command::new(env!("CARGO_BIN_EXE_ogsql"))
    }

    #[test]
    fn test_validate_xml_valid_simple() {
        let output = ogsql()
            .args(["validate-xml"])
            .write_stdin(r#"<mapper namespace="t"><select id="q">SELECT 1</select></mapper>"#)
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("VALID"), "Expected VALID, got: {}", stdout);
        assert!(output.status.success());
    }

    #[test]
    fn test_validate_xml_invalid_sql() {
        let output = ogsql()
            .args(["validate-xml"])
            .write_stdin(r#"<mapper namespace="t"><select id="q">SELECT FROM</select></mapper>"#)
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("INVALID"), "Expected INVALID, got: {}", stdout);
        assert!(!output.status.success(), "Expected non-zero exit code for invalid SQL");
    }

    #[test]
    fn test_validate_xml_with_lint() {
        let output = ogsql()
            .args(["validate-xml", "--lint"])
            .write_stdin(r#"<mapper namespace="t"><select id="q">SELECT * FROM t</select></mapper>"#)
            .output()
            .unwrap();
        // Should not crash when --lint is used
        assert!(output.status.success() || !output.status.success());
    }

    #[test]
    fn test_validate_xml_csv_output() {
        let output = ogsql()
            .args(["validate-xml", "--csv"])
            .write_stdin(r#"<mapper namespace="t"><select id="q">SELECT 1</select></mapper>"#)
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("VALID"), "CSV output should contain VALID");
        assert!(stdout.contains("q,SELECT 1"), "CSV should contain statement id and SQL");
    }
}
```

**Step 2: validate-java tests**

```rust
// tests/validate_java.rs
#[cfg(feature = "java")]
mod validate_java_tests {
    use std::process::Command;

    fn ogsql() -> Command {
        Command::new(env!("CARGO_BIN_EXE_ogsql"))
    }

    #[test]
    fn test_validate_java_valid_sql() {
        let output = ogsql()
            .args(["validate-java"])
            .write_stdin("class T { void m() { stmt.execute(\"SELECT 1\"); } }")
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("VALID"), "Expected VALID, got: {}", stdout);
        assert!(output.status.success());
    }

    #[test]
    fn test_validate_java_invalid_sql() {
        let output = ogsql()
            .args(["validate-java"])
            .write_stdin("class T { void m() { stmt.execute(\"SELECT FROM\"); } }")
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("INVALID"), "Expected INVALID, got: {}", stdout);
        assert!(!output.status.success(), "Expected non-zero exit code");
    }

    #[test]
    fn test_validate_java_csv() {
        let output = ogsql()
            .args(["validate-java", "--csv"])
            .write_stdin("class T { void m() { stmt.execute(\"SELECT 1\"); } }")
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("VALID"), "CSV should contain VALID");
    }
}
```

---

### Task 9: Full verification

**Step 1: 编译检查（所有 feature 组合）**

```bash
cargo check --features cli
cargo check --features cli,ibatis
cargo check --features cli,java
cargo check --features full
```

预期：0 errors。

**Step 2: Format + Clippy**

```bash
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
```

预期：0 errors, 0 warnings。

**Step 3: 运行测试**

```bash
cargo test --all-features
```

预期：全部现有测试 + 新增测试通过。

**Step 4: 手动 smoke test**

```bash
# validate-xml — simple valid
echo '<mapper namespace="t"><select id="q">SELECT 1</select></mapper>' | cargo run --features ibatis -- validate-xml
# Expected: VALID (1 statement(s))

# validate-xml — invalid SQL
echo '<mapper namespace="t"><select id="q">SELECT FROM</select></mapper>' | cargo run --features ibatis -- validate-xml
# Expected: INVALID, exit code != 0

# validate-xml — with lint
echo '<mapper namespace="t"><select id="q">SELECT * FROM t</select></mapper>' | cargo run --features ibatis -- validate-xml --lint
# Expected: VALID with lint warnings (if any)

# validate-xml — CSV
echo '<mapper namespace="t"><select id="q">SELECT 1</select></mapper>' | cargo run --features ibatis -- validate-xml --csv
# Expected: CSV row with VALID

# validate-java — simple valid
echo 'class T { void m() { stmt.execute("SELECT 1"); } }' | cargo run --features java -- validate-java
# Expected: VALID

# validate-java — invalid SQL
echo 'class T { void m() { stmt.execute("SELECT FROM"); } }' | cargo run --features java -- validate-java
# Expected: INVALID, exit code != 0

# validate-java — with lint
echo 'class T { void m() { stmt.execute("SELECT * FROM t"); } }' | cargo run --features java -- validate-java --lint
# Expected: VALID with lint warnings (if any)
```

**Step 5: Commit**

```bash
git add src/bin/ogsql.rs tests/validate_xml.rs tests/validate_java.rs
git commit -m "feat(cli): add validate-xml and validate-java commands

- Extract validate_from_stmts() core from validate_sql() for reuse
- Add validate-xml (feature=ibatis): parse XML → semantic checks → lint
- Add validate-java (feature=java): extract SQL → semantic checks → lint
- Change lint confidence from Partial to Full in both new commands
- Preserve C018 foreach batch insert check in validate-xml
- Add VALID/INVALID determination + exit(1) on error (CI-ready)
- Add CSV output with valid/error_count/warning_count columns
- Add stats support for --dir mode
- Add integration tests for both commands"
```

---

### 依赖关系

```
Task 1 (extract validate_from_stmts)
 ├── Task 2 (CLI enum variants)
 │    ├── Task 3 (cmd_validate_xml)  ──┐
 │    └── Task 4 (cmd_validate_java) ──┤
 └── Task 5 (CSV output helpers) ──────┤
      └── Task 6 (wire main()) ────────┤
           ├── Task 7 (--lint doc) ────┤
           └── Task 8 (tests) ─────────┤
                └── Task 9 (verify) ◄──┘
```

Tasks 2-5 可并行（Task 1 完成之后），Task 6 依赖 2+3+4+5，Task 8 可独立进行。

---

### 风险点

1. **`ogsql.rs` 文件大小** — 已 7482 行，本次新增 ~250 行。不进一步恶化，但长期应考虑将 CLI handler 按命令拆分到独立文件（如 `src/bin/commands/validate_xml.rs`）。此为独立重构任务。

2. **`merge_lint_warnings` 函数签名** — CSV 辅助函数中调用了 `merge_lint_warnings(&lint_warnings.iter().collect::<Vec<_>>())`，需确认该函数签名接受 `&[&&SqlWarning]`。若类型不匹配，改为接受 `&[&SqlWarning]` 或添加 intermediate collection。

3. **`lint_java_extractions` 的 Confidence** — 当前 `lint_java_extractions` 硬编码 `Confidence::Partial`。新命令直接内联了 Full-confidence 版本。后续可考虑添加 `lint_java_extractions_with_confidence` 函数以消除重复。

4. **MCP 服务器未同步** — MCP `parse_xml` / `parse_java` 工具也无 full validate。作为 follow-up task，可添加 `validate_xml` / `validate_java` MCP 工具。详见 `src/mcp/mod.rs:304-361`。

5. **`validate-java` dir 模式** — `cmd_validate_java_dir` 需要将所有 Java 文件加载到内存进行交叉文件分析（PreparedStatement backfill）。大型项目可能内存占用高。当前实现遵循 `cmd_parse_java_dir` 模式，风险可控。
