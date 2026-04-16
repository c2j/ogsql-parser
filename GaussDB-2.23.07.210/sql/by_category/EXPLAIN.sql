-- 类别: EXPLAIN
-- SQL 数量: 316

-- 来源: 1000_HintQueryblock
explain (blockname on,costs off) select * from t1, (select c1 from t2 group by c1) sub1 where t1.c1 = sub1.c1;

-- 来源: 1000_HintQueryblock
explain (blockname on,costs off) select /*+indexscan(@sel$2 t2) tablescan(t1)*/ * from t1, (select c1 from t2 group by c1) sub1 where t1.c1 = sub1.c1;

-- 来源: 1000_HintQueryblock
explain (blockname on,costs off) select * from t2, (select c1 from t1 where t1.c3 = 2) sub1 where t2.c1 = sub1.c1;

-- 来源: 1000_HintQueryblock
explain (blockname on,costs off) select * from t2, (select /*+ no_expand*/ c1 from t1 where t1.c3 = 2) sub1 where t2.c1 = sub1.c1;

-- 来源: 1000_HintQueryblock
explain (blockname on,costs off) select * from v1;

-- 来源: 1001_Hintschema
explain(blockname on,costs off) select /*+ tablescan(t1)*/ * from t1, (select c2 from t1 where c1=1) tt1 where t1.c1 = tt1.c2;

-- 来源: 1001_Hintschema
explain(blockname on,costs off) select /*+ tablescan(t1@sel$2)*/ * from t1, (select c2 from t1 where c1=1) tt1 where t1.c1 = tt1.c2;

-- 来源: 1005_StreamHint
explain select /*+ GATHER(REL)*/* from t1, t2, t3 where t1.c2 = t2.c2 and t2.c2 = t3.c2;

-- 来源: 1005_StreamHint
explain select /*+ GATHER(JOIN)*/* from t1, t2, t3 where t1.c1 = t2.c1 and t2.c2 = t3.c2;

-- 来源: 1005_StreamHint
explain select /*+ GATHER(ALL)*/* from t1, t2, t3 where t1.c1 = t2.c1 and t2.c2 = t3.c2;

-- 来源: 1007_Hint
explain select /*+ blockname(@sel$2 bn2) tablescan(@bn2 t2) tablescan(@sel$2 t2@bn2) indexscan(@sel$2 t2@sel$2) tablescan(@bn3 t3@bn3)*/ c2 from t1 where c1 in ( select /*+ */t2.c1 from t2 where t2.c2 = 1 group by 1) and c3 in ( select /*+ blockname(bn3)*/t3.c3 from t3 where t3.c2 = 1 group by 1);

-- 来源: 1007_Hint
explain select /*+ blockname(@sel$2 bn2) hashjoin(t1 bn2) nestloop(t1 bn3) nestloop(t1 sel$3)*/ c2 from t1 where c1 in ( select /*+ */t2.c1 from t2 where t2.c2 = 1 group by 1) and c3 in ( select /*+ blockname(bn3)*/t3.c3 from t3 where t3.c2 = 1 group by 1);

-- 来源: 1009_Hint
EXPLAIN (costs off) SELECT /*+PREDPUSH(pt2 st3) */ * FROM pt2, (SELECT /*+ indexscan(pt3) indexscan(pt4) */sum(pt3.b), pt3.a FROM pt3, pt4 where pt3.a = pt4.a GROUP BY pt3.a) st3 WHERE st3.a = pt2.a;

-- 来源: 1011_Plan Hint
EXPLAIN ANALYZE SELECT avg ( netpaid ) FROM ( select c_last_name , c_first_name , s_store_name , ca_state , s_state , i_color , i_current_price , i_manager_id , i_units , i_size , sum ( ss_sales_price ) netpaid FROM store_sales , store_returns , store , item , customer , customer_address WHERE ss_ticket_number = sr_ticket_number AND ss_item_sk = sr_item_sk AND ss_customer_sk = c_customer_sk AND ss_item_sk = i_item_sk AND ss_store_sk = s_store_sk AND c_birth_country = upper ( ca_country ) AND s_zip = ca_zip AND s_market_id = 7 GROUP BY c_last_name , c_first_name , s_store_name , ca_state , s_state , i_color , i_current_price , i_manager_id , i_units , i_size );

-- 来源: 1011_Plan Hint
EXPLAIN ANALYZE SELECT sum ( l_extendedprice ) / 7 . 0 AS avg_yearly FROM lineitem , part WHERE p_partkey = l_partkey AND p_brand = 'Brand#23' AND p_container = 'MED BOX' AND l_quantity < ( SELECT 0 . 2 * avg ( l_quantity ) FROM lineitem WHERE l_partkey = p_partkey );

-- 来源: 1016_Hint
explain (costs off) select /*+nestloop_index(t1,(t2 t3)) */* from t1,t2,t3 where t1.c1 = t2.c1 and t1.c2 = t3.c2;

-- 来源: 1016_Hint
explain (costs off) select /*+NestLoop_Index(t1,it1) */* from t1,t2 where t1.c1 = t2.c1;

-- 来源: 1016_Hint
EXPLAIN SELECT * FROM t1 , t2 WHERE t1 . c1 = t2 . c1 ;

-- 来源: 1016_Hint
EXPLAIN SELECT /*+predpush_same_level(t1, t2)*/ * FROM t1 , t2 WHERE t1 . c1 = t2 . c1 ;

-- 来源: 1018_bitmapscanHint
explain(costs off) select /*+ BitmapScan(t1 it1 it3)*/* from t1 where (t1.c1 = 5 or t1.c2=6) or (t1.c3=3 or t1.c2=7);

-- 来源: 1019_Hint
explain (costs off) select /*+materialize_inner(t1) materialize_inner(t1 t2)*/ * from t1,t2,t3 where t1.c3 = t2.c3 and t2.c2=t3.c2 and t1.c2=5;

-- 来源: 1020_aggHint
explain (costs off) select c1 from t2 where c1 in( select /*+ use_hash_agg */ t1.c1 from t1,t3 where t1.c1=t3.c1 group by 1);

