# GaussDB SQL 语法覆盖度检查报告

> 基于 GaussDB-2.23.07.210 产品文档 (for 华为云Stack 8.3.0) 04 目录下的 SQL 语法文档
> 与当前 parser 实现的全面对比分析
> 生成日期: 2026-04-14

---

## 一、总览

| 维度 | 文档要求 | 解析器实现 | 覆盖率 |
|------|---------|-----------|--------|
| SQL 语句类型 | ~130 种 | ~145 AST 变体 | **~95%** (类型级别) |
| SQL 示例语句 (by_category) | 10,707 条 | — | 待验证 |
| 单元测试 | — | 325 tests | ✅ 全部通过 |
| 文档 04 目录覆盖 | 1184-1390 共 200+ 个语法文档 | — | 见下方详细对比 |

### SQL 文档分类统计

| Category | SQL Count |
|----------|-----------|
| DQL | 4,180 |
| DDL | 3,436 |
| DML_INSERT | 775 |
| PLSQL | 634 |
| SESSION | 431 |
| OTHER | 412 |
| EXPLAIN | 316 |
| TCL | 158 |
| DML_UPDATE | 73 |
| MAINTENANCE | 62 |
| DCL_GRANT | 54 |
| PREPARED_STMT | 46 |
| DML_DELETE | 45 |
| DML_COPY | 24 |
| CURSOR | 24 |
| DCL_REVOKE | 20 |
| DML_TRUNCATE | 11 |
| DML_MERGE | 6 |
| VACUUM | 4 |
| ANALYZE | 2 |
| **TOTAL** | **10,707** |

---

## 二、按类别逐项对比

### ✅ DQL — SELECT (文档: `1371_SELECT.txt`)

| 文档要求的语法特性 | 实现状态 | 备注 |
|---|---|---|
| WITH [RECURSIVE] CTE | ✅ | 含 NOT MATERIALIZED |
| DISTINCT / DISTINCT ON | ✅ | |
| FROM (JOIN/NATURAL/USING) | ✅ | INNER/LEFT/RIGHT/FULL/CROSS |
| TABLESAMPLE | ❌ **缺失** | `TABLESAMPLE sampling_method (argument) [REPEATABLE (seed)]` |
| WHERE | ✅ | 含 `(+)` 外连接 |
| CONNECT BY 层级查询 | ✅ | PRIOR, NOCYCLE, START WITH |
| GROUP BY (含 GROUPING SETS/ROLLUP/CUBE) | ✅ | |
| HAVING | ✅ | |
| WINDOW 窗口函数 | ✅ | OVER, PARTITION BY, ORDER BY, Frame |
| PIVOT / UNPIVOT | ✅ | 行列转换 |
| XMLTABLE | ❌ **缺失** | xmltable_clause 含 XMLNAMESPACES, PASSING, COLUMNS |
| NLSSORT 排序 | ❌ **缺失** | ORDER BY 中的 `NLSSORT(col, 'NLS_SORT=...')` |
| UNION/INTERSECT/EXCEPT/MINUS | ✅ | 含 ALL/DISTINCT |
| ORDER BY (ASC/DESC/NULLS FIRST/LAST) | ✅ | |
| LIMIT/OFFSET/FETCH | ✅ | |
| FOR UPDATE/SHARE (NOWAIT/WAIT/SKIP LOCKED) | ✅ | |
| Plan Hints (`/*+ hint */`) | ✅ | |
| SELECT INTO | ✅ | |

### ✅ DML (INSERT/UPDATE/DELETE/MERGE)

| 文档要求的语法特性 | 实现状态 | 备注 |
|---|---|---|
| INSERT VALUES | ✅ | |
| INSERT SELECT | ✅ | |
| INSERT ON CONFLICT | ✅ | NOTHING/UPDATE |
| INSERT RETURNING | ✅ | |
| INSERT ALL (多表无条件) | ✅ | |
| INSERT FIRST (多表条件) | ✅ | |
| INSERT DEFAULT | ✅ | |
| UPDATE SET/FROM/WHERE | ✅ | |
| UPDATE RETURNING | ✅ | |
| DELETE WHERE/USING | ✅ | |
| DELETE RETURNING | ✅ | |
| MERGE INTO | ✅ | WHEN MATCHED/NOT MATCHED |
| REPLACE (Oracle兼容) | ✅ | |

