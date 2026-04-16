# GaussDB P0/P1 语法完备性实施计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 补齐 GaussDB 语法解析的 P0/P1 关键缺失特性，覆盖 ALTER GROUP、CTAS、CREATE AGGREGATE/OPERATOR、ALTER DEFAULT PRIVILEGES、USER MAPPING、DROP 扩展等。

**Architecture:** 所有变更遵循现有架构 — AST 类型在 `src/ast/mod.rs`，Parser 方法分布在 `src/parser/` 各模块（ddl.rs, dml.rs, select.rs, utility.rs, mod.rs），Formatter 在 `src/formatter.rs`，测试在 `src/parser/tests.rs`。每个 Task 遵循 TDD: 写测试 → 确认失败 → 实现 → 确认通过。

**Tech Stack:** Rust 2021, thiserror 2.0, serde (Serialize + Deserialize)

**回归验证:** 每个 Task 完成后运行:
```bash
cargo test
```

---

## Task 1: ALTER GROUP dispatch 修复 [P0-1] — 极简

**背景:** `AlterGroupStatement` 和 `AlterGroupAction`（AddUser/DropUser）AST 已完整定义（ast/mod.rs:1505-1514），Formatter 的 `format_alter_group` 也已存在。唯一缺失的是 `dispatch_alter()` 中没有 `GROUP_P` 分支。Parser 的 `parse_alter_group` 方法也已存在。

**Files:**
- Modify: `src/parser/mod.rs` — `dispatch_alter()` 方法中添加 GROUP_P 分支
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

在 `src/parser/tests.rs` 末尾添加:

```rust
// ========== Task 1: ALTER GROUP ==========

#[test]
fn test_alter_group_add_user() {
    let stmt = parse_one("ALTER GROUP my_group ADD USER user1, user2");
    match stmt {
        Statement::AlterGroup(ag) => {
            assert_eq!(ag.name, "my_group");
            assert!(matches!(ag.action, AlterGroupAction::AddUser(ref u) if u == "user1, user2" || u == "user1"));
        }
        _ => panic!("expected AlterGroup, got {:?}", stmt),
    }
}

#[test]
fn test_alter_group_drop_user() {
    let stmt = parse_one("ALTER GROUP my_group DROP USER user1");
    match stmt {
        Statement::AlterGroup(ag) => {
            assert_eq!(ag.name, "my_group");
        }
        _ => panic!("expected AlterGroup, got {:?}", stmt),
    }
}
```

**Step 2: 确认测试失败**

Run: `cargo test test_alter_group`
Expected: 解析返回 Empty（因为 dispatch_alter 走到了 skip_to_semicolon）

**Step 3: 修改 dispatch_alter**

在 `src/parser/mod.rs` 的 `dispatch_alter()` 方法中（约 line 1941），在 `Some(Keyword::SYSTEM_P)` 分支之前，添加:

```rust
Some(Keyword::GROUP_P) => {
    self.advance();
    match self.parse_alter_group() {
        Ok(stmt) => {
            self.try_consume_semicolon();
            crate::ast::Statement::AlterGroup(stmt)
        }
        Err(e) => {
            self.add_error(e);
            self.skip_to_semicolon()
        }
    }
}
```

但需要先检查 `parse_alter_group` 方法是否已存在。如果不存在，需要在 `utility.rs` 中添加（与 `parse_alter_role` 模式一致）:

```rust
pub(crate) fn parse_alter_group(&mut self) -> Result<AlterGroupStatement, ParserError> {
    self.expect_keyword(Keyword::GROUP_P)?;
    let name = self.parse_identifier()?;
    let action = if self.match_keyword(Keyword::ADD_P) {
        self.advance();
        self.expect_keyword(Keyword::USER)?;
        let user = self.parse_identifier()?;
        // consume additional users if comma-separated
        while self.match_token(&Token::Comma) {
            self.advance();
            self.parse_identifier()?;
        }
        AlterGroupAction::AddUser(user)
    } else if self.match_keyword(Keyword::DROP) {
        self.advance();
        self.expect_keyword(Keyword::USER)?;
        let user = self.parse_identifier()?;
        while self.match_token(&Token::Comma) {
            self.advance();
            self.parse_identifier()?;
        }
        AlterGroupAction::DropUser(user)
    } else {
        return Err(ParserError::UnexpectedToken {
            location: self.current_location(),
            expected: "ADD USER or DROP USER".to_string(),
            got: format!("{:?}", self.peek()),
        });
    };
    Ok(AlterGroupStatement { name, action })
}
```

**Step 4: 运行测试**