-- 来源: 1020_aggHint
explain (costs off) select c1 from t2 where c1 in( select /*+ use_sort_agg */ t1.c1 from t1,t3 where t1.c1=t3.c1 group by 1);

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT /*+EXPAND_SUBLINK_HAVING*/ a,sum(b) AS value FROM rewrite_rule_hint_t1 GROUP BY a HAVING sum(a) >= (SELECT avg(b) FROM rewrite_rule_hint_t1) ORDER BY value DESC;

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT /*+NO_EXPAND_SUBLINK_HAVING*/ a,sum(b) AS value FROM rewrite_rule_hint_t1 GROUP BY a HAVING sum(a) >= (SELECT avg(b) FROM rewrite_rule_hint_t1) ORDER BY value DESC;

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE a > ANY(SELECT /*+EXPAND_SUBLINK*/ a FROM rewrite_rule_hint_t2) AND b > ANY (SELECT /*+EXPAND_SUBLINK*/a FROM rewrite_rule_hint_t3);

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE a > ANY(SELECT /*+NO_EXPAND_SUBLINK*/ a FROM rewrite_rule_hint_t2) AND b > ANY (SELECT /*+EXPAND_SUBLINK*/a FROM rewrite_rule_hint_t3);

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT a,(SELECT /*+EXPAND_SUBLINK_TARGET*/ avg(b) FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = rewrite_rule_hint_t2.b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a < 100 ORDER BY rewrite_rule_hint_t2.b;

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT a,(SELECT /*+NO_EXPAND_SUBLINK_TARGET*/ avg(b) FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = rewrite_rule_hint_t2.b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a < 100 ORDER BY rewrite_rule_hint_t2.b;

-- 来源: 1021_Hint
EXPLAIN(costs off) SELECT rewrite_rule_hint_t1 FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = 10 AND rewrite_rule_hint_t1.c < (SELECT /*+USE_MAGIC_SET*/ sum(c) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.a);

-- 来源: 1021_Hint
EXPLAIN(costs off) SELECT rewrite_rule_hint_t1 FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = 10 AND rewrite_rule_hint_t1.c < (SELECT /*+NO_USE_MAGIC_SET*/ sum(c) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.a);

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.a = (SELECT /*+EXPAND_SUBLINK_UNIQUE_CHECK*/ rewrite_rule_hint_t2.a FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.b);

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.a = (SELECT /*+NO_EXPAND_SUBLINK_UNIQUE_CHECK*/ rewrite_rule_hint_t2.a FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.b);

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE (0 =(SELECT /*+NO_SUBLINK_DISABLE_REPLICATED*/ count(*) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a) OR NOT EXISTS(SELECT /*+NO_SUBLINK_DISABLE_REPLICATED*/1 FROM rewrite_rule_hint_t3 WHERE rewrite_rule_hint_t3.b = rewrite_rule_hint_t1.b));

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE (0 =(SELECT /*+SUBLINK_DISABLE_REPLICATED*/ count(*) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a) OR NOT EXISTS(SELECT /*+NO_SUBLINK_DISABLE_REPLICATED*/1 FROM rewrite_rule_hint_t3 WHERE rewrite_rule_hint_t3.b = rewrite_rule_hint_t1.b));

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = (SELECT /*+NO_SUBLINK_DISABLE_EXPR*/ max(b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a);

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = (SELECT /*+SUBLINK_DISABLE_EXPR*/ max(b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a);

-- 来源: 1021_Hint
EXPLAIN (costs off)SELECT cntrycode,count(*) AS numcust,sum(c_acctbal) AS totacctbal FROM (SELECT substring(c_phone from 1 for 2) AS cntrycode,c_acctbal FROM rewrite_rule_hint_customer WHERE substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')AND c_acctbal > (SELECT /*+ENABLE_SUBLINK_ENHANCED*/ avg(c_acctbal) FROM rewrite_rule_hint_customer WHERE c_acctbal > 0.00 AND substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')) AND NOT EXISTS (SELECT * FROM rewrite_rule_hint_orders WHERE o_custkey = c_custkey)) AS custsale GROUP BY cntrycode ORDER BY cntrycode;

-- 来源: 1021_Hint
EXPLAIN (costs off)SELECT cntrycode,count(*) AS numcust,sum(c_acctbal) AS totacctbal FROM (SELECT substring(c_phone from 1 for 2) AS cntrycode,c_acctbal FROM rewrite_rule_hint_customer WHERE substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')AND c_acctbal > (SELECT /*+NO_ENABLE_SUBLINK_ENHANCED*/ avg(c_acctbal) FROM rewrite_rule_hint_customer WHERE c_acctbal > 0.00 AND substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')) AND NOT EXISTS (SELECT * FROM rewrite_rule_hint_orders WHERE o_custkey = c_custkey)) AS custsale GROUP BY cntrycode ORDER BY cntrycode;

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1,(SELECT /*+REDUCE_ORDER_BY*/ * FROM rewrite_rule_hint_t2 ORDER BY a DESC);

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1,(SELECT /*+NO_REDUCE_ORDER_BY*/ * FROM rewrite_rule_hint_t2 ORDER BY a DESC);

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.b,sum(cc) FROM (SELECT /*+LAZY_AGG*/b,sum(c) AS cc FROM rewrite_rule_hint_t2 GROUP BY b) s1,rewrite_rule_hint_t1 WHERE s1.b = rewrite_rule_hint_t1.b GROUP BY rewrite_rule_hint_t1.b ORDER BY 1,2;

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.b,sum(cc) FROM (SELECT /*+NO_LAZY_AGG*/b,sum(c) AS cc FROM rewrite_rule_hint_t2 GROUP BY b) s1,rewrite_rule_hint_t1 WHERE s1.b = rewrite_rule_hint_t1.b GROUP BY rewrite_rule_hint_t1.b ORDER BY 1,2;

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT /*+PUSHDOWN_HAVING*/ sum(a),b,c FROM rewrite_rule_hint_t1 WHERE b > 0 GROUP BY b,c HAVING sum(a) > 100 AND c > 0;

-- 来源: 1021_Hint
EXPLAIN(costs off)SELECT /*+NO_PUSHDOWN_HAVING*/ sum(a),b,c FROM rewrite_rule_hint_t1 WHERE b > 0 GROUP BY b,c HAVING sum(a) > 100 AND c > 0;

-- 来源: 1021_Hint
EXPLAIN (costs off)SELECT * FROM rewrite_rule_hint_t5 WHERE slot = '5' AND (name) IN (SELECT /*+INLIST_TO_JOIN*/ name FROM rewrite_rule_hint_t5 WHERE slot = '5'AND cid IN (5,1000,1001,1002,1003,1004,1005,1006,1007,2000,4000,10781986,10880002)LIMIT 50);

-- 来源: 1021_Hint
EXPLAIN (costs off)SELECT * FROM rewrite_rule_hint_t5 WHERE slot = '5' AND (name) IN (SELECT /*+NO_INLIST_TO_JOIN*/ name FROM rewrite_rule_hint_t5 WHERE slot = '5'AND cid IN (5,1000,1001,1002,1003,1004,1005,1006,1007,2000,4000,10781986,10880002)LIMIT 50);

-- 来源: 1021_Hint
EXPLAIN (costs off) SELECT * FROM (SELECT /*+ROWNUM_PUSHDOWN*/rownum rn, a FROM rewrite_rule_hint_t1) WHERE rn BETWEEN 5 AND 10;

