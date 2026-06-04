# GaussDB / openGauss SQL 语法支持详情

> OGSQL Parser 支持的 GaussDB / openGauss SQL 语法完整清单  
> Complete list of supported GaussDB / openGauss SQL syntax  
> 版本 0.6.10 | 最后更新 2026-06-04

---

## 目录

- [1. DML 语句 (数据操作语言)](#1-dml-语句)
- [2. DDL 语句 (数据定义语言)](#2-ddl-语句)
  - [2.1 CREATE 语句](#21-create-语句)
  - [2.2 ALTER 语句](#22-alter-语句)
  - [2.3 DROP 语句](#23-drop-语句)
- [3. PL/pgSQL 过程化语言](#3-plpgsql-过程化语言)
- [4. 表达式与运算符](#4-表达式与运算符)
- [5. 数据类型](#5-数据类型)
- [6. 内置函数](#6-内置函数)
- [7. GaussDB 专有扩展](#7-gaussdb-专有扩展)
- [8. Oracle 兼容特性](#8-oracle-兼容特性)
- [9. 其他语句](#9-其他语句)

---

## 1. DML 语句

### SELECT

| 特性 | 示例语法 |
|------|----------|
| 基本 SELECT | `SELECT col1, col2 FROM table_name` |
| WHERE 条件 | `SELECT * FROM t WHERE col = 1` |
| GROUP BY / HAVING | `SELECT dept, COUNT(*) FROM t GROUP BY dept HAVING COUNT(*) > 5` |
| ORDER BY | `SELECT * FROM t ORDER BY col ASC/DESC NULLS FIRST/LAST` |
| LIMIT / OFFSET | `SELECT * FROM t LIMIT 10 OFFSET 20` |
| FETCH FIRST | `SELECT * FROM t FETCH FIRST 10 ROWS ONLY [WITH TIES]` |
| DISTINCT / DISTINCT ON | `SELECT DISTINCT col FROM t` / `SELECT DISTINCT ON (col) * FROM t` |
| CTE (WITH) | `WITH cte AS (SELECT ...) SELECT * FROM cte` |
| 递归 CTE | `WITH RECURSIVE cte AS (...) SELECT * FROM cte` |
| MATERIALIZED / NOT MATERIALIZED | `WITH cte AS NOT MATERIALIZED (...)` |
| 子查询 | `SELECT * FROM (SELECT * FROM t) AS sub` |
| EXISTS | `SELECT * FROM t WHERE EXISTS (SELECT 1 FROM t2)` |
| 集合运算 | `UNION [ALL]`, `INTERSECT [ALL]`, `EXCEPT [ALL]`, `MINUS [ALL]` |
| JOIN | `INNER JOIN`, `LEFT/RIGHT/FULL OUTER JOIN`, `CROSS JOIN`, `NATURAL JOIN`, `USING` |
| LATERAL | `SELECT * FROM t, LATERAL (SELECT ...) sub` |
| 窗口函数 | `ROW_NUMBER() OVER (PARTITION BY col ORDER BY col)` |
| 窗口帧 | `ROWS/RANGE/GROUPS BETWEEN ... AND ...` |
| PIVOT / UNPIVOT | `SELECT * FROM t PIVOT (SUM(amount) FOR month IN (...))` |
| TABLESAMPLE | `SELECT * FROM t TABLESAMPLE BERNOULLI(10)` |
| FOR UPDATE / SHARE | `SELECT * FROM t FOR UPDATE [NOWAIT | SKIP LOCKED | WAIT n]` |
| CONNECT BY | `CONNECT BY [NOCYCLE] PRIOR id = parent_id START WITH ...` |
| ORDER SIBLINGS BY | `ORDER SIBLINGS BY col` |
| INTO TABLE | `SELECT * INTO [UNLOGGED] TABLE new_t FROM t` |
| SELECT INTO 变量 | `SELECT cols INTO variables FROM ...` |
| BULK COLLECT INTO | `SELECT ... BULK COLLECT INTO collection FROM ...` |
| HINT | `/*+ SeqScan(t) */ SELECT * FROM t` |
| 分区引用 | `SELECT * FROM t PARTITION (p1)` |
| 子分区引用 | `SELECT * FROM t SUBPARTITION (sp1)` |
| 时间胶囊 | `SELECT * FROM t TIMECAPSULE (TIMESTAMP ...)` / `TIMECAPSULE (CSN ...)` |

### INSERT

| 特性 | 示例语法 |
|------|----------|
| INSERT VALUES | `INSERT INTO t (col1, col2) VALUES (1, 'a')` |
| INSERT SELECT | `INSERT INTO t SELECT * FROM t2` |
| 多行 VALUES | `INSERT INTO t VALUES (1, 'a'), (2, 'b')` |
| DEFAULT VALUES | `INSERT INTO t DEFAULT VALUES` |
| SET 子句 | `INSERT INTO t SET col1 = val1, col2 = val2` |
| ON CONFLICT | `INSERT INTO t VALUES (...) ON CONFLICT (col) DO UPDATE SET ...` |
| ON DUPLICATE KEY | `INSERT INTO t VALUES (...) ON DUPLICATE KEY UPDATE SET ...` |
| RETURNING | `INSERT INTO t VALUES (...) RETURNING *` |
| RETURNING INTO | `INSERT INTO t VALUES (...) RETURNING * INTO variables` |
| BULK COLLECT INTO | `INSERT INTO t VALUES (...) RETURNING * BULK COLLECT INTO collection` |
| WITH (CTE) | `WITH cte AS (...) INSERT INTO t SELECT * FROM cte` |
| INSERT ALL (Oracle) | `INSERT ALL INTO t1 VALUES (...) INTO t2 VALUES (...) SELECT * FROM dual` |
| INSERT FIRST (Oracle) | `INSERT FIRST WHEN cond THEN INTO t1 ... ELSE INTO t2 ... SELECT ...` |
| REPLACE (Oracle) | `REPLACE INTO t VALUES (...)` |
| 分区指定 | `INSERT INTO t PARTITION (p1) VALUES (...)` |
| 记录变量插入 | `INSERT INTO t VALUES record_variable` (Oracle/%ROWTYPE) |

### UPDATE

| 特性 | 示例语法 |
|------|----------|
| 基本 UPDATE | `UPDATE t SET col = val WHERE cond` |
| 多列 SET | `UPDATE t SET (col1, col2) = (val1, val2)` |
| FROM 子句 | `UPDATE t SET col = t2.val FROM t2 WHERE t.id = t2.id` |
| RETURNING | `UPDATE t SET col = val RETURNING *` |
| RETURNING INTO | `UPDATE t SET col = val RETURNING * INTO variables` |
| BULK COLLECT INTO | `UPDATE t SET col = val RETURNING * BULK COLLECT INTO collection` |
| WHERE CURRENT OF | `UPDATE t SET col = val WHERE CURRENT OF cursor_name` |
| WITH (CTE) | `WITH cte AS (...) UPDATE t SET ...` |
| 分区指定 | `UPDATE t PARTITION (p1) SET ...` |

### DELETE

| 特性 | 示例语法 |
|------|----------|
| 基本 DELETE | `DELETE FROM t WHERE cond` |
| USING 子句 | `DELETE t FROM t1 USING t2 WHERE t1.id = t2.id` |
| ORDER BY / LIMIT | `DELETE FROM t WHERE cond ORDER BY col LIMIT n` |
| RETURNING | `DELETE FROM t WHERE cond RETURNING *` |
| RETURNING INTO | `DELETE FROM t WHERE cond RETURNING * INTO variables` |
| WHERE CURRENT OF | `DELETE FROM t WHERE CURRENT OF cursor_name` |
| WITH (CTE) | `WITH cte AS (...) DELETE FROM t WHERE ...` |
| 分区指定 | `DELETE FROM t PARTITION (p1) WHERE ...` |

### MERGE

| 特性 | 示例语法 |
|------|----------|
| MERGE INTO | `MERGE INTO target t USING source s ON (t.id = s.id) ...` |
| WHEN MATCHED UPDATE | `WHEN MATCHED THEN UPDATE SET col = val [WHERE cond]` |
| WHEN MATCHED DELETE | `WHEN MATCHED THEN DELETE [WHERE cond]` |
| WHEN NOT MATCHED INSERT | `WHEN NOT MATCHED THEN INSERT (...) VALUES (...) [WHERE cond]` |
| 语义校验 | 内置检测非确定性和无效 MERGE 模式 |
| 分区指定 | 支持目标表和源表的分区指定 |

### VALUES

`VALUES (1, 'a'), (2, 'b')` 作为独立语句

---

## 2. DDL 语句

### 2.1 CREATE 语句

#### 表与索引

| 语句 | 说明 |
|------|------|
| `CREATE TABLE` | 完整支持：列定义、约束（PK/UNIQUE/CHECK/FK/NOT NULL/DEFAULT/GENERATED）、表选项、分区（RANGE/LIST/HASH）、子分区、分布策略（HASH/REPLICATION/ROUNDROBIN/MODULO）、LIKE 子句、INHERITS、ON COMMIT、TABLESPACE、压缩、ILM 策略、行迁移、字符集 |
| `CREATE TABLE AS` | `CREATE TABLE ... AS SELECT ... [WITH [NO] DATA]` |
| `CREATE TEMPORARY/UNLOGGED TABLE` | 临时表和 UNLOGGED 表 |
| `CREATE FOREIGN TABLE` | 外部表定义，含 SERVER 和 OPTIONS |
| `CREATE INDEX` | 索引创建：UNIQUE、IF NOT EXISTS、WHERE 条件、INCLUDE 列、排序方向、NULLS 排序、TABLESPACE、COLLATE、OPCLASS |
| `CREATE GLOBAL INDEX` | 分布式表的全局索引 |
| `CREATE TABLESPACE` | 表空间管理 |

#### 视图

| 语句 | 说明 |
|------|------|
| `CREATE VIEW` | 视图定义，含 WITH CHECK OPTION、RECURSIVE、SECURITY |
| `CREATE MATERIALIZED VIEW` | 物化视图，含 WITH DATA / WITH NO DATA、INCREMENTAL |
| `REFRESH MATERIALIZED VIEW` | 刷新物化视图，含 CONCURRENTLY |

#### 函数与过程

| 语句 | 说明 |
|------|------|
| `CREATE FUNCTION` | 完整支持：参数模式（IN/OUT/INOUT/VARIADIC）、返回类型（含 RETURNS TABLE/SETOF）、LANGUAGE、IMMUTABLE/STABLE/VOLATILE、STRICT/CALELD ON NULL INPUT、SECURITY DEFINER/INVOKER、SHIPPABLE、COST/ROWS、PARALLEL、PL/pgSQL 体 |
| `CREATE PROCEDURE` | 完整支持：参数、PL/pgSQL 体、LANGUAGE |
| `CREATE AGGREGATE` | 自定义聚合函数 |
| `CREATE OPERATOR` | 自定义操作符 |

#### 包（Oracle 兼容）

| 语句 | 说明 |
|------|------|
| `CREATE PACKAGE` | 包规范：过程/函数声明、游标声明、变量声明 |
| `CREATE PACKAGE BODY` | 包体：过程/函数实现 |

#### 触发器

| 语句 | 说明 |
|------|------|
| `CREATE TRIGGER` | BEFORE/AFTER/INSTEAD OF，事件（INSERT/UPDATE/DELETE/TRUNCATE），FOR EACH ROW/STATEMENT，WHEN 条件，EXECUTE FUNCTION/PROCEDURE，CONSTRAINT TRIGGER |

#### 对象与类型

| 语句 | 说明 |
|------|------|
| `CREATE TYPE` | 复合类型、枚举类型、范围类型、基础类型、TABLE OF、VARRAY |
| `CREATE DOMAIN` | 带约束的域类型 |
| `CREATE CAST` | 类型转换定义（IMPLICIT/EXPLICIT/ASSIGNMENT） |
| `CREATE SEQUENCE` | 序列：START/INCREMENT/MINVALUE/MAXVALUE/CACHE/CYCLE/OWNED BY |
| `CREATE OPERATOR` | 自定义操作符 |
| `CREATE OPCLASS / OPFAMILY` | 操作符类和操作符族 |
| `CREATE CONVERSION` | 字符集转换 |

#### 外部数据

| 语句 | 说明 |
|------|------|
| `CREATE FOREIGN SERVER` | 外部服务器 |
| `CREATE FOREIGN DATA WRAPPER` | 外部数据包装器 |
| `CREATE USER MAPPING` | 用户映射 |
| `CREATE DATABASE LINK` | 数据库链接 |

#### 复制与安全

| 语句 | 说明 |
|------|------|
| `CREATE PUBLICATION` | 逻辑复制发布 |
| `CREATE SUBSCRIPTION` | 逻辑复制订阅 |
| `CREATE ROLE / USER / GROUP` | 角色和用户管理（含 SUPERUSER/CREATEDB/CREATEROLE/LOGIN/PASSWORD/VALID UNTIL/CONNECTION LIMIT 等） |
| `CREATE RLS POLICY` | 行级安全策略 |
| `CREATE AUDIT POLICY` | 审计策略 |
| `CREATE MASKING POLICY` | 数据脱敏策略 |

#### GaussDB 专有 CREATE

| 语句 | 说明 |
|------|------|
| `CREATE NODE / NODE GROUP` | 分布式节点管理 |
| `CREATE RESOURCE POOL` | 资源池 |
| `CREATE WORKLOAD GROUP` | 工作负载组 |
| `CREATE DATA SOURCE` | 数据源 |
| `CREATE EVENT` | 事件 |
| `CREATE DIRECTORY` | 目录对象（Oracle 兼容） |
| `CREATE SYNONYM` | 同义词（Oracle 兼容） |
| `CREATE MODEL` | AI 模型（配合 PREDICT BY 使用） |
| `CREATE STREAM / CONTINUOUS QUERY` | 流式计算 |
| `CREATE KEY` | 密钥管理 |
| `CREATE LANGUAGE` | 过程语言 |
| `CREATE EXTENSION` | 扩展管理 |
| `CREATE SCHEMA` | 模式创建 |
| `CREATE DATABASE` | 数据库创建 |
| `CREATE TEXT SEARCH CONFIG/DICT` | 全文搜索配置 |
| `CREATE APP WORKLOAD GROUP MAPPING` | 应用工作负载组映射 |
| `CREATE WEAK PASSWORD DICTIONARY` | 弱口令字典 |
| `CREATE ACCESS METHOD` | 访问方法 |
| `CREATE RULE` | 规则 |

### 2.2 ALTER 语句

| 语句 | 说明 |
|------|------|
| `ALTER TABLE` | 列操作（ADD/DROP/ALTER/RENAME COLUMN）、约束操作（ADD/DROP/VALIDATE）、分区操作（ADD/DROP/MERGE/SPLIT/EXCHANGE/TRUNCATE/MOVE PARTITION）、表选项（SET SCHEMA/OWNER/TABLESPACE/OPTIONS）、行级安全、行迁移、压缩、ILM 策略、节点操作 |
| `ALTER INDEX` | 索引修改：重命名、REBUILD PARTITION、MOVE PARTITION、SET TABLESPACE、SET/RESET OPTIONS、UNUSABLE |
| `ALTER SEQUENCE` | 序列修改：RESTART、INCREMENT、MINVALUE/MAXVALUE、CACHE、CYCLE、OWNED BY |
| `ALTER VIEW` | 视图修改：RENAME、ALTER COLUMN SET DEFAULT、OWNER TO、SET SCHEMA |
| `ALTER MATERIALIZED VIEW` | 物化视图修改：RENAME、OWNER、SCHEMA、TABLESPACE |
| `ALTER FUNCTION` | 函数修改：RENAME、OWNER、SCHEMA、IMMUTABLE/STABLE/VOLATILE、LEAKPROOF、STRICT、SHIPPABLE、PACKAGE、COMPILE、SET/RESET |
| `ALTER PROCEDURE` | 存储过程修改（同 ALTER FUNCTION） |
| `ALTER TRIGGER` | 触发器修改：RENAME TO |
| `ALTER SCHEMA` | 模式修改：RENAME TO、OWNER TO、CHARACTER SET |
| `ALTER DATABASE` | 数据库修改：RENAME、OWNER、TABLESPACE、CONNECTION LIMIT、SET/RESET、ENABLE PRIVATE OBJECT |
| `ALTER TABLESPACE` | 表空间修改：RENAME、OWNER、SET/RESET OPTIONS |
| `ALTER ROLE / USER / GROUP` | 角色修改：PASSWORD、SUPERUSER、CREATEDB、LOGIN、INHERIT、RENAME、VALID UNTIL 等 |
| `ALTER EXTENSION` | 扩展修改：UPDATE、ADD/DROP OBJECT、SET SCHEMA |
| `ALTER TYPE` | 类型修改：ADD/DROP/RENAME ATTRIBUTE |
| `ALTER DOMAIN` | 域修改：SET DEFAULT、DROP NOT NULL、ADD/DROP CONSTRAINT |
| `ALTER FOREIGN TABLE` | 外部表修改：OPTIONS、SERVER、SCHEMA、OWNER |
| `ALTER FOREIGN SERVER` | 外部服务器修改：VERSION、OPTIONS |
| `ALTER FOREIGN DATA WRAPPER` | FDW 修改：HANDLER、VALIDATOR、OPTIONS |
| `ALTER PUBLICATION` | 发布修改：ADD/DROP/SET TABLES、RENAME、OWNER |
| `ALTER SUBSCRIPTION` | 订阅修改：CONNECTION、REFRESH、PUBLICATION、SLOT |
| `ALTER USER MAPPING` | 用户映射修改：OPTIONS |
| `ALTER SYSTEM` | 系统参数设置：SET/RESET |
| `ALTER SYSTEM KILL SESSION` | 终止会话 |
| `ALTER DEFAULT PRIVILEGES` | 默认权限 |
| `ALTER OP FAMILY` | 操作符族修改 |
| `ALTER OPERATOR` | 操作符修改 |

**GaussDB 专有 ALTER**：NODE、NODE GROUP、RESOURCE POOL、WORKLOAD GROUP、AUDIT/MASKING/RLS POLICY、DATA SOURCE、EVENT、GLOBAL CONFIG、SESSION、DATABASE LINK、DIRECTORY、LANGUAGE、PACKAGE、COORDINATOR、APP WORKLOAD GROUP MAPPING、SYNONYM、TEXT SEARCH CONFIG/DICT

### 2.3 DROP 语句

支持 30+ 种对象类型的 DROP：

`TABLE`, `INDEX`, `VIEW`, `MATERIALIZED VIEW`, `FUNCTION`, `PROCEDURE`, `TRIGGER`, `SCHEMA`, `DATABASE`, `TABLESPACE`, `SEQUENCE`, `TYPE`, `DOMAIN`, `CAST`, `EXTENSION`, `ROLE`, `USER`, `GROUP`, `FOREIGN TABLE`, `SERVER`, `FOREIGN DATA WRAPPER`, `PUBLICATION`, `SUBSCRIPTION`, `USER MAPPING`, `RULE`, `PACKAGE`, `PACKAGE BODY`, `SYNONYM`, `POLICY`, `NODE`, `NODE GROUP`, `CONVERSION`, `LANGUAGE`, `AGGREGATE`, `OPERATOR`, `OP CLASS`, `OP FAMILY`, `DIRECTORY`, `TEXT SEARCH CONFIG`, `TEXT SEARCH DICT`, `DATA SOURCE`, `EVENT`, `MODEL`, `RESOURCE POOL`, `WORKLOAD GROUP`, `AUDIT POLICY`, `MASKING POLICY`, `RLS POLICY`, `WEAK PASSWORD DICTIONARY`, `POLICY LABEL`, `APP WORKLOAD GROUP MAPPING`, `DATABASE LINK`

所有 DROP 均支持 `IF EXISTS` 和 `CASCADE/RESTRICT`。

---

## 3. PL/pgSQL 过程化语言

### 块结构

| 特性 | 语法 |
|------|------|
| DO 语句 | `DO [LANGUAGE lang] $$ ... $$` / `DO $tag$ ... $tag$` |
| 匿名块 | `[DECLARE] BEGIN ... END` |
| 子块 | 嵌套的 `BEGIN ... END` 块，带可选标签 `<<label>>` |

### 声明

| 特性 | 语法 |
|------|------|
| 变量声明 | `varname datatype [CONSTANT] [NOT NULL] [:= expr] [COLLATE collation]` |
| 游标声明 | `CURSOR [(params)] [SCROLL/NO SCROLL] FOR/IS SELECT ...` |
| RECORD 类型 | `RECORD` |
| TYPE 定义 | `TYPE name IS RECORD (...)` / `TABLE OF ...` / `VARRAY(n) OF ...` / `REF CURSOR` |
| %TYPE | 列类型引用：`table.column%TYPE` |
| %ROWTYPE | 表行类型引用：`table%ROWTYPE` |
| 异常声明 | `exception_name EXCEPTION` |
| PRAGMA | `PRAGMA name (args)` |
| 嵌套过程/函数 | `PROCEDURE name IS ...` / `FUNCTION name RETURN type IS ...` |

### 控制流

| 特性 | 语法 |
|------|------|
| IF/ELSIF/ELSE | `IF cond THEN ... ELSIF cond THEN ... ELSE ... END IF` |
| CASE | `CASE [expr] WHEN val THEN ... ELSE ... END CASE` |
| LOOP | `[<<label>>] LOOP ... END LOOP [label]` |
| WHILE | `WHILE cond LOOP ... END LOOP` |
| FOR (整数) | `FOR i IN [REVERSE] start..end [BY step] LOOP ... END LOOP` |
| FOR (查询) | `FOR rec IN SELECT ... LOOP ... END LOOP` |
| FOR (动态 SQL) | `FOR rec IN EXECUTE expr [USING args] LOOP ... END LOOP` |
| FOR (游标) | `FOR rec IN cursor_name[(args)] LOOP ... END LOOP` |
| FOREACH | `FOREACH item [SLICE n] IN ARRAY arr LOOP ... END LOOP` |
| EXIT | `EXIT [label] [WHEN cond]` |
| CONTINUE | `CONTINUE [label] [WHEN cond]` |
| RETURN | `RETURN [expr]` |
| RETURN NEXT | `RETURN NEXT expr` |
| RETURN QUERY | `RETURN QUERY [EXECUTE] query [USING args]` |
| GOTO | `GOTO label` |
| NULL | `NULL;` (空语句) |

### 动态 SQL

| 特性 | 语法 |
|------|------|
| EXECUTE IMMEDIATE | `EXECUTE [IMMEDIATE] sql_string [INTO vars] [USING IN/OUT/INOUT params]` |
| PERFORM | `PERFORM expr` (执行并丢弃结果) |
| EXECUTE (PL) | `EXECUTE sql_expr` |
| CALL | `CALL procedure_name(args)` |

### 游标操作

| 特性 | 语法 |
|------|------|
| OPEN | `OPEN cursor [FOR SELECT ...]` / `OPEN cursor FOR EXECUTE expr [USING args]` |
| FETCH | `FETCH [direction] cursor [BULK COLLECT] INTO variables` |
| CLOSE | `CLOSE cursor` |
| MOVE | `MOVE [direction] cursor` |
| 游标属性 | `cursor%NOTFOUND`, `%FOUND`, `%ISOPEN`, `%ROWCOUNT`, `%BULK_EXCEPTIONS` |

**FETCH 方向**：`NEXT`、`PRIOR`、`FIRST`、`LAST`、`ABSOLUTE n`、`RELATIVE n`、`FORWARD [n/ALL]`、`BACKWARD [n/ALL]`、`ALL`

### 异常处理

| 特性 | 语法 |
|------|------|
| EXCEPTION 块 | `EXCEPTION WHEN exc_name THEN ...` |
| 多个处理器 | `WHEN ... THEN ... WHEN ... THEN ...` |
| GET DIAGNOSTICS | `GET [STACKED] DIAGNOSTICS var = item` |
| RAISE | `RAISE [level] 'format', args... [USING option = expr]` |
| RAISE 重新抛出 | `RAISE;` |

**GET DIAGNOSTICS 项目**：`ROW_COUNT`, `RESULT_STATUS`, `RETURNED_SQLSTATE`, `MESSAGE_TEXT`, `DETAIL`, `HINT`, `CONTEXT`, `SCHEMA_NAME`, `TABLE_NAME`, `COLUMN_NAME`, `DATATYPE_NAME`, `CONSTRAINT_NAME`, `PG_EXCEPTION_CONTEXT`

**RAISE 级别**：`DEBUG`, `LOG`, `INFO`, `NOTICE`, `WARNING`, `EXCEPTION`

### 事务控制

| 特性 | 语法 |
|------|------|
| COMMIT | `COMMIT [AND CHAIN]` |
| ROLLBACK | `ROLLBACK [AND CHAIN] [TO SAVEPOINT name]` |
| SAVEPOINT | `SAVEPOINT name` |
| RELEASE SAVEPOINT | `RELEASE SAVEPOINT name` |
| SET TRANSACTION | `SET TRANSACTION [ISOLATION LEVEL ...] [READ ONLY/WRITE] [DEFERRABLE]` |

**隔离级别**：`READ COMMITTED`, `REPEATABLE READ`, `SERIALIZABLE`

### 其他

| 特性 | 语法 |
|------|------|
| FORALL | `FORALL i IN start..end [SAVE EXCEPTIONS] ...` 批量操作 |
| PIPE ROW | `PIPE ROW (row_value)` |
| 赋值 | `variable := expression` |
| SELECT INTO | `SELECT cols INTO variables FROM ...` |
| BULK COLLECT | `SELECT ... BULK COLLECT INTO collection` |
| 变量 SET/RESET | `SET variable = value` / `RESET variable` |

---

## 4. 表达式与运算符

### 运算符

| 类别 | 运算符 |
|------|--------|
| 算术 | `+`, `-`, `*`, `/`, `%`, `^` (幂), `\|/` (平方根), `\|\|/` (立方根), `@` (绝对值) |
| 比较 | `=`, `<>`/`!=`, `<`, `>`, `<=`, `>=`, `IS DISTINCT FROM` |
| 逻辑 | `AND`, `OR`, `NOT` |
| 位运算 | `&`, `\|`, `#`, `~`, `<<`, `>>` |
| 字符串 | `\|\|` (连接) |
| 模式匹配 | `LIKE`, `NOT LIKE`, `ILIKE`, `NOT ILIKE`, `SIMILAR TO`, `NOT SIMILAR TO` |
| IS 运算 | `IS NULL`, `IS NOT NULL`, `IS TRUE`, `IS FALSE`, `IS UNKNOWN`, `IS DISTINCT FROM`, `IS OF` |
| 成员测试 | `IN`, `NOT IN`, `BETWEEN`, `NOT BETWEEN` |
| 存在性 | `EXISTS`, `ANY`, `SOME`, `ALL` |
| 类型转换 | `::` (PostgreSQL 风格), `CAST(... AS ...)` |
| JSON | `->`, `->>`, `#>`, `#>>` |

### 表达式类型

| 类型 | 示例 |
|------|------|
| 字面量 | `42`, `3.14`, `'text'`, `TRUE`, `NULL` |
| 特殊字面量 | `E'...'` (转义串), `B'...'` (位串), `X'...'` (十六进制), `N'...'` (国际化), `$$...$$` / `$tag$...$tag$` (美元引用) |
| 列引用 | `col`, `t.col`, `schema.table.col` |
| 限定星号 | `t.*` |
| 二元运算 | `a + b`, `c AND d` |
| 一元运算 | `-x`, `NOT x`, `~x` |
| 函数调用 | `count(*)`, `max(col)`, `COALESCE(a, b, c)` |
| 特殊函数 | `EXTRACT(...)`, `SUBSTRING(...)`, `TRIM(...)`, `OVERLAY(...)`, `POSITION(...)`, `CONVERT(... USING ...)` |
| CASE | `CASE WHEN ... THEN ... ELSE ... END` / `CASE expr WHEN ... THEN ... END` |
| BETWEEN | `x BETWEEN a AND b` |
| IN 列表 | `x IN (1, 2, 3)` |
| IN 子查询 | `x IN (SELECT ...)` |
| EXISTS | `EXISTS (SELECT ...)` |
| 子查询 | `(SELECT ...)` |
| 标量子链接 | `expr = ANY/SOME/ALL (SELECT ...)` |
| IS NULL / IS BOOLEAN | `x IS NULL`, `x IS TRUE`, `x IS FALSE` |
| 类型转换 | `x::type`, `CAST(x AS type)` |
| TREAT | `TREAT(x AS type)` |
| 参数 | `$1`, `$2`, ... |
| MyBatis 参数 | `#{param}`, `${expr}` |
| 数组构造 | `ARRAY[1, 2, 3]`, `ARRAY(SELECT ...)` |
| 数组下标 | `arr[i]`, `arr[lo:hi]` (切片) |
| 字段访问 | `rec.field` |
| 行构造器 | `ROW(val1, val2)` |
| PRIOR | `PRIOR col` (CONNECT BY) |
| DEFAULT | `DEFAULT` 关键字 |
| SYSDATE | `SYSDATE` (Oracle 兼容) |
| 序列函数 | `seq.NEXTVAL`, `seq.CURRVAL` |
| 游标属性 | `cursor%NOTFOUND`, `cursor%FOUND`, `cursor%ISOPEN`, `cursor%ROWCOUNT` |
| XML 表达式 | `XMLELEMENT`, `XMLCONCAT`, `XMLFOREST`, `XMLPARSE`, `XMLPI`, `XMLROOT`, `XMLSERIALIZE` |
| PREDICT BY | `PREDICT BY model_name (FEATURES col1, col2)` |

### 窗口函数

| 特性 | 语法 |
|------|------|
| OVER 子句 | `func() OVER (...)` |
| PARTITION BY | `PARTITION BY expr, ...` |
| ORDER BY | `ORDER BY expr ASC/DESC` |
| 帧规范 | `ROWS/RANGE/GROUPS BETWEEN bound AND bound` |
| 帧边界 | `UNBOUNDED PRECEDING/FOLLOWING`, `CURRENT ROW`, `n PRECEDING/FOLLOWING` |
| 命名窗口 | `WINDOW w AS (...)` |
| FILTER | `func() FILTER (WHERE cond) OVER (...)` |
| WITHIN GROUP | `func() WITHIN GROUP (ORDER BY ...)` |

---

## 5. 数据类型

### 数值类型

| 类型 | 说明 |
|------|------|
| `BOOLEAN` | 布尔 |
| `TINYINT` | 1 字节整数 |
| `SMALLINT` | 2 字节整数 |
| `INTEGER` / `INT` | 4 字节整数 |
| `BIGINT` | 8 字节整数 |
| `SERIAL` / `SMALLSERIAL` / `BIGSERIAL` | 自增整数 |
| `REAL` | 单精度浮点 |
| `FLOAT(n)` | 浮点数 |
| `DOUBLE PRECISION` | 双精度浮点 |
| `NUMERIC(p, s)` / `DECIMAL(p, s)` | 精确数值 |
| `BINARY_FLOAT` / `BINARY_DOUBLE` | Oracle 二进制浮点 |

### 字符串类型

| 类型 | 说明 |
|------|------|
| `CHAR(n)` | 定长字符 |
| `VARCHAR(n)` | 变长字符 |
| `TEXT` | 无限制文本 |
| `BYTEA` | 二进制数据 |
| `VARCHAR2` / `NVARCHAR2` | Oracle 字符类型 |

### 日期时间类型

| 类型 | 说明 |
|------|------|
| `DATE` | 日期 |
| `TIME` / `TIME WITH TIME ZONE` | 时间 |
| `TIMESTAMP` / `TIMESTAMP WITH TIME ZONE` | 时间戳 |
| `INTERVAL` | 时间间隔 |

### 特殊类型

| 类型 | 说明 |
|------|------|
| `JSON` / `JSONB` | JSON 数据 |
| `UUID` | UUID |
| `BIT(n)` / `BIT VARYING(n)` | 位串 |
| `CIDR` / `INET` / `MACADDR` / `MACADDR8` | 网络地址 |
| `ARRAY` | 数组 |
| `RECORD` | 记录 |
| `CLOB` / `BLOB` | Oracle 大对象 |
| `RAW` | Oracle 原始二进制 |
| `SYS_REFCURSOR` | Oracle 系统游标 |
| 自定义复合类型 | `CREATE TYPE ... AS (...)` |
| 枚举类型 | `CREATE TYPE ... AS ENUM (...)` |
| 范围类型 | `int4range`, `daterange`, 自定义 |
| 域类型 | `CREATE DOMAIN ...` |

---

## 6. 内置函数

解析器注册了 **449 个内置函数**，按类别和领域分类：

### 按类别

| 类别 | 说明 | 代表函数 |
|------|------|----------|
| Aggregate | 聚合函数 | `count`, `sum`, `avg`, `min`, `max`, `array_agg`, `string_agg`, `json_agg`, `jsonb_agg`, `xmlagg`, `median`, `mode`, `percentile_cont`, `percentile_disc`, `listagg`, `group_concat`, `wm_concat` |
| Window | 窗口函数 | `row_number`, `rank`, `dense_rank`, `lead`, `lag`, `first_value`, `last_value`, `nth_value`, `percent_rank`, `cume_dist`, `ntile`, `ratio_to_report` |
| Scalar | 标量函数 | 数学、字符串、日期时间、系统等 |
| SetReturning | 集合返回函数 | `generate_series`, `json_each`, `jsonb_each`, `unnest`, `regexp_split_to_table` |
| Special | 特殊函数 | `coalesce`, `nullif`, `greatest`, `least`, `decode` |
| TypeConstructor | 类型构造函数 | `json`, `jsonb` |

### 按领域

| 领域 | 函数数量 | 代表函数 |
|------|----------|----------|
| Math (数学) | ~30 | `abs`, `ceil`, `floor`, `round`, `trunc`, `sqrt`, `power`, `log`, `exp`, `sin`, `cos`, `tan`, `atan2`, `degrees`, `radians`, `gcd`, `factorial`, `div`, `mod`, `sign`, `width_bucket` |
| String (字符串) | ~30 | `concat`, `concat_ws`, `substr`/`substring`, `replace`, `trim`, `ltrim`, `rtrim`, `upper`, `lower`, `initcap`, `length`, `position`, `overlay`, `lpad`, `rpad`, `reverse`, `split_part`, `regexp_like`, `regexp_replace`, `regexp_substr`, `regexp_count`, `regexp_instr`, `instr`, `left`, `right`, `repeat`, `translate`, `encode`, `decode` |
| DateTime (日期时间) | ~25 | `current_date`, `current_time`, `current_timestamp`, `now`, `age`, `date_part`, `date_trunc`, `extract`, `clock_timestamp`, `to_char`, `to_date`, `to_timestamp`, `to_number`, `add_months`, `last_day`, `next_day`, `months_between`, `trunc(date)`, `sysdate`, `justify_days`, `justify_hours`, `make_date`, `make_time`, `make_timestamp` |
| Aggregate (聚合) | ~25 | `count`, `sum`, `avg`, `min`, `max`, `stddev`, `variance`, `corr`, `covar_pop`, `covar_samp`, `regr_*` 系列, `percentile_cont`, `percentile_disc`, `mode`, `rank`, `dense_rank`, `group_concat`, `listagg` |
| Array (数组) | ~10 | `array_append`, `array_length`, `array_to_string`, `array_to_json`, `unnest`, `string_to_array`, `generate_subscripts` |
| JSON | ~30 | `json_agg`, `jsonb_agg`, `json_array_elements`, `jsonb_array_elements`, `json_each`, `jsonb_each`, `json_object_keys`, `jsonb_object_keys`, `json_typeof`, `jsonb_typeof`, `jsonb_pretty`, `jsonb_set`, `jsonb_insert`, `jsonb_delete`, `json_build_array`, `jsonb_build_array`, `json_build_object`, `jsonb_build_object`, `to_json`, `to_jsonb`, `row_to_json`, `json_populate_record`, `json_strip_nulls`, `xpath`, `xpath_exists` |
| Network (网络) | ~10 | `abbrev`, `broadcast`, `family`, `host`, `hostmask`, `masklen`, `netmask`, `network`, `set_masklen` |
| Geometric (几何) | ~5 | `area`, `center`, `circle`, `diameter`, `point`, `polygon` |
| Hash (哈希) | ~3 | `crc32`, `md5` |
| Crypto (加密) | ~8 | `digest`, `gen_random_uuid`, `gs_encrypt`, `gs_decrypt`, `gs_encrypt_aes128`, `gs_decrypt_aes128` |
| System (系统) | ~15 | `current_database`, `current_schema`, `current_user`, `current_setting`, `format_type`, `col_description`, `has_table_privilege`, `has_schema_privilege`, `inet_client_addr`, `inet_server_addr`, `version`, `set_config`, `pg_backend_pid` |
| TextSearch (全文搜索) | ~5 | `get_current_ts_config`, `to_tsvector`, `to_tsquery`, `plainto_tsquery`, `ts_rank`, `ts_headline` |
| TypeConversion (类型转换) | ~5 | `convert`, `convert_from`, `convert_to`, `to_char`, `to_number` |
| OracleCompat (Oracle 兼容) | ~15 | `decode`, `nvl`, `nvl2`, `add_months`, `last_day`, `next_day`, `months_between`, `instr`, `instrb`, `substrb`, `nls_initcap`, `nls_lower`, `nls_upper`, `nlssort`, `nls_sort`, `listagg`, `wm_concat`, `group_concat` |

### Oracle 兼容包函数

| 包名 | 函数 |
|------|------|
| DBE_FILE | `open`, `close`, `read_line`, `write_line`, `copy`, `remove`, `rename` |
| DBE_LOB | `append`, `compare`, `copy`, `createtemporary`, `erase`, `freetemporary`, `getlength`, `instr`, `read`, `substr`, `trim`, `write` |
| DBE_OUTPUT | `disable`, `enable`, `get_line`, `get_lines`, `new_line`, `print`, `put`, `put_line` |
| DBE_SCHEDULER | `create_job`, `drop_job`, `run_job` |
| DBE_SESSION | `clear_context`, `set_context` |
| DBE_SQL | `close_cursor`, `column_value`, `execute`, `fetch_rows`, `open_cursor`, `register_variable` |
| DBE_STATS | `lock_table_stats`, `unlock_table_stats` |
| DBE_UTILITY | `format_error_backtrace`, `format_error_stack`, `get_time` |
| DBMS_LOB | `append`, `read`, `substr`, `write` |
| DBMS_OUTPUT | `disable`, `enable`, `put`, `put_line` |
| DBMS_SCHEDULER | `create_job`, `drop_job`, `run_job` |
| DBMS_SQL | `close_cursor`, `column_value`, `execute`, `fetch_rows`, `open_cursor` |
| DBMS_UTILITY | `format_error_backtrace`, `get_time` |
| UTL_FILE | 文件操作函数 |
| PKG_SERVICE | 服务包函数 |
| XML | XML 处理函数 |

---

## 7. GaussDB 专有扩展

### 分布式特性

| 特性 | 说明 |
|------|------|
| DISTRIBUTE BY | 分布策略：HASH / REPLICATION / ROUNDROBIN / MODULO |
| SUBPARTITION | 子分区定义 |
| NODE / NODE GROUP | 分布式节点和节点组管理 |
| GLOBAL INDEX | 全局索引（跨节点） |
| CLEAN CONNECTION | 清理连接 |
| EXECUTE DIRECT | 直接在指定节点执行 SQL |
| BARRIER | 分布式一致性屏障 |
| LOCK BUCKETS / MARK BUCKETS | 桶级锁操作 |

### 安全特性

| 特性 | 说明 |
|------|------|
| RLS POLICY | 行级安全策略（CREATE/ALTER/DROP） |
| AUDIT POLICY | 审计策略（CREATE/ALTER） |
| MASKING POLICY | 数据脱敏策略（CREATE/ALTER） |
| WEAK PASSWORD DICTIONARY | 弱口令字典管理 |
| SECURITY LABEL | 安全标签 |
| POLICY LABEL | 策略标签管理 |

### 资源管理

| 特性 | 说明 |
|------|------|
| RESOURCE POOL | 资源池管理 |
| WORKLOAD GROUP | 工作负载组管理 |
| APP WORKLOAD GROUP MAPPING | 应用工作负载组映射 |
| COORDINATOR | 协调器管理 |

### AI 特性

| 特性 | 说明 |
|------|------|
| CREATE MODEL | AI 模型创建和管理 |
| PREDICT BY | AI 预测表达式 |

### 运维特性

| 特性 | 说明 |
|------|------|
| SHUTDOWN | 关闭数据库 |
| BARRIER | 一致性屏障 |
| PURGE | 清理回收站 |
| TIMECAPSULE | 时间胶囊（闪回查询），支持 TIMESTAMP 和 CSN |
| SNAPSHOT | 快照管理 |
| SHRINK | 收缩表空间 |
| VERIFY | 数据校验 |
| CHECKPOINT | 检查点 |
| ALTER SYSTEM KILL SESSION | 终止会话 |
| ALTER SESSION | 会话参数设置 |
| COMPILE | 重编译对象 |

### 数据导入导出

| 特性 | 说明 |
|------|------|
| COPY | 数据导入导出，支持 CSV/BINARY/FIXED 格式 |
| EXPDP DATABASE/TABLE | 数据库/表导出 |
| IMPDP DATABASE/TABLE | 数据库/表导入 |

### 其他专有特性

| 特性 | 说明 |
|------|------|
| DATA SOURCE | 数据源管理 |
| EVENT | 事件管理 |
| DIRECTORY | 目录对象 |
| KEY | 密钥管理 |
| STREAM / CONTINUOUS QUERY | 流式计算 |
| GLOBAL CONFIG | 全局配置 |
| TEXT SEARCH CONFIG/DICT | 全文搜索配置 |
| WEAK PASSWORD DICTIONARY | 弱口令字典 |

---

## 8. Oracle 兼容特性

OGSQL Parser 通过 GaussDB 的 A_FORMAT 和 B_FORMAT 兼容模式，支持以下 Oracle 兼容特性：

### 数据类型

- `VARCHAR2` / `NVARCHAR2`
- `NUMBER(p, s)`
- `DATE` (含时间)
- `CLOB` / `BLOB`
- `RAW`
- `SYS_REFCURSOR`
- `BINARY_FLOAT` / `BINARY_DOUBLE`

### SQL 语法

| 特性 | 语法 |
|------|------|
| DECODE | `DECODE(expr, val1, result1, val2, result2, default)` |
| NVL / NVL2 | `NVL(expr, default)` / `NVL2(expr, val1, val2)` |
| SYSDATE | `SYSDATE` 获取当前日期时间 |
| ROWNUM | `ROWNUM` 伪列 |
| CONNECT BY | 层级查询 `CONNECT BY [NOCYCLE] PRIOR ... START WITH ...` |
| INSERT ALL / FIRST | 多表插入 |
| REPLACE INTO | 替换插入 |
| GROUP_CONCAT | `GROUP_CONCAT(expr ORDER BY ... SEPARATOR ...)` |
| LISTAGG | `LISTAGG(expr, delimiter) WITHIN GROUP (ORDER BY ...)` |
| MINUS | `MINUS` 等价于 `EXCEPT` |

### PL/pgSQL Oracle 兼容

| 特性 | 说明 |
|------|------|
| PACKAGE / PACKAGE BODY | Oracle 包规范和包体 |
| SYNONYM | 同义词 |
| FORALL | 批量操作 |
| PIPE ROW | 管道行输出 |
| BULK COLLECT INTO | 批量收集 |
| EXECUTE IMMEDIATE | 动态 SQL |
| DBMS_* / DBE_* | Oracle 兼容包函数 |
| UTL_FILE | 文件操作包 |
| 嵌套过程/函数 | 过程/函数内嵌套定义 |

---

## 9. 其他语句

### 查询相关

| 语句 | 说明 |
|------|------|
| `EXPLAIN` / `EXPLAIN ANALYZE` | 执行计划，支持 VERBOSE、PERFORMANCE、FORMAT、COSTS、BUFFERS 等 |
| `EXPLAIN PLAN` | `EXPLAIN PLAN [SET STATEMENT_ID = id] FOR statement` |
| `PREPARE` / `EXECUTE` / `DEALLOCATE` | 预编译语句 |

### 事务控制

| 语句 | 说明 |
|------|------|
| `BEGIN` / `START TRANSACTION` | 事务开始，支持隔离级别、READ ONLY/WRITE、DEFERRABLE |
| `COMMIT` / `ROLLBACK` | 提交/回滚，支持 AND CHAIN |
| `SAVEPOINT` / `RELEASE SAVEPOINT` | 保存点 |
| `SET TRANSACTION` | 设置事务特性 |
| `ABORT` | 中止事务 |

### 游标

| 语句 | 说明 |
|------|------|
| `DECLARE CURSOR` | 游标声明（SCROLL/NO SCROLL、WITH HOLD/WITHOUT HOLD） |
| `FETCH` | 获取游标数据（支持多种方向） |
| `CLOSE` | 关闭游标 |
| `MOVE` | 移动游标位置 |

### 维护

| 语句 | 说明 |
|------|------|
| `VACUUM` / `VACUUM FULL` | 清理（支持 VERBOSE、ANALYZE、FREEZE） |
| `ANALYZE` | 统计信息收集 |
| `REINDEX` | 重建索引 |
| `CLUSTER` | 聚集表 |

### 系统管理

| 语句 | 说明 |
|------|------|
| `SET` / `SHOW` / `RESET` | 参数管理（LOCAL/SESSION/GLOBAL） |
| `DISCARD` | 丢弃会话状态（ALL/PLANS/SEQUENCES/TEMP） |
| `LISTEN` / `NOTIFY` / `UNLISTEN` | 通知机制 |
| `LOCK TABLE` | 表锁定（SHARE/EXCLUSIVE/ACCESS SHARE/ACCESS EXCLUSIVE 等模式） |
| `CHECKPOINT` | 检查点 |
| `CALL` | 调用存储过程（支持命名参数 `name := value`） |
| `COMMENT ON` | 注释对象 |
| `COPY` | 数据导入导出 |
| `GRANT` / `REVOKE` | 权限管理 |
| `GRANT ROLE` / `REVOKE ROLE` | 角色授权 |
| `SECURITY LABEL` | 安全标签 |
| `SET SESSION AUTHORIZATION` | 设置会话授权 |
| `REASSIGN OWNED` | 重新分配对象所有权 |

---

## 兼容性说明

- **openGauss 兼容**：解析器基于 openGauss gram.y（35,325 行）和 PL/pgSQL gram.y（15,770 行）构建，通过全部 1,409 个回归测试
- **GaussDB 兼容**：额外支持 GaussDB 专有的分布式、安全、AI、运维等特性（177+ 语句类型）
- **PostgreSQL 兼容**：由于 openGauss 基于 PostgreSQL，绝大多数 PostgreSQL SQL 语法被自然支持
- **Oracle 兼容**：通过 A_FORMAT/B_FORMAT 兼容模式支持 Oracle 语法（DECODE、SYSDATE、CONNECT BY、PACKAGE 等）
- **MyBatis 兼容**：支持 `#{param}` 和 `${expr}` 占位符语法

---

*文档版本 0.6.10 | 最后更新 2026-06-04*
