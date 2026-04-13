-- 来源: 3044_EXPLAIN PLAN.txt
-- SQL 数量: 7

CREATE TABLE foo1(f1 int, f2 text, f3 text[]);

CREATE TABLE foo2(f1 int, f2 text, f3 text[]);

--执行explain plan。
EXPLAIN PLAN SET STATEMENT_ID = 'TPCH-Q4' FOR SELECT f1, count(*) FROM foo1 WHERE f1 > 1 AND f1 < 3 AND EXISTS (SELECT * FROM foo2) GROUP BY f1;

SELECT * FROM plan_table;

DELETE FROM plan_table WHERE STATEMENT_ID = 'TPCH-Q4' ;

DROP TABLE foo1 ;

DROP TABLE foo2 ;