Run: `cargo test test_alter_group`
Expected: PASS

**Step 5: 回归测试**

Run: `cargo test`
Expected: 298+ passed; 0 failed

**Step 6: Commit**

```bash
git add -A && git commit -m "feat: wire ALTER GROUP into dispatch_alter"
```

---

## Task 2: CREATE TABLE AS (CTAS) [P0-2]

**背景:** GaussDB 支持 `CREATE TABLE new_table AS SELECT ...` 语法。当前解析器不支持此语法。

**Files:**
- Modify: `src/ast/mod.rs` — 新增 `CreateTableAsStatement`，在 `Statement` 枚举中添加变体
- Modify: `src/parser/mod.rs` — dispatch_create 中添加 CTAS 分支
- Modify: `src/parser/ddl.rs` — 添加 `parse_create_table_as` 方法
- Modify: `src/formatter.rs` — 添加 `format_create_table_as`
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
// ========== Task 2: CREATE TABLE AS ==========

#[test]
fn test_create_table_as() {
    let stmt = parse_one("CREATE TABLE new_users AS SELECT id, name FROM users WHERE active = true");
    match stmt {
        Statement::CreateTableAs(ctas) => {
            assert_eq!(ctas.name, vec!["new_users"]);
            assert!(ctas.query.targets.len() >= 2);
        }
        _ => panic!("expected CreateTableAs, got {:?}", stmt),
    }
}

#[test]
fn test_create_table_as_with_if_not_exists() {
    let stmt = parse_one("CREATE TABLE IF NOT EXISTS backup AS SELECT * FROM source");
    match stmt {
        Statement::CreateTableAs(ctas) => {
            assert!(ctas.if_not_exists);
        }
        _ => panic!("expected CreateTableAs, got {:?}", stmt),
    }
}

#[test]
fn test_create_table_as_with_no_data() {
    let stmt = parse_one("CREATE TABLE template AS SELECT * FROM source WITH NO DATA");
    match stmt {
        Statement::CreateTableAs(ctas) => {
            assert!(!ctas.with_data);
        }
        _ => panic!("expected CreateTableAs, got {:?}", stmt),
    }
}

#[test]
fn test_create_table_as_temporary() {
    let stmt = parse_one("CREATE TEMP TABLE temp_result AS SELECT id FROM src");
    match stmt {
        Statement::CreateTableAs(ctas) => {
            assert!(ctas.temporary);
        }
        _ => panic!("expected CreateTableAs, got {:?}", stmt),
    }
}
```

**Step 2: 确认失败**

Run: `cargo test test_create_table_as`
Expected: 编译失败（Statement::CreateTableAs 不存在）

**Step 3: AST 变更 — `src/ast/mod.rs`**

在 `CreateTableStatement` 之后添加:

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTableAsStatement {
    pub temporary: bool,
    pub unlogged: bool,
    pub if_not_exists: bool,
    pub name: ObjectName,
    pub column_names: Vec<String>,
    pub query: Box<SelectStatement>,
    pub with_data: bool,
}
```

在 `Statement` 枚举中添加:
```rust
CreateTableAs(CreateTableAsStatement),
```

**Step 4: Parser 变更 — `src/parser/ddl.rs`**

添加 `parse_create_table_as` 方法:

```rust
pub(crate) fn parse_create_table_as(
    &mut self,
    temporary: bool,
    unlogged: bool,
) -> Result<CreateTableAsStatement, ParserError> {
    self.expect_keyword(Keyword::TABLE)?;
    let if_not_exists = self.parse_if_not_exists();
    let name = self.parse_object_name()?;

    let column_names = if self.match_token(&Token::LParen) {
        self.advance();
        let mut cols = vec![self.parse_identifier()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            cols.push(self.parse_identifier()?);
        }
        self.expect_token(&Token::RParen)?;
        cols
    } else {
        Vec::new()
    };

    self.expect_keyword(Keyword::AS)?;
    let query = Box::new(self.parse_select_statement()?);

    let with_data = if self.match_keyword(Keyword::WITH) {
        self.advance();
        if self.match_keyword(Keyword::NO) {
            self.advance();
            self.expect_keyword(Keyword::DATA_P)?;
            false
        } else {
            self.expect_keyword(Keyword::DATA_P)?;
            true
        }
    } else {
        true // default
    };

    Ok(CreateTableAsStatement {
        temporary,
        unlogged,
        if_not_exists,
        name,
        column_names,
        query,
        with_data,
    })
}
```

**Step 5: dispatch 变更 — `src/parser/mod.rs`**

