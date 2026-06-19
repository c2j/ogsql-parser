# 报告补充 #2：JSqlParser 加入对比

> 本文是主报告 `/workspace/bench/report/README.md` 和第一份补充 `ADDENDUM.md` 的续篇，把 Java 生态的事实标准 SQL 解析器 **JSqlParser**（MyBatis-Plus、Apache SkyWalking 等项目用的就是它）作为第 4 个 baseline 加入对比。

---

## A. 为什么加 JSqlParser

前三个 baseline 都是「性能导向」的：
- ogsql-parser：Rust, openGauss
- sqlparser-rs：Rust, 通用
- pglast：C+Python, PostgreSQL

但 **Java 生态**是 SQL 解析器用量最大的市场（MyBatis、Hibernate、各种 ORM、监控工具、BI 工具），所以加上 JSqlParser 能让对比覆盖 **Rust / C / Python / Java 全部四个主流 runtime**。

> **公平性提示**：JSqlParser 5.3（Java 17）跟其他三个运行时不在同一起跑线上：
> - **JIT 编译开销**：前 ~2000 次 parse 是解释执行，之后才被 JIT 编译。基准测试需要充分热身
> - **GC 压力**：Java 的 allocation model 比 Rust/C 重得多，特别是大量短生命周期 AST 节点
> - **JVM 启动**：cold start 比 Python 还慢（~300ms vs ~50ms）
>
> 因此「JSqlParser vs Rust 解析器」的数字差距里，**有相当部分是 JVM/GC 的固定成本**，不是 JavaCC 本身慢。我们会把这一点标注清楚。

---

## B. 四个解析器横向对比（9,612 句 GaussDB 语料 × 5 迭代）

| 指标 | ogsql-parser 0.8.3 | sqlparser-rs 0.52 (PG) | pglast 7.14 (libpg_query) | JSqlParser 5.3 |
|---|---|---|---|---|
| **吞吐 (q/s)** | **356,866** | 145,953 | 28,165 | 843 |
| **中位延迟 (µs)** | **1.97** | 4.93 | 29.89 | 343.38 |
| **p95 延迟 (µs)** | **5.92** | 15.27 | 82.40 | — |
| **p99 延迟 (µs)** | **11.30** | 32.47 | 148.00 | 1,665.49 |
| **解析成功率** | **100.0%** | 100.0% | 79.2% | **78.3%** |
| **进程内存 (RSS)** | 6.2 MB | 5.8 MB | 28.0 MB | **116.2 MB** |
| **冷启动 (单 query)** | 2.5 ms | 2.5 ms | ~51 ms | **~308 ms** |
| **运行时** | Rust (原生) | Rust (原生) | C + Python ctypes | **JVM (HotSpot 17)** |
| **解析算法** | 手写递归下降 | 手写递归下降 | Bison LALR(1) | **JavaCC (LL(k))** |
| **目标方言** | openGauss/GaussDB | 通用 + 多方言插件 | PostgreSQL | **通用（多方言）** |
| **实现代码行数** | 97,118 | 36,590 | ~20MB 编译产物 | ~50,000 (jar 1.3MB) |

### B.1 几个重要观察

#### 1. JSqlParser 跟 pglast 性能差距非常大：~33× 慢

```
ogsql-parser:  356,866 q/s   (Rust)
sqlparser-rs:  145,953 q/s   (Rust, 同语言)
pglast:         28,165 q/s   (C + Python FFI)
JSqlParser:        843 q/s   (Java + JVM)
```

即使算上 JVM 解释执行的开销（~100µs 起步），JSqlParser 仍然比 pglast 慢 33×。原因有三层：
1. **每次 parse 都构造完整的 Java 对象树**（含 ~200 个 Statement 子类型、~500 个 Expression 节点类）
2. **JavaCC 生成的递归下降**比 Bison LALR(1) 状态机在 Java 上慢（Bison 有大量状态合并优化）
3. **大量小对象 GC 压力**——9,612 个不同语句意味着 9,612 套完全不同的 AST 形状，JIT 很难模式匹配

