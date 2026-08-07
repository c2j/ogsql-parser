# 第二轮 SQLsmith 验证报告 / Second SQLsmith Verification Report

> 日期 / Date: 2026-08-08
> 验证对象 / Verified against: `d88306a` — fix: address PR#315 SQLsmith-discovered parser/tokenizer issues (7 categories) (#317)
> 验证方式 / Method: `sqlsmith-harness guard` — 对 `regress/` 中 47 个守护案例重新解析 + round-trip
> 关键改动 / Key change: harness 的 `try_parse` 现在**只把硬错误计为失败**（`is_warning` 过滤：函数 arity 警告、保留字 AS-alias 不再误判）

---

## 总体结果 / Overall

| 指标 | 值 |
|---|---|
| 守护案例总数 | 47 |
| **已修复并锁定（expected_outcome=OK）** | **14** |
| 仍预期失败（expected_outcome=FAIL） | 33 |
| **回归（expected OK → actual FAIL）** | **0** ✅ |
| guard 退出码 | 0 ✅ |

修复提交 `#317` 的 7 类修复在**简单/典型形式下全部生效**，并新增了 14 个正向守护案例。

---

## 14 个已修复案例 / Fixed cases

| 类别 | 案例 | 说明 |
|---|---|---|
| P1-B `?` 操作符 | #0018 | `?|` 等 JSONB 操作符正确识别 |
| P0-A 表达式操作符 | #0015 #0025 | 嵌套表达式中的 `%`、`>` 等 |
| P1-A AT TIME ZONE | （简单形式） | `AT TIME ZONE` 解析通过 |
| P2 函数 arity | #0004 #0026 #0027 #0033 #0039 #0040 #0041 #0043 #0044 | concat/几何函数/pg_* 等 arity 修正（部分降为 warning 而非 error）|
| P2 保留字 | #0013 #0022 | current_user/session_user AS-alias |

> 注：#0004/#0026 等函数 arity 案例实际是**从硬错误降为 warning**——harness 修正后正确视为"通过（有提示）"。这是 harness 判定逻辑的修复，非 parser 行为变化。

---

## 33 个剩余失败案例 / Remaining failures (guard 重新解析确认)

| 根因类别 | 数量 | 案例 | 详情 |
|---|---|---|---|
| **C. 子查询中 JOIN 关键字** | 13 | #0003 #0006 #0007 #0008 #0010 #0011 #0012 #0017 #0020 #0024 #0036 #0037 #0046 | `tablesample system (N) INNER JOIN ...` 组合未覆盖；修复只处理了 `MERGE USING ... TABLESAMPLE` |
| **B. 嵌套表达式操作符** | 9 | #0005 #0016 #0021 #0023 #0028 #0029 #0038 #0042 #0047 | `~*`、`<= >= <>` 等在深层嵌套（CASE/CAST/函数参数）仍失败 |
| **A. `?#` 几何操作符** | 3 | #0001 #0002 #0045 | **明确遗漏**：修复只覆盖 `?| ?& ?- ?-| ?||`，漏了 `?#`（lseg/box 相交测试）|
| **D. AT TIME ZONE 嵌套** | 3 | #0009 #0032 #0035 | 复杂嵌套上下文（CASE THEN 内、WITH 内）仍失败 |
| **G. MERGE WHEN AND** | 2 | #0019 #0030 | `WHEN MATCHED THEN ... AND ...` 复合条件 |
| **E. 括号开头语句** | 2 | #0031 #0034 | `(SELECT ...)` / `FROM (SELECT ...)` 组合 |
| **F. ON CONFLICT** | 1 | #0014 | ✅ **预期行为**（openGauss 不支持，known_acceptable）|

---

## 第二轮新增发现 / New findings (第二轮验证的增量价值)

1. **`?#` 操作符遗漏**（#0001 #0002 #0045）—— 最明确的 bug。`SELECT lseg '...' ?# box '...'` 直接失败：`got JdbcParam`。修复提交的 P1-B 只覆盖 `?| ?& ?- ?-| ?||`，`?#` 未列入。
2. **`tablesample` + JOIN 组合**（13 个案例的主力）—— `FROM t tablesample system (N) INNER JOIN u ON ...` 是 SQLsmith 高频输出模式，修复仅覆盖 `MERGE USING`。
3. **AT TIME ZONE 在复杂上下文**（#0009 #0032 #0035）—— 简单 `SELECT x AT TIME ZONE y` 已通过，但 `CASE WHEN ... THEN x AT TIME ZONE y` / WITH 内仍失败。
4. **函数 arity 判定改进** —— harness 的 `is_warning` 过滤是本次验证的重要修正：11 个函数 arity 案例从"失败"正确变为"通过（warning）"。

---

## 建议修复顺序 / Suggested next fixes

| 优先级 | 类别 | 案例数 | 建议 |
|---|---|---|---|
| P0 | `?#` 操作符 | 3 | tokenizer 的 `?` 分支补 `#` 字符（1 行 + 测试）|
| P0 | `tablesample`+JOIN | 13 | select.rs 的 FROM 子句在 table modifiers 后支持 JOIN 关键字 |
| P1 | 嵌套表达式操作符 | 9 | expr.rs 的 Pratt 解析器操作符表补全 `~* <= >= <>` 等 |
| P1 | AT TIME ZONE 嵌套 | 3 | 检查 AtTimeZone 后置解析在 CASE/表达式嵌套的递归 |
| P2 | MERGE WHEN AND | 2 | dml.rs 的 MERGE WHEN 条件支持复合 AND |
| P2 | 括号开头语句 | 2 | 语句分发支持括号包裹的 SELECT |

修复后运行：
```sh
make guard   # 相关案例 expected_outcome FAIL→OK + fixed_in_commit 填入
```

---

## 附：harness 变更 / Harness changes this round

`tests/sqlsmith/sqlsmith_harness.rs`:
- `try_parse` 的 parse 判定改为只统计**硬错误**（`!is_warning(e)`），Warning / ReservedKeywordAsIdentifier 视为"通过（有提示）"
- round-trip reparse 判定同步修正
- 影响：函数 arity 警告类案例不再误报为失败（修复前 guard 报告 4 个修复，修正后正确报告 14 个）
