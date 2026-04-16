-- 来源: 1333_EXPLAIN PLAN.txt
-- SQL 数量: 18

CREATE TABLE foo1 ( f1 int , f2 text , f3 text []);

CREATE TABLE foo2 ( f1 int , f2 text , f3 text []);

EXPLAIN PLAN SET STATEMENT_ID = 'TPCH-Q4' FOR SELECT f1 , count ( * ) FROM foo1 WHERE f1 > 1 AND f1 < 3 AND EXISTS ( SELECT * FROM foo2 ) GROUP BY f1 ;

SELECT * FROM plan_table;

DELETE FROM plan_table WHERE STATEMENT_ID = 'TPCH-Q4' ;

DROP TABLE foo1 ;

DROP TABLE foo2 ;

CREATE TABLE pt_t1 ( a integer , b int , c int ) WITH ( autovacuum_enabled = off ) DISTRIBUTE hash ( c );

CREATE TABLE pt_t1 ( a int , b int , c int ) WITH ( autovacuum_enabled = off ) DISTRIBUTE hash ( c );

EXPLAIN PLAN SET statement_id = 'test remote query' FOR SELECT current_user FROM pt_t1 , pt_t2 ;

SELECT * FROM plan_table ;

DROP TABLE pt_t1 ;

DROP TABLE pg_t2 ;

SET enable_stream_recursive = off ;

CREATE TABLE chinamap ( id integer , pid integer , name text ) DISTRIBUTE BY hash ( id );

EXPLAIN PLAN SET statement_id = 'cte can not be push down' FOR WITH RECURSIVE rq AS ( SELECT id , name FROM chinamap WHERE id = 11 UNION ALL SELECT origin . id , rq . name || ' > ' || origin . name FROM rq JOIN chinamap origin ON origin . pid = rq . id ) SELECT id , name FROM rq ORDER BY 1 ;

SELECT * FROM plan_table ;

DROP TABLE chinamap ;