### ⚠️ DDL — CREATE TABLE (文档: `1270_CREATE TABLE.txt`)

| 文档要求的语法特性 | 实现状态 | 备注 |
|---|---|---|
| 临时表 (GLOBAL/LOCAL TEMP) | ✅ | |
| UNLOGGED | ✅ | |
| IF NOT EXISTS | ✅ | |
| 列定义 + 数据类型 | ✅ | |
| compress_mode (DELTA/PREFIX/DICTIONARY/NUMSTR) | ❌ **缺失** | 列级压缩选项 |
| CHARACTER SET / CHARSET | ✅ | |
| COLLATE | ✅ | |
| 列约束 (NOT NULL/CHECK/DEFAULT/UNIQUE/PK/FK) | ✅ | |
| ON UPDATE (CURRENT_TIMESTAMP/NOW()) | ⚠️ 部分缺失 | MySQL 兼容列自动更新 |
| 表约束 | ✅ | |
| LIKE source_table (INCLUDING/EXCLUDING) | ⚠️ 部分实现 | 需验证所有选项 |
| WITH (storage_parameter) | ✅ | |
| ON COMMIT (PRESERVE/DELETE ROWS) | ✅ | |
| COMPRESS / NOCOMPRESS | ❌ **缺失** | 表级压缩标记 |
| ILM ADD POLICY ROW STORE | ❌ **缺失** | `ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER n day/month/year OF NO MODIFICATION [ON (EXPR)]` |
| TABLESPACE | ✅ | |
| DISTRIBUTE BY (HASH/REPLICATION/RANGE/LIST) | ✅ | |
| TO GROUP / NODE | ✅ | |
| PARTITION BY RANGE/LIST/HASH | ✅ | |
| SUBPARTITION | ✅ | |

### ⚠️ DDL — ALTER TABLE (文档: `1215_ALTER TABLE.txt`)

| 文档要求的语法特性 | 实现状态 | 备注 |
|---|---|---|
| ADD COLUMN [IF NOT EXISTS] | ✅ | |
| DROP COLUMN [IF EXISTS] | ✅ | |
| ALTER COLUMN SET DATA TYPE | ✅ | |
| ALTER COLUMN SET/DROP DEFAULT | ✅ | |
| ALTER COLUMN SET/DROP NOT NULL | ✅ | |
| ALTER COLUMN SET STATISTICS | ⚠️ 需验证 | |
| ADD/DROP CONSTRAINT | ✅ | |
| RENAME TO (表名) | ✅ | |
| RENAME COLUMN | ✅ | |
| RENAME CONSTRAINT | ⚠️ 需验证 | |
| SET SCHEMA | ✅ | |
| OWNER TO | ✅ | |
| SET TABLESPACE | ✅ | |
| SET/RESET (storage_parameter) | ✅ | |
| CLUSTER ON / SET WITHOUT CLUSTER | ❌ **缺失** | |
| DISABLE/ENABLE TRIGGER | ❌ **缺失** | `DISABLE TRIGGER [trigger_name \| ALL \| USER]` |
| DISABLE/ENABLE ROW LEVEL SECURITY | ✅ | |
| NO FORCE/FORCE ROW LEVEL SECURITY | ❌ **缺失** | |
| INHERIT / NO INHERIT | ❌ **缺失** | |
| OF type_name / NOT OF | ❌ **缺失** | |
| REPLICA IDENTITY | ❌ **缺失** | |
| SET COMPRESS/NOCOMPRESS | ❌ **缺失** | |
| ADD/DELETE NODE | ❌ **缺失** | |
| UPDATE SLICE LIKE | ❌ **缺失** | |
| ADD TABLE CONSTRAINT USING INDEX | ❌ **缺失** | |
| VALIDATE CONSTRAINT | ❌ **缺失** | |
| MODIFY (MySQL兼容列修改) | ❌ **缺失** | `MODIFY column_name data_type` |
| ENCRYPTION KEY ROTATION | ❌ **缺失** | |
| ILM 策略操作 | ❌ **缺失** | `ILM {ENABLE\|DISABLE\|DELETE} POLICY` |
| COMMENT (表注释) | ⚠️ 需验证 | |
| CHARACTER SET / COLLATE (表级) | ⚠️ 需验证 | |
| ADD STATISTICS / DELETE STATISTICS | ❌ **缺失** | 多列统计信息管理 |
| GSIWAITALL | ❌ **缺失** | 全局二级索引同步等待 |