-- 来源: 1021_Hint
EXPLAIN (costs off) SELECT * FROM (SELECT /*+NO_ROWNUM_PUSHDOWN*/rownum rn, a FROM rewrite_rule_hint_t1) WHERE rn BETWEEN 5 AND 10;

-- 来源: 1024_SQL PATCH
explain select * from hint_t1 where hint_t1 . a = 1 ;

-- 来源: 1036_GUCbest_agg_plan
explain select a , count ( 1 ) from agg_t1 group by a ;

-- 来源: 1036_GUCbest_agg_plan
explain select b , count ( 1 ) from t1 group by b ;

-- 来源: 1036_GUCbest_agg_plan
explain select b , count ( 1 ) from t1 group by b ;

-- 来源: 1036_GUCbest_agg_plan
explain select b , count ( 1 ) from t1 group by b ;

-- 来源: 1041_GUCrewrite_rule
explain (costs off) select * from t1, (select sum(c2), c1 from t2 group by c1) st2 where st2.c1 = t1.c1;

-- 来源: 1041_GUCrewrite_rule
explain (costs off) select * from t1, (select sum(c2), c1 from t2 group by c1) st2 where st2.c1 = t1.c1;

-- 来源: 1041_GUCrewrite_rule
explain (costs off) select * from t1, (select sum(c2), c1 from t2 group by c1) st2 where st2.c1 = t1.c1;

-- 来源: 1041_GUCrewrite_rule
explain (costs off) select * from t1, (select sum(c2), c1 from t2 group by c1) st2 where st2.c1 = t1.c1;

-- 来源: 1041_GUCrewrite_rule
explain (costs off) select * from t_dis where a = any(select a from t_rep) or a > 100;

-- 来源: 1041_GUCrewrite_rule
explain (costs off) select * from t_dis where a = any(select a from t_rep) or a > 100;

-- 来源: 1042_DN GatherStream
explain select count(*) from t1, t2 where t1.b = t2.b;

-- 来源: 1042_DN GatherStream
explain select count(*) from t1, t2 where t1.b = t2.b;

-- 来源: 1042_DN GatherStream
explain select * from t1, t2, t3, t4 where t1.b = t2.b and t2.c = t3.c and t3.d = t4.d order by t1.a;

-- 来源: 1042_DN GatherStream
explain select * from t1, t2, t3, t4 where t1.b = t2.b and t2.c = t3.c and t3.d = t4.d order by t1.a;

-- 来源: 1042_DN GatherStream
explain select count(*) from t1, t2, t3, t4 where t1.b = t2.b and t2.c = t3.c and t3.d = t4.d group by t1.b order by t1.b;

-- 来源: 1042_DN GatherStream
explain select count(*) from t1, t2, t3, t4 where t1.b = t2.b and t2.c = t3.c and t3.d = t4.d group by t1.b order by t1.b;

-- 来源: 1042_DN GatherStream
explain select count(*) from t1 group by b;

-- 来源: 1042_DN GatherStream
explain select count(*) from t1 group by b;

-- 来源: 1042_DN GatherStream
explain select b from t1 group by b;

-- 来源: 1042_DN GatherStream
explain select b from t1 group by b;

-- 来源: 1042_DN GatherStream
explain select count(*) over (partition by b) a from t1;

-- 来源: 1042_DN GatherStream
explain select count(*) over (partition by b) a from t1;

-- 来源: 1042_DN GatherStream
explain select sum(b) over (partition by b) a from t1 group by b;

-- 来源: 1042_DN GatherStream
explain select sum(b) over (partition by b) a from t1 group by b;

-- 来源: 1042_DN GatherStream
explain select t1.a, t2.b from t1, t2 where t1.b = t2.b union all select t3.a, t3.b from t3, t4 where t3.b = t4.b;

-- 来源: 1042_DN GatherStream
explain select t1.a, t2.b from t1, t2 where t1.b = t2.b union all select t3.a, t3.b from t3, t4 where t3.b = t4.b;

-- 来源: 1042_DN GatherStream
explain select t1.a, t2.b from t1, t2 where t1.b = t2.b union select t3.a, t3.b from t3, t4 where t3.b = t4.b order by a, b;

-- 来源: 1042_DN GatherStream
explain select t1.a, t2.b from t1, t2 where t1.b = t2.b union select t3.a, t3.b from t3, t4 where t3.b = t4.b order by a, b;

-- 来源: 1042_DN GatherStream
explain select b, count(*) from t1 group by b union all select b, count(*) from t2 group by b order by b;

-- 来源: 1042_DN GatherStream
explain select b, count(*) from t1 group by b union all select b, count(*) from t2 group by b order by b;

-- 来源: 1042_DN GatherStream
explain select b, count(*) from t1 group by b union all select count(distinct a) a , count(distinct b)b from t2 order by b;

-- 来源: 1042_DN GatherStream
explain select b, count(*) from t1 group by b union all select count(distinct a) a , count(distinct b)b from t2 order by b;

-- 来源: 1072_file_1072
EXPLAIN SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- 来源: 1072_file_1072
EXPLAIN SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- 来源: 1072_file_1072
EXPLAIN SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- 来源: 1072_file_1072
explain select * from logs_text t1 where t1 . log_id = 'FE306991300002 ' :: bpchar ;

-- 来源: 1072_file_1072
EXPLAIN SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- 来源: 1072_file_1072
EXPLAIN SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- 来源: 1137_file_1137
EXPLAIN SELECT a , rownum FROM test group by a , rownum having rownum < 5 ;

-- 来源: 1137_file_1137
EXPLAIN SELECT * FROM ( SELECT * FROM test WHERE rownum < 5 ) WHERE b < 5 ;

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
EXPLAIN ( COSTS OFF ) SELECT * FROM all_data ;

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
EXPLAIN ( COSTS OFF ) SELECT * FROM all_data ;

-- 来源: 1332_EXPLAIN
EXPLAIN SELECT * FROM tpcds . customer_address_p1 ;

-- 来源: 1332_EXPLAIN
EXPLAIN ( FORMAT JSON ) SELECT * FROM tpcds . customer_address_p1 ;

-- 来源: 1332_EXPLAIN
EXPLAIN SELECT * FROM tpcds . customer_address_p1 WHERE ca_address_sk = 10000 ;

-- 来源: 1332_EXPLAIN
EXPLAIN ( FORMAT YAML ) SELECT * FROM tpcds . customer_address_p1 WHERE ca_address_sk = 10000 ;

-- 来源: 1332_EXPLAIN
EXPLAIN ( COSTS FALSE ) SELECT * FROM tpcds . customer_address_p1 WHERE ca_address_sk = 10000 ;

-- 来源: 1332_EXPLAIN
EXPLAIN SELECT SUM ( ca_address_sk ) FROM tpcds . customer_address_p1 WHERE ca_address_sk < 10000 ;

