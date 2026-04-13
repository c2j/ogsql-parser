-- 来源: 3186_DBE_PROFILER.txt
-- SQL 数量: 32

DROP TABLE IF EXISTS t1 ;

CREATE TABLE t1 ( i int );

CREATE OR REPLACE PROCEDURE p1 () AS sql_stmt varchar2 ( 200 );

CREATE OR REPLACE PROCEDURE p2 () AS BEGIN p1 ();

CREATE OR REPLACE PROCEDURE p3 () AS BEGIN p2 ();

SELECT dbe_profiler . pl_start_profiling ( '123' );

CALL p3 ();

SELECT * FROM dbe_profiler . pl_profiling_functions ORDER BY run_id , funcoid ;

SELECT * FROM dbe_profiler . pl_profiling_details WHERE funcoid = 16770 ORDER BY run_id , funcoid , line # ;

SELECT * FROM dbe_profiler . pl_profiling_callgraph ORDER BY run_id , stack ;

SELECT step_name , loops_count FROM dbe_profiler . pl_profiling_trackinfo WHERE funcoid = 16770 ;

SELECT dbe_profiler . pl_clear_profiling ( '' );

SELECT step_name , loops_count FROM dbe_profiler . pl_profiling_trackinfo WHERE funcoid = 16770 ;

DROP TABLE t1 ;

CREATE TABLE t2 ( a int , b int );

CREATE OR REPLACE PROCEDURE autonomous ( a int , b int ) AS DECLARE num3 int : = a ;

CREATE OR REPLACE PROCEDURE autonomous_1 ( a int , b int ) AS DECLARE BEGIN dbe_output . print_line ( 'just no use call.' );

SELECT dbe_profiler . pl_start_profiling ( '100' );

CALL autonomous ( 11 , 22 );

SELECT * FROM dbe_profiler . pl_profiling_functions ORDER BY run_id , funcoid ;

SELECT * FROM dbe_profiler . pl_profiling_details ORDER BY run_id , funcoid , line # ;

SELECT * FROM dbe_profiler . pl_profiling_callgraph ORDER BY run_id , stack ;

SELECT * FROM dbe_profiler . pl_profiling_trackinfo ORDER BY run_id , funcoid ;

SELECT dbe_profiler . pl_start_profiling ( '101' );

CALL autonomous_1 ( 11 , 22 );

SELECT * FROM dbe_profiler . pl_profiling_functions ORDER BY run_id , funcoid ;

SELECT * FROM dbe_profiler . pl_profiling_details ORDER BY run_id , funcoid , line # ;

SELECT * FROM dbe_profiler . pl_profiling_callgraph ORDER BY run_id , stack ;

SELECT * FROM dbe_profiler . pl_profiling_trackinfo ORDER BY run_id , funcoid ;

SELECT dbe_profiler . pl_clear_profiling ( '' );

SELECT * FROM dbe_profiler . pl_profiling_functions ;

DROP TABLE t2 ;