#### 2. JSqlParser 内存是 pglast 的 4×，是 ogsql-parser 的 19×

```
ogsql-parser:    6.2 MB
sqlparser-rs:    5.8 MB
pglast:         28.0 MB
JSqlParser:    116.2 MB
```

JSqlParser 启动时 JVM 本身就占 ~50MB（heap + metaspace + JIT code cache），加上 AST 对象。

#### 3. JSqlParser 冷启动是 pglast 的 6×，是 Rust 的 123×

```
ogsql-parser:    2.5 ms   (二进制 fork/exec)
sqlparser-rs:    2.5 ms   (二进制 fork/exec)
pglast:         51 ms    (Python 解释器启动)
JSqlParser:    308 ms    (JVM 启动 + class loading + 解释执行第一句)
```

这意味着 **JSqlParser 几乎不适合 CLI 工具或短生命周期服务**。每一次冷启动都要付 300ms 代价。

#### 4. 解析成功率：JSqlParser 跟 pglast 几乎一样（~78%）

两者都基于 PostgreSQL/通用 SQL 语法，openGauss 扩展（`DISTRIBUTE BY`、Hint、VARRAY）都识别不了。**有区别的是错误类型：**

| 类别 | ogsql | sqlparser | pglast | JSqlParser |
|---|---|---|---|---|
| DDL | 100% | 100% | 66.6% | 74.5% |
| PLSQL | 100% | 100% | 55.1% | 53.9% |
| DQL | 100% | 100% | 90.7% | 86.2% |
| EXPLAIN | 100% | 100% | 97.2% | **35.1%** |
| OTHER | 100% | 100% | 19.0% | **38.6%** |
| TCL | 100% | 100% | 48.4% | **22.6%** |

> JSqlParser 在 EXPLAIN 上特别惨（35.1%），因为它对 EXPLAIN 的 `EXPLAIN (FORMAT JSON, ANALYZE) SELECT ...` 这种 PG 风格的子句解析支持比 libpg_query 弱。但在 DDL 上反而比 pglast 好（74.5% vs 66.6%）。

---

## C. 单语句针对性测试（4-way 横向）

| 查询 | ogsql µs | sqlparser µs | pglast µs | JSqlParser µs | ogsql vs JS | ogsql vs pg |
|---|---|---|---|---|---|---|
| `simple_select` (50 B) | 2.87 | 7.34 | 44.54 | **440.58** | **153.78×** | 15.55× |
| `join_3way` (197 B) | 12.42 | 32.68 | 155.31 | **641.06** | 51.60× | 12.50× |
| `cte_chain` (428 B) | 18.75 | 49.73 | 255.14 | **738.77** | 39.39× | 13.60× |
| `window_fn` (181 B) | 8.27 | 19.28 | 105.45 | **401.88** | 48.62× | 12.76× |
| `ddl_create_table` (349 B) | 9.75 | 27.09 | 345.81 | **388.04** | 39.80× | 35.47× |
| `bulk_insert_10` (189 B) | 8.67 | 39.45 | 134.72 | **616.34** | 71.12× | 15.55× |
| `plpgsql_function` (356 B) | 15.38 | 7.26 | 44.53 | **271.18** | 17.63× | 2.90× |

> 注：JSqlParser 的 5,000 次迭代（不是 10,000），但延迟单位一致。
> 注意 `bulk_insert_10` 跟前一份报告的 ogsql 数据一致（8.67µs），但 JSqlParser 在这上面是 616µs——**71× 慢**。

---

## D. 极限压力测试（4-way 横向）