-- 来源: 1333_EXPLAIN PLAN
EXPLAIN PLAN SET STATEMENT_ID = 'TPCH-Q4' FOR SELECT f1 , count ( * ) FROM foo1 WHERE f1 > 1 AND f1 < 3 AND EXISTS ( SELECT * FROM foo2 ) GROUP BY f1 ;

-- 来源: 1333_EXPLAIN PLAN
EXPLAIN PLAN SET statement_id = 'test remote query' FOR SELECT current_user FROM pt_t1 , pt_t2 ;

-- 来源: 1333_EXPLAIN PLAN
EXPLAIN PLAN SET statement_id = 'cte can not be push down' FOR WITH RECURSIVE rq AS ( SELECT id , name FROM chinamap WHERE id = 11 UNION ALL SELECT origin . id , rq . name || ' > ' || origin . name FROM rq JOIN chinamap origin ON origin . pid = rq . id ) SELECT id , name FROM rq ORDER BY 1 ;

-- 来源: 2442_file_2442
EXPLAIN ( COSTS OFF ) SELECT * FROM public . all_data ;

-- 来源: 2442_file_2442
EXPLAIN ( COSTS OFF ) SELECT * FROM public . all_data ;

-- 来源: 2688_SQL
explain select * from t1,t2 where t1.c1 = t2.c2;

-- 来源: 2688_SQL
explain select * from t1,t2 where t1.c1=t2.c2;

-- 来源: 2689_file_2689
EXPLAIN SELECT * FROM t1,t2 WHERE t1.c1 = t2.c2;

-- 来源: 2689_file_2689
explain performance select sum(t2.c1) from t1,t2 where t1.c1=t2.c2 group by t1.c2;

-- 来源: 2702_file_2702
explain ( analyze on , costs off ) select * from t1 where c1 = 10004 ;

-- 来源: 2702_file_2702
explain ( analyze on , costs off ) select * from t1 where c1 = 10004 ;

-- 来源: 2702_file_2702
explain analyze select count(*) from t1,t2 where t1.c1=t2.c2;

-- 来源: 2702_file_2702
explain analyze select count(*) from t1,t2 where t1.c1=t2.c2;

-- 来源: 2702_file_2702
explain analyze select count(*) from t1 group by c1;

-- 来源: 2702_file_2702
explain analyze select count(*) from t1 group by c1;

-- 来源: 2707_HintQueryblock
explain (blockname on,costs off) select * from t1, (select c1 from t2 group by c1) sub1 where t1.c1 = sub1.c1;

-- 来源: 2707_HintQueryblock
explain (blockname on,costs off) select /*+indexscan(@sel$2 t2) tablescan(t1)*/ * from t1, (select c1 from t2 group by c1) sub1 where t1.c1 = sub1.c1;

-- 来源: 2707_HintQueryblock
explain (blockname on,costs off) select * from t2, (select c1 from t1 where t1.c3 = 2) sub1 where t2.c1 = sub1.c1;

-- 来源: 2707_HintQueryblock
explain (blockname on,costs off) select * from t2, (select /*+ no_expand*/ c1 from t1 where t1.c3 = 2) sub1 where t2.c1 = sub1.c1;

-- 来源: 2707_HintQueryblock
explain (blockname on,costs off) select/*+ indexscan(@sel$2 t1)*/ * from t2, (select c1 from t1 where t1.c3 = 2) sub1 where t2.c1 = sub1.c1;

-- 来源: 2707_HintQueryblock
explain (blockname on,costs off) select * from v1;

-- 来源: 2708_Hintschema
explain(blockname on,costs off) select /*+ indexscan(t1)*/ * from t1, (select c2 from t1 where c1=1) tt1 where t1.c1 = tt1.c2;

-- 来源: 2708_Hintschema
explain(blockname on,costs off) select /*+ indexscan(t1@sel$2)*/ * from t1, (select c2 from t1 where c1=1) tt1 where t1.c1 = tt1.c2;

-- 使用索引
-- 来源: 2713_ScanHint
EXPLAIN SELECT /*+ gsi(gsi_test gsi_test_idx) */ * FROM gsi_test where b = 1;

-- 来源: 2713_ScanHint
EXPLAIN SELECT /*+ gsitable(gsi_test gsi_test_idx) */ * FROM gsi_test where b = 1;

-- 来源: 2714_Hint
explain select /*+ blockname(@sel$2 bn2) tablescan(@bn2 t2) tablescan(@sel$2 t2@bn2) indexscan(@sel$2 t2@sel$2) tablescan(@bn3 t3@bn3)*/ c2 from t1 where c1 in ( select /*+ */t2.c1 from t2 where t2.c2 = 1 group by 1) and c3 in ( select /*+ blockname(bn3)*/t3.c3 from t3 where t3.c2 = 1 group by 1);

-- 来源: 2714_Hint
explain select /*+ blockname(@sel$2 bn2) hashjoin(t1 bn2) nestloop(t1 bn3) nestloop(t1 sel$3)*/ c2 from t1 where c1 in ( select /*+ */t2.c1 from t2 where t2.c2 = 1 group by 1) and c3 in ( select /*+ blockname(bn3)*/t3.c3 from t3 where t3.c2 = 1 group by 1);

-- 来源: 2720_Hint
explain (costs off) select /*+nestloop_index(t1,(t2 t3)) */* from t1,t2,t3 where t1.c1 = t2.c1 and t1.c2 = t3.c2;

-- 来源: 2720_Hint
explain (costs off) select /*+NestLoop_Index(t1,it1) */* from t1,t2 where t1.c1 = t2.c1;

-- 来源: 2720_Hint
EXPLAIN SELECT * FROM t1, t2 WHERE t1.c1 = t2.c1;

-- 来源: 2720_Hint
EXPLAIN SELECT /*+predpush_same_level(t1, t2)*/ * FROM t1, t2 WHERE t1.c1 = t2.c1;

-- 来源: 2724_bitmapscanHint
explain(costs off) select /*+ BitmapScan(t1 it1 it3)*/* from t1 where (t1.c1 = 5 or t1.c2=6) or (t1.c3=3 or t1.c2=7);

-- 来源: 2725_Hint
explain (costs off) select /*+materialize_inner(t1) materialize_inner(t1 t2)*/ * from t1,t2,t3 where t1.c3 = t2.c3 and t2.c2=t3.c2 and t1.c2=5;

-- 来源: 2726_aggHint
explain (costs off) select c1 from t2 where c1 in( select /*+ use_hash_agg */ t1.c1 from t1,t3 where t1.c1=t3.c1 group by 1);

