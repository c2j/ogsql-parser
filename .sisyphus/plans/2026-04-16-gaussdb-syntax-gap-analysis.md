# ogsql-parser GaussDB/openGauss 语法差距分析

> 日期: 2026-04-16
> 对照: lib/opengauss-Server v5.0.3 + GaussDB 2.23.07.210 产品文档 (for 华为云Stack 8.3.0) 04

## 统计汇总

| 分类 | 数量 | 说明 |
|------|------|------|
| 完全缺失的语句 | 52 | GaussDB 文档中有但 AST 和解析逻辑均无 |
| Stub（有 AST 无解析） | 37 | 结构体已定义但 parser 跳过 |
| 子语法不完整 | 12 | 已有基本解析但缺 GaussDB 特有扩展子句 |
| **合计** | **101** | — |

---

## P0 — 高优先（核心 DDL/DML，影响回归测试覆盖）

### P0-1: ALTER TABLE PARTITION
- 分区表专用 ALTER 语法（独立于通用 ALTER TABLE）
- 含: ADD/DROP/MERGE/SPLIT/EXCHANGE/TRUNCATE/RENAME PARTITION
- GaussDB 文档: `1216_ALTER TABLE PARTITION.txt`

### P0-2: CREATE TABLE PARTITION
- 独立分区表创建语法
- 含: 范围/列表/哈希分区定义、SUBPARTITION、TEMPLATE 等
- GaussDB 文档: `1273_CREATE TABLE PARTITION.txt`

### P0-3: CREATE GLOBAL INDEX
- 分布式全局索引，区别于普通 CREATE INDEX
- GaussDB 文档: `1249_CREATE GLOBAL INDEX.txt`

### P0-4: EXPLAIN PLAN
- `EXPLAIN PLAN [SET STATEMENT_ID = ...] FOR ...`
- 将计划保存到 PLAN_TABLE
- GaussDB 文档: `1333_EXPLAIN PLAN.txt`

### P0-5: SELECT INTO
- `SELECT ... INTO [TABLE|VARIABLE] ...` 独立语法形式
- GaussDB 文档: `1372_SELECT INTO.txt`

### P0-6: 补全 37 个 stub 语句的解析逻辑
以下语句有 `stub_struct!` AST 定义但 parser 仅 `skip_to_semicolon`:

| # | 语句 | AST 结构体 |
|---|------|-----------|
| 1 | CREATE CONVERSION | `CreateConversionStatement` |
| 2 | ALTER DOMAIN | `AlterDomainStatement` |
| 3 | CREATE SYNONYM | `CreateSynonymStatement` |
| 4 | CREATE MODEL | `CreateModelStatement` |
| 5 | CREATE AM (Access Method) | `CreateAmStatement` |
| 6 | CREATE DIRECTORY | `CreateDirectoryStatement` |
| 7 | CREATE DATA SOURCE | `CreateDataSourceStatement` |
| 8 | CREATE EVENT | `CreateEventStatement` |
| 9 | CREATE OPCLASS | `CreateOpClassStatement` |
| 10 | CREATE OPFAMILY | `CreateOpFamilyStatement` |
| 11 | CREATE CONTQUERY | `CreateContQueryStatement` |
| 12 | CREATE STREAM | `CreateStreamStatement` |
| 13 | CREATE KEY | `CreateKeyStatement` |
| 14 | ALTER FOREIGN TABLE | `AlterForeignTableStatement` |
| 15 | ALTER FOREIGN SERVER | `AlterForeignServerStatement` |
| 16 | ALTER FDW | `AlterFdwStatement` |
| 17 | ALTER PUBLICATION | `AlterPublicationStatement` |
| 18 | ALTER SUBSCRIPTION | `AlterSubscriptionStatement` |
| 19 | ALTER NODE | `AlterNodeStatement` |
| 20 | ALTER NODE GROUP | `AlterNodeGroupStatement` |
| 21 | ALTER WORKLOAD GROUP | `AlterWorkloadGroupStatement` |
| 22 | ALTER AUDIT POLICY | `AlterAuditPolicyStatement` |
| 23 | ALTER RLS POLICY | `AlterRlsPolicyStatement` |
| 24 | ALTER DATA SOURCE | `AlterDataSourceStatement` |
| 25 | ALTER EVENT | `AlterEventStatement` |
| 26 | ALTER OPFAMILY | `AlterOpFamilyStatement` |
| 27 | SHUTDOWN | `ShutdownStatement` |
| 28 | BARRIER | `BarrierStatement` |
| 29 | PURGE | `PurgeStatement` |
| 30 | TIMECAPSULE TABLE | `TimeCapsuleStatement` |
| 31 | SNAPSHOT | `SnapshotStatement` |
| 32 | SHRINK | `ShrinkStatement` |
| 33 | VERIFY | `VerifyStatement` |
| 34 | CLEAN CONNECTION | `CleanConnStatement` |
| 35 | COMPILE | `CompileStatement` |
| 36 | SECURITY LABEL | `SecLabelStatement` |
| 37 | ALTER MATERIALIZED VIEW | 缺失（仅有 Refresh） |

---

## P1 — 中优先（运维和管理功能）

### P1-1: ABORT 语句
- `ABORT [WORK|TRANSACTION]`
- 文档: `1184_ABORT.txt`

### P1-2: SET ROLE / SET CONSTRAINTS / SET TRANSACTION 独立语句
- `SET ROLE [role_name|NONE]`
- `SET CONSTRAINTS {ALL|name} {DEFERRED|IMMEDIATE}`
- `SET TRANSACTION ISOLATION LEVEL ...`
- 文档: `1374~1377`