在 `dispatch_create` 中，`Some(Keyword::TABLE)` 分支需要检查 AS 关键字:

需要修改现有的 TABLE dispatch。在解析 CREATE TABLE 之前，需要先 lookahead 判断是 CREATE TABLE 还是 CREATE TABLE AS:

```rust
Some(Keyword::TABLE) => {
    // Lookahead to check for CREATE TABLE AS
    let is_ctas = {
        let mut depth = 0usize;
        let mut look = self.pos + 1;
        // Skip past IF NOT EXISTS and table name tokens to find AS
        while look < self.tokens.len() {
            match &self.tokens[look].token {
                Token::LParen => depth += 1,
                Token::RParen => depth = depth.saturating_sub(1),
                Token::Keyword(Keyword::AS) if depth == 0 => break true,
                Token::Semicolon | Token::Eof => break false,
                Token::Keyword(Keyword::PARTITION)
                | Token::Keyword(Keyword::DISTRIBUTE)
                | Token::Keyword(Keyword::WITH)
                | Token::Keyword(Keyword::INHERITS)
                | Token::Keyword(Keyword::TABLESPACE)
                | Token::Keyword(Keyword::ON)
                | Token::Keyword(Keyword::TO) => break false,
                _ => {}
            }
            look += 1;
            // Safety: don't look too far
            if look > self.pos + 50 { break false; }
        }
    };

    if is_ctas {
        match self.parse_create_table_as(temp, unlogged) {
            Ok(stmt) => crate::ast::Statement::CreateTableAs(stmt),
            Err(e) => { self.add_error(e); self.skip_to_semicolon() }
        }
    } else {
        match self.parse_create_table(temp, unlogged) {
            Ok(stmt) => crate::ast::Statement::CreateTable(stmt),
            Err(e) => { self.add_error(e); self.skip_to_semicolon() }
        }
    }
}
```

**注意:** 上面的 lookahead 逻辑可能有 edge case。更简单的方案是：先尝试正常 parse_create_table，如果遇到 AS 而不是 LParen（列定义开始），则回退使用 parse_create_table_as。但这需要 parser 支持 backtracking。

**推荐的简化方案:** 在 `parse_create_table` 内部检测 CTAS。当遇到 TABLE 之后、消耗了表名之后，检查下一个 token 是 AS 还是 LParen:

在 `parse_create_table` 的 `self.expect_token(&Token::LParen)?` 之前，增加 CTAS 分支:

```rust
// After parse_object_name(), before expecting LParen:
if self.match_keyword(Keyword::AS) {
    // This is CREATE TABLE ... AS SELECT ...
    self.advance();
    let query = Box::new(self.parse_select_statement()?);
    let with_data = ...; // parse WITH DATA / WITH NO DATA
    return Ok(CreateTableAsStatement { ... });
}
```

但这会改变 return type。所以最好的方式还是在 dispatch_create 中处理。

**最终方案:** 修改 dispatch_create 中的 TABLE 分支，在 `self.expect_keyword(Keyword::TABLE)` 之后，使用 parse_create_table 新增的 CTAS 检测:

在 `src/parser/ddl.rs` 中新增 `parse_create_table_or_as`:

```rust
pub(crate) fn parse_create_table_or_as(
    &mut self,
    temporary: bool,
    unlogged: bool,
) -> Result<crate::ast::Statement, ParserError> {
    self.expect_keyword(Keyword::TABLE)?;
    let if_not_exists = self.parse_if_not_exists();
    let name = self.parse_object_name()?;

    // Check for CREATE TABLE AS
    if self.match_keyword(Keyword::AS) {
        self.advance();
        let query = Box::new(self.parse_select_statement()?);
        let with_data = if self.match_keyword(Keyword::WITH) {
            self.advance();
            if self.match_keyword(Keyword::NO) {
                self.advance();
                self.expect_keyword(Keyword::DATA_P)?;
                false
            } else {
                self.expect_keyword(Keyword::DATA_P)?;
                true
            }
        } else {
            true
        };
        return Ok(crate::ast::Statement::CreateTableAs(CreateTableAsStatement {
            temporary,
            unlogged,
            if_not_exists,
            name,
            column_names: Vec::new(),
            query,
            with_data,
        }));
    }

    // Regular CREATE TABLE - parse column definitions
    // (copy rest of original parse_create_table here, or refactor to share code)
    // ... 但这意味着大量代码重复。

    // Better: keep original parse_create_table intact, and only handle CTAS
    // by peeking before calling parse_create_table
    unreachable!() // handled by caller
}
```

