# GaussDB 非 DDL/DML 功能缺失分析报告

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.
> 本文档为分析报告，非直接实施计划。实施时请参考本文档中的优先级和语法规范逐批实施。

**Goal:** 补齐 GaussDB SQL 解析器在 DDL/DML 之外的功能缺失，达到文档 04 目录的完整覆盖。

**Architecture:** 所有变更遵循现有架构 — AST 类型在 `src/ast/mod.rs`，Parser 方法分布在 `src/parser/` 各模块，Formatter 在 `src/formatter.rs`，测试在 `src/parser/tests.rs`。

**Tech Stack:** Rust 2021, thiserror 2.0, serde (Serialize + Deserialize)

**生成日期:** 2026-04-15

**前置文档:**
- 已有覆盖度审计: `docs/plans/2026-04-14-gaussdb-syntax-coverage-audit.md`
- 已有 P0/P1 计划: `docs/plans/2026-04-13-gaussdb-p0-p1-implementation.md`

---

## 一、已完整实现的非 DDL/DML 功能（无需操作）

| 类别 | 语句 | 状态 |
|------|------|------|
| TCL 事务控制 | BEGIN, START TRANSACTION, COMMIT, END, ROLLBACK, SAVEPOINT, RELEASE SAVEPOINT, PREPARE TRANSACTION, COMMIT PREPARED, ROLLBACK PREPARED, SET TRANSACTION, SET CONSTRAINTS, ABORT | ✅ |
| DCL 权限 | GRANT (17种权限), REVOKE, GRANT ROLE, REVOKE ROLE, ALTER DEFAULT PRIVILEGES | ✅ |
| 会话管理 | SET, SHOW, RESET, SET ROLE, SET SESSION AUTHORIZATION | ✅ |
| 游标 | DECLARE CURSOR, FETCH, CLOSE, MOVE | ✅ |
| 预处理 | PREPARE, EXECUTE, DEALLOCATE | ✅ |
| 维护 | VACUUM, ANALYZE, REINDEX, CLUSTER, CHECKPOINT | ✅ |
| 信息/锁 | COMMENT ON, LOCK TABLE, DISCARD | ✅ |
| 通知 | LISTEN, NOTIFY, UNLISTEN | ✅ |
| 规则 | CREATE RULE | ✅ |
| 物化视图 | CREATE MATERIALIZED VIEW, REFRESH MATERIALIZED VIEW, REFRESH INCREMENTAL MATERIALIZED VIEW | ✅ |
| PL/pgSQL | DO, 匿名块, 20+ 控制流语句 | ✅ |
| Plan Hint | 80+ hint 名称验证 | ✅ |
| 调用 | CALL (含命名参数 := 和 =>) | ✅ |
| 增量物化视图 | CREATE INCREMENTAL MATERIALIZED VIEW (dispatch_create 已实现) | ✅ |

---

## 二、完全缺失的非 DDL/DML 语句

### P1 — 中优先级（GaussDB 特有功能，文档明确要求，语法简单）

#### 2.1 ALTER TABLESPACE (文档 1217)

**语法：**
```sql
ALTER TABLESPACE tablespace_name RENAME TO new_name;
ALTER TABLESPACE tablespace_name OWNER TO new_owner;
ALTER TABLESPACE tablespace_name SET ( tablespace_option = value [, ...] );
ALTER TABLESPACE tablespace_name RESET ( tablespace_option [, ...] );
ALTER TABLESPACE tablespace_name RESIZE MAXSIZE { UNLIMITED | 'space_size' };
```

**实施要点：**
- 新增 AST: `AlterTablespaceStatement` 含 5 种 action 枚举
- dispatch_alter 中添加 TABLESPACE 分支
- 参照 AlterSchemaStatement 的模式

#### 2.2 ALTER SYSTEM KILL SESSION (文档 1214)

**语法：**
```sql
ALTER SYSTEM KILL SESSION 'session_sid, serial' [IMMEDIATE];
```

**实施要点：**
- 在 dispatch_alter 的 SYSTEM_P 分支中，检测 KILL SESSION 关键字序列
- 扩展 AlterGlobalConfigStatement 或新增 AST 变体

#### 2.3 CREATE/ALTER/DROP APP WORKLOAD GROUP MAPPING (文档 1238/1185/1288)