-- 来源: 2726_aggHint
explain (costs off) select c1 from t2 where c1 in( select /*+ use_sort_agg */ t1.c1 from t1,t3 where t1.c1=t3.c1 group by 1);

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT /*+EXPAND_SUBLINK_HAVING*/ a,sum(b) AS value FROM rewrite_rule_hint_t1 GROUP BY a HAVING sum(a) >= (SELECT avg(b) FROM rewrite_rule_hint_t1) ORDER BY value DESC;

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT /*+NO_EXPAND_SUBLINK_HAVING*/ a,sum(b) AS value FROM rewrite_rule_hint_t1 GROUP BY a HAVING sum(a) >= (SELECT avg(b) FROM rewrite_rule_hint_t1) ORDER BY value DESC;

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE a > ANY(SELECT /*+EXPAND_SUBLINK*/ a FROM rewrite_rule_hint_t2) AND b > ANY (SELECT /*+EXPAND_SUBLINK*/a FROM rewrite_rule_hint_t3);

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE a > ANY(SELECT /*+NO_EXPAND_SUBLINK*/ a FROM rewrite_rule_hint_t2) AND b > ANY (SELECT /*+EXPAND_SUBLINK*/a FROM rewrite_rule_hint_t3);

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT a,(SELECT /*+EXPAND_SUBLINK_TARGET*/ avg(b) FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = rewrite_rule_hint_t2.b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a < 100 ORDER BY rewrite_rule_hint_t2.b;

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT a,(SELECT /*+NO_EXPAND_SUBLINK_TARGET*/ avg(b) FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = rewrite_rule_hint_t2.b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a < 100 ORDER BY rewrite_rule_hint_t2.b;

-- 来源: 2727_Hint
EXPLAIN(costs off) SELECT rewrite_rule_hint_t1 FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = 10 AND rewrite_rule_hint_t1.c < (SELECT /*+USE_MAGIC_SET*/ sum(c) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.a);

-- 来源: 2727_Hint
EXPLAIN(costs off) SELECT rewrite_rule_hint_t1 FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = 10 AND rewrite_rule_hint_t1.c < (SELECT /*+NO_USE_MAGIC_SET*/ sum(c) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.a);

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.a = (SELECT /*+EXPAND_SUBLINK_UNIQUE_CHECK*/ rewrite_rule_hint_t2.a FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.b);

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.a = (SELECT /*+NO_EXPAND_SUBLINK_UNIQUE_CHECK*/ rewrite_rule_hint_t2.a FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.b);

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE (0 =(SELECT /*+NO_SUBLINK_DISABLE_REPLICATED*/ count(*) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a) OR NOT EXISTS(SELECT /*+NO_SUBLINK_DISABLE_REPLICATED*/1 FROM rewrite_rule_hint_t3 WHERE rewrite_rule_hint_t3.b = rewrite_rule_hint_t1.b));

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE (0 =(SELECT /*+SUBLINK_DISABLE_REPLICATED*/ count(*) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a) OR NOT EXISTS(SELECT /*+NO_SUBLINK_DISABLE_REPLICATED*/1 FROM rewrite_rule_hint_t3 WHERE rewrite_rule_hint_t3.b = rewrite_rule_hint_t1.b));

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = (SELECT /*+NO_SUBLINK_DISABLE_EXPR*/ max(b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a);

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = (SELECT /*+SUBLINK_DISABLE_EXPR*/ max(b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a);

-- 来源: 2727_Hint
EXPLAIN (costs off)SELECT cntrycode,count(*) AS numcust,sum(c_acctbal) AS totacctbal FROM (SELECT substring(c_phone from 1 for 2) AS cntrycode,c_acctbal FROM rewrite_rule_hint_customer WHERE substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')AND c_acctbal > (SELECT /*+ENABLE_SUBLINK_ENHANCED*/ avg(c_acctbal) FROM rewrite_rule_hint_customer WHERE c_acctbal > 0.00 AND substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')) AND NOT EXISTS (SELECT * FROM rewrite_rule_hint_orders WHERE o_custkey = c_custkey)) AS custsale GROUP BY cntrycode ORDER BY cntrycode;

-- 来源: 2727_Hint
EXPLAIN (costs off)SELECT cntrycode,count(*) AS numcust,sum(c_acctbal) AS totacctbal FROM (SELECT substring(c_phone from 1 for 2) AS cntrycode,c_acctbal FROM rewrite_rule_hint_customer WHERE substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')AND c_acctbal > (SELECT /*+NO_ENABLE_SUBLINK_ENHANCED*/ avg(c_acctbal) FROM rewrite_rule_hint_customer WHERE c_acctbal > 0.00 AND substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')) AND NOT EXISTS (SELECT * FROM rewrite_rule_hint_orders WHERE o_custkey = c_custkey)) AS custsale GROUP BY cntrycode ORDER BY cntrycode;

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1,(SELECT /*+REDUCE_ORDER_BY*/ * FROM rewrite_rule_hint_t2 ORDER BY a DESC);

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1,(SELECT /*+NO_REDUCE_ORDER_BY*/ * FROM rewrite_rule_hint_t2 ORDER BY a DESC);

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT /*+REMOVE_NOT_NULL*/ * FROM rewrite_rule_hint_t4 WHERE b > 10 OR a IS NOT NULL;

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT /*+NO_REMOVE_NOT_NULL*/ * FROM rewrite_rule_hint_t4 WHERE b > 10 OR a IS NOT NULL;

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.b,sum(cc) FROM (SELECT /*+LAZY_AGG*/b,sum(c) AS cc FROM rewrite_rule_hint_t2 GROUP BY b) s1,rewrite_rule_hint_t1 WHERE s1.b = rewrite_rule_hint_t1.b GROUP BY rewrite_rule_hint_t1.b ORDER BY 1,2;

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.b,sum(cc) FROM (SELECT /*+NO_LAZY_AGG*/b,sum(c) AS cc FROM rewrite_rule_hint_t2 GROUP BY b) s1,rewrite_rule_hint_t1 WHERE s1.b = rewrite_rule_hint_t1.b GROUP BY rewrite_rule_hint_t1.b ORDER BY 1,2;

-- 来源: 2727_Hint
EXPLAIN(costs off) SELECT * FROM rewrite_rule_hint_t1,(SELECT /*+EXPAND_SUBQUERY*/ * FROM rewrite_rule_hint_t2 WHERE a > 1) tt WHERE rewrite_rule_hint_t1.a = tt.a;

-- 来源: 2727_Hint
EXPLAIN(costs off) SELECT * FROM rewrite_rule_hint_t1,(SELECT /*+NO_EXPAND_SUBQUERY*/ * FROM rewrite_rule_hint_t2 WHERE a > 1) tt WHERE rewrite_rule_hint_t1.a = tt.a;

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT /*+PUSHDOWN_HAVING*/ sum(a),b,c FROM rewrite_rule_hint_t1 WHERE b > 0 GROUP BY b,c HAVING sum(a) > 100 AND c > 0;