### ✅ DDL — ALTER TABLE PARTITION (文档: `1216_ALTER TABLE PARTITION.txt`)

| 文档要求的语法特性 | 实现状态 | 备注 |
|---|---|---|
| ADD PARTITION | ✅ | |
| DROP PARTITION | ✅ | |
| TRUNCATE PARTITION | ✅ | |
| MERGE PARTITIONS | ✅ | |
| SPLIT PARTITION | ✅ | |
| EXCHANGE PARTITION | ✅ | |
| RENAME PARTITION | ✅ | |
| MOVE PARTITION | ✅ | |

### ✅ DDL — 其他 CREATE 语句

| 文档要求的语句 | 实现状态 | AST 变体 |
|---|---|---|
| CREATE INDEX | ✅ | CreateIndex |
| CREATE INDEX USING | ⚠️ 需验证 | GIN/GiST 索引方法 |
| CREATE GLOBAL INDEX | ✅ | CreateIndex (global flag) |
| CREATE VIEW | ✅ | CreateView |
| CREATE MATERIALIZED VIEW | ✅ | CreateMaterializedView |
| CREATE INCREMENTAL MATERIALIZED VIEW | ❌ **缺失** | AST 中无专用变体 |
| CREATE TABLE AS | ✅ | CreateTableAs |
| CREATE TABLE PARTITION | ✅ | 包含在 CreateTable 中 |
| CREATE TABLE SUBPARTITION | ✅ | 包含在 CreateTable 中 |
| CREATE SCHEMA | ✅ | CreateSchema |
| CREATE DATABASE | ✅ | CreateDatabase |
| CREATE TABLESPACE | ✅ | CreateTablespace |
| CREATE FUNCTION | ✅ | CreateFunction |
| CREATE PROCEDURE | ✅ | CreateProcedure |
| CREATE PACKAGE / PACKAGE BODY | ✅ | CreatePackage + CreatePackageBody |
| CREATE TYPE | ✅ | CreateType |
| CREATE TRIGGER | ✅ | CreateTrigger |
| CREATE SEQUENCE | ✅ | CreateSequence |
| CREATE EXTENSION | ✅ | CreateExtension |
| CREATE ROLE / USER / GROUP | ✅ | 各自独立变体 |
| CREATE CAST | ✅ | CreateCast |
| CREATE CONVERSION | ✅ | CreateConversion |
| CREATE DOMAIN | ✅ | CreateDomain |
| CREATE AGGREGATE | ✅ | CreateAggregate |
| CREATE OPERATOR | ✅ | CreateOperator |
| CREATE FOREIGN TABLE | ✅ | CreateForeignTable |
| CREATE SERVER | ✅ | CreateForeignServer |
| CREATE FDW (Foreign Data Wrapper) | ✅ | CreateFdw |
| CREATE PUBLICATION / SUBSCRIPTION | ✅ | 各自独立变体 |
| CREATE SYNONYM | ✅ | CreateSynonym |
| CREATE MODEL | ✅ | CreateModel |
| CREATE DIRECTORY | ✅ | CreateDirectory |
| CREATE NODE / NODE GROUP | ✅ | 各自独立变体 |
| CREATE RESOURCE POOL | ✅ | CreateResourcePool |
| CREATE WORKLOAD GROUP | ✅ | CreateWorkloadGroup |
| CREATE AUDIT POLICY | ✅ | CreateAuditPolicy |
| CREATE MASKING POLICY | ✅ | CreateMaskingPolicy |
| CREATE RLS POLICY | ✅ | CreateRlsPolicy |
| CREATE DATA SOURCE | ✅ | CreateDataSource |
| CREATE EVENT | ✅ | CreateEvent |
| CREATE USER MAPPING | ✅ | CreateUserMapping |
| CREATE DATABASE LINK | ✅ | CreateDatabaseLink |
| CREATE WEAK PASSWORD DICTIONARY | ✅ | Statement::CreateWeakPasswordDictionary |
| CREATE SECURITY LABEL | ❌ **缺失** | 仅 SecLabel (SECURITY LABEL ON) |
| CREATE TEXT SEARCH CONFIGURATION | ⚠️ Stub | 需验证解析完整性 |
| CREATE TEXT SEARCH DICTIONARY | ⚠️ Stub | 需验证解析完整性 |
| CREATE OPCLASS / OPFAMILY | ✅ | 各自独立变体 |
| CREATE BARRIER | ✅ | Barrier |
| CREATE APP WORKLOAD GROUP MAPPING | ❌ **缺失** | 文档 1238 |