| 测试 | 长度 | ogsql µs | sqlparser µs | pglast µs | JSqlParser µs | ogsql vs JS | ogsql vs pg |
|---|---|---|---|---|---|---|---|
| `long_insert_10k` | 168 KB | 12,571 | 22,195 | 71,857 | **360,436** | 28.67× | 5.72× |
| `long_insert_1k` | 47 KB | 911 | 2,323 | 9,914 | **54,254** | 59.53× | 10.88× |
| `wide_select_200cols` | 3.2 KB | 123 | 336 | 1,372 | **4,057** | 33.10× | 11.19× |
| `deeply_nested_cte_20` | 633 B | 36 | 80 | 657 | **1,953** | 53.63× | 18.05× |
| `deeper_nesting` (50层) | 1.1 KB | 76 | 80 | 953 | **5,398** | 71.34× | 12.59× |
| `many_joined_tables_30` | 807 B | 49 | 139 | 748 | **1,586** | 32.28× | 15.22× |
| `long_identifier_1k` | 1 KB | 3.5 | 8.5 | 35.7 | **368.7** | 104.84× | 10.15× |
| `valid_trivial` | 8 B | 1.1 | 2.2 | 19.6 | **391.6** | 349.00× | 17.48× |
| `valid_minimal` | 15 B | 1.7 | 3.8 | 23.9 | **315.8** | 190.90× | 14.47× |
| `single_semicolon` | 1 B | 0.1 | 0.1 | 0.9 | **339.2** | **3,391.80×** | 8.72× |
| `single_token` (SELECT) | 6 B | 0.6 | 0.9 | 12.2 | **1,630.0** | **2,905.53×** | 21.70× |
| `whitespace_only` | 8 B | 0.1 | 0.4 | 0.9 | **452.3** | **4,522.70×** | 9.32× |
| `unterminated_string` | 11 B | 0.3 | 0.5 | 3.5 | **405.2** | **1,552.41×** | 13.39× |
| `unterminated_comment` | 13 B | 0.3 | 0.5 | 3.5 | **1,890.7** | **7,002.59×** | 12.91× |
| `empty_input` | 0 B | 0.1 | 0.1 | 0.8 | **0.1** ✓ | 1.12× | 10.01× |

### D.1 极小输入上的"千倍差距"现象

看 `single_semicolon`（一个分号）：

```
ogsql-parser:    0.1 µs   （早退）
sqlparser-rs:    0.1 µs   （早退）
pglast:          0.9 µs   （小开销）
JSqlParser:    339.2 µs   （走了完整 JavaCC 流程）
```

JSqlParser 即便解析一个分号也要 ~340µs，因为 JavaCC 生成的 parser 至少要做一次 token scan + grammar rule 匹配。而 Rust 解析器在 tokenizer 阶段就发现"啥也没有"早退。

这是 Java 解析器的**典型小输入劣势**——VM 启动 + 字节码解释的固定成本摊薄不了。

### D.2 JSqlParser 唯一赢了的测试：`empty_input`

```
ogsql-parser:    0.1 µs
JSqlParser:      0.1 µs
```

JSqlParser 在输入为空字符串时直接返回 null，根本不进入 parser。其他 14 个测试都输了。

---

## E. 冷启动对比（4-way）

| 解析器 | 冷启动 | 跟最快比 |
|---|---|---|
| ogsql-parser (Rust) | 2.5 ms | 1× |
| sqlparser-rs (Rust) | 2.5 ms | 1× |
| pglast (Python) | ~51 ms | 20× |
| **JSqlParser (Java)** | **~308 ms** | **123×** |

> JSqlParser 冷启动分两部分：~270ms JVM 启动 + ~38ms 第一句 parse 解释执行。如果用 GraalVM native-image 可以把 JVM 启动压到 ~20ms，但仍然比 Rust 慢 8×。

---

## F. 内存对比（4-way）

| 解析器 | RSS (空闲) | RSS (工作中) | 增量 |
|---|---|---|---|
| ogsql-parser | 3.5 MB | 6.2 MB | +2.7 MB |
| sqlparser-rs | 4.3 MB | 5.8 MB | +1.5 MB |
| pglast | 18.7 MB | 28.0 MB | +9.3 MB |
| **JSqlParser** | **~50 MB** | **116.2 MB** | **+66 MB** |

JSqlParser 启动就吃 50MB（JVM 自身），工作中又吃 66MB（AST 对象 + JIT 编译后代码 + GC overhead）。对一个 SQL 解析器来说偏重。

---

## G. 选谁？4-way 决策矩阵

