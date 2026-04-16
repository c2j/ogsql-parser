-- 来源: 1087_file_1087.txt
-- SQL 数量: 57

CREATE TABLE tab ( a int );

INSERT INTO tab values ( 1 );

INSERT INTO tab values ( 2 );

SELECT sum ( a ) FROM tab ;

SELECT MAX ( inv_quantity_on_hand ) FROM tpcds . inventory ;

SELECT MIN ( inv_quantity_on_hand ) FROM tpcds . inventory ;

SELECT AVG ( inv_quantity_on_hand ) FROM tpcds . inventory ;

SELECT COUNT ( inv_quantity_on_hand ) FROM tpcds . inventory ;

SELECT COUNT ( * ) FROM tpcds . inventory ;

SELECT ARRAY_AGG ( sr_fee ) FROM tpcds . store_returns WHERE sr_customer_sk = 2 ;

SELECT string_agg ( sr_item_sk , ',' ) FROM tpcds . store_returns WHERE sr_item_sk < 3 ;

SELECT deptno , listagg ( ename , ',' ) WITHIN GROUP ( ORDER BY ename ) AS employees FROM emp GROUP BY deptno ;

SELECT deptno , listagg ( mgrno , ',' ) WITHIN GROUP ( ORDER BY mgrno NULLS FIRST ) AS mgrnos FROM emp GROUP BY deptno ;

SELECT job , listagg ( bonus , '($);

SELECT deptno , listagg ( hiredate , ', ' ) WITHIN GROUP ( ORDER BY hiredate DESC ) AS hiredates FROM emp GROUP BY deptno ;

SELECT deptno , listagg ( vacationTime , ';

SELECT deptno , listagg ( job ) WITHIN GROUP ( ORDER BY job ) AS jobs FROM emp GROUP BY deptno ;

SELECT deptno , mgrno , bonus , listagg ( ename , ';

SELECT id , group_concat ( v separator ';

SELECT id , group_concat ( id , v ) FROM t GROUP BY id ORDER BY id ASC ;

SELECT id , group_concat ( v ) FROM t GROUP BY id ORDER BY id ASC ;

SELECT id , group_concat ( v separator ';

SELECT id , group_concat ( v separator ';

SELECT id , group_concat ( hiredate separator ';

SELECT id , group_concat ( v separator ';

SELECT id , group_concat ( vacationt separator ';

SELECT id , group_concat ( distinct v ) FROM t GROUP BY id ORDER BY id ASC ;

SELECT id , group_concat ( v ORDER BY v desc ) FROM t GROUP BY id ORDER BY id ASC ;

SELECT COVAR_POP ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

SELECT COVAR_SAMP ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

SELECT STDDEV_POP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

SELECT STDDEV_SAMP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

SELECT VAR_POP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

SELECT VAR_SAMP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

SELECT BIT_AND ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

SELECT BIT_OR ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

SELECT bool_and ( 100 < 2500 );

SELECT bool_or ( 100 < 2500 );

SELECT CORR ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

SELECT every ( 100 < 2500 );

SELECT d_moy , d_fy_week_seq , rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

SELECT REGR_AVGX ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

SELECT REGR_AVGY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

SELECT REGR_COUNT ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

SELECT REGR_INTERCEPT ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

SELECT REGR_R2 ( sr_fee , sr_net_loss ) FROM store_returns WHERE sr_customer_sk < 1000 ;

SELECT REGR_SLOPE ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

SELECT REGR_SXX ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

SELECT REGR_SXY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

SELECT REGR_SYY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

SELECT STDDEV ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

SELECT VARIANCE ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

SELECT * FROM pivot_func_test;

SELECT id, pivot_func(val) FROM pivot_func_test GROUP BY id;

SELECT CHECKSUM ( inv_quantity_on_hand ) FROM tpcds . inventory ;

SELECT CHECKSUM ( inv_quantity_on_hand :: TEXT ) FROM tpcds . inventory ;

SELECT CHECKSUM ( inventory :: TEXT ) FROM tpcds . inventory ;