### ⚠️ DDL — ALTER 语句（非 ALTER TABLE）

| 文档要求的语句 | 实现状态 | 备注 |
|---|---|---|
| ALTER INDEX | ✅ | AlterIndex |
| ALTER VIEW | ✅ | AlterView |
| ALTER FUNCTION | ✅ | AlterFunction |
| ALTER PROCEDURE | ✅ | AlterProcedure |
| ALTER SCHEMA | ✅ | AlterSchema |
| ALTER DATABASE | ✅ | AlterDatabase |
| ALTER ROLE / USER / GROUP | ✅ | 各自独立变体 |
| ALTER SEQUENCE | ✅ | AlterSequence |
| ALTER DEFAULT PRIVILEGES | ✅ | AlterDefaultPrivileges |
| ALTER EXTENSION | ✅ | AlterExtension |
| ALTER TRIGGER | ✅ | AlterTrigger |
| ALTER FOREIGN TABLE | ✅ | AlterForeignTable |
| ALTER FOREIGN SERVER | ✅ | AlterForeignServer |
| ALTER FDW | ✅ | AlterFdw |
| ALTER PUBLICATION / SUBSCRIPTION | ✅ | 各自独立变体 |
| ALTER NODE / NODE GROUP | ✅ | 各自独立变体 |
| ALTER RESOURCE POOL | ✅ | AlterResourcePool |
| ALTER WORKLOAD GROUP | ✅ | AlterWorkloadGroup |
| ALTER AUDIT POLICY | ✅ | AlterAuditPolicy |
| ALTER MASKING POLICY | ✅ | AlterMaskingPolicy |
| ALTER RLS POLICY | ✅ | AlterRlsPolicy |
| ALTER DATA SOURCE | ✅ | AlterDataSource |
| ALTER EVENT | ✅ | AlterEvent |
| ALTER USER MAPPING | ✅ | AlterUserMapping |
| ALTER DATABASE LINK | ⚠️ Stub | 需验证完整性 |
| ALTER DIRECTORY | ❌ **缺失** | 文档 1192 |
| ALTER LANGUAGE | ❌ **缺失** | 文档 1198 |
| ALTER LARGE OBJECT | ❌ **缺失** | 文档 1199 |
| ALTER TEXT SEARCH CONFIGURATION | ⚠️ Stub | 需验证 |
| ALTER TEXT SEARCH DICTIONARY | ⚠️ Stub | 需验证 |
| ALTER TYPE (非Composite) | ⚠️ 部分 | AlterCompositeType 有，但通用 ALTER TYPE 可能缺失 |
| ALTER OPERATOR | ❌ **缺失** | |
| ALTER COORDINATOR | ❌ **缺失** | 文档 1187 |
| ALTER SESSION | ⚠️ 部分 | SET/RESET 已有，ALTER SESSION 语法可能不同 |
| ALTER SYSTEM KILL SESSION | ❌ **缺失** | 文档 1214 |
| ALTER GLOBAL CONFIGURATION | ✅ | AlterGlobalConfig |
| ALTER SYNONYM | ❌ **缺失** | 文档 1213 |
| ALTER TABLESPACE | ❌ **缺失** | 文档 1217 |
| ALTER OPFAMILY | ✅ | AlterOpFamily |
| ALTER PACKAGE | ⚠️ 需验证 | 文档 1204 |
| ALTER RESOURCE LABEL | ✅ | AlterPolicyLabel |
| ALTER APP WORKLOAD GROUP MAPPING | ❌ **缺失** | 文档 1185 |