**语法：**
```sql
CREATE APP WORKLOAD GROUP MAPPING app_name [WITH (WORKLOAD_GPNAME = wg_name)];
ALTER APP WORKLOAD GROUP MAPPING app_name WITH (WORKLOAD_GPNAME = wg_name);
DROP APP WORKLOAD GROUP MAPPING app_name;
```

**实施要点：**
- 3 条语句，语法极简
- dispatch_create 中添加 APP 分支（需检测 APP WORKLOAD GROUP MAPPING 序列）
- dispatch_alter 中添加 APP 分支
- DROP 使用现有通用 Drop + ObjectType::App

#### 2.4 ALTER DIRECTORY (文档 1192)

**语法：**
```sql
ALTER DIRECTORY directory_name OWNER TO new_owner;
```

**实施要点：**
- 最简单的 ALTER：仅 OWNER TO 一种操作
- dispatch_alter 中添加 DIRECTORY 分支

#### 2.5 EXECUTE DIRECT (文档 1329)

**语法：**
```sql
EXECUTE DIRECT ON ( nodename [, ...] ) query;
EXECUTE DIRECT ON ( nodeoid [, ...] ) query;
EXECUTE DIRECT ON { COORDINATORS | DATANODES | ALL } query;
```

**实施要点：**
- 新增 AST: `ExecuteDirectStatement` 含 node_list 或 all_nodes 枚举 + query (String)
- parse_statement 中 EXECUTE 分支添加 DIRECT 关键字检测
- PREPARED_STMT 类别中有 4 条测试 SQL

#### 2.6 ALTER SYNONYM (文档 1213)

**语法：** 需查阅文档确认（可能仅 OWNER TO / RENAME TO）

#### 2.7 ALTER LANGUAGE (文档 1198)

**语法：** 需查阅文档确认

#### 2.8 ALTER OPERATOR (文档 — OTHER 类别)

**语法：**
```sql
ALTER OPERATOR @@ (text, text) OWNER TO omm;
```

**实施要点：**
- 简单：OWNER TO / SET SCHEMA / RENAME TO

### P2 — 低优先级（数据迁移/导出，内部使用为主）

#### 2.9 EXPDP DATABASE / EXPDP TABLE (文档 1330/1331)

**语法：**
```sql
EXPDP DATABASE db_name LOCATION = 'directory';
EXPDP TABLE table_name LOCATION = 'directory';
```

**实施要点：**
- 极简语法，各 1 行
- 新增 AST: `ExpdpStatement` 含 kind (Database/Table) + name + location
- parse_statement 中需新增 EXPDP 关键字处理

#### 2.10 IMPDP 系列 (文档 1339/1340/1341/1342)

**语法：**
```sql
IMPDP DATABASE [db_name] CREATE SOURCE = 'directory' OWNER = user [LOCAL];
IMPDP DATABASE RECOVER SOURCE = 'directory' OWNER = user [LOCAL];
IMPDP TABLE [AS table_name] SOURCE = 'directory' OWNER = user;
IMPDP TABLE PREPARE SOURCE = 'directory' OWNER = user;
```

**实施要点：**
- 4 条变体，可统一为 `ImpdpStatement` 含 kind 枚举
- parse_statement 中需新增 IMPDP 关键字处理

#### 2.11 CREATE SECURITY LABEL (文档 1266)

**语法：**
```sql
CREATE SECURITY LABEL label_name 'label context';
```

**实施要点：**
- 极简：名称 + 字符串内容
- dispatch_create 中添加 SECURITY 分支（检测 SECURITY LABEL 序列）
- 已有 SecLabel (SECURITY LABEL ON) 可复用模式

#### 2.12 CREATE/DROP CLIENT MASTER KEY (TDE 透明加密)

**语法：**
```sql
CREATE CLIENT MASTER KEY cmk1 WITH (KEY_STORE = hcs_kms, KEY_PATH = '...', ALGORITHM = AES_256);
DROP CLIENT MASTER KEY cmk1;
```

**实施要点：**
- 参考 CREATE DIRECTORY 的 stub 模式
- dispatch_create 中添加 CLIENT 分支（检测 CLIENT MASTER KEY 序列）

#### 2.13 CREATE/DROP COLUMN ENCRYPTION KEY (TDE 列加密)

**语法：**
```sql
CREATE COLUMN ENCRYPTION KEY cek1 WITH VALUES (CLIENT_MASTER_KEY = cmk1, ALGORITHM = AEAD_AES_256_CBC_HMAC_SHA256);
DROP COLUMN ENCRYPTION KEY cek1;
```

