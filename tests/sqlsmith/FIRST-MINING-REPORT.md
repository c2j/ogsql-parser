# 首次 SQLsmith Mining 结果报告 / First SQLsmith Mining Report

> 日期 / Date: 2026-08-05
> 语料 / Corpus: seed=42, max_queries=5000（冷启动规模，正式可扩到 5×50000）
> 生成命令 / Command: `./bin/corpus-gen.sh` + `cargo run --bin sqlsmith-harness -- mine fixtures/ --out regress/`

---

## 总体统计 / Overall Stats

| 指标 | 值 |
|---|---|
| SQLsmith 生成语句数 | 5000 |
| ogsql-parser 解析 OK | 1663 (33.3%) |
| 解析失败 | 3337 (66.7%) |
| **去重后独立失败案例** | **47** |
| 其中 PG-only 已知可接受 | 12 |
| **真实/可疑问题** | **35** |

> 66.7% 失败率是**预期的**：SQLsmith 生成 PG 方言，与 openGauss 方言存在系统性差异；
> 且复杂嵌套查询（数百行子查询）会触发大量边界 case。失败率高 ≠ 解析器质量差，
> 关键看失败样本是否命中**真实语法**。

---

## 按真实根因分类 / Triage by root cause

### 🔥 已确认的真实 bug（需修 parser，按优先级）

#### P0-A. 表达式操作符位置错误 — 10 个
`#0005 #0008 #0015 #0016 #0021 #0023 #0025 #0028 #0029 #0038`

错误特征：`expected expression, got Op("~*") / OpShiftR / Percent / Lt / Gt / Eq / Ne / Le / Ge`

- 涉及操作符：`< > = <> <= >= % >> ~*`
- 简单形式 `SELECT 1 < 2` 可解析，失败发生在**深层嵌套表达式**（CASE 内、CAST 内、函数参数内）
- 影响面最大：任何真实 SQL 都可能踩中

#### P0-B. 子查询中的 JOIN 关键字 — 8 个
`#0003 #0006 #0007 #0010 #0011 #0012 #0017 #0020`

错误特征：`expected RParen / statement, got Keyword(INNER_P/LEFT/RIGHT/ON)`

- `(SELECT ... INNER JOIN ... ON ...)` 在子查询中使用 JOIN 时解析失败
- 简单形式 `SELECT * FROM (SELECT * FROM a INNER JOIN b ON a.id=b.id) x` 可解析
- 失败发生在**嵌套子查询 + JOIN + 其他复杂元素**组合

#### P1-A. `AT TIME ZONE` 操作符 — 3 个
`#0009 #0032 #0035`

错误特征：`expected then/statement/RParen, got Keyword(AT)`

- **确认真实 bug**：`SELECT x AT TIME ZONE x FROM t` 直接失败
- openGauss 官方文档包含 AT TIME ZONE（GaussDB 2.23.07.210 文档有引用），属于 openGauss 支持的语法
- ogsql-parser 的 keyword.rs 没有 AT TIME ZONE 操作符处理

#### P1-B. `?` 操作符被识别为 JDBC 占位符 — 3 个
`#0001 #0002 #0018`

错误特征：`expected ..., got JdbcParam`

- PG 几何/自定义操作符 `?#`、`?` 等被 tokenizer 当作 JDBC `?` 占位符
- 冲突点：README 声称支持 JDBC `?` 占位符，但几何操作符 `?#` 也是合法 PG 语法
- 需 tokenizer 上下文判断：`?` 后跟字母的操作符应识别为 Op，单独 `?` 才是 JdbcParam

#### P1-C. MERGE 复杂语法 — 2 个
`#0019 #0030`

错误特征：
- `#0019`: `expected then, got Keyword(AND)` — MERGE WHEN MATCHED THEN UPDATE SET a=x AND b=y 或复合条件
- `#0030`: `expected on, got Keyword(TABLESAMPLE)` — MERGE USING ... TABLESAMPLE

#### P1-D. 括号开头 SELECT — 1 个
`#0034`

错误特征：`expected statement, got LParen`

- `(SELECT ...)` 作为顶层语句时失败（PG 允许括号包裹的 SELECT）

### ⚠️ 需人工判断 / Needs manual review

#### 函数 arity — 5 个
`#0033 #0036 #0041 #0042 #0045`

| 案例 | 函数 | ogsql 报错 | 需确认 |
|---|---|---|---|
| #0033 | concat | requires at least 2 | PG 允许 0+ 参数，疑似 ogsql 过严（**可能是真 bug**） |
| #0036 | regexp_substr | takes at most 4 | openGauss 版可能是 3 参数（PG 的 regexp_substr 不同） |
| #0041 | date_trunc | requires exactly 2 | openGauss 的 date_trunc 参数数需查文档 |
| #0042 | regexp_instr | takes at most 5 | 同上 |
| #0045 | width_bucket | requires at least 3 | openGauss 支持 3 参形式，PG 也如此，疑似 ogsql 过严 |

#### SQL 保留字 — 3 个
`#0013 #0022 #0031`

- `current_user` / `session_user` / `cast` 作为标识符被拒绝
- 需确认 openGauss 是否允许这些作为非保留关键字使用（PG 中 current_user 是保留字，但 cast 不是）

### ✅ PG-only 已知可接受（不计回归）— 12 个

`#0004 #0014 #0024 #0026 #0027 #0037 #0039 #0040 #0043 #0044 #0046 #0047`

| 案例 | 原因 |
|---|---|
| #0014 | ON CONFLICT：openGauss 有意不支持 PG 的 ON CONFLICT（ogsql 有专门测试拒绝） |
| #0004 #0039 #0040 #0043 #0047 | `pg_*` 内部函数 arity（pg_stat_file, pg_read_file, pg_terminate_backend 等） |
| #0024 #0026 #0027 | 几何类型函数（point, circle, polygon）—— openGauss 不支持几何类型 |
| #0037 #0044 #0046 | XML 函数（cursor_to_xmlschema, database_to_xml, xpath） |

---

## 修正过程记录 / Correction log

1. **ON CONFLICT 误判为 bug → 实为设计行为**：ogsql-parser 有专门测试 `test_insert_on_conflict_rejected` 拒绝 PG 的 ON CONFLICT（openGauss 用 `ON DUPLICATE KEY UPDATE` 变体）。`#0014` 已标记 `known_acceptable=true`。
2. **TABLESAMPLE 是支持的**：ogsql 的 keyword.rs + formatter 都有 TABLESAMPLE 处理，所以 SQLsmith 的 TABLESAMPLE 失败（`#0030`）是复杂上下文的**真实 bug**，不是方言差异。

---

## 建议修复顺序 / Suggested fix order

| 优先级 | 类别 | 案例 | 工作量估计 |
|---|---|---|---|
| 1 | 表达式操作符（嵌套表达式位置） | 10 | 中 — 修 expr.rs 的 Pratt 解析器操作符表 |
| 2 | 子查询 JOIN | 8 | 中 — 修 select.rs 的 JOIN 解析递归 |
| 3 | AT TIME ZONE | 3 | 小 — 加操作符解析 + formatter |
| 4 | JdbcParam 误识别 | 3 | 中 — tokenizer 上下文判断 |
| 5 | MERGE 复杂语法 | 2 | 小 |
| 6 | 括号开头 SELECT | 1 | 小 |

修复后运行 `make guard`：
- 相关案例的 `expected_outcome` 从 FAIL 改 OK + 填 `fixed_in_commit`
- `regressions.csv` 应消失，`improvements.csv` 列出修复的案例