### P1-3: PREPARE TRANSACTION / COMMIT PREPARED / ROLLBACK PREPARED
- 二阶段提交三件套
- `PREPARE TRANSACTION 'gid'`
- `COMMIT PREPARED 'gid'`
- `ROLLBACK PREPARED 'gid'`
- 文档: `1354, 1236, 1366`

### P1-4: REASSIGN OWNED / DROP OWNED
- `REASSIGN OWNED BY old_role TO new_role`
- `DROP OWNED BY role_name`
- 文档: `1357, 1305`

### P1-5: VALUES 独立查询语句
- `VALUES (...), (...) [ORDER BY ...] [LIMIT ...]`
- 文档: `1388_VALUES.txt`

### P1-6: EXECUTE DIRECT
- `EXECUTE DIRECT ON (dn_name) 'sql'`
- 文档: `1329_EXECUTE DIRECT.txt`

### P1-7: ALTER 系列（管理对象）
- ALTER DATABASE LINK (`1189`)
- ALTER DIRECTORY (`1192`)
- ALTER LANGUAGE (`1198`)
- ALTER LARGE OBJECT (`1199`)
- ALTER PACKAGE (`1204`)
- ALTER SESSION (`1212`)
- ALTER SYSTEM KILL SESSION (`1214`)

### P1-8: ALTER 系列（扩展对象）
- ALTER SYNONYM (`1213`)
- ALTER TEXT SEARCH CONFIGURATION (`1218`)
- ALTER TEXT SEARCH DICTIONARY (`1219`)
- ALTER APP WORKLOAD GROUP MAPPING (`1185`)
- ALTER COORDINATOR (`1187`)

### P1-9: CREATE 系列（扩展对象）
- CREATE BARRIER
- CREATE INCREMENTAL MATERIALIZED VIEW (`1251`)
- CREATE LANGUAGE (`1253`)
- CREATE SECURITY LABEL (`1266`)
- CREATE WEAK PASSWORD DICTIONARY (`1281`)

### P1-10: DROP 系列（扩展对象）
- DROP PACKAGE (`1306`)
- DROP OWNED (`1305`)
- DROP GLOBAL CONFIGURATION (`1296`)
- DROP SECURITY LABEL (`1313`)
- DROP TEXT SEARCH CONFIGURATION/DICTIONARY (`1319, 1320`)
- DROP WEAK PASSWORD DICTIONARY (`1326`)
- DROP APP WORKLOAD GROUP MAPPING (`1288`)

### P1-11: 补全 CREATE TABLE 缺失子语法
- `LIKE source_table [like_option]` (含 INCLUDING/EXCLUDING 系列)
- `DISTRIBUTE BY RANGE/LIST ... SLICE ...` 分布规则定义
- `UPDATE SLICE LIKE table_name`
- `ENCRYPTION COLUMN encryption_column_spec`
- `TABLE OPTION (COMMENT/CHARSET/COLLATE)`
- `CHARACTER SET / CHARSET` 列级字符集
- `COLLATE` 列级排序规则
- `ON UPDATE {CURRENT_TIMESTAMP|NOW()}` 列级更新表达式

### P1-12: 补全 ALTER TABLE 缺失子语法
- `MODIFY column_name data_type` 子句
- `ADD (col1, col2, ...)` 多列添加
- `MODIFY (col1 type, col2 type, ...)` 多列修改
- `ADD/DELETE/ENABLE/DISABLE STATISTICS ((col1, col2))`
- `ALTER COLUMN SET STATISTICS [PERCENT]`
- `ALTER COLUMN SET STORAGE {PLAIN|EXTERNAL|EXTENDED|MAIN}`
- `GSIWAITALL` 等待全局索引
- `ENCRYPTION KEY ROTATION`
- `COMMENT [ = ] 'string'`
- `{DISABLE|ENABLE} [REPLICA|ALWAYS] RULE`

### P1-13: 补全 CREATE FUNCTION/PROCEDURE 缺失子语法
- `FENCED / NOT FENCED`
- `SHIPPABLE / NOT SHIPPABLE`
- `PACKAGE`

### P1-14: 补全 CREATE VIEW 缺失子语法
- `SECURITY BARRIER / SECURITY INVOKER`

### P1-15: 补全 GRANT/REVOKE 缺失目标类型
- `ON PROCEDURE`
- `ON LANGUAGE`
- `ON LARGE OBJECT`
- `ON TABLESPACE`
- `ON TYPE`

---

## P2 — 低优先（小众/管理工具语句）

### P2-1: 全文搜索相关
- CREATE/ALTER/DROP TEXT SEARCH CONFIGURATION
- CREATE/ALTER/DROP TEXT SEARCH DICTIONARY

### P2-2: 安全标签
- CREATE/DROP SECURITY LABEL
- SECURITY LABEL ON

### P2-3: 导入导出
- EXPDP DATABASE / EXPDP TABLE
- IMPDP DATABASE CREATE / IMPDP RECOVER / IMPDP TABLE / IMPDP TABLE PREPARE

### P2-4: AI 特性
- CREATE MODEL
- PREDICT BY

### P2-5: Hash bucket 管理
- LOCK BUCKETS
- MARK BUCKETS

### P2-6: 小众管理语句
- ALTER LARGE OBJECT
- SET SESSION AUTHORIZATION
- REPLACE (Oracle 兼容)
- MOVE
- REFRESH INCREMENTAL MATERIALIZED VIEW
- CREATE/DROP APP WORKLOAD GROUP MAPPING