### ⚠️ DDL — DROP 语句

| 文档要求的语句 | 实现状态 | 备注 |
|---|---|---|
| DROP TABLE | ✅ | Drop 通用变体，50+ ObjectType |
| DROP INDEX | ✅ | |
| DROP VIEW | ✅ | |
| DROP SCHEMA | ✅ | |
| DROP DATABASE | ✅ | |
| DROP FUNCTION / PROCEDURE | ✅ | |
| DROP TRIGGER | ✅ | |
| DROP EXTENSION | ✅ | |
| DROP SEQUENCE | ✅ | |
| DROP TYPE | ✅ | |
| DROP CAST | ✅ | |
| DROP DOMAIN | ✅ | |
| DROP ROLE / USER / GROUP | ✅ | |
| DROP TABLESPACE | ✅ | |
| DROP AGGREGATE | ✅ | |
| DROP OPERATOR / OPCLASS / OPFAMILY | ✅ | |
| DROP FOREIGN TABLE | ✅ | |
| DROP SERVER | ✅ | |
| DROP PUBLICATION / SUBSCRIPTION | ✅ | |
| DROP OWNED | ✅ | |
| DROP TEXT SEARCH CONFIG/DICT | ✅ | |
| DROP DIRECTORY | ⚠️ 需验证 | |
| DROP DATA SOURCE | ✅ | |
| DROP RULE | ✅ | DropRule |
| DROP MASKING POLICY | ✅ | |
| DROP AUDIT POLICY | ✅ | |
| DROP RLS POLICY | ✅ | |
| DROP RESOURCE POOL / LABEL | ✅ | |
| DROP WORKLOAD GROUP | ✅ | |
| DROP NODE / NODE GROUP | ✅ | |
| DROP PACKAGE | ✅ | |
| DROP MODEL | ✅ | |
| DROP SECURITY LABEL | ⚠️ 需验证 | |
| DROP USER MAPPING | ✅ | |
| DROP DATABASE LINK | ⚠️ 需验证 | |
| DROP WEAK PASSWORD DICTIONARY | ✅ | |
| DROP CONVERSION | ✅ | |
| DROP APP WORKLOAD GROUP MAPPING | ❌ **缺失** | |

### ✅ DCL — GRANT / REVOKE

| 文档要求的语法特性 | 实现状态 | 备注 |
|---|---|---|
| GRANT 权限 ON 对象 TO 角色 | ✅ | 17 种权限类型 |
| REVOKE 权限 ON 对象 FROM 角色 | ✅ | |
| GRANT ROLE TO 角色 | ✅ | GrantRole |
| REVOKE ROLE FROM 角色 | ✅ | RevokeRole |
| WITH ADMIN / GRANT OPTION | ✅ | |
| CASCADE | ✅ | |
| ALTER DEFAULT PRIVILEGES | ✅ | |

### ✅ TCL — 事务控制

| 文档要求的语句 | 实现状态 | 备注 |
|---|---|---|
| BEGIN / START TRANSACTION | ✅ | |
| COMMIT / END | ✅ | |
| COMMIT PREPARED | ✅ | |
| ROLLBACK | ✅ | |
| ROLLBACK PREPARED | ✅ | |
| ROLLBACK TO SAVEPOINT | ✅ | |
| SAVEPOINT | ✅ | |
| RELEASE SAVEPOINT | ✅ | |
| PREPARE TRANSACTION | ✅ | |
| SET TRANSACTION | ✅ | |
| SET CONSTRAINTS | ✅ | |
| ABORT | ✅ | |