**最终最优方案:** 在 `dispatch_create` 中，对 `Keyword::TABLE` 分支，先消耗 `TABLE` + `IF NOT EXISTS` + 表名，然后检查下一个 token:
- 如果是 `AS` → CTAS 路径
- 如果是 `(` → 普通 CREATE TABLE 路径

这需要对 `parse_create_table` 做小重构：将 `TABLE` 关键字消费和 `IF NOT EXISTS`、表名的解析提取到 dispatch 层。

**实际执行时:** 采用最小改动方案 — 在 dispatch_create 中 TABLE 分支，使用 lookahead 检查表名后面是否跟 AS。这可以通过扫描 tokens 来实现:

```rust
Some(Keyword::TABLE) => {
    // Use parse_create_table which now detects CTAS internally
    match self.parse_create_table_or_as(temp, unlogged) {
        Ok(stmt) => stmt,
        Err(e) => { self.add_error(e); self.skip_to_semicolon() }
    }
}
```

`parse_create_table_or_as` 在 `ddl.rs` 中实现：先消费 TABLE/IF NOT EXISTS/表名，然后根据下一个 token 分支到 CTAS 或普通 CT。

**Step 6: Formatter 变更 — `src/formatter.rs`**

```rust
Statement::CreateTableAs(s) => self.format_create_table_as(s),
```

```rust
fn format_create_table_as(&self, stmt: &CreateTableAsStatement) -> String {
    let mut s = self.kw("CREATE").to_string();
    if stmt.temporary { s.push(' '); s.push_str(&self.kw("TEMPORARY")); }
    if stmt.unlogged { s.push(' '); s.push_str(&self.kw("UNLOGGED")); }
    s.push(' '); s.push_str(&self.kw("TABLE"));
    if stmt.if_not_exists { s.push(' '); s.push_str(&self.kw("IF NOT EXISTS")); }
    s.push(' '); s.push_str(&self.format_object_name(&stmt.name));
    if !stmt.column_names.is_empty() {
        s.push_str(&format!(" ({})", stmt.column_names.join(", ")));
    }
    s.push(' '); s.push_str(&self.kw("AS"));
    s.push(' '); s.push_str(&self.format_select(&stmt.query));
    if !stmt.with_data {
        s.push(' '); s.push_str(&self.kw("WITH NO DATA"));
    }
    s
}
```

**Step 7: 运行测试 + 回归**

Run: `cargo test`
Expected: 全部通过

**Step 8: Commit**

```bash
git add -A && git commit -m "feat: CREATE TABLE AS (CTAS) syntax support"
```

---

## Task 3: CREATE AGGREGATE [P0-3]

**背景:** GaussDB 支持 `CREATE AGGREGATE name (input_type) (SFUNC = func, STYPE = stype, ...)` 语法。

**Files:**
- Modify: `src/ast/mod.rs` — 新增 `CreateAggregateStatement`
- Modify: `src/parser/mod.rs` — dispatch_create 添加 AGGREGATE 分支
- Modify: `src/parser/utility.rs` — 添加 `parse_create_aggregate`
- Modify: `src/formatter.rs` — 添加 formatter
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
// ========== Task 3: CREATE AGGREGATE ==========

#[test]
fn test_create_aggregate_basic() {
    let stmt = parse_one("CREATE AGGREGATE my_sum (INTEGER) (SFUNC = sum_func, STYPE = BIGINT)");
    match stmt {
        Statement::CreateAggregate(ca) => {
            assert_eq!(ca.name, "my_sum");
            assert!(ca.options.len() >= 2);
        }
        _ => panic!("expected CreateAggregate, got {:?}", stmt),
    }
}
```

**Step 2: 确认失败**

**Step 3: AST**

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateAggregateStatement {
    pub name: String,
    pub base_types: Vec<DataType>,
    pub options: Vec<(String, String)>,
}
```

Statement 枚举新增: `CreateAggregate(CreateAggregateStatement)`

**Step 4: Parser**

```rust
pub(crate) fn parse_create_aggregate(&mut self) -> Result<CreateAggregateStatement, ParserError> {
    self.expect_keyword(Keyword::AGGREGATE)?;
    let name = self.parse_identifier()?;

    let base_types = if self.match_token(&Token::LParen) {
        self.advance();
        let mut types = vec![self.parse_data_type()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            types.push(self.parse_data_type()?);
        }
        self.expect_token(&Token::RParen)?;
        types
    } else {
        Vec::new()
    };

    let options = self.parse_generic_options();

    Ok(CreateAggregateStatement { name, base_types, options })
}
```

