-- 来源: 1284_DEALLOCATE.txt
-- SQL 数量: 9

SELECT name , statement , parameter_types FROM pg_prepared_statements ;

PREPARE q1 AS SELECT 1 AS a ;

PREPARE q2 AS SELECT 1 AS a ;

PREPARE q3 AS SELECT 1 AS a ;

SELECT name , statement , parameter_types FROM pg_prepared_statements ;

DEALLOCATE q1 ;

SELECT name , statement , parameter_types FROM pg_prepared_statements ;

DEALLOCATE ALL ;

SELECT name , statement , parameter_types FROM pg_prepared_statements ;