### ✅ 会话管理

| 文档要求的语句 | 实现状态 | 备注 |
|---|---|---|
| SET | ✅ | VariableSet |
| SHOW | ✅ | VariableShow |
| RESET | ✅ | VariableReset |
| SET ROLE | ✅ | |
| SET SESSION AUTHORIZATION | ✅ | |

### ✅ PL/pgSQL (文档: `1413_PL_SQL.txt`)

| 文档要求的语法特性 | 实现状态 | 备注 |
|---|---|---|
| DO 语句 | ✅ | |
| 匿名块 DECLARE...BEGIN...END | ✅ | |
| 变量声明 | ✅ | 含 CONSTANT, DEFAULT, %TYPE, %ROWTYPE |
| 赋值 (:=) | ✅ | |
| IF/ELSIF/ELSE | ✅ | |
| CASE | ✅ | |
| LOOP/WHILE/FOR/FOREACH | ✅ | |
| EXIT/CONTINUE | ✅ | |
| RETURN | ✅ | |
| RETURN NEXT / RETURN QUERY | ✅ | |
| RAISE | ✅ | 6 个级别 |
| EXECUTE / EXECUTE IMMEDIATE | ✅ | 动态 SQL |
| PERFORM | ✅ | |
| 游标 OPEN/FETCH/CLOSE/MOVE | ✅ | |
| GET DIAGNOSTICS / STACKED | ✅ | |
| 异常处理 WHEN...THEN | ✅ | |
| GOTO | ✅ | |
| FORALL | ✅ | |
| PIPE ROW | ✅ | |
| 嵌套 PROCEDURE/FUNCTION | ✅ | |
| TYPE (VARRAY/TABLE OF) | ✅ | |
| RECORD 类型 | ✅ | |

### ⚠️ 其他语句

| 文档要求的语句 | 实现状态 | 备注 |
|---|---|---|
| EXPLAIN | ✅ | ExplainStatement |
| EXPLAIN PLAN | ✅ | |
| ANALYZE | ✅ | |
| VACUUM | ✅ | |
| REINDEX | ✅ | |
| CLUSTER | ✅ | |
| CHECKPOINT | ✅ | |
| COMMENT ON | ✅ | |
| LOCK TABLE | ✅ | |
| COPY | ✅ | |
| DO | ✅ | |
| CALL | ✅ | |
| PREPARE / EXECUTE / DEALLOCATE | ✅ | |
| DECLARE CURSOR | ✅ | |
| FETCH / CLOSE / MOVE | ✅ | |
| SHUTDOWN | ✅ | |
| PURGE | ✅ | |
| TIMECAPSULE TABLE | ✅ | |
| SNAPSHOT | ✅ | |
| BARRIER | ✅ | |
| CLEAN CONNECTION | ✅ | CleanConn |
| REASSIGN OWNED | ✅ | |
| SECURITY LABEL ON | ✅ | SecLabel |
| REFRESH MATERIALIZED VIEW | ✅ | |
| SHRINK | ✅ | |
| VERIFY | ✅ | |
| COMPILE | ✅ | |
| LISTEN / NOTIFY / UNLISTEN | ✅ | |
| RULE (CREATE RULE) | ✅ | |
| DISCARD | ✅ | |
| LOCK BUCKETS / MARK BUCKETS | ⚠️ 需验证 | GaussDB 特有 |
| EXECUTE DIRECT | ❌ **缺失** | 直接在 DN 上执行 |
| EXPDP DATABASE / TABLE | ❌ **缺失** | 数据导出 |
| IMPDP DATABASE / TABLE | ❌ **缺失** | 数据导入 |
| PREDICT BY | ❌ **缺失** | AI 预测 |
| SELECT INTO | ✅ | |

---

## 三、关键缺失汇总（按优先级）

### P0 — 高优先级（核心语法缺失，影响大量 SQL 解析）