**Step 5: dispatch_create 添加**

```rust
Some(Keyword::AGGREGATE) => {
    self.advance();
    match self.parse_create_aggregate() {
        Ok(stmt) => { self.try_consume_semicolon(); Statement::CreateAggregate(stmt) }
        Err(e) => { self.add_error(e); self.skip_to_semicolon() }
    }
}
```

**Step 6: Formatter**

```rust
fn format_create_aggregate(&self, stmt: &CreateAggregateStatement) -> String {
    let mut s = format!("{} {}", self.kw("CREATE"), self.kw("AGGREGATE"));
    s.push_str(&format!(" {}", stmt.name));
    if !stmt.base_types.is_empty() {
        s.push('(');
        s.push_str(&stmt.base_types.iter().map(|t| format_data_type(t)).collect::<Vec<_>>().join(", "));
        s.push(')');
    }
    if !stmt.options.is_empty() {
        s.push(' ');
        s.push_str(&self.format_generic_options(&stmt.options));
    }
    s
}
```

**Step 7: 测试 + 回归 + Commit**

---

## Task 4: CREATE OPERATOR [P0-4]

**背景:** GaussDB 支持 `CREATE OPERATOR name (PROCEDURE = func, LEFTARG = type, RIGHTARG = type, ...)` 语法。

**Files:**
- Modify: `src/ast/mod.rs` — 新增 `CreateOperatorStatement`
- Modify: `src/parser/mod.rs` — dispatch_create 添加 OPERATOR 分支
- Modify: `src/parser/utility.rs` — 添加 `parse_create_operator`
- Modify: `src/formatter.rs` — 添加 formatter
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
#[test]
fn test_create_operator() {
    let stmt = parse_one("CREATE OPERATOR === (PROCEDURE = my_eq, LEFTARG = int, RIGHTARG = int)");
    match stmt {
        Statement::CreateOperator(co) => {
            assert!(co.name.contains("===") || co.name == "===");
            assert!(co.options.len() >= 3);
        }
        _ => panic!("expected CreateOperator, got {:?}", stmt),
    }
}
```

**Step 2: 确认失败**

**Step 3: AST**

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateOperatorStatement {
    pub name: String,
    pub options: Vec<(String, String)>,
}
```

Statement 枚举新增: `CreateOperator(CreateOperatorStatement)`

**Step 4: Parser**

```rust
pub(crate) fn parse_create_operator(&mut self) -> Result<CreateOperatorStatement, ParserError> {
    self.expect_keyword(Keyword::OPERATOR)?;
    let name = self.parse_identifier()?;
    let options = self.parse_generic_options();
    Ok(CreateOperatorStatement { name, options })
}
```

注意：operator name 可能是特殊符号如 `===`、`@@` 等。需要确保 `parse_identifier` 或替代方案能处理这些。查看 tokenizer 是否将这类符号作为 Ident 处理。如果不行，可能需要使用 `self.peek().clone()` 直接获取 token 文本。

**Step 5-7: 同上模式**

---

## Task 5: ALTER DEFAULT PRIVILEGES [P0-5]

**背景:** GaussDB 支持 `ALTER DEFAULT PRIVILEGES FOR ROLE role_name IN SCHEMA schema_name GRANT/REVOKE ...` 语法。

**Files:**
- Modify: `src/ast/mod.rs` — 新增 `AlterDefaultPrivilegesStatement`
- Modify: `src/parser/mod.rs` — dispatch ALTER DEFAULT 分支
- Modify: `src/parser/utility.rs` — 添加 `parse_alter_default_privileges`
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
// ========== Task 5: ALTER DEFAULT PRIVILEGES ==========

#[test]
fn test_alter_default_privileges_grant() {
    let stmt = parse_one("ALTER DEFAULT PRIVILEGES FOR ROLE admin IN SCHEMA public GRANT SELECT ON TABLES TO user1");
    match stmt {
        Statement::AlterDefaultPrivileges(adp) => {
            assert_eq!(adp.role.as_deref(), Some("admin"));
            assert_eq!(adp.schema.as_deref(), Some("public"));
            assert!(matches!(adp.action, DefaultPrivilegeAction::Grant(_)));
        }
        _ => panic!("expected AlterDefaultPrivileges, got {:?}", stmt),
    }
}