-- 来源: 2727_Hint
EXPLAIN(costs off)SELECT /*+NO_PUSHDOWN_HAVING*/ sum(a),b,c FROM rewrite_rule_hint_t1 WHERE b > 0 GROUP BY b,c HAVING sum(a) > 100 AND c > 0;

-- 来源: 2727_Hint
EXPLAIN (costs off)SELECT * FROM rewrite_rule_hint_t5 WHERE slot = '5' AND (name) IN (SELECT /*+INLIST_TO_JOIN*/ name FROM rewrite_rule_hint_t5 WHERE slot = '5'AND cid IN (5,1000,1001,1002,1003,1004,1005,1006,1007,2000,4000,10781986,10880002)LIMIT 50);

-- 来源: 2727_Hint
EXPLAIN (costs off)SELECT * FROM rewrite_rule_hint_t5 WHERE slot = '5' AND (name) IN (SELECT /*+NO_INLIST_TO_JOIN*/ name FROM rewrite_rule_hint_t5 WHERE slot = '5'AND cid IN (5,1000,1001,1002,1003,1004,1005,1006,1007,2000,4000,10781986,10880002)LIMIT 50);

-- 来源: 2727_Hint
EXPLAIN (costs off) SELECT * FROM (SELECT /*+ROWNUM_PUSHDOWN*/rownum rn, a FROM rewrite_rule_hint_t1) WHERE rn BETWEEN 5 AND 10;

-- 来源: 2727_Hint
EXPLAIN (costs off) SELECT * FROM (SELECT /*+NO_ROWNUM_PUSHDOWN*/rownum rn, a FROM rewrite_rule_hint_t1) WHERE rn BETWEEN 5 AND 10;

-- 来源: 2728_Outline Hint
EXPLAIN (OUTLINE ON, COSTS OFF) SELECT * FROM t1 JOIN t2 ON t1.a = t2.a;

-- 来源: 2728_Outline Hint
EXPLAIN (OUTLINE ON, COSTS OFF) SELECT /*+ BEGIN_OUTLINE_DATA HashJoin(@"sel$1" t1@"sel$1" t2@"sel$1") Leading(@"sel$1" (t1@"sel$1" t2@"sel$1")) TableScan(@"sel$1" t1@"sel$1") TableScan(@"sel$1" t2@"sel$1") VERSION("1.0.0") END_OUTLINE_DATA */ * FROM t1 JOIN t2 ON t1.a = t2.a;

-- 来源: 2728_Outline Hint
EXPLAIN (OUTLINE ON, COSTS OFF) SELECT /*+ NestLoop(@"sel$1" t1@"sel$1" t2@"sel$1") Leading(@"sel$1" (t1@"sel$1" t2@"sel$1")) TableScan(@"sel$1" t1@"sel$1") TableScan(@"sel$1" t2@"sel$1") */ * FROM t1 JOIN t2 ON t1.a = t2.a;

-- 来源: 2728_Outline Hint
EXPLAIN (OUTLINE ON, COSTS OFF) SELECT /*+ BEGIN_OUTLINE_DATA NestLoop(@"sel$1" t1@"sel$1" t2@"sel$1") Leading(@"sel$1" (t1@"sel$1" t2@"sel$1")) TableScan(@"sel$1" t1@"sel$1") TableScan(@"sel$1" t2@"sel$1") VERSION("1.0.0") END_OUTLINE_DATA */ * from t1 join t2 on t1.a = t2.a;

-- 来源: 2731_SQL PATCH
explain select * from hint_t1 t1 where t1 . a = 1 ;

-- 来源: 2733_GUCrewrite_rule
explain (verbose on, costs off) select c1,(select avg(c2) from t2 where t2.c2=t1.c2) from t1 where t1.c1<100 order by t1.c2;

-- 来源: 2733_GUCrewrite_rule
explain (verbose on, costs off) select c1,(select avg(c2) from t2 where t2.c2=t1.c2) from t1 where t1.c1<100 order by t1.c2;

-- 来源: 2733_GUCrewrite_rule
explain verbose select t1.c1 from t1 where t1.c1 = (select t2.c1 from t2 where t1.c1=t2.c1);

-- 来源: 2733_GUCrewrite_rule
explain (costs off) select t.b, sum(cc) from (select b, sum(c) as cc from t1 group by b) s1, t where s1.b=t.b group by t.b order by 1,2;

-- 来源: 2733_GUCrewrite_rule
explain (costs off) select t.b, sum(cc) from (select b, sum(c) as cc from t1 group by b) s1, t where s1.b=t.b group by t.b order by 1,2;

-- 来源: 2733_GUCrewrite_rule
explain (costs off) select t1 from t1 where t1.b = 10 and t1.c < (select sum(c) from t2 where t1.a = t2.a);

-- 来源: 2769_file_2769
EXPLAIN SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- 来源: 2769_file_2769
EXPLAIN SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- 来源: 2769_file_2769
EXPLAIN SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- 来源: 2769_file_2769
EXPLAIN SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- 来源: 2769_file_2769
EXPLAIN SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- 来源: 2769_file_2769
explain SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- 来源: 2769_file_2769
EXPLAIN SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id :: bpchar = t2 . log_id ;

-- 来源: 2835_file_2835
EXPLAIN SELECT a , rownum FROM test group by a , rownum having rownum < 5 ;

-- 来源: 2835_file_2835
EXPLAIN SELECT * FROM ( SELECT * FROM test WHERE rownum < 5 ) WHERE b < 5 ;

-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
EXPLAIN(COSTS OFF) SELECT * FROM all_data;

-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
EXPLAIN(COSTS OFF) SELECT * FROM all_data;

-- 来源: 3043_EXPLAIN
EXPLAIN (NODE true) INSERT INTO student VALUES(5,'a'),(6,'b');

-- 来源: 3043_EXPLAIN
EXPLAIN (NUM_NODES true) INSERT INTO student VALUES(5,'a'),(6,'b');

--显示表简单查询的执行计划。
-- 来源: 3043_EXPLAIN
EXPLAIN SELECT * FROM tpcds. customer_address_p1;

--以JSON格式输出的执行计划（explain_perf_mode为normal时）。
-- 来源: 3043_EXPLAIN
EXPLAIN(FORMAT JSON) SELECT * FROM tpcds. customer_address_p1;

--如果有一个索引，当使用一个带索引WHERE条件的查询，可能会显示一个不同的计划。
-- 来源: 3043_EXPLAIN
EXPLAIN SELECT * FROM tpcds. customer_address_p1 WHERE ca_address_sk=10000;

--以YAML格式输出的执行计划（explain_perf_mode为normal时）。
-- 来源: 3043_EXPLAIN
EXPLAIN(FORMAT YAML) SELECT * FROM tpcds. customer_address_p1 WHERE ca_address_sk=10000;

