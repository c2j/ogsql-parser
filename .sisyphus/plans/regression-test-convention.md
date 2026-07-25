# Regression Test Convention（回归守护案例规范）

## 1. 目标

每次 bug 修复或新特性建设，必须创建对应的回归守护案例，确保该场景在未来不会被回退。

## 2. 目录结构

```
tests/
├── regress/                          # 回归 fixture 文件（文件名 = 类型_编号_模块）
│   ├── issue_246_linter.sql          # SQL fixture: issue_NNN_module.ext
│   ├── issue_260_parser.sql
│   ├── issue_300_ibatis.xml          # XML fixture
│   ├── issue_301_java.java           # Java fixture
│   ├── issue_302_ibatis.xml          # 多文件场景: 同名不同扩展名
│   ├── issue_302_ibatis.java         #             关联文件组
│   ├── feat_create_model_parser.sql  # 新特性: feat_name_module.ext
│   └── feat_merge_and_parser.sql
├── common/
│   └── mod.rs                        # RegressFixture 加载器
├── regression_parser.rs              # 解析器回归测试入口
├── regression_linter.rs              # Linter 回归测试入口
├── regression_tokenizer.rs           # Tokenizer 回归测试入口
├── regression_formatter.rs           # Formatter 回归测试入口
├── regression_analyzer.rs            # Analyzer 回归测试入口
├── regression_ibatis.rs              # iBatis XML 回归测试入口
├── regression_java.rs                # Java SQL 提取回归测试入口
├── roundtrip.rs                      # 现有，不动
├── plpgsql.rs                        # 现有，不动
├── multi_statement.rs                # 现有，不动
└── error_handling.rs                 # 现有，不动
```

设计原则：
- fixture 文件与 Rust 测试代码物理分离
- **文件名编码模块归属**（`{type}_{id}_{module}.{ext}`），无需 `Module:` 元数据字段
- fixture 按文件名扁平存放，扩展名区分输入类型
- Rust 测试入口按模块拆分（`regression_{module}.rs`），每个文件只加载自己模块的 fixture
- 现有测试文件不受影响
- 新 PL/pgSQL 修复也走此体系（`tests/regress/` + `tests/regression_parser.rs`），`src/parser/tests_plsql_fixes.rs` 仅维护历史案例

## 3. Fixture 文件格式

### 3.1 通用结构

每个 fixture 文件 = **元数据头部注释** + **文件体**。

元数据字段（3 个必填）：

| 字段 | 含义 | 示例 |
|------|------|------|
| `Issue:` | Issue 编号或 Feature 名 | `#246` / `feat_create_model` |
| `Description:` | 一句话描述问题 | `Double-iteration causes N×M warning duplication` |
| `Expect:` | 期望结果（成功 / 失败 + 简要描述） | `parse success` / `parse error: unexpected token` / `linter S006 count=2` |

模块归属由文件名确定，不再在元数据中重复声明。

### 3.2 SQL fixture（`.sql`）

注释语法：`-- Key: Value`

**成功场景**：
```sql
-- Issue: #246
-- Description: Double-iteration causes N×M warning duplication
-- Expect: linter S006 fires 2 times (not 5×2 = 10)
SELECT * FROM t1 LIMIT 5;
SELECT id FROM t2 WHERE id = 1;
SELECT * FROM t3 LIMIT 10;
SELECT id FROM t4 ORDER BY id;
SELECT id FROM t5 ORDER BY id LIMIT 1;
```

**错误场景**：
```sql
-- Issue: #400
-- Description: Unterminated dollar-quote in PL block causes panic
-- Expect: parse error (not panic)
DO $$ BEGIN NULL; END
```

### 3.3 XML fixture（`.xml`）

注释语法：`<!-- Key: Value -->`

```xml
<!-- Issue: #300 -->
<!-- Description: <foreach> inside <where> generates incorrect SQL with extra WHERE -->
<!-- Expect: flat_sql = "SELECT * FROM users WHERE id IN (...)" -->
<?xml version="1.0" encoding="UTF-8"?>
<mapper namespace="com.example.UserMapper">
    <select id="findByIds">
        SELECT * FROM users
        <where>
            <foreach item="id" collection="ids" open="id IN (" close=")" separator=",">
                #{id}
            </foreach>
        </where>
    </select>
</mapper>
```

### 3.4 Java fixture（`.java`）

注释语法：`// Key: Value`

元数据注释必须在 `package` 声明之前（Java 允许文件以注释开头）。