#[test]
fn test_alter_default_privileges_revoke() {
    let stmt = parse_one("ALTER DEFAULT PRIVILEGES IN SCHEMA hr REVOKE INSERT ON TABLES FROM public");
    match stmt {
        Statement::AlterDefaultPrivileges(adp) => {
            assert!(adp.role.is_none());
            assert!(matches!(adp.action, DefaultPrivilegeAction::Revoke(_)));
        }
        _ => panic!("expected AlterDefaultPrivileges, got {:?}", stmt),
    }
}
```

**Step 2: 确认失败**

**Step 3: AST**

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterDefaultPrivilegesStatement {
    pub role: Option<String>,
    pub schema: Option<String>,
    pub action: DefaultPrivilegeAction,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DefaultPrivilegeAction {
    Grant(GrantStatement),
    Revoke(RevokeStatement),
}
```

Statement 枚举新增: `AlterDefaultPrivileges(AlterDefaultPrivilegesStatement)`

**Step 4: Parser**

在 `dispatch_alter` 中检测 `DEFAULT` 关键字:

```rust
// 在 dispatch_alter 中，match self.peek_keyword() 之前
// 需要特殊处理 ALTER DEFAULT PRIVILEGES
// 因为 DEFAULT 不是后面跟一个对象类型，而是跟 PRIVILEGES

// 在 dispatch_alter 最前面添加:
if self.match_keyword(Keyword::DEFAULT) {
    // ALTER DEFAULT PRIVILEGES
    self.advance();
    match self.parse_alter_default_privileges() {
        Ok(stmt) => { self.try_consume_semicolon(); Statement::AlterDefaultPrivileges(stmt) }
        Err(e) => { self.add_error(e); self.skip_to_semicolon() }
    }
} else {
    // existing match on peek_keyword
}
```

注意这需要重构 dispatch_alter 的控制流 — 将现有 match 包在 else 分支中。

`parse_alter_default_privileges` 实现:

```rust
pub(crate) fn parse_alter_default_privileges(&mut self) -> Result<AlterDefaultPrivilegesStatement, ParserError> {
    self.expect_keyword(Keyword::PRIVILEGES)?;

    let mut role = None;
    let mut schema = None;

    if self.try_consume_keyword(Keyword::FOR) {
        self.try_consume_keyword(Keyword::ROLE);
        role = Some(self.parse_identifier()?);
    }

    if self.try_consume_keyword(Keyword::IN_P) {
        self.try_consume_keyword(Keyword::SCHEMA);
        schema = Some(self.parse_identifier()?);
    }

    let action = if self.match_keyword(Keyword::GRANT) {
        self.advance();
        let grant = self.parse_grant()?;
        DefaultPrivilegeAction::Grant(grant)
    } else if self.match_keyword(Keyword::REVOKE) {
        self.advance();
        let revoke = self.parse_revoke()?;
        DefaultPrivilegeAction::Revoke(revoke)
    } else {
        return Err(ParserError::UnexpectedToken {
            location: self.current_location(),
            expected: "GRANT or REVOKE".to_string(),
            got: format!("{:?}", self.peek()),
        });
    };

    Ok(AlterDefaultPrivilegesStatement { role, schema, action })
}
```

**Step 5-7: 同上模式**

---

## Task 6: CREATE USER MAPPING / ALTER / DROP [P0-6]

**背景:** GaussDB 外部数据访问需要 USER MAPPING: `CREATE USER MAPPING FOR user_name SERVER server_name OPTIONS (...)`.

**Files:**
- Modify: `src/ast/mod.rs` — 新增 `CreateUserMappingStatement`, `AlterUserMappingStatement`, `DropUserMappingStatement`
- Modify: `src/parser/mod.rs` — dispatch 中添加 USER MAPPING 分支
- Modify: `src/parser/utility.rs` — 添加解析方法
- Modify: `src/parser/ddl.rs` — DROP 中添加 USER MAPPING 对象类型
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
// ========== Task 6: USER MAPPING ==========

#[test]
fn test_create_user_mapping() {
    let stmt = parse_one("CREATE USER MAPPING FOR current_user SERVER my_server OPTIONS (user 'remote_user', password 'secret')");
    match stmt {
        Statement::CreateUserMapping(cum) => {
            assert!(cum.if_not_exists == false);
            assert!(cum.server.len() >= 1);
        }
        _ => panic!("expected CreateUserMapping, got {:?}", stmt),
    }
}

#[test]
fn test_alter_user_mapping() {
    let stmt = parse_one("ALTER USER MAPPING FOR current_user SERVER my_server OPTIONS (SET password 'new_secret')");
    match stmt {
        Statement::AlterUserMapping(aum) => {
            assert!(aum.server.len() >= 1);
        }
        _ => panic!("expected AlterUserMapping, got {:?}", stmt),
    }
}