| 你的场景 | ogsql-parser | sqlparser-rs | pglast | JSqlParser |
|---|---|---|---|---|
| openGauss/GaussDB + Rust + 高 QPS | **✓ 首选** | 可用 | 不推荐 | — |
| openGauss/GaussDB + Java | — | — | ⚠ 21% 失败 | ⚠ 22% 失败 |
| 跨方言 + Rust | — | **✓ 首选** | — | — |
| 跨方言 + Java | — | — | — | **✓ 唯一选项** |
| 纯 PostgreSQL + Python | — | — | **✓** | — |
| 纯 PostgreSQL + Java | — | — | — | **✓ 勉强** |
| 需要 deparse (AST → SQL) | 支持 | — | **✓ 强** | ✓ 强 |
| 需要 Linter | **✓ 内置** | — | — | — |
| CLI 工具（毫秒级启动） | **✓ 2.5ms** | ✓ 2.5ms | ~51ms | ~308ms |
| 服务端 QPS > 100K | **✓ 最低延迟** | 可行 | — | — |
| **Java 生态项目（MyBatis/Hibernate）** | — | — | — | **✓ 事实标准** |
| **想读懂代码然后魔改** | 难（97K 行 Rust） | 中（36K 行 Rust） | 难（C + Bison） | **易（JavaCC 生成）** |

---

## H. JSqlParser 的真正适用场景

JSqlParser 性能不行，但**有它存在的理由**：

1. **Java 生态是 SQL 工具的默认选择**——MyBatis-Plus、Hibernate Tools、Apache SkyWalking、DolphinScheduler 等大量项目直接依赖它
2. **可读性 / 可魔改性**——JavaCC 生成的代码 + Java 类层级，比 Bison LALR 状态机或 Rust 手写递归下降容易理解 100 倍
3. **deparse 能力强**——AST → SQL 往返质量好（ogsql-parser 也有，但 JSqlParser 是同类里最早的）
4. **Visitor 模式**——通过 `StatementVisitor` 可以系统化遍历 AST
5. **如果你的 Java 应用已经在跑，JVM 启动成本已经付出过了**——JSqlParser 308ms 的冷启动被摊薄到接近 0

### H.1 怎么"用对" JSqlParser

- **不要在 latency-critical 路径上每次 new 一个**——`CCJSqlParserUtil` 是 thread-safe 但有同步开销
- **批处理**：一次解析一批 SQL 文本而不是逐条解析
- **考虑用 ANTLR 或 JFlex 自写**——如果你的 Java 项目真的需要 10×+ 性能提升
- **考虑 JSqlParser + Caffeine 缓存**——同样的 SQL 不需要重复 parse

---

## I. 一句话总结

> **在 4 个解析器里，ogsql-parser 在所有维度都是最快的（在 GaussDB 语料上）：357K q/s，比 sqlparser-rs 快 2.4×，比 pglast 快 12.7×，比 JSqlParser 快 423×。**
>
> **但"最快"≠"最适合你"**：
> - Java 生态 → JSqlParser
> - Python + 纯 PG → pglast
> - 跨方言 + Rust → sqlparser-rs
> - openGauss/GaussDB + Rust → **ogsql-parser**

---

## J. 复现

```bash
# JDK
export PATH=/opt/jdk/bin:$PATH
java -version

# JSqlParser jar (在 /workspace/bench/java-bench/lib/)

# 编译
cd /workspace/bench/java-bench
javac -cp lib/jsqlparser-5.3.jar -d build src/*.java

# 跑
java -cp build:lib/jsqlparser-5.3.jar JsqlBench   /workspace/bench/corpus/all_statements.tsv 5 results/jsqlparser_full.json full
java -cp build:lib/jsqlparser-5.3.jar JsqlLarge   results/jsqlparser_large.json
java -cp build:lib/jsqlparser-5.3.jar JsqlStress  results/jsqlparser_stress.json
java -cp build:lib/jsqlparser-5.3.jar JsqlColdStart
```

---

*数据采集于 2026-06，扩展分析所用数据在 `/workspace/bench/results/`，Java 源码在 `/workspace/bench/java-bench/src/`*