**实施要点：**
- 参考 CREATE CLIENT MASTER KEY

### P3 — 极低优先级（罕见、内部使用、或分布式特有）

#### 2.14 LOCK BUCKETS (文档 1346)

**语法：**
```sql
LOCK BUCKETS (bucketlist) IN {ACCESS SHARE | ACCESS EXCLUSIVE} MODE [CANCELABLE];
-- 仅在事务块内有效
```

**实施要点：**
- 仅扩容期间使用，管理员权限
- parse_statement 中 LOCK 分支添加 BUCKETS 检测

#### 2.15 MARK BUCKETS (文档 1348)

**语法：**
```sql
MARK BUCKETS (bucketlist) FINISH FROM datanode_name TO datanode_name;
```

**实施要点：**
- 仅扩容期间使用

#### 2.16 PREDICT BY (文档 1352)

**语法：** 文档仅 1 行，分布式场景暂不支持

#### 2.17 ALTER COORDINATOR (文档 1187)

**语法：** 集群管理，极罕见

#### 2.18 ALTER LARGE OBJECT (文档 1199)

**语法：** 大对象管理，极罕见

#### 2.19 SAMPLE SNAPSHOT

**语法：**
```sql
SAMPLE SNAPSHOT s1@2.0 stratify by name as nick at ratio .5;
```

**实施要点：** 快照采样，极罕见

#### 2.20 独立 CURSOR 声明（非 DECLARE 前缀）

```sql
CURSOR cursor1 FOR SELECT * FROM t;
```

**实施要点：** OTHER 类别中有出现，需确认是否需要支持

---

## 三、ALTER TABLE 子操作缺失（非 DDL/DML 但影响回归测试）

| # | 缺失子操作 | 语法 | 优先级 |
|---|-----------|------|--------|
| 1 | DISABLE/ENABLE TRIGGER | `DISABLE TRIGGER [name\|ALL\|USER]` | P0 — 23+ 回归失败 |
| 2 | CLUSTER ON / SET WITHOUT CLUSTER | 聚簇管理 | P2 |
| 3 | INHERIT / NO INHERIT | 继承管理 | P1 |
| 4 | REPLICA IDENTITY | `REPLICA IDENTITY {DEFAULT|USING INDEX|FULL|NOTHING}` | P2 |
| 5 | VALIDATE CONSTRAINT | `VALIDATE CONSTRAINT name` | P1 |
| 6 | ADD CONSTRAINT USING INDEX | `ADD CONSTRAINT name {UNIQUE|PK} USING INDEX idx` | P1 |
| 7 | SET COMPRESS/NOCOMPRESS | 压缩标记 | P1 |
| 8 | MODIFY (MySQL兼容) | `MODIFY col_name data_type [constraints]` | P2 |
| 9 | FORCE/NO FORCE ROW LEVEL SECURITY | RLS 增强 | P3 |
| 10 | OF type_name / NOT OF | 类型关联 | P3 |
| 11 | ADD/DELETE NODE | 在线扩缩容 | P3 |
| 12 | UPDATE SLICE LIKE | 切片更新 | P3 |
| 13 | ADD/DELETE STATISTICS | 多列统计 | P3 |
| 14 | GSIWAITALL | GSI 同步 | P3 |
| 15 | ILM 策略操作 | `ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER n day/month/year OF NO MODIFICATION` | P0 — 23+ 回归失败 |
| 16 | ENCRYPTION KEY ROTATION | 加密轮转 | P3 |

---

## 四、SELECT/DML 层面缺失

| # | 特性 | 状态 | 文档参考 | 工作量 |
|---|------|------|---------|--------|
| 1 | TABLESAMPLE | ❌ 缺失 | SELECT (1371) | 低 |
| 2 | XMLTABLE | ❌ 缺失 | SELECT (1371) | 高 |
| 3 | NLSSORT 排序 | ❌ 缺失 | SELECT (1371) | 低 |
| 4 | 独立 VALUES 查询 | ⚠️ 需验证 | VALUES (1388) | 低 |

---

## 五、已实现但为 Stub 的非 DDL/DML 语句（仅 skip_to_semicolon）

这些有 AST 类型但解析器只是跳过。**需要时逐个补全解析逻辑。**