--禁止开销估计的执行计划。
-- 来源: 3043_EXPLAIN
EXPLAIN(COSTS FALSE) SELECT * FROM tpcds. customer_address_p1 WHERE ca_address_sk=10000;

--带有聚集函数查询的执行计划。
-- 来源: 3043_EXPLAIN
EXPLAIN SELECT SUM(ca_address_sk) FROM tpcds. customer_address_p1 WHERE ca_address_sk<10000;

--执行带有二级分区表的查询语句。
--Iterations 和 Sub Iterations分别标识遍历了几个一级分区和二级分区。
--Selected Partitions标识哪些一级分区被实际扫描，Selected Subpartitions: (p:s)标识第p个一级分区下s个二级分区被实际扫描，如果一级分区下所有二级分区都被扫描则s显示为ALL。
-- 来源: 3043_EXPLAIN
EXPLAIN SELECT * FROM range_list WHERE dept_code = '1';

-- 来源: 3043_EXPLAIN
EXPLAIN (OPTEVAL on )SELECT * FROM tb_a a, tb_b b WHERE a.c1=b.c1 AND a.c1=1;

--执行explain plan。
-- 来源: 3044_EXPLAIN PLAN
EXPLAIN PLAN SET STATEMENT_ID = 'TPCH-Q4' FOR SELECT f1, count(*) FROM foo1 WHERE f1 > 1 AND f1 < 3 AND EXISTS (SELECT * FROM foo2) GROUP BY f1;

-- 来源: 3120_SQL
explain select * from t1 where not exists(select * from t2 where t1.c1 = t2.c1);

-- 来源: 4316_file_4316
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = 1;

-- 来源: 4316_file_4316
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 < 1;

-- 来源: 4316_file_4316
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 > 11;

-- 来源: 4316_file_4316
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 is NULL;

-- 来源: 4316_file_4316
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = 1 AND c2 = 2;

-- 来源: 4316_file_4316
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = 1 OR c1 = 2;

-- 来源: 4316_file_4316
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE NOT c1 = 1;

-- 来源: 4316_file_4316
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 IN (1, 2, 3);

-- 来源: 4316_file_4316
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = ALL(ARRAY[1, 2, 3]);

-- 来源: 4316_file_4316
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = ANY(ARRAY[1, 2, 3]);

-- 来源: 4316_file_4316
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = SOME(ARRAY[1, 2, 3]);

-- 来源: 4316_file_4316
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = ALL(SELECT c2 FROM t1 WHERE c1 > 10);

-- 来源: 4318_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p1(1);

-- 来源: 4318_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p2(1, 2);

-- 来源: 4318_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p3('12');

-- 来源: 4318_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p4(1);

-- 来源: 4318_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p5('12');

-- 来源: 4318_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p6('seq');

-- 来源: 4319_file_4319
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) indexscan(t1) indexscan(t2) */ * FROM t2 JOIN t1 ON t1.c1 = t2.c1;

-- 来源: 4319_file_4319
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) indexscan(t1) indexscan(t2) */ * FROM t2 JOIN t1 ON t1.c1 < t2.c1;

-- 来源: 4319_file_4319
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) indexscan(t1) indexscan(t2) */ * FROM t2 JOIN t1 ON t1.c1 < t2.c1;

-- 来源: 4319_file_4319
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) indexscan(t1) indexscan(t2) */ * FROM t2 JOIN t1 ON t1.c1 = t2.c1 AND t1.c2 = 2;

-- 来源: 4319_file_4319
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) */ * FROM t2 JOIN t1 ON t1.c1 = t2.c1 OR t1.c2 = 2;

-- 来源: 4319_file_4319
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 JOIN t3 ON t1.c1 = t3.c1;

-- 来源: 4319_file_4319
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 JOIN t3 ON t1.c1 = LENGTHB(t3.c1);

-- 来源: 4323_file_4323
EXPLAIN SELECT * FROM test_int4_maxvalue PARTITION(id1);

-- 查询指定分区max_id
-- 来源: 4323_file_4323
EXPLAIN SELECT * FROM test_int4_maxvalue PARTITION(max_id);

-- 查询指定分区p_
-- 来源: 4323_file_4323
EXPLAIN SELECT * FROM test_default PARTITION(p_1);

-- 查询指定分区p_
-- 来源: 4323_file_4323
EXPLAIN SELECT * FROM test_default PARTITION(p_3);

-- 来源: 4434_file_4434
explain SELECT c_discount from bmsql_customer where c_w_id = 10;

-- 来源: 4434_file_4434
explain SELECT c_discount from bmsql_customer where c_w_id = 10;

-- 来源: 4546_file_4546
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = 1;

-- 来源: 4546_file_4546
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 < 1;

-- 来源: 4546_file_4546
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 > 11;

-- 来源: 4546_file_4546
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 is NULL;

-- 来源: 4546_file_4546
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = 1 AND c2 = 2;

-- 来源: 4546_file_4546
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = 1 OR c1 = 2;

-- 来源: 4546_file_4546
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE NOT c1 = 1;

-- 来源: 4546_file_4546
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 IN (1, 2, 3);

-- 来源: 4546_file_4546
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = ALL(ARRAY[1, 2, 3]);

-- 来源: 4546_file_4546
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = ANY(ARRAY[1, 2, 3]);

-- 来源: 4546_file_4546
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = SOME(ARRAY[1, 2, 3]);

-- 来源: 4546_file_4546
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = ALL(SELECT c2 FROM t1 WHERE c1 > 10);

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p1(1);

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p2(1);

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p3(1);

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p5(1, 2);

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p6(1, 2);

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) execute p7(1);

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p8(1, 2, 3);

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p9(1, 2, 3);

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p10(1, 2, 3);

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p11(1, 2, 3);

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p12(1, 2, 3);

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p13('12');

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p14('hello');

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p15(1);

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p16('12');

-- 来源: 4548_PBE
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p17('seq');

-- 来源: 4549_file_4549
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t2 JOIN t1 ON t1.c1 = t2.c2;

-- 来源: 4549_file_4549
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t2 JOIN t1 ON t1.c1 < t2.c2;

-- 来源: 4549_file_4549
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t2 JOIN t1 ON t1.c1 > t2.c2;

-- 来源: 4549_file_4549
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t2 JOIN t1 ON t1.c1 = t2.c2 AND t1.c2 = 2;

-- 来源: 4549_file_4549
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t2 JOIN t1 ON t1.c1 = t2.c2 OR t1.c1 = 2;

-- 来源: 4549_file_4549
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 JOIN t3 ON t1.c1 = t3.c1;

-- 来源: 4549_file_4549
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 JOIN t3 ON t1.c1 = LENGTHB(t3.c1);

-- 来源: 4551_Partition Iterator
EXPLAIN SELECT * FROM test_range_pt WHERE a = 3000;