```java
// Issue: #301
// Description: PreparedStatement backfill fails to trace across void methods
// Expect: 1 extraction, sql contains __JAVA_VAR_JDBC_PARAM_1__
public class UserDao {
    public void findUser(String id) {
        PreparedStatement ps = conn.prepareStatement(
            "SELECT * FROM users WHERE id = ?");
    }
}
```

### 3.5 多文件场景

同名不同扩展名 = 关联文件组。`load_all(id, module)` 返回全部匹配文件。

```
tests/regress/
├── issue_302_ibatis.xml    # iBatis mapper
├── issue_302_ibatis.java   # Java DTO（用于参数类型推断）
```

## 4. 公共 Helper（`tests/common/mod.rs`）

```rust
/// 回归 fixture 文件的解析后结构
pub struct RegressFixture {
    pub issue: String,          // "#246" | "feat_create_model"
    pub description: String,    // 一行描述
    pub expect: String,         // 期望结果
    pub content: String,        // 文件体（元数据注释已剥离）
    pub file_type: FileType,    // Sql | Xml | Java
}

pub enum FileType { Sql, Xml, Java }

/// 加载指定 issue 的指定模块的所有 fixture 文件
/// 扫描 tests/regress/{type}_{id}_{module}.* 匹配的所有文件
/// type: "issue" 或 "feat"
/// id: issue 编号（如 246）或 feature 名（如 "create_model"）
/// module: 模块名（如 "parser", "linter"）
pub fn load_all(typ: &str, id: &str, module: &str) -> Vec<RegressFixture>;

/// 便捷函数：加载 issue 的 fixture
pub fn load_issue(id: u32, module: &str) -> Vec<RegressFixture> {
    load_all("issue", &id.to_string(), module)
}

/// 便捷函数：加载 feature 的 fixture
pub fn load_feat(name: &str, module: &str) -> Vec<RegressFixture> {
    load_all("feat", name, module)
}
```

元数据解析：
- 根据 `FileType`（由扩展名推断）选择注释语法：`-- ` / `<!-- -->` / `// `
- 只解析文件开头的连续元数据注释行（`Key: Value` 格式）
- 第一个非注释、非空行之后的内容均为 `content`
- Java 文件：`package`、`import` 声明视为 `content` 的开始，不是元数据的一部分

## 5. Rust 测试入口命名约定

### 5.1 测试函数命名

```
test_<简短问题描述>_issue_<NNN>
test_<简短问题描述>_feat_<name>
```

文件已在 `regression_{module}.rs` 中，无需在函数名中重复 `_regression_`。

示例：

| 场景 | 测试名 |
|------|--------|
| Linter Issue #246 | `test_double_iteration_no_duplicate_issue_246` |
| Parser 修复 | `test_cursor_params_in_function_issue_260` |
| 新特性 CREATE MODEL | `test_create_model_predict_by_feat_create_model` |
| 解析错误场景 | `test_unterminated_dollar_quote_no_panic_issue_400` |

### 5.2 测试函数模板

```rust
// ── Issue #NNN: <一句话描述> ──
#[test]
fn test_xxx_issue_NNN() {
    let fixtures = common::load_issue(NNN, "parser");
    assert!(!fixtures.is_empty(), "回归守护: fixture 文件缺失");

    // 执行操作，断言期望行为
    // 失败消息以 "回归守护:" 开头
    assert!(..., "回归守护: <失败含义>");
}
```

### 5.3 回归测试区段标识

每个 `tests/regression_{module}.rs` 文件内用区段注释包裹：

```rust
// ══════════════════════════════════════════════════════════════════
// Regression guard tests（回归守护案例）
// ══════════════════════════════════════════════════════════════════
// 命名: test_<what>_{issue_NNN|feat_<name>}
// 加载: common::load_{issue|feat}(id, "module")
// ══════════════════════════════════════════════════════════════════
```

## 6. 工作流

### 6.1 Bug 修复流程

```
1. git checkout -b fix/xxx
2. 创建 tests/regress/issue_NNN_{module}.sql（复现 bug 的 SQL）
3. 在 tests/regression_{module}.rs 中编写测试函数
4. cargo test -- regression_{module} → 确认 FAIL（复现成功）
5. 实施修复代码
6. cargo test -- regression_{module} → 确认 PASS（修复有效）
7. 提交（fixture + 测试代码 + 修复代码在同一 commit）
```

### 6.2 新特性建设流程

