-- 类别: PREPARED_STMT
-- SQL 数量: 46

-- 来源: 1015_Hint
deallocate all;

-- 来源: 1015_Hint
prepare p1 as insert /*+ no_gpc */ into t1 select c1,c2 from t2 where c1=$1;

-- 来源: 1015_Hint
execute p1(3);

-- 来源: 1284_DEALLOCATE
PREPARE q1 AS SELECT 1 AS a ;

-- 来源: 1284_DEALLOCATE
PREPARE q2 AS SELECT 1 AS a ;

-- 来源: 1284_DEALLOCATE
PREPARE q3 AS SELECT 1 AS a ;

-- 来源: 1284_DEALLOCATE
DEALLOCATE q1 ;

-- 来源: 1284_DEALLOCATE
DEALLOCATE ALL ;

-- 来源: 1328_EXECUTE
PREPARE insert_reason ( integer , character ( 16 ), character ( 100 )) AS INSERT INTO tpcds . reason_t1 VALUES ( $ 1 , $ 2 , $ 3 );

-- 来源: 1328_EXECUTE
EXECUTE insert_reason ( 52 , 'AAAAAAAADDAAAAAA' , 'reason 52' );

-- 来源: 1329_EXECUTE DIRECT
EXECUTE DIRECT ON ( dn_6001_6002 ) 'select count(*) from tpcds.customer_address' ;

-- 来源: 1329_EXECUTE DIRECT
EXECUTE DIRECT ON ( 16385 , 16386 , 16384 ) 'SELECT * FROM gs_get_listen_address_ext_info();

-- 来源: 1470_DBE_ILM
EXECUTE DIRECT ON DATANODES 'SELECT A.DBNAME, A.JOB_STATUS, A.ENABLE, A.FAILURE_MSG FROM PG_JOB A WHERE A.DBNAME = ''ilmtabledb'' AND A.JOB_NAME LIKE ''ilmjob$_%'' ORDER BY A.JOB_NAME DESC LIMIT 1' ;

-- 来源: 2719_Hint
deallocate all;

-- 来源: 2719_Hint
prepare p1 as insert /*+ no_gpc*/ into t1 select c1,c2 from t2 where c1=$1;

-- 来源: 2719_Hint
execute p1(3);

-- 来源: 2991_DEALLOCATE
PREPARE q1 AS SELECT 1 AS a ;

-- 来源: 2991_DEALLOCATE
PREPARE q2 AS SELECT 1 AS a ;

-- 来源: 2991_DEALLOCATE
PREPARE q3 AS SELECT 1 AS a ;

-- 来源: 2991_DEALLOCATE
DEALLOCATE q1 ;

-- 来源: 2991_DEALLOCATE
DEALLOCATE ALL ;

--为一个INSERT语句创建一个预备语句然后执行它。
-- 来源: 3040_EXECUTE
PREPARE insert_reason(integer,character(16),character(100)) AS INSERT INTO tpcds. reason_t1 VALUES($1,$2,$3);

-- 来源: 3040_EXECUTE
EXECUTE insert_reason(52, 'AAAAAAAADDAAAAAA', 'reason 52');

-- 来源: 4318_PBE
PREPARE p1(int) AS SELECT * FROM t1 WHERE c1 = $1;

-- 来源: 4318_PBE
PREPARE p2(INT, INT) AS SELECT * FROM t1 WHERE c1 = $1 AND c2 = $2;

-- 来源: 4318_PBE
PREPARE p3(TEXT) AS SELECT * FROM t1 WHERE c1 = $1;

-- 来源: 4318_PBE
PREPARE p4(INT) AS SELECT * FROM t1 WHERE c1 = ALL(SELECT c2 FROM t1 WHERE c1 > $1);

-- 来源: 4318_PBE
PREPARE p5(name) AS SELECT * FROM t1 WHERE c1 = $1;

-- 来源: 4318_PBE
PREPARE p6(TEXT) AS SELECT * FROM t1 WHERE c1 = currval($1);

-- 来源: 4548_PBE
PREPARE p1(int) AS SELECT * FROM t1 WHERE c1 = $1;

-- 来源: 4548_PBE
PREPARE p2(int) AS SELECT * FROM t1 WHERE c1 < $1;

-- 来源: 4548_PBE
PREPARE p3(int) AS SELECT * FROM t1 WHERE c1 > $1;

-- 来源: 4548_PBE
PREPARE p5(INT, INT) AS SELECT * FROM t1 WHERE c1 = $1 AND c2 = $2;

-- 来源: 4548_PBE
PREPARE p6(INT, INT) AS SELECT * FROM t1 WHERE c1 = $1 OR c2 = $2;

-- 来源: 4548_PBE
PREPARE p7(INT) AS SELECT * FROM t1 WHERE NOT c1 = $1;

-- 来源: 4548_PBE
PREPARE p8(INT, INT, INT) AS SELECT * FROM t1 WHERE c1 IN ($1, $2, $3);

-- 来源: 4548_PBE
PREPARE p9(INT, INT, INT) AS SELECT * FROM t1 WHERE c1 NOT IN ($1, $2, $3);

-- 来源: 4548_PBE
PREPARE p10(INT, INT, INT) AS SELECT * FROM t1 WHERE c1 = ALL(ARRAY[$1, $2, $3]);

-- 来源: 4548_PBE
PREPARE p11(INT, INT, INT) AS SELECT * FROM t1 WHERE c1 = ANY(ARRAY[$1, $2, $3]);

-- 来源: 4548_PBE
PREPARE p12(INT, INT, INT) AS SELECT * FROM t1 WHERE c1 = SOME(ARRAY[$1, $2, $3]);

-- 来源: 4548_PBE
PREPARE p13(TEXT) AS SELECT * FROM t1 WHERE c1 = $1;

-- 来源: 4548_PBE
PREPARE p14(TEXT) AS SELECT * FROM t1 WHERE c1 = LENGTHB($1);

-- 来源: 4548_PBE
PREPARE p15(INT) AS SELECT * FROM t1 WHERE c1 = ALL(SELECT c2 FROM t1 WHERE c1 > $1);

-- 来源: 4548_PBE
PREPARE p16(name) AS SELECT * FROM t1 WHERE c1 = $1;

-- 来源: 4548_PBE
PREPARE p17(TEXT) AS SELECT * FROM t1 WHERE c1 = currval($1);

--INSERT带参数/简单表达式，执行FastPath优化
-- 来源: 4554_file_4554
prepare insert_t1 as insert into fastpath_t1 values($1 + 1 + $2, $2);