#[test]
fn test_drop_user_mapping() {
    let stmt = parse_one("DROP USER MAPPING IF EXISTS FOR current_user SERVER my_server");
    match stmt {
        Statement::DropUserMapping(dum) => {
            assert!(dum.if_exists);
        }
        _ => panic!("expected DropUserMapping, got {:?}", stmt),
    }
}
```

**Step 2: 确认失败**

**Step 3: AST**

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateUserMappingStatement {
    pub if_not_exists: bool,
    pub user_name: String,      // user/role name or "CURRENT_USER"/"PUBLIC"
    pub server: ObjectName,
    pub options: Vec<(String, Option<String>)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterUserMappingStatement {
    pub user_name: String,
    pub server: ObjectName,
    pub options: Vec<(String, Option<String>)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DropUserMappingStatement {
    pub if_exists: bool,
    pub user_name: String,
    pub server: ObjectName,
}
```

Statement 枚举新增:
```rust
CreateUserMapping(CreateUserMappingStatement),
AlterUserMapping(AlterUserMappingStatement),
DropUserMapping(DropUserMappingStatement),
```

**Step 4: Parser**

在 `dispatch_create` 中检测 USER 后面跟 MAPPING:

```rust
Some(Keyword::USER) => {
    self.advance();
    // Check for USER MAPPING
    if self.match_keyword(Keyword::MAPPING) {
        self.advance();
        match self.parse_create_user_mapping() {
            Ok(stmt) => { self.try_consume_semicolon(); Statement::CreateUserMapping(stmt) }
            Err(e) => { self.add_error(e); self.skip_to_semicolon() }
        }
    } else {
        // existing CREATE USER logic
        match self.parse_create_role_options() { ... }
    }
}
```

同理 dispatch_alter 中也添加 USER MAPPING 分支。

DROP 中在 ObjectType 枚举中新增 `UserMapping`，在 parse_drop 中添加:

```rust
// 在 parse_drop 中
Some(Keyword::USER) => {
    self.advance();
    if self.match_keyword(Keyword::MAPPING) {
        self.advance();
        ObjectType::UserMapping
    } else {
        ObjectType::User  // if DROP USER exists
    }
}
```

parse_create_user_mapping:

```rust
pub(crate) fn parse_create_user_mapping(&mut self) -> Result<CreateUserMappingStatement, ParserError> {
    let if_not_exists = self.parse_if_not_exists();
    self.expect_keyword(Keyword::FOR)?;
    let user_name = self.parse_identifier()?;
    self.expect_keyword(Keyword::SERVER)?;
    let server = self.parse_object_name()?;
    let options = self.parse_generic_options();
    Ok(CreateUserMappingStatement { if_not_exists, user_name, server, options })
}
```

**Step 5-7: 同上模式**

---

## Task 7: DROP 对象类型扩展 [P1-7]

**背景:** 当前 `parse_drop()` 仅支持 16 种 ObjectType，缺少 GaussDB 常用的 AGGREGATE、CAST、OPERATOR、OPCLASS、OPFAMILY、CONVERSION、LANGUAGE、TEXT SEARCH CONFIG/DICT、RULE、POLICY、USER MAPPING、DOMAIN 等。

**Files:**
- Modify: `src/ast/mod.rs` — `ObjectType` 枚举新增变体
- Modify: `src/parser/ddl.rs` — `parse_drop()` 新增分支
- Modify: `src/formatter.rs` — `format_drop` 适配
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
// ========== Task 7: DROP 扩展 ==========

#[test]
fn test_drop_aggregate() {
    let stmt = parse_one("DROP AGGREGATE IF EXISTS my_sum(INTEGER)");
    match stmt {
        Statement::Drop(d) => {
            assert!(matches!(d.object_type, ObjectType::Aggregate));
            assert!(d.if_exists);
        }
        _ => panic!("expected Drop, got {:?}", stmt),
    }
}

#[test]
fn test_drop_operator() {
    let stmt = parse_one("DROP OPERATOR === (int, int)");
    match stmt {
        Statement::Drop(d) => {
            assert!(matches!(d.object_type, ObjectType::Operator));
        }
        _ => panic!("expected Drop, got {:?}", stmt),
    }
}

#[test]
fn test_drop_cast() {
    let stmt = parse_one("DROP CAST (int AS bigint)");
    match stmt {
        Statement::Drop(d) => {
            assert!(matches!(d.object_type, ObjectType::Cast));
        }
        _ => panic!("expected Drop, got {:?}", stmt),
    }
}