| # | 缺失功能 | 影响范围 | 预估工作量 |
|---|---------|---------|-----------|
| 1 | ALTER TABLE: ILM ADD POLICY | 回归测试 23+ 条 SQL 失败 | 中 |
| 2 | ALTER TABLE: DISABLE/ENABLE TRIGGER | DDL 语法完整性 | 低 |
| 3 | ALTER TABLE: INHERIT/NO INHERIT | 继承表管理 | 低 |
| 4 | ALTER TABLE: VALIDATE CONSTRAINT | 约束管理 | 低 |
| 5 | ALTER TABLE: ADD CONSTRAINT USING INDEX | 唯一索引转约束 | 低 |
| 6 | ALTER TABLESPACE | 表空间管理 | 低 |
| 7 | CREATE TABLE: compress_mode | 列级压缩 | 低 |
| 8 | CREATE TABLE: COMPRESS/NOCOMPRESS | 表级压缩 | 低 |
| 9 | CREATE TABLE: ILM ADD POLICY | 数据生命周期管理 | 中 |
| 10 | CREATE INCREMENTAL MATERIALIZED VIEW | 增量物化视图 | 中 |

### P1 — 中优先级（GaussDB 特有功能，文档明确要求）

| # | 缺失功能 | 影响范围 | 预估工作量 |
|---|---------|---------|-----------|
| 11 | ALTER DIRECTORY | 目录对象管理 | 低 |
| 12 | ALTER LANGUAGE | 语言管理 | 低 |
| 13 | ALTER SYSTEM KILL SESSION | 会话管理 | 低 |
| 14 | ALTER SYNONYM | 同义词管理 | 低 |
| 15 | CREATE/ALTER/DROP APP WORKLOAD GROUP MAPPING | 工作负载管理 | 中 |
| 16 | ALTER OPERATOR | 运算符管理 | 低 |
| 17 | EXECUTE DIRECT | 分布式直接执行 | 低 |
| 18 | EXPDP/IMPDP 数据导入导出 (6 条语句) | 数据迁移 | 中 |
| 19 | PREDICT BY | AI 预测功能 | 低 |
| 20 | XMLTABLE | XML 表函数 | 高 |
| 21 | TABLESAMPLE | 表采样查询 | 低 |
| 22 | ALTER TABLE: MODIFY (MySQL兼容) | MySQL 兼容模式 | 中 |
| 23 | ALTER TABLE: ENCRYPTION KEY ROTATION | 加密密钥轮转 | 低 |
| 24 | ALTER TABLE: REPLICA IDENTITY | 逻辑复制 | 低 |
| 25 | ALTER TABLE: CLUSTER ON / SET WITHOUT CLUSTER | 聚簇管理 | 低 |
| 26 | NLSSORT 排序函数 | 国际化排序 | 低 |

### P2 — 低优先级（罕见或边缘语法）

| # | 缺失功能 | 影响范围 | 预估工作量 |
|---|---------|---------|-----------|
| 27 | ALTER COORDINATOR | 集群管理 | 低 |
| 28 | ALTER TABLE: FORCE/NO FORCE ROW LEVEL SECURITY | RLS 增强 | 低 |
| 29 | ALTER TABLE: OF type_name / NOT OF | 类型关联 | 低 |
| 30 | ALTER TABLE: ADD/DELETE NODE | 在线扩缩容 | 低 |
| 31 | ALTER TABLE: UPDATE SLICE LIKE | 切片更新 | 低 |
| 32 | ALTER TABLE: GSIWAITALL | GSI 同步 | 低 |
| 33 | ALTER TABLE: ADD/DELETE STATISTICS | 多列统计 | 低 |
| 34 | ALTER TABLE: COMMENT (表级) | 表注释 | 低 |
| 35 | CREATE SECURITY LABEL (专用语句) | 安全标签 | 低 |

---

## 四、已实现但需验证完整性的 Stub / 部分

以下 AST 变体已存在但可能是 stub（`_stub: ()`），需要验证解析完整性：