```
1. git checkout -b feat/xxx
2. 创建 tests/regress/feat_{name}_{module}.sql（期望支持的新语法）
3. 在 tests/regression_{module}.rs 中编写测试函数
4. cargo test → 确认 FAIL（当前不支持）
5. 实施特性代码
6. cargo test → 确认 PASS
7. 补充边界 fixture：feat_{name}_{module}_edge.sql（空输入、非法输入等）
8. 提交
```

### 6.3 测试粒度要求（最小契约）

| 类型 | 最小 fixture 数 | 说明 |
|------|----------------|------|
| Bug 修复 | 1 个复现场景 | 必须能复现旧错误行为 |
| 新特性 | 1 个 happy path + 1 个边界 case | 正常 + 边界覆盖 |

## 7. CI 强制策略

### 7.1 检测规则

PR 中包含对 `src/` 目录的非测试文件变更（即 `src/**/*.rs` 中非 `tests.rs` / `tests_*.rs` 的文件）时，CI 检查是否存在对应的回归测试变更：

1. `tests/regress/` 目录有新增或修改文件，**或**
2. `tests/regression_*.rs` 有新增测试函数

不满足则 **阻断合并**。

### 7.2 实现方式

在 `.github/workflows/ci.yml` 中新增 job：

```yaml
regression-check:
  runs-on: ubuntu-latest
  if: github.event_name == 'pull_request'
  steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0
    - name: Check regression tests
      run: scripts/check-regression-tests.sh
```

`scripts/check-regression-tests.sh` 逻辑：
```bash
#!/bin/bash
set -euo pipefail

BASE="${{ github.event.pull_request.base.sha }}"
HEAD="${{ github.event.pull_request.head.sha }}"

# 1. 检测 src/ 中是否有非测试 .rs 文件变更
SRC_CHANGED=$(git diff --name-only "$BASE" "$HEAD" -- 'src/**/*.rs' | grep -v '/tests\.rs$' | grep -v '/tests_.*\.rs$' || true)

if [ -z "$SRC_CHANGED" ]; then
  echo "No src/ changes detected, skipping regression check."
  exit 0
fi

# 2. 检测 tests/regress/ 或 tests/regression_*.rs 是否有变更
REGRESS_CHANGED=$(git diff --name-only "$BASE" "$HEAD" -- 'tests/regress/' 'tests/regression_*.rs' || true)

if [ -z "$REGRESS_CHANGED" ]; then
  echo "ERROR: src/ files changed but no regression test added/updated."
  echo ""
  echo "Changed src/ files:"
  echo "$SRC_CHANGED"
  echo ""
  echo "Please add a regression fixture under tests/regress/ and/or"
  echo "a test function in tests/regression_{module}.rs."
  echo "See docs: .sisyphus/plans/regression-test-convention.md"
  exit 1
fi

echo "Regression tests found:"
echo "$REGRESS_CHANGED"
```

### 7.3 例外豁免

以下情况 CI 不阻断：
- 变更仅涉及 `Cargo.toml`、`.github/`、`docs/`、`examples/` 等非核心源码
- 纯测试基础设施变更（`tests/common/mod.rs`、测试框架本身）
- 仅注释 / 文档字符串变更（可手动 review 确认）

## 8. 与现有测试的关系

| 维度 | 现有测试 | 回归守护测试 |
|------|---------|------------|
| 目的 | 验证功能正确性 | 防止已知问题回退 |
| 覆盖 | 泛化场景 | 精确打击已知问题点 |
| 生命周期 | 可随重构调整 | 永久保留（除非特性被移除） |
| 位置 | `src/{module}/tests.rs` / `tests/*.rs` | `tests/regress/` + `tests/regression_*.rs` |

## 9. 迁移策略

- 现有测试文件**不动**：`src/parser/tests_plsql_fixes.rs`、`examples/test_issues.rs`、`tests/*.rs` 等保持原样
- **从现在开始**的所有新增 bug 修复和特性建设，按本规范创建回归守护案例
- 新 PL/pgSQL 修复使用 `tests/regress/issue_NNN_parser.sql` + `tests/regression_parser.rs`
- 在 `src/parser/tests_plsql_fixes.rs` 文件头添加注释指向新体系，避免后续贡献者混淆

## 10. 待定事项

- [ ] 实现 `tests/common/mod.rs`（元数据解析、fixture 加载）
- [ ] 实现 `scripts/check-regression-tests.sh`
- [ ] CI workflow 中新增 `regression-check` job
- [ ] 在 `src/parser/tests_plsql_fixes.rs` 文件头添加迁移指引注释