#[test]
fn test_drop_conversion() {
    let stmt = parse_one("DROP CONVERSION my_conv");
    match stmt {
        Statement::Drop(d) => {
            assert!(matches!(d.object_type, ObjectType::Conversion));
        }
        _ => panic!("expected Drop, got {:?}", stmt),
    }
}

#[test]
fn test_drop_operator_class() {
    let stmt = parse_one("DROP OPERATOR CLASS my_opclass USING btree");
    match stmt {
        Statement::Drop(d) => {
            assert!(matches!(d.object_type, ObjectType::OperatorClass));
        }
        _ => panic!("expected Drop, got {:?}", stmt),
    }
}

#[test]
fn test_drop_rule() {
    let stmt = parse_one("DROP RULE my_rule ON my_table");
    match stmt {
        Statement::Drop(d) => {
            assert!(matches!(d.object_type, ObjectType::Rule));
        }
        _ => panic!("expected Drop, got {:?}", stmt),
    }
}
```

**Step 2: 确认失败**

**Step 3: AST — ObjectType 新增**

```rust
pub enum ObjectType {
    // existing...
    Aggregate,
    Cast,
    Conversion,
    Operator,
    OperatorClass,
    OperatorFamily,
    Rule,
    Language,
    TextSearchConfig,
    TextSearchDict,
    UserMapping,
    Domain,
    Policy,
}
```

**Step 4: Parser — parse_drop 新增分支**

```rust
Some(Keyword::AGGREGATE) => { self.advance(); ObjectType::Aggregate }
Some(Keyword::CAST) => { self.advance(); ObjectType::Cast }
Some(Keyword::CONVERSION) => { self.advance(); ObjectType::Conversion }
Some(Keyword::OPERATOR) => {
    self.advance();
    if self.match_keyword(Keyword::CLASS) {
        self.advance();
        ObjectType::OperatorClass
    } else if self.match_keyword(Keyword::FAMILY) {
        self.advance();
        ObjectType::OperatorFamily
    } else {
        ObjectType::Operator
    }
}
Some(Keyword::RULE) => { self.advance(); ObjectType::Rule }
Some(Keyword::LANGUAGE) => { self.advance(); ObjectType::Language }
Some(Keyword::DOMAIN_P) => { self.advance(); ObjectType::Domain }
Some(Keyword::POLICY) => { self.advance(); ObjectType::Policy }
```

注意 DROP AGGREGATE/OPERATOR/CAST 后面有参数部分如 `(INTEGER)` 或 `(int, int)` 或 `(int AS bigint)`。在 `parse_drop_statement_with_type` 中需要跳过这些参数:

```rust
// After parsing names, for certain object types, skip parenthesized arguments
if matches!(object_type, ObjectType::Aggregate | ObjectType::Operator | ObjectType::Cast) {
    if self.match_token(&Token::LParen) {
        self.advance();
        let _depth = self.skip_balanced_parens_content();
    }
}
// For OPERATOR CLASS/FAMILY, skip USING keyword
if matches!(object_type, ObjectType::OperatorClass | ObjectType::OperatorFamily) {
    if self.match_keyword(Keyword::USING) {
        self.advance();
        let _ = self.parse_identifier();
    }
}
// For RULE, skip ON table_name
if matches!(object_type, ObjectType::Rule) {
    if self.match_keyword(Keyword::ON) {
        self.advance();
        let _ = self.parse_object_name();
    }
}
```

**Step 5: Formatter 适配**

在 format_drop 中添加新的 object_type 字符串映射。

**Step 6-7: 测试 + 回归 + Commit**

---

## 执行顺序

| 顺序 | Task | 复杂度 | 新增测试 | 依赖 |
|------|------|--------|---------|------|
| 1 | Task 1: ALTER GROUP dispatch | 极低 | 2 | 无 |
| 2 | Task 3: CREATE AGGREGATE | 低 | 1-2 | 无 |
| 3 | Task 4: CREATE OPERATOR | 低 | 1-2 | 无 |
| 4 | Task 5: ALTER DEFAULT PRIVILEGES | 中 | 2-3 | 无 |
| 5 | Task 6: USER MAPPING (CREATE/ALTER/DROP) | 中 | 3 | 无 |
| 6 | Task 7: DROP 对象类型扩展 | 中 | 6+ | 无 |
| 7 | Task 2: CREATE TABLE AS | 高 | 4 | 无 |

**并行化:** Task 1/3/4 完全独立可以并行。Task 5/6 独立可以并行。Task 7 独立。Task 2 最复杂建议放最后。

**每个 Task 完成后验证:**
```bash
cargo test
```
