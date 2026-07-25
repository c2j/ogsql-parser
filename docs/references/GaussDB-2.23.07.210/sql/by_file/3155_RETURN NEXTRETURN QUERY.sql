-- 来源: 3155_RETURN NEXTRETURN QUERY.txt
-- SQL 数量: 7

DROP TABLE t1 ;

CREATE TABLE t1 ( a int );

INSERT INTO t1 VALUES ( 1 ),( 10 );

CREATE OR REPLACE FUNCTION fun_for_return_next () RETURNS SETOF t1 AS $$ DECLARE r t1 % ROWTYPE ;

call fun_for_return_next ();

CREATE OR REPLACE FUNCTION fun_for_return_query () RETURNS SETOF t1 AS $$ DECLARE r t1 % ROWTYPE ;

call fun_for_return_query ();