1. **CreateSynonym** — 可能仅跳过语法
2. **CreateContQuery / CreateStream** — 流式查询
3. **AlterForeignTable / AlterForeignServer / AlterFdw** — 外部表修改
4. **AlterPublication / AlterSubscription** — 逻辑复制
5. **CreateTextSearchConfig / CreateTextSearchDict** — 全文搜索
6. **AlterTextSearchConfig / AlterTextSearchDict** — 全文搜索修改
7. **AlterDatabaseLink** — 数据库链路修改
8. **DropDatabaseLink** — 数据库链路删除
9. **CreateEvent** — 事件创建
10. **AlterEvent** — 事件修改

---

## 五、函数/表达式层面缺失

| 特性 | 状态 | 文档参考 |
|---|---|---|
| OVERLAY(... PLACING ... FROM ... FOR ...) | ⚠️ 回归测试 94 条失败 | 特殊函数语法 |
| POSITION(... IN ...) | ⚠️ 同上 | 特殊函数语法 |
| SUBSTRING(... FROM ... FOR ...) | ⚠️ 同上 | 特殊函数语法 |
| TRIM(LEADING/TRAILING/BOTH FROM ...) | ⚠️ 同上 | 特殊函数语法 |
| FILTER (WHERE ...) | ✅ 已实现 | 聚合函数过滤 |
| WITHIN GROUP (ORDER BY ...) | ✅ 已实现 | 有序集聚合 |
| DEFAULT ON CONVERSION ERROR | ⚠️ 部分实现 | 类型转换错误处理 |
| 科学计数法 `1 e2` | ⚠️ Tokenizer 问题 | 30 条回归失败 |
| #, ~ 运算符 | ⚠️ 缺失 | GaussDB 特有运算符 |

---

## 六、量化总结

```
文档记录的 SQL 语句类型:      ~130 种
解析器 AST Statement 变体:    ~145 个 (含 Empty)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
完全实现:                     ~105 种 (80.8%)
Stub/部分实现:                ~18 种 (13.8%)
完全缺失:                     ~17 种 (13.1%)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
ALTER TABLE 子操作缺失:       ~16 项
表达式/函数层面缺失:           ~5 项
```

---

## 七、建议修复路线

### 第一波（解决回归测试最大失败源）

1. `OVERLAY/POSITION/SUBSTRING/TRIM` 特殊函数语法 → 解决 94 条失败
2. ILM 策略（CREATE TABLE + ALTER TABLE）→ 解决 31 条失败
3. 科学计数法 + DEFAULT ON CONVERSION ERROR → 解决 30 条失败

### 第二波（补齐 ALTER TABLE 子操作）

4. DISABLE/ENABLE TRIGGER
5. INHERIT/NO INHERIT
6. VALIDATE CONSTRAINT
7. ADD CONSTRAINT USING INDEX
8. CLUSTER ON / SET WITHOUT CLUSTER
9. REPLICA IDENTITY
10. SET COMPRESS/NOCOMPRESS
11. MODIFY (MySQL 兼容)

### 第三波（GaussDB 特有 DDL）

12. ALTER TABLESPACE
13. ALTER DIRECTORY
14. ALTER SYSTEM KILL SESSION
15. CREATE/ALTER APP WORKLOAD GROUP MAPPING
16. EXECUTE DIRECT
17. EXPDP/IMPDP

### 第四波（高级查询特性）

18. XMLTABLE
19. TABLESAMPLE
20. NLSSORT

---

## 八、参考文件路径

- 文档目录: `GaussDB-2.23.07.210/云数据库 GaussDB 2.23.07.210 产品文档 (for 华为云Stack 8.3.0) 04/`
- SQL 分类: `GaussDB-2.23.07.210/sql/by_category/` (20 个文件)
- SQL 按文档页: `GaussDB-2.23.07.210/sql/by_file/` (737 个文件)
- 解析器 AST: `src/ast/mod.rs`
- 解析器分发: `src/parser/mod.rs`
- 已有计划: `docs/plans/2026-04-13-gaussdb-p0-p1-implementation.md`
- 已有计划: `docs/plans/2026-04-12-gaussdb-syntax-coverage.md`