-- 来源: 4551_Partition Iterator
EXPLAIN SELECT * FROM test_range_pt WHERE a = 3000;

-- 来源: 4552_Merge Append
EXPLAIN ANALYZE SELECT * FROM test_range_pt WHERE b >10 AND b < 5000 ORDER BY b LIMIT 10;

-- 来源: 4552_Merge Append
EXPLAIN ANALYZE SELECT * FROM test_range_pt WHERE b >10 AND b < 5000 ORDER BY b LIMIT 10;

-- 来源: 4553_Max_Min
explain analyze select min(b) from test_range_pt;

-- 来源: 4553_Max_Min
explain analyze select min(b) from test_range_pt;

--INSERT常量，执行FastPath优化
-- 来源: 4554_file_4554
explain insert into fastpath_t1 values (0, 'test_insert');

-- 来源: 4554_file_4554
explain execute insert_t1(10, '0');

-- 来源: 4554_file_4554
explain insert into fastpath_t1 select * from test_1;

-- 查询指定分区id
-- 来源: 4558_file_4558
EXPLAIN SELECT * FROM test_int4_maxvalue PARTITION(id1);

-- 查询指定分区max_id
-- 来源: 4558_file_4558
EXPLAIN SELECT * FROM test_int4_maxvalue PARTITION(max_id);

-- 查询指定分区p_
-- 来源: 4558_file_4558
EXPLAIN SELECT * FROM test_default PARTITION(p_1);

-- 来源: 4558_file_4558
EXPLAIN SELECT * FROM test_default PARTITION(p_3);

-- 来源: 4694_file_4694
explain SELECT c_discount from bmsql_customer where c_w_id = 10;

-- 来源: 4694_file_4694
explain SELECT c_discount from bmsql_customer where c_w_id = 10;

-- 来源: 4733_DB4AI
Explain CREATE MODEL patient_logisitic_regression USING logistic_regression FEATURES second_attack, treatment TARGET trait_anxiety > 50 FROM patients WITH batch_size=10, learning_rate = 0.05;

-- 来源: 744_file_744
EXPLAIN ( COSTS OFF ) SELECT * FROM public . all_data ;

-- 来源: 744_file_744
EXPLAIN ( COSTS OFF ) SELECT * FROM public . all_data ;

-- 来源: 977_SQL
explain select * from t1,t2 where t1.c1=t2.c2;

-- 来源: 977_SQL
explain select * from t1,t2 where t1.c1=t2.c2;

-- 来源: 978_file_978
EXPLAIN SELECT * FROM t1,t2 WHERE t1.c1 = t2.c2;

-- 来源: 978_file_978
explain select c1 , count ( 1 ) from t1 group by c1 ;

-- 来源: 978_file_978
explain select c1,count(1) from t1 group by c1;

-- 来源: 978_file_978
explain performance select count(1) from t1;

-- 来源: 991_file_991
explain select * from t where c1 > 1;

-- 来源: 991_file_991
explain select * from t limit 1;

-- 来源: 991_file_991
explain select sum(c1), count(*) from t;

-- 来源: 991_file_991
explain select * from t1 join t on t.c1 = t1.c1;

-- 来源: 991_file_991
explain select * from t1 join t on t.c1 = t1.c2;

-- 来源: 991_file_991
EXPLAIN (COSTS OFF) SELECT * FROM t1 UNION ALL SELECT * FROM t2;

-- 来源: 991_file_991
EXPLAIN (COSTS OFF) SELECT * FROM t1 UNION SELECT * FROM t2;

-- 来源: 991_file_991
EXPLAIN (COSTS OFF) SELECT * FROM t1 WHERE a = 1 UNION SELECT * FROM t2 WHERE a = 1;

-- 来源: 991_file_991
EXPLAIN (COSTS OFF) SELECT * FROM t1 WHERE a = 1;

-- 来源: 991_file_991
EXPLAIN (COSTS OFF) SELECT * FROM t2 WHERE a = 3;

-- 来源: 991_file_991
EXPLAIN (COSTS OFF) SELECT * FROM t1 WHERE a = 1 UNION SELECT * FROM t2 WHERE a = 3;

-- 来源: 991_file_991
EXPLAIN ( COSTS OFF ) WITH cte AS ( SELECT * FROM t1 ) SELECT * FROM cte ;

-- 来源: 991_file_991
EXPLAIN ( COSTS OFF ) WITH cte AS ( SELECT * FROM t1 WHERE a = 1 ) SELECT * FROM cte ;

-- 来源: 991_file_991
EXPLAIN ( COSTS OFF ) WITH cte AS ( SELECT * FROM t1 ORDER BY a ) SELECT * FROM cte ;

-- 来源: 991_file_991
EXPLAIN ( COSTS OFF ) WITH RECURSIVE cte AS ( SELECT * FROM t1 UNION ALL SELECT t2 . * FROM t2 , cte WHERE cte . a = t2 . a ) SELECT * FROM cte ;

-- 来源: 991_file_991
EXPLAIN ( COSTS OFF ) WITH RECURSIVE cte AS ( SELECT * FROM t1 WHERE a = 1 UNION ALL SELECT t2 . * FROM t2 , cte WHERE cte . a = t2 . a AND t2 . a = 1 ) SELECT * FROM cte ;

-- 来源: 991_file_991
EXPLAIN ( COSTS OFF ) WITH RECURSIVE cte AS ( SELECT * FROM t1 UNION SELECT t2 . * FROM t2 , cte WHERE cte . a = t2 . a ) SELECT * FROM cte ;

-- 来源: 991_file_991
explain update customer1 set C_NAME = 'a' returning c_name ;

-- 来源: 991_file_991
explain verbose select count ( c_custkey order by c_custkey ) from customer1 ;

-- 来源: 991_file_991
explain verbose select count ( distinct b ) from test_stream ;

-- 来源: 991_file_991
explain verbose select distinct on ( c_custkey ) c_custkey from customer1 order by c_custkey ;

-- 来源: 991_file_991
explain verbose select array [ c_custkey , 1 ] from customer1 order by c_custkey ;

-- 来源: 994_file_994
explain ( analyze on , costs off ) select * from t1 where c2 = 10004 ;

-- 来源: 994_file_994
explain ( analyze on , costs off ) select * from t1 where c2 = 10004 ;

-- 来源: 994_file_994
explain analyze select count(*) from t2,t1 where t1.c1=t2.c2;

-- 来源: 994_file_994
explain analyze select count(*) from t2,t1 where t1.c1=t2.c2;

-- 来源: 994_file_994
explain analyze select count(*) from t1 group by c2;

-- 来源: 994_file_994
explain analyze select count(*) from t1 group by c2;

-- 来源: 995_file_995
explain performance select count ( * ) from inventory ;