| 语句 | AST 变体 | 文档编号 |
|------|---------|---------|
| CREATE SYNONYM | `CreateSynonym` | 1269 |
| CREATE MODEL | `CreateModel` | 1256 |
| CREATE DIRECTORY | `CreateDirectory` | 1245 |
| CREATE DATA SOURCE | `CreateDataSource` | 1244 |
| CREATE EVENT | `CreateEvent` | — |
| CREATE AM | `CreateAm` | — |
| CREATE CONTQUERY | `CreateContQuery` | — |
| CREATE STREAM | `CreateStream` | — |
| ALTER FOREIGN TABLE | `AlterForeignTable` | 1193 |
| ALTER FOREIGN SERVER | `AlterForeignServer` | 1211 |
| ALTER FDW | `AlterFdw` | — |
| ALTER PUBLICATION | `AlterPublication` | — |
| ALTER SUBSCRIPTION | `AlterSubscription` | — |
| ALTER NODE | `AlterNode` | 1202 |
| ALTER NODE GROUP | `AlterNodeGroup` | 1203 |
| ALTER WORKLOAD GROUP | `AlterWorkloadGroup` | 1224 |
| ALTER AUDIT POLICY | `AlterAuditPolicy` | 1186 |
| ALTER RLS POLICY | `AlterRlsPolicy` | 1208 |
| ALTER DATA SOURCE | `AlterDataSource` | 1190 |
| ALTER EVENT | `AlterEvent` | — |
| ALTER OPFAMILY | `AlterOpFamily` | — |
| ALTER DATABASE LINK | `AlterDatabaseLink` | 1189 |
| CREATE TEXT SEARCH CONFIG/DICT | 两者 | 1274/1275 |
| ALTER TEXT SEARCH CONFIG/DICT | 两者 | 1218/1219 |

---

## 六、量化总结

```
非DDL/DML缺失统计:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
完全缺失:               ~20 种语句 (含 4 种 TDE 加密)
  P1 (应优先实施):       ~8 种
  P2 (数据迁移/安全):    ~7 种
  P3 (罕见/内部):        ~5 种

Stub需补全:              ~24 种 (有AST无解析)

ALTER TABLE 子操作缺失:  ~16 项
SELECT 层面缺失:          ~4 项

预估总工作量:
  P1 全部实施:           ~2-3 天
  P2 全部实施:           ~1-2 天
  P3 全部实施:           ~1 天
  Stub 补全 (如需):      ~5-7 天
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## 七、建议实施路线

### 第一波 — 简单高价值 (P1, 每条 1-2 小时)
1. ALTER TABLESPACE (5 种子操作)
2. ALTER DIRECTORY (仅 OWNER TO)
3. ALTER SYSTEM KILL SESSION
4. CREATE/ALTER/DROP APP WORKLOAD GROUP MAPPING (3 条)
5. EXECUTE DIRECT (含节点名/oid/ALL)
6. ALTER SYNONYM
7. ALTER LANGUAGE
8. ALTER OPERATOR

### 第二波 — 数据迁移类 (P2)
9. EXPDP DATABASE + EXPDP TABLE (2 条)
10. IMPDP DATABASE CREATE/RECOVER + IMPDP TABLE/PREPARE (4 条)
11. CREATE SECURITY LABEL

### 第三波 — 安全/加密类 (P2)
12. CREATE/DROP CLIENT MASTER KEY (2 条)
13. CREATE/DROP COLUMN ENCRYPTION KEY (2 条)

### 第四波 — 罕见/内部 (P3)
14. LOCK BUCKETS / MARK BUCKETS
15. PREDICT BY
16. ALTER COORDINATOR / ALTER LARGE OBJECT
17. SAMPLE SNAPSHOT

---

## 八、参考文件路径

- 文档目录: `GaussDB-2.23.07.210/云数据库 GaussDB 2.23.07.210 产品文档 (for 华为云Stack 8.3.0) 04/`
- SQL 分类目录: `GaussDB-2.23.07.210/sql/by_category/` (20 个文件, 10707 条 SQL)
- 已有覆盖度审计: `docs/plans/2026-04-14-gaussdb-syntax-coverage-audit.md`
- 已有 P0/P1 计划: `docs/plans/2026-04-13-gaussdb-p0-p1-implementation.md`
- 解析器 AST: `src/ast/mod.rs`
- 解析器分发: `src/parser/mod.rs`
- 解析器 Utility: `src/parser/utility/`
