-- 来源: 3929_DBE_PLDEBUGGER Schema.txt
-- SQL 数量: 17

CREATE OR REPLACE PROCEDURE test_debug ( IN x INT ) AS BEGIN INSERT INTO t1 ( a ) VALUES ( x );

SELECT OID FROM PG_PROC WHERE PRONAME = 'test_debug' ;

SELECT * FROM DBE_PLDEBUGGER . turn_on ( 16389 );

call test_debug ( 1 );

SELECT * FROM DBE_PLDEBUGGER . attach ( 'datanode' , 0 );

SELECT * FROM DBE_PLDEBUGGER . next ();

SELECT * FROM DBE_PLDEBUGGER . info_locals ();

SELECT * FROM DBE_PLDEBUGGER . set_var ( 'x' , 2 );

SELECT * FROM DBE_PLDEBUGGER . print_var ( 'x' );

SELECT * FROM DBE_PLDEBUGGER . continue ();

SELECT * FROM DBE_PLDEBUGGER . continue ();

SELECT * FROM DBE_PLDEBUGGER . error_end ();

SELECT * FROM DBE_PLDEBUGGER . abort ();

SELECT * FROM DBE_PLDEBUGGER . info_code ( 16389 );

SELECT * FROM DBE_PLDEBUGGER . add_breakpoint ( 16389 , 4 );

SELECT * FROM DBE_PLDEBUGGER . info_breakpoints ();

SELECT * FROM DBE_PLDEBUGGER . continue ();

