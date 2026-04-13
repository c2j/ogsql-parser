-- GaussDB 文档 SQL 汇总
-- 总 SQL 语句数量: 10707
-- 提取来源: GaussDB-2.23.07.210/云数据库 GaussDB 2.23.07.210 产品文档 (for 华为云Stack 8.3.0) 04


================================================================================
-- 来源: 1000_HintQueryblock.txt
================================================================================

-- [SESSION]
set explain_perf_mode = pretty;

-- [SESSION]
set enable_fast_query_shipping = off;

-- [EXPLAIN]
explain (blockname on,costs off) select * from t1, (select c1 from t2 group by c1) sub1 where t1.c1 = sub1.c1;

-- [EXPLAIN]
explain (blockname on,costs off) select /*+indexscan(@sel$2 t2) tablescan(t1)*/ * from t1, (select c1 from t2 group by c1) sub1 where t1.c1 = sub1.c1;

-- [EXPLAIN]
explain (blockname on,costs off) select * from t2, (select c1 from t1 where t1.c3 = 2) sub1 where t2.c1 = sub1.c1;

-- [EXPLAIN]
explain (blockname on,costs off) select * from t2, (select /*+ no_expand*/ c1 from t1 where t1.c3 = 2) sub1 where t2.c1 = sub1.c1;

-- [DDL]
create view v1 as select/*+ no_expand */ c1 from t1 where c1 in (select /*+ no_expand */ c1 from t2 where t2.c3=4 );

-- [EXPLAIN]
explain (blockname on,costs off) select * from v1;


================================================================================
-- 来源: 1001_Hintschema.txt
================================================================================

-- [EXPLAIN]
explain(blockname on,costs off) select /*+ tablescan(t1)*/ * from t1, (select c2 from t1 where c1=1) tt1 where t1.c1 = tt1.c2;

-- [EXPLAIN]
explain(blockname on,costs off) select /*+ tablescan(t1@sel$2)*/ * from t1, (select c2 from t1 where c1=1) tt1 where t1.c1 = tt1.c2;


================================================================================
-- 来源: 1005_StreamHint.txt
================================================================================

-- [EXPLAIN]
explain select /*+ GATHER(REL)*/* from t1, t2, t3 where t1.c2 = t2.c2 and t2.c2 = t3.c2;

-- [EXPLAIN]
explain select /*+ GATHER(JOIN)*/* from t1, t2, t3 where t1.c1 = t2.c1 and t2.c2 = t3.c2;

-- [EXPLAIN]
explain select /*+ GATHER(ALL)*/* from t1, t2, t3 where t1.c1 = t2.c1 and t2.c2 = t3.c2;


================================================================================
-- 来源: 1007_Hint.txt
================================================================================

-- [EXPLAIN]
explain select /*+ blockname(@sel$2 bn2) tablescan(@bn2 t2) tablescan(@sel$2 t2@bn2) indexscan(@sel$2 t2@sel$2) tablescan(@bn3 t3@bn3)*/ c2 from t1 where c1 in ( select /*+ */t2.c1 from t2 where t2.c2 = 1 group by 1) and c3 in ( select /*+ blockname(bn3)*/t3.c3 from t3 where t3.c2 = 1 group by 1);

-- [EXPLAIN]
explain select /*+ blockname(@sel$2 bn2) hashjoin(t1 bn2) nestloop(t1 bn3) nestloop(t1 sel$3)*/ c2 from t1 where c1 in ( select /*+ */t2.c1 from t2 where t2.c2 = 1 group by 1) and c3 in ( select /*+ blockname(bn3)*/t3.c3 from t3 where t3.c2 = 1 group by 1);


================================================================================
-- 来源: 1009_Hint.txt
================================================================================

-- [EXPLAIN]
EXPLAIN (costs off) SELECT /*+PREDPUSH(pt2 st3) */ * FROM pt2, (SELECT /*+ indexscan(pt3) indexscan(pt4) */sum(pt3.b), pt3.a FROM pt3, pt4 where pt3.a = pt4.a GROUP BY pt3.a) st3 WHERE st3.a = pt2.a;


================================================================================
-- 来源: 1011_Plan Hint.txt
================================================================================

-- [EXPLAIN]
EXPLAIN ANALYZE SELECT avg ( netpaid ) FROM ( select c_last_name , c_first_name , s_store_name , ca_state , s_state , i_color , i_current_price , i_manager_id , i_units , i_size , sum ( ss_sales_price ) netpaid FROM store_sales , store_returns , store , item , customer , customer_address WHERE ss_ticket_number = sr_ticket_number AND ss_item_sk = sr_item_sk AND ss_customer_sk = c_customer_sk AND ss_item_sk = i_item_sk AND ss_store_sk = s_store_sk AND c_birth_country = upper ( ca_country ) AND s_zip = ca_zip AND s_market_id = 7 GROUP BY c_last_name , c_first_name , s_store_name , ca_state , s_state , i_color , i_current_price , i_manager_id , i_units , i_size );

-- [EXPLAIN]
EXPLAIN ANALYZE SELECT sum ( l_extendedprice ) / 7 . 0 AS avg_yearly FROM lineitem , part WHERE p_partkey = l_partkey AND p_brand = 'Brand#23' AND p_container = 'MED BOX' AND l_quantity < ( SELECT 0 . 2 * avg ( l_quantity ) FROM lineitem WHERE l_partkey = p_partkey );


================================================================================
-- 来源: 1015_Hint.txt
================================================================================

-- [PREPARED_STMT]
deallocate all;

-- [PREPARED_STMT]
prepare p1 as insert /*+ no_gpc */ into t1 select c1,c2 from t2 where c1=$1;

-- [PREPARED_STMT]
execute p1(3);

-- [DQL]
select * from dbe_perf.global_plancache_status where schema_name='public' order by 1,2;


================================================================================
-- 来源: 1016_Hint.txt
================================================================================

-- [EXPLAIN]
explain (costs off) select /*+nestloop_index(t1,(t2 t3)) */* from t1,t2,t3 where t1.c1 = t2.c1 and t1.c2 = t3.c2;

-- [EXPLAIN]
explain (costs off) select /*+NestLoop_Index(t1,it1) */* from t1,t2 where t1.c1 = t2.c1;

-- [SESSION]
SET rewrite_rule = 'predpushforce' ;

-- [SESSION]
SET enable_fast_query_shipping = off ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM t1 , t2 WHERE t1 . c1 = t2 . c1 ;

-- [EXPLAIN]
EXPLAIN SELECT /*+predpush_same_level(t1, t2)*/ * FROM t1 , t2 WHERE t1 . c1 = t2 . c1 ;


================================================================================
-- 来源: 1018_bitmapscanHint.txt
================================================================================

-- [EXPLAIN]
explain(costs off) select /*+ BitmapScan(t1 it1 it3)*/* from t1 where (t1.c1 = 5 or t1.c2=6) or (t1.c3=3 or t1.c2=7);


================================================================================
-- 来源: 1019_Hint.txt
================================================================================

-- [EXPLAIN]
explain (costs off) select /*+materialize_inner(t1) materialize_inner(t1 t2)*/ * from t1,t2,t3 where t1.c3 = t2.c3 and t2.c2=t3.c2 and t1.c2=5;


================================================================================
-- 来源: 1020_aggHint.txt
================================================================================

-- [EXPLAIN]
explain (costs off) select c1 from t2 where c1 in( select /*+ use_hash_agg */ t1.c1 from t1,t3 where t1.c1=t3.c1 group by 1);

-- [EXPLAIN]
explain (costs off) select c1 from t2 where c1 in( select /*+ use_sort_agg */ t1.c1 from t1,t3 where t1.c1=t3.c1 group by 1);


================================================================================
-- 来源: 1021_Hint.txt
================================================================================

-- [EXPLAIN]
EXPLAIN(costs off)SELECT /*+EXPAND_SUBLINK_HAVING*/ a,sum(b) AS value FROM rewrite_rule_hint_t1 GROUP BY a HAVING sum(a) >= (SELECT avg(b) FROM rewrite_rule_hint_t1) ORDER BY value DESC;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT /*+NO_EXPAND_SUBLINK_HAVING*/ a,sum(b) AS value FROM rewrite_rule_hint_t1 GROUP BY a HAVING sum(a) >= (SELECT avg(b) FROM rewrite_rule_hint_t1) ORDER BY value DESC;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE a > ANY(SELECT /*+EXPAND_SUBLINK*/ a FROM rewrite_rule_hint_t2) AND b > ANY (SELECT /*+EXPAND_SUBLINK*/a FROM rewrite_rule_hint_t3);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE a > ANY(SELECT /*+NO_EXPAND_SUBLINK*/ a FROM rewrite_rule_hint_t2) AND b > ANY (SELECT /*+EXPAND_SUBLINK*/a FROM rewrite_rule_hint_t3);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT a,(SELECT /*+EXPAND_SUBLINK_TARGET*/ avg(b) FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = rewrite_rule_hint_t2.b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a < 100 ORDER BY rewrite_rule_hint_t2.b;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT a,(SELECT /*+NO_EXPAND_SUBLINK_TARGET*/ avg(b) FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = rewrite_rule_hint_t2.b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a < 100 ORDER BY rewrite_rule_hint_t2.b;

-- [EXPLAIN]
EXPLAIN(costs off) SELECT rewrite_rule_hint_t1 FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = 10 AND rewrite_rule_hint_t1.c < (SELECT /*+USE_MAGIC_SET*/ sum(c) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.a);

-- [EXPLAIN]
EXPLAIN(costs off) SELECT rewrite_rule_hint_t1 FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = 10 AND rewrite_rule_hint_t1.c < (SELECT /*+NO_USE_MAGIC_SET*/ sum(c) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.a);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.a = (SELECT /*+EXPAND_SUBLINK_UNIQUE_CHECK*/ rewrite_rule_hint_t2.a FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.b);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.a = (SELECT /*+NO_EXPAND_SUBLINK_UNIQUE_CHECK*/ rewrite_rule_hint_t2.a FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.b);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE (0 =(SELECT /*+NO_SUBLINK_DISABLE_REPLICATED*/ count(*) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a) OR NOT EXISTS(SELECT /*+NO_SUBLINK_DISABLE_REPLICATED*/1 FROM rewrite_rule_hint_t3 WHERE rewrite_rule_hint_t3.b = rewrite_rule_hint_t1.b));

-- [EXPLAIN]
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE (0 =(SELECT /*+SUBLINK_DISABLE_REPLICATED*/ count(*) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a) OR NOT EXISTS(SELECT /*+NO_SUBLINK_DISABLE_REPLICATED*/1 FROM rewrite_rule_hint_t3 WHERE rewrite_rule_hint_t3.b = rewrite_rule_hint_t1.b));

-- [EXPLAIN]
EXPLAIN(costs off)SELECT a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = (SELECT /*+NO_SUBLINK_DISABLE_EXPR*/ max(b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = (SELECT /*+SUBLINK_DISABLE_EXPR*/ max(b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a);

-- [EXPLAIN]
EXPLAIN (costs off)SELECT cntrycode,count(*) AS numcust,sum(c_acctbal) AS totacctbal FROM (SELECT substring(c_phone from 1 for 2) AS cntrycode,c_acctbal FROM rewrite_rule_hint_customer WHERE substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')AND c_acctbal > (SELECT /*+ENABLE_SUBLINK_ENHANCED*/ avg(c_acctbal) FROM rewrite_rule_hint_customer WHERE c_acctbal > 0.00 AND substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')) AND NOT EXISTS (SELECT * FROM rewrite_rule_hint_orders WHERE o_custkey = c_custkey)) AS custsale GROUP BY cntrycode ORDER BY cntrycode;

-- [EXPLAIN]
EXPLAIN (costs off)SELECT cntrycode,count(*) AS numcust,sum(c_acctbal) AS totacctbal FROM (SELECT substring(c_phone from 1 for 2) AS cntrycode,c_acctbal FROM rewrite_rule_hint_customer WHERE substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')AND c_acctbal > (SELECT /*+NO_ENABLE_SUBLINK_ENHANCED*/ avg(c_acctbal) FROM rewrite_rule_hint_customer WHERE c_acctbal > 0.00 AND substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')) AND NOT EXISTS (SELECT * FROM rewrite_rule_hint_orders WHERE o_custkey = c_custkey)) AS custsale GROUP BY cntrycode ORDER BY cntrycode;

-- [SESSION]
SET rewrite_rule='intargetlist';

-- [SESSION]
SET rewrite_rule='intargetlist';

-- [EXPLAIN]
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1,(SELECT /*+REDUCE_ORDER_BY*/ * FROM rewrite_rule_hint_t2 ORDER BY a DESC);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1,(SELECT /*+NO_REDUCE_ORDER_BY*/ * FROM rewrite_rule_hint_t2 ORDER BY a DESC);

-- [SESSION]
SET enable_fast_query_shipping=off;

-- [SESSION]
SET enable_fast_query_shipping=off;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.b,sum(cc) FROM (SELECT /*+LAZY_AGG*/b,sum(c) AS cc FROM rewrite_rule_hint_t2 GROUP BY b) s1,rewrite_rule_hint_t1 WHERE s1.b = rewrite_rule_hint_t1.b GROUP BY rewrite_rule_hint_t1.b ORDER BY 1,2;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.b,sum(cc) FROM (SELECT /*+NO_LAZY_AGG*/b,sum(c) AS cc FROM rewrite_rule_hint_t2 GROUP BY b) s1,rewrite_rule_hint_t1 WHERE s1.b = rewrite_rule_hint_t1.b GROUP BY rewrite_rule_hint_t1.b ORDER BY 1,2;

-- [SESSION]
SET enable_fast_query_shipping=off;

-- [SESSION]
SET enable_fast_query_shipping=off;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT /*+PUSHDOWN_HAVING*/ sum(a),b,c FROM rewrite_rule_hint_t1 WHERE b > 0 GROUP BY b,c HAVING sum(a) > 100 AND c > 0;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT /*+NO_PUSHDOWN_HAVING*/ sum(a),b,c FROM rewrite_rule_hint_t1 WHERE b > 0 GROUP BY b,c HAVING sum(a) > 100 AND c > 0;

-- [EXPLAIN]
EXPLAIN (costs off)SELECT * FROM rewrite_rule_hint_t5 WHERE slot = '5' AND (name) IN (SELECT /*+INLIST_TO_JOIN*/ name FROM rewrite_rule_hint_t5 WHERE slot = '5'AND cid IN (5,1000,1001,1002,1003,1004,1005,1006,1007,2000,4000,10781986,10880002)LIMIT 50);

-- [EXPLAIN]
EXPLAIN (costs off)SELECT * FROM rewrite_rule_hint_t5 WHERE slot = '5' AND (name) IN (SELECT /*+NO_INLIST_TO_JOIN*/ name FROM rewrite_rule_hint_t5 WHERE slot = '5'AND cid IN (5,1000,1001,1002,1003,1004,1005,1006,1007,2000,4000,10781986,10880002)LIMIT 50);

-- [EXPLAIN]
EXPLAIN (costs off) SELECT * FROM (SELECT /*+ROWNUM_PUSHDOWN*/rownum rn, a FROM rewrite_rule_hint_t1) WHERE rn BETWEEN 5 AND 10;

-- [EXPLAIN]
EXPLAIN (costs off) SELECT * FROM (SELECT /*+NO_ROWNUM_PUSHDOWN*/rownum rn, a FROM rewrite_rule_hint_t1) WHERE rn BETWEEN 5 AND 10;


================================================================================
-- 来源: 1023_file_1023.txt
================================================================================

-- [SESSION]
SHOW try_vector_engine_strategy;

-- [SESSION]
SET try_vector_engine_strategy=force;

-- [SESSION]
SHOW try_vector_engine_strategy;


================================================================================
-- 来源: 1024_SQL PATCH.txt
================================================================================

-- [DDL]
create table hint_t1 ( a int , b int , c int );

-- [DDL]
create index on hint_t1 ( a );

-- [DML_INSERT]
insert into hint_t1 values ( 1 , 1 , 1 );

-- [MAINTENANCE]
analyze hint_t1 ;

-- [SESSION]
set track_stmt_stat_level = 'L1,L1' ;

-- [SESSION]
set enable_fast_query_shipping = off ;

-- [SESSION]
set explain_perf_mode = normal ;

-- [DQL]
select * from hint_t1 where hint_t1 . a = 1 ;

-- [OTHER]
\ x --切换扩展显示模式，便于观察计划 Expanded display is on .

-- [DQL]
select unique_query_id , query , query_plan from dbe_perf . statement_history where query like '%hint_t1%' ;

-- [OTHER]
\ x --关闭扩展显示模式

-- [DQL]
select * from dbe_sql_util . create_hint_sql_patch ( 'patch1' , 3929365485 , 'indexscan(hint_t1)' );

-- [SESSION]
set track_stmt_stat_level = 'L1,L1' ;

-- [SESSION]
set enable_fast_query_shipping = off ;

-- [EXPLAIN]
explain select * from hint_t1 where hint_t1 . a = 1 ;

-- [DQL]
select * from hint_t1 where hint_t1 . a = 1 ;

-- [OTHER]
\ x Expanded display is on .

-- [DQL]
select unique_query_id , query , query_plan from dbe_perf . statement_history where query like '%hint_t1%' ;

-- [DQL]
select * from dbe_sql_util.drop_sql_patch('patch1');

-- [DQL]
select * from dbe_sql_util.create_abort_sql_patch('patch2', 3929365485);

-- [DQL]
select * from hint_t1 t1 where t1.a = 1;

-- [DDL]
create table test_proc_patch(a int,b int);

-- [DML_INSERT]
insert into test_proc_patch values(1,2);

-- [DDL]
create procedure mypro() as num int;

-- [SESSION]
set track_stmt_stat_level = 'L0,L1';

-- [DQL]
select b from test_proc_patch where a = 1;

-- [PLSQL]
call mypro();

-- [DQL]
select unique_query_id, query, query_plan, parent_unique_sql_id from dbe_perf.statement_history where query like '%call mypro();

-- [DQL]
select * from dbe_sql_util.create_abort_sql_patch('patch1',2859505004,2502737203);

-- [DQL]
select patch_name,unique_sql_id,parent_unique_sql_id,enable,abort,hint_string from gs_sql_patch where patch_name = 'patch1';

-- [DQL]
select b from test_proc_patch where a = 1;

-- [PLSQL]
call mypro();


================================================================================
-- 来源: 1036_GUCbest_agg_plan.txt
================================================================================

-- [EXPLAIN]
explain select a , count ( 1 ) from agg_t1 group by a ;

-- [SESSION]
set best_agg_plan to 1 ;

-- [EXPLAIN]
explain select b , count ( 1 ) from t1 group by b ;

-- [SESSION]
set best_agg_plan to 2 ;

-- [EXPLAIN]
explain select b , count ( 1 ) from t1 group by b ;

-- [SESSION]
set best_agg_plan to 3 ;

-- [EXPLAIN]
explain select b , count ( 1 ) from t1 group by b ;


================================================================================
-- 来源: 1041_GUCrewrite_rule.txt
================================================================================

-- [SESSION]
show rewrite_rule;

-- [EXPLAIN]
explain (costs off) select * from t1, (select sum(c2), c1 from t2 group by c1) st2 where st2.c1 = t1.c1;

-- [SESSION]
set rewrite_rule='predpushnormal';

-- [EXPLAIN]
explain (costs off) select * from t1, (select sum(c2), c1 from t2 group by c1) st2 where st2.c1 = t1.c1;

-- [SESSION]
set rewrite_rule='predpushforce';

-- [EXPLAIN]
explain (costs off) select * from t1, (select sum(c2), c1 from t2 group by c1) st2 where st2.c1 = t1.c1;

-- [SESSION]
set rewrite_rule = 'predpush';

-- [EXPLAIN]
explain (costs off) select * from t1, (select sum(c2), c1 from t2 group by c1) st2 where st2.c1 = t1.c1;

-- [DDL]
create table t_rep(a int) distribute by replication;

-- [DDL]
create table t_dis(a int);

-- [SESSION]
set rewrite_rule = '';

-- [EXPLAIN]
explain (costs off) select * from t_dis where a = any(select a from t_rep) or a > 100;

-- [SESSION]
set rewrite_rule = disablerep;

-- [EXPLAIN]
explain (costs off) select * from t_dis where a = any(select a from t_rep) or a > 100;


================================================================================
-- 来源: 1042_DN GatherStream.txt
================================================================================

-- [SESSION]
set enable_broadcast=false;

-- [SESSION]
set explain_perf_mode=pretty;

-- [SESSION]
set enable_dngather=false;

-- [EXPLAIN]
explain select count(*) from t1, t2 where t1.b = t2.b;

-- [SESSION]
set enable_dngather=true;

-- [EXPLAIN]
explain select count(*) from t1, t2 where t1.b = t2.b;

-- [SESSION]
set enable_dngather=false;

-- [EXPLAIN]
explain select * from t1, t2, t3, t4 where t1.b = t2.b and t2.c = t3.c and t3.d = t4.d order by t1.a;

-- [SESSION]
set enable_dngather=true;

-- [EXPLAIN]
explain select * from t1, t2, t3, t4 where t1.b = t2.b and t2.c = t3.c and t3.d = t4.d order by t1.a;

-- [SESSION]
set enable_dngather=false;

-- [EXPLAIN]
explain select count(*) from t1, t2, t3, t4 where t1.b = t2.b and t2.c = t3.c and t3.d = t4.d group by t1.b order by t1.b;

-- [SESSION]
set enable_dngather=true;

-- [EXPLAIN]
explain select count(*) from t1, t2, t3, t4 where t1.b = t2.b and t2.c = t3.c and t3.d = t4.d group by t1.b order by t1.b;

-- [SESSION]
set explain_perf_mode=pretty;

-- [SESSION]
set enable_dngather=false;

-- [EXPLAIN]
explain select count(*) from t1 group by b;

-- [SESSION]
set enable_dngather=true;

-- [EXPLAIN]
explain select count(*) from t1 group by b;

-- [SESSION]
set enable_dngather=false;

-- [EXPLAIN]
explain select b from t1 group by b;

-- [SESSION]
set enable_dngather=true;

-- [EXPLAIN]
explain select b from t1 group by b;

-- [SESSION]
set explain_perf_mode=pretty;

-- [SESSION]
set enable_dngather=false;

-- [EXPLAIN]
explain select count(*) over (partition by b) a from t1;

-- [SESSION]
set enable_dngather=true;

-- [EXPLAIN]
explain select count(*) over (partition by b) a from t1;

-- [SESSION]
set enable_dngather=false;

-- [EXPLAIN]
explain select sum(b) over (partition by b) a from t1 group by b;

-- [SESSION]
set enable_dngather=true;

-- [EXPLAIN]
explain select sum(b) over (partition by b) a from t1 group by b;

-- [SESSION]
set explain_perf_mode=pretty;

-- [SESSION]
set enable_broadcast=false;

-- [SESSION]
set enable_dngather=false;

-- [EXPLAIN]
explain select t1.a, t2.b from t1, t2 where t1.b = t2.b union all select t3.a, t3.b from t3, t4 where t3.b = t4.b;

-- [SESSION]
set enable_dngather=true;

-- [EXPLAIN]
explain select t1.a, t2.b from t1, t2 where t1.b = t2.b union all select t3.a, t3.b from t3, t4 where t3.b = t4.b;

-- [SESSION]
set enable_dngather=false;

-- [EXPLAIN]
explain select t1.a, t2.b from t1, t2 where t1.b = t2.b union select t3.a, t3.b from t3, t4 where t3.b = t4.b order by a, b;

-- [SESSION]
set enable_dngather=true;

-- [EXPLAIN]
explain select t1.a, t2.b from t1, t2 where t1.b = t2.b union select t3.a, t3.b from t3, t4 where t3.b = t4.b order by a, b;

-- [SESSION]
set enable_dngather=false;

-- [EXPLAIN]
explain select b, count(*) from t1 group by b union all select b, count(*) from t2 group by b order by b;

-- [SESSION]
set enable_dngather=true;

-- [EXPLAIN]
explain select b, count(*) from t1 group by b union all select b, count(*) from t2 group by b order by b;

-- [SESSION]
set enable_dngather=false;

-- [EXPLAIN]
explain select b, count(*) from t1 group by b union all select count(distinct a) a , count(distinct b)b from t2 order by b;

-- [SESSION]
set enable_dngather=true;

-- [EXPLAIN]
explain select b, count(*) from t1 group by b union all select count(distinct a) a , count(distinct b)b from t2 order by b;


================================================================================
-- 来源: 1047_file_1047.txt
================================================================================

-- [DDL]
CREATE TABLE int_type_t1 ( IT_COL1 TINYINT, IT_COL2 TINYINT UNSIGNED );

--插入数据。
-- [DML_INSERT]
INSERT INTO int_type_t1 VALUES(10,20);

--查看数据。
-- [DQL]
SELECT * FROM int_type_t1;

--删除表。
-- [DDL]
DROP TABLE int_type_t1;

-- [DDL]
CREATE TABLE int_type_t2 ( a TINYINT , b TINYINT , c INTEGER , d INTEGER UNSIGNED , e BIGINT , f BIGINT UNSIGNED );

-- [DML_INSERT]
INSERT INTO int_type_t2 VALUES ( 100 , 10 , 1000 , 10000 , 200 , 2000 );

-- [DQL]
SELECT * FROM int_type_t2 ;

-- [DDL]
DROP TABLE int_type_t2 ;

-- [DDL]
CREATE TABLE decimal_type_t1 ( DT_COL1 DECIMAL(10,4) );

--插入数据。
-- [DML_INSERT]
INSERT INTO decimal_type_t1 VALUES(123456.122331);

--查询表中的数据。
-- [DQL]
SELECT * FROM decimal_type_t1;

--删除表。
-- [DDL]
DROP TABLE decimal_type_t1;

-- [DDL]
CREATE TABLE numeric_type_t1 ( NT_COL1 NUMERIC ( 10 , 4 ) );

-- [DML_INSERT]
INSERT INTO numeric_type_t1 VALUES ( 123456 . 12354 );

-- [DQL]
SELECT * FROM numeric_type_t1 ;

-- [DDL]
DROP TABLE numeric_type_t1 ;

-- [DDL]
CREATE TABLE smallserial_type_tab ( a SMALLSERIAL );

-- [DML_INSERT]
INSERT INTO smallserial_type_tab VALUES ( default );

-- [DML_INSERT]
INSERT INTO smallserial_type_tab VALUES ( default );

-- [DQL]
SELECT * FROM smallserial_type_tab ;

-- [DDL]
CREATE TABLE serial_type_tab ( b SERIAL );

-- [DML_INSERT]
INSERT INTO serial_type_tab VALUES ( default );

-- [DML_INSERT]
INSERT INTO serial_type_tab VALUES ( default );

-- [DQL]
SELECT * FROM serial_type_tab ;

-- [DDL]
CREATE TABLE bigserial_type_tab ( c BIGSERIAL );

-- [DML_INSERT]
INSERT INTO bigserial_type_tab VALUES ( default );

-- [DML_INSERT]
INSERT INTO bigserial_type_tab VALUES ( default );

-- [DQL]
SELECT * FROM bigserial_type_tab ;

-- [DDL]
DROP TABLE smallserial_type_tab ;

-- [DDL]
DROP TABLE serial_type_tab ;

-- [DDL]
DROP TABLE bigserial_type_tab ;

-- [DDL]
CREATE TABLE float_type_t2 ( FT_COL1 INTEGER , FT_COL2 FLOAT4 , FT_COL3 FLOAT8 , FT_COL4 FLOAT ( 3 ), FT_COL5 BINARY_DOUBLE , FT_COL6 DECIMAL ( 10 , 4 ), FT_COL7 INTEGER ( 6 , 3 ) ) DISTRIBUTE BY HASH ( ft_col1 );

-- [DML_INSERT]
INSERT INTO float_type_t2 VALUES ( 10 , 10 . 365456 , 123456 . 1234 , 10 . 3214 , 321 . 321 , 123 . 123654 , 123 . 123654 );

-- [DQL]
SELECT * FROM float_type_t2 ;

-- [DDL]
DROP TABLE float_type_t2 ;

-- [DDL]
CREATE DATABASE gaussdb_m WITH dbcompatibility 'MYSQL' ;

-- [OTHER]
\ c gaussdb_m --设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;


================================================================================
-- 来源: 1048_file_1048.txt
================================================================================

-- [DQL]
SELECT '12.34' :: float8 :: numeric :: money ;

-- [DQL]
SELECT '52093.89' :: money :: numeric :: float8 ;


================================================================================
-- 来源: 1049_file_1049.txt
================================================================================

-- [DDL]
CREATE TABLE bool_type_t1 ( BT_COL1 BOOLEAN , BT_COL2 TEXT ) DISTRIBUTE BY HASH ( BT_COL2 );

-- [DML_INSERT]
INSERT INTO bool_type_t1 VALUES ( TRUE , 'sic est' );

-- [DML_INSERT]
INSERT INTO bool_type_t1 VALUES ( FALSE , 'non est' );

-- [DQL]
SELECT * FROM bool_type_t1 ;

-- [DQL]
SELECT * FROM bool_type_t1 WHERE bt_col1 = 't' ;

-- [DDL]
DROP TABLE bool_type_t1 ;


================================================================================
-- 来源: 1050_file_1050.txt
================================================================================

-- [DDL]
CREATE TABLE varchar_maxlength_test1 (a int, b varchar, c int) DISTRIBUTE BY HASH (a);

-- varchar为1073741728，超过规定长度，插入失败
-- [DML_INSERT]
insert into varchar_maxlength_test1 values(1, repeat('a', 1073741728), 1);

-- varchar为1073741727，长度符合要求，插入成功
-- [DML_INSERT]
insert into varchar_maxlength_test1 values(1, repeat('a', 1073741727), 1);

-- 创建表，表中仅varchar一列，根据计算规则，varchar最大存储长度为1GB-85-4=
-- [DDL]
CREATE TABLE varchar_maxlength_test2 (a varchar) DISTRIBUTE BY HASH (a);

-- [DDL]
CREATE TABLE char_type_t1 ( CT_COL1 CHARACTER(4) )DISTRIBUTE BY HASH (CT_COL1);

--插入数据。
-- [DML_INSERT]
INSERT INTO char_type_t1 VALUES ('ok');

--查询表中的数据。
-- [DQL]
SELECT ct_col1, char_length(ct_col1) FROM char_type_t1;

--删除表。
-- [DDL]
DROP TABLE char_type_t1;

-- [DDL]
CREATE TABLE char_type_t2 ( CT_COL1 VARCHAR ( 5 ) ) DISTRIBUTE BY HASH ( CT_COL1 );

-- [DML_INSERT]
INSERT INTO char_type_t2 VALUES ( 'ok' );

-- [DML_INSERT]
INSERT INTO char_type_t2 VALUES ( 'good' );

-- [DML_INSERT]
INSERT INTO char_type_t2 VALUES ( 'too long' );

-- [DML_INSERT]
INSERT INTO char_type_t2 VALUES ( 'too long' :: varchar ( 5 ));

-- [DQL]
SELECT ct_col1 , char_length ( ct_col1 ) FROM char_type_t2 ;

-- [DDL]
DROP TABLE char_type_t2 ;

-- [DDL]
create database gaussdb_m with dbcompatibility 'MYSQL' ;

-- [OTHER]
\ c gaussdb_m -- 设置兼容版本控制参数 gaussdb_m =# set b_format_version = '5.7' ;


================================================================================
-- 来源: 1051_file_1051.txt
================================================================================

-- [DDL]
CREATE TABLE blob_type_t1 ( BT_COL1 INTEGER , BT_COL2 BLOB , BT_COL3 RAW , BT_COL4 BYTEA ) DISTRIBUTE BY REPLICATION ;

-- [DML_INSERT]
INSERT INTO blob_type_t1 VALUES ( 10 , empty_blob (), HEXTORAW ( 'DEADBEEF' ), E '\\xDEADBEEF' );

-- [DQL]
SELECT * FROM blob_type_t1 ;

-- [DDL]
DROP TABLE blob_type_t1 ;

-- [DDL]
CREATE DATABASE gaussdb_m WITH dbcompatibility 'MYSQL' ;

-- [OTHER]
\ c gaussdb_m --设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;


================================================================================
-- 来源: 1052__.txt
================================================================================

-- [DDL]
CREATE TABLE date_type_tab ( coll date );

-- [DML_INSERT]
INSERT INTO date_type_tab VALUES ( date '12-10-2010' );

-- [DQL]
SELECT * FROM date_type_tab ;

-- [DDL]
DROP TABLE date_type_tab ;

-- [DDL]
CREATE TABLE time_type_tab ( da time without time zone , dai time with time zone , dfgh timestamp without time zone , dfga timestamp with time zone , vbg smalldatetime );

-- [DML_INSERT]
INSERT INTO time_type_tab VALUES ( '21:21:21' , '21:21:21 pst' , '2010-12-12' , '2013-12-11 pst' , '2003-04-12 04:05:06' );

-- [DQL]
SELECT * FROM time_type_tab ;

-- [DDL]
DROP TABLE time_type_tab ;

-- [DDL]
CREATE TABLE day_type_tab ( a int , b INTERVAL DAY ( 3 ) TO SECOND ( 4 ));

-- [DML_INSERT]
INSERT INTO day_type_tab VALUES ( 1 , INTERVAL '3' DAY );

-- [DQL]
SELECT * FROM day_type_tab ;

-- [DDL]
DROP TABLE day_type_tab ;

-- [DDL]
CREATE TABLE year_type_tab ( a int , b interval year ( 6 ));

-- [DML_INSERT]
INSERT INTO year_type_tab VALUES ( 1 , interval '2' year );

-- [DQL]
SELECT * FROM year_type_tab ;

-- [DQL]
SELECT TIME 'allballs' ;

-- [DDL]
DROP TABLE year_type_tab ;

-- [DDL]
create database gaussdb_m dbcompatibility = 'MYSQL' ;

-- [DDL]
CREATE TABLE date_type_tab ( coll date );

-- [DML_INSERT]
INSERT INTO date_type_tab VALUES ( date '12-10-2010' );

-- [DQL]
SELECT * FROM date_type_tab ;

-- [SESSION]
SHOW datestyle ;

-- [SESSION]
SET datestyle = 'YMD' ;

-- [DML_INSERT]
INSERT INTO date_type_tab VALUES ( date '2010-12-11' );

-- [DQL]
SELECT * FROM date_type_tab ;

-- [DDL]
DROP TABLE date_type_tab ;

-- [DQL]
SELECT time '04:05:06' ;

-- [DQL]
SELECT time '04:05:06 PST' ;

-- [DQL]
SELECT time with time zone '04:05:06 PST' ;

-- [DDL]
CREATE TABLE realtime_type_special(col1 varchar(20), col2 date, col3 timestamp, col4 time);

--插入数据。
-- [DML_INSERT]
INSERT INTO realtime_type_special VALUES('epoch', 'epoch', 'epoch', NULL);

-- [DML_INSERT]
INSERT INTO realtime_type_special VALUES('now', 'now', 'now', 'now');

-- [DML_INSERT]
INSERT INTO realtime_type_special VALUES('today', 'today', 'today', NULL);

-- [DML_INSERT]
INSERT INTO realtime_type_special VALUES('tomorrow', 'tomorrow', 'tomorrow', NULL);

-- [DML_INSERT]
INSERT INTO realtime_type_special VALUES('yesterday', 'yesterday', 'yesterday', NULL);

--查看数据。
-- [DQL]
SELECT * FROM realtime_type_special;

-- [DQL]
SELECT * FROM realtime_type_special WHERE col3 < 'infinity';

-- [DQL]
SELECT * FROM realtime_type_special WHERE col3 > '-infinity';

-- [DQL]
SELECT * FROM realtime_type_special WHERE col3 > 'now';

-- [DQL]
SELECT * FROM realtime_type_special WHERE col3 = 'today';

-- [DQL]
SELECT * FROM realtime_type_special WHERE col3 = 'tomorrow';

-- [DQL]
SELECT * FROM realtime_type_special WHERE col3 > 'yesterday';

--删除表。
-- [DDL]
DROP TABLE realtime_type_special;

-- [DDL]
CREATE TABLE reltime_type_tab ( col1 character ( 30 ), col2 reltime );

-- [DML_INSERT]
INSERT INTO reltime_type_tab VALUES ( '90' , '90' );

-- [DML_INSERT]
INSERT INTO reltime_type_tab VALUES ( '-366' , '-366' );

-- [DML_INSERT]
INSERT INTO reltime_type_tab VALUES ( '1975.25' , '1975.25' );

-- [DML_INSERT]
INSERT INTO reltime_type_tab VALUES ( '-2 YEARS +5 MONTHS 10 DAYS' , '-2 YEARS +5 MONTHS 10 DAYS' );

-- [DML_INSERT]
INSERT INTO reltime_type_tab VALUES ( '30 DAYS 12:00:00' , '30 DAYS 12:00:00' );

-- [DML_INSERT]
INSERT INTO reltime_type_tab VALUES ( 'P-1.1Y10M' , 'P-1.1Y10M' );

-- [DQL]
SELECT * FROM reltime_type_tab ;

-- [DDL]
DROP TABLE reltime_type_tab ;


================================================================================
-- 来源: 1053_file_1053.txt
================================================================================

-- [DQL]
SELECT point(1.1, 2.2);

-- [DQL]
SELECT lseg(point(1.1, 2.2), point(3.3, 4.4));

-- [DQL]
SELECT box(point(1.1, 2.2), point(3.3, 4.4));

-- [DQL]
SELECT path(polygon '((0,0),(1,1),(2,0))');

-- [DQL]
SELECT polygon(box '((0,0),(1,1))');

-- [DQL]
SELECT circle(point(0,0),1);


================================================================================
-- 来源: 1055_file_1055.txt
================================================================================

-- [DDL]
CREATE TABLE bit_type_t1 ( BT_COL1 INTEGER , BT_COL2 BIT ( 3 ), BT_COL3 BIT VARYING ( 5 ) ) DISTRIBUTE BY REPLICATION ;

-- [DML_INSERT]
INSERT INTO bit_type_t1 VALUES ( 1 , B '101' , B '00' );

-- [DML_INSERT]
INSERT INTO bit_type_t1 VALUES ( 2 , B '10' , B '101' );

-- [DML_INSERT]
INSERT INTO bit_type_t1 VALUES ( 2 , B '10' :: bit ( 3 ), B '101' );

-- [DQL]
SELECT * FROM bit_type_t1 ;

-- [DDL]
DROP TABLE bit_type_t1 ;


================================================================================
-- 来源: 1056_file_1056.txt
================================================================================

-- [DQL]
SELECT 'a fat cat sat on a mat and ate a fat rat' :: tsvector ;

-- [DQL]
SELECT $$ the lexeme ' ' contains spaces $$ :: tsvector ;

-- [DQL]
SELECT $$ the lexeme 'Joe''s' contains a quote $$ :: tsvector ;

-- [DQL]
SELECT 'a:1 fat:2 cat:3 sat:4 on:5 a:6 mat:7 and:8 ate:9 a:10 fat:11 rat:12' :: tsvector ;

-- [DQL]
SELECT 'a:1A fat:2B,4C cat:5D' :: tsvector ;

-- [DQL]
SELECT 'The Fat Rats' :: tsvector ;

-- [DQL]
SELECT to_tsvector ( 'english' , 'The Fat Rats' );

-- [DQL]
SELECT 'fat & rat' :: tsquery ;

-- [DQL]
SELECT 'fat & (rat | cat)' :: tsquery ;

-- [DQL]
SELECT 'fat & rat & ! cat' :: tsquery ;

-- [DQL]
SELECT 'fat:ab & cat' :: tsquery ;

-- [DQL]
SELECT 'super:*' :: tsquery ;

-- [DQL]
SELECT to_tsvector ( 'seriousness' ) @@ to_tsquery ( 'series:*' ) AS RESULT ;

-- [DQL]
SELECT to_tsquery ( 'series:*' );

-- [DQL]
SELECT to_tsquery ( 'Fat:ab & Cats' );


================================================================================
-- 来源: 1058_JSON_JSONB.txt
================================================================================

-- [DQL]
SELECT 'null'::json;

-- [DQL]
SELECT 'NULL'::jsonb;

-- [DQL]
SELECT '1'::json;

-- [DQL]
SELECT '-1.5'::json;

-- [DQL]
SELECT '-1.5e-5'::jsonb, '-1.5e+2'::jsonb;

-- [DQL]
SELECT '001'::json, '+15'::json, 'NaN'::json;

-- [DQL]
SELECT 'true'::json;

-- [DQL]
SELECT '"a"'::json;

-- [DQL]
SELECT '[1, 2, "foo", null]'::json;

-- [DQL]
SELECT '[]'::json;

-- [DQL]
SELECT '[1, 2, "foo", null, [[]], {}]'::jsonb;

-- [DQL]
SELECT '{}'::json;

-- [DQL]
SELECT '{"foo": [true, "bar"], "tags": {"a": 1, "b": null}}'::jsonb;

-- [DQL]
SELECT ' [1, " a ", {"a" :1 }] '::jsonb;

-- [DQL]
SELECT '{"a" : 1, "a" : 2}'::jsonb;

-- [DQL]
SELECT '{"aa" : 1, "b" : 2, "a" : 3}'::jsonb;


================================================================================
-- 来源: 1059_HLL.txt
================================================================================

-- [DDL]
CREATE TABLE t1 ( id integer , set hll );

-- [OTHER]
\ d t1 Table "public.t1" Column | Type | Modifiers --------+---------+----------- id | integer | set | hll | -- 创建hll类型的表，指定前两个入参，后两个采用默认值。

-- [DDL]
CREATE TABLE t2 ( id integer , set hll ( 12 , 4 ));

-- [OTHER]
\ d t2 Table "public.t2" Column | Type | Modifiers --------+----------------+----------- id | integer | set | hll ( 12 , 4 , 12 , 0 ) | --创建hll类型的表，指定第三个入参，其余采用默认值。

-- [DDL]
CREATE TABLE t3 ( id int , set hll ( - 1 , - 1 , 8 , - 1 ));

-- [OTHER]
\ d t3 Table "public.t3" Column | Type | Modifiers --------+----------------+----------- id | integer | set | hll ( 14 , 10 , 8 , 0 ) | --创建hll类型的表，指定入参不合法报错。

-- [DDL]
CREATE TABLE t4 ( id int , set hll ( 5 , - 1 ));

-- [DDL]
DROP TABLE t1 , t2 , t3 ;

-- [DDL]
CREATE TABLE t1 ( id integer , set hll ( 14 ));

-- [DML_INSERT]
INSERT INTO t1 VALUES ( 1 , hll_empty ( 14 , - 1 ));

-- [DML_INSERT]
INSERT INTO t1 ( id , set ) VALUES ( 1 , hll_empty ( 14 , 5 ));

-- [DDL]
DROP TABLE t1 ;

-- [DDL]
CREATE TABLE helloworld ( id integer , set hll );

-- [DML_INSERT]
INSERT INTO helloworld ( id , set ) VALUES ( 1 , hll_empty ());

-- [DML_UPDATE]
UPDATE helloworld SET set = hll_add ( set , hll_hash_integer ( 12345 )) WHERE id = 1 ;

-- [DML_UPDATE]
UPDATE helloworld SET set = hll_add ( set , hll_hash_text ( 'hello world' )) WHERE id = 1 ;

-- [DQL]
SELECT hll_cardinality ( set ) FROM helloworld WHERE id = 1 ;

-- [DDL]
DROP TABLE helloworld ;

-- [DDL]
CREATE TABLE facts ( date date , user_id integer );

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-20' , generate_series ( 1 , 100 ));

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-21' , generate_series ( 1 , 200 ));

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-22' , generate_series ( 1 , 300 ));

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-23' , generate_series ( 1 , 400 ));

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-24' , generate_series ( 1 , 500 ));

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-25' , generate_series ( 1 , 600 ));

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-26' , generate_series ( 1 , 700 ));

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-27' , generate_series ( 1 , 800 ));

-- [DDL]
create TABLE daily_uniques ( date date UNIQUE , users hll );

-- [DML_INSERT]
INSERT INTO daily_uniques ( date , users ) SELECT date , hll_add_agg ( hll_hash_integer ( user_id )) FROM facts GROUP BY 1 ;

-- [DQL]
SELECT date , hll_cardinality ( users ) FROM daily_uniques ORDER BY date ;

-- [DQL]
SELECT hll_cardinality ( hll_union_agg ( users )) FROM daily_uniques WHERE date >= '2019-02-20' :: date AND date <= '2019-02-26' :: date ;

-- [DQL]
SELECT date , ( # hll_union_agg ( users ) OVER two_days ) - # users AS lost_uniques FROM daily_uniques WINDOW two_days AS ( ORDER BY date ASC ROWS 1 PRECEDING );

-- [DDL]
DROP TABLE facts ;

-- [DDL]
DROP TABLE daily_uniques ； 场景3：“插入数据不满足hll数据结构要求” 当用户给hll类型的字段插入数据的时候，必须保证插入的数据满足hll数据结构要求，如果解析后不满足就会报错。如下示例中： 插入数据'E\\1234'时，该数据不满足hll数据结构 要求 ，不能解析成功因此失败报错。 1 2 3

-- [DDL]
CREATE TABLE test ( id integer , set hll );

-- [DML_INSERT]
INSERT INTO test VALUES ( 1 , 'E\\1234' );

-- [DDL]
DROP TABLE test ;


================================================================================
-- 来源: 1060_file_1060.txt
================================================================================

-- [DDL]
CREATE TABLE reservation (room int, during tsrange);

-- [DML_INSERT]
INSERT INTO reservation VALUES (1108, '[2010-01-01 14:30, 2010-01-01 15:30)');

-- 包含 。
-- [DQL]
SELECT int4range(10, 20) @> 3;

-- 判断是否重叠
-- [DQL]
SELECT numrange(11.1, 22.2) && numrange(20.0, 30.0);

-- 抽取上界 。
-- [DQL]
SELECT upper(int8range(15, 25));

-- 计算交集 。
-- [DQL]
SELECT int4range(10, 20) * int4range(15, 25);

-- 判断范围是否为空 。
-- [DQL]
SELECT isempty(numrange(1, 5));

-- [DQL]
SELECT '[3,7)'::int4range;

-- 既不包括 3 也不包括 7，但是包括之间的所有点 。
-- [DQL]
SELECT '(3,7)'::int4range;

-- 只包括单独一个点 4 。
-- [DQL]
SELECT '[4,4]'::int4range;

-- 不包括点（并且将被标准化为 '空'） 。
-- [DQL]
SELECT '[4,4)'::int4range;

-- [DQL]
SELECT numrange(1.0, 14.0, '(]');

-- 如果第三个参数被忽略，则假定为 '[)'。
-- [DQL]
SELECT numrange(1.0, 14.0);

-- 尽管这里指定了 '(]'，显示时该值将被转换成标准形式，因为 int8range 是一种离散范围类型（见下文）。
-- [DQL]
SELECT int8range(1, 14, '(]');

-- 为一个界限使用 NULL 导致范围在那一边是无界的。
-- [DQL]
SELECT numrange(NULL, 2.2);


================================================================================
-- 来源: 1061_file_1061.txt
================================================================================

-- [DQL]
SELECT oid FROM pg_class WHERE relname = 'pg_type' ;

-- [DQL]
SELECT attrelid , attname , atttypid , attstattarget FROM pg_attribute WHERE attrelid = 'pg_type' :: REGCLASS ;


================================================================================
-- 来源: 1062_file_1062.txt
================================================================================

-- [DDL]
create table t1 ( a int );

-- [DML_INSERT]
insert into t1 values ( 1 ),( 2 );

-- [DDL]
CREATE OR REPLACE FUNCTION showall () RETURNS SETOF record AS $$ SELECT count ( * ) from t1 ;

-- [DQL]
SELECT showall ();

-- [DDL]
DROP FUNCTION showall ();

-- [DDL]
drop table t1 ;


================================================================================
-- 来源: 1065_XML.txt
================================================================================

-- [DDL]
CREATE TABLE xmltest ( id int, data xml );

-- [DML_INSERT]
INSERT INTO xmltest VALUES (1, 'one');

-- [DML_INSERT]
INSERT INTO xmltest VALUES (2, 'two');

-- [DQL]
SELECT * FROM xmltest ORDER BY 1;

-- [DQL]
SELECT xmlconcat(xmlcomment('hello'), xmlelement(NAME qux, 'xml'), xmlcomment('world'));

-- [DDL]
DROP TABLE xmltest;


================================================================================
-- 来源: 1066_XMLTYPE.txt
================================================================================

-- [DDL]
CREATE TABLE xmltypetest(id int, data xmltype);

-- [DML_INSERT]
INSERT INTO xmltypetest VALUES (1, '<ss/>');

-- [DML_INSERT]
INSERT INTO xmltypetest VALUES (2, '<xx/>');

-- [DQL]
SELECT * FROM xmltypetest ORDER BY 1;


================================================================================
-- 来源: 1067_aclitem.txt
================================================================================

-- [DDL]
CREATE TABLE table_acl (id int,priv aclitem,privs aclitem[]);

-- [DML_INSERT]
INSERT INTO table_acl VALUES (1,'user1=arw/omm','{omm=d/user2,omm=w/omm}');

-- [DML_INSERT]
INSERT INTO table_acl VALUES (2,'user1=aw/omm','{omm=d/user2}');

-- [DQL]
SELECT * FROM table_acl;


================================================================================
-- 来源: 1068_file_1068.txt
================================================================================

-- [DQL]
SELECT CURRENT_ROLE ;

-- [DQL]
SELECT CURRENT_SCHEMA ;

-- [DQL]
SELECT CURRENT_USER ;

-- [DQL]
SELECT LOCALTIMESTAMP ;

-- [DQL]
SELECT SESSION_USER ;

-- [DQL]
SELECT SYSDATE ;

-- [DQL]
SELECT USER ;


================================================================================
-- 来源: 1072_file_1072.txt
================================================================================

-- [DQL]
SELECT bit_length ( 'world' );

-- [DQL]
SELECT btrim ( 'sring' , 'ing' );

-- [DQL]
SELECT char_length ( 'hello' );

-- [DQL]
select dump ( 'abc测试' );

-- [DQL]
SELECT instr ( 'abcdabcdabcd' , 'bcd' , 2 , 2 );

-- [DQL]
SELECT instrb ( 'abcdabcdabcd' , 'bcd' , 2 , 2 );

-- [DQL]
SELECT lengthb ( 'hello' );

-- [DQL]
SELECT left ( 'abcde' , 2 );

-- [DQL]
SELECT length ( 'jose' , 'UTF8' );

-- [DQL]
SELECT lpad ( 'hi' , 5 , 'xyza' );

-- [DQL]
select lpad ( 'expr1' , 7 , '中国' );

-- [DQL]
select lpad ( 'expr1' , 8 , '中国' );

-- [DQL]
SELECT notlike ( 1 , 2 );

-- [DQL]
SELECT notlike ( 1 , 1 );

-- [DQL]
SELECT octet_length ( 'jose' );

-- [DQL]
SELECT overlay ( 'hello' placing 'world' from 2 for 3 );

-- [DQL]
SELECT position ( 'ing' in 'string' );

-- [DQL]
SELECT pg_client_encoding ();

-- [DQL]
SELECT quote_ident ( 'hello world' );

-- [DQL]
SELECT quote_literal ( 'hello' );

-- [DQL]
SELECT quote_literal ( E 'O\' hello ');

-- [DQL]
SELECT quote_literal ( 'O\hello' );

-- [DQL]
SELECT quote_literal ( NULL );

-- [DQL]
SELECT quote_literal ( 42 . 5 );

-- [DQL]
SELECT quote_literal ( E 'O\' 42 . 5 ');

-- [DQL]
SELECT quote_literal ( 'O\42.5' );

-- [DQL]
SELECT quote_nullable ( 'hello' );

-- [DQL]
SELECT quote_nullable ( E 'O\' hello ');

-- [DQL]
SELECT quote_nullable ( 'O\hello' );

-- [DQL]
SELECT quote_nullable ( NULL );

-- [DQL]
SELECT quote_nullable ( 42 . 5 );

-- [DQL]
SELECT quote_nullable ( E 'O\' 42 . 5 ');

-- [DQL]
SELECT quote_nullable ( 'O\42.5' );

-- [DQL]
SELECT quote_nullable ( NULL );

-- [DQL]
select substring_inner ( 'adcde' , 2 , 3 );

-- [DQL]
SELECT substring ( 'Thomas' from 2 for 3 );

-- [DQL]
select substring ( 'substrteststring' , - 5 , 5 );

-- [DQL]
SELECT substring_index ( 'Test1splitTest2splitTest3splitTest4' , 'split' , 2 );

-- [DQL]
SELECT substring_index ( 'Test1splitTest2splitTest3splitTest4' , 'split' , - 2 );

-- [DQL]
SELECT substring ( 'Thomas' from '...$' );

-- [DQL]
SELECT substring ( 'foobar' from 'o(.)b' );

-- [DQL]
SELECT substring ( 'foobar' from '(o(.)b)' );

-- [DQL]
SELECT substring ( 'Thomas' from '%#"o_a#"_' for '#' );

-- [DQL]
SELECT rawcat ( 'ab' , 'cd' );

-- [DQL]
SELECT regexp_like ( 'str' , '[ac]' );

-- [DQL]
SELECT regexp_substr ( 'str' , '[ac]' );

-- [DQL]
SELECT regexp_substr ( 'foobarbaz' , 'b(..)' , 3 , 2 ) AS RESULT ;

-- [DQL]
SELECT regexp_count('foobarbaz','b(..)', 5) AS RESULT;

-- [DQL]
SELECT regexp_instr('foobarbaz','b(..)', 1, 1, 0) AS RESULT;

-- [DQL]
SELECT regexp_instr('foobarbaz','b(..)', 1, 2, 0) AS RESULT;

-- [DQL]
SELECT regexp_matches ( 'foobarbequebaz' , '(bar)(beque)' );

-- [DQL]
SELECT regexp_matches ( 'foobarbequebaz' , 'barbeque' );

-- [DQL]
SELECT regexp_matches ( 'foobarbequebazilbarfbonk' , '(b[^b]+)(b[^b]+)' , 'g' );

-- [DQL]
SELECT regexp_match('foobarbequebaz', '(bar)(beque)');

-- [DQL]
SELECT (regexp_match('foobarbequebaz', 'bar.*que'))[1];

-- [DQL]
SELECT regexp_match('Learning #PostgreSQL', 'R', 'c');

-- [DQL]
SELECT regexp_match('hello world', 'h e l l o', 'x');

-- [DQL]
SELECT regexp_split_to_array ( 'hello world' , E '\\s+' );

-- [DQL]
SELECT regexp_split_to_table ( 'hello world' , E '\\s+' );

-- [DQL]
SELECT repeat ( 'Pg' , 4 );

-- [DQL]
SELECT replace ( 'abcdefabcdef' , 'cd' , 'XXX' );

-- [DQL]
SELECT replace ( 'abcdefabcdef' , 'cd' );

-- [DQL]
SELECT reverse ( 'abcde' );

-- [DQL]
SELECT right ( 'abcde' , 2 );

-- [DQL]
SELECT right ( 'abcde' , - 2 );

-- [DQL]
SELECT rpad ( 'hi' , 5 , 'xy' );

-- [DQL]
select rpad ( 'expr1' , 7 , '中国' ) || '*' ;

-- [DQL]
select rpad ( 'expr1' , 8 , '中国' ) || '*' ;

-- [DQL]
SELECT substr ( 'stringtest' FROM 4 );

-- [DQL]
SELECT substr ( 'stringtest' , 4 );

-- [DQL]
SELECT substr ( 'stringtest' , - 4 );

-- [DQL]
SELECT substr ( 'stringtest' , 11 );

-- [DQL]
SELECT substr ( 'teststring' FROM 5 FOR 2 );

-- [DQL]
SELECT substr ( 'teststring' , 5 , 2 );

-- [DQL]
SELECT substr ( 'teststring' , 5 , 10 );

-- [DQL]
SELECT substrb ( 'string' , 2 );

-- [DQL]
SELECT substrb ( 'string' , - 2 );

-- [DQL]
SELECT substrb ( 'string' , 10 );

-- [DQL]
SELECT substrb ( '数据库' , 1 );

-- [DQL]
SELECT substrb ( '数据库' , 2 );

-- [DQL]
SELECT substrb ( 'string' , 2 , 3 );

-- [DQL]
SELECT substrb ( 'string' , 2 , 10 );

-- [DQL]
SELECT substrb ( '数据库' , 4 , 3 );

-- [DQL]
SELECT substrb ( '数据库' , 2 , 6 ) = ' 据' as result ;

-- [DQL]
SELECT substrb ( '数据库' , 2 , 6 ) = ' 据 ' as result ;

-- [DQL]
SELECT 'MPP' || 'DB' AS RESULT ;

-- [DQL]
SELECT 'Value: ' || 42 AS RESULT ;

-- [DQL]
SELECT split_part ( 'abc~@~def~@~ghi' , '~@~' , 2 );

-- [DQL]
SELECT strpos ( 'source' , 'rc' );

-- [DQL]
SELECT to_hex ( 2147483647 );

-- [DQL]
SELECT translate ( '12345' , '143' , 'ax' );

-- [DQL]
SELECT length ( 'abcd' );

-- [DQL]
SELECT length ( '汉字abc' );

-- [DDL]
CREATE DATABASE gaussdb_m WITH dbcompatibility 'MYSQL' ;

-- [OTHER]
\ c gaussdb_m -- 设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

-- [DQL]
SELECT lengthb ( 'Chinese' );

-- [DQL]
select to_single_byte ( 'AB123' );

-- [DQL]
select to_multi_byte ( 'ABC123' );

-- [DQL]
SELECT trim ( BOTH 'x' FROM 'xTomxx' );

-- [DQL]
SELECT trim ( LEADING 'x' FROM 'xTomxx' );

-- [DQL]
SELECT trim ( TRAILING 'x' FROM 'xTomxx' );

-- [DQL]
SELECT rtrim ( 'TRIMxxxx' , 'x' );

-- [DQL]
SELECT ltrim ( 'xxxxTRIM' , 'x' );

-- [DQL]
SELECT upper ( 'tom' );

-- [DQL]
SELECT lower ( 'TOM' );

-- [DQL]
SELECT nls_upper ( 'gro?e' );

-- [DQL]
SELECT nls_upper ( 'gro?e' , 'nls_sort = XGerman' );

-- [DQL]
SELECT nls_lower ( 'INDIVISIBILITY' );

-- [DQL]
SELECT nls_lower ( 'INDIVISIBILITY' , 'nls_sort = XTurkish' );

-- [DQL]
SELECT instr ( 'corporate floor' , 'or' , 3 );

-- [DQL]
SELECT instr ( 'corporate floor' , 'or' , - 3 , 2 );

-- [DQL]
SELECT initcap ( 'hi THOMAS' );

-- [DQL]
SELECT ascii ( 'xyz' );

-- [DQL]
SELECT ascii2 ( 'xyz' );

-- [DQL]
select ascii2 ( '中xyz' );

-- [DQL]
SELECT asciistr ( 'xyz中' );

-- [DQL]
select unistr ( 'abc\0041\4E2D' );

-- [DQL]
select vsize ( 'abc测试' );

-- [DQL]
SELECT replace ( 'jack and jue' , 'j' , 'bl' );

-- [DQL]
SELECT concat ( 'Hello' , ' World!' );

-- [DQL]
SELECT concat ( 'Hello' , NULL );

-- [DDL]
CREATE TABLE test_space ( c char ( 10 ));

-- [DML_INSERT]
INSERT INTO test_space values ( 'a' );

-- [DQL]
SELECT * FROM test_space WHERE c = 'a ' ;

-- [DQL]
SELECT * FROM test_space WHERE c = 'a' || ' ' ;

-- [DQL]
SELECT chr ( 65 );

-- [DQL]
select chr ( 19968 );

-- [DQL]
SELECT chr ( 65 );

-- [DQL]
select chr ( 16705 );

-- [DQL]
select chr ( 4259905 );

-- [DQL]
SELECT nchr ( 65 );

-- [DQL]
select nchr ( 14989440 );

-- [DQL]
select nchr ( 14989440 );

-- [DQL]
select nchr ( 4321090 );

-- [DQL]
select nchr ( 14989440 );

-- [DQL]
select nchr ( 14989440 );

-- [DQL]
SELECT regexp_substr ( '500 Hello World, Redwood Shores, CA' , ',[^,]+,' ) "REGEXPR_SUBSTR" ;

-- [DQL]
SELECT regexp_replace ( 'Thomas' , '.[mN]a.' , 'M' );

-- [DQL]
SELECT regexp_replace ( 'foobarbaz' , 'b(..)' , E 'X\\1Y' , 'g' ) AS RESULT ;

-- [DQL]
SELECT regexp_replace('foobarbaz','b(..)', E'X\\1Y', 2, 2, 'n') AS RESULT;

-- [DQL]
SELECT concat_ws ( ',' , 'ABCDE' , 2 , NULL , 22 );

-- [DDL]
create table test ( a text );

-- [DML_INSERT]
insert into test ( a ) values ( 'abC 不' );

-- [DML_INSERT]
insert into test ( a ) values ( 'abC 啊' );

-- [DML_INSERT]
insert into test ( a ) values ( 'abc 啊' );

-- [DQL]
select * from test order by nlssort ( a , 'nls_sort=schinese_pinyin_m' );

-- [DQL]
select * from test order by nlssort ( a , 'nls_sort=generic_m_ci' );

-- [DQL]
SELECT convert ( 'text_in_utf8' , 'UTF8' , 'GBK' );

-- [SESSION]
show server_encoding ;

-- [DQL]
SELECT convert_from ( 'some text' , 'GBK' );

-- [DQL]
SELECT convert ( 'asdas' using 'gbk' );

-- [DQL]
SELECT convert_from ( 'text_in_utf8' , 'UTF8' );

-- [DQL]
SELECT convert_to ( 'some text' , 'UTF8' );

-- [DQL]
SELECT 'AA_BBCC' LIKE '%A@_B%' ESCAPE '@' AS RESULT ;

-- [DQL]
SELECT 'AA_BBCC' LIKE '%A@_B%' AS RESULT ;

-- [DQL]
SELECT 'AA@_BBCC' LIKE '%A@_B%' AS RESULT ;

-- [DQL]
SELECT regexp_like ( 'ABC' , '[A-Z]' );

-- [DQL]
SELECT regexp_like ( 'ABC' , '[D-Z]' );

-- [DQL]
SELECT regexp_like ( 'ABC' , '[A-Z]' , 'i' );

-- [DQL]
SELECT regexp_like ( 'ABC' , '[A-Z]' );

-- [DQL]
SELECT format ( 'Hello %s, %1$s' , 'World' );

-- [DQL]
SELECT md5 ( 'ABC' );

-- [DQL]
select sha ( 'ABC' );

-- [DQL]
select sha1 ( 'ABC' );

-- [DQL]
select sha2 ( 'ABC' , 224 );

-- [DQL]
select sha2 ( 'ABC' , 256 );

-- [DQL]
select sha2 ( 'ABC' , 0 );

-- [DQL]
SELECT decode ( 'MTIzAAE=' , 'base64' );

-- [DQL]
select similar_escape('\s+ab','2');

-- [DQL]
select find_in_set ( 'ee' , 'a,ee,c' );

-- [DQL]
SELECT encode ( E '123\\000\\001' , 'base64' );

-- [SESSION]
SET max_datanode_for_plan = 64 ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- [DQL]
SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- [DDL]
CREATE EXTENSION pkg_bpchar_opc ;

-- [SESSION]
SET max_datanode_for_plan = 64 ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- [DQL]
SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- [DDL]
DROP EXTENSION pkg_bpchar_opc ;

-- [SESSION]
SET max_datanode_for_plan = 64 ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- [DQL]
SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- [DQL]
SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: text ;

-- [DDL]
CREATE EXTENSION pkg_bpchar_opc ;

-- [SESSION]
SET max_datanode_for_plan = 64 ;

-- [EXPLAIN]
explain select * from logs_text t1 where t1 . log_id = 'FE306991300002 ' :: bpchar ;

-- [DQL]
SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- [DQL]
SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: text ;

-- [DDL]
DROP EXTENSION pkg_bpchar_opc ;

-- [SESSION]
SET max_datanode_for_plan = 64 ;

-- [DQL]
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- [DQL]
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = 'FE306991300002 ' ;

-- [DDL]
CREATE EXTENSION pkg_bpchar_opc ;

-- [SESSION]
SET max_datanode_for_plan = 64 ;

-- [DQL]
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- [DQL]
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = 'FE306991300002 ' ;

-- [DDL]
DROP EXTENSION pkg_bpchar_opc ;


================================================================================
-- 来源: 1073_file_1073.txt
================================================================================

-- [DQL]
SELECT octet_length ( E 'jo\\000se' :: bytea ) AS RESULT ;

-- [DQL]
SELECT overlay ( E 'Th\\000omas' :: bytea placing E '\\002\\003' :: bytea from 2 for 3 ) AS RESULT ;

-- [DQL]
SELECT position ( E '\\000om' :: bytea in E 'Th\\000omas' :: bytea ) AS RESULT ;

-- [DQL]
SELECT substring ( E 'Th\\000omas' :: bytea from 2 for 3 ) AS RESULT ;

-- [DQL]
select substr ( E 'Th\\000omas' :: bytea , 2 , 3 ) as result ;

-- [DQL]
SELECT trim ( E '\\000' :: bytea from E '\\000Tom\\000' :: bytea ) AS RESULT ;

-- [DQL]
SELECT btrim ( E '\\000trim\\000' :: bytea , E '\\000' :: bytea ) AS RESULT ;

-- [DQL]
SELECT get_bit ( E 'Th\\000omas' :: bytea , 45 ) AS RESULT ;

-- [DQL]
SELECT get_byte ( E 'Th\\000omas' :: bytea , 4 ) AS RESULT ;

-- [DQL]
SELECT set_bit ( E 'Th\\000omas' :: bytea , 45 , 0 ) AS RESULT ;

-- [DQL]
SELECT set_byte ( E 'Th\\000omas' :: bytea , 4 , 64 ) AS RESULT ;


================================================================================
-- 来源: 1074_file_1074.txt
================================================================================

-- [DQL]
SELECT B '10001' || B '011' AS RESULT ;

-- [DQL]
SELECT B '10001' & B '01101' AS RESULT ;

-- [DQL]
SELECT B '10001' | B '01101' AS RESULT ;

-- [DQL]
SELECT B '10001' # B '01101' AS RESULT ;

-- [DQL]
SELECT ~ B '10001' AS RESULT ;

-- [DQL]
SELECT B '10001' << 3 AS RESULT ;

-- [DQL]
SELECT B '10001' >> 2 AS RESULT ;

-- [DQL]
SELECT 44 :: bit ( 10 ) AS RESULT ;

-- [DQL]
SELECT 44 :: bit ( 3 ) AS RESULT ;

-- [DQL]
SELECT cast ( - 44 as bit ( 12 )) AS RESULT ;

-- [DQL]
SELECT '1110' :: bit ( 4 ):: integer AS RESULT ;

-- [DQL]
select substring ( '10101111' :: bit ( 8 ), 2 );


================================================================================
-- 来源: 1075_file_1075.txt
================================================================================

-- [DQL]
SELECT 'abc' LIKE 'abc' AS RESULT ;

-- [DQL]
SELECT 'abc' LIKE 'a%' AS RESULT ;

-- [DQL]
SELECT 'abc' LIKE '_b_' AS RESULT ;

-- [DQL]
SELECT 'abc' LIKE 'c' AS RESULT ;

-- [DQL]
SELECT 'abc' SIMILAR TO 'abc' AS RESULT ;

-- [DQL]
SELECT 'abc' SIMILAR TO 'a' AS RESULT ;

-- [DQL]
SELECT 'abc' SIMILAR TO '%(b|d)%' AS RESULT ;

-- [DQL]
SELECT 'abc' SIMILAR TO '(b|c)%' AS RESULT ;

-- [DQL]
SELECT 'abc' ~ 'Abc' AS RESULT ;

-- [DQL]
SELECT 'abc' ~* 'Abc' AS RESULT ;

-- [DQL]
SELECT 'abc' !~ 'Abc' AS RESULT ;

-- [DQL]
SELECT 'abc' !~* 'Abc' AS RESULT ;

-- [DQL]
SELECT 'abc' ~ '^a' AS RESULT ;

-- [DQL]
SELECT 'abc' ~ '(b|d)' AS RESULT ;

-- [DQL]
SELECT 'abc' ~ '^(b|c)' AS RESULT ;


================================================================================
-- 来源: 1076_file_1076.txt
================================================================================

-- [DQL]
SELECT 2 + 3 AS RESULT ;

-- [DQL]
SELECT 2 - 3 AS RESULT ;

-- [DQL]
SELECT 2 * 3 AS RESULT ;

-- [DQL]
SELECT 4 / 2 AS RESULT ;

-- [DQL]
SELECT 4 / 3 AS RESULT ;

-- [DQL]
SELECT - 2 AS RESULT ;

-- [DQL]
SELECT 5 % 4 AS RESULT ;

-- [DQL]
SELECT @ - 5 . 0 AS RESULT ;

-- [DQL]
SELECT 2 . 0 ^ 3 . 0 AS RESULT ;

-- [DQL]
SELECT |/ 25 . 0 AS RESULT ;

-- [DQL]
SELECT ||/ 27 . 0 AS RESULT ;

-- [DQL]
SELECT 5 ! AS RESULT ;

-- [DQL]
SELECT !! 5 AS RESULT ;

-- [DQL]
SELECT 91 & 15 AS RESULT ;

-- [DQL]
SELECT 32 | 3 AS RESULT ;

-- [DQL]
SELECT 17 # 5 AS RESULT ;

-- [DQL]
SELECT ~ 1 AS RESULT ;

-- [DQL]
SELECT 1 << 4 AS RESULT ;

-- [DQL]
SELECT 8 >> 2 AS RESULT ;

-- [DQL]
SELECT abs ( - 17 . 4 );

-- [DQL]
SELECT acos ( - 1 );

-- [DQL]
SELECT asin ( 0 . 5 );

-- [DQL]
SELECT atan ( 1 );

-- [DQL]
SELECT atan2 ( 2 , 1 );

-- [DQL]
SELECT bitand ( 127 , 63 );

-- [DQL]
SELECT cbrt ( 27 . 0 );

-- [DQL]
SELECT ceil ( - 42 . 8 );

-- [DQL]
SELECT ceiling ( - 95 . 3 );

-- [DQL]
SELECT cos ( - 3 . 1415927 );

-- [DQL]
SELECT cosh ( 4 );

-- [DQL]
SELECT cot ( 1 );

-- [DQL]
SELECT degrees ( 0 . 5 );

-- [DQL]
SELECT div ( 9 , 4 );

-- [DQL]
SELECT exp ( 1 . 0 );

-- [DQL]
SELECT floor ( - 42 . 8 );

-- [DQL]
select int1 ( '123' );

-- [DQL]
select int1 ( '1.1' );

-- [DQL]
select int2 ( '1234' );

-- [DQL]
select int2 ( 25 . 3 );

-- [DQL]
select int4 ( '789' );

-- [DQL]
select int4 ( 99 . 9 );

-- [DQL]
select int8 ( '789' );

-- [DQL]
select int8 ( 99 . 9 );

-- [DQL]
select float4 ( '789' );

-- [DQL]
select float4 ( 99 . 9 );

-- [DQL]
select float8 ( '789' );

-- [DQL]
select float8 ( 99 . 9 );

-- [DQL]
select int16 ( '789' );

-- [DQL]
select int16 ( 99 . 9 );

-- [DQL]
select "numeric" ( '789' );

-- [DQL]
select "numeric" ( 99 . 9 );

-- [DQL]
SELECT radians ( 45 . 0 );

-- [DQL]
SELECT random ();

-- [DQL]
SELECT multiply ( 9 . 0 , '3.0' );

-- [DQL]
SELECT multiply ( '9.0' , 3 . 0 );

-- [DQL]
SELECT ln ( 2 . 0 );

-- [DQL]
SELECT log ( 100 . 0 );

-- [DQL]
SELECT log ( 2 . 0 , 64 . 0 );

-- [DQL]
SELECT mod ( 9 , 4 );

-- [DQL]
SELECT mod ( 9 , 0 );

-- [DQL]
SELECT pi ();

-- [DQL]
SELECT power ( 9 . 0 , 3 . 0 );

-- [DQL]
SELECT remainder ( 11 , 4 );

-- [DQL]
SELECT remainder ( 9 , 0 );

-- [DQL]
SELECT round ( 42 . 4 );

-- [DQL]
SELECT round ( 42 . 6 );

-- [DQL]
SELECT round ( - 0 . 2 :: float8 );

-- [DQL]
SELECT round ( 42 . 4382 , 2 );

-- [DQL]
SELECT setseed ( 0 . 54823 );

-- [DQL]
SELECT sign ( - 8 . 4 );

-- [DQL]
SELECT sin ( 1 . 57079 );

-- [DQL]
SELECT sinh ( 4 );

-- [DQL]
SELECT sqrt ( 2 . 0 );

-- [DQL]
SELECT tan ( 20 );

-- [DQL]
SELECT tanh ( 0 . 1 );

-- [DQL]
SELECT trunc ( 42 . 8 );

-- [DQL]
SELECT trunc ( 42 . 4382 , 2 );

-- [DQL]
SELECT width_bucket ( 5 . 35 , 0 . 024 , 10 . 06 , 5 );

-- [DQL]
SELECT width_bucket ( 5 . 35 , 0 . 024 , 10 . 06 , 5 );

-- [DQL]
SELECT nanvl('NaN', 1.1);

-- [DQL]
SELECT numeric_eq_text(1, '1');

-- [DQL]
SELECT text_eq_numeric('1', 1);

-- [DQL]
SELECT bigint_eq_text(1, '1');

-- [DQL]
SELECT text_eq_bigint('1', 1);


================================================================================
-- 来源: 1077_file_1077.txt
================================================================================

-- [DQL]
SELECT date '2001-10-01' - '7' AS RESULT ;

-- [DQL]
SELECT date '2001-9-28' + integer '7' AS RESULT ;

-- [DQL]
SELECT date '2001-09-28' + interval '1 hour' AS RESULT ;

-- [DQL]
SELECT date '2001-09-28' + time '03:00' AS RESULT ;

-- [DQL]
SELECT interval '1 day' + interval '1 hour' AS RESULT ;

-- [DQL]
SELECT timestamp '2001-09-28 01:00' + interval '23 hours' AS RESULT ;

-- [DQL]
SELECT time '01:00' + interval '3 hours' AS RESULT ;

-- [DQL]
SELECT date '2001-10-01' - date '2001-09-28' AS RESULT ;

-- [DQL]
SELECT date '2001-10-01' - integer '7' AS RESULT ;

-- [DQL]
SELECT date '2001-09-28' - interval '1 hour' AS RESULT ;

-- [DQL]
SELECT time '05:00' - time '03:00' AS RESULT ;

-- [DQL]
SELECT time '05:00' - interval '2 hours' AS RESULT ;

-- [DQL]
SELECT timestamp '2001-09-28 23:00' - interval '23 hours' AS RESULT ;

-- [DQL]
SELECT interval '1 day' - interval '1 hour' AS RESULT ;

-- [DQL]
SELECT timestamp '2001-09-29 03:00' - timestamp '2001-09-27 12:00' AS RESULT ;

-- [DQL]
SELECT 900 * interval '1 second' AS RESULT ;

-- [DQL]
SELECT 21 * interval '1 day' AS RESULT ;

-- [DQL]
SELECT double precision '3.5' * interval '1 hour' AS RESULT ;

-- [DQL]
SELECT interval '1 hour' / double precision '1.5' AS RESULT ;

-- [DQL]
SELECT age ( timestamp '2001-04-10' , timestamp '1957-06-13' );

-- [DQL]
SELECT age ( timestamp '1957-06-13' );

-- [DQL]
SELECT clock_timestamp ();

-- [DQL]
SELECT current_date ;

-- [DQL]
SELECT current_time ;

-- [DQL]
SELECT current_timestamp ;

-- [DQL]
SELECT current_timestamp ;

-- [DQL]
SELECT current_timestamp ();

-- [DQL]
SELECT current_timestamp ( 1 );

-- [DQL]
SELECT current_timestamp ( 1 );

-- [DQL]
SELECT pg_systimestamp ();

-- [DQL]
SELECT date_part ( 'hour' , timestamp '2001-02-16 20:38:40' );

-- [DQL]
SELECT date_part ( 'month' , interval '2 years 3 months' );

-- [DQL]
SELECT date_trunc ( 'hour' , timestamp '2001-02-16 20:38:40' );

-- [DQL]
SELECT trunc ( timestamp '2001-02-16 20:38:40' );

-- [DQL]
SELECT trunc ( timestamp '2001-02-16 20:38:40' , 'hour' );

-- [DQL]
SELECT round ( timestamp '2001-02-16 20:38:40' , 'hour' );

-- [DQL]
SELECT daterange ( '2000-05-06' , '2000-08-08' );

-- [DQL]
SELECT daterange ( '2000-05-06' , '2000-08-08' , '[]' );

-- [DQL]
SELECT isfinite ( date '2001-02-16' );

-- [DQL]
SELECT isfinite ( date 'infinity' );

-- [DQL]
SELECT isfinite ( timestamp '2001-02-16 21:28:30' );

-- [DQL]
SELECT isfinite ( timestamp 'infinity' );

-- [DQL]
SELECT isfinite ( interval '4 hours' );

-- [DQL]
SELECT justify_days ( interval '35 days' );

-- [DQL]
SELECT JUSTIFY_HOURS ( INTERVAL '27 HOURS' );

-- [DQL]
SELECT JUSTIFY_INTERVAL ( INTERVAL '1 MON -1 HOUR' );

-- [DQL]
SELECT localtime AS RESULT ;

-- [DQL]
SELECT localtimestamp ;

-- [DQL]
SELECT maketime ( 8 , 15 , 26 . 53 );

-- [DQL]
SELECT maketime ( - 888 , 15 , 26 . 53 );

-- [DQL]
SELECT now ();

-- [DQL]
SELECT timenow ();

-- [DQL]
SELECT dbtimezone ;

-- [DQL]
SELECT numtodsinterval ( 100 , 'HOUR' );

-- [SESSION]
SET intervalstyle = oracle ;

-- [DQL]
SELECT numtodsinterval ( 100 , 'HOUR' );

-- [DQL]
SELECT numtoyminterval ( 100 , 'MONTH' );

-- [SESSION]
SET intervalstyle = oracle ;

-- [DQL]
SELECT numtodsinterval ( 100 , 'MONTH' );

-- [DQL]
SELECT new_time ( '1997-10-10' , 'AST' , 'EST' );

-- [DQL]
SELECT NEW_TIME ( TO_TIMESTAMP ( '10-Sep-02 14:10:10.123000' , 'DD-Mon-RR HH24:MI:SS.FF' ), 'AST' , 'PST' );

-- [DQL]
SELECT SESSIONTIMEZONE ;

-- [DQL]
SELECT LOWER ( SESSIONTIMEZONE );

-- [DQL]
SELECT SYS_EXTRACT_UTC ( TIMESTAMP '2000-03-28 11:30:00.00' );

-- [DQL]
SELECT SYS_EXTRACT_UTC ( TIMESTAMPTZ '2000-03-28 11:30:00.00 -08:00' );

-- [DQL]
SELECT TZ_OFFSET ( 'US/Pacific' );

-- [DQL]
SELECT TZ_OFFSET ( sessiontimezone );

-- [DQL]
SELECT pg_sleep ( 10 );

-- [DQL]
SELECT statement_timestamp ();

-- [DQL]
SELECT sysdate ;

-- [DQL]
SELECT current_sysdate ();

-- [DQL]
SELECT timeofday ();

-- [DQL]
SELECT transaction_timestamp ();

-- [DQL]
SELECT transaction_timestamp ();

-- [DQL]
SELECT add_months ( to_date ( '2017-5-29' , 'yyyy-mm-dd' ), 11 ) FROM sys_dummy ;

-- [DQL]
SELECT last_day ( to_date ( '2017-01-01' , 'YYYY-MM-DD' )) AS cal_result ;

-- [DQL]
SELECT months_between(to_date('2022-10-31', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

-- [DQL]
SELECT months_between(to_date('2022-10-30', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

-- [DQL]
SELECT months_between(to_date('2022-10-29', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

-- [DQL]
SELECT next_day ( timestamp '2017-05-25 00:00:00' , 'Sunday' ) AS cal_result ;

-- [DQL]
SELECT next_day ( timestamp '2024-01-17 00:00:00' , 7 . 9999999 );

-- [SESSION]
SET a_format_version = '10c' ;

-- [SESSION]
SET a_format_dev_version = 's1' ;

-- [DQL]
SELECT next_day ( timestamp '2024-01-17 00:00:00' , 7 . 9999999 );

-- [PLSQL]
CALL tinterval ( abstime 'May 10, 1947 23:59:12' , abstime 'Mon May 1 00:30:30 1995' );

-- [DQL]
SELECT tintervalend ( '["Sep 4, 1983 23:59:12" "Oct4, 1983 23:59:12"]' );

-- [DQL]
SELECT tintervalrel ( '["Sep 4, 1983 23:59:12" "Oct4, 1983 23:59:12"]' );

-- [SESSION]
SET b_format_dev_version = 's1' ;

-- [SESSION]
SET b_format_version = '5.7' ;

-- [DQL]
SELECT convert_tz ( cast ( '2023-01-01 10:10:10' as datetime ), '+00:00' , '+01:00' );

-- [DQL]
SELECT convert_tz ( cast ( '2023-01-01' as date ), '+00:00' , '+01:00' );

-- [DQL]
SELECT convert_tz ( '2023-01-01 10:10:10' , '+00:00' , '+01:00' );

-- [DQL]
SELECT convert_tz ( '2023-01-01' , '+00:00' , '+01:00' );

-- [DQL]
SELECT convert_tz ( 20230101101010 , '+00:00' , '+01:00' );

-- [DQL]
SELECT convert_tz ( 20230101 , '+00:00' , '+01:00' );

-- [DQL]
SELECT convert_tz ( '2023-01-01 10:10:10' , 'UTC' , 'PRC' );

-- [SESSION]
SET b_format_dev_version = 's1' ;

-- [SESSION]
SET b_format_version = '5.7' ;

-- [DQL]
SELECT sec_to_time ( 2000 );

-- [DQL]
SELECT sec_to_time ( '-2000' );

-- [DQL]
SELECT ADDDATE ( '2018-05-01' , INTERVAL 1 DAY );

-- [DQL]
SELECT ADDDATE('2018-05-01', 1);

-- [DQL]
SELECT curdate ();

-- [DQL]
SELECT curtime ( 3 );

-- [DQL]
SELECT DATE_ADD('2018-05-01', INTERVAL 1 DAY);

-- [DQL]
SELECT DATE_ADD('2018-05-01', 1);

-- [DQL]
SELECT date_format('2023-10-11 12:13:14.151617','%b %c %M %m');

-- [DQL]
SELECT DATE_SUB('2018-05-01', INTERVAL 1 YEAR);

-- [DQL]
SELECT DATE_SUB('2023-1-1', 20);

-- [DQL]
SELECT datediff('2021-11-12','2021-11-13');

-- [DQL]
SELECT day('2023-01-02');

-- [DQL]
SELECT dayofmonth('23-05-22');

-- [DQL]
SELECT dayname('2023-10-11');

-- [DQL]
SELECT dayofweek('2023-04-16');

-- [DQL]
SELECT dayofyear('2000-12-31');

-- [DQL]
SELECT extract(YEAR FROM '2023-10-11');

-- [DQL]
SELECT extract(QUARTER FROM '2023-10-11');

-- [DQL]
SELECT extract(MONTH FROM '2023-10-11');

-- [DQL]
SELECT extract(WEEK FROM '2023-10-11');

-- [DQL]
SELECT extract(DAY FROM '2023-10-11');

-- [DQL]
SELECT extract(HOUR FROM '2023-10-11 12:13:14');

-- [DQL]
SELECT from_days(36524);

-- [DQL]
SELECT from_unixtime(1111885200);

-- [DQL]
SELECT get_format(date, 'eur');

-- [DQL]
SELECT get_format(date, 'usa');

-- [DQL]
SELECT HOUR('10:10:10.1');

-- [DQL]
SELECT makedate(2000, 60);

-- [DQL]
SELECT MICROSECOND('2023-5-5 10:10:10.24485');

-- [DQL]
SELECT MINUTE(time'10:10:10');

-- [DQL]
SELECT month('2021-11-30');

-- [DQL]
SELECT monthname('2023-02-28');

-- [DQL]
SELECT period_add(202205, -12);

-- [DQL]
SELECT period_diff('202101', '202102');

-- [DQL]
SELECT SECOND('2023-5-5 10:10:10');

-- [DQL]
SELECT QUARTER('2012-1-1');

-- [DQL]
SELECT str_to_date('May 1, 2013','%M %d,%Y');

-- [DQL]
SELECT SUBDATE('2023-1-1', 20);

-- [DQL]
SELECT SUBDATE('2018-05-01', INTERVAL 1 YEAR);

-- [DQL]
SELECT subtime('2000-03-01 20:59:59', '22:58');

-- [DQL]
SELECT addtime('2000-03-01 20:59:59', '00:00:01');

-- [DQL]
SELECT TIME_FORMAT('25:30:30', '%T|%r|%H|%h|%I|%i|%S|%f|%p|%k');

-- [DQL]
SELECT time_to_sec('00:00:01');

-- [DQL]
SELECT timediff(date'2022-12-30',20221229);

-- [DQL]
SELECT TIMESTAMPADD(DAY,-2,'2022-07-27');

-- [DQL]
SELECT to_days('2000-1-1');

-- [DQL]
SELECT TO_SECONDS('2009-11-29 13:43:32');

-- [DQL]
SELECT UNIX_TIMESTAMP('2022-12-22');

-- [DQL]
SELECT utc_date();

-- [DQL]
SELECT utc_time();

-- [DQL]
SELECT utc_timestamp();

-- [DQL]
SELECT week(date'2000-01-01', 1);

-- [DQL]
SELECT week('2000-01-01', 2);

-- [DQL]
SELECT weekday('1970-01-01 12:00:00');

-- [DQL]
SELECT weekofyear('1970-05-22');

-- [DQL]
SELECT year('23-05-22');

-- [DQL]
SELECT yearweek(datetime'2000-01-01', 3);

-- [DQL]
SELECT timestamp_diff ( 'year' , '2018-01-01' , '2020-04-01' );

-- [DQL]
SELECT timestamp_diff ( 'month' , '2018-01-01' , '2020-04-01' );

-- [DQL]
SELECT timestamp_diff ( 'quarter' , '2018-01-01' , '2020-04-01' );

-- [DQL]
SELECT timestamp_diff ( 'week' , '2018-01-01' , '2020-04-01' );

-- [DQL]
SELECT timestamp_diff ( 'day' , '2018-01-01' , '2020-04-01' );

-- [DQL]
SELECT timestamp_diff ( 'hour' , '2018-01-01 10:10:10' , '2018-01-01 12:12:12' );

-- [DQL]
SELECT timestamp_diff ( 'minute' , '2018-01-01 10:10:10' , '2018-01-01 12:12:12' );

-- [DQL]
SELECT timestamp_diff ( 'second' , '2018-01-01 10:10:10' , '2018-01-01 10:12:12' );

-- [DQL]
SELECT timestamp_diff ( 'microsecond' , '2018-01-01 10:10:10' , '2018-01-01 10:12:12' );

-- [DQL]
SELECT TIMESTAMPDIFF ( YEAR , '2018-01-01' , '2020-01-01' );

-- [DQL]
SELECT TIMESTAMPDIFF ( QUARTER , '2018-01-01' , '2020-01-01' );

-- [DQL]
SELECT TIMESTAMPDIFF ( MONTH , '2018-01-01' , '2020-01-01' );

-- [DQL]
SELECT TIMESTAMPDIFF ( WEEK , '2018-01-01' , '2020-01-01' );

-- [DQL]
SELECT TIMESTAMPDIFF ( DAY , '2018-01-01' , '2020-01-01' );

-- [DQL]
SELECT TIMESTAMPDIFF ( HOUR , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

-- [DQL]
SELECT TIMESTAMPDIFF ( MINUTE , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

-- [DQL]
SELECT TIMESTAMPDIFF ( SECOND , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

-- [DQL]
SELECT TIMESTAMPDIFF ( MICROSECOND , '2020-01-01 10:10:10.000000' , '2020-01-01 10:10:10.111111' );

-- [DQL]
SELECT EXTRACT ( CENTURY FROM TIMESTAMP '2000-12-16 12:21:13' );

-- [DQL]
SELECT EXTRACT ( DAY FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( DAY FROM INTERVAL '40 days 1 minute' );

-- [DQL]
SELECT EXTRACT ( DECADE FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( DOW FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( DOY FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( EPOCH FROM TIMESTAMP WITH TIME ZONE '2001-02-16 20:38:40.12-08' );

-- [DQL]
SELECT EXTRACT ( EPOCH FROM INTERVAL '5 days 3 hours' );

-- [DQL]
SELECT TIMESTAMP WITH TIME ZONE 'epoch' + 982384720 . 12 * INTERVAL '1 second' AS RESULT ;

-- [DQL]
SELECT EXTRACT ( HOUR FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( ISODOW FROM TIMESTAMP '2001-02-18 20:38:40' );

-- [DQL]
SELECT EXTRACT ( ISOYEAR FROM DATE '2006-01-01' );

-- [DQL]
SELECT EXTRACT ( WEEK FROM TIMESTAMP '2006-01-01 00:00:40' );

-- [DQL]
SELECT EXTRACT(ISOYEAR FROM DATE '2006-01-02');

-- [DQL]
SELECT EXTRACT(WEEK FROM TIMESTAMP '2006-01-02 00:00:40');

-- [DQL]
SELECT EXTRACT ( MICROSECONDS FROM TIME '17:12:28.5' );

-- [DQL]
SELECT EXTRACT ( MILLENNIUM FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( MILLISECONDS FROM TIME '17:12:28.5' );

-- [DQL]
SELECT EXTRACT ( MINUTE FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( MONTH FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( MONTH FROM INTERVAL '2 years 13 months' );

-- [DQL]
SELECT EXTRACT ( QUARTER FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( SECOND FROM TIME '17:12:28.5' );

-- [DQL]
SELECT EXTRACT ( ISOYEAR FROM DATE '2006-01-01' );

-- [DQL]
SELECT EXTRACT ( WEEK FROM TIMESTAMP '2006-01-01 00:00:40' );

-- [DQL]
SELECT EXTRACT(ISOYEAR FROM DATE '2006-01-02');

-- [DQL]
SELECT EXTRACT(WEEK FROM TIMESTAMP '2006-01-02 00:00:40');

-- [DQL]
SELECT EXTRACT ( YEAR FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT date_part ( 'day' , TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT date_part ( 'hour' , INTERVAL '4 hours 3 minutes' );


================================================================================
-- 来源: 1078_file_1078.txt
================================================================================

-- [DQL]
SELECT cash_words ( '1.23' );

-- [DQL]
SELECT cast ( '22-oct-1997' as timestamp );

-- [DQL]
SELECT cast ( '22-ocX-1997' as timestamp DEFAULT '22-oct-1997' ON CONVERSION ERROR , 'DD-Mon-YYYY' );

-- [DDL]
CREATE DATABASE gaussdb_m WITH dbcompatibility 'MYSQL' ;

-- [OTHER]
\ c gaussdb_m -- 设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

-- [DQL]
SELECT CAST ( 12 AS UNSIGNED );

-- [DQL]
SELECT hextoraw ( '7D' );

-- [DQL]
SELECT numtoday ( 2 );

-- [DQL]
SELECT rawtohex ( '1234567' );

-- [DQL]
SELECT to_blob ( '0AADD343CDBBD' :: RAW ( 10 ));

-- [DQL]
SELECT to_bigint ( '123364545554455' );

-- [DQL]
SELECT to_binary_double ( '12345678' );

-- [DQL]
SELECT to_binary_double ( '1,2,3' , '9,9,9' );

-- [DQL]
SELECT to_binary_double ( 1 e2 default 12 on conversion error );

-- [DQL]
SELECT to_binary_double ( 'aa' default 12 on conversion error );

-- [DQL]
SELECT to_binary_double ( '12-' default 10 on conversion error , '99S' );

-- [DQL]
SELECT to_binary_double ( 'aa-' default 12 on conversion error , '99S' );

-- [DQL]
SELECT to_binary_float ( '12345678' );

-- [DQL]
SELECT to_binary_float ( '1,2,3' , '9,9,9' );

-- [DQL]
SELECT to_binary_float ( 1 e2 default 12 on conversion error );

-- [DQL]
SELECT to_binary_float ( 'aa' default 12 on conversion error );

-- [DQL]
SELECT to_binary_float ( '12-' default 10 on conversion error , '99S' );

-- [DQL]
SELECT to_binary_float ( 'aa-' default 12 on conversion error , '99S' );

-- [DQL]
SELECT to_char ( current_timestamp , 'HH12:MI:SS' );

-- [DQL]
SELECT to_char ( current_timestamp , 'FMHH12:FMMI:FMSS' );

-- [DQL]
SELECT to_char ( 125 . 8 :: real , '999D99' );

-- [DQL]
SELECT to_char ( 1485 , '9,999' );

-- [DQL]
SELECT to_char ( 1148 . 5 , '9,999.999' );

-- [DQL]
SELECT to_char ( 148 . 5 , '990999.909' );

-- [DQL]
SELECT to_char ( 123 , 'XXX' );

-- [DQL]
SELECT to_char ( interval '15h 2m 12s' , 'HH24:MI:SS' );

-- [DQL]
SELECT to_char ( 125 , '999' );

-- [DQL]
SELECT to_char ( - 125 . 8 , '999D99S' );

-- [DQL]
SELECT to_char ( '01110' );

-- [DQL]
SELECT to_char ( current_timestamp , 'HH12:MI:SS' );

-- [DQL]
SELECT to_nchar ( current_timestamp , 'HH12:MI:SS' );

-- [DQL]
SELECT to_nchar ( current_timestamp , 'FMHH12:FMMI:FMSS' );

-- [DQL]
SELECT to_nchar ( 125 . 8 :: real , '999D99' );

-- [DQL]
SELECT to_nchar ( 1485 , '9,999' );

-- [DQL]
SELECT to_nchar ( 1148 . 5 , '9,999.999' );

-- [DQL]
SELECT to_nchar ( 148 . 5 , '990999.909' );

-- [DQL]
SELECT to_nchar ( 123 , 'XXX' );

-- [DQL]
SELECT to_nchar ( interval '15h 2m 12s' , 'HH24:MI:SS' );

-- [DQL]
SELECT to_nchar ( 125 , '999' );

-- [DQL]
SELECT to_nchar ( - 125 . 8 , '999D99S' );

-- [DQL]
SELECT to_nchar ( '01110' );

-- [DQL]
SELECT to_nchar ( current_timestamp , 'HH12:MI:SS' );

-- [DQL]
SELECT to_clob ( 'ABCDEF' :: RAW ( 10 ));

-- [DQL]
SELECT to_clob ( 'hello111' :: CHAR ( 15 ));

-- [DQL]
SELECT to_clob ( 'gauss123' :: NCHAR ( 10 ));

-- [DQL]
SELECT to_clob ( 'gauss234' :: VARCHAR ( 10 ));

-- [DQL]
SELECT to_clob ( 'gauss345' :: VARCHAR2 ( 10 ));

-- [DQL]
SELECT to_clob ( 'gauss456' :: NVARCHAR2 ( 10 ));

-- [DQL]
SELECT to_clob ( 'World222!' :: TEXT );

-- [DQL]
SELECT to_date ( '2015-08-14' );

-- [DQL]
SELECT to_date ( '05 Dec 2000' , 'DD Mon YYYY' );

-- [DQL]
SELECT to_date ( '2015-08-14' );

-- [DQL]
SELECT to_date ( '05 Dec 2000' , 'DD Mon YYYY' );

-- [SESSION]
set a_format_version='10c';

-- [SESSION]
set a_format_dev_version='s1';

-- [SESSION]
show nls_timestamp_format;

-- [DQL]
select to_date('12-jan-2022' default '12-apr-2022' on conversion error);

-- [DQL]
select to_date('12-ja-2022' default '12-apr-2022' on conversion error);

-- [DQL]
select to_date('2022-12-12' default '2022-01-01' on conversion error, 'yyyy-mm-dd');

-- [DQL]
SELECT to_number ( '12,454.8-' , '99G999D9S' );

-- [DQL]
SELECT to_number ( '12,454.8-' , '99G999D9S' );

-- [DQL]
select to_number ( '1e2' );

-- [DQL]
select to_number ( '123.456' );

-- [DQL]
select to_number ( '123' , '999' );

-- [DQL]
select to_number ( '123-' , '999MI' );

-- [DQL]
select to_number ( '123' default '456-' on conversion error , '999MI' );

-- [DQL]
SELECT to_timestamp ( 1284352323 );

-- [SESSION]
SHOW nls_timestamp_format ;

-- [DQL]
SELECT to_timestamp ( '12-sep-2014' );

-- [DQL]
SELECT to_timestamp ( '12-Sep-10 14:10:10.123000' , 'DD-Mon-YY HH24:MI:SS.FF' );

-- [DQL]
SELECT to_timestamp ( '-1' , 'SYYYY' );

-- [DQL]
SELECT to_timestamp ( '98' , 'RR' );

-- [DQL]
SELECT to_timestamp ( '01' , 'RR' );

-- [SESSION]
set a_format_version='10c';

-- [SESSION]
set a_format_dev_version='s1';

-- [DQL]
SELECT to_timestamp('11-Sep-11' DEFAULT '12-Sep-10 14:10:10.123000' ON CONVERSION ERROR,'DD-Mon-YY HH24:MI:SS.FF');

-- [DQL]
SELECT to_timestamp('12-Sep-10 14:10:10.123000','DD-Mon-YY HH24:MI:SSXFF');

-- [DQL]
SELECT to_timestamp ( '05 Dec 2000' , 'DD Mon YYYY' );

-- [DQL]
SELECT to_timestamp_tz ( '05 DeX 2000' DEFAULT '05 Dec 2001' ON CONVERSION ERROR , 'DD Mon YYYY' );

-- [DQL]
SELECT to_timestamp_tz ( '05 DeX 2000' DEFAULT '05 Dec 2001' ON CONVERSION ERROR , 'DD Mon YYYY' , 'nls_date_language=AMERICAN' );

-- [DQL]
select to_dsinterval ( '12 1:2:3.456' );

-- [DQL]
select to_dsinterval ( 'P3DT4H5M6S' );

-- [DQL]
select to_yminterval ( '1-1' );

-- [DQL]
select to_yminterval ( 'P13Y3M4DT4H2M5S' );

-- [DDL]
create table json_doc ( data CLOB );

-- [DML_INSERT]
insert into json_doc values ( '{"name":"a"}' );

-- [DQL]
select treat ( data as json ) from json_doc ;

-- [DDL]
create or replace procedure p1 is gaussdb $ # type t1 is table of int ;

-- [PLSQL]
call p1 ();

-- [DDL]
create type t1 is table of int ;

-- [DQL]
select cast ( t1 ( 1 , 2 , 3 ) as int []) result ;

-- [DQL]
SELECT convert_to_nocase ( '12345' , 'GBK' );


================================================================================
-- 来源: 1079_file_1079.txt
================================================================================

-- [DQL]
SELECT box '((0,0),(1,1))' + point '(2.0,0)' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(1,1))' - point '(2.0,0)' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(1,1))' * point '(2.0,0)' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(2,2))' / point '(2.0,0)' AS RESULT ;

-- [DQL]
SELECT box '((1,-1),(-1,1))' # box '((1,1),(-2,-2))' AS RESULT ;

-- [DQL]
SELECT # path '((1,0),(0,1),(-1,0))' AS RESULT ;

-- [DQL]
SELECT @-@ path '((0,0),(1,0))' AS RESULT ;

-- [DQL]
SELECT @@ circle '((0,0),10)' AS RESULT ;

-- [DQL]
SELECT circle '((0,0),1)' <-> circle '((5,0),1)' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(1,1))' && box '((0,0),(2,2))' AS RESULT ;

-- [DQL]
SELECT circle '((0,0),1)' << circle '((5,0),1)' AS RESULT ;

-- [DQL]
SELECT circle '((5,0),1)' >> circle '((0,0),1)' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(1,1))' &< box '((0,0),(2,2))' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(3,3))' &> box '((0,0),(2,2))' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(3,3))' <<| box '((3,4),(5,5))' AS RESULT ;

-- [DQL]
SELECT box '((3,4),(5,5))' |>> box '((0,0),(3,3))' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(1,1))' &<| box '((0,0),(2,2))' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(3,3))' |&> box '((0,0),(2,2))' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(-3,-3))' <^ box '((0,0),(2,2))' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(2,2))' >^ box '((0,0),(-3,-3))' AS RESULT ;

-- [DQL]
SELECT lseg '((-1,0),(1,0))' ?# box '((-2,-2),(2,2))' AS RESULT ;

-- [DQL]
SELECT ?- lseg '((-1,0),(1,0))' AS RESULT ;

-- [DQL]
SELECT point '(1,0)' ?- point '(0,0)' AS RESULT ;

-- [DQL]
SELECT ?| lseg '((-1,0),(1,0))' AS RESULT ;

-- [DQL]
SELECT point '(0,1)' ?| point '(0,0)' AS RESULT ;

-- [DQL]
SELECT lseg '((0,0),(0,1))' ?-| lseg '((0,0),(1,0))' AS RESULT ;

-- [DQL]
SELECT lseg '((-1,0),(1,0))' ?|| lseg '((-1,2),(1,2))' AS RESULT ;

-- [DQL]
SELECT circle '((0,0),2)' @> point '(1,1)' AS RESULT ;

-- [DQL]
SELECT point '(1,1)' <@ circle '((0,0),2)' AS RESULT ;

-- [DQL]
SELECT polygon '((0,0),(1,1))' ~= polygon '((1,1),(0,0))' AS RESULT ;

-- [DQL]
SELECT area ( box '((0,0),(1,1))' ) AS RESULT ;

-- [DQL]
SELECT center ( box '((0,0),(1,2))' ) AS RESULT ;

-- [DQL]
SELECT diameter ( circle '((0,0),2.0)' ) AS RESULT ;

-- [DQL]
SELECT height ( box '((0,0),(1,1))' ) AS RESULT ;

-- [DQL]
SELECT isclosed ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- [DQL]
SELECT isopen ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

-- [DQL]
SELECT length ( path '((-1,0),(1,0))' ) AS RESULT ;

-- [DQL]
SELECT npoints ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

-- [DQL]
SELECT npoints ( polygon '((1,1),(0,0))' ) AS RESULT ;

-- [DQL]
SELECT pclose ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

-- [DQL]
SELECT popen ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- [DQL]
SELECT radius ( circle '((0,0),2.0)' ) AS RESULT ;

-- [DQL]
SELECT width ( box '((0,0),(1,1))' ) AS RESULT ;

-- [DQL]
SELECT box ( circle '((0,0),2.0)' ) AS RESULT ;

-- [DQL]
SELECT box ( point '(0,0)' , point '(1,1)' ) AS RESULT ;

-- [DQL]
SELECT box ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- [DQL]
SELECT circle ( box '((0,0),(1,1))' ) AS RESULT ;

-- [DQL]
SELECT circle ( point '(0,0)' , 2 . 0 ) AS RESULT ;

-- [DQL]
SELECT circle ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- [DQL]
SELECT lseg ( box '((-1,0),(1,0))' ) AS RESULT ;

-- [DQL]
SELECT lseg ( point '(-1,0)' , point '(1,0)' ) AS RESULT ;

-- [DQL]
SELECT slope(point '(1,1)', point '(0,0)') AS RESULT;

-- [DQL]
SELECT path ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- [DQL]
SELECT point ( 23 . 4 , - 44 . 5 ) AS RESULT ;

-- [DQL]
SELECT point ( box '((-1,0),(1,0))' ) AS RESULT ;

-- [DQL]
SELECT point ( circle '((0,0),2.0)' ) AS RESULT ;

-- [DQL]
SELECT point ( lseg '((-1,0),(1,0))' ) AS RESULT ;

-- [DQL]
SELECT point ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- [DQL]
SELECT polygon ( box '((0,0),(1,1))' ) AS RESULT ;

-- [DQL]
SELECT polygon ( circle '((0,0),2.0)' ) AS RESULT ;

-- [DQL]
SELECT polygon ( 12 , circle '((0,0),2.0)' ) AS RESULT ;

-- [DQL]
SELECT polygon ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;


================================================================================
-- 来源: 1080_file_1080.txt
================================================================================

-- [DQL]
SELECT inet '192.168.1.5' < inet '192.168.1.6' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.5' <= inet '192.168.1.5' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.5' = inet '192.168.1.5' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.5' >= inet '192.168.1.5' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.5' > inet '192.168.1.4' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.5' <> inet '192.168.1.4' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.5' << inet '192.168.1/24' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1/24' <<= inet '192.168.1/24' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1/24' >> inet '192.168.1.5' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1/24' >>= inet '192.168.1/24' AS RESULT ;

-- [DQL]
SELECT ~ inet '192.168.1.6' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.6' & inet '10.0.0.0' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.6' | inet '10.0.0.0' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.6' + 25 AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.43' - 36 AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.43' - inet '192.168.1.19' AS RESULT ;

-- [DQL]
SELECT abbrev ( inet '10.1.0.0/16' ) AS RESULT ;

-- [DQL]
SELECT abbrev ( cidr '10.1.0.0/16' ) AS RESULT ;

-- [DQL]
SELECT broadcast ( '192.168.1.5/24' ) AS RESULT ;

-- [DQL]
SELECT family ( '127.0.0.1' ) AS RESULT ;

-- [DQL]
SELECT host ( '192.168.1.5/24' ) AS RESULT ;

-- [DQL]
SELECT hostmask ( '192.168.23.20/30' ) AS RESULT ;

-- [DQL]
SELECT masklen ( '192.168.1.5/24' ) AS RESULT ;

-- [DQL]
SELECT netmask ( '192.168.1.5/24' ) AS RESULT ;

-- [DQL]
SELECT network ( '192.168.1.5/24' ) AS RESULT ;

-- [DQL]
SELECT set_masklen ( '192.168.1.5/24' , 16 ) AS RESULT ;

-- [DQL]
SELECT set_masklen ( '192.168.1.0/24' :: cidr , 16 ) AS RESULT ;

-- [DQL]
SELECT text ( inet '192.168.1.5' ) AS RESULT ;

-- [DQL]
SELECT trunc ( macaddr '12:34:56:78:90:ab' ) AS RESULT ;


================================================================================
-- 来源: 1081_file_1081.txt
================================================================================

-- [DQL]
SELECT to_tsvector ( 'fat cats ate rats' ) @@ to_tsquery ( 'cat & rat' ) AS RESULT ;

-- [DQL]
SELECT to_tsvector ( 'fat cats ate rats' ) @@@ to_tsquery ( 'cat & rat' ) AS RESULT ;

-- [DQL]
SELECT 'a:1 b:2' :: tsvector || 'c:1 d:2 b:3' :: tsvector AS RESULT ;

-- [DQL]
SELECT 'fat | rat' :: tsquery && 'cat' :: tsquery AS RESULT ;

-- [DQL]
SELECT 'fat | rat' :: tsquery || 'cat' :: tsquery AS RESULT ;

-- [DQL]
SELECT !! 'cat' :: tsquery AS RESULT ;

-- [DQL]
SELECT 'cat' :: tsquery @> 'cat & rat' :: tsquery AS RESULT ;

-- [DQL]
SELECT 'cat' :: tsquery <@ 'cat & rat' :: tsquery AS RESULT ;

-- [DQL]
SELECT get_current_ts_config ();

-- [DQL]
SELECT length ( 'fat:2,4 cat:3 rat:5A' :: tsvector );

-- [DQL]
SELECT numnode ( '(fat & rat) | cat' :: tsquery );

-- [DQL]
SELECT plainto_tsquery ( 'english' , 'The Fat Rats' );

-- [DQL]
SELECT querytree ( 'foo & ! bar' :: tsquery );

-- [DQL]
SELECT setweight ( 'fat:2,4 cat:3 rat:5B' :: tsvector , 'A' );

-- [DQL]
SELECT strip ( 'fat:2,4 cat:3 rat:5A' :: tsvector );

-- [DQL]
SELECT to_tsquery ( 'english' , 'The & Fat & Rats' );

-- [DQL]
SELECT to_tsvector ( 'english' , 'The Fat Rats' );

-- [DQL]
SELECT to_tsvector_for_batch ( 'english' , 'The Fat Rats' );

-- [DQL]
SELECT ts_headline ( 'x y z' , 'z' :: tsquery );

-- [DQL]
SELECT ts_rank ( 'hello world' :: tsvector , 'world' :: tsquery );

-- [DQL]
SELECT ts_rank_cd ( 'hello world' :: tsvector , 'world' :: tsquery );

-- [DQL]
SELECT ts_rewrite ( 'a & b' :: tsquery , 'a' :: tsquery , 'foo|bar' :: tsquery );

-- [DQL]
SELECT ts_rewrite ( 'world' :: tsquery , 'select ''world''::tsquery, ''hello''::tsquery' );

-- [DQL]
SELECT ts_debug ( 'english' , 'The Brightest supernovaes' );

-- [DQL]
SELECT ts_lexize ( 'english_stem' , 'stars' );

-- [DQL]
SELECT ts_parse ( 'default' , 'foo - bar' );

-- [DQL]
SELECT ts_parse ( 3722 , 'foo - bar' );

-- [DQL]
SELECT ts_token_type ( 'default' );

-- [DQL]
SELECT ts_token_type ( 3722 );

-- [DQL]
SELECT ts_stat ( 'select ''hello world''::tsvector' );


================================================================================
-- 来源: 1082_JSON_JSONB.txt
================================================================================

-- [DQL]
SELECT array_to_json('{{1,5},{99,100}}'::int[]);

-- [DQL]
SELECT row_to_json(row(1,'foo'));

-- [DQL]
SELECT json_array_element('[1,true,[1,[2,3]],null]',2);

-- [DQL]
SELECT json_array_element_text('[1,true,[1,[2,3]],null]',2);

-- [DQL]
SELECT json_object_field('{"a": {"b":"foo"}}','a');

-- [DQL]
SELECT json_object_field_text('{"a": {"b":"foo"}}','a');

-- [DQL]
SELECT json_extract_path('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', 'f4','f6');

-- [DQL]
SELECT json_extract_path_op('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', ARRAY['f4','f6']);

-- [DQL]
SELECT json_extract_path_text('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', 'f4','f6');

-- [DQL]
SELECT json_extract_path_text_op('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', ARRAY['f4','f6']);

-- [DQL]
SELECT json_array_elements('[1,true,[1,[2,3]],null]');

-- [DQL]
SELECT * FROM json_array_elements_text('[1,true,[1,[2,3]],null]');

-- [DQL]
SELECT json_array_length('[1,2,3,{"f1":1,"f2":[5,6]},4,null]');

-- [DQL]
SELECT * FROM json_each('{"f1":[1,2,3],"f2":{"f3":1},"f4":null}');

-- [DQL]
SELECT * FROM json_each_text('{"f1":[1,2,3],"f2":{"f3":1},"f4":null}');

-- [DQL]
SELECT json_object_keys('{"f1":"abc","f2":{"f3":"a", "f4":"b"}, "f1":"abcd"}');

-- [DQL]
SELECT jsonb_object_keys('{"f1":"abc","f2":{"f3":"a", "f4":"b"}, "f1":"abcd"}');

-- [DDL]
CREATE TYPE jpop AS (a text, b int, c bool);

-- [DQL]
SELECT * FROM json_populate_record(null::jpop,'{"a":"blurfl","x":43.2}');

-- [DQL]
SELECT * FROM json_populate_record((1,1,null)::jpop,'{"a":"blurfl","x":43.2}');

-- [DDL]
DROP TYPE jpop;

-- [DDL]
CREATE TYPE jpop AS (a text, b int, c bool);

-- [DQL]
SELECT * FROM json_populate_recordset(null::jpop, '[{"a":1,"b":2},{"a":3,"b":4}]');

-- [DDL]
DROP TYPE jpop;

-- [DQL]
SELECT value, json_typeof(value) FROM (values (json '123.4'), (json '"foo"'), (json 'true'), (json 'null'), (json '[1, 2, 3]'), (json '{"x":"foo", "y":123}'), (NULL::json)) AS data(value);

-- [DQL]
SELECT json_build_array('a',1,'b',1.2,'c',true,'d',null,'e',json '{"x": 3, "y": [1,2,3]}','');

-- [DQL]
SELECT json_build_object(1,2);

-- [DQL]
SELECT jsonb_build_object('a',1,'b',1.2,'c',true,'d',null,'e',json '{"x": 3, "y": [1,2,3]}');

-- [DQL]
SELECT jsonb_build_object();

-- [DQL]
SELECT * FROM json_to_record('{"a":1,"b":"foo","c":"bar"}',true) AS x(a int, b text, d text);

-- [DQL]
SELECT * FROM json_to_record('{"a": {"x": 1, "y": 2},"b":"foo","c":[1, 2]}') AS x(a json, b text, c int[]);

-- [DQL]
SELECT * FROM json_to_recordset('[{"a":1,"b":"foo","d":false},{"a":2,"b":"bar","c":true}]',false) AS x(a int, b text, c boolean);

-- [DQL]
SELECT json_object('{a,1,b,2,3,NULL,"d e f","a b c"}');

-- [DQL]
SELECT json_object('{a,b,"a b c"}', '{a,1,1}');

-- [DQL]
SELECT json_object('d',2,'c','name','b',true,'a',2,'a',NULL,'d',1);

-- [DQL]
SELECT json_object('d',2,true,'name','b',true,'a',2,'aa', current_timestamp);

-- [DQL]
SELECT json_array_append('[1, [2, 3]]', '$[1]', 4, '$[0]', false, '$[0]', null, '$[0]', current_timestamp);

-- [DQL]
SELECT json_array();

-- [DQL]
SELECT json_array(TRUE, FALSE, NULL, 114, 'text', current_timestamp);

-- [DQL]
SELECT json_array_insert('[1, [2, 3]]', '$[1]', 4);

-- [DQL]
SELECT json_array_insert('{"x": 1, "y": [1, 2]}', '$.y[0]', NULL, '$.y[0]', 123, '$.y[3]', current_timestamp);

-- [DQL]
SELECT json_contains('[1, 2, {"x": 3}]', '{"x":3}');

-- [DQL]
SELECT json_contains('[1, 2, {"x": 3},[1,2,3,4]]', '2','$[1]');

-- [DQL]
SELECT json_contains('[1, 2, {"x": 3},[1,2,3,4]]', '1','$[1]');

-- [DQL]
SELECT json_contains_path('[1, 2, {"x": 3}]', 'all', '$[0]', '$[1]', '$[2]');

-- [DQL]
SELECT json_contains_path('[1, 2, {"x": 3}]', 'all', '$[0]', '$[1]', '$[6]');

-- [DQL]
SELECT json_contains_path('[1, 2, {"x": 3}]', 'one', '$[0]', '$[1]', '$[5]');

-- [DQL]
SELECT json_depth('[]');

-- [DQL]
SELECT json_depth('{"s":1, "x":2,"y":[1]}');

-- [DQL]
SELECT json_extract('[1, 2, {"x": 3}]', '$[2]');

-- [DQL]
SELECT json_extract('["a", ["b", "c"], "d"]', '$[1]', '$[2]', '$[3]');

-- [DQL]
SELECT json_insert('[1, [2, 3], {"a": [4, 5]}]', '$[3]', 2);

-- [DQL]
SELECT json_insert('[1, [2, 3], {"a": [4, 5]}]', '$[10]', 10,'$[5]', 5);

-- [DQL]
SELECT json_keys('{"x": 1, "y": 2, "z": 3}');

-- [DQL]
SELECT json_keys('[1,2,3,{"name":"Tom"}]','$[3]');

-- [DQL]
SELECT json_length('[1,2,3,4,5]');

-- [DQL]
SELECT json_length('{"name":"Tom", "age":24, "like":"football"}');

-- [DQL]
SELECT json_merge('[1, 2]','[2]');

-- [DQL]
SELECT json_merge('{"b":"2"}','{"a":"1"}','[1,2]');

-- [DQL]
SELECT json_quote('gauss');

-- [DQL]
SELECT json_unquote('"gauss"');

-- [DQL]
SELECT json_remove('[0, 1, 2, [3, 4]]', '$[0]', '$[2]');

-- [DQL]
SELECT json_remove('[0, 1, 2, [3, 4]]', '$[0]', '$[0]','$[0]');

-- [DQL]
SELECT json_replace('{"x": 1}', '$.x', 'true');

-- [DQL]
SELECT json_replace('{"x": 1}', '$.x', true, '$.x', 123, '$.x', 'asd', '$.x', null);

-- [DQL]
SELECT json_search('{"a":"abc","b":"abc"}','all','abc');

-- [DQL]
SELECT json_search('{"a":"abc","b":"abc"}','one','abc');

-- [DQL]
SELECT json_search('{"a":"abc","b":"a%c"}','one','a\%c');

-- [DQL]
SELECT json_set('{"s":3}','$.s','d');

-- [DQL]
SELECT json_set('{"s":3}','$.a','d','$.a','1');

-- [DQL]
SELECT json_type('{"w":{"2":3},"2":4}');

-- [DQL]
SELECT json_type('[1,2,2,3,3,4,4,4,4,4,4,4,4]');

-- [DQL]
SELECT json_valid('{"name":"Tom"}');

-- [DQL]
SELECT json_valid('[1,23,4,5,5]');

-- [DQL]
SELECT json_valid('[1,23,4,5,5]}');

-- [DDL]
CREATE TABLE classes(name varchar, score int);

-- [DML_INSERT]
INSERT INTO classes VALUES('A',2);

-- [DML_INSERT]
INSERT INTO classes VALUES('A',3);

-- [DML_INSERT]
INSERT INTO classes VALUES('D',5);

-- [DML_INSERT]
INSERT INTO classes VALUES('D',null);

-- [DQL]
SELECT * FROm classes;

-- [DQL]
SELECT name, json_agg(score) score FROM classes GROUP BY name ORDER BY name;

-- [DDL]
DROP TABLE classes;

-- [DDL]
CREATE TABLE classes(name varchar, score int);

-- [DML_INSERT]
INSERT INTO classes VALUES('A',2);

-- [DML_INSERT]
INSERT INTO classes VALUES('A',3);

-- [DML_INSERT]
INSERT INTO classes VALUES('D',5);

-- [DML_INSERT]
INSERT INTO classes VALUES('D',null);

-- [DQL]
SELECT * FROM classes;

-- [DQL]
SELECT json_object_agg(name, score) FROM classes GROUP BY name ORDER BY name;

-- [DDL]
DROP TABLE classes;

-- [DQL]
SELECT jsonb_contained('[1,2,3]', '[1,2,3,4]');

-- [DQL]
SELECT jsonb_contains('[1,2,3,4]', '[1,2,3]');

-- [DQL]
SELECT jsonb_exists('["1",2,3]', '1');

-- [DQL]
SELECT jsonb_exists_all('["1","2",3]', '{1, 2}');

-- [DQL]
SELECT jsonb_exists_any('["1","2",3]', '{1, 2, 4}');

-- [DQL]
SELECT jsonb_cmp('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT jsonb_eq('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT jsonb_ne('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT jsonb_gt('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT jsonb_ge('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT jsonb_lt('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT jsonb_le('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT to_json('{1,5}'::text[]);

-- [DQL]
SELECT to_jsonb(array[1, 2, 3, 4]);

-- [DQL]
SELECT jsonb_hash('[1,2,3]');


================================================================================
-- 来源: 1083_HLL.txt
================================================================================

-- [DQL]
SELECT hll_hash_boolean ( FALSE );

-- [DQL]
SELECT hll_hash_boolean ( FALSE , 10 );

-- [DQL]
SELECT hll_hash_smallint ( 100 :: smallint );

-- [DQL]
SELECT hll_hash_smallint ( 100 :: smallint , 10 );

-- [DQL]
SELECT hll_hash_integer ( 0 );

-- [DQL]
SELECT hll_hash_integer ( 0 , 10 );

-- [DQL]
SELECT hll_hash_bigint ( 100 :: bigint );

-- [DQL]
SELECT hll_hash_bigint ( 100 :: bigint , 10 );

-- [DQL]
SELECT hll_hash_bytea ( E '\\x' );

-- [DQL]
SELECT hll_hash_bytea ( E '\\x' , 10 );

-- [DQL]
SELECT hll_hash_text ( 'AB' );

-- [DQL]
SELECT hll_hash_text ( 'AB' , 10 );

-- [DQL]
SELECT hll_hash_any ( 1 );

-- [DQL]
SELECT hll_hash_any ( '08:00:2b:01:02:03' :: macaddr );

-- [DQL]
SELECT hll_hash_any ( 1 , 10 );

-- [DQL]
SELECT hll_hashval_eq ( hll_hash_integer ( 1 ), hll_hash_integer ( 1 ));

-- [DQL]
SELECT hll_hashval_ne ( hll_hash_integer ( 1 ), hll_hash_integer ( 1 ));

-- [DQL]
SELECT hll_print ( hll_empty ());

-- [DQL]
SELECT hll_type ( hll_empty ());

-- [DQL]
SELECT hll_log2m ( hll_empty ());

-- [DQL]
SELECT hll_log2m ( hll_empty ( 10 ));

-- [DQL]
SELECT hll_log2m ( hll_empty ( - 1 ));

-- [DQL]
SELECT hll_log2explicit ( hll_empty ());

-- [DQL]
SELECT hll_log2explicit ( hll_empty ( 12 , 8 ));

-- [DQL]
SELECT hll_log2explicit ( hll_empty ( 12 , - 1 ));

-- [DQL]
SELECT hll_log2sparse ( hll_empty ());

-- [DQL]
SELECT hll_log2sparse ( hll_empty ( 12 , 8 , 10 ));

-- [DQL]
SELECT hll_log2sparse ( hll_empty ( 12 , 8 , - 1 ));

-- [DQL]
SELECT hll_duplicatecheck ( hll_empty ());

-- [DQL]
SELECT hll_duplicatecheck ( hll_empty ( 12 , 8 , 10 , 1 ));

-- [DQL]
SELECT hll_duplicatecheck ( hll_empty ( 12 , 8 , 10 , - 1 ));

-- [DQL]
SELECT hll_empty ();

-- [DQL]
SELECT hll_empty ( 10 );

-- [DQL]
SELECT hll_empty ( - 1 );

-- [DQL]
SELECT hll_empty ( 10 , 4 );

-- [DQL]
SELECT hll_empty ( 10 , - 1 );

-- [DQL]
SELECT hll_empty ( 10 , 4 , 8 );

-- [DQL]
SELECT hll_empty ( 10 , 4 , - 1 );

-- [DQL]
SELECT hll_empty ( 10 , 4 , 8 , 0 );

-- [DQL]
SELECT hll_empty ( 10 , 4 , 8 , - 1 );

-- [DQL]
SELECT hll_add ( hll_empty (), hll_hash_integer ( 1 ));

-- [DQL]
SELECT hll_add_rev ( hll_hash_integer ( 1 ), hll_empty ());

-- [DQL]
SELECT hll_eq ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

-- [DQL]
SELECT hll_ne ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

-- [DQL]
SELECT hll_cardinality ( hll_empty () || hll_hash_integer ( 1 ));

-- [DQL]
SELECT hll_union ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

-- [DDL]
CREATE TABLE t_id ( id int );

-- [DML_INSERT]
INSERT INTO t_id VALUES ( generate_series ( 1 , 500 ));

-- [DDL]
CREATE TABLE t_data ( a int , c text );

-- [DML_INSERT]
INSERT INTO t_data SELECT mod ( id , 2 ), id FROM t_id ;

-- [DDL]
CREATE TABLE t_a_c_hll ( a int , c hll );

-- [DML_INSERT]
INSERT INTO t_a_c_hll SELECT a , hll_add_agg ( hll_hash_text ( c )) FROM t_data GROUP BY a ;

-- [DQL]
SELECT a , # c AS cardinality FROM t_a_c_hll ORDER BY a ;

-- [DQL]
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), 12 )) FROM t_data ;

-- [DQL]
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 1 )) FROM t_data ;

-- [DQL]
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 6 , 10 )) FROM t_data ;

-- [DQL]
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 6 , 10 , - 1 )) FROM t_data ;

-- [DQL]
SELECT # hll_union_agg ( c ) AS cardinality FROM t_a_c_hll ;

-- [DQL]
SELECT ( hll_empty () || hll_hash_integer ( 1 )) = ( hll_empty () || hll_hash_integer ( 1 ));

-- [DQL]
SELECT hll_hash_integer ( 1 ) = hll_hash_integer ( 1 );

-- [DQL]
SELECT ( hll_empty () || hll_hash_integer ( 1 )) <> ( hll_empty () || hll_hash_integer ( 2 ));

-- [DQL]
SELECT hll_hash_integer ( 1 ) <> hll_hash_integer ( 2 );

-- [DQL]
SELECT hll_empty () || hll_hash_integer ( 1 );

-- [DQL]
SELECT hll_hash_integer ( 1 ) || hll_empty ();

-- [DQL]
SELECT ( hll_empty () || hll_hash_integer ( 1 )) || ( hll_empty () || hll_hash_integer ( 2 ));

-- [DQL]
SELECT # ( hll_empty () || hll_hash_integer ( 1 ));


================================================================================
-- 来源: 1084_SEQUENCE.txt
================================================================================

-- [DDL]
CREATE SEQUENCE seqDemo ;

-- [DQL]
SELECT nextval ( 'seqDemo' );

-- [DQL]
SELECT seqDemo . nextval ;

-- [DDL]
DROP SEQUENCE seqDemo ;

-- [DDL]
CREATE SEQUENCE seq1 ;

-- [DQL]
SELECT nextval ( 'seq1' );

-- [SESSION]
SET enable_beta_features = on ;

-- [DQL]
SELECT currval ( 'seq1' );

-- [DQL]
SELECT seq1 . currval seq1 ;

-- [DDL]
DROP SEQUENCE seq1 ;

-- [DDL]
CREATE SEQUENCE seq1 ;

-- [DQL]
SELECT nextval ( 'seq1' );

-- [SESSION]
SET enable_beta_features = on ;

-- [DQL]
SELECT lastval ();

-- [DDL]
DROP SEQUENCE seq1 ;

-- [DDL]
CREATE SEQUENCE seqDemo ;

-- [DQL]
SELECT nextval ( 'seqDemo' );

-- [DQL]
SELECT setval ( 'seqDemo' , 3 );

-- [DDL]
DROP SEQUENCE seqDemo ;

-- [DDL]
CREATE SEQUENCE seqDemo ;

-- [DQL]
SELECT nextval ( 'seqDemo' );

-- [DQL]
SELECT setval ( 'seqDemo' , 5 , true );

-- [DDL]
DROP SEQUENCE seqDemo ;


================================================================================
-- 来源: 1085_file_1085.txt
================================================================================

-- [DQL]
SELECT ARRAY [ 1 . 1 , 2 . 1 , 3 . 1 ]:: int [] = ARRAY [ 1 , 2 , 3 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 2 , 3 ] <> ARRAY [ 1 , 2 , 4 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 2 , 3 ] < ARRAY [ 1 , 2 , 4 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 4 , 3 ] > ARRAY [ 1 , 2 , 4 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 2 , 3 ] <= ARRAY [ 1 , 2 , 3 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 4 , 3 ] >= ARRAY [ 1 , 4 , 3 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 4 , 3 ] @> ARRAY [ 3 , 1 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 2 , 7 ] <@ ARRAY [ 1 , 7 , 4 , 2 , 6 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 4 , 3 ] && ARRAY [ 2 , 1 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 2 , 3 ] || ARRAY [ 4 , 5 , 6 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 2 , 3 ] || ARRAY [[ 4 , 5 , 6 ],[ 7 , 8 , 9 ]] AS RESULT ;

-- [DQL]
SELECT 3 || ARRAY [ 4 , 5 , 6 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 4 , 5 , 6 ] || 7 AS RESULT ;

-- [DQL]
SELECT array_append ( ARRAY [ 1 , 2 ], 3 ) AS RESULT ;

-- [DQL]
SELECT array_prepend ( 1 , ARRAY [ 2 , 3 ]) AS RESULT ;

-- [DQL]
SELECT array_cat ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_cat ( ARRAY [[ 1 , 2 ],[ 4 , 5 ]], ARRAY [ 6 , 7 ]) AS RESULT ;

-- [DQL]
SELECT array_union ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_union ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- [DQL]
SELECT array_union_distinct ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_union_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- [DQL]
SELECT array_intersect ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_intersect ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- [DQL]
SELECT array_intersect_distinct ( ARRAY [ 1 , 2 , 2 ], ARRAY [ 2 , 2 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_intersect_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- [DQL]
SELECT array_except ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_except ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- [DQL]
SELECT array_except ( NULL , ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_except_distinct ( ARRAY [ 1 , 2 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_except_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- [DQL]
SELECT array_except_distinct ( NULL , ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_ndims ( ARRAY [[ 1 , 2 , 3 ], [ 4 , 5 , 6 ]]) AS RESULT ;

-- [DQL]
SELECT array_dims ( ARRAY [[ 1 , 2 , 3 ], [ 4 , 5 , 6 ]]) AS RESULT ;

-- [DQL]
SELECT array_length ( array [ 1 , 2 , 3 ], 1 ) AS RESULT ;

-- [DQL]
SELECT array_lower ( '[0:2]={1,2,3}' :: int [], 1 ) AS RESULT ;

-- [DQL]
SELECT array_sort ( ARRAY [ 5 , 1 , 3 , 6 , 2 , 7 ]) AS RESULT ;

-- [DQL]
SELECT array_upper ( ARRAY [ 1 , 8 , 3 , 7 ], 1 ) AS RESULT ;

-- [DQL]
SELECT array_to_string ( ARRAY [ 1 , 2 , 3 , NULL , 5 ], ',' , '*' ) AS RESULT ;

-- [DQL]
SELECT array_delete(ARRAY[1,8,3,7]) AS RESULT;

-- [DQL]
SELECT array_deleteidx(ARRAY[1,2,3,4,5], 1) AS RESULT;

-- [DQL]
SELECT array_extendnull(ARRAY[1,8,3,7],1) AS RESULT;

-- [DQL]
SELECT array_extendnull(ARRAY[1,8,3,7],2,2) AS RESULT;

-- [DQL]
SELECT array_trim(ARRAY[1,8,3,7],1) AS RESULT;

-- [DQL]
SELECT array_exists(ARRAY[1,8,3,7],1) AS RESULT;

-- [DQL]
SELECT array_next(ARRAY[1,8,3,7],1) AS RESULT;

-- [DQL]
SELECT array_prior(ARRAY[1,8,3,7],2) AS RESULT;

-- [DQL]
SELECT string_to_array ( 'xx~^~yy~^~zz' , '~^~' , 'yy' ) AS RESULT ;

-- [DQL]
SELECT string_to_array ( 'xx~^~yy~^~zz' , '~^~' , 'y' ) AS RESULT ;

-- [DQL]
SELECT unnest ( ARRAY [ 1 , 2 ]) AS RESULT ;

-- [DQL]
SELECT cardinality(array[[1, 2], [3, 4]]);

-- [DQL]
SELECT array_positions(array[1, 2, 3, 1], 1) AS RESULT;


================================================================================
-- 来源: 1086_file_1086.txt
================================================================================

-- [DQL]
SELECT int4range ( 1 , 5 ) = '[1,4]' :: int4range AS RESULT ;

-- [DQL]
SELECT numrange ( 1 . 1 , 2 . 2 ) <> numrange ( 1 . 1 , 2 . 3 ) AS RESULT ;

-- [DQL]
SELECT int4range ( 1 , 10 ) < int4range ( 2 , 3 ) AS RESULT ;

-- [DQL]
SELECT int4range ( 1 , 10 ) > int4range ( 1 , 5 ) AS RESULT ;

-- [DQL]
SELECT numrange ( 1 . 1 , 2 . 2 ) <= numrange ( 1 . 1 , 2 . 2 ) AS RESULT ;

-- [DQL]
SELECT numrange ( 1 . 1 , 2 . 2 ) >= numrange ( 1 . 1 , 2 . 0 ) AS RESULT ;

-- [DQL]
SELECT int4range ( 2 , 4 ) @> int4range ( 2 , 3 ) AS RESULT ;

-- [DQL]
SELECT '[2011-01-01,2011-03-01)' :: tsrange @> '2011-01-10' :: timestamp AS RESULT ;

-- [DQL]
SELECT int4range ( 2 , 4 ) <@ int4range ( 1 , 7 ) AS RESULT ;

-- [DQL]
SELECT 42 <@ int4range ( 1 , 7 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 3 , 7 ) && int8range ( 4 , 12 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 1 , 10 ) << int8range ( 100 , 110 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 50 , 60 ) >> int8range ( 20 , 30 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 1 , 20 ) &< int8range ( 18 , 20 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 7 , 20 ) &> int8range ( 5 , 10 ) AS RESULT ;

-- [DQL]
SELECT numrange ( 1 . 1 , 2 . 2 ) -|- numrange ( 2 . 2 , 3 . 3 ) AS RESULT ;

-- [DQL]
SELECT numrange ( 5 , 15 ) + numrange ( 10 , 20 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 5 , 15 ) * int8range ( 10 , 20 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 5 , 15 ) - int8range ( 10 , 20 ) AS RESULT ;

-- [DQL]
SELECT numrange ( 1 . 1 , 2 . 2 ) AS RESULT ;

-- [DQL]
SELECT numrange ( 1 . 1 , 2 . 2 , '()' ) AS RESULT ;

-- [DQL]
SELECT lower ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- [DQL]
SELECT upper ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- [DQL]
SELECT isempty ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- [DQL]
SELECT lower_inc ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- [DQL]
SELECT upper_inc ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- [DQL]
SELECT lower_inf ( '(,)' :: daterange ) AS RESULT ;

-- [DQL]
SELECT upper_inf ( '(,)' :: daterange ) AS RESULT ;

-- [DQL]
SELECT elem_contained_by_range ( '2' , numrange ( 1 . 1 , 2 . 2 ));


================================================================================
-- 来源: 1087_file_1087.txt
================================================================================

-- [DDL]
CREATE TABLE tab ( a int );

-- [DML_INSERT]
INSERT INTO tab values ( 1 );

-- [DML_INSERT]
INSERT INTO tab values ( 2 );

-- [DQL]
SELECT sum ( a ) FROM tab ;

-- [DQL]
SELECT MAX ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- [DQL]
SELECT MIN ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- [DQL]
SELECT AVG ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- [DQL]
SELECT COUNT ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- [DQL]
SELECT COUNT ( * ) FROM tpcds . inventory ;

-- [DQL]
SELECT ARRAY_AGG ( sr_fee ) FROM tpcds . store_returns WHERE sr_customer_sk = 2 ;

-- [DQL]
SELECT string_agg ( sr_item_sk , ',' ) FROM tpcds . store_returns WHERE sr_item_sk < 3 ;

-- [DQL]
SELECT deptno , listagg ( ename , ',' ) WITHIN GROUP ( ORDER BY ename ) AS employees FROM emp GROUP BY deptno ;

-- [DQL]
SELECT deptno , listagg ( mgrno , ',' ) WITHIN GROUP ( ORDER BY mgrno NULLS FIRST ) AS mgrnos FROM emp GROUP BY deptno ;

-- [DQL]
SELECT job , listagg ( bonus , '($);

-- [DQL]
SELECT deptno , listagg ( hiredate , ', ' ) WITHIN GROUP ( ORDER BY hiredate DESC ) AS hiredates FROM emp GROUP BY deptno ;

-- [DQL]
SELECT deptno , listagg ( vacationTime , ';

-- [DQL]
SELECT deptno , listagg ( job ) WITHIN GROUP ( ORDER BY job ) AS jobs FROM emp GROUP BY deptno ;

-- [DQL]
SELECT deptno , mgrno , bonus , listagg ( ename , ';

-- [DQL]
SELECT id , group_concat ( v separator ';

-- [DQL]
SELECT id , group_concat ( id , v ) FROM t GROUP BY id ORDER BY id ASC ;

-- [DQL]
SELECT id , group_concat ( v ) FROM t GROUP BY id ORDER BY id ASC ;

-- [DQL]
SELECT id , group_concat ( v separator ';

-- [DQL]
SELECT id , group_concat ( v separator ';

-- [DQL]
SELECT id , group_concat ( hiredate separator ';

-- [DQL]
SELECT id , group_concat ( v separator ';

-- [DQL]
SELECT id , group_concat ( vacationt separator ';

-- [DQL]
SELECT id , group_concat ( distinct v ) FROM t GROUP BY id ORDER BY id ASC ;

-- [DQL]
SELECT id , group_concat ( v ORDER BY v desc ) FROM t GROUP BY id ORDER BY id ASC ;

-- [DQL]
SELECT COVAR_POP ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT COVAR_SAMP ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT STDDEV_POP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT STDDEV_SAMP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT VAR_POP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT VAR_SAMP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT BIT_AND ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT BIT_OR ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT bool_and ( 100 < 2500 );

-- [DQL]
SELECT bool_or ( 100 < 2500 );

-- [DQL]
SELECT CORR ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT every ( 100 < 2500 );

-- [DQL]
SELECT d_moy , d_fy_week_seq , rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT REGR_AVGX ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_AVGY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_COUNT ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_INTERCEPT ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_R2 ( sr_fee , sr_net_loss ) FROM store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_SLOPE ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_SXX ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_SXY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_SYY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT STDDEV ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT VARIANCE ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT * FROM pivot_func_test;

-- [DQL]
SELECT id, pivot_func(val) FROM pivot_func_test GROUP BY id;

-- [DQL]
SELECT CHECKSUM ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- [DQL]
SELECT CHECKSUM ( inv_quantity_on_hand :: TEXT ) FROM tpcds . inventory ;

-- [DQL]
SELECT CHECKSUM ( inventory :: TEXT ) FROM tpcds . inventory ;


================================================================================
-- 来源: 1088_file_1088.txt
================================================================================

-- [DQL]
SELECT d_moy , d_fy_week_seq , rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , Row_number () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , dense_rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , percent_rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , cume_dist () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim e_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , ntile ( 3 ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , lag ( d_moy , 3 , null ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , lead ( d_fy_week_seq , 2 ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , first_value ( d_fy_week_seq ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , last_value ( d_moy ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 6 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , nth_value ( d_fy_week_seq , 6 ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 6 ORDER BY 1 , 2 ;

-- [DQL]
SELECT sales_group , sales_id , sales_amount , RATIO_TO_REPORT ( sales_amount ) OVER ( PARTITION BY sales_group ) FROM sales_int8 ORDER BY sales_id ;

-- [DQL]
SELECT sales_group , sales_id , sales_amount , TO_CHAR ( RATIO_TO_REPORT ( sales_amount ) OVER (), '$999eeee' ) FROM sales ORDER BY sales_id ;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc IS CURSOR cur_1 IS SELECT RATIO_TO_REPORT ( sales_amount ) OVER () FROM sales_numeric ;

-- [PLSQL]
CALL PROC ();


================================================================================
-- 来源: 1089_file_1089.txt
================================================================================

-- [DQL]
SELECT gs_encrypt_aes128 ( 'MPPDB' , '1234@abc' );

-- [DQL]
SELECT gs_decrypt_aes128 ( 'OF1g3+70oeqFfyKiWlpxfYxPnpeitNc6+7nAe02Ttt37fZF8Q+bbEYhdw/YG+0c9tHKRWM6OcTzlB3HnqvX+1d8Bflo=' , '1234@abc' );

-- [DQL]
select aes_encrypt('huwei123','123456vfhex4dyu,vdaladhjsadad','1234567890123456');

-- [DQL]
select aes_decrypt(aes_encrypt('huwei123','123456vfhex4dyu,vdaladhjsadad','1234567890123456'),'123456vfhex4dyu,vdaladhjsadad','1234567890123456');

-- [DQL]
SELECT pg_catalog . gs_digest ( 'gaussdb' , 'sha256' );

-- [DQL]
SELECT gs_password_deadline ();

-- [DQL]
SELECT inet_server_addr ();

-- [DQL]
SELECT inet_client_addr ();

-- [DQL]
SELECT gs_encrypt('MPPDB', 'Asdf1234', 'sm4');

-- [DQL]
select gs_decrypt('ZBzOmaGA4Bb+coyucJ0B8AkIShqc', 'Asdf1234', 'sm4');

-- [DQL]
SELECT gs_encrypt_bytea('MPPDB', 'Asdf1234', 'sm4_ctr_sm3');

-- [DQL]
select gs_decrypt_bytea('\x90e286971c2c70410def0a2814af4ac44c737926458b66271d9d1547bc937395ca018d7755672fa9dc3cdc6ec4a76001dc0e137f3bc5c8a5c51143561f1d09a848bfdebfec5e', 'Asdf1234', 'sm4_ctr_sm3');


================================================================================
-- 来源: 1091_file_1091.txt
================================================================================

-- [DQL]
SELECT * FROM generate_series ( 2 , 4 );

-- [DQL]
SELECT * FROM generate_series ( 5 , 1 , - 2 );

-- [DQL]
SELECT * FROM generate_series ( 4 , 3 );

-- [DQL]
SELECT current_date + s . a AS dates FROM generate_series ( 0 , 14 , 7 ) AS s ( a );

-- [DQL]
SELECT * FROM generate_series ( '2008-03-01 00:00' :: timestamp , '2008-03-04 12:00' , '10 hours' );

-- [DQL]
SELECT generate_subscripts ( '{NULL,1,NULL,2}' :: int [], 1 ) AS s ;

-- [DDL]
CREATE OR REPLACE FUNCTION unnest2 ( anyarray ) RETURNS SETOF anyelement AS $$ SELECT $ 1 [ i ][ j ] FROM generate_subscripts ( $ 1 , 1 ) g1 ( i ), generate_subscripts ( $ 1 , 2 ) g2 ( j );

-- [DQL]
SELECT * FROM unnest2 ( ARRAY [[ 1 , 2 ],[ 3 , 4 ]]);

-- [DDL]
DROP FUNCTION unnest2 ;


================================================================================
-- 来源: 1092_file_1092.txt
================================================================================

-- [DQL]
SELECT coalesce ( NULL , 'hello' );

-- [DQL]
SELECT decode ( 'A' , 'A' , 1 , 'B' , 2 , 0 );

-- [DQL]
SELECT nullif ( 'hello' , 'world' );

-- [DQL]
SELECT nullif ( '1234' :: VARCHAR , 123 :: INT4 );

-- [DQL]
SELECT nullif ( '1234' :: VARCHAR , '2012-12-24' :: DATE );

-- [DQL]
SELECT nullif ( 1 :: bit , '1' :: MONEY );

-- [DQL]
SELECT nvl ( 'hello' , 'world' );

-- [DQL]
SELECT nvl2 ( 'hello' , 'world' , 'other' );

-- [DQL]
SELECT greatest ( 1 * 2 , 2 - 3 , 4 - 1 );

-- [DQL]
SELECT greatest ( 'HARRY' , 'HARRIOT' , 'HAROLD' );

-- [DQL]
SELECT least ( 1 * 2 , 2 - 3 , 4 - 1 );

-- [DQL]
SELECT least ( 'HARRY' , 'HARRIOT' , 'HAROLD' );

-- [DDL]
CREATE TABLE blob_tb ( b blob , id int ) DISTRIBUTE BY REPLICATION ;

-- [DML_INSERT]
INSERT INTO blob_tb VALUES ( empty_blob (), 1 );

-- [DDL]
DROP TABLE blob_tb ;

-- [DDL]
CREATE TABLE clob_tb ( c clob , id int );

-- [DML_INSERT]
INSERT INTO clob_tb VALUES ( empty_clob (), 1 );

-- [DDL]
DROP TABLE clob_tb ;

-- [DDL]
CREATE TABLE student_demo ( name VARCHAR2 ( 20 ), grade NUMBER ( 10 , 2 ));

-- [DML_INSERT]
INSERT INTO student_demo VALUES ( 'name0' , 0 );

-- [DML_INSERT]
INSERT INTO student_demo VALUES ( 'name1' , 1 );

-- [DML_INSERT]
INSERT INTO student_demo VALUES ( 'name2' , 2 );

-- [DQL]
SELECT * FROM student_demo WHERE LNNVL ( name = 'name1' );

-- [DQL]
SELECT isnull ( null );

-- [DQL]
SELECT isnull ( 1 );

-- [DQL]
select if ( 2 > 3 , 'true' , 'false' );

-- [DQL]
select if ( null , 'not null' , 'is null' );

-- [DQL]
select ifnull ( '' , null ) is null as a ;

-- [DQL]
select ifnull ( null , null ) is null as a ;

-- [DQL]
select ifnull ( null , 'A' ) as a ;


================================================================================
-- 来源: 1093_file_1093.txt
================================================================================

-- [DQL]
SELECT current_query ();

-- [DQL]
SELECT current_schema ();

-- [DQL]
SELECT current_schemas ( true );

-- [DQL]
SELECT database ();

-- [DQL]
SELECT current_user ;

-- [DQL]
SELECT definer_current_user ();

-- [DQL]
SELECT pg_current_sessionid ();

-- [DQL]
select pg_current_sessid();

-- [DQL]
SELECT pg_current_userid();

-- [DQL]
select tablespace_oid_name ( 1663 );

-- [DQL]
SELECT inet_client_addr ();

-- [DQL]
SELECT inet_client_port ();

-- [DQL]
SELECT inet_server_addr ();

-- [DQL]
SELECT inet_server_port ();

-- [DQL]
SELECT pg_backend_pid ();

-- [DQL]
SELECT pg_conf_load_time ();

-- [DQL]
SELECT pg_my_temp_schema ();

-- [DQL]
SELECT pg_is_other_temp_schema ( 25356 );

-- [DQL]
SELECT pg_listening_channels ();

-- [DQL]
SELECT pg_postmaster_start_time ();

-- [DQL]
select sessionid2pid ( sessid :: cstring ) from pv_session_stat limit 2 ;

-- [DQL]
SELECT session_context ( 'USERENV' , 'CURRENT_SCHEMA' );

-- [DQL]
SELECT pg_trigger_depth ();

-- [DQL]
SELECT opengauss_version ();

-- [DQL]
select gs_deployment ();

-- [DQL]
SELECT session_user ;

-- [DQL]
SELECT user ;

-- [DQL]
select get_shard_oids_byname ( 'datanode1' );

-- [DQL]
select getpgusername ();

-- [DQL]
select getdatabaseencoding ();

-- [DQL]
SELECT version ();

-- [DQL]
SELECT working_version_num ();

-- [DQL]
SELECT get_hostname ();

-- [DQL]
SELECT get_nodename ();

-- [DQL]
SELECT get_nodeinfo ( 'node_type' );

-- [DQL]
SELECT get_nodeinfo ( 'node_name' );

-- [DQL]
SELECT get_schema_oid ( 'public' );

-- [DQL]
SELECT pgxc_parse_clog ();

-- [DQL]
SELECT pgxc_parse_clog ( '-1' );

-- [DQL]
SELECT pgxc_prepared_xact ();

-- [DQL]
SELECT pgxc_xacts_iscommitted ( 1 );

-- [DQL]
SELECT pgxc_total_memory_detail ();

-- [DQL]
SELECT has_table_privilege ( 'tpcds.web_site' , 'select' );

-- [DQL]
SELECT has_table_privilege ( 'omm' , 'tpcds.web_site' , 'select,INSERT WITH GRANT OPTION ' );

-- [DQL]
SELECT relname FROM pg_class WHERE pg_table_is_visible ( oid );

-- [DQL]
SELECT format_type (( SELECT oid FROM pg_type WHERE typname = 'varchar' ), 10 );

-- [DQL]
select pg_check_authid(1);

-- [DQL]
select * from pg_get_functiondef(598);

-- [DQL]
select * from pg_get_indexdef(16416);

-- [DQL]
select * from pg_get_indexdef(16416, true);

-- [DQL]
select * from pg_get_indexdef(16416, 0, false);

-- [DQL]
select * from pg_get_indexdef(16416, 1, false);

-- [DQL]
select pg_check_authid(20);

-- [DQL]
select * from pg_get_tabledef(16384);

-- [DQL]
select * from pg_get_tabledef('t1');

-- [DQL]
SELECT pg_typeof ( 33 );

-- [DQL]
SELECT typlen FROM pg_type WHERE oid = pg_typeof ( 33 );

-- [DQL]
SELECT collation for ( description ) FROM pg_description LIMIT 1 ;

-- [DQL]
SELECT getdistributekey ( 'item' );

-- [DQL]
select * from pg_get_serial_sequence('t1', 'c1');

-- [DQL]
select * from pg_sequence_parameters(16420);

-- [DQL]
select pgxc_get_variable_info( );

-- [DQL]
select * from gs_get_index_status('public', 'index1');

-- [DQL]
select * from gs_get_kernel_info();


================================================================================
-- 来源: 1095_file_1095.txt
================================================================================

-- [DQL]
SELECT current_setting ( 'datestyle' );

-- [DQL]
SELECT set_config ( 'log_statement_stats' , 'off' , false );


================================================================================
-- 来源: 1096_file_1096.txt
================================================================================

-- [DQL]
SELECT pg_ls_dir ( './' );

-- [DQL]
SELECT pg_read_file ( 'postmaster.pid' , 0 , 100 );

-- [DQL]
SELECT convert_from ( pg_read_binary_file ( 'filename' ), 'UTF8' );

-- [DQL]
SELECT * FROM pg_stat_file ( 'filename' );

-- [DQL]
SELECT ( pg_stat_file ( 'filename' )). modification ;

-- [DQL]
SELECT convert_from ( pg_read_binary_file ( 'postmaster.pid' ), 'UTF8' );

-- [DQL]
SELECT * FROM pg_stat_file ( 'postmaster.pid' );

-- [DQL]
SELECT ( pg_stat_file ( 'postmaster.pid' )). modification ;


================================================================================
-- 来源: 1097_file_1097.txt
================================================================================

-- [DQL]
SELECT pid from pg_stat_activity ;

-- [DQL]
SELECT pg_terminate_backend ( 140657876268816 );


================================================================================
-- 来源: 1098_file_1098.txt
================================================================================

-- [DQL]
SELECT pg_start_backup ( 'label_goes_here' , true );

-- [DQL]
SELECT * FROM pg_xlogfile_name_offset ( pg_stop_backup ());


================================================================================
-- 来源: 1099_file_1099.txt
================================================================================

-- [DQL]
select * from pg_create_physical_replication_slot_extern ( 'uuid' , false , 'OBS;

-- [DQL]
select * from pg_create_physical_replication_slot_extern ( 'uuid' , false , 'NAS;

-- [DQL]
select gs_set_obs_delete_location('0/54000000');


================================================================================
-- 来源: 1102_file_1102.txt
================================================================================

-- [DQL]
SELECT pg_column_size ( 1 );

-- [DQL]
SELECT pg_database_size ( 'testdb' );

-- [MAINTENANCE]
analyze ;

-- [DQL]
select get_db_source_datasize ();

-- [DQL]
SELECT datalength(1);


================================================================================
-- 来源: 1104_file_1104.txt
================================================================================

-- [DQL]
select * from pg_create_logical_replication_slot('slot_lsn','mppdb_decoding',0);

-- [DQL]
select * from pg_create_logical_replication_slot('slot_csn','mppdb_decoding',1);

-- [DQL]
select * from pg_logical_slot_peek_changes('slot_lsn',NULL,4096,'skip-empty-xacts','on');

-- [DQL]
select * from pg_logical_slot_peek_changes('slot_csn',NULL,4096,'skip-empty-xacts','on');

-- [DQL]
select * from pg_get_replication_slots();

-- [DQL]
select * from pg_get_replication_slots();

-- [DQL]
select * from pg_logical_get_area_changes('0/502E418', NULL, NULL, 'sql_decoding', NULL);

-- [DQL]
select * from gs_get_parallel_decode_status();

-- [DQL]
select * from gs_get_slot_decoded_wal_time('replication_slot');

-- [DQL]
select * from gs_logical_parallel_decode_status('replication_slot');

-- [DQL]
select * from gs_logical_parallel_decode_status('replication_slot');

-- [DQL]
select * from gs_logical_parallel_decode_reset_status('replication_slot');

-- [DQL]
select * from gs_logical_parallel_decode_status('replication_slot');

-- [DQL]
select * from gs_get_parallel_decode_thread_info();

-- [DQL]
SELECT * FROM gs_get_distribute_decode_status();

-- [DQL]
SELECT * FROM gs_get_distribute_decode_status_detail();


================================================================================
-- 来源: 1105_file_1105.txt
================================================================================

-- [DQL]
select * from gs_seg_dump_page('pg_default', 1, 1024, 4157);

-- [DQL]
select * from gs_seg_dump_page(16788, 1024, 0);

-- [DQL]
select * from gs_seg_get_spc_location('pg_default', 1024, 4157, 0);

-- [DQL]
select * from gs_seg_get_spc_location(24578,1024, 0);

-- [DQL]
select * from gs_seg_get_location(4157);

-- [DQL]
select * from gs_seg_get_segment_layout();

-- [DQL]
select * from gs_seg_get_datafile_layout();

-- [DQL]
select * from gs_seg_get_slice_layout(1,1024,0);

-- [DQL]
select * from gs_seg_get_segment('pg_default', 1024, 4157);

-- [DQL]
select * from gs_seg_get_segment(16768, 1024);

-- [DQL]
select * from gs_seg_get_extents('pg_default', 1024, 4157);

-- [DQL]
select * from gs_seg_get_extents(16768, 1024);

-- [DQL]
select * from gs_seg_free_spc_remain_segment('pg_default', 1, 4159);

-- [DQL]
select * from gs_seg_free_spc_remain_extent('pg_default', 1, 0, 4159);

-- [DQL]
select * from gs_seg_get_datafiles();

-- [DQL]
select * from gs_seg_get_spc_extents('pg_default', 1,1024, 0);


================================================================================
-- 来源: 1106_hashbucket.txt
================================================================================

-- [DQL]
SELECT * FROM gs_redis_get_plan(16388, 16417);

-- [DQL]
SELECT * FROM gs_redis_get_bucket_statistics();

-- [DQL]
SELECT gs_redis_set_distributed_db('gaussdb');

-- [DQL]
SELECT * FROM gs_redis_hashbucket_update_segment_header(16388, 16417);

-- [DQL]
SELECT * FROM gs_redis_local_get_segment_header('mytable', '256');

-- [DQL]
SELECT * FROM gs_redis_local_update_segment_header('mytable', '4294967295,4294967295,4294967295,4294967295,....');

-- [DQL]
SELECT * FROM gs_redis_hashbucket_update_inverse_pointer('0,1,2,3,4,5,6,7,8,9,10','datanode1','datanode3');

-- [DQL]
SELECT * FROM gs_redis_hashbucket_update_inverse_pointer('0,1,2,3,4,5,6,7,8,9,10','datanode1','datanode3');

-- [DQL]
SELECT * FROM gs_redis_local_update_inverse_pointer('mytable', '4294967295,4294967295,4294967295,4294967295,....','1 2 3');

-- [DQL]
SELECT * FROM gs_redis_local_set_hashbucket_frozenxid();

-- [DQL]
SELECT * FROM gs_redis_set_hashbucket_frozenxid(16388, 16417);

-- [DQL]
SELECT * FROM gs_redis_set_nextxid('15268817');

-- [DQL]
SELECT * FROM gs_redis_set_csn('15268817');

-- [DQL]
SELECT * FROM gs_redis_check_bucket_flush('{datanode1， datanode2}');

-- [DQL]
SELECT * FROM gs_redis_show_bucketxid('1 2 3');

-- [DQL]
SELECT * FROM gs_redis_drop_bucket_files(16388, 16417);

-- [DQL]
SELECT * FROM gs_redis_local_drop_bucket_files('1 2 3', 3);


================================================================================
-- 来源: 1107_Undo.txt
================================================================================

-- [DQL]
select * from gs_global_config where name like '%undostoragetype%';

-- [DQL]
select * from gs_stat_undo(true);

-- [DQL]
select * from gs_stat_undo(false);

-- [DQL]
select * from gs_undo_meta_dump_zone(-1,true);

-- [DQL]
select * from gs_undo_translot_dump_slot(-1,true);

-- [DQL]
select * from gs_undo_translot_dump_xid('15758',false);

-- [DQL]
select * from gs_undo_dump_record('0000000000000042');

-- [DQL]
select * from gs_undo_dump_xid('15779');

-- [DQL]
select * from gs_verify_undo_record('urp', 24, 24, 1);

-- [DQL]
select * from gs_verify_undo_record('zone', 0, 2, 1);

-- [DQL]
select * from gs_verify_undo_slot('zone', 0, 2, 1);

-- [DQL]
select * from gs_verify_undo_meta('all', 0, 2, 1);


================================================================================
-- 来源: 1108_file_1108.txt
================================================================================

-- [DQL]
select table_skewness('t', 'a',5);

-- [DQL]
select table_skewness('t', 'a');

-- [DQL]
select table_data_skewness(row(index), 'R') from test1;

-- [DQL]
select pg_stat_get_env();

-- [DQL]
select locktag_decode('271b:0:0:0:0:6');

-- [DQL]
select gs_parse_page_bypath('base/16603/16394', -1, 'btree', false);

-- [DQL]
select gs_parse_page_bypath('base/12828/16771_vm', -1, 'vm', false);

-- [DQL]
select gs_parse_page_bypath('000000000000', 0, 'clog', false);

-- [DQL]
select gs_parse_page_bypath('base/12828/16777', -10, 'heap', false);

-- [DQL]
select * from gs_stat_space(false);

-- [DQL]
select * from gs_parse_page_bypath('base/15833/16768', 0, 'uheap', false);

-- [DQL]
select * from gs_xlogdump_bylastlsn('0/4593570', -1, 'uheap');

-- [DQL]
select * from gs_xlogdump_bylastlsn('0/4593570', 0, 'ubtree');

-- [DDL]
CREATE TABLE test(a int,b int);

-- [DML_INSERT]
INSERT INTO test values(1,1);

-- [DDL]
CREATE PROCEDURE mypro1() as num int;

-- [SESSION]
SET instr_unique_sql_track_type = 'all';

-- [SESSION]
SET track_stmt_stat_level = 'L0,L0';

-- [PLSQL]
CALL mypro1();

-- [SESSION]
SET track_stmt_stat_level = 'off,L0';

-- [SESSION]
SET instr_unique_sql_track_type = 'top';

-- [DQL]
SELECT query,unique_query_id,start_time,finish_time FROM dbe_perf.statement_history;

-- [DQL]
SELECT query FROM dbe_perf.get_full_sql_by_parent_id_and_timestamp(536458473,'2023-06-02 17:40:59.028144+08','2023-06-02 17:40:59.032027+08');

-- [DQL]
SELECT * FROM gs_index_dump_read(0, 'all');

-- [DQL]
SELECT * FROM gs_index_dump_read(1, 'all');


================================================================================
-- 来源: 1110_file_1110.txt
================================================================================

-- [DQL]
select pg_stat_get_role_name(10);

-- [DQL]
select * from pg_stat_get_activity(139881386280704);

-- [DQL]
select * from gs_stat_get_hotkeys_info () order by count , hash_value ;

-- [DQL]
select * from gs_stat_clean_hotkeys ();

-- [DQL]
select * from global_stat_get_hotkeys_info () order by count , hash_value ;

-- [DQL]
select * from global_stat_clean_hotkeys ();

-- [DQL]
SELECT pg_backend_pid ();

-- [DQL]
SELECT pg_stat_get_backend_pid ( 1 );

-- [DQL]
select * from gs_stack ( 139663481165568 );

-- [DQL]
select * from gs_stack ();

-- [DQL]
SELECT * FROM gs_perf_start ( 10 , 100 );

-- [DQL]
SELECT * FROM gs_perf_query () WHERE overhead > 2 AND level < 10 ;

-- [DQL]
SELECT * FROM gs_perf_clean ();

-- [DQL]
select sessionid from pg_stat_activity where usename = 'testuser';

-- [DQL]
select * from gs_session_all_settings(788861) where name = 'work_mem';

-- [DQL]
select * from gs_session_all_settings() where name = 'work_mem';

-- [DQL]
select * from gs_local_wal_preparse_statistics();

-- [DQL]
select * from gs_hot_standby_space_info();

-- [DQL]
SELECT * FROM exrto_file_read_stat();

-- [DQL]
SELECT * FROM gs_exrto_recycle_info();

-- [DQL]
SELECT * FROM gs_stat_get_db_conflict_all(12738);

-- [DQL]
SELECT * FROM gs_redo_stat_info();

-- [DQL]
SELECT * FROM gs_recovery_conflict_waitevent_info();

-- [DQL]
SELECT * FROM gs_display_delay_ddl_info();

-- [DDL]
CREATE TABLE part_tab 1 ( a int, b int ) PARTITION BY RANGE(b) ( PARTITION P1 VALUES LESS THAN(10), PARTITION P2 VALUES LESS THAN(20), PARTITION P3 VALUES LESS THAN(MAXVALUE) );

-- [DDL]
CREATE TABLE subpart_tab 1 ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY RANGE (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES LESS THAN( '2' ), SUBPARTITION p_201901_b VALUES LESS THAN( '3' ) ), PARTITION p_201902 VALUES LESS THAN( '201904' ) ( SUBPARTITION p_201902_a VALUES LESS THAN( '2' ), SUBPARTITION p_201902_b VALUES LESS THAN( '3' ) ) );

-- [DDL]
CREATE INDEX index_part_tab1 ON part_tab1(b) LOCAL ( PARTITION b_index1, PARTITION b_index2, PARTITION b_index 3 );

-- [DDL]
CREATE INDEX idx_user_no ON subpart_tab1(user_no) LOCAL;

-- [DML_INSERT]
INSERT INTO part_tab1 VALUES(1, 1);

-- [DML_INSERT]
INSERT INTO part_tab1 VALUES(1, 11);

-- [DML_INSERT]
INSERT INTO part_tab1 VALUES(1, 21);

-- [DML_UPDATE]
UPDATE part_tab1 SET a = 2 WHERE b = 1;

-- [DML_UPDATE]
UPDATE part_tab1 SET a = 3 WHERE b = 11;

-- [DML_UPDATE]
UPDATE /*+ indexscan(part_tab1) */ part_tab1 SET a = 4 WHERE b = 21;

-- [DML_DELETE]
DELETE FROM part_tab1;

-- [MAINTENANCE]
ANALYZE part_tab1;

-- [MAINTENANCE]
VACUUM part_tab1;

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201902', '1', '1', 1);

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201902', '2', '2', 1);

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201903', '1', '3', 1);

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201903', '2', '4', 1);

-- [DML_UPDATE]
UPDATE subpart_tab1 SET sales_amt = 2 WHERE user_no='1';

-- [DML_UPDATE]
UPDATE subpart_tab1 SET sales_amt = 3 WHERE user_no='2';

-- [DML_UPDATE]
UPDATE subpart_tab1 SET sales_amt = 4 WHERE user_no='3';

-- [DML_UPDATE]
UPDATE /*+ indexscan(subpart_tab1) */ subpart_tab1 SET sales_amt = 5 WHERE user_no='4';

-- [DML_DELETE]
DELETE FROM subpart_tab1;

-- [MAINTENANCE]
ANALYZE subpart_tab1;

-- [MAINTENANCE]
VACUUM subpart_tab1;

-- [DQL]
SELECT * FROM gs_stat_all_partitions;

-- [DQL]
SELECT * FROM gs_statio_all_partitions;

-- [DQL]
SELECT * FROM gs_stat_get_partition_stats(16952);

-- [TCL]
BEGIN;

-- [DML_INSERT]
INSERT INTO part_tab1 VALUES(1, 1);

-- [DML_INSERT]
INSERT INTO part_tab1 VALUES(1, 11);

-- [DML_INSERT]
INSERT INTO part_tab1 VALUES(1, 21);

-- [DML_UPDATE]
UPDATE part_tab1 SET a = 2 WHERE b = 1;

-- [DML_UPDATE]
UPDATE part_tab1 SET a = 3 WHERE b = 11;

-- [DML_UPDATE]
UPDATE /*+ indexscan(part_tab1) */ part_tab1 SET a = 4 WHERE b = 21;

-- [DML_DELETE]
DELETE FROM part_tab1;

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201902', '1', '1', 1);

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201902', '2', '2', 1);

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201903', '1', '3', 1);

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201903', '2', '4', 1);

-- [DML_UPDATE]
UPDATE subpart_tab1 SET sales_amt = 2 WHERE user_no='1';

-- [DML_UPDATE]
UPDATE subpart_tab1 SET sales_amt = 3 WHERE user_no='2';

-- [DML_UPDATE]
UPDATE subpart_tab1 SET sales_amt = 4 WHERE user_no='3';

-- [DML_UPDATE]
UPDATE /*+ indexscan(subpart_tab1) */ subpart_tab1 SET sales_amt = 5 WHERE user_no='4';

-- [DML_DELETE]
DELETE FROM subpart_tab1;

-- [DQL]
SELECT * FROM gs_stat_xact_all_partitions;

-- [DQL]
SELECT * FROM gs_stat_get_xact_partition_stats(16952);


================================================================================
-- 来源: 1112_HashFunc.txt
================================================================================

-- [DQL]
select bucketabstime ( '2011-10-01 10:10:10.112' , 1 );

-- [DQL]
select bucketbool ( true , 1 );

-- [DQL]
select bucketbool ( false , 1 );

-- [DQL]
select bucketbpchar ( 'test' , 1 );

-- [DQL]
select bucketbytea ( 'test' , 1 );

-- [DQL]
select bucketcash ( 10 :: money , 1 );

-- [DQL]
select getbucket ( 10 , 'H' );

-- [DQL]
select getbucket ( 11 , 'H' );

-- [DQL]
select getbucket ( 11 , 'R' );

-- [DQL]
select getbucket ( 12 , 'R' );

-- [DQL]
select ora_hash ( 123 );

-- [DQL]
select ora_hash ( '123' );

-- [DQL]
select ora_hash ( 'sample' );

-- [DQL]
select ora_hash ( to_date ( '2012-1-2' , 'yyyy-mm-dd' ));

-- [DQL]
select ora_hash ( 123 , 234 );

-- [DQL]
select ora_hash ( '123' , 234 );

-- [DQL]
select ora_hash ( 'sample' , 234 );

-- [DQL]
select ora_hash ( to_date ( '2012-1-2' , 'yyyy-mm-dd' ), 234 );

-- [DQL]
select hash_array ( ARRAY [[ 1 , 2 , 3 ],[ 1 , 2 , 3 ]]);

-- [DQL]
select hash_numeric ( 30 );

-- [DQL]
select hash_range ( numrange ( 1 . 1 , 2 . 2 ));

-- [DQL]
select hashbpchar ( 'hello' );

-- [DQL]
select hashbpchar ( 'hello' );

-- [DQL]
select hashchar ( 'true' );

-- [DDL]
CREATE TYPE b1 AS ENUM ( 'good' , 'bad' , 'ugly' );

-- [PLSQL]
call hashenum ( 'good' :: b1 );

-- [DQL]
select hashfloat4 ( 12 . 1234 );

-- [DQL]
select hashfloat8 ( 123456 . 1234 );

-- [DQL]
select hashinet ( '127.0.0.1' :: inet );

-- [DQL]
select hashint1 ( 20 );

-- [DQL]
select hashint2(20000);


================================================================================
-- 来源: 1121_hotkey.txt
================================================================================

-- [DQL]
select * from gs_stat_get_hotkeys_info () order by count , hash_value ;

-- [DQL]
select * from gs_stat_clean_hotkeys ();


================================================================================
-- 来源: 1122_Global SysCache.txt
================================================================================

-- [DQL]
select * from gs_gsc_catalog_detail(16574, 1260);

-- [DQL]
select * from gs_gsc_clean();

-- [DQL]
select * from gs_gsc_dbstat_info();


================================================================================
-- 来源: 1123_file_1123.txt
================================================================================

-- [DQL]
select * from gs_verify_data_file();

-- [DQL]
select * from gs_verify_data_file(true);

-- [DQL]
select * from gs_repair_file(16554,'base/16552/24745',360);

-- [DQL]
select * from local_bad_block_info();

-- [DQL]
select * from local_clear_bad_block_info();

-- [DQL]
select * from gs_verify_and_tryrepair_page('base/16552/24745',0,false,false);

-- [DQL]
select * from gs_repair_page('base/16552/24745',0,false,60);

-- [DQL]
select gs_edit_page_bypath('base/15808/25075',0,16,'0x1FFF', 2, false, 'page');

-- [DQL]
select gs_edit_page_bypath('base/15808/25075', 0,16,'@1231!', 8, false, 'page');

-- [DQL]
select gs_edit_page_bypath('/pg_log_dir/dump/1663_15808_25075_0.editpage', 0,16,'0x1FFF', 2, true, 'page');

-- [DQL]
select * from gs_repair_page_bypath('pg_log/dump/1663_15991_16767_0.editpage', 0, 'base/15991/16767', 0, 'page');

-- [DQL]
select * from gs_repair_page_bypath('standby', 0, 'base/15990/16768', 0, 'page');

-- [DQL]
select * from gs_repair_page_bypath('init_block', 0, 'base/15990/16768', 0, 'page');

-- [DQL]
select * from gs_repair_undo_byzone(4);

-- [DQL]
select * from gs_repair_undo_byzone(78);

-- [DQL]
select * from gs_repair_undo_byzone(0);

-- [DQL]
select * from gs_verify_urq(16387, 0, 1, 'free queue');

-- [DQL]
select * from gs_verify_urq(16387, 0, 1, 'empty queue');

-- [DQL]
SELECT * FROM gs_urq_dump_stat(16387, 0);

-- [DQL]
SELECT gs_urq_dump_stat(17260,0);

-- [DQL]
select * from gs_repair_urq(16387, 0);

-- [DQL]
select * from gs_get_standby_bad_block_info();


================================================================================
-- 来源: 1124_XML.txt
================================================================================

-- [DQL]
SELECT XMLPARSE ( DOCUMENT '<?xml version="1.0"?><book><title>Manual</title><chapter>...</chapter></book>' );

-- [DQL]
SELECT XMLPARSE ( CONTENT 'abc<foo>bar</foo><bar>foo</bar>' );

-- [DQL]
SELECT XMLPARSE ( CONTENT 'abc<foo>bar</foo' wellformed );

-- [SESSION]
set xmloption=content;

-- [DQL]
select XMLCONCAT(('<?xml version="1.0" encoding="GB2312" standalone="no"?><bar>foo</bar>'),('<?xml version="1.0" encoding="GB2312" standalone="no" ?><bar>foo</bar>')) ;

-- [DQL]
select XMLCONCAT('abc>');

-- [SESSION]
set a_format_version='10c';

-- [SESSION]
set a_format_dev_version=s2;

-- [SESSION]
set xmloption=content;

-- [DQL]
select XMLCONCAT(('<?xml version="1.0" encoding="GB2312" standalone="no"?><bar>foo</bar>'),('<?xml version="1.0" encoding="GB2312" standalone="no" ?><bar>foo</bar>')) ;

-- [DQL]
select XMLCONCAT('abc>');

-- [DDL]
CREATE TABLE xmltest ( id int , data xml );

-- [DML_INSERT]
INSERT INTO xmltest VALUES ( 1 , '<value>one</value>' );

-- [DML_INSERT]
INSERT INTO xmltest VALUES ( 2 , '<value>two</value>' );

-- [DQL]
SELECT xmlagg ( data ) FROM xmltest ;

-- [SESSION]
set xmloption = document ;

-- [DQL]
SELECT xmlagg ( data ) FROM xmltest ;

-- [DML_DELETE]
DELETE FROM XMLTEST ;

-- [DML_INSERT]
INSERT INTO xmltest VALUES ( 1 , '<?xml version="1.0" encoding="GBK"?><value>one</value>' );

-- [DML_INSERT]
INSERT INTO xmltest VALUES ( 2 , '<?xml version="1.0" encoding="GBK"?><value>two</value>' );

-- [DQL]
SELECT xmlagg ( data ) FROM xmltest ;

-- [DQL]
SELECT xmlagg ( data order by id desc ) FROM xmltest ;

-- [DQL]
SELECT xmlelement ( name foo );

-- [DQL]
SELECT xmlelement ( "entityescaping<>" , 'a$><&"b' );

-- [DQL]
SELECT xmlelement ( entityescaping "entityescaping<>" , 'a$><&"b' );

-- [DQL]
SELECT xmlelement ( noentityescaping "entityescaping<>" , 'a$><&"b' );

-- [DQL]
SELECT xmlelement(" entityescaping <> ", '<abc/>' b);

-- [DQL]
SELECT xmlelement(" entityescaping <> ", '<abc/>' as b);

-- [DQL]
SELECT xmlelement(" entityescaping <> ", xml('<abc/>') b);

-- [DQL]
SELECT xmlelement(" entityescaping <> ", xml('<abc/>') as b);

-- [DQL]
SELECT xmlelement(" entityescaping <> ", xmlattributes('entityescaping<>' " entityescaping <> "));

-- [DQL]
SELECT xmlelement(name " entityescaping <> ", xmlattributes(entityescaping 'entityescaping<>' " entityescaping <> "));

-- [DQL]
SELECT xmlelement(" entityescaping <> ", xmlattributes(noentityescaping 'entityescaping<>' " entityescaping <> "));

-- [SESSION]
set a_format_version = '10c' ;

-- [SESSION]
set a_format_dev_version = 's4' ;

-- [PLSQL]
declare xmldata xml ;

-- [DQL]
select getclobval ( xmlparse ( document '<a>123</a>' ));

-- [SESSION]
set a_format_version='10c';

-- [SESSION]
set a_format_dev_version='s4';

-- [PLSQL]
declare xmldata xml;

-- [DQL]
select getstringval(xmlparse(document '<a>123<b>456</b></a>'));

-- [DQL]
SELECT xmlsequence(xml('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>'));


================================================================================
-- 来源: 1125_XMLTYPE.txt
================================================================================

-- [DQL]
SELECT createxml ( '<a>123</a>' );

-- [DQL]
SELECT xmltype . createxml ( '<a>123</a>' );

-- [DQL]
select xmltype ( '<a>123<b>456</b></a>' ). extract ( '/a/b' ). getstringval ();

-- [DQL]
select getstringval ( extractxml ( xmltype ( '<a>123<b>456</b></a>' ), '/a/b' ));

-- [PLSQL]
declare a xmltype ;

-- [PLSQL]
declare xmltype_clob clob ;

-- [PLSQL]
declare xmltype_blob blob ;

-- [DQL]
SELECT getblobval ( xmltype ( '<asd/>' ), 7 );

-- [DQL]
select xmltype ( '<asd/>' ). getblobVal ( 7 );

-- [DQL]
SELECT getclobval ( xmltype ( '<a>123</a>' ));

-- [DQL]
SELECT xmltype ( '<a>123</a>' ). getclobval ();

-- [DQL]
SELECT getnumberval ( xmltype ( '<a>123</a>' ). extract ( '/a/text()' ));

-- [DQL]
SELECT xmltype ( '<a>123</a>' ). extract ( '/a/text()' ). getnumberval ();

-- [DQL]
SELECT isfragment ( xmltype ( '<a>123</a>' ));

-- [DQL]
SELECT xmltype ( '<a>123</a>' ). isfragment ();

-- [DQL]
SELECT xmltype ( '<a>123</a>' );

-- [PLSQL]
declare xmltype_clob clob ;

-- [PLSQL]
declare xmltype_blob blob ;

-- [DQL]
select getstringval('<a>123<b>456</b></a>');

-- [DQL]
select xmltype('<a>123<b>456</b></a>').getstringval();

-- [DQL]
select getrootelement('<a>123<b>456</b></a>');

-- [DQL]
select xmltype('<a>123<b>456</b></a>').getrootelement();

-- [DQL]
select getnamespace('<c:a xmlns:c="asd">123<d:b xmlns:d="qwe">456</d:b></c:a>');

-- [DQL]
select xmltype('<c:a xmlns:c="asd">123<d:b xmlns:d="qwe">456</d:b></c:a>').getnamespace();

-- [DQL]
select existsnode('<a>123<b>456</b></a>','/a/b');

-- [DQL]
select xmltype('<a>123<b>456</b></a>').existsnode('/a/b');

-- [DQL]
select existsnode('<a:b xmlns:a="asd">123<c>456</c></a:b>','/a:b/c','xmlns:a="asd"');

-- [DQL]
select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').existsnode('/a:b/c','xmlns:a="asd"');

-- [DQL]
select extractxml('<a>123<b>456</b></a>','/a/b');

-- [DQL]
select xmltype('<a>123<b>456</b></a>').extract('/a/b');

-- [DQL]
select xmltype('<a>123<b>456</b></a>').extractxml('/a/b');

-- [DQL]
select extractxml('<a:b xmlns:a="asd">123<c>456</c></a:b>','/a:b','xmlns:a="asd"');

-- [DQL]
select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').extract('/a:b','xmlns:a="asd"');

-- [DQL]
select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').extractxml('/a:b','xmlns:a="asd"');

-- [DQL]
SELECT xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>'));

-- [DQL]
SELECT unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//title/text()'))) AS title , unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//author/text()'))) AS author;

-- [DQL]
SELECT array_to_json(array_agg(row_to_json(t))) FROM ( SELECT unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//title/text()'))) AS title , unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinnger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//author/text()'))) AS author ) t;


================================================================================
-- 来源: 1126_file_1126.txt
================================================================================

-- [DQL]
select * from cross_test ;

-- [DDL]
create extension tablefunc ;

-- [DQL]
select * from crosstab ( 'select group_, id, var from cross_test order by 1, 2;

-- [DDL]
create extension tablefunc ;

-- [DQL]
select * from crosstab2 ( 'select group_, id, var from cross_test order by 1, 2;

-- [DDL]
create extension tablefunc ;

-- [DQL]
select * from crosstab ( 'select group_, id, var from cross_test order by 1, 2;


================================================================================
-- 来源: 1127_file_1127.txt
================================================================================

-- [DQL]
select uuid ();

-- [DQL]
SELECT uuid_short ();


================================================================================
-- 来源: 1128_SQL.txt
================================================================================

-- [DQL]
select gs_add_workload_rule ( 'sqlid' , 'rule for one query' , '{}' , now (), NULL , 20 , '{id=32413214}' );

-- [DDL]
create database db1 ;

-- [DDL]
create database db2 ;

-- [DQL]
select gs_add_workload_rule ( 'select' , 'rule for select' , '{db1, db2}' , NULL , NULL , 100 , '{tb1, tb2}' );

-- [DQL]
select gs_add_workload_rule ( 'resource' , 'rule for resource' , '{}' , NULL , NULL , 20 , '{cpu-80}' );

-- [DDL]
create database db1 ;

-- [DQL]
select gs_update_workload_rule ( 2 , 'rule for select 2' , '{db1}' , now (), NULL , 50 , '{tb1}' );

-- [DQL]
select gs_delete_workload_rule ( 3 );

-- [DQL]
select * from gs_get_workload_rule_stat ( 1 );

-- [DQL]
select * from gs_get_workload_rule_stat ( - 1 );


================================================================================
-- 来源: 1131_file_1131.txt
================================================================================

-- [DQL]
SELECT 2 BETWEEN 1 AND 3 AS RESULT ;

-- [DQL]
SELECT 2 >= 1 AND 2 <= 3 AS RESULT ;

-- [DQL]
SELECT 2 NOT BETWEEN 1 AND 3 AS RESULT ;

-- [DQL]
SELECT 2 < 1 OR 2 > 3 AS RESULT ;

-- [DQL]
SELECT 2 + 2 IS NULL AS RESULT ;

-- [DQL]
SELECT 2 + 2 IS NOT NULL AS RESULT ;

-- [DQL]
SELECT 2 + 2 ISNULL AS RESULT ;

-- [DQL]
SELECT 2 + 2 NOTNULL AS RESULT ;

-- [DQL]
SELECT 2 + 2 IS DISTINCT FROM NULL AS RESULT ;

-- [DQL]
SELECT 2 + 2 IS NOT DISTINCT FROM NULL AS RESULT ;

-- [DQL]
select 1 <=> 1 AS RESULT ;

-- [DQL]
select NULL <=> 1 AS RESULT ;

-- [DQL]
select NULL <=> NULL AS RESULT ;


================================================================================
-- 来源: 1132_file_1132.txt
================================================================================

-- [DDL]
CREATE TABLE tpcds . case_when_t1 ( CW_COL1 INT ) DISTRIBUTE BY HASH ( CW_COL1 );

-- [DML_INSERT]
INSERT INTO tpcds . case_when_t1 VALUES ( 1 ), ( 2 ), ( 3 );

-- [DQL]
SELECT * FROM tpcds . case_when_t1 ;

-- [DQL]
SELECT CW_COL1 , CASE WHEN CW_COL1 = 1 THEN 'one' WHEN CW_COL1 = 2 THEN 'two' ELSE 'other' END FROM tpcds . case_when_t1 ORDER BY 1 ;

-- [DDL]
DROP TABLE tpcds . case_when_t1 ;

-- [DQL]
SELECT DECODE ( 'A' , 'A' , 1 , 'B' , 2 , 0 );

-- [DDL]
CREATE TABLE tpcds . c_tabl ( description varchar ( 10 ), short_description varchar ( 10 ), last_value varchar ( 10 )) DISTRIBUTE BY HASH ( last_value );

-- [DML_INSERT]
INSERT INTO tpcds . c_tabl VALUES ( 'abc' , 'efg' , '123' );

-- [DML_INSERT]
INSERT INTO tpcds . c_tabl VALUES ( NULL , 'efg' , '123' );

-- [DML_INSERT]
INSERT INTO tpcds . c_tabl VALUES ( NULL , NULL , '123' );

-- [DQL]
SELECT description , short_description , last_value , COALESCE ( description , short_description , last_value ) FROM tpcds . c_tabl ORDER BY 1 , 2 , 3 , 4 ;

-- [DDL]
DROP TABLE tpcds . c_tabl ;

-- [DQL]
SELECT COALESCE ( NULL , 'Hello World' );

-- [DDL]
CREATE TABLE tpcds . null_if_t1 ( NI_VALUE1 VARCHAR ( 10 ), NI_VALUE2 VARCHAR ( 10 ) ) DISTRIBUTE BY HASH ( NI_VALUE1 );

-- [DML_INSERT]
INSERT INTO tpcds . null_if_t1 VALUES ( 'abc' , 'abc' );

-- [DML_INSERT]
INSERT INTO tpcds . null_if_t1 VALUES ( 'abc' , 'efg' );

-- [DQL]
SELECT NI_VALUE1 , NI_VALUE2 , NULLIF ( NI_VALUE1 , NI_VALUE2 ) FROM tpcds . null_if_t1 ORDER BY 1 , 2 , 3 ;

-- [DDL]
DROP TABLE tpcds . null_if_t1 ;

-- [DQL]
SELECT NULLIF ( 'Hello' , 'Hello World' );

-- [DQL]
SELECT greatest ( 9000 , 155555 , 2 . 01 );

-- [DQL]
SELECT least ( 9000 , 2 );

-- [DQL]
SELECT nvl ( null , 1 );

-- [DQL]
SELECT nvl ( 'Hello World' , 1 );


================================================================================
-- 来源: 1133_file_1133.txt
================================================================================

-- [DQL]
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE EXISTS ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom = store_returns . sr_reason_sk and sr_customer_sk < 10 );

-- [DQL]
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk IN ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom < 10 );

-- [DQL]
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk < ANY ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom < 10 );

-- [DQL]
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk < all ( SELECT d_dom FROMOM tpcds . date_dim WHERE d_dom < 10 );


================================================================================
-- 来源: 1134_file_1134.txt
================================================================================

-- [DQL]
SELECT 8000 + 500 IN ( 10000 , 9000 ) AS RESULT ;

-- [DQL]
SELECT 8000 + 500 NOT IN ( 10000 , 9000 ) AS RESULT ;

-- [DQL]
SELECT 8000 + 500 < SOME ( array [ 10000 , 9000 ]) AS RESULT ;

-- [DQL]
SELECT 8000 + 500 < ANY ( array [ 10000 , 9000 ]) AS RESULT ;

-- [DQL]
SELECT 8000 + 500 < ALL ( array [ 10000 , 9000 ]) AS RESULT ;


================================================================================
-- 来源: 1135_file_1135.txt
================================================================================

-- [DQL]
SELECT ROW ( 1 , 2 , NULL ) < ROW ( 1 , 3 , 0 ) AS RESULT ;

-- [DQL]
select ( 4 , 5 , 6 ) > ( 3 , 2 , 1 ) as result ;

-- [DQL]
select ( 4 , 1 , 1 ) > ( 3 , 2 , 1 ) as result ;

-- [DQL]
select ( 'test' , 'data' ) > ( 'data' , 'data' ) as result ;

-- [DQL]
select ( 4 , 1 , 1 ) > ( 3 , 2 , null ) as result ;

-- [DQL]
select ( null , 1 , 1 ) > ( 3 , 2 , 1 ) as result ;

-- [DQL]
select ( null , 5 , 6 ) > ( null , 5 , 6 ) as result ;

-- [DQL]
select ( 4 , 5 , 6 ) > ( 4 , 5 , 6 ) as result ;

-- [DQL]
select ( 2 , 2 , 5 ) >= ( 2 , 2 , 3 ) as result ;

-- [DQL]
select ( 2 , 2 , 1 ) <= ( 2 , 2 , 3 ) as result ;

-- [DQL]
select ( 1 , 2 , 3 ) = ( 1 , 2 , 3 ) as result ;

-- [DQL]
select ( 1 , 2 , 3 ) <> ( 2 , 2 , 3 ) as result ;

-- [DQL]
select ( 2 , 2 , 3 ) <> ( 2 , 2 , null ) as result ;

-- [DQL]
select ( null , 5 , 6 ) <> ( null , 5 , 6 ) as result ;


================================================================================
-- 来源: 1136_file_1136.txt
================================================================================

-- [DQL]
SELECT DATE_ADD ( '2018-05-01' , INTERVAL 1 DAY );

-- [DQL]
SELECT DATE_SUB ( '2018-05-01' , INTERVAL 1 YEAR );

-- [DQL]
SELECT DATE '2023-01-10' - INTERVAL 1 DAY ;

-- [DQL]
SELECT DATE '2023-01-10' + INTERVAL 1 MONTH ;


================================================================================
-- 来源: 1137_file_1137.txt
================================================================================

-- [DDL]
CREATE TABLE Students ( name varchar ( 20 ), id int ) with ( STORAGE_TYPE = USTORE );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Jack' , 35 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Leon' , 15 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'James' , 24 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Taker' , 81 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Mary' , 25 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Rose' , 64 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Perl' , 18 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Under' , 57 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Angel' , 101 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Frank' , 20 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Charlie' , 40 );

-- [DQL]
SELECT * FROM Students WHERE rownum <= 10 ;

-- [DQL]
SELECT * FROM Students WHERE rownum < 5 order by 1 ;

-- [DQL]
SELECT rownum , * FROM ( SELECT * FROM Students order by 1 ) WHERE rownum <= 2 ;

-- [DQL]
SELECT * FROM Students WHERE rownum > 1 ;

-- [DQL]
SELECT * FROM Students ;

-- [DML_UPDATE]
update Students set id = id + 5 WHERE rownum < 4 ;

-- [DQL]
SELECT * FROM Students ;

-- [DDL]
DROP TABLE Students ;

-- [DDL]
CREATE TABLE test ( a int , b int );

-- [DML_INSERT]
INSERT INTO test SELECT generate_series , generate_series FROM generate_series ( 1 , 10 );

-- [EXPLAIN]
EXPLAIN SELECT a , rownum FROM test group by a , rownum having rownum < 5 ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM ( SELECT * FROM test WHERE rownum < 5 ) WHERE b < 5 ;

-- [DDL]
DROP TABLE test ;

-- [DDL]
CREATE TABLE test ( a int , b int );

-- [DML_INSERT]
INSERT INTO test VALUES ( generate_series ( 1 , 10 ), generate_series ( 1 , 10 ));

-- [DQL]
SELECT rownum , * FROM test ;

-- [DQL]
SELECT rownum , * FROM test ;


================================================================================
-- 来源: 1139_file_1139.txt
================================================================================

-- [DQL]
SELECT text 'Origin' AS "label" , point '(0,0)' AS "value" ;


================================================================================
-- 来源: 1140_file_1140.txt
================================================================================

-- [DQL]
SELECT 40 ! AS "40 factorial" ;

-- [DQL]
SELECT CAST ( 40 AS bigint ) ! AS "40 factorial" ;

-- [DQL]
SELECT text 'abc' || 'def' AS "text and unknown" ;

-- [DQL]
SELECT 'abc' || 'def' AS "unspecified" ;

-- [DQL]
SELECT @ '-4.5' AS "abs" ;

-- [DQL]
SELECT array [ 1 , 2 ] <@ '{1,2,3}' as "is subset" ;


================================================================================
-- 来源: 1141_file_1141.txt
================================================================================

-- [DQL]
SELECT round ( 4 , 4 );

-- [DQL]
SELECT round ( CAST ( 4 AS numeric ), 4 );

-- [DQL]
SELECT round ( 4 . 0 , 4 );

-- [DQL]
SELECT substr ( '1234' , 3 );

-- [DQL]
SELECT substr ( varchar '1234' , 3 );

-- [DQL]
SELECT substr ( CAST ( varchar '1234' AS text ), 3 );

-- [DQL]
SELECT substr ( 1234 , 3 );

-- [DQL]
SELECT substr ( CAST ( 1234 AS text ), 3 );


================================================================================
-- 来源: 1142_file_1142.txt
================================================================================

-- [DDL]
CREATE TABLE tpcds . value_storage_t1 ( VS_COL1 CHARACTER ( 20 ) ) DISTRIBUTE BY HASH ( VS_COL1 );

-- [DML_INSERT]
INSERT INTO tpcds . value_storage_t1 VALUES ( 'abcdef' );

-- [DQL]
SELECT VS_COL1 , octet_length ( VS_COL1 ) FROM tpcds . value_storage_t1 ;

-- [DDL]
DROP TABLE tpcds . value_storage_t1 ;


================================================================================
-- 来源: 1143_UNIONCASE.txt
================================================================================

-- [DQL]
SELECT text 'a' AS "text" UNION SELECT 'b' ;

-- [DQL]
SELECT 1 . 2 AS "numeric" UNION SELECT 1 ;

-- [DQL]
SELECT 1 AS "real" UNION SELECT CAST ( '2.2' AS REAL );

-- [DDL]
CREATE DATABASE oracle_1 dbcompatibility = 'ORA';

--在TD模式下，创建TD兼容模式的数据库td_1。
-- [DDL]
CREATE DATABASE td_1 dbcompatibility = 'TD';

--删除Oracle和TD模式的数据库。
-- [DDL]
DROP DATABASE oracle_1;

-- [DDL]
DROP DATABASE td_1;

-- [DDL]
CREATE DATABASE ora_1 dbcompatibility = 'A';

--删除ORA模式的数据库。
-- [DDL]
DROP DATABASE ora_1;


================================================================================
-- 来源: 1147_file_1147.txt
================================================================================

-- [DQL]
SELECT d_dow || '-' || d_dom || '-' || d_fy_week_seq AS identify_serials FROM tpcds . date_dim WHERE d_fy_week_seq = 1 ;


================================================================================
-- 来源: 1148_file_1148.txt
================================================================================

-- [DQL]
SELECT 'a fat cat sat on a mat and ate a fat rat' :: tsvector @@ 'cat & rat' :: tsquery AS RESULT ;

-- [DQL]
SELECT 'fat & cow' :: tsquery @@ 'a fat cat sat on a mat and ate a fat rat' :: tsvector AS RESULT ;

-- [DQL]
SELECT to_tsvector ( 'fat cats ate fat rats' ) @@ to_tsquery ( 'fat & rat' ) AS RESULT ;

-- [DQL]
SELECT 'fat cats ate fat rats' :: tsvector @@ to_tsquery ( 'fat & rat' ) AS RESULT ;


================================================================================
-- 来源: 1151_file_1151.txt
================================================================================

-- [DDL]
DROP SCHEMA IF EXISTS tsearch CASCADE ;

-- [DDL]
CREATE SCHEMA tsearch ;

-- [DDL]
CREATE TABLE tsearch . pgweb ( id int , body text , title text , last_mod_date date ) with ( storage_type = ASTORE );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 1 , 'China, officially the People''s Republic of China (PRC), located in Asia, is the world''s most populous state.' , 'China' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 2 , 'America is a rock band, formed in England in 1970 by multi-instrumentalists Dewey Bunnell, Dan Peek, and Gerry Beckley.' , 'America' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 3 , 'England is a country that is part of the United Kingdom. It shares land borders with Scotland to the north and Wales to the west.' , 'England' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 4 , 'Australia, officially the Commonwealth of Australia, is a country comprising the mainland of the Australian continent, the island of Tasmania, and numerous smaller islands.' , 'Australia' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 6 , 'Japan is an island country in East Asia.' , 'Japan' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 7 , 'Germany, officially the Federal Republic of Germany, is a sovereign state and federal parliamentary republic in central-western Europe.' , 'Germany' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 8 , 'France, is a sovereign state comprising territory in western Europe and several overseas regions and territories.' , 'France' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 9 , 'Italy officially the Italian Republic, is a unitary parliamentary republic in Europe.' , 'Italy' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 10 , 'India, officially the Republic of India, is a country in South Asia.' , 'India' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 11 , 'Brazil, officially the Federative Republic of Brazil, is the largest country in both South America and Latin America.' , 'Brazil' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 12 , 'Canada is a country in the northern half of North America.' , 'Canada' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 13 , 'Mexico, officially the United Mexican States, is a federal republic in the southern part of North America.' , 'Mexico' , '2010-1-1' );

-- [DQL]
SELECT id , body , title FROM tsearch . pgweb WHERE to_tsvector ( 'english' , body ) @@ to_tsquery ( 'english' , 'america' );

-- [SESSION]
SHOW default_text_search_config ;

-- [DQL]
SELECT id , body , title FROM tsearch . pgweb WHERE to_tsvector ( body ) @@ to_tsquery ( 'america' );

-- [DQL]
SELECT title FROM tsearch . pgweb WHERE to_tsvector ( title || ' ' || body ) @@ to_tsquery ( 'north & america' ) ORDER BY last_mod_date DESC LIMIT 10 ;


================================================================================
-- 来源: 1152_file_1152.txt
================================================================================

-- [DDL]
CREATE INDEX pgweb_idx_1 ON tsearch . pgweb USING gin ( to_tsvector ( 'english' , body ));

-- [DDL]
CREATE INDEX pgweb_idx_2 ON tsearch . pgweb USING gin ( to_tsvector ( 'ngram' , body ));

-- [DDL]
CREATE INDEX pgweb_idx_3 ON tsearch . pgweb USING gin ( to_tsvector ( 'english' , title || ' ' || body ));

-- [DDL]
ALTER TABLE tsearch . pgweb ADD COLUMN textsearchable_index_col tsvector ;

-- [DML_UPDATE]
UPDATE tsearch . pgweb SET textsearchable_index_col = to_tsvector ( 'english' , coalesce ( title , '' ) || ' ' || coalesce ( body , '' ));

-- [DDL]
CREATE INDEX textsearch_idx_4 ON tsearch . pgweb USING gin ( textsearchable_index_col );

-- [DQL]
SELECT title FROM tsearch . pgweb WHERE textsearchable_index_col @@ to_tsquery ( 'north & america' ) ORDER BY last_mod_date DESC LIMIT 10 ;


================================================================================
-- 来源: 1153_file_1153.txt
================================================================================

-- [DDL]
create table table1 ( c_int int , c_bigint bigint , c_varchar varchar , c_text text ) with ( orientation = row , storage_type = ASTORE );

-- [DDL]
create text search configuration ts_conf_1 ( parser = POUND );

-- [DDL]
create text search configuration ts_conf_2 ( parser = POUND ) with ( split_flag = '%' );

-- [SESSION]
set default_text_search_config = 'ts_conf_1' ;

-- [DDL]
create index idx1 on table1 using gin ( to_tsvector ( c_text ));

-- [SESSION]
set default_text_search_config = 'ts_conf_2' ;

-- [DDL]
create index idx2 on tscp_u_m_005_tbl using gin ( to_tsvector ( c_text ));

-- [DQL]
select c_varchar , to_tsvector ( c_varchar ) from table1 where to_tsvector ( c_text ) @@ plainto_tsquery ( '￥#@……&**' ) and to_tsvector ( c_text ) @@ plainto_tsquery ( '某公司 ' ) and c_varchar is not null order by 1 desc limit 3 ;


================================================================================
-- 来源: 1155_file_1155.txt
================================================================================

-- [DQL]
SELECT to_tsvector ( 'english' , 'a fat cat sat on a mat - it ate a fat rats' );

-- [DDL]
CREATE TABLE tsearch . tt ( id int , title text , keyword text , abstract text , body text , ti tsvector );

-- [DML_INSERT]
INSERT INTO tsearch . tt ( id , title , keyword , abstract , body ) VALUES ( 1 , 'China' , 'Beijing' , 'China' , 'China, officially the People''s Republic of China (PRC), located in Asia, is the world''s most populous state.' );

-- [DML_UPDATE]
UPDATE tsearch . tt SET ti = setweight ( to_tsvector ( coalesce ( title , '' )), 'A' ) || setweight ( to_tsvector ( coalesce ( keyword , '' )), 'B' ) || setweight ( to_tsvector ( coalesce ( abstract , '' )), 'C' ) || setweight ( to_tsvector ( coalesce ( body , '' )), 'D' );

-- [DDL]
DROP TABLE tsearch . tt ;


================================================================================
-- 来源: 1156_file_1156.txt
================================================================================

-- [DQL]
SELECT to_tsquery ( 'english' , 'The & Fat & Rats' );

-- [DQL]
SELECT to_tsquery ( 'english' , 'Fat | Rats:AB' );

-- [DQL]
SELECT to_tsquery ( 'supern:*A & star:A*B' );

-- [DQL]
SELECT plainto_tsquery ( 'english' , 'The Fat Rats' );

-- [DQL]
SELECT plainto_tsquery ( 'english' , 'The Fat & Rats:C' );


================================================================================
-- 来源: 1157_file_1157.txt
================================================================================

-- [DQL]
SELECT id , title , ts_rank_cd ( to_tsvector ( body ), query ) AS rank FROM tsearch . pgweb , to_tsquery ( 'america' ) query WHERE query @@ to_tsvector ( body ) ORDER BY rank DESC LIMIT 10 ;

-- [DQL]
SELECT id , title , ts_rank_cd ( to_tsvector ( body ), query , 32 /* rank/(rank+1) */ ) AS rank FROM tsearch . pgweb , to_tsquery ( 'america' ) query WHERE query @@ to_tsvector ( body ) ORDER BY rank DESC LIMIT 10 ;

-- [DDL]
CREATE TABLE tsearch . ts_ngram ( id int , body text );

-- [DML_INSERT]
INSERT INTO tsearch . ts_ngram VALUES ( 1 , '中文' );

-- [DML_INSERT]
INSERT INTO tsearch . ts_ngram VALUES ( 2 , '中文检索' );

-- [DML_INSERT]
INSERT INTO tsearch . ts_ngram VALUES ( 3 , '检索中文' );

-- [DQL]
SELECT id , body , ts_rank_cd ( to_tsvector ( 'ngram' , body ), query ) AS rank FROM tsearch . ts_ngram , to_tsquery ( '中文' ) query WHERE query @@ to_tsvector ( body );

-- [DQL]
SELECT id , body , ts_rank_cd ( to_tsvector ( 'ngram' , body ), query ) AS rank FROM tsearch . ts_ngram , to_tsquery ( '中文' ) query WHERE query @@ to_tsvector ( 'ngram' , body );


================================================================================
-- 来源: 1158_file_1158.txt
================================================================================

-- [DQL]
SELECT ts_headline ( 'english' , 'The most common type of search is to find all documents containing given query terms and return them in order of their similarity to the query.' , to_tsquery ( 'english' , 'query & similarity' ));

-- [DQL]
SELECT ts_headline ( 'english' , 'The most common type of search is to find all documents containing given query terms and return them in order of their similarity to the query.' , to_tsquery ( 'english' , 'query & similarity' ), 'StartSel = <, StopSel = >' );


================================================================================
-- 来源: 1161_file_1161.txt
================================================================================

-- [DQL]
SELECT numnode ( plainto_tsquery ( 'the any' ));

-- [DQL]
SELECT numnode(' foo & bar ' :: tsquery );

-- [DQL]
SELECT querytree ( to_tsquery ( '!defined' ));


================================================================================
-- 来源: 1162_file_1162.txt
================================================================================

-- [DQL]
SELECT ts_rewrite ( 'a & b' :: tsquery , 'a' :: tsquery , 'c' :: tsquery );

-- [DDL]
CREATE TABLE tsearch . aliases ( id int , t tsquery , s tsquery );

-- [DML_INSERT]
INSERT INTO tsearch . aliases VALUES ( 1 , to_tsquery ( 'supernovae' ), to_tsquery ( 'supernovae|sn' ));

-- [DQL]
SELECT ts_rewrite ( to_tsquery ( 'supernovae & crab' ), 'SELECT t, s FROM tsearch.aliases' );

-- [DML_UPDATE]
UPDATE tsearch . aliases SET s = to_tsquery ( 'supernovae|sn & !nebulae' ) WHERE t = to_tsquery ( 'supernovae' );

-- [DQL]
SELECT ts_rewrite ( to_tsquery ( 'supernovae & crab' ), 'SELECT t, s FROM tsearch.aliases' );

-- [DQL]
SELECT ts_rewrite ( 'a & b' :: tsquery , 'SELECT t,s FROM tsearch.aliases WHERE ''a & b''::tsquery @> t' );

-- [DDL]
DROP TABLE tsearch . aliases ;


================================================================================
-- 来源: 1163_file_1163.txt
================================================================================

-- [DQL]
SELECT * FROM ts_stat ( 'SELECT to_tsvector(''english'', sr_reason_sk) FROM tpcds.store_returns WHERE sr_customer_sk < 10' ) ORDER BY nentry DESC , ndoc DESC , word LIMIT 10 ;

-- [DQL]
SELECT * FROM ts_stat ( 'SELECT to_tsvector(''english'', sr_reason_sk) FROM tpcds.store_returns WHERE sr_customer_sk < 10' , 'a' ) ORDER BY nentry DESC , ndoc DESC , word LIMIT 10 ;


================================================================================
-- 来源: 1164_file_1164.txt
================================================================================

-- [DQL]
SELECT alias , description , token FROM ts_debug ( 'english' , 'foo-bar-beta1' );

-- [DQL]
SELECT alias , description , token FROM ts_debug ( 'english' , 'http://example.com/stuff/index.html' );


================================================================================
-- 来源: 1166_file_1166.txt
================================================================================

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION astro_en ADD MAPPING FOR asciiword WITH astro_syn , english_ispell , english_stem ;


================================================================================
-- 来源: 1167_file_1167.txt
================================================================================

-- [DQL]
SELECT to_tsvector ( 'english' , 'in the list of stop words' );

-- [DQL]
SELECT ts_rank_cd ( to_tsvector ( 'english' , 'in the list of stop words' ), to_tsquery ( 'list & stop' ));

-- [DQL]
SELECT ts_rank_cd ( to_tsvector ( 'english' , 'list stop words' ), to_tsquery ( 'list & stop' ));


================================================================================
-- 来源: 1168_Simple.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH DICTIONARY public . simple_dict ( TEMPLATE = pg_catalog . simple , STOPWORDS = english );

-- [DQL]
SELECT ts_lexize ( 'public.simple_dict' , 'YeS' );

-- [DQL]
SELECT ts_lexize ( 'public.simple_dict' , 'The' );

-- [DDL]
ALTER TEXT SEARCH DICTIONARY public . simple_dict ( Accept = false );

-- [DQL]
SELECT ts_lexize ( 'public.simple_dict' , 'YeS' );

-- [DQL]
SELECT ts_lexize ( 'public.simple_dict' , 'The' );


================================================================================
-- 来源: 1169_Synonym.txt
================================================================================

-- [DQL]
SELECT * FROM ts_debug ( 'english' , 'Paris' );

-- [DDL]
CREATE TEXT SEARCH DICTIONARY my_synonym ( TEMPLATE = synonym , SYNONYMS = my_synonyms , FILEPATH = 'file:///home/dicts/' );

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION english ALTER MAPPING FOR asciiword WITH my_synonym , english_stem ;

-- [DQL]
SELECT * FROM ts_debug ( 'english' , 'Paris' );

-- [DQL]
SELECT * FROM ts_debug ( 'english' , 'paris' );

-- [DDL]
ALTER TEXT SEARCH DICTIONARY my_synonym ( CASESENSITIVE = true );

-- [DQL]
SELECT * FROM ts_debug ( 'english' , 'Paris' );

-- [DQL]
SELECT * FROM ts_debug ( 'english' , 'paris' );

-- [DDL]
CREATE TEXT SEARCH DICTIONARY syn ( TEMPLATE = synonym , SYNONYMS = synonym_sample );

-- [DQL]
SELECT ts_lexize ( 'syn' , 'indices' );

-- [DDL]
CREATE TEXT SEARCH CONFIGURATION tst ( copy = simple );

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION tst ALTER MAPPING FOR asciiword WITH syn ;

-- [DQL]
SELECT to_tsvector ( 'tst' , 'indices' );

-- [DQL]
SELECT to_tsquery ( 'tst' , 'indices' );

-- [DQL]
SELECT 'indexes are very useful' :: tsvector ;

-- [DQL]
SELECT 'indexes are very useful' :: tsvector @@ to_tsquery ( 'tst' , 'indices' );


================================================================================
-- 来源: 1170_Thesaurus.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH DICTIONARY thesaurus_astro ( TEMPLATE = thesaurus , DictFile = thesaurus_astro , Dictionary = pg_catalog . english_stem , FILEPATH = 'file:///home/dicts/' );

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION russian ALTER MAPPING FOR asciiword , asciihword , hword_asciipart WITH thesaurus_astro , english_stem ;

-- [DQL]
SELECT plainto_tsquery ( 'russian' , 'supernova star' );

-- [DQL]
SELECT to_tsvector ( 'russian' , 'supernova star' );

-- [DQL]
SELECT to_tsquery ( 'russian' , '''supernova star''' );

-- [DDL]
ALTER TEXT SEARCH DICTIONARY thesaurus_astro ( DictFile = thesaurus_astro , FILEPATH = 'file:///home/dicts/' );

-- [DQL]
SELECT plainto_tsquery ( 'russian' , 'supernova star' );


================================================================================
-- 来源: 1171_Ispell.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH DICTIONARY norwegian_ispell ( TEMPLATE = ispell , DictFile = nn_no , AffFile = nn_no , FilePath = 'file:///home/dicts' );

-- [DQL]
SELECT ts_lexize ( 'norwegian_ispell' , 'sjokoladefabrikk' );


================================================================================
-- 来源: 1173_file_1173.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH CONFIGURATION ts_conf ( COPY = pg_catalog . english );

-- [DDL]
CREATE TEXT SEARCH DICTIONARY gs_dict ( TEMPLATE = synonym , SYNONYMS = gs_dict , FILEPATH = 'file:///home/dicts' );

-- [DDL]
CREATE TEXT SEARCH DICTIONARY english_ispell ( TEMPLATE = ispell , DictFile = english , AffFile = english , StopWords = english , FILEPATH = 'file:///home/dicts' );

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION ts_conf ALTER MAPPING FOR asciiword , asciihword , hword_asciipart , word , hword , hword_part WITH gs_dict , english_ispell , english_stem ;

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION ts_conf DROP MAPPING FOR email , url , url_path , sfloat , float ;

-- [DQL]
SELECT * FROM ts_debug ( 'ts_conf' , ' GaussDB, the highly scalable, SQL compliant, open source object-relational database management system, is now undergoing beta testing of the next version of our software. ' );

-- [OTHER]
\ dF + ts_conf Text search configuration "public.ts_conf" Parser : "pg_catalog.default" Token | Dictionaries -----------------+------------------------------------- asciihword | gs_dict , english_ispell , english_stem asciiword | gs_dict , english_ispell , english_stem file | simple host | simple hword | gs_dict , english_ispell , english_stem hword_asciipart | gs_dict , english_ispell , english_stem hword_numpart | simple hword_part | gs_dict , english_ispell , english_stem int | simple numhword | simple numword | simple uint | simple version | simple word | gs_dict , english_ispell , english_stem

-- [SESSION]
SET default_text_search_config = 'public.ts_conf' ;

-- [SESSION]
SHOW default_text_search_config ;


================================================================================
-- 来源: 1175_file_1175.txt
================================================================================

-- [DQL]
SELECT * FROM ts_debug ( 'english' , 'a fat cat sat on a mat - it ate a fat rats' );


================================================================================
-- 来源: 1176_age.txt
================================================================================

-- [DQL]
SELECT * FROM ts_parse ( 'default' , '123 - a number' );

-- [DQL]
SELECT * FROM ts_token_type ( 'default' );


================================================================================
-- 来源: 1177_file_1177.txt
================================================================================

-- [DQL]
SELECT ts_lexize ( 'english_stem' , 'stars' );

-- [DQL]
SELECT ts_lexize ( 'english_stem' , 'a' );


================================================================================
-- 来源: 1184_ABORT.txt
================================================================================

-- [DDL]
CREATE TABLE customer_demographics_t1 ( CD_DEMO_SK INTEGER NOT NULL , CD_GENDER CHAR ( 1 ) , CD_MARITAL_STATUS CHAR ( 1 ) , CD_EDUCATION_STATUS CHAR ( 20 ) , CD_PURCHASE_ESTIMATE INTEGER , CD_CREDIT_RATING CHAR ( 10 ) , CD_DEP_COUNT INTEGER , CD_DEP_EMPLOYED_COUNT INTEGER , CD_DEP_COLLEGE_COUNT INTEGER ) DISTRIBUTE BY HASH ( CD_DEMO_SK );

-- [DML_INSERT]
INSERT INTO customer_demographics_t1 VALUES ( 1920801 , 'M' , 'U' , 'DOCTOR DEGREE' , 200 , 'GOOD' , 1 , 0 , 0 );

-- [TCL]
START TRANSACTION ;

-- [DML_UPDATE]
UPDATE customer_demographics_t1 SET cd_education_status = 'Unknown' ;

-- [TCL]
ABORT ;

-- [DQL]
SELECT * FROM customer_demographics_t1 WHERE cd_demo_sk = 1920801 ;

-- [DDL]
DROP TABLE customer_demographics_t1 ;


================================================================================
-- 来源: 1185_ALTER APP WORKLOAD GROUP MAPPING.txt
================================================================================

-- [DDL]
CREATE RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "High" );

-- [DDL]
CREATE WORKLOAD GROUP wg_hr1 USING RESOURCE POOL pool1 ;

-- [DDL]
CREATE APP WORKLOAD GROUP MAPPING app_wg_map1 ;

-- [OTHER]
ALTER APP WORKLOAD GROUP MAPPING app_wg_map1 WITH ( WORKLOAD_GPNAME = wg_hr1 );

-- [DDL]
DROP APP WORKLOAD GROUP MAPPING app_wg_map1 ;

-- [DDL]
DROP WORKLOAD GROUP wg_hr1 ;

-- [DDL]
DROP RESOURCE POOL pool1 ;


================================================================================
-- 来源: 1188_ALTER DATABASE.txt
================================================================================

-- [DDL]
CREATE DATABASE testdb;

--将testdb重命名为test_db1。
-- [DDL]
ALTER DATABASE testdb RENAME TO test_db1;

-- [DDL]
ALTER DATABASE test_db1 WITH CONNECTION LIMIT 100;

--查看test_db1信息。
-- [DQL]
SELECT datname,datconnlimit FROM pg_database WHERE datname = 'test_db1';

-- [DDL]
CREATE USER scott PASSWORD '********';

--将test_db1的所有者修改为jim。
-- [DDL]
ALTER DATABASE test_db1 OWNER TO scott;

--查看test_db1信息。
-- [DQL]
SELECT t1.datname, t2.usename FROM pg_database t1, pg_user t2 WHERE t1.datname='test_db1' AND t1.datdba=t2.usesysid;

-- [DDL]
CREATE TABLESPACE tbs_data1 RELATIVE LOCATION 'tablespace1/tbs_data1';

--修改test_db1默认表空间。
-- [DDL]
ALTER DATABASE test_db1 SET TABLESPACE tbs_data1;

--查看test_db1信息。
-- [DQL]
SELECT t1.datname AS database, t2.spcname AS tablespace FROM pg_database t1, pg_tablespace t2 WHERE t1.dattablespace = t2.oid AND t1.datname = 'test_db1';

-- [DDL]
CREATE USER jack PASSWORD '********';

-- [DDL]
CREATE TABLE test_tbl1(c1 int,c2 int);

-- [DQL]
SELECT tablename FROM pg_tables WHERE tablename = 'test_tbl1';

-- [DDL]
ALTER DATABASE test_db1 ENABLE PRIVATE OBJECT;

--由于隔离属性的原因，该查询只能查出0条数据。
-- [DQL]
SELECT tablename FROM pg_tables WHERE tablename = 'test_tbl1';

-- [DDL]
DROP TABLE public.test_tbl1;

-- [DDL]
DROP DATABASE test_db1;

-- [DDL]
DROP TABLESPACE tbs_data1;

-- [DDL]
DROP USER jack;

-- [DDL]
DROP USER scott;


================================================================================
-- 来源: 1189_ALTER DATABASE LINK.txt
================================================================================

-- [DDL]
CREATE USER user01 WITH SYSADMIN PASSWORD '********';

-- [SESSION]
SET ROLE user01 PASSWORD '********';

--创建公共dblink
-- [OTHER]
CREATE PUBLIC DATABASE LINK public_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

-- 修改dblink对象信息
-- [OTHER]
ALTER PUBLIC DATABASE LINK public_dblink CONNECT TO 'user2' IDENTIFIED BY '********';

-- [OTHER]
DROP PUBLIC DATABASE LINK public_dblink;

-- [SESSION]
RESET ROLE;

-- [DDL]
DROP USER USER01 CASCADE;


================================================================================
-- 来源: 1190_ALTER DATA SOURCE.txt
================================================================================

-- [DDL]
CREATE DATA SOURCE ds_test1 ;

-- [OTHER]
ALTER DATA SOURCE ds_test1 RENAME TO ds_test ;

-- [DDL]
CREATE USER user_test1 IDENTIFIED BY '********' ;

-- [DDL]
ALTER USER user_test1 WITH SYSADMIN ;

-- [OTHER]
ALTER DATA SOURCE ds_test OWNER TO user_test1 ;

-- [OTHER]
ALTER DATA SOURCE ds_test TYPE 'MPPDB_TYPE' VERSION 'XXX' ;

-- [OTHER]
ALTER DATA SOURCE ds_test OPTIONS ( add dsn 'mppdb' , username 'test_user' );

-- [OTHER]
ALTER DATA SOURCE ds_test OPTIONS ( set dsn 'unknown' );

-- [OTHER]
ALTER DATA SOURCE ds_test OPTIONS ( drop username );

-- [DDL]
DROP DATA SOURCE ds_test ;

-- [DDL]
DROP USER user_test1 ;


================================================================================
-- 来源: 1191_ALTER DEFAULT PRIVILEGES.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds GRANT SELECT ON TABLES TO PUBLIC ;

-- [DDL]
CREATE USER jack PASSWORD '******' ;

-- [DDL]
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds GRANT INSERT ON TABLES TO jack ;

-- [DCL_GRANT]
GRANT USAGE , CREATE ON SCHEMA tpcds TO jack ;

-- [DDL]
ALTER DEFAULT PRIVILEGES FOR ROLE jack IN SCHEMA tpcds GRANT INSERT ON TABLES TO jack ;

-- [DDL]
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds REVOKE SELECT ON TABLES FROM PUBLIC ;

-- [DDL]
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds REVOKE INSERT ON TABLES FROM jack ;

-- [DDL]
DROP USER jack ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1192_ALTER DIRECTORY.txt
================================================================================

-- [DDL]
CREATE OR REPLACE DIRECTORY dir as '/tmp/' ;

-- [DDL]
CREATE USER jim PASSWORD '********' ;

-- [DDL]
ALTER DIRECTORY dir OWNER TO jim ;

-- [DDL]
DROP DIRECTORY dir ;


================================================================================
-- 来源: 1193_ALTER FOREIGN TABLE ().txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE FOREIGN TABLE tpcds . customer_ft ( c_customer_sk integer , c_customer_id char ( 16 ) , c_current_cdemo_sk integer , c_current_hdemo_sk integer , c_current_addr_sk integer , c_first_shipto_date_sk integer , c_first_sales_date_sk integer , c_salutation char ( 10 ) , c_first_name char ( 20 ) , c_last_name char ( 30 ) , c_preferred_cust_flag char ( 1 ) , c_birth_day integer , c_birth_month integer , c_birth_year integer , c_birth_country varchar ( 20 ) , c_login char ( 13 ) , c_email_address char ( 50 ) , c_last_review_date char ( 10 ) ) SERVER gsmpp_server OPTIONS ( location 'gsfs://10.185.179.143:5000/customer1*.dat' , FORMAT 'TEXT' , DELIMITER '|' , encoding 'utf8' , mode 'Normal' ) READ ONLY ;

-- [DDL]
ALTER FOREIGN TABLE tpcds . customer_ft options ( drop mode );

-- [DDL]
DROP FOREIGN TABLE tpcds . customer_ft ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1194_ALTER FUNCTION.txt
================================================================================

-- [SESSION]
SET behavior_compat_options ='plpgsql_dependency';

-- 创建函数
-- [DDL]
CREATE OR REPLACE FUNCTION test_func(a int) RETURN int IS proc_var int;

-- 用函数名重编译函数
-- [OTHER]
ALTER PROCEDURE test_func COMPILE;

-- 用函数带类型签名重编译存储过程
-- [OTHER]
ALTER PROCEDURE test_func(int) COMPILE;

-- 删除函数
-- [DDL]
DROP FUNCTION test_func;


================================================================================
-- 来源: 1195_ALTER GLOBAL CONFIGURATION.txt
================================================================================

-- [DDL]
ALTER GLOBAL CONFIGURATION with ( redis_is_ok = true );

-- [DQL]
SELECT * FROM gs_global_config ;

-- [DDL]
ALTER GLOBAL CONFIGURATION with ( redis_is_ok = false );

-- [DQL]
SELECT * FROM gs_global_config ;

-- [DDL]
DROP GLOBAL CONFIGURATION redis_is_ok ;

-- [DQL]
SELECT * FROM gs_global_config ;


================================================================================
-- 来源: 1196_ALTER GROUP.txt
================================================================================

-- [DDL]
CREATE GROUP super_users WITH PASSWORD "********" ;

-- [DDL]
CREATE ROLE lche WITH PASSWORD "********" ;

-- [DDL]
CREATE ROLE jim WITH PASSWORD "********" ;

-- [DDL]
ALTER GROUP super_users ADD USER lche , jim ;

-- [DDL]
ALTER GROUP super_users DROP USER jim ;

-- [DDL]
ALTER GROUP super_users RENAME TO normal_users ;

-- [DDL]
DROP ROLE lche , jim ;

-- [DDL]
DROP GROUP normal_users ;


================================================================================
-- 来源: 1197_ALTER INDEX.txt
================================================================================

-- [DDL]
CREATE TABLE test1(col1 int, col2 int);

-- [DDL]
CREATE INDEX aa ON test1(col1);

--将索引aa重命名为idx_test1_col1。
-- [DDL]
ALTER INDEX aa RENAME TO idx_test1_col1;

--查询test1表上的索引信息。
-- [DQL]
SELECT tablename,indexname,tablespace FROM pg_indexes WHERE tablename = 'test1';

-- [DDL]
CREATE TABLESPACE tbs_index1 RELATIVE LOCATION 'tablespace1/tbs_index1';

--修改索引idx_test1_col1的所属表空间为tbs_index1。
-- [DDL]
ALTER INDEX IF EXISTS idx_test1_col1 SET TABLESPACE tbs_index1;

--查询test1表上的索引信息。
-- [DQL]
SELECT tablename,indexname,tablespace FROM pg_indexes WHERE tablename = 'test1';

--修改索引idx_test1_col1 的填充因子。
-- [DDL]
ALTER INDEX IF EXISTS idx_test1_col1 SET (FILLFACTOR = 70);

-- [DDL]
ALTER INDEX IF EXISTS idx_test1_col1 RESET (FILLFACTOR);

-- [DDL]
ALTER INDEX IF EXISTS idx_test1_col1 UNUSABLE;

--查看索引idx_test1_col1的可用性。
-- [DQL]
SELECT indisusable FROM pg_index WHERE indexrelid = 'idx_test1_col1'::regclass;

--重建索引idx_test1_col1。
-- [DDL]
ALTER INDEX idx_test1_col1 REBUILD;

--查看索引idx_test1_col1的可用性。
-- [DQL]
SELECT indisusable FROM pg_index WHERE indexrelid = 'idx_test1_col1'::regclass;

--删除。
-- [DDL]
DROP INDEX idx_test1_col1;

-- [DDL]
DROP TABLE test1;

-- [DDL]
DROP TABLESPACE tbs_index1;

-- [DDL]
CREATE TABLE test2(col1 int, col2 int) PARTITION BY RANGE (col1)( PARTITION p1 VALUES LESS THAN (100), PARTITION p2 VALUES LESS THAN (200) );

--创建分区索引。
-- [DDL]
CREATE INDEX idx_test2_col1 ON test2(col1) LOCAL( PARTITION p1, PARTITION p2 );

--重命名索引分区。
-- [DDL]
ALTER INDEX idx_test2_col1 RENAME PARTITION p1 TO p1_test2_idx;

-- [DDL]
ALTER INDEX idx_test2_col1 RENAME PARTITION p2 TO p2_test2_idx;

--查询索引idx_test2_col1分区的名称。
-- [DQL]
SELECT relname FROM pg_partition WHERE parentid = 'idx_test2_col1'::regclass;

-- [DDL]
CREATE TABLESPACE tbs_index2 RELATIVE LOCATION 'tablespace1/tbs_index2';

-- [DDL]
CREATE TABLESPACE tbs_index3 RELATIVE LOCATION 'tablespace1/tbs_index3';

--修改索引idx_test2_col1分区的所属表空间。
-- [DDL]
ALTER INDEX idx_test2_col1 MOVE PARTITION p1_test2_idx TABLESPACE tbs_index2;

-- [DDL]
ALTER INDEX idx_test2_col1 MOVE PARTITION p2_test2_idx TABLESPACE tbs_index3;

--查询索引idx_test2_col1分区的所属表空间。
-- [DQL]
SELECT t1.relname index_name, t2.spcname tablespace_name FROM pg_partition t1, pg_tablespace t2 WHERE t1.parentid = 'idx_test2_col1'::regclass AND t1.reltablespace = t2.oid;

--删除。
-- [DDL]
DROP INDEX idx_test2_col1;

-- [DDL]
DROP TABLE test2;

-- [DDL]
DROP TABLESPACE tbs_index2;

-- [DDL]
DROP TABLESPACE tbs_index3;


================================================================================
-- 来源: 1200_ALTER MASKING POLICY.txt
================================================================================

-- [DDL]
CREATE USER dev_mask PASSWORD '********' ;

-- [DDL]
CREATE USER bob_mask PASSWORD '********' ;

-- [DDL]
CREATE TABLE tb_for_masking ( col1 text , col2 text , col3 text );

-- [DDL]
CREATE RESOURCE LABEL mask_lb1 ADD COLUMN ( tb_for_masking . col1 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb2 ADD COLUMN ( tb_for_masking . col2 );

-- [DDL]
CREATE MASKING POLICY maskpol1 maskall ON LABEL ( mask_lb1 );

-- [DDL]
ALTER MASKING POLICY maskpol1 COMMENTS 'masking policy for tb_for_masking.col1' ;

-- [DDL]
ALTER MASKING POLICY maskpol1 ADD randommasking ON LABEL ( mask_lb2 );

-- [DDL]
ALTER MASKING POLICY maskpol1 REMOVE randommasking ON LABEL ( mask_lb2 );

-- [DDL]
ALTER MASKING POLICY maskpol1 MODIFY randommasking ON LABEL ( mask_lb1 );

-- [DDL]
ALTER MASKING POLICY maskpol1 MODIFY ( FILTER ON ROLES ( dev_mask , bob_mask ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' ));

-- [DDL]
ALTER MASKING POLICY maskpol1 DROP FILTER ;

-- [DDL]
ALTER MASKING POLICY maskpol1 DISABLE ;

-- [DDL]
DROP MASKING POLICY maskpol1 ;

-- [DDL]
DROP RESOURCE LABEL mask_lb1 , mask_lb2 ;

-- [DDL]
DROP TABLE tb_for_masking ;

-- [DDL]
DROP USER dev_mask , bob_mask ;


================================================================================
-- 来源: 1201_ALTER MATERIALIZED VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建全量物化视图。
-- [DDL]
CREATE MATERIALIZED VIEW foo AS SELECT * FROM my_table;

--把物化视图foo重命名为bar。
-- [DDL]
ALTER MATERIALIZED VIEW foo RENAME TO bar;

--删除全量物化视图。
-- [DDL]
DROP MATERIALIZED VIEW bar;

--删除表my_table。
-- [DDL]
DROP TABLE my_table;


================================================================================
-- 来源: 1204_ALTER PACKAGE.txt
================================================================================

-- [SESSION]
SET behavior_compat_options ='plpgsql_dependency';

-- 创建包
-- [DDL]
CREATE OR REPLACE PACKAGE TEST_PKG AS pkg_var int := 1;

-- [DDL]
CREATE OR REPLACE PACKAGE BODY TEST_PKG AS PROCEDURE test_pkg_proc(var int) IS BEGIN pkg_var := 1;

-- 重编译包
-- [DDL]
ALTER PACKAGE test_pkg COMPILE;

-- 删除包
-- [DDL]
DROP PACKAGE TEST_PKG;

-- 关闭依赖功能
-- [SESSION]
SET behavior_compat_options = '';


================================================================================
-- 来源: 1205_ALTER RESOURCE LABEL.txt
================================================================================

-- [DDL]
CREATE TABLE table_for_label ( col1 int , col2 text );

-- [DDL]
CREATE RESOURCE LABEL table_label ADD COLUMN ( table_for_label . col1 );

-- [DDL]
ALTER RESOURCE LABEL table_label ADD COLUMN ( table_for_label . col2 );

-- [DDL]
ALTER RESOURCE LABEL table_label REMOVE COLUMN ( table_for_label . col1 );

-- [DDL]
DROP RESOURCE LABEL table_label ;

-- [DDL]
DROP TABLE table_for_label ;


================================================================================
-- 来源: 1206_ALTER RESOURCE POOL.txt
================================================================================

-- [DDL]
CREATE RESOURCE POOL pool1 ;

-- [DDL]
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "High" );

-- [DDL]
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "class1:Low" );

-- [DDL]
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "class1:wg1" );

-- [DDL]
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "class1:wg2:3" );

-- [DDL]
DROP RESOURCE POOL pool1 ;


================================================================================
-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY.txt
================================================================================

-- [DDL]
CREATE TABLE all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

-- [OTHER]
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

-- [OTHER]
\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_rls" FOR ALL TO public USING ((( role ):: name = "current_user" ())) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no --修改行访问控制all_data_rls的名称。

-- [OTHER]
ALTER ROW LEVEL SECURITY POLICY all_data_rls ON all_data RENAME TO all_data_new_rls ;

-- [DDL]
CREATE ROLE alice WITH PASSWORD "********" ;

-- [DDL]
CREATE ROLE bob WITH PASSWORD "********" ;

-- [OTHER]
ALTER ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data TO alice , bob ;

-- [OTHER]
\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_new_rls" FOR ALL TO alice , bob USING ((( role ):: name = "current_user" ())) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --修改行访问控制策略表达式。

-- [OTHER]
ALTER ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data USING ( id > 100 AND role = current_user );

-- [OTHER]
\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_new_rls" FOR ALL TO alice , bob USING ((( id > 100 ) AND (( role ):: name = "current_user" ()))) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --删除访问控制策略。

-- [OTHER]
DROP ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data ;

-- [DDL]
DROP ROLE alice , bob ;

-- [DDL]
DROP TABLE all_data ;


================================================================================
-- 来源: 1209_ALTER SCHEMA.txt
================================================================================

-- [DDL]
CREATE DATABASE test1 WITH DBCOMPATIBILITY = 'MYSQL' ;

-- [OTHER]
\ c test1 --创建模式ds。

-- [DDL]
CREATE SCHEMA ds ;

-- [DDL]
ALTER SCHEMA ds RENAME TO ds_new ;

-- [DDL]
CREATE USER jack PASSWORD '********' ;

-- [DDL]
ALTER SCHEMA ds_new OWNER TO jack ;

-- [DDL]
CREATE SCHEMA sch ;

-- [DDL]
ALTER SCHEMA sch CHARACTER SET utf8mb4 COLLATE utf8mb4_bin ;

-- [DDL]
DROP SCHEMA ds_new ;

-- [DDL]
DROP SCHEMA sch ;

-- [DDL]
DROP USER jack ;

-- [OTHER]
\ c postgres

-- [DDL]
DROP DATABASE test1 ;


================================================================================
-- 来源: 1210_ALTER SEQUENCE.txt
================================================================================

-- [DDL]
CREATE SEQUENCE serial START 101 ;

-- [DDL]
CREATE TABLE t1 ( c1 bigint default nextval ( 'serial' ));

-- [DDL]
ALTER SEQUENCE serial OWNED BY t1 . c1 ;

-- [DDL]
DROP SEQUENCE serial CASCADE ;

-- [DDL]
DROP TABLE t1 ;


================================================================================
-- 来源: 1211_ALTER SERVER.txt
================================================================================

-- [DDL]
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw ;

-- [DDL]
ALTER SERVER my_server RENAME TO my_server_1 ;

-- [DDL]
DROP SERVER my_server_1 ;


================================================================================
-- 来源: 1212_ALTER SESSION.txt
================================================================================

-- [DDL]
CREATE SCHEMA ds ;

-- [SESSION]
SET SEARCH_PATH TO ds , public ;

-- [SESSION]
SET DATESTYLE TO postgres , dmy ;

-- [DDL]
ALTER SESSION SET NAMES 'UTF8' ;

-- [SESSION]
SET TIME ZONE 'PST8PDT' ;

-- [SESSION]
SET TIME ZONE 'Europe/Rome' ;

-- [DDL]
ALTER SESSION SET CURRENT_SCHEMA TO tpcds ;

-- [DDL]
ALTER SESSION SET XML OPTION DOCUMENT ;

-- [DDL]
CREATE ROLE joe WITH PASSWORD '********' ;

-- [DDL]
ALTER SESSION SET SESSION AUTHORIZATION joe PASSWORD '********' ;

-- [DDL]
DROP SCHEMA ds ;

-- [DDL]
DROP ROLE joe ;

-- [TCL]
START TRANSACTION ;

-- [DDL]
ALTER SESSION SET TRANSACTION READ ONLY ;

-- [OTHER]
END ;


================================================================================
-- 来源: 1213_ALTER SYNONYM.txt
================================================================================

-- [DDL]
CREATE USER sysadmin WITH SYSADMIN PASSWORD '********' ;

-- [OTHER]
\ c - sysadmin --创建同义词t1。

-- [DDL]
CREATE OR REPLACE SYNONYM t1 FOR ot . t1 ;

-- [DDL]
CREATE USER u1 PASSWORD '********' ;

-- [DCL_GRANT]
GRANT ALL ON SCHEMA sysadmin TO u1 ;

-- [DDL]
ALTER SYNONYM t1 OWNER TO u1 ;

-- [DDL]
DROP SYNONYM t1 ;

-- [DCL_REVOKE]
REVOKE ALL ON SCHEMA sysadmin FROM u1 ;

-- [DDL]
DROP USER u1 ;

-- [OTHER]
\ c - init_user --删除用户sysadmin。

-- [DDL]
DROP USER sysadmin ;


================================================================================
-- 来源: 1214_ALTER SYSTEM KILL SESSION.txt
================================================================================

-- [DQL]
SELECT sid , serial # , username FROM dv_sessions WHERE sid IN ( SELECT pid FROM pg_stat_activity );

-- [DDL]
ALTER SYSTEM KILL SESSION '140469417232128,0' IMMEDIATE ;


================================================================================
-- 来源: 1215_ALTER TABLE.txt
================================================================================

-- [DDL]
CREATE TABLE aa(c1 int, c2 int);

-- [DDL]
ALTER TABLE IF EXISTS aa RENAME TO test_alt1;

-- [DDL]
CREATE SCHEMA test_schema;

--把表test_alt1的所属模式修改为test_schema。
-- [DDL]
ALTER TABLE test_alt1 SET SCHEMA test_schema;

--查询表信息。
-- [DQL]
SELECT schemaname,tablename FROM pg_tables WHERE tablename = 'test_alt1';

-- [DDL]
CREATE USER test_user PASSWORD '********';

-- 修改test_alt1表的所有者为test_user;
-- [DDL]
ALTER TABLE IF EXISTS test_schema.test_alt1 OWNER TO test_user;

-- 查看
-- [DQL]
SELECT tablename, schemaname, tableowner FROM pg_tables WHERE tablename = 'test_alt1';

-- [DDL]
CREATE TABLESPACE tbs_data1 RELATIVE LOCATION 'tablespace1/tbs_data1';

--修改test_alt1表的空间为tbs_data1。
-- [DDL]
ALTER TABLE test_schema.test_alt1 SET TABLESPACE tbs_data1;

-- 查看。
-- [DQL]
SELECT tablename, tablespace FROM pg_tables WHERE tablename = 'test_alt1';

--删除。
-- [DDL]
DROP TABLE test_schema.test_alt1;

-- [DDL]
DROP TABLESPACE tbs_data1;

-- [DDL]
DROP SCHEMA test_schema;

-- [DDL]
DROP USER test_user;

-- [DDL]
CREATE TABLE test_alt2(c1 INT,c2 INT);

-- 修改列名
-- [DDL]
ALTER TABLE test_alt2 RENAME c1 TO id;

-- [DDL]
ALTER TABLE test_alt2 RENAME COLUMN c2 to areaid;

-- [DDL]
ALTER TABLE IF EXISTS test_alt2 ADD COLUMN name VARCHAR(20);

-- [DDL]
ALTER TABLE test_alt2 MODIFY name VARCHAR(50);

-- [DDL]
ALTER TABLE test_alt2 ALTER COLUMN name TYPE VARCHAR(25);

-- [DDL]
ALTER TABLE test_alt2 DROP COLUMN areaid;

--修改test_alt2表中name字段的存储模式。
-- [DDL]
ALTER TABLE test_alt2 ALTER COLUMN name SET STORAGE PLAIN;

--删除。
-- [DDL]
DROP TABLE test_alt2;

-- [DDL]
CREATE TABLE test_alt3(pid INT, areaid CHAR(5), name VARCHAR(20));

--为pid添加非空约束。
-- [DDL]
ALTER TABLE test_alt3 MODIFY pid NOT NULL;

-- [DDL]
ALTER TABLE test_alt3 MODIFY pid NULL;

-- [DDL]
ALTER TABLE test_alt3 ALTER COLUMN areaid SET DEFAULT '00000';

-- [DDL]
ALTER TABLE test_alt3 ALTER COLUMN areaid DROP DEFAULT;

-- [DDL]
ALTER TABLE test_alt3 ADD CONSTRAINT pk_test3_pid PRIMARY KEY (pid);

-- [DDL]
CREATE TABLE test_alt4(c1 INT, c2 INT);

--建索引。
-- [DDL]
CREATE UNIQUE INDEX pk_test4_c1 ON test_alt4(c1);

--添加约束时关联已经创建的索引。
-- [DDL]
ALTER TABLE test_alt4 ADD CONSTRAINT pk_test4_c1 PRIMARY KEY USING INDEX pk_test4_c1;

--删除。
-- [DDL]
DROP TABLE test_alt4;

-- [DDL]
ALTER TABLE test_alt3 DROP CONSTRAINT IF EXISTS pk_test3_pid;

--删除。
-- [DDL]
DROP TABLE test_alt3;


================================================================================
-- 来源: 1218_ALTER TEXT SEARCH CONFIGURATION.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH CONFIGURATION english_1 ( parser = default );

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION english_1 ADD MAPPING FOR word WITH simple , english_stem ;

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION english_1 ADD MAPPING FOR email WITH english_stem , french_stem ;

-- [DQL]
SELECT b . cfgname , a . maptokentype , a . mapseqno , a . mapdict , c . dictname FROM pg_ts_config_map a , pg_ts_config b , pg_ts_dict c WHERE a . mapcfg = b . oid AND a . mapdict = c . oid AND b . cfgname = 'english_1' ORDER BY 1 , 2 , 3 , 4 , 5 ;

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION english_1 ALTER MAPPING REPLACE french_stem with german_stem ;

-- [DQL]
SELECT b . cfgname , a . maptokentype , a . mapseqno , a . mapdict , c . dictname FROM pg_ts_config_map a , pg_ts_config b , pg_ts_dict c WHERE a . mapcfg = b . oid AND a . mapdict = c . oid AND b . cfgname = 'english_1' ORDER BY 1 , 2 , 3 , 4 , 5 ;

-- [DDL]
DROP TEXT SEARCH CONFIGURATION english_1 ;


================================================================================
-- 来源: 1219_ALTER TEXT SEARCH DICTIONARY.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH DICTIONARY my_dict ( TEMPLATE = Simple );

-- [DDL]
ALTER TEXT SEARCH DICTIONARY my_dict ( Accept = true );

-- [DDL]
ALTER TEXT SEARCH DICTIONARY my_dict ( Accept );

-- [DDL]
ALTER TEXT SEARCH DICTIONARY my_dict ( dummy );

-- [DDL]
DROP TEXT SEARCH DICTIONARY my_dict ;


================================================================================
-- 来源: 1223_ALTER VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE test_tb1(col1 INT,col2 INT);

--创建视图。
-- [DDL]
CREATE VIEW abc AS SELECT * FROM test_tb1;

--重命名视图。
-- [DDL]
ALTER VIEW IF EXISTS abc RENAME TO test_v1;

-- [DDL]
CREATE ROLE role_test PASSWORD '********';

--修改视图所有者。
-- [DDL]
ALTER VIEW IF EXISTS test_v1 OWNER TO role_test;

-- [DDL]
CREATE SCHEMA tcpds;

--修改视图所属模式。
-- [DDL]
ALTER VIEW test_v1 SET SCHEMA tcpds;

-- [DDL]
ALTER VIEW tcpds.test_v1 SET (security_barrier = TRUE);

--重置视图选项。
-- [DDL]
ALTER VIEW tcpds.test_v1 RESET (security_barrier);

--删除视图test_v1。
-- [DDL]
DROP VIEW tcpds.test_v1;

--删除表test_tb1。
-- [DDL]
DROP TABLE test_tb1;

--删除用户。
-- [DDL]
DROP ROLE role_test;

--删除schema。
-- [DDL]
DROP SCHEMA tcpds;


================================================================================
-- 来源: 1224_ALTER WORKLOAD GROUP.txt
================================================================================

-- [DDL]
CREATE RESOURCE POOL pool1 ;

-- [DDL]
CREATE WORKLOAD GROUP group1 ;

-- [DDL]
ALTER WORKLOAD GROUP group1 USING RESOURCE POOL pool1 WITH ( ACT_STATEMENTS = 10 );

-- [DDL]
DROP WORKLOAD GROUP group1 ;

-- [DDL]
DROP RESOURCE POOL pool1 ;


================================================================================
-- 来源: 1225_ANALYZE _ ANALYSE.txt
================================================================================

-- [DDL]
CREATE TABLE customer_info ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL , WR_REFUNDED_CUSTOMER_SK INTEGER ) DISTRIBUTE BY HASH ( WR_ITEM_SK );

-- [DDL]
CREATE TABLE customer_par ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL , WR_REFUNDED_CUSTOMER_SK INTEGER ) DISTRIBUTE BY HASH ( WR_ITEM_SK ) PARTITION BY RANGE ( WR_RETURNED_DATE_SK ) ( PARTITION P1 VALUES LESS THAN ( 2452275 ), PARTITION P2 VALUES LESS THAN ( 2452640 ), PARTITION P3 VALUES LESS THAN ( 2453000 ), PARTITION P4 VALUES LESS THAN ( MAXVALUE ) ) ENABLE ROW MOVEMENT ;

-- [MAINTENANCE]
ANALYZE customer_info ;

-- [MAINTENANCE]
ANALYZE VERBOSE customer_info ;

-- [DDL]
DROP TABLE customer_info ;

-- [DDL]
DROP TABLE customer_par ;


================================================================================
-- 来源: 1227_BEGIN.txt
================================================================================

-- [TCL]
BEGIN dbe_output . print_line ( 'Hello' );


================================================================================
-- 来源: 1229_CALL.txt
================================================================================

-- [DDL]
CREATE FUNCTION func_add_sql ( num1 integer , num2 integer ) RETURN integer AS BEGIN RETURN num1 + num2 ;

-- [PLSQL]
CALL func_add_sql ( 1 , 3 );

-- [PLSQL]
CALL func_add_sql ( num1 => 1 , num2 => 3 );

-- [PLSQL]
CALL func_add_sql ( num2 : = 2 , num1 : = 3 );

-- [DDL]
DROP FUNCTION func_add_sql ;

-- [DDL]
CREATE FUNCTION func_increment_sql ( num1 IN integer , num2 IN integer , res OUT integer ) RETURN integer AS BEGIN res : = num1 + num2 ;

-- [PLSQL]
CALL func_increment_sql ( 1 , 2 , 1 );

-- [PLSQL]
DECLARE res int ;

-- [DDL]
DROP FUNCTION func_increment_sql ;


================================================================================
-- 来源: 1230_CHECKPOINT.txt
================================================================================

-- [MAINTENANCE]
CHECKPOINT ;


================================================================================
-- 来源: 1231_CLEAN CONNECTION.txt
================================================================================

-- [DDL]
CREATE DATABASE test_clean_connection ;

-- [DDL]
CREATE USER jack PASSWORD '********' ;

-- [DDL]
CREATE DATABASE testdb ;

-- [OTHER]
CLEAN CONNECTION TO NODE ( dn_6001_6002 , dn_6003_6004 ) FOR DATABASE template1 ;

-- [OTHER]
CLEAN CONNECTION TO NODE ( dn_6001_6002 ) TO USER jack ;

-- [OTHER]
CLEAN CONNECTION TO ALL FORCE FOR DATABASE testdb ;

-- [DDL]
DROP DATABASE testdb ;

-- [DDL]
DROP USER jack ;

-- [DDL]
DROP DATABASE test_clean_connection ;


================================================================================
-- 来源: 1233_CLUSTER.txt
================================================================================

-- [DDL]
CREATE TABLE test_c1 ( id int , name varchar ( 20 ));

-- [DDL]
CREATE INDEX idx_test_c1_id ON test_c1 ( id );

-- [DML_INSERT]
INSERT INTO test_c1 VALUES ( 3 , 'Joe' ),( 1 , 'Jack' ),( 2 , 'Scott' );

-- [DQL]
SELECT * FROM test_c1 ;

-- [MAINTENANCE]
CLUSTER test_c1 USING idx_test_c1_id ;

-- [DQL]
SELECT * FROM test_c1 ;

-- [DDL]
DROP TABLE test_c1 ;

-- [DDL]
CREATE TABLE test(col1 int,CONSTRAINT pk_test PRIMARY KEY (col1));

-- 第一次聚簇排序不带USING关键字报错
-- [MAINTENANCE]
CLUSTER test;

-- 聚簇排序
-- [MAINTENANCE]
CLUSTER test USING pk_test;

--对已做过聚簇的表重新进行聚簇
-- [MAINTENANCE]
CLUSTER VERBOSE test;

-- 删除
-- [DDL]
DROP TABLE test;

-- [DDL]
CREATE TABLE test_c2(id int, info varchar(4)) PARTITION BY RANGE (id)( PARTITION p1 VALUES LESS THAN (11), PARTITION p2 VALUES LESS THAN (21) );

-- [DDL]
CREATE INDEX idx_test_c2_id1 ON test_c2(id);

-- [DML_INSERT]
INSERT INTO test_c2 VALUES (6,'ABBB'),(2,'ABAB'),(9,'AAAA');

-- [DML_INSERT]
INSERT INTO test_c2 VALUES (11,'AAAB'),(19,'BBBA'),(16,'BABA');

-- 查看
-- [DQL]
SELECT * FROM test_c2;

-- 对分区p2进行聚簇排序
-- [MAINTENANCE]
CLUSTER test_c2 PARTITION (p2) USING idx_test_c2_id1;

-- 查看
-- [DQL]
SELECT * FROM test_c2;

-- 删除
-- [DDL]
DROP TABLE test_c2;


================================================================================
-- 来源: 1234_COMMENT.txt
================================================================================

-- [DDL]
CREATE TABLE emp( empno varchar(7), ename varchar(50), job varchar(50), mgr varchar(7), deptno int );

--表添加注释
-- [DDL]
COMMENT ON TABLE emp IS '部门表';

--字段添加注释
-- [DDL]
COMMENT ON COLUMN emp.empno IS '员工编号';

-- [DDL]
COMMENT ON COLUMN emp.ename IS '员工姓名';

-- [DDL]
COMMENT ON COLUMN emp.job IS '职务';

-- [DDL]
COMMENT ON COLUMN emp.mgr IS '上司编号';

-- [DDL]
COMMENT ON COLUMN emp.deptno IS '部门编号';

--删除
-- [DDL]
DROP TABLE emp;


================================================================================
-- 来源: 1235_COMMIT _ END.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . customer_demographics_t2 ( CD_DEMO_SK INTEGER NOT NULL , CD_GENDER CHAR ( 1 ) , CD_MARITAL_STATUS CHAR ( 1 ) , CD_EDUCATION_STATUS CHAR ( 20 ) , CD_PURCHASE_ESTIMATE INTEGER , CD_CREDIT_RATING CHAR ( 10 ) , CD_DEP_COUNT INTEGER , CD_DEP_EMPLOYED_COUNT INTEGER , CD_DEP_COLLEGE_COUNT INTEGER ) DISTRIBUTE BY HASH ( CD_DEMO_SK );

-- [TCL]
START TRANSACTION ;

-- [DML_INSERT]
INSERT INTO tpcds . customer_demographics_t2 VALUES ( 1 , 'M' , 'U' , 'DOCTOR DEGREE' , 1200 , 'GOOD' , 1 , 0 , 0 );

-- [DML_INSERT]
INSERT INTO tpcds . customer_demographics_t2 VALUES ( 2 , 'F' , 'U' , 'MASTER DEGREE' , 300 , 'BAD' , 1 , 0 , 0 );

-- [TCL]
COMMIT ;

-- [DQL]
SELECT * FROM tpcds . customer_demographics_t2 ;

-- [DDL]
DROP TABLE tpcds . customer_demographics_t2 ;

-- [DDL]
DROP SCHEMA tpcds ;


================================================================================
-- 来源: 1236_COMMIT PREPARED.txt
================================================================================

-- [TCL]
BEGIN ;

-- [TCL]
PREPARE TRANSACTION 'trans_test' ;

-- [DDL]
CREATE TABLE item1 ( id int );

-- [TCL]
COMMIT PREPARED 'trans_test' ;

-- [DDL]
DROP TABLE item1 ;


================================================================================
-- 来源: 1237_COPY.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . ship_mode ( SM_SHIP_MODE_SK INTEGER NOT NULL , SM_SHIP_MODE_ID CHAR ( 16 ) NOT NULL , SM_TYPE CHAR ( 30 ) , SM_CODE CHAR ( 10 ) , SM_CARRIER CHAR ( 20 ) , SM_CONTRACT CHAR ( 20 ) ) DISTRIBUTE BY HASH ( SM_SHIP_MODE_SK );

-- [DML_INSERT]
INSERT INTO tpcds . ship_mode VALUES ( 1 , 'a' , 'b' , 'c' , 'd' , 'e' );

-- [DML_COPY]
COPY tpcds . ship_mode TO '/home/omm/ds_ship_mode.dat' ;

-- [DML_COPY]
COPY tpcds . ship_mode TO STDOUT ;

-- [DML_COPY]
COPY tpcds . ship_mode TO STDOUT WITH ( delimiter ',' , encoding 'utf8' );

-- [DML_COPY]
COPY tpcds . ship_mode TO STDOUT WITH ( format 'CSV' , force_quote ( SM_SHIP_MODE_SK ));

-- [DDL]
CREATE TABLE tpcds . ship_mode_t1 ( SM_SHIP_MODE_SK INTEGER NOT NULL , SM_SHIP_MODE_ID CHAR ( 16 ) NOT NULL , SM_TYPE CHAR ( 30 ) , SM_CODE CHAR ( 10 ) , SM_CARRIER CHAR ( 20 ) , SM_CONTRACT CHAR ( 20 ) ) DISTRIBUTE BY HASH ( SM_SHIP_MODE_SK );

-- [DML_COPY]
COPY tpcds . ship_mode_t1 FROM STDIN ;

-- [DML_COPY]
COPY tpcds . ship_mode_t1 FROM '/home/omm/ds_ship_mode.dat' ;

-- [DML_COPY]
COPY tpcds . ship_mode_t1 FROM '/home/omm/ds_ship_mode.dat' TRANSFORM ( SM_TYPE AS LEFT ( SM_TYPE , 10 ));

-- [DML_COPY]
COPY tpcds . ship_mode_t1 FROM '/home/omm/ds_ship_mode.dat' WITH ( format 'text' , delimiter E '\t' , ignore_extra_data 'true' , noescaping 'true' );

-- [DML_COPY]
COPY tpcds . ship_mode_t1 FROM '/home/omm/ds_ship_mode.dat' FIXED FORMATTER ( SM_SHIP_MODE_SK ( 0 , 2 ), SM_SHIP_MODE_ID ( 2 , 16 ), SM_TYPE ( 18 , 30 ), SM_CODE ( 50 , 10 ), SM_CARRIER ( 61 , 20 ), SM_CONTRACT ( 82 , 20 )) header ignore_extra_data ;

-- [DDL]
DROP TABLE tpcds . ship_mode ;

-- [DDL]
DROP TABLE tpcds . ship_mode_t1 ;

-- [DDL]
DROP SCHEMA tpcds ;


================================================================================
-- 来源: 1238_CREATE APP WORKLOAD GROUP MAPPING.txt
================================================================================

-- [DDL]
CREATE RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "High" );

-- [DDL]
CREATE WORKLOAD GROUP group1 USING RESOURCE POOL pool1 ;

-- [DDL]
CREATE APP WORKLOAD GROUP MAPPING app_wg_map1 WITH ( WORKLOAD_GPNAME = group1 );

-- [DDL]
CREATE APP WORKLOAD GROUP MAPPING app_wg_map2 ;

-- [DDL]
DROP APP WORKLOAD GROUP MAPPING app_wg_map1 ;

-- [DDL]
DROP APP WORKLOAD GROUP MAPPING app_wg_map2 ;

-- [DDL]
DROP WORKLOAD GROUP group1 ;

-- [DDL]
DROP RESOURCE POOL pool1 ;


================================================================================
-- 来源: 1239_CREATE AUDIT POLICY.txt
================================================================================

-- [DDL]
CREATE USER dev_audit PASSWORD '*********' ;

-- [DDL]
CREATE USER bob_audit PASSWORD '*********' ;

-- [DDL]
CREATE TABLE tb_for_audit ( col1 text , col2 text , col3 text );

-- [DDL]
CREATE RESOURCE LABEL adt_lb0 ADD TABLE ( tb_for_audit );

-- [DDL]
CREATE AUDIT POLICY adt1 PRIVILEGES CREATE ;

-- [DDL]
CREATE AUDIT POLICY adt2 ACCESS SELECT ;

-- [DDL]
CREATE AUDIT POLICY adt3 PRIVILEGES CREATE ON LABEL ( adt_lb0 ) FILTER ON ROLES ( dev_audit , bob_audit );

-- [DDL]
CREATE AUDIT POLICY adt4 ACCESS SELECT ON LABEL ( adt_lb0 ), INSERT ON LABEL ( adt_lb0 ), DELETE FILTER ON ROLES ( dev_audit , bob_audit ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' );

-- [DDL]
ALTER AUDIT POLICY adt4 REMOVE ACCESS ( SELECT ON LABEL ( adt_lb0 ));

-- [DDL]
DROP AUDIT POLICY adt1 , adt2 , adt3 , adt4 ;

-- [DDL]
DROP RESOURCE LABEL adt_lb0 ;

-- [DDL]
DROP TABLE tb_for_audit ;

-- [DDL]
DROP USER dev_audit , bob_audit ;


================================================================================
-- 来源: 1240_CREATE BARRIER.txt
================================================================================

-- [DDL]
CREATE BARRIER 'barrier1' ;


================================================================================
-- 来源: 1242_CREATE DATABASE.txt
================================================================================

-- [DDL]
CREATE USER jim PASSWORD '********';

--创建一个GBK编码的数据库testdb1。
-- [DDL]
CREATE DATABASE testdb1 ENCODING 'GBK' template = template0;

--查看数据库testdb1信息。
-- [DQL]
SELECT datname,pg_encoding_to_char(encoding) FROM pg_database WHERE datname = 'testdb1';

-- [DDL]
CREATE DATABASE testdb2 OWNER jim DBCOMPATIBILITY = 'ORA';

--查看testdb2信息。
-- [DQL]
SELECT t1.datname,t2.usename,t1.datcompatibility FROM pg_database t1,pg_user t2 WHERE t1.datname = 'testdb2' AND t1.datdba=t2.usesysid;

-- [SESSION]
SET a_format_version='10c';

-- [SESSION]
SET a_format_dev_version='s2';

--创建兼容ORA格式的数据库并指定时区。
-- [DDL]
CREATE DATABASE testdb3 DBCOMPATIBILITY 'ORA' DBTIMEZONE='+08:00';

--查看testdb3信息。
-- [DQL]
SELECT datname,datcompatibility,dattimezone FROM pg_database WHERE datname = 'testdb3';

-- [DDL]
DROP DATABASE testdb1;

-- [DDL]
DROP DATABASE testdb2;

-- [DDL]
DROP DATABASE testdb3;


================================================================================
-- 来源: 1243_CREATE DATABASE LINK.txt
================================================================================

-- [DDL]
CREATE USER user01 WITH SYSADMIN PASSWORD '********';

-- [SESSION]
SET ROLE user01 PASSWORD '********';

-- [DDL]
CREATE DATABASE LINK private_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

-- [DDL]
DROP DATABASE LINK private_dblink;

-- [OTHER]
CREATE PUBLIC DATABASE LINK public_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

-- [OTHER]
DROP PUBLIC DATABASE LINK public_dblink;

-- [SESSION]
RESET ROLE;

-- [DDL]
DROP USER user01 CASCADE;


================================================================================
-- 来源: 1244_CREATE DATA SOURCE.txt
================================================================================

-- [DDL]
CREATE DATA SOURCE ds_test1 ;

-- [DDL]
CREATE DATA SOURCE ds_test2 TYPE 'MPPDB' VERSION NULL ;

-- [DDL]
CREATE DATA SOURCE ds_test3 OPTIONS ( dsn 'GaussDB' , encoding 'utf8' );

-- [DDL]
CREATE DATA SOURCE ds_test4 TYPE 'unknown' VERSION '11.2.3' OPTIONS ( dsn 'GaussDB' , username 'userid' , password '********' , encoding '' );

-- [DDL]
DROP DATA SOURCE ds_test1 ;

-- [DDL]
DROP DATA SOURCE ds_test2 ;

-- [DDL]
DROP DATA SOURCE ds_test3 ;

-- [DDL]
DROP DATA SOURCE ds_test4 ;


================================================================================
-- 来源: 1245_CREATE DIRECTORY.txt
================================================================================

-- [DDL]
CREATE OR REPLACE DIRECTORY dir AS '/tmp/' ;

-- [DDL]
DROP DIRECTORY dir ;


================================================================================
-- 来源: 1246_CREATE EXTENSION.txt
================================================================================

-- [DDL]
CREATE EXTENSION IF NOT EXISTS security_plugin;


================================================================================
-- 来源: 1247_CREATE FOREIGN TABLE ().txt
================================================================================

-- [DDL]
CREATE FOREIGN TABLE foreign_HR_staffS ( staff_ID NUMBER ( 6 ) , FIRST_NAME VARCHAR2 ( 20 ), LAST_NAME VARCHAR2 ( 25 ), EMAIL VARCHAR2 ( 25 ), PHONE_NUMBER VARCHAR2 ( 20 ), HIRE_DATE DATE , employment_ID VARCHAR2 ( 10 ), SALARY NUMBER ( 8 , 2 ), COMMISSION_PCT NUMBER ( 2 , 2 ), MANAGER_ID NUMBER ( 6 ), section_ID NUMBER ( 4 ) ) SERVER gsmpp_server OPTIONS ( location 'gsfs://192.168.0.90:5000/* | gsfs://192.168.0.91:5000/*' , format 'TEXT' , delimiter E '\x20' , null '' ) WITH err_HR_staffS ;

-- [DDL]
CREATE FOREIGN TABLE foreign_HR_staffS_ft3 ( staff_ID NUMBER ( 6 ) , FIRST_NAME VARCHAR2 ( 20 ), LAST_NAME VARCHAR2 ( 25 ), EMAIL VARCHAR2 ( 25 ), PHONE_NUMBER VARCHAR2 ( 20 ), HIRE_DATE DATE , employment_ID VARCHAR2 ( 10 ), SALARY NUMBER ( 8 , 2 ), COMMISSION_PCT NUMBER ( 2 , 2 ), MANAGER_ID NUMBER ( 6 ), section_ID NUMBER ( 4 ) ) SERVER gsmpp_server OPTIONS ( location 'gsfs://192.168.0.90:5000/* | gsfs://192.168.0.91:5000/*' , format 'TEXT' , delimiter E '\x20' , null '' , reject_limit '2' ) WITH err_HR_staffS_ft3 ;

-- [DDL]
CREATE FOREIGN TABLE foreign_HR_staffS_ft1 ( staff_ID NUMBER ( 6 ) , FIRST_NAME VARCHAR2 ( 20 ), LAST_NAME VARCHAR2 ( 25 ), EMAIL VARCHAR2 ( 25 ), PHONE_NUMBER VARCHAR2 ( 20 ), HIRE_DATE DATE , employment_ID VARCHAR2 ( 10 ), SALARY NUMBER ( 8 , 2 ), COMMISSION_PCT NUMBER ( 2 , 2 ), MANAGER_ID NUMBER ( 6 ), section_ID NUMBER ( 4 ) ) SERVER gsmpp_server OPTIONS ( location 'file:///input_data/*' , format 'csv' , mode 'private' , delimiter ',' ) WITH err_HR_staffS_ft1 ;

-- [DDL]
CREATE FOREIGN TABLE foreign_HR_staffS_ft2 ( staff_ID NUMBER ( 6 ) , FIRST_NAME VARCHAR2 ( 20 ), LAST_NAME VARCHAR2 ( 25 ), EMAIL VARCHAR2 ( 25 ), PHONE_NUMBER VARCHAR2 ( 20 ), HIRE_DATE DATE , employment_ID VARCHAR2 ( 10 ), SALARY NUMBER ( 8 , 2 ), COMMISSION_PCT NUMBER ( 2 , 2 ), MANAGER_ID NUMBER ( 6 ), section_ID NUMBER ( 4 ) ) SERVER gsmpp_server OPTIONS ( location 'file:///output_data/' , format 'csv' , delimiter '|' , header 'on' ) WRITE ONLY ;

-- [DDL]
DROP FOREIGN TABLE foreign_HR_staffS ;

-- [DDL]
DROP FOREIGN TABLE foreign_HR_staffS_ft1 ;

-- [DDL]
DROP FOREIGN TABLE foreign_HR_staffS_ft2 ;

-- [DDL]
DROP FOREIGN TABLE foreign_HR_staffS_ft3 ;


================================================================================
-- 来源: 1248_CREATE FUNCTION.txt
================================================================================

-- [DDL]
CREATE DATABASE ora_compatible_db DBCOMPATIBILITY 'ORA' ;

-- [OTHER]
\ c ora_compatible_db --定义函数为SQL查询。

-- [DDL]
CREATE FUNCTION func_add_sql ( integer , integer ) RETURNS integer AS 'select $1 + $2;

-- [DDL]
CREATE OR REPLACE FUNCTION func_increment_plsql ( i integer ) RETURNS integer AS $$ BEGIN RETURN i + 1 ;

-- [DDL]
CREATE OR REPLACE FUNCTION compute ( i int , out result_1 bigint , out result_2 bigint ) RETURNS SETOF RECORD AS $$ BEGIN result_1 = i + 1 ;

-- [DDL]
CREATE FUNCTION func_dup_sql ( in int , out f1 int , out f2 text ) AS $$ SELECT $ 1 , CAST ( $ 1 AS text ) || ' is text' $$ LANGUAGE SQL ;

-- [DQL]
SELECT * FROM func_dup_sql ( 42 );

-- [DDL]
CREATE FUNCTION func_add_sql2 ( num1 integer , num2 integer ) RETURN integer AS BEGIN RETURN num1 + num2 ;

-- [DDL]
ALTER FUNCTION func_add_sql2 ( INTEGER , INTEGER ) IMMUTABLE ;

-- [DDL]
ALTER FUNCTION func_add_sql2 ( INTEGER , INTEGER ) RENAME TO add_two_number ;

-- [DDL]
CREATE USER jim PASSWORD '********' ;

-- [DDL]
ALTER FUNCTION add_two_number ( INTEGER , INTEGER ) OWNER TO jim ;

-- [DDL]
DROP FUNCTION func_add_sql ;

-- [DDL]
DROP FUNCTION func_increment_plsql ;

-- [DDL]
DROP FUNCTION compute ;

-- [DDL]
DROP FUNCTION func_dup_sql ;

-- [DDL]
DROP FUNCTION add_two_number ;

-- [DDL]
DROP USER jim ;

-- [DDL]
CREATE TYPE rec AS ( c1 int , c2 int );

-- [DDL]
CREATE OR REPLACE FUNCTION func ( a in out rec , b in out int ) RETURN int AS BEGIN a . c1 : = 100 ;

-- [PLSQL]
DECLARE r rec ;

-- [DDL]
DROP FUNCTION func ;

-- [DDL]
DROP TYPE rec ;

-- [DDL]
CREATE OR REPLACE FUNCTION func_001 ( a in out date , b in out date ) --#add in & inout #defult value RETURN integer AS BEGIN raise info '%' , a ;

-- [PLSQL]
DECLARE date1 date : = '2022-02-02' ;

-- [DDL]
CREATE OR REPLACE FUNCTION func_001 ( a in out INT , b in out date ) --#add in & inout #defult value RETURN INT AS BEGIN raise info '%' , a ;

-- [PLSQL]
DECLARE date1 int : = 1 ;

-- [DDL]
DROP FUNCTION func_001 ;


================================================================================
-- 来源: 1249_CREATE GLOBAL INDEX.txt
================================================================================

-- [DDL]
CREATE TABLE test1 ( c1 int , c2 int , c3 int );

-- [DDL]
CREATE GLOBAL INDEX idx_gsi_1 ON test1 ( c2 ) CONTAINING ( c3 ) DISTRIBUTE BY HASH ( c2 );

-- [DDL]
CREATE TABLE test2 ( c1 int , c2 int , c3 int );

-- [DDL]
CREATE GLOBAL INDEX idx_gsi_2 ON test2 ( c2 ) CONTAINING ( c3 ) ;

-- [DDL]
CREATE TABLE test3 ( c1 int , c2 int , c3 int );

-- [DDL]
CREATE GLOBAL UNIQUE INDEX idx_gsi_3 ON test3 ( c2 ) DISTRIBUTE BY HASH ( c2 );

-- [DDL]
DROP INDEX idx_gsi_1 ;

-- [DDL]
DROP INDEX idx_gsi_2 ;

-- [DDL]
DROP INDEX idx_gsi_3 ;

-- [DDL]
DROP TABLE test1 ;

-- [DDL]
DROP TABLE test2 ;

-- [DDL]
DROP TABLE test3 ;


================================================================================
-- 来源: 1250_CREATE GROUP.txt
================================================================================

-- [DDL]
CREATE GROUP my_group PASSWORD '********';

--删除组。
-- [DDL]
DROP GROUP my_group;


================================================================================
-- 来源: 1251_CREATE INCREMENTAL MATERIALIZED VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE my_table (c1 int, c2 int);

--创建增量物化视图。
-- [DDL]
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--基表写入数据。
-- [DML_INSERT]
INSERT INTO my_table VALUES(1,1),(2,2);

--对增量物化视图my_imv进行增量刷新。
-- [OTHER]
REFRESH INCREMENTAL MATERIALIZED VIEW my_imv;

--删除增量物化视图。
-- [DDL]
DROP MATERIALIZED VIEW my_imv;

--删除普通表my_table。
-- [DDL]
DROP TABLE my_table;


================================================================================
-- 来源: 1252_CREATE INDEX.txt
================================================================================

-- [DDL]
CREATE TABLE tbl_test1( id int, --用户id name varchar(50), --用户姓名 postcode char(6) --邮编 );

--创建表空间tbs_index1。
-- [DDL]
CREATE TABLESPACE tbs_index1 RELATIVE LOCATION 'test_tablespace/tbs_index1';

--为表tbl_test1创建索引idx_test1指定表空间。
-- [DDL]
CREATE INDEX idx_test1 ON tbl_test1(name) TABLESPACE tbs_index1;

--查询索引idx_test1信息。
-- [DQL]
SELECT indexname,tablename,tablespace FROM pg_indexes WHERE indexname = 'idx_test1';

--删除索引。
-- [DDL]
DROP INDEX idx_test1;

--删除表空间
-- [DDL]
DROP TABLESPACE tbs_index1;

-- [DDL]
CREATE UNIQUE INDEX idx_test2 ON tbl_test1(id);

--删除索引。
-- [DDL]
DROP INDEX idx_test2;

-- [DDL]
CREATE INDEX idx_test3 ON tbl_test1(substr(postcode,2));

--删除索引。
-- [DDL]
DROP INDEX idx_test3;

-- [DDL]
CREATE INDEX idx_test4 ON tbl_test1(id) WHERE id IS NOT NULL;

-- 删除索引。
-- [DDL]
DROP INDEX idx_test4;

-- 删除表
-- [DDL]
DROP TABLE tbl_test1;

-- [DDL]
CREATE TABLE student(id int, name va(20)) PARTITION BY RANGE (id) ( PARTITION p1 VALUES LESS THAN (200), PARTITION pmax VALUES LESS THAN (MAXVALUE) );

--创建LOCAL分区索引不指定索引分区的名称。
-- [DDL]
CREATE INDEX idx_student1 ON student(id) LOCAL;

--查看索引分区信息，LOCAL索引分区数和表的分区数一致。
-- [DQL]
SELECT relname FROM pg_partition WHERE parentid = 'idx_student1'::regclass;

--删除LOCAL分区索引。
-- [DDL]
DROP INDEX idx_student1;

--创建GLOBAL索引。
-- [DDL]
CREATE INDEX idx_student2 ON student(name) GLOBAL;

--查看索引分区信息，GLOBAL索引分区数和表的分区数不一致。
-- [DQL]
SELECT relname FROM pg_partition WHERE parentid = 'idx_student2'::regclass;

--删除GLOBAL分区索引。
-- [DDL]
DROP INDEX idx_student2;

--删除表。
-- [DDL]
DROP TABLE student;


================================================================================
-- 来源: 1254_CREATE MASKING POLICY.txt
================================================================================

-- [DDL]
CREATE USER dev_mask PASSWORD '********' ;

-- [DDL]
CREATE USER bob_mask PASSWORD '********' ;

-- [DDL]
CREATE TABLE tb_for_masking ( idx int , col1 text , col2 text , col3 text , col4 text , col5 text , col6 text , col7 text , col8 text );

-- [DML_INSERT]
INSERT INTO tb_for_masking VALUES ( 1 , '9876543210' , 'usr321usr' , 'abc@huawei.com' , 'abc@huawei.com' , '1234-4567-7890-0123' , 'abcdef 123456 ui 323 jsfd321 j3k2l3' , '4880-9898-4545-2525' , 'this is a llt case' );

-- [DML_INSERT]
INSERT INTO tb_for_masking VALUES ( 2 , '0123456789' , 'lltc123llt' , 'abc@gmail.com' , 'abc@gmail.com' , '9876-5432-1012-3456' , '1234 abcd ef 56 gh78ijk90lm' , '4856-7654-1234-9865' , 'this,is.a!LLT?case' );

-- [DDL]
CREATE RESOURCE LABEL mask_lb1 ADD COLUMN ( tb_for_masking . col1 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb2 ADD COLUMN ( tb_for_masking . col2 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb3 ADD COLUMN ( tb_for_masking . col3 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb4 ADD COLUMN ( tb_for_masking . col4 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb5 ADD COLUMN ( tb_for_masking . col5 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb6 ADD COLUMN ( tb_for_masking . col6 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb7 ADD COLUMN ( tb_for_masking . col7 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb8 ADD COLUMN ( tb_for_masking . col8 );

-- [DDL]
CREATE MASKING POLICY maskpol1 maskall ON LABEL ( mask_lb1 );

-- [DDL]
CREATE MASKING POLICY maskpol2 alldigitsmasking ON LABEL ( mask_lb2 );

-- [DDL]
CREATE MASKING POLICY maskpol3 basicemailmasking ON LABEL ( mask_lb3 );

-- [DDL]
CREATE MASKING POLICY maskpol4 fullemailmasking ON LABEL ( mask_lb4 );

-- [DDL]
CREATE MASKING POLICY maskpol5 creditcardmasking ON LABEL ( mask_lb5 );

-- [DDL]
CREATE MASKING POLICY maskpol6 shufflemasking ON LABEL ( mask_lb6 );

-- [DDL]
CREATE MASKING POLICY maskpol7 regexpmasking ( '[\d+]' , '*' , 2 , 9 ) ON LABEL ( mask_lb7 );

-- [DDL]
CREATE MASKING POLICY maskpol8 randommasking ON LABEL ( mask_lb8 ) FILTER ON ROLES ( dev_mask , bob_mask ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' );

-- [DQL]
SELECT * FROM tb_for_masking ;

-- [DCL_GRANT]
GRANT ALL PRIVILEGES TO dev_mask ;

-- [DCL_GRANT]
GRANT ALL PRIVILEGES TO bob_mask ;

-- [SESSION]
SET role dev_mask PASSWORD '********' ;

-- [DQL]
SELECT col8 FROM tb_for_masking ;

-- [SESSION]
SET role bob_mask PASSWORD '********' ;

-- [DQL]
SELECT col8 FROM tb_for_masking ;

-- [DDL]
DROP MASKING POLICY maskpol1 , maskpol2 , maskpol3 , maskpol4 , maskpol5 , maskpol6 , maskpol7 , maskpol8 ;

-- [DDL]
DROP RESOURCE LABEL mask_lb1 , mask_lb2 , mask_lb3 , mask_lb4 , mask_lb5 , mask_lb6 , mask_lb7 , mask_lb8 ;

-- [DDL]
DROP TABLE tb_for_masking ;

-- [DDL]
DROP USER dev_mask , bob_mask ;


================================================================================
-- 来源: 1255_CREATE MATERIALIZED VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE my_table (c1 int, c2 int);

--创建全量物化视图。
-- [DDL]
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--基表写入数据。
-- [DML_INSERT]
INSERT INTO my_table VALUES(1,1),(2,2);

--对全量物化视图my_mv进行全量刷新。
-- [OTHER]
REFRESH MATERIALIZED VIEW my_mv;

--删除全量物化视图。
-- [DDL]
DROP MATERIALIZED VIEW my_mv;

--删除普通表my_table。
-- [DDL]
DROP TABLE my_table;


================================================================================
-- 来源: 1257_CREATE NODE.txt
================================================================================

-- [DDL]
CREATE NODE datanode1 WITH( TYPE = datanode, PREFERRED = false );

-- [DDL]
CREATE NODE datanode2 WITH( TYPE = datanode, PREFERRED = false );

-- 查询集群DN初始状态。
-- [DQL]
SELECT node_name, nodeis_preferred FROM pgxc_node WHERE node_type = 'D' ORDER BY 1;

-- 将datanode1设为preferred DN。
-- [DDL]
ALTER NODE datanode1 WITH(preferred = true);

-- 查询集群DN变更后状态。
-- [DQL]
SELECT node_name, nodeis_preferred FROM pgxc_node WHERE node_type = 'D' ORDER BY 1;

-- 删除集群节点。
-- [DDL]
DROP NODE datanode1;

-- [DDL]
DROP NODE datanode2;


================================================================================
-- 来源: 1258_CREATE NODE GROUP.txt
================================================================================

-- [DQL]
SELECT node_name, nodeis_preferred FROM pgxc_node WHERE node_type = 'D' ORDER BY 1;

-- 创建node group，用上一步中查询到的真实节点名称替换dn_6001_6002_6003。
-- [DDL]
CREATE NODE GROUP test_group WITH ( dn_6001_6002_6003 );

-- 查询node group。
-- [DQL]
SELECT group_name, group_members FROM pgxc_group;

-- 删除node group。
-- [DDL]
DROP NODE GROUP test_group;


================================================================================
-- 来源: 1259_CREATE PACKAGE.txt
================================================================================

-- [DDL]
CREATE DATABASE ora_compat_db DBCOMPATIBILITY 'ORA';

-- [DDL]
CREATE OR REPLACE PACKAGE emp_bonus IS var1 INT:=1;

-- [DDL]
DROP PACKAGE emp_bonus;

-- [DDL]
DROP TABLE IF EXISTS test1;

--创建包头
-- [DDL]
CREATE OR REPLACE PACKAGE emp_bonus IS var1 INT:=1;

--创建包体
-- [DDL]
CREATE OR REPLACE PACKAGE BODY emp_bonus IS var3 INT:=3;

-- [DDL]
ALTER PACKAGE emp_bonus OWNER TO omm;

--将PACKAGE emp_bonus的所属者改为omm 调用PACKAGE示例
-- [PLSQL]
CALL emp_bonus.testpro1(1);

-- [DDL]
DROP TABLE IF EXISTS test1;

-- [DQL]
SELECT emp_bonus.testpro1(1);

-- [DDL]
DROP TABLE IF EXISTS test1;

--匿名块里调用package存储过程
-- [TCL]
BEGIN emp_bonus.testpro1(1);

-- [DDL]
DROP TABLE IF EXISTS test1;

--删除PACKAGE。
-- [DDL]
DROP PACKAGE emp_bonus;

--删除数据库。
-- [DDL]
DROP DATABASE ora_compat_db;


================================================================================
-- 来源: 1260_CREATE PROCEDURE.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE prc_add ( param1 IN INTEGER , param2 IN OUT INTEGER ) AS BEGIN param2 : = param1 + param2 ;

-- [DQL]
SELECT prc_add ( 2 , 3 );

-- [DDL]
CREATE OR REPLACE PROCEDURE pro_variadic ( var1 VARCHAR2 ( 10 ) DEFAULT 'hello!' , var4 VARIADIC int4 []) AS BEGIN dbe_output . print_line ( var1 );

-- [DQL]
SELECT pro_variadic ( var1 => 'hello' , VARIADIC var4 => array [ 1 , 2 , 3 , 4 ]);

-- [DDL]
CREATE TABLE tb1 ( a integer );

-- [DDL]
CREATE PROCEDURE insert_data ( v integer ) SECURITY INVOKER AS BEGIN INSERT INTO tb1 VALUES ( v );

-- [PLSQL]
CALL insert_data ( 1 );

-- [DDL]
DROP PROCEDURE prc_add ;

-- [DDL]
DROP PROCEDURE pro_variadic ;

-- [DDL]
DROP PROCEDURE insert_data ;

-- [DDL]
DROP TABLE tb1 ;


================================================================================
-- 来源: 1261_CREATE RESOURCE LABEL.txt
================================================================================

-- [DDL]
CREATE TABLE tb_for_label ( col1 text , col2 text , col3 text );

-- [DDL]
CREATE SCHEMA schema_for_label ;

-- [DDL]
CREATE VIEW view_for_label AS SELECT 1 ;

-- [DDL]
CREATE FUNCTION func_for_label RETURNS TEXT AS $$ SELECT col1 FROM tb_for_label ;

-- [DDL]
CREATE RESOURCE LABEL IF NOT EXISTS table_label add TABLE ( public . tb_for_label );

-- [DDL]
CREATE RESOURCE LABEL IF NOT EXISTS column_label add COLUMN ( public . tb_for_label . col1 );

-- [DDL]
CREATE RESOURCE LABEL IF NOT EXISTS schema_label add SCHEMA ( schema_for_label );

-- [DDL]
CREATE RESOURCE LABEL IF NOT EXISTS view_label add VIEW ( view_for_label );

-- [DDL]
CREATE RESOURCE LABEL IF NOT EXISTS func_label add FUNCTION ( func_for_label );

-- [DDL]
DROP RESOURCE LABEL func_label , view_label , schema_label , column_label , table_label ;

-- [DDL]
DROP FUNCTION func_for_label ;

-- [DDL]
DROP VIEW view_for_label ;

-- [DDL]
DROP SCHEMA schema_for_label ;

-- [DDL]
DROP TABLE tb_for_label ;


================================================================================
-- 来源: 1262_CREATE RESOURCE POOL.txt
================================================================================

-- [DDL]
CREATE RESOURCE POOL pool1 ;

-- [DDL]
CREATE RESOURCE POOL pool2 WITH ( CONTROL_GROUP = "High" );

-- [DDL]
CREATE RESOURCE POOL pool3 WITH ( CONTROL_GROUP = "class1:Low" );

-- [DDL]
CREATE RESOURCE POOL pool4 WITH ( CONTROL_GROUP = "class1:wg1" );

-- [DDL]
CREATE RESOURCE POOL pool5 WITH ( CONTROL_GROUP = "class1:wg2:3" );

-- [DDL]
DROP RESOURCE POOL pool1 ;

-- [DDL]
DROP RESOURCE POOL pool2 ;

-- [DDL]
DROP RESOURCE POOL pool3 ;

-- [DDL]
DROP RESOURCE POOL pool4 ;

-- [DDL]
DROP RESOURCE POOL pool5 ;


================================================================================
-- 来源: 1263_CREATE ROLE.txt
================================================================================

-- [DDL]
CREATE ROLE manager IDENTIFIED BY '********' ;

-- [DDL]
CREATE ROLE miriam WITH LOGIN PASSWORD '********' VALID BEGIN '2015-01-01' VALID UNTIL '2026-01-01' ;

-- [DDL]
ALTER ROLE manager IDENTIFIED BY '**********' REPLACE '********' ;

-- [DDL]
ALTER ROLE manager SYSADMIN ;

-- [DDL]
DROP ROLE manager ;

-- [DDL]
DROP GROUP miriam ;


================================================================================
-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY.txt
================================================================================

-- [DDL]
CREATE USER alice PASSWORD '*********' ;

-- [DDL]
CREATE USER bob PASSWORD '*********' ;

-- [DDL]
CREATE TABLE public . all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

-- [DML_INSERT]
INSERT INTO all_data VALUES ( 1 , 'alice' , 'alice data' );

-- [DML_INSERT]
INSERT INTO all_data VALUES ( 2 , 'bob' , 'bob data' );

-- [DML_INSERT]
INSERT INTO all_data VALUES ( 3 , 'peter' , 'peter data' );

-- [DCL_GRANT]
GRANT SELECT ON all_data TO alice , bob ;

-- [DDL]
ALTER TABLE all_data ENABLE ROW LEVEL SECURITY ;

-- [OTHER]
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

-- [OTHER]
\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_rls" FOR ALL TO public USING ((( role ):: name = "current_user" ())) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --当前用户执行SELECT操作。

-- [DQL]
SELECT * FROM all_data ;

-- [DDL]
ALTER USER alice LOGIN ;

-- [EXPLAIN]
EXPLAIN ( COSTS OFF ) SELECT * FROM all_data ;

-- [DQL]
SELECT * FROM all_data ;

-- [EXPLAIN]
EXPLAIN ( COSTS OFF ) SELECT * FROM all_data ;

-- [OTHER]
DROP ROW LEVEL SECURITY POLICY all_data_rls ON all_data ;

-- [DDL]
DROP TABLE public . all_data ;

-- [DDL]
DROP USER alice , bob ;


================================================================================
-- 来源: 1265_CREATE SCHEMA.txt
================================================================================

-- [DDL]
CREATE ROLE role1 IDENTIFIED BY '********' ;

-- [DDL]
CREATE SCHEMA AUTHORIZATION role1 CREATE TABLE films ( title text , release date , awards text []) CREATE VIEW winners AS SELECT title , release FROM films WHERE awards IS NOT NULL ;

-- [DDL]
DROP SCHEMA role1 CASCADE ;

-- [DDL]
DROP USER role1 CASCADE ;


================================================================================
-- 来源: 1266_CREATE SECURITY LABEL.txt
================================================================================

-- [DDL]
CREATE SECURITY LABEL sec_label 'L1:G4' ;

-- [DDL]
DROP SECURITY LABEL sec_label ;


================================================================================
-- 来源: 1267_CREATE SEQUENCE.txt
================================================================================

-- [DDL]
CREATE SEQUENCE seq1 START 101 INCREMENT 10 ;

-- [DQL]
SELECT nextval ( 'seq1' );

-- [DQL]
SELECT nextval ( 'seq1' );

-- [DDL]
DROP SEQUENCE seq1 ;

-- [DDL]
CREATE TABLE test1 ( id int PRIMARY KEY , name varchar ( 20 ));

-- [DDL]
CREATE SEQUENCE test_seq2 START 1 NO CYCLE OWNED BY test1 . id ;

-- [DDL]
ALTER TABLE test1 ALTER COLUMN id SET DEFAULT nextval ( 'test_seq2' :: regclass );

-- [DML_INSERT]
INSERT INTO test1 ( name ) values ( 'Joe' ),( 'Scott' ),( 'Ben' );

-- [DQL]
SELECT * FROM test1 ;

-- [DDL]
DROP SEQUENCE test_seq2 CASCADE ;

-- [DDL]
DROP TABLE test1 ;


================================================================================
-- 来源: 1268_CREATE SERVER.txt
================================================================================

-- [DDL]
CREATE SERVER my_server FOREIGN DATA WRAPPER file_fdw ;

-- [DDL]
DROP SERVER my_server ;

-- [DDL]
CREATE SERVER server_remote FOREIGN DATA WRAPPER GC_FDW OPTIONS ( address '10.146.187.231:8000,10.180.157.130:8000' , dbname 'test' , username 'test' , password '********' );

-- [DDL]
DROP SERVER server_remote ;


================================================================================
-- 来源: 1269_CREATE SYNONYM.txt
================================================================================

-- [DDL]
CREATE SCHEMA ot ;

-- [DDL]
CREATE TABLE ot . t1 ( id int , name varchar2 ( 10 )) DISTRIBUTE BY hash ( id );

-- [DDL]
CREATE OR REPLACE SYNONYM t1 FOR ot . t1 ;

-- [DQL]
SELECT * FROM t1 ;

-- [DML_INSERT]
INSERT INTO t1 VALUES ( 1 , 'ada' ), ( 2 , 'bob' );

-- [DML_UPDATE]
UPDATE t1 SET t1 . name = 'cici' WHERE t1 . id = 2 ;

-- [DDL]
CREATE SYNONYM v1 FOR ot . v_t1 ;

-- [DDL]
CREATE VIEW ot . v_t1 AS SELECT * FROM ot . t1 ;

-- [DQL]
SELECT * FROM v1 ;

-- [DDL]
CREATE OR REPLACE FUNCTION ot . add ( a integer , b integer ) RETURNS integer AS $$ SELECT $ 1 + $ 2 $$ LANGUAGE sql ;

-- [DDL]
CREATE OR REPLACE FUNCTION ot . add ( a decimal ( 5 , 2 ), b decimal ( 5 , 2 )) RETURNS decimal ( 5 , 2 ) AS $$ SELECT $ 1 + $ 2 $$ LANGUAGE sql ;

-- [DDL]
CREATE OR REPLACE SYNONYM add FOR ot . add ;

-- [DQL]
SELECT add ( 1 , 2 );

-- [DQL]
SELECT add ( 1 . 2 , 2 . 3 );

-- [DDL]
CREATE PROCEDURE ot . register ( n_id integer , n_name varchar2 ( 10 )) SECURITY INVOKER AS BEGIN INSERT INTO ot . t1 VALUES ( n_id , n_name );

-- [DDL]
CREATE OR REPLACE SYNONYM register FOR ot . register ;

-- [PLSQL]
CALL register ( 3 , 'mia' );

-- [DDL]
DROP SYNONYM t1 ;

-- [DDL]
DROP SYNONYM IF EXISTS v1 ;

-- [DDL]
DROP SYNONYM IF EXISTS add ;

-- [DDL]
DROP SYNONYM register ;

-- [DDL]
DROP SCHEMA ot CASCADE ;


================================================================================
-- 来源: 1270_CREATE TABLE.txt
================================================================================

-- [DQL]
SELECT a.count,b.node_name FROM (SELECT count(*) AS count,xc_node_id FROM tablename GROUP BY xc_node_id) a, pgxc_node b WHERE a.xc_node_id=b.node_id ORDER BY a.count DESC;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t1 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- [DDL]
CREATE TABLE tpcds . warehouse_t2 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ), W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- [DDL]
DROP TABLE tpcds . warehouse_t2 ;

-- [DDL]
DROP TABLE tpcds . warehouse_t1 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t3 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) DEFAULT 'GA' , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- [DDL]
DROP TABLE tpcds . warehouse_t3 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t4 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) UNIQUE DEFERRABLE , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- [DDL]
DROP TABLE tpcds . warehouse_t4 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t5 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), UNIQUE ( W_WAREHOUSE_NAME ) WITH ( fillfactor = 70 ) );

-- [DDL]
DROP TABLE tpcds . warehouse_t5 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t6 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) UNIQUE , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) WITH ( fillfactor = 70 );

-- [DDL]
DROP TABLE tpcds . warehouse_t6 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE UNLOGGED TABLE tpcds . warehouse_t7 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_

-- [DDL]
DROP TABLE tpcds . warehouse_t7 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE TEMPORARY TABLE warehouse_t24 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- [DDL]
CREATE TEMPORARY TABLE warehouse_t25 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) ON COMMIT DELETE ROWS ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE IF NOT EXISTS tpcds . warehouse_t8 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- [DDL]
DROP TABLE tpcds . warehouse_t8 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE TABLESPACE DS_TABLESPACE1 RELATIVE LOCATION 'tablespace/tablespace_1' ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t9 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) TABLESPACE DS_TABLESPACE1 ;

-- [DDL]
DROP TABLE tpcds . warehouse_t9 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t10 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) UNIQUE USING INDEX TABLESPACE DS_TABLESPACE1 , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- [DDL]
DROP TABLE tpcds . warehouse_t10 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t11 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- [DDL]
DROP TABLE tpcds . warehouse_t11 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t12 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), PRIMARY KEY ( W_WAREHOUSE_SK ) );

-- [DDL]
DROP TABLE tpcds . warehouse_t12 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t13 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT W_CSTR_KEY1 PRIMARY KEY ( W_WAREHOUSE_SK ) );

-- [DDL]
DROP TABLE tpcds . warehouse_t13 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t13_1 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT PRIMARY KEY USING BTREE ( W_WAREHOUSE_SK DESC ) );

-- [DDL]
DROP TABLE tpcds . warehouse_t13_1 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t14 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT W_CSTR_KEY2 PRIMARY KEY ( W_WAREHOUSE_SK , W_WAREHOUSE_ID ) );

-- [DDL]
DROP TABLE tpcds . warehouse_t14 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t19 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY CHECK ( W_WAREHOUSE_SK > 0 ), W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) CHECK ( W_WAREHOUSE_NAME IS NOT NULL ), W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- [DDL]
CREATE TABLE tpcds . warehouse_t20 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) CHECK ( W_WAREHOUSE_NAME IS NOT NULL ), W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT W_CONSTR_KEY2 CHECK ( W_WAREHOUSE_SK > 0 AND W_WAREHOUSE_NAME IS NOT NULL ) );

-- [DDL]
ALTER TABLE tpcds . warehouse_t19 ADD W_GOODS_CATEGORY varchar ( 30 );

-- [DDL]
ALTER TABLE tpcds . warehouse_t19 ADD CONSTRAINT W_CONSTR_KEY4 CHECK ( W_STATE IS NOT NULL );

-- [DDL]
ALTER TABLE tpcds . warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY TYPE varchar ( 80 ), ALTER COLUMN W_STREET_NAME TYPE varchar ( 100 );

-- [DDL]
ALTER TABLE tpcds . warehouse_t19 MODIFY ( W_GOODS_CATEGORY varchar ( 30 ), W_STREET_NAME varchar ( 60 ));

-- [DDL]
ALTER TABLE tpcds . warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY SET NOT NULL ;

-- [DDL]
ALTER TABLE tpcds . warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY DROP NOT NULL ;

-- [DDL]
ALTER TABLE tpcds . warehouse_t19 SET TABLESPACE PG_DEFAULT ;

-- [DDL]
CREATE SCHEMA joe ;

-- [DDL]
ALTER TABLE tpcds . warehouse_t19 SET SCHEMA joe ;

-- [DDL]
ALTER TABLE joe . warehouse_t19 RENAME TO warehouse_t23 ;

-- [DDL]
ALTER TABLE joe . warehouse_t23 DROP COLUMN W_STREET_NAME ;

-- [DDL]
DROP TABLESPACE DS_TABLESPACE1 ;

-- [DDL]
DROP SCHEMA IF EXISTS joe CASCADE ;

-- [DDL]
DROP TABLE tpcds . warehouse_t20 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t21 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) DISTRIBUTE BY REPLICATION ;

-- [DDL]
ALTER TABLE tpcds . warehouse_t21 SET ( primarynode = on );

-- [OTHER]
\ d + tpcds . warehouse_t21 Table "tpcds.warehouse_t21" Column | Type | Modifiers | Storage | Stats target | Description -------------------+-----------------------+-----------+----------+--------------+------------- w_warehouse_sk | integer | not null | plain | | w_warehouse_id | character ( 16 ) | not null | extended | | w_warehouse_name | character varying ( 20 ) | | extended | | w_warehouse_sq_ft | integer | | plain | | w_street_number | character ( 10 ) | | extended | | w_street_name | character varying ( 60 ) | | extended | | w_street_type | character ( 15 ) | | extended | | w_suite_number | character ( 10 ) | | extended | | w_city | character varying ( 60 ) | | extended | | w_county | character varying ( 30 ) | | extended | | w_state | character ( 2 ) | | extended | | w_zip | character ( 10 ) | | extended | | w_country | character varying ( 20 ) | | extended | | w_gmt_offset | numeric ( 5 , 2 ) | | main | | Has OIDs : no Distribute By : REPLICATION Location Nodes : ALL DATANODES Options : orientation = row , logical_repl_node =- 1 , compression = no , primarynode = on

-- [DDL]
DROP TABLE tpcds . warehouse_t21 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t22 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT W_CONSTR_KEY3 UNIQUE ( W_WAREHOUSE_SK ) ) DISTRIBUTE BY HASH ( W_WAREHOUSE_SK );

-- [DDL]
DROP TABLE tpcds . warehouse_t22 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DQL]
SELECT node_name FROM pgxc_node ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t26 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) DISTRIBUTE BY RANGE ( W_WAREHOUSE_ID ) ( SLICE s1 VALUES LESS THAN ( 10 ) DATANODE datanode1 , SLICE s2 VALUES LESS THAN ( 20 ) DATANODE datanode2 , SLICE s3 VALUES LESS THAN ( 30 ) DATANODE datanode3 , SLICE s4 VALUES LESS THAN ( MAXVALUE ) DATANODE datanode4 );

-- [DDL]
DROP TABLE tpcds . warehouse_t26 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE TABLE lrt_range ( f_int1 int , f_int2 int , f_varchar1 varchar2 ( 100 )) distribute by range ( f_int1 , f_int2 ) ( slice s1 values less than ( 100 , 100 ) datanode ( datanode1 , datanode2 ), slice s2 values less than ( 200 , 200 ) datanode datanode2 , slice s3 values less than ( 300 , 300 ) datanode datanode2 , slice s4 values less than ( maxvalue , maxvalue ) datanode ( datanode1 , datanode2 ) );

-- [DML_INSERT]
INSERT INTO lrt_range VALUES ( generate_series ( 1 , 4 ), generate_series ( 1 , 4 ));

-- [DQL]
SELECT node_name , node_type , node_id FROM pgxc_node ;

-- [DQL]
SELECT xc_node_id , * FROM lrt_range ;

-- [DDL]
CREATE TABLE t_news ( county varchar ( 30 ), year varchar ( 60 ), name varchar ( 60 ), age int , news text ) distribute by list ( county , year ) ( slice s1 values (( 'china' , '2020' ),( 'china' , '2021' )) datanode ( datanode1 , datanode2 ), slice s2 values (( 'china' , '2022' ),( 'china' , '2023' ),( 'china' , '2024' )) datanode ( datanode1 , datanode2 ), slice s3 values (( 'china' , '2025' )) datanode ( datanode1 , datanode2 ), slice s4 values (( 'canada' , '2021' )) datanode datanode1 , slice s5 values (( 'canada' , '2022' )) datanode datanode2 , slice s6 values (( 'canada' , '2023' )) datanode datanode1 , slice s7 values (( 'uk' , '2021' )) datanode datanode1 , slice s8 values (( 'uk' , '2022' )) datanode datanode2 , slice s9 values (( 'uk' , '2023' )) datanode datanode1 , slice s0 values ( default ) datanode ( datanode1 , datanode2 ) );

-- [DML_INSERT]
INSERT INTO t_news values ( 'china' , '2020' , '张三' , 21 );

-- [DML_INSERT]
INSERT INTO t_news values ( 'china' , '2021' , '张三' , 21 );

-- [DML_INSERT]
INSERT INTO t_news values ( 'china' , '2022' , '张三' , 21 );

-- [DML_INSERT]
INSERT INTO t_news values ( 'china' , '2023' , '张三' , 21 );

-- [DML_INSERT]
INSERT INTO t_news values ( 'china' , '2024' , '张三' , 21 );

-- [DML_INSERT]
INSERT INTO t_news values ( 'china' , '2025' , '张三' , 21 );

-- [DQL]
SELECT node_name , node_type , node_id FROM pgxc_node ;

-- [DQL]
SELECT xc_node_id , * FROM t_news ;

-- [DML_DELETE]
DELETE FROM t_news ;

-- [DML_INSERT]
INSERT INTO t_news values ( 'Japan' , '2020' , '赵六' , 18 ),( 'Japan' , '2021' , '赵六' , 19 ),( 'Japan' , '2022' , '赵六' , 20 ),( 'Japan' , '2027' , '赵六' , 21 );

-- [DQL]
SELECT xc_node_id , * FROM t_news ;

-- [DDL]
CREATE TABLE t_ran1 ( c1 int , c2 int , c3 int , c4 int , c5 int ) distribute by range ( c1 , c2 ) ( SLICE s1 VALUES LESS THAN ( 10 , 10 ) DATANODE datanode1 , SLICE s2 VALUES LESS THAN ( 10 , 20 ) DATANODE datanode2 , SLICE s3 VALUES LESS THAN ( 20 , 10 ) DATANODE datanode3 );

-- [DML_INSERT]
INSERT INTO t_ran1 values ( 9 , 5 , 'a' );

-- [DML_INSERT]
INSERT INTO t_ran1 values ( 9 , 20 , 'a' );

-- [DML_INSERT]
INSERT INTO t_ran1 values ( 9 , 21 , 'a' );

-- [DML_INSERT]
INSERT INTO t_ran1 values ( 10 , 5 , 'a' );

-- [DML_INSERT]
INSERT INTO t_ran1 values ( 10 , 15 , 'a' );

-- [DML_INSERT]
INSERT INTO t_ran1 values ( 10 , 20 , 'a' );

-- [DML_INSERT]
INSERT INTO t_ran1 values ( 10 , 21 , 'a' );

-- [DML_INSERT]
INSERT INTO t_ran1 values ( 11 , 5 , 'a' );

-- [DML_INSERT]
INSERT INTO t_ran1 values ( 11 , 20 , 'a' );

-- [DML_INSERT]
INSERT INTO t_ran1 values ( 11 , 21 , 'a' );

-- [DQL]
SELECT node_name , node_type , node_id FROM pgxc_node ;

-- [DQL]
SELECT xc_node_id , * FROM t_ran1 ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t27 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) DISTRIBUTE BY RANGE ( W_WAREHOUSE_ID ) SLICE REFERENCES warehouse_t26 ;

-- [DDL]
DROP TABLE tpcds . warehouse_t27 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . warehouse_t28 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) DISTRIBUTE BY LIST ( W_COUNTRY ) ( SLICE s1 VALUES ( 'USA' ) DATANODE datanode1 , SLICE s2 VALUES ( 'CANADA' ) DATANODE datanode2 , SLICE s3 VALUES ( 'UK' ) DATANODE datanode3 , SLICE s4 VALUES ( DEFAULT ) DATANODE datanode4 );

-- [DDL]
DROP TABLE tpcds . warehouse_t28 ;

-- [DDL]
DROP SCHEMA tpcds ;

-- [DDL]
CREATE TABLE creditcard_info ( id_number int , name text encrypted with ( column_encryption_key = ImgCEK , encryption_type = DETERMINISTIC ), credit_card varchar ( 19 ) encrypted with ( column_encryption_key = ImgCEK1 , encryption_type = DETERMINISTIC ));

-- [SESSION]
show server_encoding ;

-- [SESSION]
show sql_compatibility ;

-- [DDL]
CREATE TABLE t1 ( c1 text , c2 text charset utf8mb4 collate utf8mb4_unicode_ci ) charset utf8mb4 collate utf8mb4_bin ;

-- [DDL]
ALTER TABLE t1 charset utf8mb4 collate utf8mb4_general_ci ;

-- [DDL]
ALTER TABLE t1 add c3 varchar ( 20 ) charset utf8mb4 collate utf8mb4_bin ;

-- [SESSION]
SET b_format_version = '5.7' ;

-- [SESSION]
SET b_format_dev_version = 's1' ;

-- [DDL]
CREATE TABLE t1_on_update ( TS0 TIMESTAMP ON UPDATE CURRENT_TIMESTAMP , TS1 TIMESTAMP ON UPDATE CURRENT_TIMESTAMP () , TS2 TIMESTAMP ( 6 ) ON UPDATE CURRENT_TIMESTAMP ( 6 ) , DT0 DATETIME ON UPDATE LOCALTIMESTAMP , DT1 DATETIME ON UPDATE NOW () , IN0 INT ) DISTRIBUTE BY HASH ( IN0 );

-- [DDL]
ALTER TABLE t1_on_update ADD TS3 timestamp ON UPDATE CURRENT_TIMESTAMP ;

-- [DDL]
CREATE DATABASE ilmtabledb with dbcompatibility = 'ORA' ;

-- [OTHER]
\ c ilmtabledb

-- [DDL]
ALTER DATABASE ilmtabledb TO GROUP GROUP1 ;

-- [DDL]
CREATE TABLE ilm_table ( a int ) WITH ( STORAGE_TYPE = ASTORE ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION ON ( a != 0 );


================================================================================
-- 来源: 1271_CREATE TABLESPACE.txt
================================================================================

-- [DDL]
CREATE TABLESPACE ds_location1 RELATIVE LOCATION 'test_tablespace/test_tablespace_1' ;

-- [DDL]
CREATE ROLE joe IDENTIFIED BY '********' ;

-- [DDL]
CREATE ROLE jay IDENTIFIED BY '********' ;

-- [DDL]
CREATE TABLESPACE ds_location2 OWNER joe RELATIVE LOCATION 'test_tablespace/test_tablespace_2' ;

-- [DDL]
ALTER TABLESPACE ds_location1 RENAME TO ds_location3 ;

-- [DDL]
ALTER TABLESPACE ds_location2 OWNER TO jay ;

-- [DDL]
DROP TABLESPACE ds_location2 ;

-- [DDL]
DROP TABLESPACE ds_location3 ;

-- [DDL]
DROP ROLE joe ;

-- [DDL]
DROP ROLE jay ;


================================================================================
-- 来源: 1272_CREATE TABLE AS.txt
================================================================================

-- [DDL]
CREATE TABLE test1(col1 int PRIMARY KEY,col2 varchar(10));

-- [DML_INSERT]
INSERT INTO test1 VALUES (1,'col1'),(101,'col101');

-- 查询表中col1<100的数据。
-- [DQL]
SELECT * FROM test1 WHERE col1 < 100;

-- 创建test2表并向表中插入上面查询的数据。
-- [DDL]
CREATE TABLE test2 AS SELECT * FROM test1 WHERE col1 < 100;

-- [DDL]
CREATE TABLE test3(c1,c2) AS SELECT * FROM test1;

-- 删除。
-- [DDL]
DROP TABLE test1,test2,test3;

-- [DDL]
CREATE DATABASE ilmtabledb WITH dbcompatibility = 'ORA' ;

-- [OTHER]
\ c ilmtabledb --开启数据库ILM特性

-- [DDL]
ALTER DATABASE SET ILM = on ;

-- [DDL]
CREATE TABLE old_table ( a int );

-- [DDL]
CREATE TABLE ilm_table ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION AS ( SELECT * FROM old_table );

-- [DDL]
DROP TABLE old_table , ilm_table ;

-- [OTHER]
\ c postgres

-- [DDL]
DROP DATABASE ilmtabledb ;


================================================================================
-- 来源: 1273_CREATE TABLE PARTITION.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [SESSION]
SET CURRENT_SCHEMA TO tpcds ;

-- [DDL]
CREATE TABLE tpcds . web_returns ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- [DDL]
CREATE TABLE tpcds . web_returns_p1 ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL , WR_REFUNDED_CUSTOMER_SK INTEGER , WR_REFUNDED_CDEMO_SK INTEGER , WR_REFUNDED_HDEMO_SK INTEGER , WR_REFUNDED_ADDR_SK INTEGER , WR_RETURNING_CUSTOMER_SK INTEGER , WR_RETURNING_CDEMO_SK INTEGER , WR_RETURNING_HDEMO_SK INTEGER , WR_RETURNING_ADDR_SK INTEGER , WR_WEB_PAGE_SK INTEGER , WR_REASON_SK INTEGER , WR_ORDER_NUMBER BIGINT NOT NULL , WR_RETURN_QUANTITY INTEGER , WR_RETURN_AMT DECIMAL ( 7 , 2 ) , WR_RETURN_TAX DECIMAL ( 7 , 2 ) , WR_RETURN_AMT_INC_TAX DECIMAL ( 7 , 2 ) , WR_FEE DECIMAL ( 7 , 2 ) , WR_RETURN_SHIP_COST DECIMAL ( 7 , 2 ) , WR_REFUNDED_CASH DECIMAL ( 7 , 2 ) , WR_REVERSED_CHARGE DECIMAL ( 7 , 2 ) , WR_ACCOUNT_CREDIT DECIMAL ( 7 , 2 ) , WR_NET_LOSS DECIMAL ( 7 , 2 ) ) DISTRIBUTE BY HASH ( WR_ITEM_SK ) PARTITION BY RANGE ( WR_RETURNED_DATE_SK ) ( PARTITION P1 VALUES LESS THAN ( 2450815 ), PARTITION P2 VALUES LESS THAN ( 2451179 ), PARTITION P3 VALUES LESS THAN ( 2451544 ), PARTITION P4 VALUES LESS THAN ( 2451910 ), PARTITION P5 VALUES LESS THAN ( 2452275 ), PARTITION P6 VALUES LESS THAN ( 2452640 ), PARTITION P7 VALUES LESS THAN ( 2453005 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) );

-- [DML_INSERT]
INSERT INTO tpcds . web_returns_p1 SELECT * FROM tpcds . web_returns ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p1 DROP PARTITION P8 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p1 ADD PARTITION P8 VALUES LESS THAN ( 2453105 );

-- [DDL]
ALTER TABLE tpcds . web_returns_p1 ADD PARTITION P9 VALUES LESS THAN ( MAXVALUE );

-- [DDL]
ALTER TABLE tpcds . web_returns_p1 DROP PARTITION FOR ( 2453005 );

-- [DDL]
ALTER TABLE tpcds . web_returns_p1 RENAME PARTITION P7 TO P10 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p1 RENAME PARTITION FOR ( 2452639 ) TO P11 ;

-- [DQL]
SELECT count ( * ) FROM tpcds . web_returns_p1 PARTITION ( P10 );

-- [DQL]
SELECT COUNT ( * ) FROM tpcds . web_returns_p1 PARTITION FOR ( 2450815 );

-- [DDL]
DROP TABLE tpcds . web_returns_p1 ;

-- [DDL]
DROP TABLE tpcds . web_returns ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;

-- [DDL]
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1/tablespace_1' ;

-- [DDL]
CREATE TABLESPACE example2 RELATIVE LOCATION 'tablespace2/tablespace_2' ;

-- [DDL]
CREATE TABLESPACE example3 RELATIVE LOCATION 'tablespace3/tablespace_3' ;

-- [DDL]
CREATE TABLESPACE example4 RELATIVE LOCATION 'tablespace4/tablespace_4' ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [SESSION]
SET CURRENT_SCHEMA TO tpcds ;

-- [DDL]
CREATE TABLE tpcds . web_returns_p2 ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL , WR_REFUNDED_CUSTOMER_SK INTEGER , WR_REFUNDED_CDEMO_SK INTEGER , WR_REFUNDED_HDEMO_SK INTEGER , WR_REFUNDED_ADDR_SK INTEGER , WR_RETURNING_CUSTOMER_SK INTEGER , WR_RETURNING_CDEMO_SK INTEGER , WR_RETURNING_HDEMO_SK INTEGER , WR_RETURNING_ADDR_SK INTEGER , WR_WEB_PAGE_SK INTEGER , WR_REASON_SK INTEGER , WR_ORDER_NUMBER BIGINT NOT NULL , WR_RETURN_QUANTITY INTEGER , WR_RETURN_AMT DECIMAL ( 7 , 2 ) , WR_RETURN_TAX DECIMAL ( 7 , 2 ) , WR_RETURN_AMT_INC_TAX DECIMAL ( 7 , 2 ) , WR_FEE DECIMAL ( 7 , 2 ) , WR_RETURN_SHIP_COST DECIMAL ( 7 , 2 ) , WR_REFUNDED_CASH DECIMAL ( 7 , 2 ) , WR_REVERSED_CHARGE DECIMAL ( 7 , 2 ) , WR_ACCOUNT_CREDIT DECIMAL ( 7 , 2 ) , WR_NET_LOSS DECIMAL ( 7 , 2 ) ) TABLESPACE example1 DISTRIBUTE BY HASH ( WR_ITEM_SK ) PARTITION BY RANGE ( WR_RETURNED_DATE_SK ) ( PARTITION P1 VALUES LESS THAN ( 2450815 ), PARTITION P2 VALUES LESS THAN ( 2451179 ), PARTITION P3 VALUES LESS THAN ( 2451544 ), PARTITION P4 VALUES LESS THAN ( 2451910 ), PARTITION P5 VALUES LESS THAN ( 2452275 ), PARTITION P6 VALUES LESS THAN ( 2452640 ), PARTITION P7 VALUES LESS THAN ( 2453005 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) TABLESPACE example2 ) ENABLE ROW MOVEMENT ;

-- [DDL]
CREATE TABLE tpcds . web_returns_p3 ( LIKE tpcds . web_returns_p2 INCLUDING PARTITION );

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P1 TABLESPACE example2 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P2 TABLESPACE example3 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 SPLIT PARTITION P8 AT ( 2453010 ) INTO ( PARTITION P9 , PARTITION P10 );

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 MERGE PARTITIONS P6 , P7 INTO PARTITION P8 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 DISABLE ROW MOVEMENT ;

-- [DDL]
DROP TABLE tpcds . web_returns_p1 ;

-- [DDL]
DROP TABLE tpcds . web_returns_p2 ;

-- [DDL]
DROP TABLE tpcds . web_returns_p3 ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;

-- [DDL]
DROP TABLESPACE example1 ;

-- [DDL]
DROP TABLESPACE example2 ;

-- [DDL]
DROP TABLESPACE example3 ;

-- [DDL]
DROP TABLESPACE example4 ;

-- [DDL]
CREATE TABLESPACE startend_tbs1 LOCATION '/home/omm/startend_tbs1' ;

-- [DDL]
CREATE TABLESPACE startend_tbs2 LOCATION '/home/omm/startend_tbs2' ;

-- [DDL]
CREATE TABLESPACE startend_tbs3 LOCATION '/home/omm/startend_tbs3' ;

-- [DDL]
CREATE TABLESPACE startend_tbs4 LOCATION '/home/omm/startend_tbs4' ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [SESSION]
SET CURRENT_SCHEMA TO tpcds ;

-- [DDL]
CREATE TABLE tpcds . startend_pt ( c1 INT , c2 INT ) TABLESPACE startend_tbs1 DISTRIBUTE BY HASH ( c1 ) PARTITION BY RANGE ( c2 ) ( PARTITION p1 START ( 1 ) END ( 1000 ) EVERY ( 200 ) TABLESPACE startend_tbs2 , PARTITION p2 END ( 2000 ), PARTITION p3 START ( 2000 ) END ( 2500 ) TABLESPACE startend_tbs3 , PARTITION p4 START ( 2500 ), PARTITION p5 START ( 3000 ) END ( 5000 ) EVERY ( 1000 ) TABLESPACE startend_tbs4 ) ENABLE ROW MOVEMENT ;

-- [DQL]
SELECT relname , boundaries , spcname FROM pg_partition p JOIN pg_tablespace t ON p . reltablespace = t . oid and p . parentid = 'tpcds.startend_pt' :: regclass ORDER BY 1 ;

-- [DML_INSERT]
INSERT INTO tpcds . startend_pt VALUES ( GENERATE_SERIES ( 0 , 4999 ), GENERATE_SERIES ( 0 , 4999 ));

-- [DQL]
SELECT COUNT ( * ) FROM tpcds . startend_pt PARTITION FOR ( 0 );

-- [DQL]
SELECT COUNT ( * ) FROM tpcds . startend_pt PARTITION ( p3 );

-- [DDL]
ALTER TABLE tpcds . startend_pt ADD PARTITION p6 START ( 5000 ) END ( 6000 ) EVERY ( 300 ) TABLESPACE startend_tbs4 ;

-- [DDL]
ALTER TABLE tpcds . startend_pt ADD PARTITION p7 END ( MAXVALUE );

-- [DDL]
ALTER TABLE tpcds . startend_pt RENAME PARTITION p7 TO p8 ;

-- [DDL]
ALTER TABLE tpcds . startend_pt DROP PARTITION p8 ;

-- [DDL]
ALTER TABLE tpcds . startend_pt RENAME PARTITION FOR ( 5950 ) TO p71 ;

-- [DDL]
ALTER TABLE tpcds . startend_pt SPLIT PARTITION FOR ( 4500 ) INTO ( PARTITION q1 START ( 4000 ) END ( 5000 ) EVERY ( 250 ) TABLESPACE startend_tbs3 );

-- [DDL]
ALTER TABLE tpcds . startend_pt MOVE PARTITION p2 TABLESPACE startend_tbs4 ;

-- [DQL]
SELECT relname , boundaries , spcname FROM pg_partition p JOIN pg_tablespace t ON p . reltablespace = t . oid and p . parentid = 'tpcds.startend_pt' :: regclass ORDER BY 1 ;

-- [DDL]
DROP TABLE tpcds . startend_pt ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;

-- [DDL]
DROP TABLESPACE startend_tbs1 ;

-- [DDL]
DROP TABLESPACE startend_tbs2 ;

-- [DDL]
DROP TABLESPACE startend_tbs3 ;

-- [DDL]
DROP TABLESPACE startend_tbs4 ;

-- [DDL]
CREATE TABLE test_list ( col1 int , col2 int ) partition by list ( col1 ) ( partition p1 values ( 2000 ), partition p2 values ( 3000 ), partition p3 values ( 4000 ), partition p4 values ( 5000 ) );

-- [DML_INSERT]
INSERT INTO test_list VALUES ( 2000 , 2000 );

-- [DML_INSERT]
INSERT INTO test_list VALUES ( 3000 , 3000 );

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- [DML_INSERT]
INSERT INTO test_list VALUES ( 6000 , 6000 );

-- [DDL]
ALTER TABLE test_list add partition p5 values ( 6000 );

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- [DML_INSERT]
INSERT INTO test_list VALUES ( 6000 , 6000 );

-- [DDL]
CREATE TABLE t1 ( col1 int , col2 int );

-- [DQL]
SELECT * FROM test_list partition ( p1 );

-- [DDL]
ALTER TABLE test_list exchange partition ( p1 ) with table t1 ;

-- [DQL]
SELECT * FROM test_list partition ( p1 );

-- [DQL]
SELECT * FROM t1 ;

-- [DQL]
SELECT * FROM test_list partition ( p2 );

-- [DDL]
ALTER TABLE test_list truncate partition p2 ;

-- [DQL]
SELECT * FROM test_list partition ( p2 );

-- [DDL]
alter table test_list drop partition p5 ;

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- [DML_INSERT]
INSERT INTO test_list VALUES ( 6000 , 6000 );

-- [DDL]
alter table test_list merge partitions p1 , p2 into partition p2 ;

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- [DDL]
alter table test_list split partition p2 values ( 2000 ) into ( partition p1 , partition p2 );

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- [DDL]
drop table test_list ;

-- [DDL]
DROP TABLE t1 ;

-- [DDL]
create table test_hash ( col1 int , col2 int ) partition by hash ( col1 ) ( partition p1 , partition p2 );

-- [DML_INSERT]
INSERT INTO test_hash VALUES ( 1 , 1 );

-- [DML_INSERT]
INSERT INTO test_hash VALUES ( 2 , 2 );

-- [DML_INSERT]
INSERT INTO test_hash VALUES ( 3 , 3 );

-- [DML_INSERT]
INSERT INTO test_hash VALUES ( 4 , 4 );

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_hash' AND t1 . parttype = 'p' ;

-- [DQL]
select * from test_hash partition ( p1 );

-- [DQL]
select * from test_hash partition ( p2 );

-- [DDL]
create table t1 ( col1 int , col2 int );

-- [DDL]
alter table test_hash exchange partition ( p1 ) with table t1 ;

-- [DQL]
select * from test_hash partition ( p1 );

-- [DQL]
select * from t1 ;

-- [DDL]
alter table test_hash truncate partition p2 ;

-- [DQL]
select * from test_hash partition ( p2 );

-- [DDL]
drop table test_hash ;

-- [DDL]
CREATE TABLE t_multi_keys_list ( a int , b varchar ( 4 ), c int ) PARTITION BY LIST ( a , b ) ( PARTITION p1 VALUES ( ( 0 , NULL ) ), PARTITION p2 VALUES ( ( 0 , '1' ), ( 0 , '2' ), ( 0 , '3' ), ( 1 , '1' ), ( 1 , '2' ) ), PARTITION p3 VALUES ( ( NULL , '0' ), ( 2 , '1' ) ), PARTITION p4 VALUES ( ( 3 , '2' ), ( NULL , NULL ) ), PARTITION pd VALUES ( DEFAULT ) );

-- [DDL]
DROP TABLE t_multi_keys_list ;

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_part ( a int ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION , PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- [DDL]
DROP TABLE ilm_part ;

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- [DDL]
DROP TABLE ilm_part ;

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- [DDL]
DROP TABLE ilm_part ;

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- [DDL]
DROP TABLE ilm_part ;

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- [DDL]
DROP TABLE ilm_part ;


================================================================================
-- 来源: 1274_CREATE TEXT SEARCH CONFIGURATION.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH CONFIGURATION ngram2 ( parser = ngram ) WITH ( gram_size = 2 , grapsymbol_ignore = false );

-- [DDL]
CREATE TEXT SEARCH CONFIGURATION ngram3 ( copy = ngram2 ) WITH ( gram_size = 2 , grapsymbol_ignore = false );

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION ngram2 ADD MAPPING FOR multisymbol WITH simple ;

-- [DDL]
CREATE USER joe IDENTIFIED BY '********' ;

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION ngram2 OWNER TO joe ;

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION ngram2 SET SCHEMA joe ;

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION joe . ngram2 RENAME TO ngram_2 ;

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION joe . ngram_2 DROP MAPPING IF EXISTS FOR multisymbol ;

-- [DDL]
DROP TEXT SEARCH CONFIGURATION joe . ngram_2 ;

-- [DDL]
DROP TEXT SEARCH CONFIGURATION ngram3 ;

-- [DDL]
DROP SCHEMA IF EXISTS joe CASCADE ;

-- [DDL]
DROP ROLE IF EXISTS joe ;


================================================================================
-- 来源: 1276_CREATE TRIGGER.txt
================================================================================

-- [DDL]
CREATE TABLE test_trigger_src_tbl ( id1 INT , id2 INT , id3 INT );

-- [DDL]
CREATE TABLE test_trigger_des_tbl ( id1 INT , id2 INT , id3 INT );

-- [DDL]
CREATE OR REPLACE FUNCTION tri_insert_func () RETURNS TRIGGER AS $$ DECLARE BEGIN INSERT INTO test_trigger_des_tbl VALUES ( NEW . id1 , NEW . id2 , NEW . id3 );

-- [DDL]
CREATE OR REPLACE FUNCTION tri_update_func () RETURNS TRIGGER AS $$ DECLARE BEGIN UPDATE test_trigger_des_tbl SET id3 = NEW . id3 WHERE id1 = OLD . id1 ;

-- [DDL]
CREATE OR REPLACE FUNCTION tri_delete_func () RETURNS TRIGGER AS $$ DECLARE BEGIN DELETE FROM test_trigger_des_tbl WHERE id1 = OLD . id1 ;

-- [DDL]
CREATE TRIGGER insert_trigger BEFORE INSERT ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_insert_func ();

-- [DDL]
CREATE TRIGGER update_trigger AFTER UPDATE ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_update_func ();

-- [DDL]
CREATE TRIGGER delete_trigger BEFORE DELETE ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_delete_func ();

-- [DML_INSERT]
INSERT INTO test_trigger_src_tbl VALUES ( 100 , 200 , 300 );

-- [DQL]
SELECT * FROM test_trigger_src_tbl ;

-- [DQL]
SELECT * FROM test_trigger_des_tbl ;

-- [DML_UPDATE]
UPDATE test_trigger_src_tbl SET id3 = 400 WHERE id1 = 100 ;

-- [DQL]
SELECT * FROM test_trigger_src_tbl ;

-- [DQL]
SELECT * FROM test_trigger_des_tbl ;

-- [DML_DELETE]
DELETE FROM test_trigger_src_tbl WHERE id1 = 100 ;

-- [DQL]
SELECT * FROM test_trigger_src_tbl ;

-- [DQL]
SELECT * FROM test_trigger_des_tbl ;

-- [DDL]
ALTER TRIGGER delete_trigger ON test_trigger_src_tbl RENAME TO delete_trigger_renamed ;

-- [DDL]
ALTER TABLE test_trigger_src_tbl DISABLE TRIGGER insert_trigger ;

-- [DDL]
ALTER TABLE test_trigger_src_tbl DISABLE TRIGGER ALL ;

-- [DDL]
DROP TRIGGER insert_trigger ON test_trigger_src_tbl ;

-- [DDL]
DROP TRIGGER update_trigger ON test_trigger_src_tbl ;

-- [DDL]
DROP TRIGGER delete_trigger_renamed ON test_trigger_src_tbl ;

-- [DDL]
DROP FUNCTION tri_insert_func ();

-- [DDL]
DROP FUNCTION tri_update_func ();

-- [DDL]
DROP FUNCTION tri_delete_func ();

-- [DDL]
DROP TABLE test_trigger_src_tbl ;

-- [DDL]
DROP TABLE test_trigger_des_tbl ;


================================================================================
-- 来源: 1277_CREATE TYPE.txt
================================================================================

-- [DDL]
CREATE TYPE compfoo AS ( f1 int , f2 text );

-- [DDL]
CREATE TABLE t1_compfoo ( a int , b compfoo );

-- [DDL]
CREATE TABLE t2_compfoo ( a int , b compfoo );

-- [DML_INSERT]
INSERT INTO t1_compfoo values ( 1 ,( 1 , 'demo' ));

-- [DML_INSERT]
INSERT INTO t2_compfoo select * from t1_compfoo ;

-- [DQL]
SELECT ( b ). f1 FROM t1_compfoo ;

-- [DQL]
SELECT * FROM t1_compfoo t1 join t2_compfoo t2 on ( t1 . b ). f1 = ( t1 . b ). f1 ;

-- [DDL]
ALTER TYPE compfoo RENAME TO compfoo1 ;

-- [DDL]
CREATE USER usr1 PASSWORD '********' ;

-- [DDL]
ALTER TYPE compfoo1 OWNER TO usr1 ;

-- [DDL]
ALTER TYPE compfoo1 SET SCHEMA usr1 ;

-- [DDL]
ALTER TYPE usr1 . compfoo1 ADD ATTRIBUTE f3 int ;

-- [DDL]
DROP TYPE usr1 . compfoo1 CASCADE ;

-- [DDL]
DROP TABLE t1_compfoo ;

-- [DDL]
DROP TABLE t2_compfoo ;

-- [DDL]
DROP SCHEMA usr1 ;

-- [DDL]
DROP USER usr1 ;

-- [DDL]
CREATE TYPE bugstatus AS ENUM ( 'create' , 'modify' , 'closed' );

-- [DDL]
ALTER TYPE bugstatus ADD VALUE IF NOT EXISTS 'regress' BEFORE 'closed' ;

-- [DDL]
ALTER TYPE bugstatus RENAME VALUE 'create' TO 'new' ;

-- [DDL]
CREATE TYPE bugstatus_table AS TABLE OF bugstatus ;

-- [DDL]
CREATE TYPE complex ;

-- [DDL]
CREATE FUNCTION complex_in ( cstring ) RETURNS complex AS 'filename' LANGUAGE C IMMUTABLE STRICT not fenced ;

-- [DDL]
CREATE FUNCTION complex_out ( complex ) RETURNS cstring AS 'filename' LANGUAGE C IMMUTABLE STRICT not fenced ;

-- [DDL]
CREATE FUNCTION complex_recv ( internal ) RETURNS complex AS 'filename' LANGUAGE C IMMUTABLE STRICT not fenced ;

-- [DDL]
CREATE FUNCTION complex_send ( complex ) RETURNS bytea AS 'filename' LANGUAGE C IMMUTABLE STRICT not fenced ;

-- [DDL]
CREATE TYPE complex ( internallength = 16 , input = complex_in , output = complex_out , receive = complex_recv , send = complex_send , alignment = double );

-- [DDL]
DROP TYPE complex ;

-- [DDL]
DROP FUNCTION complex_send ;

-- [DDL]
DROP FUNCTION complex_recv ;

-- [DDL]
DROP FUNCTION complex_out ;

-- [DDL]
DROP FUNCTION complex_in ;

-- [DDL]
DROP TYPE bugstatus_table ;

-- [DDL]
DROP TYPE bugstatus CASCADE ;


================================================================================
-- 来源: 1278_CREATE USER.txt
================================================================================

-- [DDL]
CREATE USER jim PASSWORD '********';

--下面语句与上面的等价。
-- [DDL]
CREATE USER kim IDENTIFIED BY '********';

--如果创建有“创建数据库”权限的用户，则需要加CREATEDB关键字。
-- [DDL]
CREATE USER dim CREATEDB PASSWORD '********';

--将用户jim的登录密码由********修改为**********。
-- [DDL]
ALTER USER jim IDENTIFIED BY '**********' REPLACE '********';

--为用户jim追加CREATEROLE权限。
-- [DDL]
ALTER USER jim CREATEROLE;

--锁定jim账户。
-- [DDL]
ALTER USER jim ACCOUNT LOCK;

--删除用户。
-- [DDL]
DROP USER kim CASCADE;

-- [DDL]
DROP USER jim CASCADE;

-- [DDL]
DROP USER dim CASCADE;


================================================================================
-- 来源: 1279_CREATE VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE test_tb1(col1 int, col2 int);

-- [DML_INSERT]
INSERT INTO test_tb1 VALUES (generate_series(1,100),generate_series(1,100));

--创建一个col1小于5的视图。
-- [DDL]
CREATE VIEW test_v1 AS SELECT * FROM test_tb1 WHERE col1 < 3;

--查看视图。
-- [DQL]
SELECT * FROM test_v1;

--删除表和视图。
-- [DDL]
DROP VIEW test_v1;

-- [DDL]
DROP TABLE test_tb1;

-- [DDL]
CREATE TABLE test_tb2(c1 int, c2 int);

-- [DDL]
CREATE TEMP VIEW test_v2 AS SELECT * FROM test_tb2;

--删除视图和表。
-- [DDL]
DROP VIEW test_v2 ;

-- [DDL]
DROP TABLE test_tb2;


================================================================================
-- 来源: 1280_CREATE WORKLOAD GROUP.txt
================================================================================

-- [DDL]
CREATE WORKLOAD GROUP wg_name1 ;

-- [DDL]
CREATE RESOURCE POOL pool1 ;

-- [DDL]
CREATE WORKLOAD GROUP wg_name2 USING RESOURCE POOL pool1 ;

-- [DDL]
CREATE WORKLOAD GROUP wg_name3 USING RESOURCE POOL pool1 WITH ( ACT_STATEMENTS = 10 );

-- [DDL]
DROP WORKLOAD GROUP wg_name1 ;

-- [DDL]
DROP WORKLOAD GROUP wg_name2 ;

-- [DDL]
DROP WORKLOAD GROUP wg_name3 ;

-- [DDL]
DROP RESOURCE POOL pool1 ;


================================================================================
-- 来源: 1281_CREATE WEAK PASSWORD DICTIONARY.txt
================================================================================

-- [DDL]
CREATE WEAK PASSWORD DICTIONARY WITH VALUES ('********');

--向gs_global_config系统表中插入多个弱口令。
-- [DDL]
CREATE WEAK PASSWORD DICTIONARY WITH VALUES ('********'),('********');

--清空gs_global_config系统表中所有弱口令。
-- [DDL]
DROP WEAK PASSWORD DICTIONARY;

--查看现有弱口令。
-- [DQL]
SELECT * FROM gs_global_config WHERE NAME LIKE 'weak_password';


================================================================================
-- 来源: 1284_DEALLOCATE.txt
================================================================================

-- [DQL]
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- [PREPARED_STMT]
PREPARE q1 AS SELECT 1 AS a ;

-- [PREPARED_STMT]
PREPARE q2 AS SELECT 1 AS a ;

-- [PREPARED_STMT]
PREPARE q3 AS SELECT 1 AS a ;

-- [DQL]
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- [PREPARED_STMT]
DEALLOCATE q1 ;

-- [DQL]
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- [PREPARED_STMT]
DEALLOCATE ALL ;

-- [DQL]
SELECT name , statement , parameter_types FROM pg_prepared_statements ;


================================================================================
-- 来源: 1286_DELETE.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . customer_address ( ca_address_sk INTEGER NOT NULL , ca_address_id CHARACTER ( 16 ) NOT NULL , ca_street_number INTEGER , ca_street_name CHARACTER ( 20 ) );

-- [DML_INSERT]
INSERT INTO tpcds . customer_address VALUES ( 1 , 'AAAAAAAABAAAAAAA' , '18' , 'Jackson' ),( 10000 , 'AAAAAAAACAAAAAAA' , '362' , 'Washington 6th' ),( 15000 , 'AAAAAAAADAAAAAAA' , '585' , 'Dogwood Washington' );

-- [DDL]
CREATE TABLE tpcds . customer_address_bak AS TABLE tpcds . customer_address ;

-- [DML_DELETE]
DELETE FROM tpcds . customer_address_bak WHERE ca_address_sk < 14888 ;

-- [DML_DELETE]
DELETE FROM tpcds . customer_address_bak ;

-- [DDL]
DROP TABLE tpcds . customer_address_bak ;

-- [DDL]
DROP TABLE tpcds . customer_address ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1287_DO.txt
================================================================================

-- [DDL]
CREATE USER webuser PASSWORD '********' ;

-- [PLSQL]
DO $$ DECLARE r record ;

-- [DDL]
DROP USER webuser CASCADE ;


================================================================================
-- 来源: 1292_DROP DATA SOURCE.txt
================================================================================

-- [DDL]
CREATE DATA SOURCE ds_tst1 ;

-- [DDL]
DROP DATA SOURCE ds_tst1 CASCADE ;

-- [DDL]
DROP DATA SOURCE IF EXISTS ds_tst1 RESTRICT ;


================================================================================
-- 来源: 1293_DROP DIRECTORY.txt
================================================================================

-- [DDL]
CREATE OR REPLACE DIRECTORY dir as '/tmp/' ;

-- [DDL]
DROP DIRECTORY dir ;


================================================================================
-- 来源: 1300_DROP MASKING POLICY.txt
================================================================================

-- [DDL]
DROP MASKING POLICY IF EXISTS maskpol1 ;

-- [DDL]
DROP MASKING POLICY IF EXISTS maskpol1 , maskpol2 , maskpol3 ;


================================================================================
-- 来源: 1301_DROP MATERIALIZED VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建名为my_mv的物化视图。
-- [DDL]
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--删除名为my_mv的物化视图。
-- [DDL]
DROP MATERIALIZED VIEW my_mv;

--删除表。
-- [DDL]
DROP TABLE my_table;


================================================================================
-- 来源: 1305_DROP OWNED.txt
================================================================================

-- [DDL]
CREATE USER jim PASSWORD '********' ;

-- [DDL]
DROP OWNED BY jim ;

-- [DDL]
DROP USER jim ;


================================================================================
-- 来源: 1306_DROP PACKAGE.txt
================================================================================

-- [DDL]
CREATE DATABASE ora_compat_db DBCOMPATIBILITY 'ORA';

--创建PACKAGE。
-- [DDL]
CREATE OR REPLACE PACKAGE emp_bonus IS var1 INT:=1;

--删除PACKAGE。
-- [DDL]
DROP PACKAGE emp_bonus;

-- [DDL]
DROP DATABASE ora_compat_db;


================================================================================
-- 来源: 1311_DROP ROW LEVEL SECURITY POLICY.txt
================================================================================

-- [DDL]
CREATE TABLE all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

-- [OTHER]
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

-- [OTHER]
DROP ROW LEVEL SECURITY POLICY all_data_rls ON all_data ;

-- [DDL]
DROP TABLE all_data ;


================================================================================
-- 来源: 1313_DROP SECURITY LABEL.txt
================================================================================

-- [DDL]
CREATE SECURITY LABEL sec_label 'L1:G4' ;

-- [DDL]
DROP SECURITY LABEL sec_label ;


================================================================================
-- 来源: 1314_DROP SEQUENCE.txt
================================================================================

-- [DDL]
CREATE SEQUENCE serial START 101 ;

-- [DDL]
DROP SEQUENCE serial ;


================================================================================
-- 来源: 1315_DROP SERVER.txt
================================================================================

-- [DDL]
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--删除my_server。
-- [DDL]
DROP SERVER my_server;


================================================================================
-- 来源: 1320_DROP TEXT SEARCH DICTIONARY.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH DICTIONARY english ( TEMPLATE = simple );

-- [DDL]
DROP TEXT SEARCH DICTIONARY english ;


================================================================================
-- 来源: 1328_EXECUTE.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . reason ( CD_DEMO_SK INTEGER NOT NULL , CD_GENDER character ( 16 ) , CD_MARITAL_STATUS character ( 100 ) );

-- [DML_INSERT]
INSERT INTO tpcds . reason VALUES ( 51 , 'AAAAAAAADDAAAAAA' , 'reason 51' );

-- [DDL]
CREATE TABLE tpcds . reason_t1 AS TABLE tpcds . reason ;

-- [PREPARED_STMT]
PREPARE insert_reason ( integer , character ( 16 ), character ( 100 )) AS INSERT INTO tpcds . reason_t1 VALUES ( $ 1 , $ 2 , $ 3 );

-- [PREPARED_STMT]
EXECUTE insert_reason ( 52 , 'AAAAAAAADDAAAAAA' , 'reason 52' );

-- [DDL]
DROP TABLE tpcds . reason ;

-- [DDL]
DROP TABLE tpcds . reason_t1 ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1329_EXECUTE DIRECT.txt
================================================================================

-- [DQL]
SELECT * FROM pgxc_node ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . customer_address ( ca_address_sk INTEGER NOT NULL , ca_address_id CHARACTER ( 16 ) NOT NULL );

-- [PREPARED_STMT]
EXECUTE DIRECT ON ( dn_6001_6002 ) 'select count(*) from tpcds.customer_address' ;

-- [DQL]
SELECT COUNT ( * ) FROM tpcds . customer_address ;

-- [DQL]
SELECT oid FROM pgxc_node where node_name = 'dn_6001_6002_6003' ;

-- [SESSION]
SET enable_direct_standby_datanodes = on ;

-- [PREPARED_STMT]
EXECUTE DIRECT ON ( 16385 , 16386 , 16384 ) 'SELECT * FROM gs_get_listen_address_ext_info();

-- [DDL]
DROP TABLE tpcds . customer_address ;

-- [DDL]
DROP SCHEMA tpcds ;


================================================================================
-- 来源: 1330_EXPDP DATABASE.txt
================================================================================

-- [OTHER]
EXPDP DATABASE test LOCATION = '/data1/expdp/database';


================================================================================
-- 来源: 1331_EXPDP TABLE.txt
================================================================================

-- [OTHER]
EXPDP TABLE test_t LOCATION = '/data1/expdp/table0';


================================================================================
-- 来源: 1332_EXPLAIN.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . customer_address ( ca_address_sk INTEGER NOT NULL , ca_address_id CHARACTER ( 16 ) NOT NULL );

-- [DML_INSERT]
INSERT INTO tpcds . customer_address VALUES ( 5000 , 'AAAAAAAABAAAAAAA' ),( 10000 , 'AAAAAAAACAAAAAAA' );

-- [DDL]
CREATE TABLE tpcds . customer_address_p1 AS TABLE tpcds . customer_address ;

-- [SESSION]
SET explain_perf_mode = normal ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM tpcds . customer_address_p1 ;

-- [EXPLAIN]
EXPLAIN ( FORMAT JSON ) SELECT * FROM tpcds . customer_address_p1 ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM tpcds . customer_address_p1 WHERE ca_address_sk = 10000 ;

-- [EXPLAIN]
EXPLAIN ( FORMAT YAML ) SELECT * FROM tpcds . customer_address_p1 WHERE ca_address_sk = 10000 ;

-- [EXPLAIN]
EXPLAIN ( COSTS FALSE ) SELECT * FROM tpcds . customer_address_p1 WHERE ca_address_sk = 10000 ;

-- [EXPLAIN]
EXPLAIN SELECT SUM ( ca_address_sk ) FROM tpcds . customer_address_p1 WHERE ca_address_sk < 10000 ;

-- [DDL]
DROP TABLE tpcds . customer_address_p1 ;

-- [DDL]
DROP TABLE tpcds . customer_address ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1333_EXPLAIN PLAN.txt
================================================================================

-- [DDL]
CREATE TABLE foo1 ( f1 int , f2 text , f3 text []);

-- [DDL]
CREATE TABLE foo2 ( f1 int , f2 text , f3 text []);

-- [EXPLAIN]
EXPLAIN PLAN SET STATEMENT_ID = 'TPCH-Q4' FOR SELECT f1 , count ( * ) FROM foo1 WHERE f1 > 1 AND f1 < 3 AND EXISTS ( SELECT * FROM foo2 ) GROUP BY f1 ;

-- [DQL]
SELECT * FROM plan_table;

-- [DML_DELETE]
DELETE FROM plan_table WHERE STATEMENT_ID = 'TPCH-Q4' ;

-- [DDL]
DROP TABLE foo1 ;

-- [DDL]
DROP TABLE foo2 ;

-- [DDL]
CREATE TABLE pt_t1 ( a integer , b int , c int ) WITH ( autovacuum_enabled = off ) DISTRIBUTE hash ( c );

-- [DDL]
CREATE TABLE pt_t1 ( a int , b int , c int ) WITH ( autovacuum_enabled = off ) DISTRIBUTE hash ( c );

-- [EXPLAIN]
EXPLAIN PLAN SET statement_id = 'test remote query' FOR SELECT current_user FROM pt_t1 , pt_t2 ;

-- [DQL]
SELECT * FROM plan_table ;

-- [DDL]
DROP TABLE pt_t1 ;

-- [DDL]
DROP TABLE pg_t2 ;

-- [SESSION]
SET enable_stream_recursive = off ;

-- [DDL]
CREATE TABLE chinamap ( id integer , pid integer , name text ) DISTRIBUTE BY hash ( id );

-- [EXPLAIN]
EXPLAIN PLAN SET statement_id = 'cte can not be push down' FOR WITH RECURSIVE rq AS ( SELECT id , name FROM chinamap WHERE id = 11 UNION ALL SELECT origin . id , rq . name || ' > ' || origin . name FROM rq JOIN chinamap origin ON origin . pid = rq . id ) SELECT id , name FROM rq ORDER BY 1 ;

-- [DQL]
SELECT * FROM plan_table ;

-- [DDL]
DROP TABLE chinamap ;


================================================================================
-- 来源: 1335_FETCH.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . customer_address ( ca_address_sk INTEGER NOT NULL , ca_address_id CHARACTER ( 16 ) NOT NULL , ca_street_number INTEGER , ca_street_name CHARACTER ( 20 ) );

-- [DML_INSERT]
INSERT INTO tpcds . customer_address VALUES ( 1 , 'AAAAAAAABAAAAAAA' , '18' , 'Jackson' ),( 2 , 'AAAAAAAACAAAAAAA' , '362' , 'Washington 6th' ),( 3 , 'AAAAAAAADAAAAAAA' , '585' , 'Dogwood Washington' );

-- [TCL]
START TRANSACTION ;

-- [OTHER]
CURSOR cursor1 FOR SELECT * FROM tpcds . customer_address ORDER BY 1 ;

-- [CURSOR]
FETCH FORWARD 3 FROM cursor1 ;

-- [CURSOR]
CLOSE cursor1 ;

-- [OTHER]
END ;

-- [TCL]
START TRANSACTION ;

-- [OTHER]
CURSOR cursor2 FOR VALUES ( 1 , 2 ),( 0 , 3 ) ORDER BY 1 ;

-- [CURSOR]
FETCH FORWARD 2 FROM cursor2 ;

-- [CURSOR]
CLOSE cursor2 ;

-- [OTHER]
END ;

-- [TCL]
START TRANSACTION ;

-- [PLSQL]
DECLARE cursor1 CURSOR WITH HOLD FOR SELECT * FROM tpcds . customer_address ORDER BY 1 ;

-- [CURSOR]
FETCH FORWARD 2 FROM cursor1 ;

-- [OTHER]
END ;

-- [CURSOR]
FETCH FORWARD 1 FROM cursor1 ;

-- [CURSOR]
CLOSE cursor1 ;

-- [DDL]
DROP TABLE tpcds . customer_address ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1337_GRANT.txt
================================================================================

-- [DDL]
CREATE USER joe PASSWORD 'xxxxxxxxxx' ;

-- [DCL_GRANT]
GRANT ALL PRIVILEGES TO joe ;

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . reason ( r_reason_sk INTEGER NOT NULL , r_reason_id CHAR ( 16 ) NOT NULL , r_reason_desc VARCHAR ( 20 ) );

-- [DCL_REVOKE]
REVOKE ALL PRIVILEGES FROM joe ;

-- [DCL_GRANT]
GRANT USAGE ON SCHEMA tpcds TO joe ;

-- [DCL_GRANT]
GRANT ALL PRIVILEGES ON tpcds . reason TO joe ;

-- [DCL_GRANT]
GRANT select ( r_reason_sk , r_reason_id , r_reason_desc ), update ( r_reason_desc ) ON tpcds . reason TO joe ;

-- [DCL_GRANT]
GRANT select ( r_reason_sk , r_reason_id ) ON tpcds . reason TO joe WITH GRANT OPTION ;

-- [DDL]
CREATE DATABASE testdb ;

-- [DCL_GRANT]
GRANT create , connect on database testdb TO joe WITH GRANT OPTION ;

-- [DDL]
CREATE ROLE tpcds_manager PASSWORD 'xxxxxxxxxx' ;

-- [DCL_GRANT]
GRANT USAGE , CREATE ON SCHEMA tpcds TO tpcds_manager ;

-- [DDL]
CREATE TABLESPACE tpcds_tbspc RELATIVE LOCATION 'tablespace/tablespace_1' ;

-- [DCL_GRANT]
GRANT ALL ON TABLESPACE tpcds_tbspc TO joe ;

-- [DDL]
CREATE or replace FUNCTION tpcds.fun1() RETURN boolean AS BEGIN SELECT current_user;

-- [DCL_GRANT]
GRANT ALTER ON FUNCTION tpcds.fun1() TO joe;

-- [DDL]
CREATE ROLE manager PASSWORD 'xxxxxxxxxxx' ;

-- [DCL_GRANT]
GRANT joe TO manager WITH ADMIN OPTION ;

-- [DDL]
CREATE ROLE senior_manager PASSWORD 'xxxxxxxxxxx' ;

-- [DCL_GRANT]
GRANT manager TO senior_manager ;

-- [DCL_REVOKE]
REVOKE joe FROM manager ;

-- [DCL_REVOKE]
REVOKE manager FROM senior_manager ;

-- [DDL]
DROP USER manager ;

-- [DDL]
DROP DATABASE testdb ;


================================================================================
-- 来源: 1339_IMPDP DATABASE CREATE.txt
================================================================================

-- [OTHER]
IMPDP DATABASE test CREATE SOURCE = '/data1/impdp/database' OWNER = admin;


================================================================================
-- 来源: 1340_IMPDP RECOVER.txt
================================================================================

-- [OTHER]
IMPDP DATABASE RECOVER SOURCE = '/data1/impdp/database' owner=admin;


================================================================================
-- 来源: 1341_IMPDP TABLE.txt
================================================================================

-- [OTHER]
IMPDP TABLE SOURCE = '/data1/impdp/table0' OWNER=admin;


================================================================================
-- 来源: 1342_IMPDP TABLE PREPARE.txt
================================================================================

-- [OTHER]
IMPDP TABLE PREPARE SOURCE = '/data1/impdp/table0' OWNER=admin;


================================================================================
-- 来源: 1343_INSERT.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

-- [DML_INSERT]
INSERT INTO tpcds . reason ( r_reason_sk , r_reason_id , r_reason_desc ) VALUES ( 0 , 'AAAAAAAAAAAAAAAA' , 'reason0' );

-- [DDL]
CREATE TABLE tpcds . reason_t2 ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

-- [DML_INSERT]
INSERT INTO tpcds . reason_t2 ( r_reason_sk , r_reason_id , r_reason_desc ) VALUES ( 1 , 'AAAAAAAABAAAAAAA' , 'reason1' );

-- [DML_INSERT]
INSERT INTO tpcds . reason_t2 VALUES ( 2 , 'AAAAAAAABAAAAAAA' , 'reason2' );

-- [DML_INSERT]
INSERT INTO tpcds . reason_t2 VALUES ( 3 , 'AAAAAAAACAAAAAAA' , 'reason3' ),( 4 , 'AAAAAAAADAAAAAAA' , 'reason4' ),( 5 , 'AAAAAAAAEAAAAAAA' , 'reason5' );

-- [DML_INSERT]
INSERT INTO tpcds . reason_t2 SELECT * FROM tpcds . reason WHERE r_reason_sk < 5 ;

-- [DDL]
CREATE UNIQUE INDEX reason_t2_u_index ON tpcds . reason_t2 ( r_reason_sk );

-- [DML_INSERT]
INSERT INTO tpcds . reason_t2 VALUES ( 5 , 'BBBBBBBBCAAAAAAA' , 'reason5' ),( 6 , 'AAAAAAAADAAAAAAA' , 'reason6' ) ON DUPLICATE KEY UPDATE r_reason_id = 'BBBBBBBBCAAAAAAA' ;

-- [DML_INSERT]
INSERT INTO tpcds . reason_t2 VALUES ( 5 , 'BBBBBBBBCAAAAAAA' , 'reason5' ) ON DUPLICATE KEY UPDATE r_reason_desc = 'reason5_new' RETURNING * ;

-- [DDL]
DROP TABLE tpcds . reason_t2 ;

-- [DDL]
DROP TABLE tpcds . reason ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1345_LOCK.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . reason ( r_reason_sk INTEGER NOT NULL , r_reason_id CHAR ( 16 ) NOT NULL , r_reason_desc INTEGER );

-- [DML_INSERT]
INSERT INTO tpcds . reason VALUES ( 1 , 'AAAAAAAABAAAAAAA' , '18' ),( 5 , 'AAAAAAAACAAAAAAA' , '362' ),( 7 , 'AAAAAAAADAAAAAAA' , '585' );

-- [DDL]
CREATE TABLE tpcds . reason_t1 AS TABLE tpcds . reason ;

-- [TCL]
START TRANSACTION ;

-- [MAINTENANCE]
LOCK TABLE tpcds . reason_t1 IN SHARE ROW EXCLUSIVE MODE ;

-- [DML_DELETE]
DELETE FROM tpcds . reason_t1 WHERE r_reason_desc IN ( SELECT r_reason_desc FROM tpcds . reason_t1 WHERE r_reason_sk < 6 );

-- [DML_DELETE]
DELETE FROM tpcds . reason_t1 WHERE r_reason_sk = 7 ;

-- [TCL]
COMMIT ;

-- [DDL]
DROP TABLE tpcds . reason_t1 ;

-- [DDL]
DROP TABLE tpcds . reason ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1346_LOCK BUCKETS.txt
================================================================================

-- [TCL]
START TRANSACTION ;

-- [MAINTENANCE]
LOCK BUCKETS ( 0 , 1 , 2 , 3 ) IN ACCESS EXCLUSIVE MODE ;

-- [TCL]
COMMIT ;


================================================================================
-- 来源: 1348_MARK BUCKETS.txt
================================================================================

-- [OTHER]
MARK BUCKETS ( 0 , 1 , 2 , 3 ) FINISH FROM datanode1 TO datanode3 ;


================================================================================
-- 来源: 1349_MERGE INTO.txt
================================================================================

-- [DDL]
CREATE TABLE products ( product_id INTEGER , product_name VARCHAR2 ( 60 ), category VARCHAR2 ( 60 ) );

-- [DML_INSERT]
INSERT INTO products VALUES ( 1501 , 'vivitar 35mm' , 'electrncs' );

-- [DML_INSERT]
INSERT INTO products VALUES ( 1502 , 'olympus is50' , 'electrncs' );

-- [DML_INSERT]
INSERT INTO products VALUES ( 1600 , 'play gym' , 'toys' );

-- [DML_INSERT]
INSERT INTO products VALUES ( 1601 , 'lamaze' , 'toys' );

-- [DML_INSERT]
INSERT INTO products VALUES ( 1666 , 'harry potter' , 'dvd' );

-- [DDL]
CREATE TABLE newproducts ( product_id INTEGER , product_name VARCHAR2 ( 60 ), category VARCHAR2 ( 60 ) );

-- [DML_INSERT]
INSERT INTO newproducts VALUES ( 1502 , 'olympus camera' , 'electrncs' );

-- [DML_INSERT]
INSERT INTO newproducts VALUES ( 1601 , 'lamaze' , 'toys' );

-- [DML_INSERT]
INSERT INTO newproducts VALUES ( 1666 , 'harry potter' , 'toys' );

-- [DML_INSERT]
INSERT INTO newproducts VALUES ( 1700 , 'wait interface' , 'books' );

-- [DML_MERGE]
MERGE INTO products p USING newproducts np ON ( p . product_id = np . product_id ) WHEN MATCHED THEN UPDATE SET p . product_name = np . product_name , p . category = np . category WHERE p . product_name != 'play gym' WHEN NOT MATCHED THEN INSERT VALUES ( np . product_id , np . product_name , np . category ) WHERE np . category = 'books' ;

-- [DQL]
SELECT * FROM products ORDER BY product_id ;

-- [DDL]
DROP TABLE products ;

-- [DDL]
DROP TABLE newproducts ;


================================================================================
-- 来源: 1350_MOVE.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . reason ( r_reason_sk INTEGER NOT NULL , r_reason_id CHAR ( 16 ) NOT NULL , r_reason_desc VARCHAR ( 40 ) );

-- [DML_INSERT]
INSERT INTO tpcds . reason VALUES ( 1 , 'AAAAAAAABAAAAAAA' , 'Xxxxxxxxx' ),( 2 , 'AAAAAAAACAAAAAAA' , 'Xxxxxxxxx' ),( 3 , 'AAAAAAAADAAAAAAA' , ' Xxxxxxxxx' ),( 4 , 'AAAAAAAAEAAAAAAA' , 'Not the product that was ordered' ),( 5 , 'AAAAAAAAFAAAAAAA' , 'Parts missing' ),( 6 , 'AAAAAAAAGAAAAAAA' , 'Does not work with a product that I have' ),( 7 , 'AAAAAAAAHAAAAAAA' , 'Gift exchange' );

-- [TCL]
START TRANSACTION ;

-- [OTHER]
CURSOR cursor1 FOR SELECT * FROM tpcds . reason ;

-- [OTHER]
MOVE FORWARD 3 FROM cursor1 ;

-- [CURSOR]
FETCH 4 FROM cursor1 ;

-- [CURSOR]
CLOSE cursor1 ;

-- [OTHER]
END ;

-- [DDL]
DROP TABLE tpcds . reason ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1355_PURGE.txt
================================================================================

-- [DDL]
CREATE ROLE tpcds IDENTIFIED BY '*********';

-- 创建表空间reason_table_space。
-- [DDL]
CREATE TABLESPACE REASON_TABLE_SPACE1 owner tpcds RELATIVE location 'tablespace/tsp_reason1';

-- 创建SCHEMA。
-- [DDL]
CREATE SCHEMA tpcds;

-- 在表空间创建表tpcds.reason_t1。
-- [DDL]
CREATE TABLE tpcds.reason_t1 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) with(storage_type=ustore) tablespace reason_table_space1;

-- 在表空间创建表tpcds.reason_t2。
-- [DDL]
CREATE TABLE tpcds.reason_t2 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) with(storage_type=ustore) tablespace reason_table_space1;

-- 在表空间创建表tpcds.reason_t3。
-- [DDL]
CREATE TABLE tpcds.reason_t3 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) with(storage_type=ustore) tablespace reason_table_space1;

-- 对表tpcds.reason_t1创建索引。
-- [DDL]
CREATE INDEX index_t1 on tpcds.reason_t1(r_reason_id);

-- [DDL]
DROP TABLE tpcds.reason_t1;

-- [DDL]
DROP TABLE tpcds.reason_t2;

-- [DDL]
DROP TABLE tpcds.reason_t3;

--查看回收站。
-- [DQL]
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

--PURGE清除表。
-- [OTHER]
PURGE TABLE tpcds.reason_t3;

-- [DQL]
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

--PURGE清除索引。
-- [OTHER]
PURGE INDEX tpcds.index_t1;

-- [DQL]
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

--PURGE清除回收站所有对象。
-- [OTHER]
PURGE recyclebin;

-- [DQL]
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

-- 删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 1357_REASSIGN OWNED.txt
================================================================================

-- [DDL]
CREATE USER jim PASSWORD '********' ;

-- [DDL]
CREATE USER tom PASSWORD '********' ;

-- [OTHER]
REASSIGN OWNED BY jim TO tom ;

-- [DDL]
DROP USER jim , tom CASCADE ;


================================================================================
-- 来源: 1358_REFRESH INCREMENTAL MATERIALIZED VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE my_table (c1 int, c2 int);

--创建增量物化视图。
-- [DDL]
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--基表写入数据。
-- [DML_INSERT]
INSERT INTO my_table VALUES(1,1),(2,2);

--对增量物化视图my_imv进行增量刷新。
-- [OTHER]
REFRESH INCREMENTAL MATERIALIZED VIEW my_imv;

--删除增量物化视图。
-- [DDL]
DROP MATERIALIZED VIEW my_imv;

--删除表my_table。
-- [DDL]
DROP TABLE my_table;


================================================================================
-- 来源: 1359_REFRESH MATERIALIZED VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE my_table (c1 int, c2 int);

--创建全量物化视图。
-- [DDL]
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--创建增量物化视图。
-- [DDL]
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--基表写入数据。
-- [DML_INSERT]
INSERT INTO my_table VALUES(1,1),(2,2);

--对全量物化视图my_mv进行全量刷新。
-- [OTHER]
REFRESH MATERIALIZED VIEW my_mv;

--对增量物化视图my_imv进行全量刷新。
-- [OTHER]
REFRESH MATERIALIZED VIEW my_imv;

--删除增量物化视图。
-- [DDL]
DROP MATERIALIZED VIEW my_imv;

--删除全量物化视图。
-- [DDL]
DROP MATERIALIZED VIEW my_mv;

--删除表my_table。
-- [DDL]
DROP TABLE my_table;


================================================================================
-- 来源: 1360_REINDEX.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . customer ( c_customer_sk INTEGER NOT NULL , c_customer_id CHAR ( 16 ) NOT NULL );

-- [DML_INSERT]
INSERT INTO tpcds . customer VALUES ( 1 , 'AAAAAAAABAAAAAAA' ),( 5 , 'AAAAAAAACAAAAAAA' ),( 10 , 'AAAAAAAADAAAAAAA' );

-- [DDL]
CREATE TABLE tpcds . customer_t1 ( c_customer_sk integer not null , c_customer_id char ( 16 ) not null , c_current_cdemo_sk integer , c_current_hdemo_sk integer , c_current_addr_sk integer , c_first_shipto_date_sk integer , c_first_sales_date_sk integer , c_salutation char ( 10 ) , c_first_name char ( 20 ) , c_last_name char ( 30 ) , c_preferred_cust_flag char ( 1 ) , c_birth_day integer , c_birth_month integer , c_birth_year integer , c_birth_country varchar ( 20 ) , c_login char ( 13 ) , c_email_address char ( 50 ) , c_last_review_date char ( 10 ) ) WITH ( orientation = row );

-- [DDL]
CREATE INDEX tpcds_customer_index1 ON tpcds . customer_t1 ( c_customer_sk );

-- [DML_INSERT]
INSERT INTO tpcds . customer_t1 SELECT * FROM tpcds . customer WHERE c_customer_sk < 10 ;

-- [MAINTENANCE]
REINDEX INDEX tpcds . tpcds_customer_index1 ;

-- [MAINTENANCE]
REINDEX INDEX CONCURRENTLY tpcds . tpcds_customer_index1 ;

-- [MAINTENANCE]
REINDEX TABLE tpcds . customer_t1 ;

-- [MAINTENANCE]
REINDEX TABLE CONCURRENTLY tpcds . customer_t1 ;

-- [DDL]
DROP TABLE tpcds . customer_t1 ;

-- [DDL]
DROP TABLE tpcds . customer ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1361_RELEASE SAVEPOINT.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . table1 ( a int );

-- [TCL]
START TRANSACTION ;

-- [DML_INSERT]
INSERT INTO tpcds . table1 VALUES ( 3 );

-- [TCL]
SAVEPOINT my_savepoint ;

-- [DML_INSERT]
INSERT INTO tpcds . table1 VALUES ( 4 );

-- [TCL]
RELEASE SAVEPOINT my_savepoint ;

-- [TCL]
COMMIT ;

-- [DQL]
SELECT * FROM tpcds . table1 ;

-- [DDL]
DROP TABLE tpcds . table1 ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1362_REPLACE.txt
================================================================================

-- [DDL]
CREATE TABLE test(f1 int primary key, f2 int, f3 int);

--插入数据。
-- [DML_INSERT]
INSERT INTO test VALUES(1, 1, 1), (2, 2, 2), (3, 3, 3);

--值替换插入数据。
-- [OTHER]
REPLACE INTO test VALUES(1, 11, 11);

--查询值替换插入的结果
-- [DQL]
SELECT * FROM test WHERE f1 = 1;

--查询替换插入数据。
-- [OTHER]
REPLACE INTO test SELECT 2, 22, 22;

-- [DQL]
SELECT * FROM test WHERE f1 = 2;

--设置指定字段替换插入数据。
-- [OTHER]
REPLACE INTO test SET f1 = f1 + 3, f2 = f1 * 10 + 3, f3 = f2;

-- [DQL]
SELECT * FROM test WHERE f1 = 3;

-- [DDL]
DROP TABLE test;


================================================================================
-- 来源: 1363_RESET.txt
================================================================================

-- [SESSION]
RESET timezone ;

-- [SESSION]
RESET ALL ;


================================================================================
-- 来源: 1364_REVOKE.txt
================================================================================

-- [DCL_REVOKE]
REVOKE jerry FROM tom ;

-- [DCL_REVOKE]
REVOKE SELECT ON TABLE jerry . t1 FROM tom ;

-- [DCL_REVOKE]
REVOKE EXECUTE ON FUNCTION jerry . fun1 () FROM tom ;

-- [DCL_REVOKE]
REVOKE CONNECT ON database DB1 FROM tom ;


================================================================================
-- 来源: 1365_ROLLBACK.txt
================================================================================

-- [TCL]
START TRANSACTION ;

-- [TCL]
ROLLBACK ;


================================================================================
-- 来源: 1367_ROLLBACK TO SAVEPOINT.txt
================================================================================

-- [TCL]
START TRANSACTION ;

-- [TCL]
SAVEPOINT my_savepoint ;

-- [TCL]
ROLLBACK TO SAVEPOINT my_savepoint ;

-- [PLSQL]
DECLARE foo CURSOR FOR SELECT 1 UNION SELECT 2 ;

-- [TCL]
SAVEPOINT foo ;

-- [CURSOR]
FETCH 1 FROM foo ;

-- [TCL]
ROLLBACK TO SAVEPOINT foo ;

-- [CURSOR]
FETCH 1 FROM foo ;

-- [TCL]
RELEASE SAVEPOINT my_savepoint ;

-- [TCL]
COMMIT ;


================================================================================
-- 来源: 1369_SAVEPOINT.txt
================================================================================

-- [DDL]
CREATE TABLE table1 ( a int );

-- [TCL]
START TRANSACTION ;

-- [DML_INSERT]
INSERT INTO table1 VALUES ( 1 );

-- [TCL]
SAVEPOINT my_savepoint ;

-- [DML_INSERT]
INSERT INTO table1 VALUES ( 2 );

-- [TCL]
ROLLBACK TO SAVEPOINT my_savepoint ;

-- [DML_INSERT]
INSERT INTO table1 VALUES ( 3 );

-- [TCL]
COMMIT ;

-- [DQL]
SELECT * FROM table1 ;

-- [DDL]
DROP TABLE table1 ;

-- [DDL]
CREATE TABLE table2 ( a int );

-- [TCL]
START TRANSACTION ;

-- [DML_INSERT]
INSERT INTO table2 VALUES ( 3 );

-- [TCL]
SAVEPOINT my_savepoint ;

-- [DML_INSERT]
INSERT INTO table2 VALUES ( 4 );

-- [TCL]
RELEASE SAVEPOINT my_savepoint ;

-- [TCL]
COMMIT ;

-- [DQL]
SELECT * FROM table2 ;

-- [DDL]
DROP TABLE table2 ;


================================================================================
-- 来源: 1370_SECURITY LABEL ON.txt
================================================================================

-- [DDL]
CREATE SECURITY LABEL sec_label 'L1:G4' ;

-- [DDL]
CREATE TABLE tbl ( c1 int , c2 int );

-- [DDL]
CREATE USER bob WITH PASSWORD '********' ;

-- [OTHER]
SECURITY LABEL ON ROLE bob IS 'sec_label' ;

-- [OTHER]
SECURITY LABEL ON TABLE tbl IS 'sec_label' ;

-- [OTHER]
SECURITY LABEL ON COLUMN tbl . c1 IS 'sec_label' ;

-- [OTHER]
SECURITY LABEL ON ROLE bob IS NULL ;

-- [OTHER]
SECURITY LABEL ON TABLE tbl IS NULL ;

-- [OTHER]
SECURITY LABEL ON COLUMN tbl . c1 IS NULL ;

-- [DDL]
DROP SECURITY LABEL sec_label ;

-- [DDL]
DROP TABLE tbl ;

-- [DDL]
DROP USER bob ;


================================================================================
-- 来源: 1371_SELECT.txt
================================================================================

-- [DQL]
SELECT * FROM XMLTABLE( XMLNAMESPACES('nspace1' AS "ns1", 'nspace2' AS "ns2"), -- 声明两个XML的命名空间'nspace1'和'nspace2'及对应的别名"ns1"和"ns2" '/ns1:root/*:child' -- 经row_expression从传入的数据中选取命名空间为'nspace1'的root节点，在选取其下面的所有child节点，忽略child的命名空间；其中ns1为'nspace1'的别名 PASSING xmltype( '<root xmlns="nspace1"> <child> <name>peter</name> <age>11</age> </child> <child xmlns="nspace1"> <name>qiqi</name> <age>12</age> </child> <child xmlns="nspace2"> <name>hacker</name> <age>15</age> </child> </root>') COLUMNS column FOR ORDINALITY, -- 该列为行号列 name varchar(10) path 'ns1:name', -- 从row_expression获取的每个child节点中选取命名空间为'nspace1'的name节点，并将节点中的值转换为varchar(10)返回；其中ns1为'nspace1'的别名 age int);

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

-- [DML_INSERT]
INSERT INTO tpcds . reason values ( 3 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 2' ),( 4 , 'AAAAAAAABAAAAAAA' , 'reason 3' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 4' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 5' ),( 20 , 'AAAAAAAACAAAAAAA' , 'N%reason 6' ),( 30 , 'AAAAAAAACAAAAAAA' , 'W%reason 7' );

-- [DQL]
WITH temp_t ( name , isdba ) AS ( SELECT usename , usesuper FROM pg_user ) SELECT * FROM temp_t ;

-- [DQL]
SELECT DISTINCT ( r_reason_sk ) FROM tpcds . reason ;

-- [DQL]
SELECT * FROM tpcds . reason LIMIT 1 ;

-- [DQL]
SELECT r_reason_desc FROM tpcds . reason ORDER BY r_reason_desc ;

-- [DQL]
SELECT a . usename , b . locktime FROM pg_user a , pg_user_status b WHERE a . usesysid = b . roloid ;

-- [DQL]
SELECT a . usename , b . locktime , a . usesuper FROM pg_user a FULL JOIN pg_user_status b on a . usesysid = b . roloid ;

-- [DQL]
SELECT r_reason_id , AVG ( r_reason_sk ) FROM tpcds . reason GROUP BY r_reason_id HAVING AVG ( r_reason_sk ) > 25 ;

-- [DQL]
SELECT r_reason_id , AVG ( r_reason_sk ) FROM tpcds . reason GROUP BY CUBE ( r_reason_id , r_reason_sk );

-- [DQL]
SELECT r_reason_id , AVG ( r_reason_sk ) FROM tpcds . reason GROUP BY GROUPING SETS (( r_reason_id , r_reason_sk ), r_reason_sk );

-- [DQL]
SELECT r_reason_sk , tpcds . reason . r_reason_desc FROM tpcds . reason WHERE tpcds . reason . r_reason_desc LIKE 'W%' UNION SELECT r_reason_sk , tpcds . reason . r_reason_desc FROM tpcds . reason WHERE tpcds . reason . r_reason_desc LIKE 'N%' ;

-- [DQL]
SELECT * FROM tpcds . reason ORDER BY NLSSORT ( r_reason_desc , 'NLS_SORT = SCHINESE_PINYIN_M' );

-- [DQL]
SELECT * FROM tpcds . reason ORDER BY NLSSORT ( r_reason_desc , 'NLS_SORT = generic_m_ci' );

-- [DDL]
CREATE TABLE tpcds . reason_p ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) ) PARTITION BY RANGE ( r_reason_sk ) ( partition P_05_BEFORE values less than ( 05 ), partition P_15 values less than ( 15 ), partition P_25 values less than ( 25 ), partition P_35 values less than ( 35 ), partition P_45_AFTER values less than ( MAXVALUE ) );

-- [DML_INSERT]
INSERT INTO tpcds . reason_p values ( 3 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 2' ),( 4 , 'AAAAAAAABAAAAAAA' , 'reason 3' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 4' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 5' ),( 20 , 'AAAAAAAACAAAAAAA' , 'reason 6' ),( 30 , 'AAAAAAAACAAAAAAA' , 'reason 7' );

-- [DQL]
SELECT * FROM tpcds . reason_p PARTITION ( P_05_BEFORE );

-- [DQL]
SELECT * FROM tpcds . reason_p PARTITION ( P_05_BEFORE , P_15 , P_25 ) ORDER BY 1 ;

-- [DQL]
SELECT COUNT ( * ), r_reason_id FROM tpcds . reason_p GROUP BY r_reason_id ;

-- [DQL]
SELECT * FROM tpcds . reason GROUP BY CUBE ( r_reason_id , r_reason_sk , r_reason_desc );

-- [DQL]
SELECT * FROM tpcds . reason GROUP BY GROUPING SETS (( r_reason_id , r_reason_sk ), r_reason_desc );

-- [DQL]
SELECT COUNT ( * ) c , r_reason_id FROM tpcds . reason_p GROUP BY r_reason_id HAVING c > 2 ;

-- [DQL]
SELECT COUNT ( * ), r_reason_id FROM tpcds . reason_p GROUP BY r_reason_id HAVING r_reason_id IN ( 'AAAAAAAABAAAAAAA' , 'AAAAAAAADAAAAAAA' );

-- [DQL]
SELECT * FROM tpcds . reason_p WHERE r_reason_id = 'AAAAAAAABAAAAAAA' INTERSECT SELECT * FROM tpcds . reason_p WHERE r_reason_sk < 5 ;

-- [DQL]
SELECT * FROM tpcds . reason_p WHERE r_reason_id = 'AAAAAAAABAAAAAAA' EXCEPT SELECT * FROM tpcds . reason_p WHERE r_reason_sk < 4 ;

-- [DDL]
CREATE TABLE tpcds . store_returns ( sr_item_sk int , sr_customer_id varchar ( 50 ), sr_customer_sk int );

-- [DDL]
CREATE TABLE tpcds . customer ( c_item_sk int , c_customer_id varchar ( 50 ), c_customer_sk int );

-- [DQL]
SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE t1 . sr_customer_sk = t2 . c_customer_sk ( + ) ORDER BY 1 DESC LIMIT 1 ;

-- [DQL]
SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE t1 . sr_customer_sk ( + ) = t2 . c_customer_sk ORDER BY 1 DESC LIMIT 1 ;

-- [DQL]
SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE t1 . sr_customer_sk = t2 . c_customer_sk ( + ) AND t2 . c_customer_sk ( + ) < 1 ORDER BY 1 LIMIT 1 ;

-- [DQL]
SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE NOT ( t1 . sr_customer_sk = t2 . c_customer_sk ( + ) AND t2 . c_customer_sk ( + ) < 1 );

-- [DQL]
SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE ( t1 . sr_customer_sk = t2 . c_customer_sk ( + )):: bool ;

-- [DQL]
SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE t1 . sr_customer_sk ( + ) = t2 . c_customer_sk ( + );

-- [DDL]
DROP TABLE tpcds.reason_p;

-- [DQL]
WITH RECURSIVE t1(a) as ( select 100 ), t(n) AS ( VALUES (1) UNION ALL SELECT n+1 FROM t WHERE n < (select max(a) from t1) ) SELECT sum(n) FROM t;

-- [DDL]
CREATE DATABASE pivot_db dbcompatibility ' ORA ';

-- [DDL]
DROP DATABASE pivot_db;

-- [DDL]
CREATE TABLE skiplocked_astore(id int, info text) WITH (storage_type=astore);

-- [DML_INSERT]
INSERT INTO skiplocked_astore VALUES (1, ' abc '), (2, ' bcd '), (3, ' cdf '),(3, ' dfg ' );

-- [TCL]
BEGIN ;

-- [DQL]
SELECT * FROM skiplocked_astore WHERE id = 1 FOR UPDATE ;

-- [DQL]
SELECT * FROM skiplocked_astore FOR UPDATE SKIP LOCKED ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1372_SELECT INTO.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

-- [DML_INSERT]
INSERT INTO tpcds . reason values ( 1 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 2 , 'AAAAAAAABAAAAAAA' , 'reason 2' ),( 3 , 'AAAAAAAABAAAAAAA' , 'reason 3' ),( 4 , 'AAAAAAAABAAAAAAA' , 'reason 4' ),( 4 , 'AAAAAAAABAAAAAAA' , 'reason 5' ),( 4 , 'AAAAAAAACAAAAAAA' , 'reason 6' ),( 5 , 'AAAAAAAACAAAAAAA' , 'reason 7' );

-- [DQL]
SELECT * INTO tpcds . reason_t1 FROM tpcds . reason WHERE r_reason_sk < 5 ;

-- [DDL]
DROP TABLE tpcds . reason_t1 , tpcds . reason ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1373_SET.txt
================================================================================

-- [SESSION]
SET search_path TO tpcds , public ;

-- [SESSION]
SET datestyle TO postgres ;


================================================================================
-- 来源: 1374_SET CONSTRAINTS.txt
================================================================================

-- [SESSION]
SET CONSTRAINTS ALL DEFERRED ;


================================================================================
-- 来源: 1375_SET ROLE.txt
================================================================================

-- [DDL]
CREATE ROLE paul IDENTIFIED BY '********' ;

-- [SESSION]
SET ROLE paul PASSWORD '********' ;

-- [DDL]
DROP USER paul ;


================================================================================
-- 来源: 1376_SET SESSION AUTHORIZATION.txt
================================================================================

-- [DDL]
CREATE ROLE paul IDENTIFIED BY '********' ;

-- [SESSION]
SET SESSION AUTHORIZATION paul password '********' ;

-- [DDL]
DROP USER paul ;


================================================================================
-- 来源: 1377_SET TRANSACTION.txt
================================================================================

-- [TCL]
START TRANSACTION ;

-- [SESSION]
SET LOCAL TRANSACTION ISOLATION LEVEL READ COMMITTED READ ONLY ;

-- [TCL]
COMMIT ;


================================================================================
-- 来源: 1378_SHOW.txt
================================================================================

-- [SESSION]
SHOW timezone ;

-- [SESSION]
SHOW ALL ;

-- [SESSION]
SHOW VARIABLES LIKE var ;


================================================================================
-- 来源: 1379_SHUTDOWN.txt
================================================================================

-- [OTHER]
SHUTDOWN;

--使用fast模式关闭当前数据库节点。
-- [OTHER]
SHUTDOWN FAST;


================================================================================
-- 来源: 1380_START TRANSACTION.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . reason ( c1 int , c2 int );

-- [TCL]
START TRANSACTION ;

-- [DQL]
SELECT * FROM tpcds . reason ;

-- [OTHER]
END ;

-- [TCL]
BEGIN ;

-- [DQL]
SELECT * FROM tpcds . reason ;

-- [OTHER]
END ;

-- [TCL]
START TRANSACTION ISOLATION LEVEL READ COMMITTED READ WRITE ;

-- [DQL]
SELECT * FROM tpcds . reason ;

-- [TCL]
COMMIT ;

-- [DDL]
DROP TABLE tpcds . reason ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1382_TIMECAPSULE TABLE.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

-- 删除表tpcds.reason_t2。
-- [DDL]
DROP TABLE IF EXISTS tpcds.reason_t2;

-- 创建表tpcds.reason_t2。
-- [DDL]
CREATE TABLE tpcds.reason_t2 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) )with(storage_type = ustore);

--向表tpcds.reason_t2中插入记录。
-- [DML_INSERT]
INSERT INTO tpcds.reason_t2 VALUES (1, 'AA', 'reason1'),(2, 'AB', 'reason2'),(3, 'AC', 'reason3');

--清空tpcds.reason_t2表中的数据。
-- [DML_TRUNCATE]
TRUNCATE TABLE tpcds.reason_t2;

--查询tpcds.reason_t2表中的数据。
-- [DQL]
SELECT * FROM tpcds.reason_t2;

--执行闪回TRUNCATE。
-- [OTHER]
TIMECAPSULE TABLE tpcds.reason_t2 to BEFORE TRUNCATE;

-- [DQL]
SELECT * FROM tpcds.reason_t2;

--删除表tpcds.reason_t2。
-- [DDL]
DROP TABLE tpcds.reason_t2;

--执行闪回DROP。
-- [OTHER]
TIMECAPSULE TABLE tpcds.reason_t2 to BEFORE DROP;

-- 清空回收站，删除SCHEMA。
-- [OTHER]
PURGE RECYCLEBIN;

-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 1383_TRUNCATE.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

-- [DML_INSERT]
INSERT INTO tpcds . reason values ( 1 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 5 , 'AAAAAAAABAAAAAAA' , 'reason 2' ),( 15 , 'AAAAAAAABAAAAAAA' , 'reason 3' ),( 25 , 'AAAAAAAABAAAAAAA' , 'reason 4' ),( 35 , 'AAAAAAAABAAAAAAA' , 'reason 5' ),( 45 , 'AAAAAAAACAAAAAAA' , 'reason 6' ),( 55 , 'AAAAAAAACAAAAAAA' , 'reason 7' );

-- [DDL]
CREATE TABLE tpcds . reason_t1 AS TABLE tpcds . reason ;

-- [DML_TRUNCATE]
TRUNCATE TABLE tpcds . reason_t1 ;

-- [DDL]
DROP TABLE tpcds . reason_t1 ;

-- [DDL]
CREATE TABLE tpcds . reason_p ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) ) PARTITION BY RANGE ( r_reason_sk ) ( partition p_05_before values less than ( 05 ), partition p_15 values less than ( 15 ), partition p_25 values less than ( 25 ), partition p_35 values less than ( 35 ), partition p_45_after values less than ( MAXVALUE ) );

-- [DML_INSERT]
INSERT INTO tpcds . reason_p SELECT * FROM tpcds . reason ;

-- [DDL]
ALTER TABLE tpcds . reason_p TRUNCATE PARTITION p_05_before ;

-- [DDL]
ALTER TABLE tpcds . reason_p TRUNCATE PARTITION for ( 13 );

-- [DML_TRUNCATE]
TRUNCATE TABLE tpcds . reason_p ;

-- [DDL]
DROP TABLE tpcds . reason_p ;

-- [DDL]
DROP TABLE tpcds . reason ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1385_UPDATE.txt
================================================================================

-- [DDL]
CREATE TABLE tbl_test1(id int, info varchar(10));

-- [DML_INSERT]
INSERT INTO tbl_test1 VALUES (1, 'A'), (2, 'B');

--修改tbl_test1表中所有数据的info列。
-- [DML_UPDATE]
UPDATE tbl_test1 SET info = 'aa';

--查询tbl_test1表。
-- [DQL]
SELECT * FROM tbl_test1;

-- [DML_UPDATE]
UPDATE tbl_test1 SET info = 'bb' WHERE id = 2;

--查询tbl_test1表。
-- [DQL]
SELECT * FROM tbl_test1;

-- [DML_UPDATE]
UPDATE tbl_test1 SET info = 'ABC' WHERE id = 1 RETURNING info;

-- 删除tbl_test1表。
-- [DDL]
DROP TABLE tbl_test1;

-- [DDL]
CREATE TABLE test_grade ( sid int, --学号 name varchar(50), --姓名 score char, --成绩 examtime date, --考试时间 last_exam boolean --是否是最后一次考试 );

--插入数据。
-- [DML_INSERT]
INSERT INTO test_grade VALUES (1,'Scott','A','2008-07-08',1),(2,'Ben','D','2008-07-08',1),(3,'Jack','D','2008-07-08',1);

--查询。
-- [DQL]
SELECT * FROM test_grade;

--2008-08-25 Ben参加了补考,成绩为B，正常步骤需要先修改last_exam为否,然后插入2008-08-25这一天的成绩。
-- [DQL]
WITH old_exa AS ( UPDATE test_grade SET last_exam = 0 WHERE sid = 2 AND examtime = '2008-07-08' RETURNING sid, name ) INSERT INTO test_grade VALUES ( ( SELECT sid FROM old_exa ), (SELECT name FROM old_exa), 'B', '2008-08-25', 1 );

--查询。
-- [DQL]
SELECT * FROM test_grade;

--删除。
-- [DDL]
DROP TABLE test_grade;


================================================================================
-- 来源: 1387_VACUUM.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds ;

-- [DDL]
CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

-- [DML_INSERT]
INSERT INTO tpcds . reason values ( 1 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 2 , 'AAAAAAAABAAAAAAA' , 'reason 2' );

-- [DDL]
CREATE UNIQUE INDEX ds_reason_index1 ON tpcds . reason ( r_reason_sk );

-- [MAINTENANCE]
VACUUM ( VERBOSE , ANALYZE ) tpcds . reason ;

-- [DDL]
DROP INDEX tpcds . ds_reason_index1 CASCADE ;

-- [DDL]
DROP TABLE tpcds . reason ;

-- [DDL]
DROP SCHEMA tpcds CASCADE ;


================================================================================
-- 来源: 1396_file_1396.txt
================================================================================

-- [DQL]
select $$it's an example$$;


================================================================================
-- 来源: 1420_file_1420.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE array_proc AS DECLARE TYPE ARRAY_INTEGER IS VARRAY ( 1024 ) OF INTEGER ;

-- [PLSQL]
CALL array_proc ();

-- [DDL]
DROP PROCEDURE array_proc ;


================================================================================
-- 来源: 1421_file_1421.txt
================================================================================

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set a_format_version='10c';

-- [SESSION]
set a_format_dev_version='s1';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- n大于数组元素个数, 清空数组元素
-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- 数组未初始化
-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回NULL
-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回NULL
-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回NULL
-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 下标越界，大于数组范围
-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回NULL
-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 下标越界，大于数组范围
-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回false
-- [PLSQL]
declare type varr is varray(10) of varchar(3);


================================================================================
-- 来源: 1423_file_1423.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER ;

-- [PLSQL]
CALL table_proc ();

-- [DDL]
DROP PROCEDURE table_proc ;

-- [DDL]
CREATE OR REPLACE PROCEDURE nest_table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER ;

-- [PLSQL]
CALL nest_table_proc ();

-- [DDL]
DROP PROCEDURE nest_table_proc ;

-- [DDL]
CREATE OR REPLACE PROCEDURE index_table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER INDEX BY INTEGER ;

-- [PLSQL]
CALL index_table_proc ();

-- [DDL]
DROP PROCEDURE index_table_proc ;

-- [DDL]
CREATE OR REPLACE PROCEDURE nest_table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER INDEX BY INTEGER ;

-- [PLSQL]
CALL nest_table_proc ();

-- [DDL]
DROP PROCEDURE nest_table_proc ;


================================================================================
-- 来源: 1424_file_1424.txt
================================================================================

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of varchar2;

-- [PLSQL]
declare type nest is table of varchar2 index by varchar2;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type t1 is table of int index by int;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type t1 is table of int index by int;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type t1 is table of int index by int;

-- [PLSQL]
declare type nest is table of int;

-- [DDL]
create or replace procedure p1 () gaussdb -# as gaussdb $ # type t1 is table of int ;

-- [PLSQL]
call p1 ();

-- [DDL]
drop procedure if exists p1 ();

-- [DDL]
create or replace procedure p1 () is gaussdb $ # type rec is record ( c1 int , c2 int );

-- [PLSQL]
call p1 ();

-- [DDL]
drop procedure if exists p1 ();

-- [DDL]
create or replace procedure p1 () gaussdb -# as gaussdb $ # type t1 is table of int index by int ;

-- [PLSQL]
call p1 ();

-- [DDL]
drop procedure if exists p1 ();

-- [DDL]
create or replace procedure p1 () is gaussdb $ # type rec is record ( c1 int , c2 int );

-- [PLSQL]
call p1 ();

-- [DDL]
drop procedure if exists p1 ();


================================================================================
-- 来源: 1425_record.txt
================================================================================

-- [DDL]
create table emp_rec ( gaussdb ( # empno numeric ( 4 , 0 ) not null , gaussdb ( # ename varchar ( 10 ) gaussdb ( # );

-- [DML_INSERT]
insert into emp_rec values ( 111 , 'aaa' ), ( 222 , 'bbb' ), ( 333 , 'ccc' );

-- [OTHER]
\ d emp_rec Table "public.emp_rec" Column | Type | Modifiers --------+-----------------------+----------- empno | numeric ( 4 , 0 ) | not null ename | character varying ( 10 ) | --演示在函数中对record进行操作。

-- [DDL]
CREATE OR REPLACE FUNCTION regress_record ( p_w VARCHAR2 ) RETURNS VARCHAR2 AS $$ gaussdb $ # DECLARE gaussdb $ # --声明一个record类型. gaussdb $ # type rec_type is record ( name varchar2 ( 100 ), epno int );

-- [PLSQL]
CALL regress_record ( 'abc' );

-- [DDL]
DROP FUNCTION regress_record ;

-- [DDL]
DROP TABLE emp_rec ;

-- [DDL]
create type rec_type is ( c1 int , c2 int );

-- [SESSION]
set behavior_compat_options = 'proc_outparam_override' ;

-- [DDL]
create or replace function func ( a in int ) return rec_type is gaussdb $ # r rec_type ;

-- [PLSQL]
call func ( 0 );

-- [DDL]
drop function func ;

-- [DDL]
drop type rec_type ;

-- [SESSION]
set behavior_compat_options = 'proc_outparam_override' ;

-- [DDL]
create or replace function func ( a out int ) return record is gaussdb $ # type rc is record ( c1 int , c2 int );

-- [PLSQL]
call func ( 1 );

-- [DDL]
drop function func ;


================================================================================
-- 来源: 1428_file_1428.txt
================================================================================

-- [TCL]
BEGIN NULL ;

-- [TCL]
BEGIN dbe_output . print_line ( 'hello world!' );

-- [PLSQL]
DECLARE my_var VARCHAR2 ( 30 );


================================================================================
-- 来源: 1434_file_1434.txt
================================================================================

-- [PLSQL]
DECLARE emp_id INTEGER : = 7788 ;

-- [PLSQL]
DECLARE emp_id INTEGER : = 7788 ;


================================================================================
-- 来源: 1435_file_1435.txt
================================================================================

-- [DDL]
DROP TABLE IF EXISTS customers;

-- [DDL]
CREATE TABLE customers(id int,name varchar);

-- [DML_INSERT]
INSERT INTO customers VALUES(1,'ab');

-- [PLSQL]
DECLARE my_id integer;

-- [PLSQL]
DECLARE type id_list is varray(6) of customers.id%type;


================================================================================
-- 来源: 1436_file_1436.txt
================================================================================

-- [DDL]
CREATE SCHEMA hr ;

-- [SESSION]
SET CURRENT_SCHEMA = hr ;

-- [DDL]
CREATE TABLE staffs ( section_id INTEGER , salary INTEGER );

-- [DML_INSERT]
INSERT INTO staffs VALUES ( 30 , 10 );

-- [DML_INSERT]
INSERT INTO staffs VALUES ( 30 , 20 );

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_staffs ( section NUMBER ( 6 ), salary_sum out NUMBER ( 8 , 2 ), staffs_count out INTEGER ) IS BEGIN SELECT sum ( salary ), count ( * ) INTO salary_sum , staffs_count FROM hr . staffs where section_id = section ;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_return AS v_num NUMBER ( 8 , 2 );

-- [PLSQL]
CALL proc_return ();

-- [DDL]
DROP PROCEDURE proc_staffs ;

-- [DDL]
DROP PROCEDURE proc_return ;

-- [DDL]
CREATE OR REPLACE FUNCTION func_return returns void language plpgsql AS $$ DECLARE v_num INTEGER : = 1 ;

-- [PLSQL]
CALL func_return ();

-- [DDL]
DROP FUNCTION func_return ;

-- [DDL]
DROP SCHEMA hr CASCADE ;


================================================================================
-- 来源: 1438_file_1438.txt
================================================================================

-- [DDL]
DROP SCHEMA IF EXISTS hr CASCADE ;

-- [DDL]
CREATE SCHEMA hr ;

-- [SESSION]
SET CURRENT_SCHEMA = hr ;

-- [DDL]
CREATE TABLE staffs ( staff_id NUMBER , first_name VARCHAR2 , salary NUMBER );

-- [DML_INSERT]
INSERT INTO staffs VALUES ( 200 , 'mike' , 5800 );

-- [DML_INSERT]
INSERT INTO staffs VALUES ( 201 , 'lily' , 3000 );

-- [DML_INSERT]
INSERT INTO staffs VALUES ( 202 , 'john' , 4400 );

-- [PLSQL]
DECLARE staff_count VARCHAR2 ( 20 );

-- [DDL]
CREATE OR REPLACE PROCEDURE dynamic_proc AS staff_id NUMBER ( 6 ) : = 200 ;

-- [PLSQL]
CALL dynamic_proc ();

-- [DDL]
DROP PROCEDURE dynamic_proc ;

-- [DDL]
CREATE SCHEMA hr ;

-- [SESSION]
SET CURRENT_SCHEMA = hr ;

-- [DDL]
CREATE TABLE staffs ( section_id NUMBER , first_name VARCHAR2 , phone_number VARCHAR2 , salary NUMBER );

-- [DML_INSERT]
INSERT INTO staffs VALUES ( 30 , 'mike' , '13567829252' , 5800 );

-- [DML_INSERT]
INSERT INTO staffs VALUES ( 40 , 'john' , '17896354637' , 4000 );

-- [PLSQL]
DECLARE name VARCHAR2 ( 20 );


================================================================================
-- 来源: 1439_file_1439.txt
================================================================================

-- [DDL]
CREATE TABLE sections_t1 ( section NUMBER ( 4 ) , section_name VARCHAR2 ( 30 ), manager_id NUMBER ( 6 ), place_id NUMBER ( 4 ) ) DISTRIBUTE BY hash ( manager_id );

-- [PLSQL]
DECLARE section NUMBER ( 4 ) : = 280 ;

-- [DQL]
SELECT * FROM sections_t1 ;

-- [DDL]
DROP TABLE sections_t1 ;


================================================================================
-- 来源: 1440_file_1440.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_add ( param1 in INTEGER , param2 out INTEGER , param3 in INTEGER ) AS BEGIN param2 : = param1 + param3 ;

-- [PLSQL]
DECLARE input1 INTEGER : = 1 ;

-- [DDL]
DROP PROCEDURE proc_add ;


================================================================================
-- 来源: 1441_file_1441.txt
================================================================================

-- [DDL]
DROP SCHEMA IF EXISTS hr CASCADE ;

-- [DDL]
CREATE SCHEMA hr ;

-- [SESSION]
SET CURRENT_SCHEMA = hr ;

-- [DDL]
CREATE TABLE staffs ( staff_id NUMBER , first_name VARCHAR2 , salary NUMBER );

-- [DML_INSERT]
INSERT INTO staffs VALUES ( 200 , 'mike' , 5800 );

-- [DML_INSERT]
INSERT INTO staffs VALUES ( 201 , 'lily' , 3000 );

-- [DML_INSERT]
INSERT INTO staffs VALUES ( 202 , 'john' , 4400 );

-- [DDL]
CREATE OR REPLACE PROCEDURE dynamic_proc AS staff_id NUMBER ( 6 ) : = 200 ;

-- [PLSQL]
CALL dynamic_proc ();

-- [DDL]
DROP PROCEDURE dynamic_proc ;


================================================================================
-- 来源: 1445_RETURN NEXTRETURN QUERY.txt
================================================================================

-- [DDL]
DROP TABLE t1 ;

-- [DDL]
CREATE TABLE t1 ( a int );

-- [DML_INSERT]
INSERT INTO t1 VALUES ( 1 ),( 10 );

-- [DDL]
CREATE OR REPLACE FUNCTION fun_for_return_next () RETURNS SETOF t1 AS $$ DECLARE r t1 % ROWTYPE ;

-- [PLSQL]
call fun_for_return_next ();

-- [DDL]
CREATE OR REPLACE FUNCTION fun_for_return_query () RETURNS SETOF t1 AS $$ DECLARE r t1 % ROWTYPE ;

-- [PLSQL]
call fun_for_return_query ();


================================================================================
-- 来源: 1446_file_1446.txt
================================================================================

-- [PLSQL]
DECLARE v_user_id integer default 1 ;

-- [PLSQL]
DECLARE v_user_id integer default 0 ;

-- [PLSQL]
DECLARE v_user_id integer default 1 ;

-- [PLSQL]
DECLARE v_user_id integer default NULL ;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_control_structure ( i in integer ) AS BEGIN IF i > 0 THEN raise info 'i:% is greater than 0. ' , i ;

-- [PLSQL]
CALL proc_control_structure ( 3 );

-- [DDL]
DROP PROCEDURE proc_control_structure ;


================================================================================
-- 来源: 1447_file_1447.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_loop ( i in integer , count out integer ) AS BEGIN count : = 0 ;

-- [PLSQL]
CALL proc_loop ( 10 , 5 );

-- [DDL]
CREATE TABLE integertable ( c1 integer ) DISTRIBUTE BY hash ( c1 );

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_while_loop ( maxval in integer ) AS DECLARE i int : = 1 ;

-- [PLSQL]
CALL proc_while_loop ( 10 );

-- [DDL]
DROP PROCEDURE proc_while_loop ;

-- [DDL]
DROP TABLE integertable ;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_for_loop () AS BEGIN FOR I IN 0 .. 5 LOOP DBE_OUTPUT . PRINT_LINE ( 'It is ' || to_char ( I ) || ' time;

-- [PLSQL]
CALL proc_for_loop ();

-- [DDL]
DROP PROCEDURE proc_for_loop ;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_for_loop_query () AS record VARCHAR2 ( 50 );

-- [PLSQL]
CALL proc_for_loop_query ();

-- [DDL]
DROP PROCEDURE proc_for_loop_query ;

-- [DDL]
CREATE TABLE TEST_t1 ( title NUMBER ( 6 ), did VARCHAR2 ( 20 ), data_period VARCHAR2 ( 25 ), kind VARCHAR2 ( 25 ), interval VARCHAR2 ( 20 ), time DATE , isModified VARCHAR2 ( 10 ) ) DISTRIBUTE BY hash ( did );

-- [DML_INSERT]
INSERT INTO TEST_t1 VALUES ( 8 , 'Donald' , 'OConnell' , 'DOCONNEL' , '650.507.9833' , to_date ( '21-06-1999' , 'dd-mm-yyyy' ), 'SH_CLERK' );

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_forall () AS BEGIN FORALL i IN 100 .. 120 update TEST_t1 set title = title + 100 * i ;

-- [PLSQL]
CALL proc_forall ();

-- [DQL]
SELECT * FROM TEST_t1 ;

-- [DDL]
DROP PROCEDURE proc_forall ;

-- [DDL]
DROP TABLE TEST_t1 ;


================================================================================
-- 来源: 1448_file_1448.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_case_branch ( pi_result in integer , pi_return out integer ) AS BEGIN CASE pi_result WHEN 1 THEN pi_return : = 111 ;

-- [PLSQL]
CALL proc_case_branch ( 3 , 0 );

-- [DDL]
DROP PROCEDURE proc_case_branch ;


================================================================================
-- 来源: 1449_file_1449.txt
================================================================================

-- [PLSQL]
DECLARE v_num integer default NULL;


================================================================================
-- 来源: 1450_file_1450.txt
================================================================================

-- [DDL]
CREATE TABLE mytab ( id INT , firstname VARCHAR ( 20 ), lastname VARCHAR ( 20 )) DISTRIBUTE BY hash ( id );

-- [DML_INSERT]
INSERT INTO mytab ( firstname , lastname ) VALUES ( 'Tom' , 'Jones' );

-- [DDL]
CREATE FUNCTION fun_exp () RETURNS INT AS $$ DECLARE x INT : = 0 ;

-- [PLSQL]
call fun_exp ();

-- [DQL]
select * from mytab ;

-- [DDL]
DROP FUNCTION fun_exp ();

-- [DDL]
DROP TABLE mytab ;

-- [DDL]
CREATE TABLE db ( a INT , b TEXT );

-- [DDL]
CREATE FUNCTION merge_db ( key INT , data TEXT ) RETURNS VOID AS $$ BEGIN LOOP --第一次尝试更新key UPDATE db SET b = data WHERE a = key ;

-- [DQL]
SELECT merge_db ( 1 , 'david' );

-- [DQL]
SELECT merge_db ( 1 , 'dennis' );

-- [DDL]
DROP FUNCTION merge_db ;

-- [DDL]
DROP TABLE db ;


================================================================================
-- 来源: 1451_GOTO.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE GOTO_test () AS DECLARE v1 int ;

-- [PLSQL]
call GOTO_test ();


================================================================================
-- 来源: 1452_file_1452.txt
================================================================================

-- [DDL]
DROP TABLE IF EXISTS EXAMPLE1;

-- [DDL]
CREATE TABLE EXAMPLE1(COL1 INT);

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE() AS BEGIN FOR i IN 0..20 LOOP INSERT INTO EXAMPLE1(COL1) VALUES (i);

-- [PLSQL]
call TRANSACTION_EXAMPLE();

-- [DDL]
CREATE OR REPLACE PROCEDURE TEST_COMMIT_INSERT_EXCEPTION_ROLLBACK() AS BEGIN DROP TABLE IF EXISTS TEST_COMMIT;

-- [PLSQL]
call TEST_COMMIT_INSERT_EXCEPTION_ROLLBACK();

-- [TCL]
BEGIN;

-- [DDL]
CREATE OR REPLACE PROCEDURE TEST_COMMIT2() IS BEGIN DROP TABLE IF EXISTS TEST_COMMIT;

-- [PLSQL]
call TEST_COMMIT2();

-- [SESSION]
SHOW explain_perf_mode;

-- [SESSION]
SHOW enable_force_vector_engine;

-- [DDL]
CREATE OR REPLACE PROCEDURE GUC_ROLLBACK() AS BEGIN SET enable_force_vector_engine = on;

-- [PLSQL]
call GUC_ROLLBACK();

-- [SESSION]
SHOW explain_perf_mode;

-- [SESSION]
SHOW enable_force_vector_engine;

-- [SESSION]
SET enable_force_vector_engine = off;

-- [DDL]
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE1() AS BEGIN INSERT INTO EXAMPLE1 VALUES(1);

-- [PLSQL]
call STP_SAVEPOINT_EXAMPLE1();

-- [DDL]
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE2() AS BEGIN INSERT INTO EXAMPLE1 VALUES(2);

-- [TCL]
BEGIN;

-- [PLSQL]
CALL STP_SAVEPOINT_EXAMPLE2();

-- [DQL]
SELECT * FROM EXAMPLE1;

-- [TCL]
COMMIT;

-- [DDL]
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE3() AS BEGIN INSERT INTO EXAMPLE1 VALUES(1);

-- [TCL]
BEGIN;

-- [DML_INSERT]
INSERT INTO EXAMPLE1 VALUES(3);

-- [PLSQL]
CALL STP_SAVEPOINT_EXAMPLE3();

-- [TCL]
ROLLBACK TO SAVEPOINT s1;

-- [DQL]
SELECT * FROM EXAMPLE1;

-- [TCL]
COMMIT;

-- [DDL]
CREATE OR REPLACE FUNCTION FUNCTION_EXAMPLE1() RETURN INT AS EXP INT;

-- [PLSQL]
call FUNCTION_EXAMPLE1();

-- [DDL]
CREATE OR REPLACE FUNCTION FUNCTION_TRI_EXAMPLE2() RETURN TRIGGER AS EXP INT;

-- [DDL]
CREATE TRIGGER TRIGGER_EXAMPLE AFTER DELETE ON EXAMPLE1 FOR EACH ROW EXECUTE PROCEDURE FUNCTION_TRI_EXAMPLE2();

-- [DML_DELETE]
DELETE FROM EXAMPLE1;

-- [DDL]
DROP TABLE IF EXISTS EXAMPLE1;

-- [DDL]
CREATE TABLE EXAMPLE1(COL1 INT);

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE1() IMMUTABLE AS EXP INT;

-- [PLSQL]
CALL TRANSACTION_EXAMPLE1();

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE2(EXP_OUT OUT INT) AS EXP INT;

-- [PLSQL]
CALL TRANSACTION_EXAMPLE2(100);

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE3() AS BEGIN FOR i IN 0..20 LOOP INSERT INTO EXAMPLE1 (col1) VALUES (i);

-- [PLSQL]
CALL TRANSACTION_EXAMPLE3();

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE4() SET ARRAY_NULLS TO "ON" AS BEGIN FOR i IN 0..20 LOOP INSERT INTO EXAMPLE1 (col1) VALUES (i);

-- [PLSQL]
CALL TRANSACTION_EXAMPLE4();

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE5(INTIN IN INT, INTOUT OUT INT) AS BEGIN INTOUT := INTIN + 1;

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE6() AS CURSOR CURSOR1(EXPIN INT) IS SELECT TRANSACTION_EXAMPLE5(EXPIN);

-- [PLSQL]
CALL TRANSACTION_EXAMPLE6();

-- [DDL]
CREATE OR REPLACE PROCEDURE exec_func1() AS BEGIN CREATE TABLE TEST_exec(A INT);

-- [DDL]
CREATE OR REPLACE PROCEDURE exec_func2() AS BEGIN EXECUTE exec_func1();

-- [PLSQL]
CALL exec_func2();

-- [DDL]
CREATE OR REPLACE PROCEDURE exec_func3(RET_NUM OUT INT) AS BEGIN RET_NUM := 1+1;

-- [DDL]
CREATE OR REPLACE PROCEDURE exec_func4(ADD_NUM IN INT) AS SUM_NUM INT;

-- [PLSQL]
CALL exec_func4(1);

-- [DDL]
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE3() AS BEGIN INSERT INTO EXAMPLE1 VALUES(2);

-- [TCL]
BEGIN;


================================================================================
-- 来源: 1458_file_1458.txt
================================================================================

-- [DDL]
drop schema if exists hr cascade;

-- [DDL]
create schema hr;

-- [SESSION]
set current_schema = hr;

-- [DDL]
drop table if exists sections;

-- [DDL]
drop table if exists staffs;

-- [DDL]
drop table if exists department;

--创建部门表
-- [DDL]
create table sections( section_name varchar(100), place_id int, section_id int );

-- [DML_INSERT]
insert into sections values ('hr',1,1);

--创建员工表
-- [DDL]
create table staffs( staff_id number(6), salary number(8,2), section_id int, first_name varchar(20) );

-- [DML_INSERT]
insert into staffs values (1,100,1,'Tom');

--创建部门表
-- [DDL]
create table department( section_id int );

-- [DDL]
CREATE OR REPLACE PROCEDURE cursor_proc1 () AS DECLARE DEPT_NAME VARCHAR ( 100 );

-- [PLSQL]
CALL cursor_proc1 ();

-- [OTHER]
hr ---1 hr ---1 hr ---1 cursor_proc1 -------------- ( 1 row )

-- [DDL]
DROP PROCEDURE cursor_proc1 ;

-- [DDL]
CREATE TABLE hr . staffs_t1 AS TABLE hr . staffs ;

-- [DDL]
CREATE OR REPLACE PROCEDURE cursor_proc2 () AS DECLARE V_EMPNO NUMBER ( 6 );

-- [PLSQL]
CALL cursor_proc2 ();

-- [DDL]
DROP PROCEDURE cursor_proc2 ;

-- [DDL]
DROP TABLE hr . staffs_t1 ;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_sys_ref ( O OUT SYS_REFCURSOR ) IS C1 SYS_REFCURSOR ;

-- [PLSQL]
DECLARE C1 SYS_REFCURSOR ;

-- [DDL]
DROP PROCEDURE proc_sys_ref ;


================================================================================
-- 来源: 1459_file_1459.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_cursor3 () AS DECLARE V_DEPTNO NUMBER ( 4 ) : = 100 ;

-- [PLSQL]
CALL proc_cursor3 ();

-- [DDL]
DROP PROCEDURE proc_cursor3 ;


================================================================================
-- 来源: 1460_file_1460.txt
================================================================================

-- [TCL]
BEGIN FOR ROW_TRANS IN SELECT first_name FROM hr . staffs LOOP DBE_OUTPUT . PRINT_LINE ( ROW_TRANS . first_name );

-- [OTHER]
Tom ANONYMOUS BLOCK EXECUTE --创建表

-- [DDL]
CREATE TABLE integerTable1 ( A INTEGER ) DISTRIBUTE BY hash ( A );

-- [DDL]
CREATE TABLE integerTable2 ( B INTEGER ) DISTRIBUTE BY hash ( B );

-- [DML_INSERT]
INSERT INTO integerTable2 VALUES ( 2 );

-- [DDL]
DROP TABLE integerTable1 ;

-- [DDL]
DROP TABLE integerTable2 ;


================================================================================
-- 来源: 1468_DBE_COMPRESSION.txt
================================================================================

-- [DDL]
create database ilmtabledb with dbcompatibility = 'ORA' ;

-- [OTHER]
\ c ilmtabledb

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE user user1 IDENTIFIED BY 'Gauss_123' ;

-- [SESSION]
SET ROLE user1 PASSWORD 'Gauss_123' ;

-- [DDL]
CREATE TABLE TEST_DATA ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) with ( storage_type = astore ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- [DML_INSERT]
INSERT INTO TEST_DATA VALUES ( 1 , '零食大礼包A' , NOW ());

-- [PLSQL]
DECLARE o_blkcnt_cmp integer ;

-- [DDL]
create database ilmtabledb with dbcompatibility = 'ORA' ;

-- [OTHER]
\ c ilmtabledb

-- [DDL]
alter database set ilm = on ;

-- [DDL]
CREATE user user1 IDENTIFIED BY 'Gauss_1234' ;

-- [SESSION]
SET ROLE user1 PASSWORD 'Gauss_1234' ;

-- [DDL]
CREATE TABLE TEST_DATA ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- [DML_INSERT]
INSERT INTO TEST_DATA VALUES ( 1 , '零食大礼包A' , NOW ());

-- [DQL]
SELECT DBE_COMPRESSION . GET_COMPRESSION_TYPE ( 'user1' , 'test_data' , '(0,1)' , NULL );


================================================================================
-- 来源: 1470_DBE_ILM.txt
================================================================================

-- [DDL]
CREATE DATABASE ilmtabledb with dbcompatibility = 'ORA' ;

-- [OTHER]
\ c ilmtabledb

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE Schema ILM_DATA ;

-- [SESSION]
SET current_schema = ILM_DATA ;

-- [DDL]
CREATE SEQUENCE ILM_DATA . ORDER_TABLE_SE_ORDER_ID MINVALUE 1 ;

-- [DDL]
CREATE OR REPLACE PROCEDURE ILM_DATA . ORDER_TABLE_CREATE_DATA ( NUM INTEGER ) IS BEGIN FOR X IN 1 .. NUM LOOP INSERT INTO ORDER_TABLE VALUES ( ORDER_TABLE_SE_ORDER_ID . nextval , '零食大礼包A' , NOW ());

-- [DDL]
CREATE TABLE ILM_DATA . ORDER_TABLE ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) WITH ( STORAGE_TYPE = ASTORE ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- [PREPARED_STMT]
EXECUTE DIRECT ON DATANODES 'SELECT A.DBNAME, A.JOB_STATUS, A.ENABLE, A.FAILURE_MSG FROM PG_JOB A WHERE A.DBNAME = ''ilmtabledb'' AND A.JOB_NAME LIKE ''ilmjob$_%'' ORDER BY A.JOB_NAME DESC LIMIT 1' ;

-- [PLSQL]
CALL DBE_ILM . STOP_ILM ( - 1 , true , NULL );


================================================================================
-- 来源: 1471_DBE_ILM_ADMIN.txt
================================================================================

-- [OTHER]
DBE_ILM_ADMIN . DISABLE_ILM ();

-- [OTHER]
DBE_ILM_ADMIN . ENABLE_ILM ();

-- [PLSQL]
CALL DBE_ILM_ADMIN . CUSTOMIZE_ILM ( 1 , 15 );

-- [DQL]
select * from gs_adm_ilmparameters ;


================================================================================
-- 来源: 1477_DBE_SCHEDULER.txt
================================================================================

-- [PLSQL]
CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER . create_schedule ( 'schedule1' , NULL , 'sysdate' , NULL , 'test' );

-- [PLSQL]
CALL DBE_SCHEDULER . create_job ( job_name => 'job1' , program_name => 'program1' , schedule_name => 'schedule1' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_schedule ( 'schedule1' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

-- [PLSQL]
CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER . create_schedule ( 'schedule1' , NULL , 'sysdate' , NULL , 'test' );

-- [PLSQL]
CALL DBE_SCHEDULER . create_job ( job_name => 'job1' , program_name => 'program1' , schedule_name => 'schedule1' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_schedule ( 'schedule1' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER.create_job('job1', 'program1', '2021-07-20', 'interval ''3 minute''', '2121-07-20', 'DEFAULT_JOB_CLASS', false, false,'test', 'style', NULL, NULL);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_single_job('job1', false, false);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_program('program1', false);

-- [PLSQL]
CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER . set_attribute ( 'program1' , 'number_of_arguments' , 0 );

-- [PLSQL]
CALL DBE_SCHEDULER . set_attribute ( 'program1' , 'program_type' , 'STORED_PROCEDURE' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

-- [DQL]
SELECT dbe_scheduler . create_job ( 'job1' , 'PLSQL_BLOCK' , 'begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER . run_job ( 'job1' , false );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

-- [DQL]
SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.run_backend_job('job1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [DDL]
create user test1 identified by '*********';

-- [DQL]
select DBE_SCHEDULER.create_credential('cre_1', 'test1', '*********');

-- [DQL]
select DBE_SCHEDULER.create_job(job_name=>'job1', job_type=>'EXTERNAL_SCRIPT', job_action=>'/usr/bin/pwd', enabled=>true, auto_drop=>false, credential_name => 'cre_1');

-- [PLSQL]
CALL DBE_SCHEDULER.run_foreground_job('job1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_credential('cre_1', false);

-- [DDL]
drop user test1;

-- [DQL]
SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.stop_job('job1', true, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [DQL]
SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.stop_single_job('job1', true);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.generate_job_name();

-- [PLSQL]
CALL DBE_SCHEDULER.generate_job_name();

-- [PLSQL]
CALL DBE_SCHEDULER.generate_job_name('job');

-- [PLSQL]
CALL DBE_SCHEDULER.generate_job_name('job');

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_program('program1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER.define_program_argument('program1', 1, 'pa1', 'type1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.define_program_argument('program1', 1, 'pa1', 'type1', 'value1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_program('program1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_program('program1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_single_program('program1', false);

-- [PLSQL]
CALL dbe_scheduler.create_job('job1','EXTERNAL_SCRIPT','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.set_job_argument_value('job1', 1, 'value1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

-- [PLSQL]
CALL DBE_SCHEDULER.drop_schedule('schedule1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_schedule('schedule2', false);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_schedule('schedule3', true);

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

-- [PLSQL]
CALL DBE_SCHEDULER.drop_schedule('schedule1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_schedule('schedule2', false);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_schedule('schedule3', true);

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

-- [PLSQL]
CALL DBE_SCHEDULER.drop_single_schedule('schedule1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_single_schedule('schedule2', false);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_single_schedule('schedule3', true);

-- [PLSQL]
CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job_class('jc1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job_class('jc1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_single_job_class('jc1', false);

-- [DDL]
create user user1 password '1*s*****';

-- [PLSQL]
CALL DBE_SCHEDULER.grant_user_authorization('user1', 'create job');

-- [DDL]
drop user user1;

-- [DDL]
create user user1 password '1*s*****';

-- [PLSQL]
CALL DBE_SCHEDULER.grant_user_authorization('user1', 'create job');

-- [PLSQL]
CALL DBE_SCHEDULER.revoke_user_authorization('user1', 'create job');

-- [DDL]
drop user user1;

-- [PLSQL]
CALL DBE_SCHEDULER.create_credential('cre_1', 'user1', '');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_credential('cre_1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.create_credential('cre_1', 'user1', '');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_credential('cre_1', false);

-- [PLSQL]
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'stored_procedure', 'insert into tb_job_test(key) values(null);

-- [PLSQL]
CALL DBE_SCHEDULER.enable('job1');

-- [PLSQL]
CALL DBE_SCHEDULER.enable('program1', 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_program('program1', false);

-- [PLSQL]
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.enable_single('job1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'stored_procedure', 'insert into tb_job_test(key) values(null);

-- [PLSQL]
CALL DBE_SCHEDULER.disable('job1');

-- [PLSQL]
CALL DBE_SCHEDULER.disable('program1', false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_program('program1', false);

-- [PLSQL]
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.disable_single('job1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.eval_calendar_string('FREQ=DAILY;

-- [DDL]
CREATE OR REPLACE PROCEDURE pr1(calendar_str text) as DECLARE start_date timestamp with time zone;

-- [PLSQL]
CALL pr1('FREQ=hourly;


================================================================================
-- 来源: 1480_DBE_TASK.txt
================================================================================

-- [DQL]
select DBE_TASK . SUBMIT ( 'call pro_xxx();

-- [DQL]
select DBE_TASK . SUBMIT ( 'call pro_xxx();

-- [PLSQL]
DECLARE gaussdb -# jobid int ;

-- [PLSQL]
DECLARE gaussdb -# id integer ;

-- [PLSQL]
CALL dbe_task . id_submit ( 101 , 'insert_msg_statistic1;

-- [PLSQL]
CALL dbe_task.cancel(101);

-- [TCL]
BEGIN gaussdb $ # DBE_TASK . ID_SUBMIT ( 12345 , 'insert_msg_statistic1;

-- [PLSQL]
CALL dbe_task . id_submit ( 101 , 'insert_msg_statistic1;

-- [PLSQL]
CALL dbe_task . finish ( 101 , true );

-- [PLSQL]
CALL dbe_task . finish ( 101 , false , sysdate );

-- [PLSQL]
CALL dbe_task . update ( 101 , 'call userproc();

-- [PLSQL]
CALL dbe_task . update ( 101 , 'insert into tbl_a values(sysdate);

-- [TCL]
BEGIN gaussdb $ # DBE_TASK . CHANGE ( gaussdb $ # job => 101 , gaussdb $ # what => 'insert into t2 values (2);

-- [PLSQL]
CALL dbe_task . content ( 101 , 'call userproc();

-- [PLSQL]
CALL dbe_task . content ( 101 , 'insert into tbl_a values(sysdate);

-- [PLSQL]
CALL dbe_task . next_time ( 101 , sysdate );

-- [PLSQL]
CALL dbe_task . interval ( 101 , 'sysdate + 1.0/1440' );

-- [PLSQL]
CALL dbe_task . cancel ( 101 );


================================================================================
-- 来源: 1485_Retry.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE retry_basic ( IN x INT ) AS BEGIN INSERT INTO t1 ( a ) VALUES ( x );

-- [PLSQL]
CALL retry_basic ( 1 );


================================================================================
-- 来源: 1489_file_1489.txt
================================================================================

-- [DDL]
CREATE TABLE test_trigger_des_tbl(id1 int, id2 int, id3 int);

-- [DDL]
CREATE OR REPLACE FUNCTION tri_insert_func() RETURNS TRIGGER AS $$ DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
DROP TABLE test_trigger_des_tbl;

-- [DDL]
CREATE TABLE t1(a INT ,b TEXT);

-- [PLSQL]
DECLARE

-- [DQL]
SELECT * FROM t1;

-- [DDL]
DROP TABLE t1;

-- [DDL]
CREATE TABLE sections(section_id INT);

-- [DML_INSERT]
INSERT INTO sections VALUES(1);

-- [DML_INSERT]
INSERT INTO sections VALUES(1);

-- [DML_INSERT]
INSERT INTO sections VALUES(1);

-- [DML_INSERT]
INSERT INTO sections VALUES(1);

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_sys_ref(OUT c1 REFCURSOR) IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_sys_call() AS DECLARE c1 SYS_REFCURSOR;

-- [PLSQL]
CALL proc_sys_call();

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_sys_ref(IN c1 REFCURSOR) IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_sys_call() AS DECLARE c1 SYS_REFCURSOR;

-- [PLSQL]
CALL proc_sys_call();

-- [DDL]
DROP PROCEDURE IF EXISTS proc_sys_ref;

-- [DDL]
CREATE OR REPLACE function proc_sys_ref() RETURN SYS_REFCURSOR IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
CREATE OR REPLACE FUNCTION proc_sys_ref(c1 OUT SYS_REFCURSOR) RETURN SYS_REFCURSOR IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_sys_ref(OUT c1 REFCURSOR) IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [TCL]
begin;

-- [PLSQL]
call proc_sys_ref(null);

-- [CURSOR]
fetch "<unnamed portal 1>";

-- [DDL]
DROP PROCEDURE proc_sys_ref;

-- [DDL]
DROP TABLE sections;

-- [DDL]
CREATE OR REPLACE PROCEDURE autonomous_test_in_p_116(num1 INT ) IMMUTABLE AS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
CREATE OR REPLACE PROCEDURE autonomous_test_in_p_117(num1 INT ) STABLE AS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
CREATE TABLE test_lock (id INT,a DATE);

-- [DML_INSERT]
INSERT INTO test_lock VALUES (10,SYSDATE),(11,SYSDATE),(12,SYSDATE);

-- [DDL]
CREATE OR REPLACE FUNCTION autonomous_test_lock(num1 INT,num2 INT) RETURNS INTEGER LANGUAGE plpgsql AS $$ DECLARE num3 INT := 4;

-- [TCL]
START TRANSACTION;

-- [DML_UPDATE]
UPDATE test_lock SET a=SYSDATE WHERE id =11;

-- [PLSQL]
CALL autonomous_test_lock(1,1);

-- [DDL]
DROP TABLE test_lock;

-- [DDL]
CREATE OR REPLACE FUNCTION auto_func() RETURN RECORD AS DECLARE TYPE rec_type IS RECORD(c1 INT, c2 INT);

-- [DQL]
SELECT auto_func();

-- [DDL]
CREATE OR REPLACE PROCEDURE auto_func(r INT) AS DECLARE a INT;

-- [PLSQL]
call auto_func(1);

-- [DDL]
CREATE OR REPLACE FUNCTION test_set() RETURN SETOF INT AS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [PLSQL]
CALL test_set();


================================================================================
-- 来源: 1490_file_1490.txt
================================================================================

-- [DDL]
CREATE TABLE t2(a INT, b INT);

-- [DML_INSERT]
INSERT INTO t2 VALUES(1,2);

-- [DQL]
SELECT * FROM t2;

-- [DDL]
CREATE OR REPLACE PROCEDURE autonomous_4(a INT, b INT) AS DECLARE num3 INT := a;

-- [DDL]
CREATE OR REPLACE PROCEDURE autonomous_5(a INT, b INT) AS DECLARE BEGIN DBE_OUTPUT.PRINT_LINE('JUST NO USE CALL.');

-- [DQL]
SELECT autonomous_5(11,22);

-- [DQL]
SELECT * FROM t2 ORDER BY a;

-- [DDL]
DROP TABLE t2;


================================================================================
-- 来源: 1491_file_1491.txt
================================================================================

-- [DDL]
CREATE TABLE t1(a INT ,B TEXT);

-- [TCL]
START TRANSACTION;

-- [PLSQL]
DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,'YOU WILL ROLLBACK!');

-- [TCL]
ROLLBACK;

-- [DQL]
SELECT * FROM t1;

-- [DDL]
DROP TABLE t1;


================================================================================
-- 来源: 1492_file_1492.txt
================================================================================

-- [DDL]
CREATE TABLE t4(a INT, b INT, c TEXT);

-- [DDL]
CREATE OR REPLACE FUNCTION autonomous_32(a INT ,b INT ,c TEXT) RETURN INT AS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
CREATE OR REPLACE FUNCTION autonomous_33(num1 INT) RETURN INT AS DECLARE num3 INT := 220;

-- [DQL]
SELECT autonomous_33(0);

-- [DQL]
SELECT * FROM t4;

-- [DDL]
DROP TABLE t4;


================================================================================
-- 来源: 1493_PACKAGE.txt
================================================================================

-- [DDL]
create database test DBCOMPATIBILITY = 'ORA';

-- [DDL]
CREATE TABLE t2(a INT, b INT);

-- [DML_INSERT]
INSERT INTO t2 VALUES(1,2);

-- [DQL]
SELECT * FROM t2;

-- [DDL]
CREATE OR REPLACE PACKAGE autonomous_pkg AS PROCEDURE autonomous_4(a INT, b INT);

-- [DDL]
CREATE OR REPLACE PACKAGE BODY autonomous_pkg AS PROCEDURE autonomous_4(a INT, b INT) AS DECLARE num3 INT := a;

-- [DDL]
CREATE OR REPLACE PROCEDURE autonomous_5(a INT, b INT) AS DECLARE va INT;

-- [DQL]
SELECT autonomous_5(11,22);

-- [DQL]
SELECT * FROM t2 ORDER BY a;

-- [DDL]
DROP TABLE t2;


================================================================================
-- 来源: 1909_PG_REPLICATION_SLOTS.txt
================================================================================

-- [DQL]
SELECT * FROM pg_replication_slots;

--在CN上执行查询。
-- [DQL]
SELECT * FROM pg_replication_slots;


================================================================================
-- 来源: 1962_PGXC_THREAD_WAIT_STATUS.txt
================================================================================

-- [DQL]
SELECT * FROM pg_thread_wait_status WHERE query_id > 0 ;

-- [DQL]
SELECT * FROM pgxc_thread_wait_status WHERE query_id > 0 ;


================================================================================
-- 来源: 2120_SESSION_STAT_ACTIVITY.txt
================================================================================

-- [DQL]
SELECT datname, usename, usesysid,state,pid FROM pg_stat_activity;


================================================================================
-- 来源: 2121_GLOBAL_SESSION_STAT_ACTIVITY.txt
================================================================================

-- [DQL]
SELECT datname, usename, usesysid,state,pid FROM pg_stat_activity;


================================================================================
-- 来源: 2292_DBE_PLDEBUGGER Schema.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE test_debug ( IN x INT ) AS BEGIN INSERT INTO t1 ( a ) VALUES ( x );

-- [DQL]
SELECT OID FROM PG_PROC WHERE PRONAME = 'test_debug' ;

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . turn_on ( 16389 );

-- [PLSQL]
call test_debug ( 1 );

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . attach ( 'datanode' , 0 );

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . next ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . info_locals ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . set_var ( 'x' , 2 );

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . print_var ( 'x' );

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . continue ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . continue ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . error_end ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . abort ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . info_code ( 16389 );

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . add_breakpoint ( 16389 , 4 );

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . info_breakpoints ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . continue ();


================================================================================
-- 来源: 2316_file_2316.txt
================================================================================

-- [SESSION]
SHOW server_version ;

-- [SESSION]
SHOW ALL ;

-- [DQL]
SELECT * FROM pg_settings WHERE NAME = 'server_version' ;

-- [DQL]
SELECT * FROM pg_settings ;

-- [SESSION]
SHOW client_encoding ;


================================================================================
-- 来源: 2317_file_2317.txt
================================================================================

-- [SESSION]
SET paraname TO value ;

-- [SESSION]
SHOW hot_standby ;

-- [SESSION]
SHOW authentication_timeout ;

-- [SESSION]
SHOW explain_perf_mode ;

-- [SESSION]
SET explain_perf_mode TO pretty ;

-- [SESSION]
SHOW explain_perf_mode ;

-- [SESSION]
SHOW max_connections ;

-- [OTHER]
\ q 修改 集群 所有CN 的最大连接数。 gs_guc set -Z coordinator -N all -I all -c "max_connections = 800" 重启 集群 。 gs_om -t stop && gs_om -t start 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看最大连接数。 1 2 3 4

-- [SESSION]
SHOW max_connections ;

-- [SESSION]
SHOW authentication_timeout ;

-- [OTHER]
\ q 修改 集群 所有CN 的客户端认证最长时间。 gs_guc reload -Z coordinator -N all -I all -c "authentication_timeout = 59s" 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看客户端认证的最长时间。 1 2 3 4

-- [SESSION]
SHOW authentication_timeout ;

-- [SESSION]
SHOW max_connections ;

-- [OTHER]
\ q 修改 集群 所有 CN和DN 的最大连接数。 gs_guc set -Z coordinator -Z datanode -N all -I all -c "max_connections = 500" 重启 集群 。 gs_om -t stop gs_om -t start 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看最大连接数。 1 2 3 4

-- [SESSION]
SHOW max_connections ;

-- [SESSION]
SHOW authentication_timeout ;

-- [OTHER]
\ q 修改 集群 所有 CN和DN 的客户端认证最长时间。 gs_guc reload -Z coordinator -Z datanode -N all -I all -c "authentication_timeout = 30s" 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看客户端认证的最长时间。 1 2 3 4

-- [SESSION]
SHOW authentication_timeout ;


================================================================================
-- 来源: 2350_file_2350.txt
================================================================================

-- [SESSION]
show logging_module ;

-- [SESSION]
set logging_module = 'on(SSL)' ;

-- [SESSION]
show logging_module ;

-- [SESSION]
set logging_module = 'off(ALL)' ;

-- [SESSION]
show logging_module ;

-- [SESSION]
set logging_module = 'on(ALL)' ;

-- [SESSION]
show logging_module ;


================================================================================
-- 来源: 2366_file_2366.txt
================================================================================

-- [DQL]
select 0 . 1231243 as a , 0 . 1231243 :: numeric as b , 0 . 1231243 :: integer ( 10 , 3 ) as c , length ( 0 . 1242343 ) as d ;

-- [DQL]
select 0 . 1231243 as a , 0 . 1231243 :: numeric as b , 0 . 1231243 :: integer ( 10 , 3 ) as c , length ( 0 . 1242343 ) as d ;

-- [DQL]
select add_months ( '2018-02-28' , 3 ) from sys_dummy ;

-- [DQL]
select add_months ( '2018-02-28' , 3 ) from sys_dummy ;

-- [DQL]
select '' AS to_number_14 , to_number ( '34,50' , '999,99' );

-- [DQL]
select '' AS to_number_14 , to_number ( '34,50' , '999,99' );

-- [DQL]
select ( - 2147483648 ):: int4 / ( - 1 ):: int4 ;

-- [DQL]
select ( - 2147483648 ):: int4 / ( - 1 ):: int4 ;

-- [DDL]
create table test1 ( c1 int , c2 varchar );

-- [DML_INSERT]
insert into test1 values ( 2 , '1.1' );

-- [SESSION]
set behavior_compat_options = '' ;

-- [DQL]
select * from test1 where c2 > 1 ;

-- [SESSION]
set behavior_compat_options = 'convert_string_digit_to_numeric' ;

-- [DQL]
select * from test1 where c2 > 1 ;

-- [DQL]
select length ( lpad ( '123' , 0 , '*' )) from sys_dummy ;

-- [DQL]
select length ( lpad ( '123' , 0 , '*' )) from sys_dummy ;

-- [DQL]
select concat ( variadic NULL :: int []) is NULL ;

-- [DQL]
select concat ( variadic NULL :: int []) is NULL ;

-- [DQL]
select concat ( variadic NULL :: int []) is NULL ;

-- [SESSION]
set behavior_compat_options='hide_tailing_zero';

-- [DQL]
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- [SESSION]
set behavior_compat_options='';

-- [DQL]
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- [SESSION]
set behavior_compat_options='';

-- [DDL]
create table tab_1(col1 varchar(3));

-- [DDL]
create table tab_2(col2 char(3));

-- [DML_INSERT]
insert into tab_2 values(' ');

-- [DML_INSERT]
insert into tab_1 select col2 from tab_2;

-- [DQL]
select * from tab_1 where col1 is null;

-- [DQL]
select * from tab_1 where col1=' ';

-- [DML_DELETE]
delete from tab_1;

-- [SESSION]
set behavior_compat_options = 'char_coerce_compat';

-- [DML_INSERT]
insert into tab_1 select col2 from tab_2;

-- [DQL]
select * from tab_1 where col1 is null;

-- [DQL]
select * from tab_1 where col1=' ';

-- [SESSION]
set behavior_compat_options='truncate_numeric_tail_zero';

-- [DQL]
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- [SESSION]
set behavior_compat_options='';

-- [DQL]
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- [DDL]
create or replace function test(f1 int, f2 int default 20, f3 int, f4 int default 40, f5 int default 50) return int gaussdb -# as gaussdb $# begin gaussdb $# raise info 'f1:%',f1;

-- [DQL]
select test(1,2);

-- [DDL]
create or replace function test(f1 int, f2 int default 20, f3 int, f4 int default 40, f5 int default 50) return int gaussdb -# as gaussdb $# begin gaussdb $# raise info 'f1:%',f1;

-- [DQL]
select test(1,2);

-- [DQL]
SELECT power(2,3);

-- [DQL]
SELECT count(*) FROM db_ind_columns;

-- [DQL]
SELECT count(index_name) FROM db_ind_columns;

-- [DQL]
SELECT left('abcde', 2);

-- [DQL]
SELECT pg_client_encoding();

-- [SESSION]
SET behavior_compat_options = 'enable_funcname_with_argsname';

-- [DQL]
SELECT power(2,3);

-- [DQL]
SELECT count(*) FROM db_ind_columns;

-- [DQL]
SELECT count(index_name) FROM db_ind_columns;

-- [DQL]
SELECT left('abcde', 2);

-- [DQL]
SELECT pg_client_encoding();

-- [SESSION]
SET behavior_compat_options='proc_outparam_override,proc_outparam_transfer_length';

-- [DDL]
CREATE OR REPLACE PROCEDURE out_param_test1(m in int, v inout varchar2,v1 inout varchar2) is gaussdb$# begin gaussdb$# v := 'aaaddd';

-- [DDL]
CREATE OR REPLACE PROCEDURE call_out_param_test1 is gaussdb$# v varchar2(5) := 'aabbb';

-- [PLSQL]
CALL call_out_param_test1();

-- [DDL]
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of varchar(5);

-- [PLSQL]
CALL p1();

-- [SESSION]
SET behavior_compat_options = 'tableof_elem_constraints';

-- [DDL]
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of varchar(5);

-- [PLSQL]
CALL p1();

-- [DDL]
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of int index by varchar(5);

-- [PLSQL]
CALL p1();

-- [SESSION]
SET behavior_compat_options = 'tableof_elem_constraints';

-- [DDL]
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of int index by varchar(5);

-- [PLSQL]
CALL p1();

-- [SESSION]
set behavior_compat_options='current_sysdate';

-- [DQL]
select sysdate;

-- [DDL]
create or replace function proc_test return varchar2 as gaussdb$# begin gaussdb$# return '1';

-- [DDL]
create or replace procedure proc_test as gaussdb$# begin gaussdb$# null;

-- 设置参数后允许替换类型
-- [SESSION]
set behavior_compat_options='allow_function_procedure_replace';

-- [DDL]
create or replace procedure proc_test as gaussdb$# begin gaussdb$# null;

-- [DDL]
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- [PLSQL]
call p1();

-- [DDL]
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- [PLSQL]
call p1();

-- [DDL]
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- [PLSQL]
call p1();

-- [SESSION]
set behavior_compat_options = 'collection_exception_backcompat';

-- [DDL]
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- [PLSQL]
call p1();

-- [DDL]
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- [PLSQL]
call p1();

-- [DDL]
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- [PLSQL]
call p1();

-- [SESSION]
set behavior_compat_options='enable_case_when_alias';

-- [DDL]
create table test(c1 varchar2);

-- [DML_INSERT]
insert into test values('x');

-- [DQL]
select decode(c1,'x','0','default') from test;

-- [DQL]
select (case c1 when 'x' then '0' else 'default' end) from test;

-- [DDL]
create user plsql_rollback1 password 'huawei@123';

-- [DDL]
create user plsql_rollback2 password 'huawei@123';

-- [DCL_GRANT]
grant plsql_rollback1 to plsql_rollback2;

-- [DDL]
create or replace procedure plsql_rollback1.p1 () authid definer as gaussdb$# va int;

-- [SESSION]
set session AUTHORIZATION plsql_rollback2 PASSWORD 'huawei@123';

-- [DQL]
select timestamp '2024-03-20 01:30:00’ at time zone 'Europe/Moscow' from dual;

-- [SESSION]
set behavior_compat_options='enable_use_ora_timestamptz';

-- [DQL]
select timestamp '2024-03-20 01:30:00’ at time zone 'Europe/Moscow' from dual;

-- [SESSION]
set gs_format_behavior_compat_options='allow_textconcat_null';

-- [DQL]
select 'a' || null || 'b';


================================================================================
-- 来源: 2431_file_2431.txt
================================================================================

-- [DDL]
create table a(id int, value int);

-- [DML_INSERT]
insert into a values(1,4);

-- [DML_INSERT]
insert into a values(2,4);

-- [TCL]
start transaction isolation level repeatable read;

-- [DQL]
select * from a;

-- [DML_UPDATE]
update a set value = 6 where id = 1;

-- [DQL]
select * from a;

-- [TCL]
start transaction isolation level repeatable read;

-- [DQL]
select * from a;

-- [DML_UPDATE]
update a set value = 6 where id = 2;

-- [DQL]
select * from a;

-- [TCL]
commit;

-- [TCL]
commit;

-- [DQL]
select * from a;

-- [DDL]
create table a(id int primary key, value int);

-- [DML_INSERT]
insert into a values(1,10);

-- [TCL]
start transaction isolation level repeatable read;

-- [DML_DELETE]
delete a where id = 1;

-- [TCL]
start transaction isolation level repeatable read;

-- [DQL]
select * from a;

-- [TCL]
commit;

-- [DML_INSERT]
insert into a values(1, 100);

-- [DQL]
select * from a;


================================================================================
-- 来源: 2436_file_2436.txt
================================================================================

-- [DDL]
CREATE USER sysadmin WITH SYSADMIN password "********" ;

-- [DDL]
ALTER USER joe SYSADMIN ;

-- [DDL]
CREATE USER createrole WITH CREATEROLE password "********" ;

-- [DDL]
ALTER USER joe CREATEROLE ;

-- [DDL]
CREATE USER auditadmin WITH AUDITADMIN password "********" ;

-- [DDL]
ALTER USER joe AUDITADMIN ;

-- [DDL]
CREATE USER monadmin WITH MONADMIN password "********" ;

-- [DDL]
ALTER USER joe MONADMIN ;

-- [DDL]
CREATE USER opradmin WITH OPRADMIN password "********" ;

-- [DDL]
ALTER USER joe OPRADMIN ;

-- [DDL]
CREATE USER poladmin WITH POLADMIN password "********" ;

-- [DDL]
ALTER USER joe POLADMIN ;


================================================================================
-- 来源: 2438_file_2438.txt
================================================================================

-- [DDL]
CREATE USER joe WITH CREATEDB PASSWORD "********" ;

-- [DQL]
SELECT * FROM pg_user ;

-- [DQL]
SELECT * FROM pg_authid ;

-- [DDL]
CREATE USER user_persistence WITH PERSISTENCE IDENTIFIED BY "********" ;


================================================================================
-- 来源: 2440_Schema.txt
================================================================================

-- [DQL]
SELECT s . nspname , u . usename AS nspowner FROM pg_namespace s , pg_user u WHERE nspname = 'schema_name' AND s . nspowner = u . usesysid ;

-- [DQL]
SELECT * FROM pg_namespace ;

-- [DQL]
SELECT distinct ( tablename ), schemaname from pg_tables where schemaname = 'pg_catalog' ;

-- [SESSION]
SHOW SEARCH_PATH ;

-- [SESSION]
SET SEARCH_PATH TO myschema , public ;


================================================================================
-- 来源: 2441_file_2441.txt
================================================================================

-- [DCL_GRANT]
GRANT USAGE ON SCHEMA tpcds TO joe ;

-- [DCL_GRANT]
GRANT SELECT ON TABLE tpcds . web_returns to joe ;

-- [DDL]
CREATE ROLE lily WITH CREATEDB PASSWORD "********" ;

-- [DCL_GRANT]
GRANT USAGE ON SCHEMA tpcds TO lily ;

-- [DCL_GRANT]
GRANT SELECT ON TABLE tpcds . web_returns to lily ;

-- [DCL_GRANT]
GRANT lily to joe ;


================================================================================
-- 来源: 2442_file_2442.txt
================================================================================

-- [DDL]
CREATE USER alice PASSWORD '********' ;

-- [DDL]
CREATE USER bob PASSWORD '********' ;

-- [DDL]
CREATE USER peter PASSWORD '********' ;

-- [DDL]
CREATE TABLE all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

-- [DML_INSERT]
INSERT INTO all_data VALUES ( 1 , 'alice' , 'alice data' );

-- [DML_INSERT]
INSERT INTO all_data VALUES ( 2 , 'bob' , 'bob data' );

-- [DML_INSERT]
INSERT INTO all_data VALUES ( 3 , 'peter' , 'peter data' );

-- [DCL_GRANT]
GRANT SELECT ON all_data TO alice , bob , peter ;

-- [DDL]
ALTER TABLE all_data ENABLE ROW LEVEL SECURITY ;

-- [OTHER]
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

-- [OTHER]
\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_rls" FOR ALL TO public USING ((( role ):: name = "current_user" ())) Has OIDs : no Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --切换至用户alice，执行SQL"SELECT * FROM public.all_data"

-- [DQL]
SELECT * FROM public . all_data ;

-- [EXPLAIN]
EXPLAIN ( COSTS OFF ) SELECT * FROM public . all_data ;

-- [DQL]
SELECT * FROM public . all_data ;

-- [EXPLAIN]
EXPLAIN ( COSTS OFF ) SELECT * FROM public . all_data ;


================================================================================
-- 来源: 2449_file_2449.txt
================================================================================

-- [DDL]
CREATE USER joe WITH PASSWORD "********" ;

-- [DCL_GRANT]
GRANT ALL PRIVILEGES TO joe;


================================================================================
-- 来源: 2450_file_2450.txt
================================================================================

-- [DDL]
CREATE DATABASE db_tpcc ;

-- [DDL]
CREATE DATABASE db_tpcc WITH TABLESPACE = hr_local ;

-- [OTHER]
gsql ((GaussDB Kernel XXX.X.XXX build f521c606) compiled at 2021-09-16 14:55:22 commit 2935 last mr 6385 release) Non-SSL connection (SSL connection is recommended when requiring high-security) Type "help" for help. db_tpcc=> 查看数据库 使用\l元命令查看数据库系统的数据库列表。

-- [OTHER]
\ l 使用如下命令通过系统表pg_database查询数据库列表。

-- [DQL]
SELECT datname FROM pg_database ;

-- [DDL]
ALTER DATABASE db_tpcc SET search_path TO pa_catalog , public ;

-- [DDL]
ALTER DATABASE db_tpcc RENAME TO human_tpcds ;

-- [DDL]
DROP DATABASE human_tpcds ;


================================================================================
-- 来源: 2451_file_2451.txt
================================================================================

-- [DDL]
CREATE TABLE customer_t1 ( state_ID CHAR ( 2 ), state_NAME VARCHAR2 ( 40 ), area_ID NUMBER );

-- [DDL]
DROP TABLE customer_t1 ;

-- [DDL]
CREATE TABLE customer_t2 ( state_ID CHAR ( 2 ), state_NAME VARCHAR2 ( 40 ), area_ID NUMBER ) WITH ( ORIENTATION = COLUMN );

-- [DDL]
DROP TABLE customer_t2 ;


================================================================================
-- 来源: 2452_file_2452.txt
================================================================================

-- [DDL]
CREATE USER jack IDENTIFIED BY '********' ;

-- [DDL]
CREATE TABLESPACE fastspace RELATIVE LOCATION 'tablespace/tablespace_1' ;

-- [DCL_GRANT]
GRANT CREATE ON TABLESPACE fastspace TO jack ;

-- [DDL]
CREATE TABLE foo ( i int ) TABLESPACE fastspace ;

-- [SESSION]
SET default_tablespace = 'fastspace' ;

-- [DDL]
CREATE TABLE foo2 ( i int );

-- [DQL]
SELECT spcname FROM pg_tablespace ;

-- [DQL]
SELECT PG_TABLESPACE_SIZE ( 'example' );

-- [DDL]
ALTER TABLESPACE fastspace RENAME TO fspace ;

-- [DDL]
DROP USER jack CASCADE ;

-- [DDL]
DROP TABLE foo ;

-- [DDL]
DROP TABLE foo2 ;

-- [DDL]
DROP TABLESPACE fspace ;


================================================================================
-- 来源: 2454_file_2454.txt
================================================================================

-- [DDL]
CREATE TABLE customer_t1 ( c_customer_sk integer , c_customer_id char ( 5 ), c_first_name char ( 6 ), c_last_name char ( 8 ) );


================================================================================
-- 来源: 2455_file_2455.txt
================================================================================

-- [DDL]
CREATE TABLE table1 ( id int , a char ( 6 ), b varchar ( 6 ), c varchar ( 6 ));

-- [DDL]
CREATE TABLE table2 ( id int , a char ( 20 ), b varchar ( 20 ), c varchar ( 20 ));

-- [DML_INSERT]
INSERT INTO table1 VALUES ( 1 , reverse ( '123ＡＡ78' ), reverse ( '123ＡＡ78' ), reverse ( '123ＡＡ78' ));

-- [DML_INSERT]
INSERT INTO table1 VALUES ( 2 , reverse ( '123Ａ78' ), reverse ( '123Ａ78' ), reverse ( '123Ａ78' ));

-- [DML_INSERT]
INSERT INTO table1 VALUES ( 3 , '87Ａ123' , '87Ａ123' , '87Ａ123' );

-- [DML_INSERT]
INSERT INTO table2 VALUES ( 1 , reverse ( '123ＡＡ78' ), reverse ( '123ＡＡ78' ), reverse ( '123ＡＡ78' ));

-- [DML_INSERT]
INSERT INTO table2 VALUES ( 2 , reverse ( '123Ａ78' ), reverse ( '123Ａ78' ), reverse ( '123Ａ78' ));

-- [DML_INSERT]
INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 3769 , 'hello' , 'Grace' );

-- [DML_INSERT]
INSERT INTO customer_t1 VALUES ( 3769 , 'hello' , 'Grace' );

-- [DML_INSERT]
INSERT INTO customer_t1 ( c_customer_sk , c_first_name ) VALUES ( 3769 , 'Grace' );

-- [DML_INSERT]
INSERT INTO customer_t1 VALUES ( 3769 , 'hello' );

-- [DML_INSERT]
INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 3769 , 'hello' , DEFAULT );

-- [DML_INSERT]
INSERT INTO customer_t1 DEFAULT VALUES ;

-- [DML_INSERT]
INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 6885 , 'maps' , 'Joes' ), ( 4321 , 'tpcds' , 'Lily' ), ( 9527 , 'world' , 'James' );

-- [DDL]
CREATE TABLE customer_t2 ( c_customer_sk integer , c_customer_id char ( 5 ), c_first_name char ( 6 ), c_last_name char ( 8 ) );

-- [DML_INSERT]
INSERT INTO customer_t2 SELECT * FROM customer_t1 ;

-- [DDL]
DROP TABLE customer_t2 CASCADE ;


================================================================================
-- 来源: 2456_file_2456.txt
================================================================================

-- [DML_UPDATE]
UPDATE customer_t1 SET c_customer_sk = 9876 WHERE c_customer_sk = 9527 ;

-- [DML_UPDATE]
UPDATE customer_t1 SET c_customer_sk = c_customer_sk + 100 ;

-- [DML_UPDATE]
UPDATE customer_t1 SET c_customer_id = 'Admin' , c_first_name = 'Local' WHERE c_customer_sk = 4421 ;


================================================================================
-- 来源: 2457_file_2457.txt
================================================================================

-- [DQL]
SELECT * FROM pg_tables ;

-- [OTHER]
\ d + customer_t1 ;

-- [DQL]
SELECT count ( * ) FROM customer_t1 ;

-- [DQL]
SELECT * FROM customer_t1 ;

-- [DQL]
SELECT c_customer_sk FROM customer_t1 ;

-- [DQL]
SELECT DISTINCT ( c_customer_sk ) FROM customer_t1 ;

-- [DQL]
SELECT * FROM customer_t1 WHERE c_customer_sk = 3869 ;

-- [DQL]
SELECT * FROM customer_t1 ORDER BY c_customer_sk ;


================================================================================
-- 来源: 2458_file_2458.txt
================================================================================

-- [DML_DELETE]
DELETE FROM customer_t1 WHERE c_customer_sk = 3869 ;

-- [DML_DELETE]
DELETE FROM customer_t1 ;

-- [DML_TRUNCATE]
TRUNCATE TABLE customer_t1 ;

-- [DDL]
DROP TABLE customer_t1 ;


================================================================================
-- 来源: 2459_file_2459.txt
================================================================================

-- [DDL]
CREATE TABLE public.search_table_t1(a int);

-- [DDL]
CREATE TABLE public.search_table_t2(b int);

-- [DDL]
CREATE TABLE public.search_table_t3(c int);

-- [DDL]
CREATE TABLE public.search_table_t4(d int);

-- [DDL]
CREATE TABLE public.search_table_t5(e int);

-- [DQL]
SELECT distinct ( tablename ) FROM pg_tables WHERE SCHEMANAME = 'public' AND TABLENAME LIKE 'search_table%' ;


================================================================================
-- 来源: 2461_schema.txt
================================================================================

-- [DDL]
CREATE SCHEMA myschema ;

-- [DDL]
CREATE SCHEMA myschema AUTHORIZATION omm ;

-- [DDL]
CREATE TABLE myschema . mytable ( id int , name varchar ( 20 ));

-- [DQL]
SELECT * FROM myschema . mytable ;

-- [SESSION]
SHOW SEARCH_PATH ;

-- [SESSION]
SET SEARCH_PATH TO myschema , public ;

-- [DCL_REVOKE]
REVOKE CREATE ON SCHEMA public FROM PUBLIC ;

-- [DQL]
SELECT current_schema ();

-- [DDL]
CREATE USER jack IDENTIFIED BY '********' ;

-- [DCL_GRANT]
GRANT USAGE ON schema myschema TO jack ;

-- [DCL_REVOKE]
REVOKE USAGE ON schema myschema FROM jack ;

-- [DDL]
DROP SCHEMA IF EXISTS nullschema ;

-- [DDL]
DROP SCHEMA myschema CASCADE ;

-- [DDL]
DROP USER jack ;


================================================================================
-- 来源: 2462_file_2462.txt
================================================================================

-- [DDL]
CREATE TABLE tpcds . customer_address ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) ) ENABLE ROW MOVEMENT ;

-- [DDL]
CREATE TABLE tpcds . web_returns_p2 ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) ) ENABLE ROW MOVEMENT ;

-- [DML_INSERT]
INSERT INTO tpcds . web_returns_p2 SELECT * FROM tpcds . customer_address ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 DISABLE ROW MOVEMENT ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 DROP PARTITION P8 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 ADD PARTITION P8 VALUES LESS THAN ( MAXVALUE );

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION P8 TO P_9 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION FOR ( 40000 ) TO P8 ;

-- [DQL]
SELECT * FROM tpcds . web_returns_p2 PARTITION ( P6 );

-- [DQL]
SELECT * FROM tpcds . web_returns_p2 PARTITION FOR ( 35888 );

-- [DDL]
DROP TABLE tpcds . customer_address ;

-- [DDL]
DROP TABLE tpcds . web_returns_p2 ;

-- [DDL]
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1/tablespace_1' ;

-- [DDL]
CREATE TABLESPACE example2 RELATIVE LOCATION 'tablespace2/tablespace_2' ;

-- [DDL]
CREATE TABLESPACE example3 RELATIVE LOCATION 'tablespace3/tablespace_3' ;

-- [DDL]
CREATE TABLESPACE example4 RELATIVE LOCATION 'tablespace4/tablespace_4' ;

-- [DDL]
CREATE TABLE tpcds . customer_address ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) TABLESPACE example1 PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) TABLESPACE example2 ) ENABLE ROW MOVEMENT ;

-- [DDL]
CREATE TABLE tpcds . web_returns_p2 ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) TABLESPACE example1 PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) TABLESPACE example2 ) ENABLE ROW MOVEMENT ;

-- [DML_INSERT]
INSERT INTO tpcds . web_returns_p2 SELECT * FROM tpcds . customer_address ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 DISABLE ROW MOVEMENT ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 DROP PARTITION P8 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 ADD PARTITION P8 VALUES LESS THAN ( MAXVALUE );

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION P8 TO P_9 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION FOR ( 40000 ) TO P8 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P6 TABLESPACE example3 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P4 TABLESPACE example4 ;

-- [DQL]
SELECT * FROM tpcds . web_returns_p2 PARTITION ( P6 );

-- [DQL]
SELECT * FROM tpcds . web_returns_p2 PARTITION FOR ( 35888 );

-- [DDL]
DROP TABLE tpcds . web_returns_p2 ;

-- [DDL]
DROP TABLESPACE example1 ;

-- [DDL]
DROP TABLESPACE example2 ;

-- [DDL]
DROP TABLESPACE example3 ;

-- [DDL]
DROP TABLESPACE example4 ;


================================================================================
-- 来源: 2463_file_2463.txt
================================================================================

-- [DDL]
CREATE INDEX tpcds_web_returns_p2_index1 ON tpcds . web_returns_p2 ( ca_address_id ) LOCAL ;

-- [DDL]
CREATE INDEX tpcds_web_returns_p2_index2 ON tpcds . web_returns_p2 ( ca_address_sk ) LOCAL ( PARTITION web_returns_p2_P1_index , PARTITION web_returns_p2_P2_index TABLESPACE example3 , PARTITION web_returns_p2_P3_index TABLESPACE example4 , PARTITION web_returns_p2_P4_index , PARTITION web_returns_p2_P5_index , PARTITION web_returns_p2_P6_index , PARTITION web_returns_p2_P7_index , PARTITION web_returns_p2_P8_index ) TABLESPACE example2 ;

-- [DDL]
ALTER INDEX tpcds . tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P2_index TABLESPACE example1 ;

-- [DDL]
ALTER INDEX tpcds . tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P3_index TABLESPACE example2 ;

-- [DDL]
ALTER INDEX tpcds . tpcds_web_returns_p2_index2 RENAME PARTITION web_returns_p2_P8_index TO web_returns_p2_P8_index_new ;

-- [DQL]
SELECT RELNAME FROM PG_CLASS WHERE RELKIND = 'i' or RELKIND = 'I' ;

-- [OTHER]
\ di + tpcds . tpcds_web_returns_p2_index2 删除索引 1

-- [DDL]
DROP INDEX tpcds . tpcds_web_returns_p2_index1 ;

-- [DDL]
DROP INDEX tpcds . tpcds_web_returns_p2_index2 ;

-- [DDL]
CREATE TABLE tpcds . customer_address_bak AS TABLE tpcds . customer_address ;

-- [DQL]
SELECT ca_address_sk FROM tpcds . customer_address_bak WHERE ca_address_sk = 14888 ;

-- [DDL]
CREATE INDEX index_wr_returned_date_sk ON tpcds . customer_address_bak ( ca_address_sk );

-- [DDL]
CREATE UNIQUE INDEX ds_ship_mode_t1_index1 ON tpcds. ship_mode_t1(SM_SHIP_MODE_SK);

-- [DQL]
SELECT ca_address_sk , ca_address_id FROM tpcds . customer_address_bak WHERE ca_address_sk = 5050 AND ca_street_number < 1000 ;

-- [DDL]
CREATE INDEX more_column_index ON tpcds . customer_address_bak ( ca_address_sk , ca_street_number );

-- [DDL]
CREATE INDEX part_index ON tpcds . customer_address_bak ( ca_address_sk ) WHERE ca_address_sk = 5050 ;

-- [DQL]
SELECT * FROM tpcds . customer_address_bak WHERE trunc ( ca_street_number ) < 1000 ;

-- [DDL]
CREATE INDEX para_index ON tpcds . customer_address_bak ( trunc ( ca_street_number ));

-- [DDL]
DROP TABLE tpcds . customer_address_bak ;


================================================================================
-- 来源: 2464_file_2464.txt
================================================================================

-- [DDL]
CREATE OR REPLACE VIEW MyView AS SELECT * FROM tpcds . web_returns WHERE trunc ( wr_refunded_cash ) > 10000 ;

-- [DQL]
SELECT * FROM MyView ;

-- [OTHER]
\ d + MyView View "PG_CATALOG.MyView" Column | Type | Modifiers | Storage | Description ----------+-----------------------+-----------+----------+------------- USERNAME | CHARACTER VARYING ( 64 ) | | extended | View definition : SELECT PG_AUTHID . ROLNAME :: CHARACTER VARYING ( 64 ) AS USERNAME FROM PG_AUTHID ;

-- [DDL]
DROP VIEW MyView ;


================================================================================
-- 来源: 2465_file_2465.txt
================================================================================

-- [DDL]
CREATE TABLE T1 ( id serial , name text );

-- [DDL]
CREATE SEQUENCE seq1 cache 100 ;

-- [DDL]
CREATE TABLE T2 ( id int not null default nextval ( 'seq1' ), name text );

-- [DDL]
ALTER SEQUENCE seq1 OWNED BY T2 . id ;


================================================================================
-- 来源: 2466_file_2466.txt
================================================================================

-- [DDL]
CREATE TABLE test ( id int , time date );

-- [DDL]
CREATE OR REPLACE PROCEDURE PRC_JOB_1 () AS N_NUM integer : = 1 ;

-- [PLSQL]
call dbe_task . submit ( 'call public.prc_job_1();

-- [PLSQL]
call dbe_task . id_submit ( 1 , 'call public.prc_job_1();

-- [DQL]
select job , dbname , start_date , last_date , this_date , next_date , broken , status , interval , failures , what from my_jobs ;

-- [PLSQL]
call dbe_task . finish ( 1 , true );

-- [PLSQL]
call dbe_task . finish ( 1 , false );

-- [PLSQL]
call dbe_task . next_date ( 1 , sysdate + 1 . 0 / 24 );

-- [PLSQL]
call dbe_task . interval ( 1 , 'sysdate + 1.0/24' );

-- [PLSQL]
call dbe_task . content ( 1 , 'insert into public.test values(333, sysdate+5);

-- [PLSQL]
call dbe_task . update ( 1 , 'call public.prc_job_1();

-- [PLSQL]
call dbe_task . cancel ( 1 );


================================================================================
-- 来源: 2478_SQL.txt
================================================================================

-- [DQL]
SELECT CURRENT_DATE ;

-- [DQL]
SELECT CURRENT_TIME ;

-- [DQL]
SELECT CURRENT_TIMESTAMP ( 6 );


================================================================================
-- 来源: 2688_SQL.txt
================================================================================

-- [EXPLAIN]
explain select * from t1,t2 where t1.c1 = t2.c2;

-- [EXPLAIN]
explain select * from t1,t2 where t1.c1=t2.c2;


================================================================================
-- 来源: 2689_file_2689.txt
================================================================================

-- [EXPLAIN]
EXPLAIN SELECT * FROM t1,t2 WHERE t1.c1 = t2.c2;

-- [EXPLAIN]
explain performance select sum(t2.c1) from t1,t2 where t1.c1=t2.c2 group by t1.c2;


================================================================================
-- 来源: 2702_file_2702.txt
================================================================================

-- [EXPLAIN]
explain ( analyze on , costs off ) select * from t1 where c1 = 10004 ;

-- [DDL]
create index idx on t1 ( c1 );

-- [EXPLAIN]
explain ( analyze on , costs off ) select * from t1 where c1 = 10004 ;

-- [EXPLAIN]
explain analyze select count(*) from t1,t2 where t1.c1=t2.c2;

-- [SESSION]
set enable_mergejoin=off;

-- [SESSION]
set enable_nestloop=off;

-- [EXPLAIN]
explain analyze select count(*) from t1,t2 where t1.c1=t2.c2;

-- [EXPLAIN]
explain analyze select count(*) from t1 group by c1;

-- [SESSION]
set enable_sort=off;

-- [EXPLAIN]
explain analyze select count(*) from t1 group by c1;


================================================================================
-- 来源: 2707_HintQueryblock.txt
================================================================================

-- [SESSION]
set explain_perf_mode = pretty;

-- [EXPLAIN]
explain (blockname on,costs off) select * from t1, (select c1 from t2 group by c1) sub1 where t1.c1 = sub1.c1;

-- [EXPLAIN]
explain (blockname on,costs off) select /*+indexscan(@sel$2 t2) tablescan(t1)*/ * from t1, (select c1 from t2 group by c1) sub1 where t1.c1 = sub1.c1;

-- [EXPLAIN]
explain (blockname on,costs off) select * from t2, (select c1 from t1 where t1.c3 = 2) sub1 where t2.c1 = sub1.c1;

-- [EXPLAIN]
explain (blockname on,costs off) select * from t2, (select /*+ no_expand*/ c1 from t1 where t1.c3 = 2) sub1 where t2.c1 = sub1.c1;

-- [EXPLAIN]
explain (blockname on,costs off) select/*+ indexscan(@sel$2 t1)*/ * from t2, (select c1 from t1 where t1.c3 = 2) sub1 where t2.c1 = sub1.c1;

-- [DDL]
create view v1 as select/*+ no_expand */ c1 from t1 where c1 in (select /*+ no_expand */ c1 from t2 where t2.c3=4 );

-- [EXPLAIN]
explain (blockname on,costs off) select * from v1;


================================================================================
-- 来源: 2708_Hintschema.txt
================================================================================

-- [EXPLAIN]
explain(blockname on,costs off) select /*+ indexscan(t1)*/ * from t1, (select c2 from t1 where c1=1) tt1 where t1.c1 = tt1.c2;

-- [EXPLAIN]
explain(blockname on,costs off) select /*+ indexscan(t1@sel$2)*/ * from t1, (select c2 from t1 where c1=1) tt1 where t1.c1 = tt1.c2;


================================================================================
-- 来源: 2713_ScanHint.txt
================================================================================

-- 使用索引
-- [EXPLAIN]
EXPLAIN SELECT /*+ gsi(gsi_test gsi_test_idx) */ * FROM gsi_test where b = 1;

-- [EXPLAIN]
EXPLAIN SELECT /*+ gsitable(gsi_test gsi_test_idx) */ * FROM gsi_test where b = 1;


================================================================================
-- 来源: 2714_Hint.txt
================================================================================

-- [EXPLAIN]
explain select /*+ blockname(@sel$2 bn2) tablescan(@bn2 t2) tablescan(@sel$2 t2@bn2) indexscan(@sel$2 t2@sel$2) tablescan(@bn3 t3@bn3)*/ c2 from t1 where c1 in ( select /*+ */t2.c1 from t2 where t2.c2 = 1 group by 1) and c3 in ( select /*+ blockname(bn3)*/t3.c3 from t3 where t3.c2 = 1 group by 1);

-- [EXPLAIN]
explain select /*+ blockname(@sel$2 bn2) hashjoin(t1 bn2) nestloop(t1 bn3) nestloop(t1 sel$3)*/ c2 from t1 where c1 in ( select /*+ */t2.c1 from t2 where t2.c2 = 1 group by 1) and c3 in ( select /*+ blockname(bn3)*/t3.c3 from t3 where t3.c2 = 1 group by 1);


================================================================================
-- 来源: 2719_Hint.txt
================================================================================

-- [PREPARED_STMT]
deallocate all;

-- [PREPARED_STMT]
prepare p1 as insert /*+ no_gpc*/ into t1 select c1,c2 from t2 where c1=$1;

-- [PREPARED_STMT]
execute p1(3);

-- [DQL]
select * from dbe_perf.global_plancache_status where schema_name='public' order by 1,2;


================================================================================
-- 来源: 2720_Hint.txt
================================================================================

-- [EXPLAIN]
explain (costs off) select /*+nestloop_index(t1,(t2 t3)) */* from t1,t2,t3 where t1.c1 = t2.c1 and t1.c2 = t3.c2;

-- [EXPLAIN]
explain (costs off) select /*+NestLoop_Index(t1,it1) */* from t1,t2 where t1.c1 = t2.c1;

-- [SESSION]
SET rewrite_rule = 'predpushforce' ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM t1, t2 WHERE t1.c1 = t2.c1;

-- [EXPLAIN]
EXPLAIN SELECT /*+predpush_same_level(t1, t2)*/ * FROM t1, t2 WHERE t1.c1 = t2.c1;


================================================================================
-- 来源: 2724_bitmapscanHint.txt
================================================================================

-- [EXPLAIN]
explain(costs off) select /*+ BitmapScan(t1 it1 it3)*/* from t1 where (t1.c1 = 5 or t1.c2=6) or (t1.c3=3 or t1.c2=7);


================================================================================
-- 来源: 2725_Hint.txt
================================================================================

-- [EXPLAIN]
explain (costs off) select /*+materialize_inner(t1) materialize_inner(t1 t2)*/ * from t1,t2,t3 where t1.c3 = t2.c3 and t2.c2=t3.c2 and t1.c2=5;


================================================================================
-- 来源: 2726_aggHint.txt
================================================================================

-- [EXPLAIN]
explain (costs off) select c1 from t2 where c1 in( select /*+ use_hash_agg */ t1.c1 from t1,t3 where t1.c1=t3.c1 group by 1);

-- [EXPLAIN]
explain (costs off) select c1 from t2 where c1 in( select /*+ use_sort_agg */ t1.c1 from t1,t3 where t1.c1=t3.c1 group by 1);


================================================================================
-- 来源: 2727_Hint.txt
================================================================================

-- [EXPLAIN]
EXPLAIN(costs off)SELECT /*+EXPAND_SUBLINK_HAVING*/ a,sum(b) AS value FROM rewrite_rule_hint_t1 GROUP BY a HAVING sum(a) >= (SELECT avg(b) FROM rewrite_rule_hint_t1) ORDER BY value DESC;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT /*+NO_EXPAND_SUBLINK_HAVING*/ a,sum(b) AS value FROM rewrite_rule_hint_t1 GROUP BY a HAVING sum(a) >= (SELECT avg(b) FROM rewrite_rule_hint_t1) ORDER BY value DESC;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE a > ANY(SELECT /*+EXPAND_SUBLINK*/ a FROM rewrite_rule_hint_t2) AND b > ANY (SELECT /*+EXPAND_SUBLINK*/a FROM rewrite_rule_hint_t3);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE a > ANY(SELECT /*+NO_EXPAND_SUBLINK*/ a FROM rewrite_rule_hint_t2) AND b > ANY (SELECT /*+EXPAND_SUBLINK*/a FROM rewrite_rule_hint_t3);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT a,(SELECT /*+EXPAND_SUBLINK_TARGET*/ avg(b) FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = rewrite_rule_hint_t2.b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a < 100 ORDER BY rewrite_rule_hint_t2.b;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT a,(SELECT /*+NO_EXPAND_SUBLINK_TARGET*/ avg(b) FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = rewrite_rule_hint_t2.b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a < 100 ORDER BY rewrite_rule_hint_t2.b;

-- [EXPLAIN]
EXPLAIN(costs off) SELECT rewrite_rule_hint_t1 FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = 10 AND rewrite_rule_hint_t1.c < (SELECT /*+USE_MAGIC_SET*/ sum(c) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.a);

-- [EXPLAIN]
EXPLAIN(costs off) SELECT rewrite_rule_hint_t1 FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = 10 AND rewrite_rule_hint_t1.c < (SELECT /*+NO_USE_MAGIC_SET*/ sum(c) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.a);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.a = (SELECT /*+EXPAND_SUBLINK_UNIQUE_CHECK*/ rewrite_rule_hint_t2.a FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.b);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.a = (SELECT /*+NO_EXPAND_SUBLINK_UNIQUE_CHECK*/ rewrite_rule_hint_t2.a FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t1.a = rewrite_rule_hint_t2.b);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE (0 =(SELECT /*+NO_SUBLINK_DISABLE_REPLICATED*/ count(*) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a) OR NOT EXISTS(SELECT /*+NO_SUBLINK_DISABLE_REPLICATED*/1 FROM rewrite_rule_hint_t3 WHERE rewrite_rule_hint_t3.b = rewrite_rule_hint_t1.b));

-- [EXPLAIN]
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1 WHERE (0 =(SELECT /*+SUBLINK_DISABLE_REPLICATED*/ count(*) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a) OR NOT EXISTS(SELECT /*+NO_SUBLINK_DISABLE_REPLICATED*/1 FROM rewrite_rule_hint_t3 WHERE rewrite_rule_hint_t3.b = rewrite_rule_hint_t1.b));

-- [EXPLAIN]
EXPLAIN(costs off)SELECT a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = (SELECT /*+NO_SUBLINK_DISABLE_EXPR*/ max(b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT a FROM rewrite_rule_hint_t1 WHERE rewrite_rule_hint_t1.b = (SELECT /*+SUBLINK_DISABLE_EXPR*/ max(b) FROM rewrite_rule_hint_t2 WHERE rewrite_rule_hint_t2.a = rewrite_rule_hint_t1.a);

-- [EXPLAIN]
EXPLAIN (costs off)SELECT cntrycode,count(*) AS numcust,sum(c_acctbal) AS totacctbal FROM (SELECT substring(c_phone from 1 for 2) AS cntrycode,c_acctbal FROM rewrite_rule_hint_customer WHERE substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')AND c_acctbal > (SELECT /*+ENABLE_SUBLINK_ENHANCED*/ avg(c_acctbal) FROM rewrite_rule_hint_customer WHERE c_acctbal > 0.00 AND substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')) AND NOT EXISTS (SELECT * FROM rewrite_rule_hint_orders WHERE o_custkey = c_custkey)) AS custsale GROUP BY cntrycode ORDER BY cntrycode;

-- [EXPLAIN]
EXPLAIN (costs off)SELECT cntrycode,count(*) AS numcust,sum(c_acctbal) AS totacctbal FROM (SELECT substring(c_phone from 1 for 2) AS cntrycode,c_acctbal FROM rewrite_rule_hint_customer WHERE substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')AND c_acctbal > (SELECT /*+NO_ENABLE_SUBLINK_ENHANCED*/ avg(c_acctbal) FROM rewrite_rule_hint_customer WHERE c_acctbal > 0.00 AND substring(c_phone from 1 for 2) IN ('22', '25', '26', '14', '18', '30', '17')) AND NOT EXISTS (SELECT * FROM rewrite_rule_hint_orders WHERE o_custkey = c_custkey)) AS custsale GROUP BY cntrycode ORDER BY cntrycode;

-- [SESSION]
SET rewrite_rule='intargetlist';

-- [SESSION]
SET rewrite_rule='intargetlist';

-- [EXPLAIN]
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1,(SELECT /*+REDUCE_ORDER_BY*/ * FROM rewrite_rule_hint_t2 ORDER BY a DESC);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT * FROM rewrite_rule_hint_t1,(SELECT /*+NO_REDUCE_ORDER_BY*/ * FROM rewrite_rule_hint_t2 ORDER BY a DESC);

-- [EXPLAIN]
EXPLAIN(costs off)SELECT /*+REMOVE_NOT_NULL*/ * FROM rewrite_rule_hint_t4 WHERE b > 10 OR a IS NOT NULL;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT /*+NO_REMOVE_NOT_NULL*/ * FROM rewrite_rule_hint_t4 WHERE b > 10 OR a IS NOT NULL;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.b,sum(cc) FROM (SELECT /*+LAZY_AGG*/b,sum(c) AS cc FROM rewrite_rule_hint_t2 GROUP BY b) s1,rewrite_rule_hint_t1 WHERE s1.b = rewrite_rule_hint_t1.b GROUP BY rewrite_rule_hint_t1.b ORDER BY 1,2;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT rewrite_rule_hint_t1.b,sum(cc) FROM (SELECT /*+NO_LAZY_AGG*/b,sum(c) AS cc FROM rewrite_rule_hint_t2 GROUP BY b) s1,rewrite_rule_hint_t1 WHERE s1.b = rewrite_rule_hint_t1.b GROUP BY rewrite_rule_hint_t1.b ORDER BY 1,2;

-- [EXPLAIN]
EXPLAIN(costs off) SELECT * FROM rewrite_rule_hint_t1,(SELECT /*+EXPAND_SUBQUERY*/ * FROM rewrite_rule_hint_t2 WHERE a > 1) tt WHERE rewrite_rule_hint_t1.a = tt.a;

-- [EXPLAIN]
EXPLAIN(costs off) SELECT * FROM rewrite_rule_hint_t1,(SELECT /*+NO_EXPAND_SUBQUERY*/ * FROM rewrite_rule_hint_t2 WHERE a > 1) tt WHERE rewrite_rule_hint_t1.a = tt.a;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT /*+PUSHDOWN_HAVING*/ sum(a),b,c FROM rewrite_rule_hint_t1 WHERE b > 0 GROUP BY b,c HAVING sum(a) > 100 AND c > 0;

-- [EXPLAIN]
EXPLAIN(costs off)SELECT /*+NO_PUSHDOWN_HAVING*/ sum(a),b,c FROM rewrite_rule_hint_t1 WHERE b > 0 GROUP BY b,c HAVING sum(a) > 100 AND c > 0;

-- [EXPLAIN]
EXPLAIN (costs off)SELECT * FROM rewrite_rule_hint_t5 WHERE slot = '5' AND (name) IN (SELECT /*+INLIST_TO_JOIN*/ name FROM rewrite_rule_hint_t5 WHERE slot = '5'AND cid IN (5,1000,1001,1002,1003,1004,1005,1006,1007,2000,4000,10781986,10880002)LIMIT 50);

-- [EXPLAIN]
EXPLAIN (costs off)SELECT * FROM rewrite_rule_hint_t5 WHERE slot = '5' AND (name) IN (SELECT /*+NO_INLIST_TO_JOIN*/ name FROM rewrite_rule_hint_t5 WHERE slot = '5'AND cid IN (5,1000,1001,1002,1003,1004,1005,1006,1007,2000,4000,10781986,10880002)LIMIT 50);

-- [EXPLAIN]
EXPLAIN (costs off) SELECT * FROM (SELECT /*+ROWNUM_PUSHDOWN*/rownum rn, a FROM rewrite_rule_hint_t1) WHERE rn BETWEEN 5 AND 10;

-- [EXPLAIN]
EXPLAIN (costs off) SELECT * FROM (SELECT /*+NO_ROWNUM_PUSHDOWN*/rownum rn, a FROM rewrite_rule_hint_t1) WHERE rn BETWEEN 5 AND 10;


================================================================================
-- 来源: 2728_Outline Hint.txt
================================================================================

-- [EXPLAIN]
EXPLAIN (OUTLINE ON, COSTS OFF) SELECT * FROM t1 JOIN t2 ON t1.a = t2.a;

-- [EXPLAIN]
EXPLAIN (OUTLINE ON, COSTS OFF) SELECT /*+ BEGIN_OUTLINE_DATA HashJoin(@"sel$1" t1@"sel$1" t2@"sel$1") Leading(@"sel$1" (t1@"sel$1" t2@"sel$1")) TableScan(@"sel$1" t1@"sel$1") TableScan(@"sel$1" t2@"sel$1") VERSION("1.0.0") END_OUTLINE_DATA */ * FROM t1 JOIN t2 ON t1.a = t2.a;

-- [EXPLAIN]
EXPLAIN (OUTLINE ON, COSTS OFF) SELECT /*+ NestLoop(@"sel$1" t1@"sel$1" t2@"sel$1") Leading(@"sel$1" (t1@"sel$1" t2@"sel$1")) TableScan(@"sel$1" t1@"sel$1") TableScan(@"sel$1" t2@"sel$1") */ * FROM t1 JOIN t2 ON t1.a = t2.a;

-- [EXPLAIN]
EXPLAIN (OUTLINE ON, COSTS OFF) SELECT /*+ BEGIN_OUTLINE_DATA NestLoop(@"sel$1" t1@"sel$1" t2@"sel$1") Leading(@"sel$1" (t1@"sel$1" t2@"sel$1")) TableScan(@"sel$1" t1@"sel$1") TableScan(@"sel$1" t2@"sel$1") VERSION("1.0.0") END_OUTLINE_DATA */ * from t1 join t2 on t1.a = t2.a;


================================================================================
-- 来源: 2729_file_2729.txt
================================================================================

-- [SESSION]
SHOW try_vector_engine_strategy;

-- [SESSION]
SET try_vector_engine_strategy=force;

-- [SESSION]
SHOW try_vector_engine_strategy;


================================================================================
-- 来源: 2731_SQL PATCH.txt
================================================================================

-- [DDL]
create table hint_t1 ( a int , b int , c int );

-- [DDL]
create index on hint_t1 ( a );

-- [DML_INSERT]
insert into hint_t1 values ( 1 , 1 , 1 );

-- [SESSION]
set track_stmt_stat_level = 'L1,L1' ;

-- [SESSION]
set explain_perf_mode = normal ;

-- [DQL]
select * from hint_t1 t1 where t1 . a = 1 ;

-- [OTHER]
\ x --切换扩展显示模式，便于观察计划 Expanded display is on .

-- [DQL]
select unique_query_id , query , query_plan from dbe_perf . statement_history where query like '%hint_t1%' ;

-- [OTHER]
\ x Expanded display is off .

-- [DQL]
select * from dbe_sql_util . create_hint_sql_patch ( 'patch1' , 2311517824 , 'indexscan(t1)' );

-- [EXPLAIN]
explain select * from hint_t1 t1 where t1 . a = 1 ;

-- [DQL]
select * from hint_t1 t1 where t1 . a = 1 ;

-- [OTHER]
\ x Expanded display is on .

-- [DQL]
select unique_query_id , query , query_plan from dbe_perf . statement_history where query like '%hint_t1%' ;

-- [DQL]
select * from dbe_sql_util.drop_sql_patch('patch1');

-- [DQL]
select * from dbe_sql_util.create_abort_sql_patch('patch2', 2311517824);

-- [DQL]
select * from hint_t1 t1 where t1.a = 1;

-- [DDL]
create table test_proc_patch(a int,b int);

-- [DML_INSERT]
insert into test_proc_patch values(1,2);

-- [DDL]
create index test_a on test_proc_patch(a);

-- [DDL]
create procedure mypro() as num int;

-- [SESSION]
set track_stmt_stat_level = 'L0,L1';

-- [DQL]
select b from test_proc_patch where a = 1;

-- [PLSQL]
call mypro();

-- [SESSION]
set track_stmt_stat_level = 'OFF,L0';

-- [DQL]
select unique_query_id, query, query_plan, parent_unique_sql_id from dbe_perf.statement_history where query like '%call mypro();

-- 根据parentid可以调用重载函数限制存储过程内生效
-- [DQL]
select * from dbe_sql_util.create_hint_sql_patch('patch1',2859505004,3460545602,'indexscan(test_proc_patch)');

-- [DQL]
select patch_name,unique_sql_id,parent_unique_sql_id,enable,abort,hint_string from gs_sql_patch where patch_name = 'patch1';

-- [SESSION]
set track_stmt_stat_level = 'L0,L1';

-- [DQL]
select b from test_proc_patch where a = 1;

-- [PLSQL]
call mypro();

-- [DQL]
select unique_query_id, query, query_plan, parent_unique_sql_id from dbe_perf.statement_history where query like '%test_proc_patch%' order by start_time;


================================================================================
-- 来源: 2733_GUCrewrite_rule.txt
================================================================================

-- [SESSION]
set rewrite_rule='none';

-- [DDL]
create table t1(c1 int,c2 int);

-- [DDL]
create table t2(c1 int,c2 int);

-- [EXPLAIN]
explain (verbose on, costs off) select c1,(select avg(c2) from t2 where t2.c2=t1.c2) from t1 where t1.c1<100 order by t1.c2;

-- [SESSION]
set rewrite_rule='intargetlist';

-- [EXPLAIN]
explain (verbose on, costs off) select c1,(select avg(c2) from t2 where t2.c2=t1.c2) from t1 where t1.c1<100 order by t1.c2;

-- [SESSION]
set rewrite_rule='uniquecheck';

-- [EXPLAIN]
explain verbose select t1.c1 from t1 where t1.c1 = (select t2.c1 from t2 where t1.c1=t2.c1);

-- [DQL]
select * from t1 order by c2;

-- [DQL]
select * from t2 order by c2;

-- [DQL]
select t1.c1 from t1 where t1.c1 = (select t2.c1 from t2 where t1.c1=t2.c2) ;

-- [SESSION]
set rewrite_rule='uniquecheck';

-- [DQL]
select t1.c1 from t1 where t1.c1 = (select t2.c1 from t2 where t1.c1=t2.c2) ;

-- [SESSION]
set rewrite_rule =none;

-- [DDL]
create table t (a int, b int, c int, d int);

-- [DDL]
create table t1 (a int, b int, c int, d int);

-- [EXPLAIN]
explain (costs off) select t.b, sum(cc) from (select b, sum(c) as cc from t1 group by b) s1, t where s1.b=t.b group by t.b order by 1,2;

-- [SESSION]
set rewrite_rule = lazyagg;

-- [EXPLAIN]
explain (costs off) select t.b, sum(cc) from (select b, sum(c) as cc from t1 group by b) s1, t where s1.b=t.b group by t.b order by 1,2;

-- [DDL]
create table t1(a int, b varchar, c int, d int);

-- [DDL]
create table t2(a int, b varchar, c int, d int);

-- [SESSION]
set rewrite_rule = none;

-- [EXPLAIN]
explain (costs off) select t1 from t1 where t1.b = 10 and t1.c < (select sum(c) from t2 where t1.a = t2.a);


================================================================================
-- 来源: 2743_file_2743.txt
================================================================================

-- [DDL]
CREATE TABLE int_type_t1 ( IT_COL1 TINYINT , IT_COL2 TINYINT UNSIGNED );

-- [DML_INSERT]
INSERT INTO int_type_t1 VALUES ( 10 , 20 );

-- [DQL]
SELECT * FROM int_type_t1 ;

-- [DDL]
DROP TABLE int_type_t1 ;

-- [DDL]
CREATE TABLE int_type_t2 ( a TINYINT , b TINYINT , c INTEGER , d INTEGER UNSIGNED , e BIGINT , f BIGINT UNSIGNED );

-- [DML_INSERT]
INSERT INTO int_type_t2 VALUES ( 100 , 10 , 1000 , 10000 , 200 , 2000 );

-- [DQL]
SELECT * FROM int_type_t2 ;

-- [DDL]
DROP TABLE int_type_t2 ;

-- [DDL]
CREATE TABLE decimal_type_t1 ( DT_COL1 DECIMAL(10,4) );

--插入数据。
-- [DML_INSERT]
INSERT INTO decimal_type_t1 VALUES(123456.122331);

--查询表中的数据。
-- [DQL]
SELECT * FROM decimal_type_t1;

--删除表。
-- [DDL]
DROP TABLE decimal_type_t1;

-- [DDL]
CREATE TABLE numeric_type_t1 ( NT_COL1 NUMERIC ( 10 , 4 ) );

-- [DML_INSERT]
INSERT INTO numeric_type_t1 VALUES ( 123456 . 12354 );

-- [DQL]
SELECT * FROM numeric_type_t1 ;

-- [DDL]
DROP TABLE numeric_type_t1 ;

-- [DDL]
CREATE TABLE smallserial_type_tab ( a SMALLSERIAL );

-- [DML_INSERT]
INSERT INTO smallserial_type_tab VALUES ( default );

-- [DML_INSERT]
INSERT INTO smallserial_type_tab VALUES ( default );

-- [DQL]
SELECT * FROM smallserial_type_tab ;

-- [DDL]
CREATE TABLE serial_type_tab ( b SERIAL );

-- [DML_INSERT]
INSERT INTO serial_type_tab VALUES ( default );

-- [DML_INSERT]
INSERT INTO serial_type_tab VALUES ( default );

-- [DQL]
SELECT * FROM serial_type_tab ;

-- [DDL]
CREATE TABLE bigserial_type_tab ( c BIGSERIAL );

-- [DML_INSERT]
INSERT INTO bigserial_type_tab VALUES ( default );

-- [DML_INSERT]
INSERT INTO bigserial_type_tab VALUES ( default );

-- [DQL]
SELECT * FROM bigserial_type_tab ;

-- [DDL]
CREATE TABLE largeserial_type_tab ( c LARGESERIAL );

-- [DML_INSERT]
INSERT INTO largeserial_type_tab VALUES ( default );

-- [DML_INSERT]
INSERT INTO largeserial_type_tab VALUES ( default );

-- [DQL]
SELECT * FROM largeserial_type_tab ;

-- [DDL]
DROP TABLE smallserial_type_tab ;

-- [DDL]
DROP TABLE serial_type_tab ;

-- [DDL]
DROP TABLE bigserial_type_tab ;

-- [DDL]
CREATE TABLE float_type_t2 ( FT_COL1 INTEGER , FT_COL2 FLOAT4 , FT_COL3 FLOAT8 , FT_COL4 FLOAT ( 3 ), FT_COL5 BINARY_DOUBLE , FT_COL6 DECIMAL ( 10 , 4 ), FT_COL7 INTEGER ( 6 , 3 ) );

-- [DML_INSERT]
INSERT INTO float_type_t2 VALUES ( 10 , 10 . 365456 , 123456 . 1234 , 10 . 3214 , 321 . 321 , 123 . 123654 , 123 . 123654 );

-- [DQL]
SELECT * FROM float_type_t2 ;

-- [DDL]
DROP TABLE float_type_t2 ;

-- [DDL]
CREATE DATABASE gaussdb_m WITH dbcompatibility 'B' ;

-- [OTHER]
\ c gaussdb_m --设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;


================================================================================
-- 来源: 2744_file_2744.txt
================================================================================

-- [DQL]
SELECT '12.34' :: float8 :: numeric :: money ;

-- [DQL]
SELECT '52093.89' :: money :: numeric :: float8 ;


================================================================================
-- 来源: 2745_file_2745.txt
================================================================================

-- [DDL]
CREATE TABLE bool_type_t1 ( BT_COL1 BOOLEAN , BT_COL2 TEXT );

-- [DML_INSERT]
INSERT INTO bool_type_t1 VALUES ( TRUE , 'sic est' );

-- [DML_INSERT]
INSERT INTO bool_type_t1 VALUES ( FALSE , 'non est' );

-- [DQL]
SELECT * FROM bool_type_t1 ;

-- [DQL]
SELECT * FROM bool_type_t1 WHERE bt_col1 = 't' ;

-- [DDL]
DROP TABLE bool_type_t1 ;


================================================================================
-- 来源: 2746_file_2746.txt
================================================================================

-- [DDL]
CREATE TABLE varchar_maxlength_test1 (a int, b varchar, c int);

-- varchar为1073741728，超过规定长度，插入失败
-- [DML_INSERT]
insert into varchar_maxlength_test1 values(1, repeat('a', 1073741728), 1);

-- varchar为1073741727，长度符合要求，插入成功
-- [DML_INSERT]
insert into varchar_maxlength_test1 values(1, repeat('a', 1073741727), 1);

-- 创建表，表中仅varchar一列，根据计算规则，varchar最大存储长度为1GB-85-4=
-- [DDL]
CREATE TABLE varchar_maxlength_test2 (a varchar);

-- [DDL]
CREATE TABLE char_type_t1 ( CT_COL1 CHARACTER(4) );

--插入数据。
-- [DML_INSERT]
INSERT INTO char_type_t1 VALUES ('ok');

--查询表中的数据。
-- [DQL]
SELECT ct_col1, char_length(ct_col1) FROM char_type_t1;

--删除表。
-- [DDL]
DROP TABLE char_type_t1;

--创建表。
-- [DDL]
CREATE TABLE char_type_t2 ( CT_COL1 VARCHAR(5) );

--插入数据。
-- [DML_INSERT]
INSERT INTO char_type_t2 VALUES ('ok');

-- [DML_INSERT]
INSERT INTO char_type_t2 VALUES ('good');

--插入的数据长度超过类型规定的长度报错。
-- [DML_INSERT]
INSERT INTO char_type_t2 VALUES ('too long');

--明确类型的长度，超过数据类型长度后会自动截断。
-- [DML_INSERT]
INSERT INTO char_type_t2 VALUES ('too long'::varchar(5));

--查询数据。
-- [DQL]
SELECT ct_col1, char_length(ct_col1) FROM char_type_t2;

--删除数据。
-- [DDL]
DROP TABLE char_type_t2;

-- [DDL]
create database gaussdb_m with dbcompatibility 'b';


================================================================================
-- 来源: 2747_file_2747.txt
================================================================================

-- [DDL]
CREATE TABLE blob_type_t1 ( BT_COL1 INTEGER , BT_COL2 BLOB , BT_COL3 RAW , BT_COL4 BYTEA ) ;

-- [DML_INSERT]
INSERT INTO blob_type_t1 VALUES ( 10 , empty_blob (), HEXTORAW ( 'DEADBEEF' ), E '\\xDEADBEEF' );

-- [DQL]
SELECT * FROM blob_type_t1 ;

-- [DDL]
DROP TABLE blob_type_t1 ;

-- [DDL]
CREATE DATABASE gaussdb_m WITH dbcompatibility 'B' ;

-- [OTHER]
\ c gaussdb_m --设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;


================================================================================
-- 来源: 2748__.txt
================================================================================

-- [DDL]
CREATE TABLE date_type_tab ( coll date );

-- [DML_INSERT]
INSERT INTO date_type_tab VALUES ( date '12-10-2010' );

-- [DQL]
SELECT * FROM date_type_tab ;

-- [DDL]
DROP TABLE date_type_tab ;

-- [DDL]
CREATE TABLE time_type_tab ( da time without time zone , dai time with time zone , dfgh timestamp without time zone , dfga timestamp with time zone , vbg smalldatetime );

-- [DML_INSERT]
INSERT INTO time_type_tab VALUES ( '21:21:21' , '21:21:21 pst' , '2010-12-12' , '2013-12-11 pst' , '2003-04-12 04:05:06' );

-- [DQL]
SELECT * FROM time_type_tab ;

-- [DDL]
DROP TABLE time_type_tab ;

-- [DDL]
CREATE TABLE day_type_tab ( a int , b INTERVAL DAY ( 3 ) TO SECOND ( 4 ));

-- [DML_INSERT]
INSERT INTO day_type_tab VALUES ( 1 , INTERVAL '3' DAY );

-- [DQL]
SELECT * FROM day_type_tab ;

-- [DDL]
DROP TABLE day_type_tab ;

-- [DDL]
CREATE TABLE year_type_tab ( a int , b interval year ( 6 ));

-- [DML_INSERT]
INSERT INTO year_type_tab VALUES ( 1 , interval '2' year );

-- [DQL]
SELECT * FROM year_type_tab ;

-- [DDL]
DROP TABLE year_type_tab ;

-- [DDL]
create database gaussdb_m dbcompatibility = 'B' ;

-- [DDL]
CREATE TABLE date_type_tab ( coll date );

-- [DML_INSERT]
INSERT INTO date_type_tab VALUES ( date '12-10-2010' );

-- [DQL]
SELECT * FROM date_type_tab ;

-- [SESSION]
SHOW datestyle ;

-- [SESSION]
SET datestyle = 'YMD' ;

-- [DML_INSERT]
INSERT INTO date_type_tab VALUES ( date '2010-12-11' );

-- [DQL]
SELECT * FROM date_type_tab ;

-- [DDL]
DROP TABLE date_type_tab ;

-- [DQL]
SELECT time '04:05:06' ;

-- [DQL]
SELECT time '04:05:06 PST' ;

-- [DQL]
SELECT time with time zone '04:05:06 PST' ;

-- [DDL]
CREATE TABLE realtime_type_special(col1 varchar(20), col2 date, col3 timestamp, col4 time);

--插入数据。
-- [DML_INSERT]
INSERT INTO realtime_type_special VALUES('epoch', 'epoch', 'epoch', NULL);

-- [DML_INSERT]
INSERT INTO realtime_type_special VALUES('now', 'now', 'now', 'now');

-- [DML_INSERT]
INSERT INTO realtime_type_special VALUES('today', 'today', 'today', NULL);

-- [DML_INSERT]
INSERT INTO realtime_type_special VALUES('tomorrow', 'tomorrow', 'tomorrow', NULL);

-- [DML_INSERT]
INSERT INTO realtime_type_special VALUES('yesterday', 'yesterday', 'yesterday', NULL);

--查看数据。
-- [DQL]
SELECT * FROM realtime_type_special;

-- [DQL]
SELECT * FROM realtime_type_special WHERE col3 < 'infinity';

-- [DQL]
SELECT * FROM realtime_type_special WHERE col3 > '-infinity';

-- [DQL]
SELECT * FROM realtime_type_special WHERE col3 > 'now';

-- [DQL]
SELECT * FROM realtime_type_special WHERE col3 = 'today';

-- [DQL]
SELECT * FROM realtime_type_special WHERE col3 = 'tomorrow';

-- [DQL]
SELECT * FROM realtime_type_special WHERE col3 > 'yesterday';

-- [DQL]
SELECT TIME 'allballs';

--删除表。
-- [DDL]
DROP TABLE realtime_type_special;

-- [DDL]
CREATE TABLE reltime_type_tab ( col1 character ( 30 ), col2 reltime );

-- [DML_INSERT]
INSERT INTO reltime_type_tab VALUES ( '90' , '90' );

-- [DML_INSERT]
INSERT INTO reltime_type_tab VALUES ( '-366' , '-366' );

-- [DML_INSERT]
INSERT INTO reltime_type_tab VALUES ( '1975.25' , '1975.25' );

-- [DML_INSERT]
INSERT INTO reltime_type_tab VALUES ( '-2 YEARS +5 MONTHS 10 DAYS' , '-2 YEARS +5 MONTHS 10 DAYS' );

-- [DML_INSERT]
INSERT INTO reltime_type_tab VALUES ( '30 DAYS 12:00:00' , '30 DAYS 12:00:00' );

-- [DML_INSERT]
INSERT INTO reltime_type_tab VALUES ( 'P-1.1Y10M' , 'P-1.1Y10M' );

-- [DQL]
SELECT * FROM reltime_type_tab ;

-- [DDL]
DROP TABLE reltime_type_tab ;


================================================================================
-- 来源: 2751_file_2751.txt
================================================================================

-- [DDL]
CREATE TABLE bit_type_t1 ( BT_COL1 INTEGER , BT_COL2 BIT ( 3 ), BT_COL3 BIT VARYING ( 5 ) ) ;

-- [DML_INSERT]
INSERT INTO bit_type_t1 VALUES ( 1 , B '101' , B '00' );

-- [DML_INSERT]
INSERT INTO bit_type_t1 VALUES ( 2 , B '10' , B '101' );

-- [DML_INSERT]
INSERT INTO bit_type_t1 VALUES ( 2 , B '10' :: bit ( 3 ), B '101' );

-- [DQL]
SELECT * FROM bit_type_t1 ;

-- [DDL]
DROP TABLE bit_type_t1 ;


================================================================================
-- 来源: 2752_file_2752.txt
================================================================================

-- [DQL]
SELECT 'a fat cat sat on a mat and ate a fat rat' :: tsvector ;

-- [DQL]
SELECT $$ the lexeme ' ' contains spaces $$ :: tsvector ;

-- [DQL]
SELECT $$ the lexeme 'Joe''s' contains a quote $$ :: tsvector ;

-- [DQL]
SELECT 'a:1 fat:2 cat:3 sat:4 on:5 a:6 mat:7 and:8 ate:9 a:10 fat:11 rat:12' :: tsvector ;

-- [DQL]
SELECT 'a:1A fat:2B,4C cat:5D' :: tsvector ;

-- [DQL]
SELECT 'The Fat Rats' :: tsvector ;

-- [DQL]
SELECT to_tsvector ( 'english' , 'The Fat Rats' );

-- [DQL]
SELECT 'fat & rat' :: tsquery ;

-- [DQL]
SELECT 'fat & (rat | cat)' :: tsquery ;

-- [DQL]
SELECT 'fat & rat & ! cat' :: tsquery ;

-- [DQL]
SELECT 'fat:ab & cat' :: tsquery ;

-- [DQL]
SELECT 'super:*' :: tsquery ;

-- [DQL]
SELECT to_tsvector ( 'seriousness' ) @@ to_tsquery ( 'series:*' ) AS RESULT ;

-- [DQL]
SELECT to_tsquery ( 'series:*' );

-- [DQL]
SELECT to_tsquery ( 'Fat:ab & Cats' );


================================================================================
-- 来源: 2754_JSON_JSONB.txt
================================================================================

-- [DQL]
SELECT 'null'::json;

-- [DQL]
SELECT 'NULL'::jsonb;

-- [DQL]
SELECT '1'::json;

-- [DQL]
SELECT '-1.5'::json;

-- [DQL]
SELECT '-1.5e-5'::jsonb, '-1.5e+2'::jsonb;

-- [DQL]
SELECT '001'::json, '+15'::json, 'NaN'::json;

-- [DQL]
SELECT 'true'::json;

-- [DQL]
SELECT 'false'::jsonb;

-- [DQL]
SELECT '"a"'::json;

-- [DQL]
SELECT '"abc"'::jsonb;

-- [DQL]
SELECT '[1, 2, "foo", null]'::json;

-- [DQL]
SELECT '[]'::json;

-- [DQL]
SELECT '[1, 2, "foo", null, [[]], {}]'::jsonb;

-- [DQL]
SELECT '{}'::json;

-- [DQL]
SELECT '{"a": 1, "b": {"a": 2, "b": null}}'::json;

-- [DQL]
SELECT '{"foo": [true, "bar"], "tags": {"a": 1, "b": null}}'::jsonb;

-- [DQL]
SELECT ' [1, " a ", {"a" :1 }] '::jsonb;

-- [DQL]
SELECT '{"a" : 1, "a" : 2}'::jsonb;

-- [DQL]
SELECT '{"aa" : 1, "b" : 2, "a" : 3}'::jsonb;

-- [DQL]
SELECT '"foo"'::jsonb @> '"foo"'::jsonb;

-- [DQL]
SELECT '[1, "aa", 3]'::jsonb ? 'aa';

-- [DQL]
SELECT '[1, 2, 3]'::jsonb @> '[1, 3, 1]'::jsonb;

-- [DQL]
SELECT '{"product": "PostgreSQL", "version": 9.4, "jsonb":true}'::jsonb @> '{"version":9.4}'::jsonb;

-- [DQL]
SELECT '[1, 2, [1, 3]]'::jsonb @> '[1, 3]'::jsonb;

-- [DQL]
SELECT '{"foo": {"bar": "baz"}}'::jsonb @> '{"bar": "baz"}'::jsonb;


================================================================================
-- 来源: 2755_HLL.txt
================================================================================

-- [DDL]
CREATE TABLE t1 ( id integer , set hll );

-- [OTHER]
\ d t1 Table "public.t1" Column | Type | Modifiers --------+---------+----------- id | integer | set | hll | -- 创建hll类型的表，指定前两个入参，后两个采用默认值。

-- [DDL]
CREATE TABLE t2 ( id integer , set hll ( 12 , 4 ));

-- [OTHER]
\ d t2 Table "public.t2" Column | Type | Modifiers --------+----------------+----------- id | integer | set | hll ( 12 , 4 , 12 , 0 ) | --创建hll类型的表，指定第三个入参，其余采用默认值。

-- [DDL]
CREATE TABLE t3 ( id int , set hll ( - 1 , - 1 , 8 , - 1 ));

-- [OTHER]
\ d t3 Table "public.t3" Column | Type | Modifiers --------+----------------+----------- id | integer | set | hll ( 14 , 10 , 8 , 0 ) | --创建hll类型的表，指定入参不合法报错。

-- [DDL]
CREATE TABLE t4 ( id int , set hll ( 5 , - 1 ));

-- [DDL]
DROP TABLE t1 , t2 , t3 ;

-- [DDL]
CREATE TABLE t1 ( id integer , set hll ( 14 ));

-- [DML_INSERT]
INSERT INTO t1 VALUES ( 1 , hll_empty ( 14 , - 1 ));

-- [DML_INSERT]
INSERT INTO t1 ( id , set ) VALUES ( 1 , hll_empty ( 14 , 5 ));

-- [DDL]
DROP TABLE t1 ;

-- [DDL]
CREATE TABLE helloworld ( id integer , set hll );

-- [DML_INSERT]
INSERT INTO helloworld ( id , set ) VALUES ( 1 , hll_empty ());

-- [DML_UPDATE]
UPDATE helloworld SET set = hll_add ( set , hll_hash_integer ( 12345 )) WHERE id = 1 ;

-- [DML_UPDATE]
UPDATE helloworld SET set = hll_add ( set , hll_hash_text ( 'hello world' )) WHERE id = 1 ;

-- [DQL]
SELECT hll_cardinality ( set ) FROM helloworld WHERE id = 1 ;

-- [DDL]
DROP TABLE helloworld ;

-- [DDL]
CREATE TABLE facts ( date date , user_id integer );

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-20' , generate_series ( 1 , 100 ));

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-21' , generate_series ( 1 , 200 ));

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-22' , generate_series ( 1 , 300 ));

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-23' , generate_series ( 1 , 400 ));

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-24' , generate_series ( 1 , 500 ));

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-25' , generate_series ( 1 , 600 ));

-- [DML_INSERT]
INSERT INTO facts values ( '2019-02-26' , generate_series ( 1 , 700 ));

-- [DML_INSERT]
INSERT INTO facts VALUES ( '2019-02-27' , generate_series ( 1 , 800 ));

-- [DDL]
CREATE TABLE daily_uniques ( date date UNIQUE , users hll );

-- [DML_INSERT]
INSERT INTO daily_uniques ( date , users ) SELECT date , hll_add_agg ( hll_hash_integer ( user_id )) FROM facts GROUP BY 1 ;

-- [DQL]
select date , hll_cardinality ( users ) from daily_uniques order by date ;

-- [DQL]
SELECT hll_cardinality ( hll_union_agg ( users )) FROM daily_uniques WHERE date >= '2019-02-20' :: date AND date <= '2019-02-26' :: date ;

-- [DQL]
SELECT date , ( # hll_union_agg ( users ) OVER two_days ) - # users AS lost_uniques FROM daily_uniques WINDOW two_days AS ( ORDER BY date ASC ROWS 1 PRECEDING );

-- [DDL]
DROP TABLE facts ;

-- [DDL]
DROP TABLE daily_uniques ;

-- [DDL]
CREATE TABLE test ( id integer , set hll );

-- [DML_INSERT]
INSERT INTO test VALUES ( 1 , 'E\\1234' );

-- [DDL]
DROP TABLE test ;


================================================================================
-- 来源: 2756_file_2756.txt
================================================================================

-- [DDL]
CREATE TABLE reservation (room int, during tsrange);

-- [DML_INSERT]
INSERT INTO reservation VALUES (1108, '[2010-01-01 14:30, 2010-01-01 15:30)');

-- 包含 。
-- [DQL]
SELECT int4range(10, 20) @> 3;

-- 重叠 。
-- [DQL]
SELECT numrange(11.1, 22.2) && numrange(20.0, 30.0);

-- 抽取上界 。
-- [DQL]
SELECT upper(int8range(15, 25));

-- 计算交集 。
-- [DQL]
SELECT int4range(10, 20) * int4range(15, 25);

-- 范围 是否为空 。
-- [DQL]
SELECT isempty(numrange(1, 5));

-- [DDL]
DROP TABLE reservation;

-- [DQL]
SELECT '[3,7)'::int4range;

-- 既不包括 3 也不包括 7，但是包括之间的所有点 。
-- [DQL]
SELECT '(3,7)'::int4range;

-- 只包括单独一个点 4 。
-- [DQL]
SELECT '[4,4]'::int4range;

-- 不包括点（并且将被标准化为 '空'） 。
-- [DQL]
SELECT '[4,4)'::int4range;

-- [DQL]
SELECT numrange(1.0, 14.0, '(]');

-- 如果第三个参数被忽略，则假定为 '[)'。
-- [DQL]
SELECT numrange(1.0, 14.0);

-- 尽管这里指定了 '(]'，显示时该值将被转换成标准形式，因为 int8range 是一种离散范围类型（见下文）。
-- [DQL]
SELECT int8range(1, 14, '(]');

-- 为一个界限使用 NULL 导致范围在那一边是无界的。
-- [DQL]
SELECT numrange(NULL, 2.2);

-- [DDL]
CREATE TYPE floatrange AS RANGE ( subtype = float8, subtype_diff = float8mi );

-- [DQL]
SELECT '[1.234, 5.678]'::floatrange;

-- [DDL]
CREATE FUNCTION time_subtype_diff(x time, y time) RETURNS float8 AS 'SELECT EXTRACT(EPOCH FROM (x - y))' LANGUAGE sql STRICT IMMUTABLE;

-- [DDL]
CREATE TYPE timerange AS RANGE ( subtype = time, subtype_diff = time_subtype_diff );

-- [DQL]
SELECT '[11:10, 23:00]'::timerange;


================================================================================
-- 来源: 2757_file_2757.txt
================================================================================

-- [DQL]
SELECT oid FROM pg_class WHERE relname = 'pg_type' ;

-- [DQL]
SELECT attrelid , attname , atttypid , attstattarget FROM pg_attribute WHERE attrelid = 'pg_type' :: REGCLASS ;


================================================================================
-- 来源: 2758_file_2758.txt
================================================================================

-- [DDL]
create table t1 ( a int );

-- [DML_INSERT]
insert into t1 values ( 1 ),( 2 );

-- [DDL]
CREATE OR REPLACE FUNCTION showall () RETURNS SETOF record AS $$ SELECT count ( * ) from t1 ;

-- [DQL]
SELECT showall ();

-- [DDL]
DROP FUNCTION showall ();

-- [DDL]
drop table t1 ;


================================================================================
-- 来源: 2760_XML.txt
================================================================================

-- [DDL]
CREATE TABLE xmltest ( id int, data xml );

-- [DML_INSERT]
INSERT INTO xmltest VALUES (1, 'one');

-- [DML_INSERT]
INSERT INTO xmltest VALUES (2, 'two');

-- [DQL]
SELECT * FROM xmltest ORDER BY 1;

-- [DQL]
SELECT xmlconcat(xmlcomment('hello'), xmlelement(NAME qux, 'xml'), xmlcomment('world'));

-- [DDL]
DROP TABLE xmltest;


================================================================================
-- 来源: 2761_XMLTYPE.txt
================================================================================

-- [DDL]
CREATE TABLE xmltypetest(id int, data xmltype);

-- [DML_INSERT]
INSERT INTO xmltypetest VALUES (1, '<ss/>');

-- [DML_INSERT]
INSERT INTO xmltypetest VALUES (2, '<xx/>');

-- [DQL]
SELECT * FROM xmltypetest ORDER BY 1;


================================================================================
-- 来源: 2763_SET.txt
================================================================================

-- [DDL]
CREATE TABLE employee ( name text, site SET('beijing','shanghai','nanjing','wuhan') );

-- [DML_INSERT]
INSERT INTO employee values('zhangsan', 'nanjing,beijing');

-- [DML_INSERT]
INSERT INTO employee VALUES ('zhangsan', 'hangzhou');

-- [DQL]
SELECT * FROM employee;

-- [DML_INSERT]
INSERT INTO employee values('lisi', 9);

-- [DQL]
SELECT * FROM employee;

-- [DDL]
DROP TABLE employee;


================================================================================
-- 来源: 2764_aclitem.txt
================================================================================

-- [DDL]
CREATE TABLE table_acl (id int,priv aclitem,privs aclitem[]);

-- [DML_INSERT]
INSERT INTO table_acl VALUES (1,'user1=arw/omm','{omm=d/user2,omm=w/omm}');

-- [DML_INSERT]
INSERT INTO table_acl VALUES (2,'user1=aw/omm','{omm=d/user2}');

-- [DQL]
SELECT * FROM table_acl;


================================================================================
-- 来源: 2765_file_2765.txt
================================================================================

-- [DQL]
SELECT CURRENT_ROLE ;

-- [DQL]
SELECT CURRENT_SCHEMA ;

-- [DQL]
SELECT CURRENT_USER ;

-- [DQL]
SELECT LOCALTIMESTAMP ;

-- [DQL]
SELECT SESSION_USER ;

-- [DQL]
SELECT SYSDATE ;

-- [DQL]
SELECT USER ;


================================================================================
-- 来源: 2769_file_2769.txt
================================================================================

-- [DQL]
SELECT bit_length ( 'world' );

-- [DQL]
SELECT btrim ( 'sring' , 'ing' );

-- [DQL]
SELECT char_length ( 'hello' );

-- [DQL]
select dump ( 'abc测试' );

-- [DQL]
SELECT instr ( 'abcdabcdabcd' , 'bcd' , 2 , 2 );

-- [DQL]
SELECT instrb ( 'abcdabcdabcd' , 'bcd' , 2 , 2 );

-- [DQL]
SELECT lengthb ( 'hello' );

-- [DQL]
SELECT left ( 'abcde' , 2 );

-- [DQL]
SELECT length ( 'jose' , 'UTF8' );

-- [DQL]
SELECT lpad ( 'hi' , 5 , 'xyza' );

-- [DQL]
select lpad ( 'expr1' , 7 , '中国' );

-- [DQL]
select lpad ( 'expr1' , 8 , '中国' );

-- [DQL]
SELECT notlike ( 1 , 2 );

-- [DQL]
SELECT notlike ( 1 , 1 );

-- [DQL]
SELECT octet_length ( 'jose' );

-- [DQL]
SELECT overlay ( 'hello' placing 'world' from 2 for 3 );

-- [DQL]
SELECT position ( 'ing' in 'string' );

-- [DQL]
SELECT pg_client_encoding ();

-- [DQL]
SELECT quote_ident ( 'hello world' );

-- [DQL]
SELECT quote_literal ( 'hello' );

-- [DQL]
SELECT quote_literal ( E 'O\' hello ');

-- [DQL]
SELECT quote_literal ( 'O\hello' );

-- [DQL]
SELECT quote_literal ( NULL );

-- [DQL]
SELECT quote_literal ( 42 . 5 );

-- [DQL]
SELECT quote_literal ( E 'O\' 42 . 5 ');

-- [DQL]
SELECT quote_literal ( 'O\42.5' );

-- [DQL]
SELECT quote_nullable ( 'hello' );

-- [DQL]
SELECT quote_nullable ( E 'O\' hello ');

-- [DQL]
SELECT quote_nullable ( 'O\hello' );

-- [DQL]
SELECT quote_nullable ( NULL );

-- [DQL]
SELECT quote_nullable ( 42 . 5 );

-- [DQL]
SELECT quote_nullable ( E 'O\' 42 . 5 ');

-- [DQL]
SELECT quote_nullable ( 'O\42.5' );

-- [DQL]
SELECT quote_nullable ( NULL );

-- [DQL]
select substring_inner ( 'adcde' , 2 , 3 );

-- [DQL]
SELECT substring ( 'Thomas' from 2 for 3 );

-- [DQL]
select substring ( 'substrteststring' , - 5 , 5 );

-- [DQL]
SELECT substring_index ( 'Test1splitTest2splitTest3splitTest4' , 'split' , 2 );

-- [DQL]
SELECT substring_index ( 'Test1splitTest2splitTest3splitTest4' , 'split' , - 2 );

-- [DQL]
SELECT substring ( 'Thomas' from '...$' );

-- [DQL]
SELECT substring ( 'foobar' from 'o(.)b' );

-- [DQL]
SELECT substring ( 'foobar' from '(o(.)b)' );

-- [DQL]
SELECT substring ( 'Thomas' from '%#"o_a#"_' for '#' );

-- [DQL]
SELECT rawcat ( 'ab' , 'cd' );

-- [DQL]
SELECT regexp_like ( 'str' , '[ac]' );

-- [DQL]
SELECT regexp_substr ( 'str' , '[ac]' );

-- [DQL]
SELECT regexp_substr ( 'foobarbaz' , 'b(..)' , 3 , 2 ) AS RESULT ;

-- [DQL]
SELECT regexp_count('foobarbaz','b(..)', 5) AS RESULT;

-- [DQL]
SELECT regexp_instr('foobarbaz','b(..)', 1, 1, 0) AS RESULT;

-- [DQL]
SELECT regexp_instr('foobarbaz','b(..)', 1, 2, 0) AS RESULT;

-- [DQL]
SELECT regexp_matches ( 'foobarbequebaz' , '(bar)(beque)' );

-- [DQL]
SELECT regexp_matches ( 'foobarbequebaz' , 'barbeque' );

-- [DQL]
SELECT regexp_matches ( 'foobarbequebazilbarfbonk' , '(b[^b]+)(b[^b]+)' , 'g' );

-- [DQL]
SELECT regexp_match('foobarbequebaz', '(bar)(beque)');

-- [DQL]
SELECT (regexp_match('foobarbequebaz', 'bar.*que'))[1];

-- [DQL]
SELECT regexp_match('Learning #PostgreSQL', 'R', 'c');

-- [DQL]
SELECT regexp_match('hello world', 'h e l l o', 'x');

-- [DQL]
SELECT regexp_split_to_array ( 'hello world' , E '\\s+' );

-- [DQL]
SELECT regexp_split_to_table ( 'hello world' , E '\\s+' );

-- [DQL]
SELECT repeat ( 'Pg' , 4 );

-- [DQL]
SELECT replace ( 'abcdefabcdef' , 'cd' , 'XXX' );

-- [DQL]
SELECT replace ( 'abcdefabcdef' , 'cd' );

-- [DQL]
SELECT reverse ( 'abcde' );

-- [DQL]
SELECT right ( 'abcde' , 2 );

-- [DQL]
SELECT right ( 'abcde' , - 2 );

-- [DQL]
SELECT rpad ( 'hi' , 5 , 'xy' );

-- [DQL]
select rpad ( 'expr1' , 7 , '中国' ) || '*' ;

-- [DQL]
select rpad ( 'expr1' , 8 , '中国' ) || '*' ;

-- [DQL]
SELECT substr ( 'stringtest' FROM 4 );

-- [DQL]
SELECT substr ( 'stringtest' , 4 );

-- [DQL]
SELECT substr ( 'stringtest' , - 4 );

-- [DQL]
SELECT substr ( 'stringtest' , 11 );

-- [DQL]
SELECT substr ( 'teststring' FROM 5 FOR 2 );

-- [DQL]
SELECT substr ( 'teststring' , 5 , 2 );

-- [DQL]
SELECT substr ( 'teststring' , 5 , 10 );

-- [DQL]
SELECT substrb ( 'string' , 2 );

-- [DQL]
SELECT substrb ( 'string' , - 2 );

-- [DQL]
SELECT substrb ( 'string' , 10 );

-- [DQL]
SELECT substrb ( '数据库' , 1 );

-- [DQL]
SELECT substrb ( '数据库' , 2 );

-- [DQL]
SELECT substrb ( 'string' , 2 , 3 );

-- [DQL]
SELECT substrb ( 'string' , 2 , 10 );

-- [DQL]
SELECT substrb ( '数据库' , 4 , 3 );

-- [DQL]
SELECT substrb ( '数据库' , 2 , 6 ) = ' 据' as result ;

-- [DQL]
SELECT substrb ( '数据库' , 2 , 6 ) = ' 据 ' as result ;

-- [DQL]
SELECT 'MPP' || 'DB' AS RESULT ;

-- [DQL]
SELECT 'Value: ' || 42 AS RESULT ;

-- [DQL]
SELECT split_part ( 'abc~@~def~@~ghi' , '~@~' , 2 );

-- [DQL]
SELECT strpos ( 'source' , 'rc' );

-- [DQL]
SELECT to_hex ( 2147483647 );

-- [DQL]
SELECT translate ( '12345' , '143' , 'ax' );

-- [DQL]
SELECT length ( 'abcd' );

-- [DQL]
SELECT length ( '汉字abc' );

-- [DDL]
CREATE DATABASE gaussdb_m WITH dbcompatibility 'B' ;

-- [OTHER]
\ c gaussdb_m -- 设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

-- [DQL]
SELECT lengthb ( 'Chinese' );

-- [DQL]
SELECT trim ( BOTH 'x' FROM 'xTomxx' );

-- [DQL]
SELECT trim ( LEADING 'x' FROM 'xTomxx' );

-- [DQL]
SELECT trim ( TRAILING 'x' FROM 'xTomxx' );

-- [DQL]
select to_single_byte ( 'ＡＢ１２３' );

-- [DQL]
select to_multi_byte ( 'ABC123' );

-- [DQL]
SELECT rtrim ( 'TRIMxxxx' , 'x' );

-- [DQL]
SELECT ltrim ( 'xxxxTRIM' , 'x' );

-- [DQL]
SELECT upper ( 'tom' );

-- [DQL]
SELECT lower ( 'TOM' );

-- [DQL]
SELECT nls_upper ( 'gro?e' );

-- [DQL]
SELECT nls_upper ( 'gro?e' , 'nls_sort = XGerman' );

-- [DQL]
SELECT nls_lower ( 'INDIVISIBILITY' );

-- [DQL]
SELECT nls_lower ( 'INDIVISIBILITY' , 'nls_sort = XTurkish' );

-- [DQL]
SELECT instr ( 'corporate floor' , 'or' , 3 );

-- [DQL]
SELECT instr ( 'corporate floor' , 'or' , - 3 , 2 );

-- [DQL]
SELECT initcap ( 'hi THOMAS' );

-- [DQL]
SELECT ascii ( 'xyz' );

-- [DQL]
SELECT ascii2 ( 'xyz' );

-- [DQL]
select ascii2 ( '中xyz' );

-- [DQL]
SELECT asciistr ( 'xyz中' );

-- [DQL]
select unistr ( 'abc\0041\4E2D' );

-- [DQL]
select vsize ( 'abc测试' );

-- [DQL]
SELECT replace ( 'jack and jue' , 'j' , 'bl' );

-- [DQL]
SELECT concat ( 'Hello' , ' World!' );

-- [DQL]
SELECT concat ( 'Hello' , NULL );

-- [DDL]
CREATE TABLE test_space ( c char ( 10 ));

-- [DML_INSERT]
INSERT INTO test_space values ( 'a' );

-- [DQL]
SELECT * frOm test_space WHERE c = 'a ' ;

-- [DQL]
SELECT * FROM test_space WHERE c = 'a' || ' ' ;

-- [DQL]
SELECT chr ( 65 );

-- [DQL]
select chr ( 19968 );

-- [DQL]
SELECT chr ( 65 );

-- [DQL]
select chr ( 16705 );

-- [DQL]
select chr ( 4259905 );

-- [DQL]
SELECT nchr ( 65 );

-- [DQL]
SELECT nchr ( 14989440 );

-- [DQL]
SELECT nchr ( 14989440 );

-- [DQL]
SELECT nchr ( 4321090 );

-- [DQL]
SELECT nchr ( 14989440 );

-- [DQL]
SELECT nchr ( 14989440 );

-- [DQL]
SELECT regexp_substr ( '500 Hello World, Redwood Shores, CA' , ',[^,]+,' ) "REGEXPR_SUBSTR" ;

-- [DQL]
SELECT regexp_replace ( 'Thomas' , '.[mN]a.' , 'M' );

-- [DQL]
SELECT regexp_replace ( 'foobarbaz' , 'b(..)' , E 'X\\1Y' , 'g' ) AS RESULT ;

-- [DQL]
SELECT regexp_replace('foobarbaz','b(..)', E'X\\1Y', 2, 2, 'n') AS RESULT;

-- [DQL]
SELECT concat_ws ( ',' , 'ABCDE' , 2 , NULL , 22 );

-- [DDL]
create table test ( a text );

-- [DML_INSERT]
insert into test ( a ) values ( 'abC 不' );

-- [DML_INSERT]
insert into test ( a ) values ( 'abC 啊' );

-- [DML_INSERT]
insert into test ( a ) values ( 'abc 啊' );

-- [DQL]
select * from test order by nlssort ( a , 'nls_sort=schinese_pinyin_m' );

-- [DQL]
select * from test order by nlssort ( a , 'nls_sort=generic_m_ci' );

-- [DQL]
SELECT convert ( 'text_in_utf8' , 'UTF8' , 'GBK' );

-- [SESSION]
show server_encoding ;

-- [DQL]
SELECT convert_from ( 'some text' , 'GBK' );

-- [DQL]
SELECT convert ( 'asdas' using 'gbk' );

-- [DQL]
SELECT convert_from ( 'text_in_utf8' , 'UTF8' );

-- [DQL]
SELECT convert_to ( 'some text' , 'UTF8' );

-- [DQL]
SELECT 'AA_BBCC' LIKE '%A@_B%' ESCAPE '@' AS RESULT ;

-- [DQL]
SELECT 'AA_BBCC' LIKE '%A@_B%' AS RESULT ;

-- [DQL]
SELECT 'AA@_BBCC' LIKE '%A@_B%' AS RESULT ;

-- [DQL]
SELECT regexp_like ( 'ABC' , '[A-Z]' );

-- [DQL]
SELECT regexp_like ( 'ABC' , '[D-Z]' );

-- [DQL]
SELECT regexp_like ( 'ABC' , '[a-z]' , 'i' );

-- [DQL]
SELECT format ( 'Hello %s, %1$s' , 'World' );

-- [DQL]
SELECT md5 ( 'ABC' );

-- [DQL]
select sha ( 'ABC' );

-- [DQL]
select sha1 ( 'ABC' );

-- [DQL]
select sha2 ( 'ABC' , 224 );

-- [DQL]
select sha2 ( 'ABC' , 256 );

-- [DQL]
select sha2 ( 'ABC' , 0 );

-- [DQL]
SELECT decode ( 'MTIzAAE=' , 'base64' );

-- [DQL]
select similar_escape('\s+ab','2');

-- [DQL]
select site , find_in_set ( 'wuhan' , site ) from employee ;

-- [DQL]
select find_in_set ( 'ee' , 'a,ee,c' );

-- [DQL]
SELECT encode ( E '123\\000\\001' , 'base64' );

-- [DQL]
SELECT translate('12345','123','');

-- [EXPLAIN]
EXPLAIN SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- [DQL]
SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- [DDL]
CREATE EXTENSION pkg_bpchar_opc ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- [DQL]
SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- [DDL]
DROP EXTENSION pkg_bpchar_opc ;

-- [DQL]
SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- [DDL]
CREATE EXTENSION pkg_bpchar_opc ;

-- [DQL]
SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- [DDL]
DROP EXTENSION pkg_bpchar_opc ;

-- [DQL]
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- [DQL]
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = 'FE306991300002 ' ;

-- [DDL]
CREATE EXTENSION pkg_bpchar_opc ;

-- [DQL]
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- [EXPLAIN]
explain SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- [DQL]
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = 'FE306991300002 ' ;

-- [DQL]
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id :: bpchar = t2 . log_id ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id :: bpchar = t2 . log_id ;

-- [DQL]
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = 'FE306991300002 ' ;

-- [DDL]
DROP EXTENSION pkg_bpchar_opc ;


================================================================================
-- 来源: 2770_file_2770.txt
================================================================================

-- [DQL]
SELECT octet_length ( E 'jo\\000se' :: bytea ) AS RESULT ;

-- [DQL]
SELECT overlay ( E 'Th\\000omas' :: bytea placing E '\\002\\003' :: bytea from 2 for 3 ) AS RESULT ;

-- [DQL]
SELECT position ( E '\\000om' :: bytea in E 'Th\\000omas' :: bytea ) AS RESULT ;

-- [DQL]
SELECT substring ( E 'Th\\000omas' :: bytea from 2 for 3 ) AS RESULT ;

-- [DQL]
select substr ( E 'Th\\000omas' :: bytea , 2 , 3 ) as result ;

-- [DQL]
SELECT trim ( E '\\000' :: bytea from E '\\000Tom\\000' :: bytea ) AS RESULT ;

-- [DQL]
SELECT btrim ( E '\\000trim\\000' :: bytea , E '\\000' :: bytea ) AS RESULT ;

-- [DQL]
SELECT get_bit ( E 'Th\\000omas' :: bytea , 45 ) AS RESULT ;

-- [DQL]
SELECT get_byte ( E 'Th\\000omas' :: bytea , 4 ) AS RESULT ;

-- [DQL]
SELECT set_bit ( E 'Th\\000omas' :: bytea , 45 , 0 ) AS RESULT ;

-- [DQL]
SELECT set_byte ( E 'Th\\000omas' :: bytea , 4 , 64 ) AS RESULT ;


================================================================================
-- 来源: 2771_file_2771.txt
================================================================================

-- [DQL]
SELECT B '10001' || B '011' AS RESULT ;

-- [DQL]
SELECT B '10001' & B '01101' AS RESULT ;

-- [DQL]
SELECT B '10001' | B '01101' AS RESULT ;

-- [DQL]
SELECT B '10001' # B '01101' AS RESULT ;

-- [DQL]
SELECT ~ B '10001' AS RESULT ;

-- [DQL]
SELECT B '10001' << 3 AS RESULT ;

-- [DQL]
SELECT B '10001' >> 2 AS RESULT ;

-- [DQL]
SELECT 44 :: bit ( 10 ) AS RESULT ;

-- [DQL]
SELECT 44 :: bit ( 3 ) AS RESULT ;

-- [DQL]
SELECT cast ( - 44 as bit ( 12 )) AS RESULT ;

-- [DQL]
SELECT '1110' :: bit ( 4 ):: integer AS RESULT ;

-- [DQL]
select substring ( '10101111' :: bit ( 8 ), 2 );


================================================================================
-- 来源: 2772_file_2772.txt
================================================================================

-- [DQL]
SELECT 'abc' LIKE 'abc' AS RESULT ;

-- [DQL]
SELECT 'abc' LIKE 'a%' AS RESULT ;

-- [DQL]
SELECT 'abc' LIKE '_b_' AS RESULT ;

-- [DQL]
SELECT 'abc' LIKE 'c' AS RESULT ;

-- [DQL]
SELECT 'abc' SIMILAR TO 'abc' AS RESULT ;

-- [DQL]
SELECT 'abc' SIMILAR TO 'a' AS RESULT ;

-- [DQL]
SELECT 'abc' SIMILAR TO '%(b|d)%' AS RESULT ;

-- [DQL]
SELECT 'abc' SIMILAR TO '(b|c)%' AS RESULT ;

-- [DQL]
SELECT 'abc' ~ 'Abc' AS RESULT ;

-- [DQL]
SELECT 'abc' ~* 'Abc' AS RESULT ;

-- [DQL]
SELECT 'abc' !~ 'Abc' AS RESULT ;

-- [DQL]
SELECT 'abc' !~* 'Abc' AS RESULT ;

-- [DQL]
SELECT 'abc' ~ '^a' AS RESULT ;

-- [DQL]
SELECT 'abc' ~ '(b|d)' AS RESULT ;

-- [DQL]
SELECT 'abc' ~ '^(b|c)' AS RESULT ;


================================================================================
-- 来源: 2773_file_2773.txt
================================================================================

-- [DQL]
SELECT 2 + 3 AS RESULT ;

-- [DQL]
SELECT 2 - 3 AS RESULT ;

-- [DQL]
SELECT 2 * 3 AS RESULT ;

-- [DQL]
SELECT 4 / 2 AS RESULT ;

-- [DQL]
SELECT 4 / 3 AS RESULT ;

-- [DQL]
SELECT - 2 AS RESULT ;

-- [DQL]
SELECT 5 % 4 AS RESULT ;

-- [DQL]
SELECT @ - 5 . 0 AS RESULT ;

-- [DQL]
SELECT 2 . 0 ^ 3 . 0 AS RESULT ;

-- [DQL]
SELECT |/ 25 . 0 AS RESULT ;

-- [DQL]
SELECT ||/ 27 . 0 AS RESULT ;

-- [DQL]
SELECT 5 ! AS RESULT ;

-- [DQL]
SELECT !! 5 AS RESULT ;

-- [DQL]
SELECT 91 & 15 AS RESULT ;

-- [DQL]
SELECT 32 | 3 AS RESULT ;

-- [DQL]
SELECT 17 # 5 AS RESULT ;

-- [DQL]
SELECT ~ 1 AS RESULT ;

-- [DQL]
SELECT 1 << 4 AS RESULT ;

-- [DQL]
SELECT 8 >> 2 AS RESULT ;

-- [DQL]
SELECT abs ( - 17 . 4 );

-- [DQL]
SELECT acos ( - 1 );

-- [DQL]
SELECT asin ( 0 . 5 );

-- [DQL]
SELECT atan ( 1 );

-- [DQL]
SELECT atan2 ( 2 , 1 );

-- [DQL]
SELECT bitand ( 127 , 63 );

-- [DQL]
SELECT cbrt ( 27 . 0 );

-- [DQL]
SELECT ceil ( - 42 . 8 );

-- [DQL]
SELECT ceiling ( - 95 . 3 );

-- [DQL]
SELECT cos ( - 3 . 1415927 );

-- [DQL]
SELECT cosh ( 4 );

-- [DQL]
SELECT cot ( 1 );

-- [DQL]
SELECT degrees ( 0 . 5 );

-- [DQL]
SELECT div ( 9 , 4 );

-- [DQL]
SELECT exp ( 1 . 0 );

-- [DQL]
SELECT floor ( - 42 . 8 );

-- [DQL]
select int1 ( '123' );

-- [DQL]
select int1 ( '1.1' );

-- [DQL]
select int2 ( '1234' );

-- [DQL]
select int2 ( 25 . 3 );

-- [DQL]
select int4 ( '789' );

-- [DQL]
select int4 ( 99 . 9 );

-- [DQL]
select float4 ( '789' );

-- [DQL]
select float4 ( 99 . 9 );

-- [DQL]
select float8 ( '789' );

-- [DQL]
select float8 ( 99 . 9 );

-- [DQL]
select int16 ( '789' );

-- [DQL]
select int16 ( 99 . 9 );

-- [DQL]
select "numeric" ( '789' );

-- [DQL]
select "numeric" ( 99 . 9 );

-- [DQL]
SELECT radians ( 45 . 0 );

-- [DQL]
SELECT random ();

-- [DQL]
SELECT multiply ( 9 . 0 , '3.0' );

-- [DQL]
SELECT multiply ( '9.0' , 3 . 0 );

-- [DQL]
SELECT ln ( 2 . 0 );

-- [DQL]
SELECT log ( 100 . 0 );

-- [DQL]
SELECT log ( 2 . 0 , 64 . 0 );

-- [DQL]
SELECT mod ( 9 , 4 );

-- [DQL]
SELECT mod ( 9 , 0 );

-- [DQL]
SELECT pi ();

-- [DQL]
SELECT power ( 9 . 0 , 3 . 0 );

-- [DQL]
SELECT remainder ( 11 , 4 );

-- [DQL]
SELECT remainder ( 9 , 0 );

-- [DQL]
SELECT round ( 42 . 4 );

-- [DQL]
SELECT round ( 42 . 6 );

-- [DQL]
SELECT round ( - 0 . 2 :: float8 );

-- [DQL]
SELECT round ( 42 . 4382 , 2 );

-- [DQL]
SELECT setseed ( 0 . 54823 );

-- [DQL]
SELECT sign ( - 8 . 4 );

-- [DQL]
SELECT sin ( 1 . 57079 );

-- [DQL]
SELECT sinh ( 4 );

-- [DQL]
SELECT sqrt ( 2 . 0 );

-- [DQL]
SELECT tan ( 20 );

-- [DQL]
SELECT tanh ( 0 . 1 );

-- [DQL]
SELECT trunc ( 42 . 8 );

-- [DQL]
SELECT trunc ( 42 . 4382 , 2 );

-- [DQL]
SELECT width_bucket ( 5 . 35 , 0 . 024 , 10 . 06 , 5 );

-- [DQL]
SELECT width_bucket ( 5 . 35 , 0 . 024 , 10 . 06 , 5 );

-- [DQL]
SELECT nanvl('NaN', 1.1);

-- [DQL]
SELECT numeric_eq_text(1, '1');

-- [DQL]
SELECT text_eq_numeric('1', 1);

-- [DQL]
SELECT bigint_eq_text(1, '1');

-- [DQL]
SELECT text_eq_bigint('1', 1);


================================================================================
-- 来源: 2774_file_2774.txt
================================================================================

-- [DQL]
SELECT date '2001-10-01' - '7' AS RESULT ;

-- [DQL]
SELECT date '2001-9-28' + integer '7' AS RESULT ;

-- [DQL]
SELECT date '2001-09-28' + interval '1 hour' AS RESULT ;

-- [DQL]
SELECT date '2001-09-28' + time '03:00' AS RESULT ;

-- [DQL]
SELECT interval '1 day' + interval '1 hour' AS RESULT ;

-- [DQL]
SELECT timestamp '2001-09-28 01:00' + interval '23 hours' AS RESULT ;

-- [DQL]
SELECT time '01:00' + interval '3 hours' AS RESULT ;

-- [DQL]
SELECT date '2001-10-01' - date '2001-09-28' AS RESULT ;

-- [DQL]
SELECT date '2001-10-01' - integer '7' AS RESULT ;

-- [DQL]
SELECT date '2001-09-28' - interval '1 hour' AS RESULT ;

-- [DQL]
SELECT time '05:00' - time '03:00' AS RESULT ;

-- [DQL]
SELECT time '05:00' - interval '2 hours' AS RESULT ;

-- [DQL]
SELECT timestamp '2001-09-28 23:00' - interval '23 hours' AS RESULT ;

-- [DQL]
SELECT interval '1 day' - interval '1 hour' AS RESULT ;

-- [DQL]
SELECT timestamp '2001-09-29 03:00' - timestamp '2001-09-27 12:00' AS RESULT ;

-- [DQL]
SELECT 900 * interval '1 second' AS RESULT ;

-- [DQL]
SELECT 21 * interval '1 day' AS RESULT ;

-- [DQL]
SELECT double precision '3.5' * interval '1 hour' AS RESULT ;

-- [DQL]
SELECT interval '1 hour' / double precision '1.5' AS RESULT ;

-- [DQL]
SELECT age ( timestamp '2001-04-10' , timestamp '1957-06-13' );

-- [DQL]
SELECT age ( timestamp '1957-06-13' );

-- [DQL]
SELECT clock_timestamp ();

-- [DQL]
SELECT current_date ;

-- [DQL]
SELECT current_time ;

-- [DQL]
SELECT current_timestamp ;

-- [DQL]
SELECT current_timestamp ;

-- [DQL]
SELECT current_timestamp ();

-- [DQL]
SELECT current_timestamp ( 1 );

-- [DQL]
SELECT current_timestamp ( 1 );

-- [DQL]
SELECT pg_systimestamp ();

-- [DQL]
SELECT date_part ( 'hour' , timestamp '2001-02-16 20:38:40' );

-- [DQL]
SELECT date_part ( 'month' , interval '2 years 3 months' );

-- [DQL]
SELECT date_trunc ( 'hour' , timestamp '2001-02-16 20:38:40' );

-- [DQL]
SELECT trunc ( timestamp '2001-02-16 20:38:40' );

-- [DQL]
SELECT trunc ( timestamp '2001-02-16 20:38:40' , 'hour' );

-- [DQL]
SELECT round ( timestamp '2001-02-16 20:38:40' , 'hour' );

-- [DQL]
SELECT daterange ( '2000-05-06' , '2000-08-08' );

-- [DQL]
SELECT daterange ( '2000-05-06' , '2000-08-08' , '[]' );

-- [DQL]
SELECT isfinite ( date '2001-02-16' );

-- [DQL]
SELECT isfinite ( date 'infinity' );

-- [DQL]
SELECT isfinite ( timestamp '2001-02-16 21:28:30' );

-- [DQL]
SELECT isfinite ( timestamp 'infinity' );

-- [DQL]
SELECT isfinite ( interval '4 hours' );

-- [DQL]
SELECT justify_days ( interval '35 days' );

-- [DQL]
SELECT JUSTIFY_HOURS ( INTERVAL '27 HOURS' );

-- [DQL]
SELECT JUSTIFY_INTERVAL ( INTERVAL '1 MON -1 HOUR' );

-- [DQL]
SELECT localtime AS RESULT ;

-- [DQL]
SELECT localtimestamp ;

-- [DQL]
SELECT maketime ( 8 , 15 , 26 . 53 );

-- [DQL]
SELECT maketime ( - 888 , 15 , 26 . 53 );

-- [DQL]
SELECT now ();

-- [DQL]
SELECT timenow ();

-- [DQL]
SELECT dbtimezone ;

-- [DQL]
SELECT numtodsinterval ( 100 , 'HOUR' );

-- [SESSION]
SET intervalstyle = a ;

-- [DQL]
SELECT numtodsinterval ( 100 , 'HOUR' );

-- [DQL]
SELECT numtoyminterval ( 100 , 'MONTH' );

-- [SESSION]
SET intervalstyle = oracle ;

-- [DQL]
SELECT numtodsinterval ( 100 , 'MONTH' );

-- [DQL]
SELECT new_time ( '1997-10-10' , 'AST' , 'EST' );

-- [DQL]
SELECT NEW_TIME ( TO_TIMESTAMP ( '10-Sep-02 14:10:10.123000' , 'DD-Mon-RR HH24:MI:SS.FF' ), 'AST' , 'PST' );

-- [DQL]
SELECT SESSIONTIMEZONE ;

-- [DQL]
SELECT LOWER ( SESSIONTIMEZONE );

-- [DQL]
SELECT SYS_EXTRACT_UTC ( TIMESTAMP '2000-03-28 11:30:00.00' );

-- [DQL]
SELECT SYS_EXTRACT_UTC ( TIMESTAMPTZ '2000-03-28 11:30:00.00 -08:00' );

-- [DQL]
SELECT TZ_OFFSET ( 'US/Pacific' );

-- [DQL]
SELECT TZ_OFFSET ( sessiontimezone );

-- [DQL]
SELECT pg_sleep ( 10 );

-- [DQL]
SELECT statement_timestamp ();

-- [DQL]
SELECT sysdate ;

-- [DQL]
SELECT current_sysdate ();

-- [DQL]
SELECT timeofday ();

-- [DQL]
SELECT transaction_timestamp ();

-- [DQL]
SELECT transaction_timestamp ();

-- [DQL]
SELECT add_months ( to_date ( '2017-5-29' , 'yyyy-mm-dd' ), 11 ) FROM sys_dummy ;

-- [DQL]
SELECT last_day ( to_date ( '2017-01-01' , 'YYYY-MM-DD' )) AS cal_result ;

-- [DQL]
SELECT months_between(to_date('2022-10-31', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

-- [DQL]
SELECT months_between(to_date('2022-10-30', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

-- [DQL]
SELECT months_between(to_date('2022-10-29', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

-- [DQL]
SELECT next_day ( timestamp '2017-05-25 00:00:00' , 'Sunday' ) AS cal_result ;

-- [DQL]
SELECT next_day ( timestamp '2024-01-17 00:00:00' , 7 . 9999999 );

-- [SESSION]
SET a_format_version = '10c' ;

-- [SESSION]
SET a_format_dev_version = 's1' ;

-- [DQL]
SELECT next_day ( timestamp '2024-01-17 00:00:00' , 7 . 9999999 );

-- [PLSQL]
CALL tinterval ( abstime 'May 10, 1947 23:59:12' , abstime 'Mon May 1 00:30:30 1995' );

-- [DQL]
SELECT tintervalend ( '["Sep 4, 1983 23:59:12" "Oct4, 1983 23:59:12"]' );

-- [DQL]
SELECT tintervalrel ( '["Sep 4, 1983 23:59:12" "Oct4, 1983 23:59:12"]' );

-- [DDL]
CREATE USER JIM PASSWORD '*********' ;

-- [DDL]
CREATE DATABASE testdb3 OWNER JIM DBCOMPATIBILITY = 'B' ;

-- [OTHER]
\ c testdb3 testdb3 =# SET b_format_dev_version = 's1' ;

-- [DDL]
CREATE USER JIM PASSWORD '*********' ;

-- [DDL]
CREATE DATABASE testdb3 OWNER JIM DBCOMPATIBILITY = 'B' ;

-- [OTHER]
\ c testdb3 testdb3 =# SET b_format_dev_version = 's1' ;

-- [DQL]
SELECT ADDDATE ( '2018-05-01' , INTERVAL 1 DAY );

-- [DQL]
SELECT ADDDATE('2018-05-01', 1);

-- [DQL]
SELECT curdate ();

-- [DQL]
SELECT curtime ( 3 );

-- [DQL]
SELECT DATE_ADD('2018-05-01', INTERVAL 1 DAY);

-- [DQL]
SELECT DATE_ADD('2018-05-01', 1);

-- [DQL]
SELECT date_format('2023-10-11 12:13:14.151617','%b %c %M %m');

-- [DQL]
SELECT DATE_SUB('2018-05-01', INTERVAL 1 YEAR);

-- [DQL]
SELECT DATE_SUB('2023-1-1', 20);

-- [DQL]
SELECT datediff('2021-11-12','2021-11-13');

-- [DQL]
SELECT day('2023-01-02');

-- [DQL]
SELECT dayofmonth('23-05-22');

-- [DQL]
SELECT dayname('2023-10-11');

-- [DQL]
SELECT dayofweek('2023-04-16');

-- [DQL]
SELECT dayofyear('2000-12-31');

-- [DQL]
SELECT extract(YEAR FROM '2023-10-11');

-- [DQL]
SELECT extract(QUARTER FROM '2023-10-11');

-- [DQL]
SELECT extract(MONTH FROM '2023-10-11');

-- [DQL]
SELECT extract(WEEK FROM '2023-10-11');

-- [DQL]
SELECT extract(DAY FROM '2023-10-11');

-- [DQL]
SELECT extract(HOUR FROM '2023-10-11 12:13:14');

-- [DQL]
SELECT from_days(36524);

-- [DQL]
SELECT from_unixtime(1111885200);

-- [DQL]
SELECT get_format(date, 'eur');

-- [DQL]
SELECT get_format(date, 'usa');

-- [DQL]
SELECT HOUR('10:10:10.1');

-- [DQL]
SELECT makedate(2000, 60);

-- [DQL]
SELECT MICROSECOND('2023-5-5 10:10:10.24485');

-- [DQL]
SELECT MINUTE(time'10:10:10');

-- [DQL]
SELECT month('2021-11-30');

-- [DQL]
SELECT monthname('2023-02-28');

-- [DQL]
SELECT period_add(202205, -12);

-- [DQL]
SELECT period_diff('202101', '202102');

-- [DQL]
SELECT SECOND('2023-5-5 10:10:10');

-- [DQL]
SELECT QUARTER('2012-1-1');

-- [DQL]
SELECT str_to_date('May 1, 2013','%M %d,%Y');

-- [DQL]
SELECT SUBDATE('2023-1-1', 20);

-- [DQL]
SELECT SUBDATE('2018-05-01', INTERVAL 1 YEAR);

-- [DQL]
SELECT subtime('2000-03-01 20:59:59', '22:58');

-- [DQL]
SELECT addtime('2000-03-01 20:59:59', '00:00:01');

-- [DQL]
SELECT TIME_FORMAT('25:30:30', '%T|%r|%H|%h|%I|%i|%S|%f|%p|%k');

-- [DQL]
SELECT time_to_sec('00:00:01');

-- [DQL]
SELECT timediff(date'2022-12-30',20221229);

-- [DQL]
SELECT TIMESTAMPADD(DAY,-2,'2022-07-27');

-- [DQL]
SELECT to_days('2000-1-1');

-- [DQL]
SELECT TO_SECONDS('2009-11-29 13:43:32');

-- [DQL]
SELECT UNIX_TIMESTAMP('2022-12-22');

-- [DQL]
SELECT utc_date();

-- [DQL]
SELECT utc_time();

-- [DQL]
SELECT utc_timestamp();

-- [DQL]
SELECT week(date'2000-01-01', 1);

-- [DQL]
SELECT week('2000-01-01', 2);

-- [DQL]
SELECT weekday('1970-01-01 12:00:00');

-- [DQL]
SELECT weekofyear('1970-05-22');

-- [DQL]
SELECT year('23-05-22');

-- [DQL]
SELECT yearweek(datetime'2000-01-01', 3);

-- [DQL]
SELECT timestamp_diff ( 'year' , '2018-01-01' , '2020-04-01' );

-- [DQL]
SELECT timestamp_diff ( 'month' , '2018-01-01' , '2020-04-01' );

-- [DQL]
SELECT timestamp_diff ( 'quarter' , '2018-01-01' , '2020-04-01' );

-- [DQL]
SELECT timestamp_diff ( 'week' , '2018-01-01' , '2020-04-01' );

-- [DQL]
SELECT timestamp_diff ( 'day' , '2018-01-01' , '2020-04-01' );

-- [DQL]
SELECT timestamp_diff ( 'hour' , '2018-01-01 10:10:10' , '2018-01-01 12:12:12' );

-- [DQL]
SELECT timestamp_diff ( 'minute' , '2018-01-01 10:10:10' , '2018-01-01 12:12:12' );

-- [DQL]
SELECT timestamp_diff ( 'second' , '2018-01-01 10:10:10' , '2018-01-01 10:12:12' );

-- [DQL]
SELECT timestamp_diff ( 'microsecond' , '2018-01-01 10:10:10' , '2018-01-01 10:12:12' );

-- [DQL]
SELECT TIMESTAMPDIFF ( YEAR , '2018-01-01' , '2020-01-01' );

-- [DQL]
SELECT TIMESTAMPDIFF ( QUARTER , '2018-01-01' , '2020-01-01' );

-- [DQL]
SELECT TIMESTAMPDIFF ( MONTH , '2018-01-01' , '2020-01-01' );

-- [DQL]
SELECT TIMESTAMPDIFF ( WEEK , '2018-01-01' , '2020-01-01' );

-- [DQL]
SELECT TIMESTAMPDIFF ( DAY , '2018-01-01' , '2020-01-01' );

-- [DQL]
SELECT TIMESTAMPDIFF ( HOUR , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

-- [DQL]
SELECT TIMESTAMPDIFF ( MINUTE , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

-- [DQL]
SELECT TIMESTAMPDIFF ( SECOND , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

-- [DQL]
SELECT TIMESTAMPDIFF ( MICROSECOND , '2020-01-01 10:10:10.000000' , '2020-01-01 10:10:10.111111' );

-- [DQL]
SELECT EXTRACT ( CENTURY FROM TIMESTAMP '2000-12-16 12:21:13' );

-- [DQL]
SELECT EXTRACT ( DAY FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( DAY FROM INTERVAL '40 days 1 minute' );

-- [DQL]
SELECT EXTRACT ( DECADE FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( DOW FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( DOY FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( EPOCH FROM TIMESTAMP WITH TIME ZONE '2001-02-16 20:38:40.12-08' );

-- [DQL]
SELECT EXTRACT ( EPOCH FROM INTERVAL '5 days 3 hours' );

-- [DQL]
SELECT TIMESTAMP WITH TIME ZONE 'epoch' + 982384720 . 12 * INTERVAL '1 second' AS RESULT ;

-- [DQL]
SELECT EXTRACT ( HOUR FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( ISODOW FROM TIMESTAMP '2001-02-18 20:38:40' );

-- [DQL]
SELECT EXTRACT ( ISOYEAR FROM DATE '2006-01-01' );

-- [DQL]
SELECT EXTRACT ( WEEK FROM TIMESTAMP '2006-01-01 00:00:40' );

-- [DQL]
SELECT EXTRACT(ISOYEAR FROM DATE '2006-01-02');

-- [DQL]
SELECT EXTRACT(WEEK FROM TIMESTAMP '2006-01-02 00:00:40');

-- [DQL]
SELECT EXTRACT ( MICROSECONDS FROM TIME '17:12:28.5' );

-- [DQL]
SELECT EXTRACT ( MILLENNIUM FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( MILLISECONDS FROM TIME '17:12:28.5' );

-- [DQL]
SELECT EXTRACT ( MINUTE FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( MONTH FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( MONTH FROM INTERVAL '2 years 13 months' );

-- [DQL]
SELECT EXTRACT ( QUARTER FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT EXTRACT ( SECOND FROM TIME '17:12:28.5' );

-- [DQL]
SELECT EXTRACT ( ISOYEAR FROM DATE '2006-01-01' );

-- [DQL]
SELECT EXTRACT ( WEEK FROM TIMESTAMP '2006-01-01 00:00:40' );

-- [DQL]
SELECT EXTRACT(ISOYEAR FROM DATE '2006-01-02');

-- [DQL]
SELECT EXTRACT(WEEK FROM TIMESTAMP '2006-01-02 00:00:40');

-- [DQL]
SELECT EXTRACT ( YEAR FROM TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT date_part ( 'day' , TIMESTAMP '2001-02-16 20:38:40' );

-- [DQL]
SELECT date_part ( 'hour' , INTERVAL '4 hours 3 minutes' );


================================================================================
-- 来源: 2775_file_2775.txt
================================================================================

-- [DQL]
SELECT cash_words ( '1.23' );

-- [DQL]
SELECT convert ( 12 . 5 , text );

-- [DQL]
SELECT cast ( '22-oct-1997' as timestamp );

-- [DQL]
SELECT cast ( '22-ocX-1997' as timestamp DEFAULT '22-oct-1997' ON CONVERSION ERROR , 'DD-Mon-YYYY' );

-- [DDL]
CREATE DATABASE gaussdb_m WITH dbcompatibility 'b' ;

-- [OTHER]
\ c gaussdb_m -- 设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

-- [DQL]
SELECT CAST ( 12 AS UNSIGNED );

-- [DQL]
SELECT hextoraw ( '7D' );

-- [DQL]
SELECT numtoday ( 2 );

-- [DQL]
SELECT rawtohex ( '1234567' );

-- [SESSION]
set a_format_dev_version='s2';

-- [DQL]
select rawtohex2('12\n?$\123/2');

-- [SESSION]
set a_format_dev_version='s2';

-- [DQL]
select bit2coding('1234567890');

-- [SESSION]
set a_format_dev_version='s2';

-- [DQL]
select bit4coding('1234567890');

-- [DQL]
SELECT to_blob ( '0AADD343CDBBD' :: RAW ( 10 ));

-- [DQL]
SELECT to_bigint ( '123364545554455' );

-- [DQL]
SELECT to_binary_double ( '12345678' );

-- [DQL]
SELECT to_binary_double ( '1,2,3' , '9,9,9' );

-- [DQL]
SELECT to_binary_double ( 1 e2 default 12 on conversion error );

-- [DQL]
SELECT to_binary_double ( 'aa' default 12 on conversion error );

-- [DQL]
SELECT to_binary_double ( '12-' default 10 on conversion error , '99S' );

-- [DQL]
SELECT to_binary_double ( 'aa-' default 12 on conversion error , '99S' );

-- [DQL]
SELECT to_binary_float ( '12345678' );

-- [DQL]
SELECT to_binary_float ( '1,2,3' , '9,9,9' );

-- [DQL]
SELECT to_binary_float ( 1 e2 default 12 on conversion error );

-- [DQL]
SELECT to_binary_float ( 'aa' default 12 on conversion error );

-- [DQL]
SELECT to_binary_float ( '12-' default 10 on conversion error , '99S' );

-- [DQL]
SELECT to_binary_float ( 'aa-' default 12 on conversion error , '99S' );

-- [DQL]
SELECT to_char ( current_timestamp , 'HH12:MI:SS' );

-- [DQL]
SELECT to_char ( current_timestamp , 'FMHH12:FMMI:FMSS' );

-- [DQL]
SELECT to_char ( 125 . 8 :: real , '999D99' );

-- [DQL]
SELECT to_char ( 1485 , '9,999' );

-- [DQL]
SELECT to_char ( 1148 . 5 , '9,999.999' );

-- [DQL]
SELECT to_char ( 148 . 5 , '990999.909' );

-- [DQL]
SELECT to_char ( 123 , 'XXX' );

-- [DQL]
SELECT to_char ( interval '15h 2m 12s' , 'HH24:MI:SS' );

-- [DQL]
SELECT to_char ( 125 , '999' );

-- [DQL]
select to_char ( site ) from employee ;

-- [DQL]
SELECT to_char ( - 125 . 8 , '999D99S' );

-- [DQL]
SELECT to_char ( '01110' );

-- [DQL]
SELECT to_char ( current_timestamp , 'HH12:MI:SS' );

-- [DQL]
SELECT to_nchar ( current_timestamp , 'HH12:MI:SS' );

-- [DQL]
SELECT to_nchar ( current_timestamp , 'FMHH12:FMMI:FMSS' );

-- [DQL]
SELECT to_nchar ( 125 . 8 :: real , '999D99' );

-- [DQL]
SELECT to_nchar ( 1485 , '9,999' );

-- [DQL]
SELECT to_nchar ( 1148 . 5 , '9,999.999' );

-- [DQL]
SELECT to_nchar ( 148 . 5 , '990999.909' );

-- [DQL]
SELECT to_nchar ( 123 , 'XXX' );

-- [DQL]
SELECT to_nchar ( interval '15h 2m 12s' , 'HH24:MI:SS' );

-- [DQL]
SELECT to_nchar ( 125 , '999' );

-- [DQL]
SELECT to_nchar ( - 125 . 8 , '999D99S' );

-- [DQL]
SELECT to_nchar ( '01110' );

-- [DQL]
SELECT to_nchar ( current_timestamp , 'HH12:MI:SS' );

-- [DQL]
SELECT to_clob ( 'ABCDEF' :: RAW ( 10 ));

-- [DQL]
SELECT to_clob ( 'hello111' :: CHAR ( 15 ));

-- [DQL]
SELECT to_clob ( 'gauss123' :: NCHAR ( 10 ));

-- [DQL]
SELECT to_clob ( 'gauss234' :: VARCHAR ( 10 ));

-- [DQL]
SELECT to_clob ( 'gauss345' :: VARCHAR2 ( 10 ));

-- [DQL]
SELECT to_clob ( 'gauss456' :: NVARCHAR2 ( 10 ));

-- [DQL]
SELECT to_clob ( 'World222!' :: TEXT );

-- [DQL]
SELECT to_date ( '2015-08-14' );

-- [DQL]
SELECT to_date ( '05 Dec 2000' , 'DD Mon YYYY' );

-- [DQL]
SELECT to_date ( '2015-08-14' );

-- [DQL]
SELECT to_date ( '05 Dec 2000' , 'DD Mon YYYY' );

-- [SESSION]
set a_format_version='10c';

-- [SESSION]
set a_format_dev_version='s1';

-- [SESSION]
show nls_timestamp_format;

-- [DQL]
select to_date('12-jan-2022' default '12-apr-2022' on conversion error);

-- [DQL]
select to_date('12-ja-2022' default '12-apr-2022' on conversion error);

-- [DQL]
select to_date('2022-12-12' default '2022-01-01' on conversion error, 'yyyy-mm-dd');

-- [DQL]
SELECT to_number ( '12,454.8-' , '99G999D9S' );

-- [DQL]
SELECT to_number ( '12,454.8-' , '99G999D9S' );

-- [DQL]
select to_number ( '1e2' );

-- [DQL]
select to_number ( '123.456' );

-- [DQL]
select to_number ( '123' , '999' );

-- [DQL]
select to_number ( '123-' , '999MI' );

-- [DQL]
select to_number ( '123' default '456-' on conversion error , '999MI' );

-- [DQL]
SELECT to_timestamp ( 1284352323 );

-- [SESSION]
SHOW nls_timestamp_format ;

-- [DQL]
SELECT to_timestamp ( '12-sep-2014' );

-- [DQL]
SELECT to_timestamp ( '12-Sep-10 14:10:10.123000' , 'DD-Mon-YY HH24:MI:SS.FF' );

-- [DQL]
SELECT to_timestamp ( '-1' , 'SYYYY' );

-- [DQL]
SELECT to_timestamp ( '98' , 'RR' );

-- [DQL]
SELECT to_timestamp ( '01' , 'RR' );

-- [SESSION]
set a_format_version='10c';

-- [SESSION]
set a_format_dev_version='s1';

-- [DQL]
SELECT to_timestamp('11-Sep-11' DEFAULT '12-Sep-10 14:10:10.123000' ON CONVERSION ERROR,'DD-Mon-YY HH24:MI:SS.FF');

-- [DQL]
SELECT to_timestamp('12-Sep-10 14:10:10.123000','DD-Mon-YY HH24:MI:SSXFF');

-- [DQL]
SELECT to_timestamp ( '05 Dec 2000' , 'DD Mon YYYY' );

-- [DQL]
SELECT to_timestamp_tz ( '05 DeX 2000' DEFAULT '05 Dec 2001' ON CONVERSION ERROR , 'DD Mon YYYY' );

-- [DQL]
SELECT to_timestamp_tz ( '05 DeX 2000' DEFAULT '05 Dec 2001' ON CONVERSION ERROR , 'DD Mon YYYY' , 'nls_date_language=AMERICAN' );

-- [DQL]
select to_dsinterval ( '12 1:2:3.456' );

-- [DQL]
select to_dsinterval ( 'P3DT4H5M6S' );

-- [DQL]
select to_yminterval ( '1-1' );

-- [DQL]
select to_yminterval ( 'P13Y3M4DT4H2M5S' );

-- [DDL]
create table json_doc ( data CLOB );

-- [DML_INSERT]
insert into json_doc values ( '{"name":"a"}' );

-- [DQL]
select treat ( data as json ) from json_doc ;

-- [DDL]
create or replace procedure p1 is gaussdb $ # type t1 is table of int ;

-- [PLSQL]
call p1 ();

-- [DDL]
create type t1 is table of int ;

-- [DQL]
select cast ( t1 ( 1 , 2 , 3 ) as int []) result ;

-- [DDL]
create or replace package pkg1 is gaussdb $ # type t1 is table of int index by int ;

-- [DDL]
create or replace package body pkg1 is gaussdb $ # procedure p1 () is gaussdb $ # v1 t1 : = t1 ( 1 => 1 , 2 => 2 , 3 => 3 );

-- [PLSQL]
call pkg1 . p1 ();

-- [DQL]
select indexbytableint_to_array ( pkg1 . t1 ( 1 => 1 , 2 => 2 , 3 => 3 ));

-- [DQL]
SELECT convert_to_nocase ( '12345' , 'GBK' );


================================================================================
-- 来源: 2776_file_2776.txt
================================================================================

-- [DQL]
SELECT box '((0,0),(1,1))' + point '(2.0,0)' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(1,1))' - point '(2.0,0)' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(1,1))' * point '(2.0,0)' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(2,2))' / point '(2.0,0)' AS RESULT ;

-- [DQL]
SELECT box '((1,-1),(-1,1))' # box '((1,1),(-2,-2))' AS RESULT ;

-- [DQL]
SELECT # path '((1,0),(0,1),(-1,0))' AS RESULT ;

-- [DQL]
SELECT @-@ path '((0,0),(1,0))' AS RESULT ;

-- [DQL]
SELECT @@ circle '((0,0),10)' AS RESULT ;

-- [DQL]
SELECT circle '((0,0),1)' <-> circle '((5,0),1)' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(1,1))' && box '((0,0),(2,2))' AS RESULT ;

-- [DQL]
SELECT circle '((0,0),1)' << circle '((5,0),1)' AS RESULT ;

-- [DQL]
SELECT circle '((5,0),1)' >> circle '((0,0),1)' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(1,1))' &< box '((0,0),(2,2))' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(3,3))' &> box '((0,0),(2,2))' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(3,3))' <<| box '((3,4),(5,5))' AS RESULT ;

-- [DQL]
SELECT box '((3,4),(5,5))' |>> box '((0,0),(3,3))' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(1,1))' &<| box '((0,0),(2,2))' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(3,3))' |&> box '((0,0),(2,2))' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(-3,-3))' <^ box '((0,0),(2,2))' AS RESULT ;

-- [DQL]
SELECT box '((0,0),(2,2))' >^ box '((0,0),(-3,-3))' AS RESULT ;

-- [DQL]
SELECT lseg '((-1,0),(1,0))' ?# box '((-2,-2),(2,2))' AS RESULT ;

-- [DQL]
SELECT ?- lseg '((-1,0),(1,0))' AS RESULT ;

-- [DQL]
SELECT point '(1,0)' ?- point '(0,0)' AS RESULT ;

-- [DQL]
SELECT ?| lseg '((-1,0),(1,0))' AS RESULT ;

-- [DQL]
SELECT point '(0,1)' ?| point '(0,0)' AS RESULT ;

-- [DQL]
SELECT lseg '((0,0),(0,1))' ?-| lseg '((0,0),(1,0))' AS RESULT ;

-- [DQL]
SELECT lseg '((-1,0),(1,0))' ?|| lseg '((-1,2),(1,2))' AS RESULT ;

-- [DQL]
SELECT circle '((0,0),2)' @> point '(1,1)' AS RESULT ;

-- [DQL]
SELECT point '(1,1)' <@ circle '((0,0),2)' AS RESULT ;

-- [DQL]
SELECT polygon '((0,0),(1,1))' ~= polygon '((1,1),(0,0))' AS RESULT ;

-- [DQL]
SELECT area ( box '((0,0),(1,1))' ) AS RESULT ;

-- [DQL]
SELECT center ( box '((0,0),(1,2))' ) AS RESULT ;

-- [DQL]
SELECT diameter ( circle '((0,0),2.0)' ) AS RESULT ;

-- [DQL]
SELECT height ( box '((0,0),(1,1))' ) AS RESULT ;

-- [DQL]
SELECT isclosed ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- [DQL]
SELECT isopen ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

-- [DQL]
SELECT length ( path '((-1,0),(1,0))' ) AS RESULT ;

-- [DQL]
SELECT npoints ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

-- [DQL]
SELECT npoints ( polygon '((1,1),(0,0))' ) AS RESULT ;

-- [DQL]
SELECT pclose ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

-- [DQL]
SELECT popen ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- [DQL]
SELECT radius ( circle '((0,0),2.0)' ) AS RESULT ;

-- [DQL]
SELECT width ( box '((0,0),(1,1))' ) AS RESULT ;

-- [DQL]
SELECT box ( circle '((0,0),2.0)' ) AS RESULT ;

-- [DQL]
SELECT box ( point '(0,0)' , point '(1,1)' ) AS RESULT ;

-- [DQL]
SELECT box ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- [DQL]
SELECT circle ( box '((0,0),(1,1))' ) AS RESULT ;

-- [DQL]
SELECT circle ( point '(0,0)' , 2 . 0 ) AS RESULT ;

-- [DQL]
SELECT circle ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- [DQL]
SELECT lseg ( box '((-1,0),(1,0))' ) AS RESULT ;

-- [DQL]
SELECT lseg ( point '(-1,0)' , point '(1,0)' ) AS RESULT ;

-- [DQL]
SELECT slope(point '(1,1)', point '(0,0)') AS RESULT;

-- [DQL]
SELECT path ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- [DQL]
SELECT point ( 23 . 4 , - 44 . 5 ) AS RESULT ;

-- [DQL]
SELECT point ( box '((-1,0),(1,0))' ) AS RESULT ;

-- [DQL]
SELECT point ( circle '((0,0),2.0)' ) AS RESULT ;

-- [DQL]
SELECT point ( lseg '((-1,0),(1,0))' ) AS RESULT ;

-- [DQL]
SELECT point ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- [DQL]
SELECT polygon ( box '((0,0),(1,1))' ) AS RESULT ;

-- [DQL]
SELECT polygon ( circle '((0,0),2.0)' ) AS RESULT ;

-- [DQL]
SELECT polygon ( 12 , circle '((0,0),2.0)' ) AS RESULT ;

-- [DQL]
SELECT polygon ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;


================================================================================
-- 来源: 2777_file_2777.txt
================================================================================

-- [DQL]
SELECT inet '192.168.1.5' < inet '192.168.1.6' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.5' <= inet '192.168.1.5' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.5' = inet '192.168.1.5' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.5' >= inet '192.168.1.5' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.5' > inet '192.168.1.4' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.5' <> inet '192.168.1.4' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.5' << inet '192.168.1/24' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1/24' <<= inet '192.168.1/24' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1/24' >> inet '192.168.1.5' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1/24' >>= inet '192.168.1/24' AS RESULT ;

-- [DQL]
SELECT ~ inet '192.168.1.6' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.6' & inet '10.0.0.0' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.6' | inet '10.0.0.0' AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.6' + 25 AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.43' - 36 AS RESULT ;

-- [DQL]
SELECT inet '192.168.1.43' - inet '192.168.1.19' AS RESULT ;

-- [DQL]
SELECT abbrev ( inet '10.1.0.0/16' ) AS RESULT ;

-- [DQL]
SELECT abbrev ( cidr '10.1.0.0/16' ) AS RESULT ;

-- [DQL]
SELECT broadcast ( '192.168.1.5/24' ) AS RESULT ;

-- [DQL]
SELECT family ( '127.0.0.1' ) AS RESULT ;

-- [DQL]
SELECT host ( '192.168.1.5/24' ) AS RESULT ;

-- [DQL]
SELECT hostmask ( '192.168.23.20/30' ) AS RESULT ;

-- [DQL]
SELECT masklen ( '192.168.1.5/24' ) AS RESULT ;

-- [DQL]
SELECT netmask ( '192.168.1.5/24' ) AS RESULT ;

-- [DQL]
SELECT network ( '192.168.1.5/24' ) AS RESULT ;

-- [DQL]
SELECT set_masklen ( '192.168.1.5/24' , 16 ) AS RESULT ;

-- [DQL]
SELECT set_masklen ( '192.168.1.0/24' :: cidr , 16 ) AS RESULT ;

-- [DQL]
SELECT text ( inet '192.168.1.5' ) AS RESULT ;

-- [DQL]
SELECT trunc ( macaddr '12:34:56:78:90:ab' ) AS RESULT ;


================================================================================
-- 来源: 2778_file_2778.txt
================================================================================

-- [DQL]
SELECT to_tsvector ( 'fat cats ate rats' ) @@ to_tsquery ( 'cat & rat' ) AS RESULT ;

-- [DQL]
SELECT to_tsvector ( 'fat cats ate rats' ) @@@ to_tsquery ( 'cat & rat' ) AS RESULT ;

-- [DQL]
SELECT 'a:1 b:2' :: tsvector || 'c:1 d:2 b:3' :: tsvector AS RESULT ;

-- [DQL]
SELECT 'fat | rat' :: tsquery && 'cat' :: tsquery AS RESULT ;

-- [DQL]
SELECT 'fat | rat' :: tsquery || 'cat' :: tsquery AS RESULT ;

-- [DQL]
SELECT !! 'cat' :: tsquery AS RESULT ;

-- [DQL]
SELECT 'cat' :: tsquery @> 'cat & rat' :: tsquery AS RESULT ;

-- [DQL]
SELECT 'cat' :: tsquery <@ 'cat & rat' :: tsquery AS RESULT ;

-- [DQL]
SELECT get_current_ts_config ();

-- [DQL]
SELECT length ( 'fat:2,4 cat:3 rat:5A' :: tsvector );

-- [DQL]
SELECT numnode ( '(fat & rat) | cat' :: tsquery );

-- [DQL]
SELECT plainto_tsquery ( 'english' , 'The Fat Rats' );

-- [DQL]
SELECT querytree ( 'foo & ! bar' :: tsquery );

-- [DQL]
SELECT setweight ( 'fat:2,4 cat:3 rat:5B' :: tsvector , 'A' );

-- [DQL]
SELECT strip ( 'fat:2,4 cat:3 rat:5A' :: tsvector );

-- [DQL]
SELECT to_tsquery ( 'english' , 'The & Fat & Rats' );

-- [DQL]
SELECT to_tsvector ( 'english' , 'The Fat Rats' );

-- [DQL]
SELECT to_tsvector_for_batch ( 'english' , 'The Fat Rats' );

-- [DQL]
SELECT ts_headline ( 'x y z' , 'z' :: tsquery );

-- [DQL]
SELECT ts_rank ( 'hello world' :: tsvector , 'world' :: tsquery );

-- [DQL]
SELECT ts_rank_cd ( 'hello world' :: tsvector , 'world' :: tsquery );

-- [DQL]
SELECT ts_rewrite ( 'a & b' :: tsquery , 'a' :: tsquery , 'foo|bar' :: tsquery );

-- [DQL]
SELECT ts_rewrite ( 'world' :: tsquery , 'select ''world''::tsquery, ''hello''::tsquery' );

-- [DQL]
SELECT ts_debug ( 'english' , 'The Brightest supernovaes' );

-- [DQL]
SELECT ts_lexize ( 'english_stem' , 'stars' );

-- [DQL]
SELECT ts_parse ( 'default' , 'foo - bar' );

-- [DQL]
SELECT ts_parse ( 3722 , 'foo - bar' );

-- [DQL]
SELECT ts_token_type ( 'default' );

-- [DQL]
SELECT ts_token_type ( 3722 );

-- [DQL]
SELECT ts_stat ( 'select ''hello world''::tsvector' );


================================================================================
-- 来源: 2779_JSON_JSONB.txt
================================================================================

-- [DQL]
SELECT array_to_json('{{1,5},{99,100}}'::int[]);

-- [DQL]
SELECT row_to_json(row(1,'foo'));

-- [DQL]
SELECT json_array_element('[1,true,[1,[2,3]],null]',2);

-- [DQL]
SELECT json_array_element_text('[1,true,[1,[2,3]],null]',2);

-- [DQL]
SELECT json_object_field('{"a": {"b":"foo"}}','a');

-- [DQL]
SELECT json_object_field_text('{"a": {"b":"foo"}}','a');

-- [DQL]
SELECT json_extract_path('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', 'f4','f6');

-- [DQL]
SELECT json_extract_path_op('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', ARRAY['f4','f6']);

-- [DQL]
SELECT json_extract_path_text('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', 'f4','f6');

-- [DQL]
SELECT json_extract_path_text_op('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', ARRAY['f4','f6']);

-- [DQL]
SELECT json_array_elements('[1,true,[1,[2,3]],null]');

-- [DQL]
SELECT * FROM json_array_elements_text('[1,true,[1,[2,3]],null]');

-- [DQL]
SELECT json_array_length('[1,2,3,{"f1":1,"f2":[5,6]},4,null]');

-- [DQL]
SELECT * FROM json_each('{"f1":[1,2,3],"f2":{"f3":1},"f4":null}');

-- [DQL]
SELECT * FROM json_each_text('{"f1":[1,2,3],"f2":{"f3":1},"f4":null}');

-- [DQL]
SELECT json_object_keys('{"f1":"abc","f2":{"f3":"a", "f4":"b"}, "f1":"abcd"}');

-- [DQL]
SELECT jsonb_object_keys('{"f1":"abc","f2":{"f3":"a", "f4":"b"}, "f1":"abcd"}');

-- [DDL]
CREATE TYPE jpop AS (a text, b int, c bool);

-- [DQL]
SELECT * FROM json_populate_record(null::jpop,'{"a":"blurfl","x":43.2}');

-- [DQL]
SELECT * FROM json_populate_record((1,1,null)::jpop,'{"a":"blurfl","x":43.2}');

-- [DDL]
DROP TYPE jpop;

-- [DDL]
CREATE TYPE jpop AS (a text, b int, c bool);

-- [DQL]
SELECT * FROM json_populate_recordset(null::jpop, '[{"a":1,"b":2},{"a":3,"b":4}]');

-- [DDL]
DROP TYPE jpop;

-- [DQL]
SELECT value, json_typeof(value) FROM (values (json '123.4'), (json '"foo"'), (json 'true'), (json 'null'), (json '[1, 2, 3]'), (json '{"x":"foo", "y":123}'), (NULL::json)) AS data(value);

-- [DQL]
SELECT json_build_array('a',1,'b',1.2,'c',true,'d',null,'e',json '{"x": 3, "y": [1,2,3]}','');

-- [DQL]
SELECT json_build_object(1,2);

-- [DQL]
SELECT jsonb_build_object('a',1,'b',1.2,'c',true,'d',null,'e',json '{"x": 3, "y": [1,2,3]}');

-- [DQL]
SELECT jsonb_build_object();

-- [DQL]
SELECT * FROM json_to_record('{"a":1,"b":"foo","c":"bar"}',true) AS x(a int, b text, d text);

-- [DQL]
SELECT * FROM json_to_record('{"a": {"x": 1, "y": 2},"b":"foo","c":[1, 2]}') AS x(a json, b text, c int[]);

-- [DQL]
SELECT * FROM json_to_recordset('[{"a":1,"b":"foo","d":false},{"a":2,"b":"bar","c":true}]',false) AS x(a int, b text, c boolean);

-- [DQL]
SELECT json_object('{a,1,b,2,3,NULL,"d e f","a b c"}');

-- [DQL]
SELECT json_object('{a,b,"a b c"}', '{a,1,1}');

-- [DQL]
SELECT json_object('d',2,'c','name','b',true,'a',2,'a',NULL,'d',1);

-- [DQL]
SELECT json_object('d',2,true,'name','b',true,'a',2,'aa', current_timestamp);

-- [DQL]
SELECT json_array_append('[1, [2, 3]]', '$[1]', 4, '$[0]', false, '$[0]', null, '$[0]', current_timestamp);

-- [DQL]
SELECT json_array();

-- [DQL]
SELECT json_array(TRUE, FALSE, NULL, 114, 'text', current_timestamp);

-- [DQL]
SELECT json_array_insert('[1, [2, 3]]', '$[1]', 4);

-- [DQL]
SELECT json_array_insert('{"x": 1, "y": [1, 2]}', '$.y[0]', NULL, '$.y[0]', 123, '$.y[3]', current_timestamp);

-- [DQL]
SELECT json_contains('[1, 2, {"x": 3}]', '{"x":3}');

-- [DQL]
SELECT json_contains('[1, 2, {"x": 3},[1,2,3,4]]', '2','$[1]');

-- [DQL]
SELECT json_contains('[1, 2, {"x": 3},[1,2,3,4]]', '1','$[1]');

-- [DQL]
SELECT json_contains_path('[1, 2, {"x": 3}]', 'all', '$[0]', '$[1]', '$[2]');

-- [DQL]
SELECT json_contains_path('[1, 2, {"x": 3}]', 'all', '$[0]', '$[1]', '$[6]');

-- [DQL]
SELECT json_contains_path('[1, 2, {"x": 3}]', 'one', '$[0]', '$[1]', '$[5]');

-- [DQL]
SELECT json_depth('[]');

-- [DQL]
SELECT json_depth('{"s":1, "x":2,"y":[1]}');

-- [DQL]
SELECT json_extract('[1, 2, {"x": 3}]', '$[2]');

-- [DQL]
SELECT json_extract('["a", ["b", "c"], "d"]', '$[1]', '$[2]', '$[3]');

-- [DQL]
SELECT json_insert('[1, [2, 3], {"a": [4, 5]}]', '$[3]', 2);

-- [DQL]
SELECT json_insert('[1, [2, 3], {"a": [4, 5]}]', '$[10]', 10,'$[5]', 5);

-- [DQL]
SELECT json_keys('{"x": 1, "y": 2, "z": 3}');

-- [DQL]
SELECT json_keys('[1,2,3,{"name":"Tom"}]','$[3]');

-- [DQL]
SELECT json_length('[1,2,3,4,5]');

-- [DQL]
SELECT json_length('{"name":"Tom", "age":24, "like":"football"}');

-- [DQL]
SELECT json_merge('[1, 2]','[2]');

-- [DQL]
SELECT json_merge('{"b":"2"}','{"a":"1"}','[1,2]');

-- [DQL]
SELECT json_quote('gauss');

-- [DQL]
SELECT json_unquote('"gauss"');

-- [DQL]
SELECT json_remove('[0, 1, 2, [3, 4]]', '$[0]', '$[2]');

-- [DQL]
SELECT json_remove('[0, 1, 2, [3, 4]]', '$[0]', '$[0]','$[0]');

-- [DQL]
SELECT json_replace('{"x": 1}', '$.x', 'true');

-- [DQL]
SELECT json_replace('{"x": 1}', '$.x', true, '$.x', 123, '$.x', 'asd', '$.x', null);

-- [DQL]
SELECT json_search('{"a":"abc","b":"abc"}','all','abc');

-- [DQL]
SELECT json_search('{"a":"abc","b":"abc"}','one','abc');

-- [DQL]
SELECT json_search('{"a":"abc","b":"a%c"}','one','a\%c');

-- [DQL]
SELECT json_set('{"s":3}','$.s','d');

-- [DQL]
SELECT json_set('{"s":3}','$.a','d','$.a','1');

-- [DQL]
SELECT json_type('{"w":{"2":3},"2":4}');

-- [DQL]
SELECT json_type('[1,2,2,3,3,4,4,4,4,4,4,4,4]');

-- [DQL]
SELECT json_valid('{"name":"Tom"}');

-- [DQL]
SELECT json_valid('[1,23,4,5,5]');

-- [DQL]
SELECT json_valid('[1,23,4,5,5]}');

-- [DDL]
CREATE TABLE classes(name varchar, score int);

-- [DML_INSERT]
INSERT INTO classes VALUES('A',2);

-- [DML_INSERT]
INSERT INTO classes VALUES('A',3);

-- [DML_INSERT]
INSERT INTO classes VALUES('D',5);

-- [DML_INSERT]
INSERT INTO classes VALUES('D',null);

-- [DQL]
SELECT * FROM classes;

-- [DQL]
SELECT name, json_agg(score) score FROM classes GROUP BY name ORDER BY name;

-- [DDL]
DROP TABLE classes;

-- [DDL]
CREATE TABLE classes(name varchar, score int);

-- [DML_INSERT]
INSERT INTO classes VALUES('A',2);

-- [DML_INSERT]
INSERT INTO classes VALUES('A',3);

-- [DML_INSERT]
INSERT INTO classes VALUES('D',5);

-- [DML_INSERT]
INSERT INTO classes VALUES('D',null);

-- [DQL]
SELECT * FROM classes;

-- [DQL]
SELECT json_object_agg(name, score) FROM classes GROUP BY name ORDER BY name;

-- [DDL]
DROP TABLE classes;

-- [DQL]
SELECT jsonb_contained('[1,2,3]', '[1,2,3,4]');

-- [DQL]
SELECT jsonb_contains('[1,2,3,4]', '[1,2,3]');

-- [DQL]
SELECT jsonb_exists('["1",2,3]', '1');

-- [DQL]
SELECT jsonb_exists_all('["1","2",3]', '{1, 2}');

-- [DQL]
SELECT jsonb_exists_any('["1","2",3]', '{1, 2, 4}');

-- [DQL]
SELECT jsonb_cmp('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT jsonb_eq('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT jsonb_ne('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT jsonb_gt('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT jsonb_ge('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT jsonb_lt('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT jsonb_le('["a", "b"]', '{"a":1, "b":2}');

-- [DQL]
SELECT to_json('{1,5}'::text[]);

-- [DQL]
SELECT to_jsonb(array[1, 2, 3, 4]);

-- [DQL]
SELECT jsonb_hash('[1,2,3]');


================================================================================
-- 来源: 2780_HLL.txt
================================================================================

-- [DQL]
SELECT hll_hash_boolean ( FALSE );

-- [DQL]
SELECT hll_hash_boolean ( FALSE , 10 );

-- [DQL]
SELECT hll_hash_smallint ( 100 :: smallint );

-- [DQL]
SELECT hll_hash_smallint ( 100 :: smallint , 10 );

-- [DQL]
SELECT hll_hash_integer ( 0 );

-- [DQL]
SELECT hll_hash_integer ( 0 , 10 );

-- [DQL]
SELECT hll_hash_bigint ( 100 :: bigint );

-- [DQL]
SELECT hll_hash_bigint ( 100 :: bigint , 10 );

-- [DQL]
SELECT hll_hash_bytea ( E '\\x' );

-- [DQL]
SELECT hll_hash_bytea ( E '\\x' , 10 );

-- [DQL]
SELECT hll_hash_text ( 'AB' );

-- [DQL]
SELECT hll_hash_text ( 'AB' , 10 );

-- [DQL]
SELECT hll_hash_any ( 1 );

-- [DQL]
SELECT hll_hash_any ( '08:00:2b:01:02:03' :: macaddr );

-- [DQL]
SELECT hll_hash_any ( 1 , 10 );

-- [DQL]
SELECT hll_hashval_eq ( hll_hash_integer ( 1 ), hll_hash_integer ( 1 ));

-- [DQL]
SELECT hll_hashval_ne ( hll_hash_integer ( 1 ), hll_hash_integer ( 1 ));

-- [DQL]
SELECT hll_print ( hll_empty ());

-- [DQL]
SELECT hll_type ( hll_empty ());

-- [DQL]
SELECT hll_log2m ( hll_empty ());

-- [DQL]
SELECT hll_log2m ( hll_empty ( 10 ));

-- [DQL]
SELECT hll_log2m ( hll_empty ( - 1 ));

-- [DQL]
SELECT hll_log2explicit ( hll_empty ());

-- [DQL]
SELECT hll_log2explicit ( hll_empty ( 12 , 8 ));

-- [DQL]
SELECT hll_log2explicit ( hll_empty ( 12 , - 1 ));

-- [DQL]
SELECT hll_log2sparse ( hll_empty ());

-- [DQL]
SELECT hll_log2sparse ( hll_empty ( 12 , 8 , 10 ));

-- [DQL]
SELECT hll_log2sparse ( hll_empty ( 12 , 8 , - 1 ));

-- [DQL]
SELECT hll_duplicatecheck ( hll_empty ());

-- [DQL]
SELECT hll_duplicatecheck ( hll_empty ( 12 , 8 , 10 , 1 ));

-- [DQL]
SELECT hll_duplicatecheck ( hll_empty ( 12 , 8 , 10 , - 1 ));

-- [DQL]
SELECT hll_empty ();

-- [DQL]
SELECT hll_empty ( 10 );

-- [DQL]
SELECT hll_empty ( - 1 );

-- [DQL]
SELECT hll_empty ( 10 , 4 );

-- [DQL]
SELECT hll_empty ( 10 , - 1 );

-- [DQL]
SELECT hll_empty ( 10 , 4 , 8 );

-- [DQL]
SELECT hll_empty ( 10 , 4 , - 1 );

-- [DQL]
SELECT hll_empty ( 10 , 4 , 8 , 0 );

-- [DQL]
SELECT hll_empty ( 10 , 4 , 8 , - 1 );

-- [DQL]
SELECT hll_add ( hll_empty (), hll_hash_integer ( 1 ));

-- [DQL]
SELECT hll_add_rev ( hll_hash_integer ( 1 ), hll_empty ());

-- [DQL]
SELECT hll_eq ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

-- [DQL]
SELECT hll_ne ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

-- [DQL]
SELECT hll_cardinality ( hll_empty () || hll_hash_integer ( 1 ));

-- [DQL]
SELECT hll_union ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

-- [DDL]
CREATE TABLE t_id ( id int );

-- [DML_INSERT]
INSERT INTO t_id VALUES ( generate_series ( 1 , 500 ));

-- [DDL]
CREATE TABLE t_data ( a int , c text );

-- [DML_INSERT]
INSERT INTO t_data SELECT mod ( id , 2 ), id FROM t_id ;

-- [DDL]
CREATE TABLE t_a_c_hll ( a int , c hll );

-- [DML_INSERT]
INSERT INTO t_a_c_hll SELECT a , hll_add_agg ( hll_hash_text ( c )) FROM t_data GROUP BY a ;

-- [DQL]
SELECT a , # c AS cardinality FROM t_a_c_hll ORDER BY a ;

-- [DQL]
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), 12 )) FROM t_data ;

-- [DQL]
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 1 )) FROM t_data ;

-- [DQL]
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 6 , 10 )) FROM t_data ;

-- [DQL]
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 6 , 10 , - 1 )) FROM t_data ;

-- [DQL]
SELECT # hll_union_agg ( c ) AS cardinality FROM t_a_c_hll ;

-- [DQL]
SELECT ( hll_empty () || hll_hash_integer ( 1 )) = ( hll_empty () || hll_hash_integer ( 1 ));

-- [DQL]
SELECT hll_hash_integer ( 1 ) = hll_hash_integer ( 1 );

-- [DQL]
SELECT ( hll_empty () || hll_hash_integer ( 1 )) <> ( hll_empty () || hll_hash_integer ( 2 ));

-- [DQL]
SELECT hll_hash_integer ( 1 ) <> hll_hash_integer ( 2 );

-- [DQL]
SELECT hll_empty () || hll_hash_integer ( 1 );

-- [DQL]
SELECT hll_hash_integer ( 1 ) || hll_empty ();

-- [DQL]
SELECT ( hll_empty () || hll_hash_integer ( 1 )) || ( hll_empty () || hll_hash_integer ( 2 ));

-- [DQL]
SELECT # ( hll_empty () || hll_hash_integer ( 1 ));


================================================================================
-- 来源: 2781_SEQUENCE.txt
================================================================================

-- [DDL]
CREATE SEQUENCE seqDemo ;

-- [DQL]
SELECT nextval ( 'seqDemo' );

-- [DQL]
SELECT seqDemo . nextval ;

-- [DDL]
DROP SEQUENCE seqDemo ;

-- [DDL]
CREATE SEQUENCE seq1 ;

-- [DQL]
SELECT nextval ( 'seq1' );

-- [DQL]
SELECT currval ( 'seq1' );

-- [DQL]
SELECT seq1 . currval ;

-- [DDL]
DROP SEQUENCE seq1 ;

-- [DDL]
CREATE SEQUENCE seq1 ;

-- [DQL]
SELECT nextval ( 'seq1' );

-- [DQL]
SELECT lastval ();

-- [DDL]
DROP SEQUENCE seq1 ;

-- [DDL]
CREATE SEQUENCE seqDemo ;

-- [DQL]
SELECT nextval ( 'seqDemo' );

-- [DQL]
SELECT setval ( 'seqDemo' , 5 );

-- [DDL]
DROP SEQUENCE seqDemo ;

-- [DDL]
CREATE SEQUENCE seqDemo ;

-- [DQL]
SELECT nextval ( 'seqDemo' );

-- [DQL]
SELECT setval ( 'seqDemo' , 5 , true );

-- [DDL]
DROP SEQUENCE seqDemo ;

-- [DQL]
SELECT last_insert_id ( 100 );

-- [DQL]
SELECT last_insert_id ();


================================================================================
-- 来源: 2782_file_2782.txt
================================================================================

-- [DQL]
SELECT ARRAY [ 1 . 1 , 2 . 1 , 3 . 1 ]:: int [] = ARRAY [ 1 , 2 , 3 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 2 , 3 ] <> ARRAY [ 1 , 2 , 4 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 2 , 3 ] < ARRAY [ 1 , 2 , 4 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 4 , 3 ] > ARRAY [ 1 , 2 , 4 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 2 , 3 ] <= ARRAY [ 1 , 2 , 3 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 4 , 3 ] >= ARRAY [ 1 , 4 , 3 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 4 , 3 ] @> ARRAY [ 3 , 1 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 2 , 7 ] <@ ARRAY [ 1 , 7 , 4 , 2 , 6 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 4 , 3 ] && ARRAY [ 2 , 1 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 2 , 3 ] || ARRAY [ 4 , 5 , 6 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 1 , 2 , 3 ] || ARRAY [[ 4 , 5 , 6 ],[ 7 , 8 , 9 ]] AS RESULT ;

-- [DQL]
SELECT 3 || ARRAY [ 4 , 5 , 6 ] AS RESULT ;

-- [DQL]
SELECT ARRAY [ 4 , 5 , 6 ] || 7 AS RESULT ;

-- [DQL]
SELECT array_append ( ARRAY [ 1 , 2 ], 3 ) AS RESULT ;

-- [DQL]
SELECT array_prepend ( 1 , ARRAY [ 2 , 3 ]) AS RESULT ;

-- [DQL]
SELECT array_cat ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_cat ( ARRAY [[ 1 , 2 ],[ 4 , 5 ]], ARRAY [ 6 , 7 ]) AS RESULT ;

-- [DQL]
SELECT array_union ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_union ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- [DQL]
SELECT array_union_distinct ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_union_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- [DQL]
SELECT array_intersect ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_intersect ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- [DQL]
SELECT array_intersect_distinct ( ARRAY [ 1 , 2 , 2 ], ARRAY [ 2 , 2 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_intersect_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- [DQL]
SELECT array_except ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_except ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- [DQL]
SELECT array_except ( NULL , ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_except_distinct ( ARRAY [ 1 , 2 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_except_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- [DQL]
SELECT array_except_distinct ( NULL , ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- [DQL]
SELECT array_ndims ( ARRAY [[ 1 , 2 , 3 ], [ 4 , 5 , 6 ]]) AS RESULT ;

-- [DQL]
SELECT array_dims ( ARRAY [[ 1 , 2 , 3 ], [ 4 , 5 , 6 ]]) AS RESULT ;

-- [DQL]
SELECT array_length ( array [ 1 , 2 , 3 ], 1 ) AS RESULT ;

-- [DQL]
SELECT array_length ( array [[ 1 , 2 , 3 ],[ 4 , 5 , 6 ]], 2 ) AS RESULT ;

-- [DQL]
SELECT array_lower ( '[0:2]={1,2,3}' :: int [], 1 ) AS RESULT ;

-- [DQL]
SELECT array_upper ( ARRAY [ 1 , 8 , 3 , 7 ], 1 ) AS RESULT ;

-- [DQL]
SELECT array_to_string ( ARRAY [ 1 , 2 , 3 , NULL , 5 ], ',' , '*' ) AS RESULT ;

-- [DQL]
SELECT array_delete(ARRAY[1,8,3,7]) AS RESULT;

-- [DQL]
SELECT array_deleteidx(ARRAY[1,2,3,4,5], 1) AS RESULT;

-- [DQL]
SELECT array_extendnull(ARRAY[1,8,3,7],1) AS RESULT;

-- [DQL]
SELECT array_extendnull(ARRAY[1,8,3,7],2,2) AS RESULT;

-- [DQL]
SELECT array_trim(ARRAY[1,8,3,7],1) AS RESULT;

-- [DQL]
SELECT array_exists(ARRAY[1,8,3,7],1) AS RESULT;

-- [DQL]
SELECT array_next(ARRAY[1,8,3,7],1) AS RESULT;

-- [DQL]
SELECT array_prior(ARRAY[1,8,3,7],2) AS RESULT;

-- [DQL]
SELECT string_to_array ( 'xx~^~yy~^~zz' , '~^~' , 'yy' ) AS RESULT ;

-- [DQL]
SELECT string_to_array ( 'xx~^~yy~^~zz' , '~^~' , 'y' ) AS RESULT ;

-- [DQL]
SELECT unnest ( ARRAY [ 1 , 2 ]) AS RESULT ;

-- [PLSQL]
call f1 ();

-- [PLSQL]
call f1 ();

-- [DQL]
SELECT cardinality(array[[1, 2], [3, 4]]);

-- [DQL]
SELECT array_positions(array[1, 2, 3, 1], 1) AS RESULT;


================================================================================
-- 来源: 2783_file_2783.txt
================================================================================

-- [DQL]
SELECT int4range ( 1 , 5 ) = '[1,4]' :: int4range AS RESULT ;

-- [DQL]
SELECT numrange ( 1 . 1 , 2 . 2 ) <> numrange ( 1 . 1 , 2 . 3 ) AS RESULT ;

-- [DQL]
SELECT int4range ( 1 , 10 ) < int4range ( 2 , 3 ) AS RESULT ;

-- [DQL]
SELECT int4range ( 1 , 10 ) > int4range ( 1 , 5 ) AS RESULT ;

-- [DQL]
SELECT numrange ( 1 . 1 , 2 . 2 ) <= numrange ( 1 . 1 , 2 . 2 ) AS RESULT ;

-- [DQL]
SELECT numrange ( 1 . 1 , 2 . 2 ) >= numrange ( 1 . 1 , 2 . 0 ) AS RESULT ;

-- [DQL]
SELECT int4range ( 2 , 4 ) @> int4range ( 2 , 3 ) AS RESULT ;

-- [DQL]
SELECT '[2011-01-01,2011-03-01)' :: tsrange @> '2011-01-10' :: timestamp AS RESULT ;

-- [DQL]
SELECT int4range ( 2 , 4 ) <@ int4range ( 1 , 7 ) AS RESULT ;

-- [DQL]
SELECT 42 <@ int4range ( 1 , 7 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 3 , 7 ) && int8range ( 4 , 12 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 1 , 10 ) << int8range ( 100 , 110 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 50 , 60 ) >> int8range ( 20 , 30 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 1 , 20 ) &< int8range ( 18 , 20 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 7 , 20 ) &> int8range ( 5 , 10 ) AS RESULT ;

-- [DQL]
SELECT numrange ( 1 . 1 , 2 . 2 ) -|- numrange ( 2 . 2 , 3 . 3 ) AS RESULT ;

-- [DQL]
SELECT numrange ( 5 , 15 ) + numrange ( 10 , 20 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 5 , 15 ) * int8range ( 10 , 20 ) AS RESULT ;

-- [DQL]
SELECT int8range ( 5 , 15 ) - int8range ( 10 , 20 ) AS RESULT ;

-- [DQL]
SELECT numrange ( 1 . 1 , 2 . 2 ) AS RESULT ;

-- [DQL]
SELECT numrange ( 1 . 1 , 2 . 2 , '()' ) AS RESULT ;

-- [DQL]
SELECT lower ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- [DQL]
SELECT upper ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- [DQL]
SELECT isempty ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- [DQL]
SELECT lower_inc ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- [DQL]
SELECT upper_inc ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- [DQL]
SELECT lower_inf ( '(,)' :: daterange ) AS RESULT ;

-- [DQL]
SELECT upper_inf ( '(,)' :: daterange ) AS RESULT ;

-- [DQL]
SELECT elem_contained_by_range ( '2' , numrange ( 1 . 1 , 2 . 2 ));


================================================================================
-- 来源: 2784_file_2784.txt
================================================================================

-- [DDL]
CREATE TABLE tab ( a int );

-- [DML_INSERT]
INSERT INTO tab values ( 1 );

-- [DML_INSERT]
INSERT INTO tab values ( 2 );

-- [DQL]
SELECT sum ( a ) FROM tab ;

-- [DQL]
SELECT MAX ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- [DQL]
SELECT MIN ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- [DQL]
SELECT AVG ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- [DQL]
SELECT COUNT ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- [DQL]
SELECT COUNT ( * ) FROM tpcds . inventory ;

-- [DQL]
SELECT ARRAY_AGG ( sr_fee ) FROM tpcds . store_returns WHERE sr_customer_sk = 2 ;

-- [DQL]
SELECT string_agg ( sr_item_sk , ',' ) FROM tpcds . store_returns WHERE sr_item_sk < 3 ;

-- [DQL]
SELECT deptno , listagg ( ename , ',' ) WITHIN GROUP ( ORDER BY ename ) AS employees FROM emp GROUP BY deptno ;

-- [DQL]
SELECT deptno , listagg ( mgrno , ',' ) WITHIN GROUP ( ORDER BY mgrno NULLS FIRST ) AS mgrnos FROM emp GROUP BY deptno ;

-- [DQL]
SELECT job , listagg ( bonus , '($);

-- [DQL]
SELECT deptno , listagg ( hiredate , ', ' ) WITHIN GROUP ( ORDER BY hiredate DESC ) AS hiredates FROM emp GROUP BY deptno ;

-- [DQL]
SELECT deptno , listagg ( vacationTime , ';

-- [DQL]
SELECT deptno , listagg ( job ) WITHIN GROUP ( ORDER BY job ) AS jobs FROM emp GROUP BY deptno ;

-- [DQL]
SELECT deptno , mgrno , bonus , listagg ( ename , ';

-- [DQL]
SELECT id , group_concat ( v separator ';

-- [DQL]
SELECT id , group_concat ( id , v ) FROM t GROUP BY id ORDER BY id ASC ;

-- [DQL]
SELECT id , group_concat ( v ) FROM t GROUP BY id ORDER BY id ASC ;

-- [DQL]
SELECT id , group_concat ( v separator ';

-- [DQL]
SELECT id , group_concat ( v separator ';

-- [DQL]
SELECT id , group_concat ( hiredate separator ';

-- [DQL]
SELECT id , group_concat ( v separator ';

-- [DQL]
SELECT id , group_concat ( vacationt separator ';

-- [DQL]
SELECT id , group_concat ( distinct v ) FROM t GROUP BY id ORDER BY id ASC ;

-- [DQL]
SELECT id , group_concat ( v ORDER BY v desc ) FROM t GROUP BY id ORDER BY id ASC ;

-- [DQL]
SELECT COVAR_POP ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT COVAR_SAMP ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT STDDEV_POP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT STDDEV_SAMP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT VAR_POP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT VAR_SAMP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT BIT_AND ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT BIT_OR ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT bool_and ( 100 < 2500 );

-- [DQL]
SELECT bool_or ( 100 < 2500 );

-- [DQL]
SELECT CORR ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT every ( 100 < 2500 );

-- [DQL]
SELECT REGR_AVGX ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_AVGY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_COUNT ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_INTERCEPT ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_R2 ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_SLOPE ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_SXX ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_SXY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT REGR_SYY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- [DQL]
SELECT STDDEV ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT VARIANCE ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- [DQL]
SELECT CHECKSUM ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- [DQL]
SELECT CHECKSUM ( inv_quantity_on_hand :: TEXT ) FROM tpcds . inventory ;

-- [DQL]
SELECT CHECKSUM ( inventory :: TEXT ) FROM tpcds . inventory ;

-- [DQL]
SELECT percentile_cont(0) WITHIN GROUP (ORDER BY value) FROM (VALUES (1),(2)) v(value);

-- [DQL]
SELECT mode() WITHIN GROUP (ORDER BY value) FROM (values(1, 'a'), (2, 'b'), (2, 'c')) v(value, tag);

-- [DQL]
SELECT mode() WITHIN GROUP (ORDER BY tag) FROM (values(1, 'a'), (2, 'b'), (2, 'c')) v(value, tag);

-- [DQL]
SELECT * FROM pivot_func_test;

-- [DQL]
SELECT id, pivot_func(val) FROM pivot_func_test GROUP BY id;


================================================================================
-- 来源: 2785_file_2785.txt
================================================================================

-- [DQL]
SELECT d_moy , d_fy_week_seq , rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , Row_number () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , dense_rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , percent_rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , cume_dist () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim e_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , ntile ( 3 ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , lag ( d_moy , 3 , null ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy, d_fy_week_seq, lead(d_fy_week_seq,2) OVER(PARTITION BY d_moy ORDER BY d_fy_week_seq) FROM tpcds.date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1,2;

-- [DQL]
SELECT d_moy , d_fy_week_seq , first_value ( d_fy_week_seq ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- [DQL]
SELECT d_moy , d_fy_week_seq , last_value ( d_moy ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 6 ORDER BY 1 , 2 ;

-- [DQL]
SELECT sales_group , sales_id , sales_amount , RATIO_TO_REPORT ( sales_amount ) OVER ( PARTITION BY sales_group ) FROM sales_int8 ORDER BY sales_id ;

-- [DQL]
SELECT sales_group , sales_id , sales_amount , TO_CHAR ( RATIO_TO_REPORT ( sales_amount ) OVER (), '$999eeee' ) FROM sales ORDER BY sales_id ;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc IS CURSOR cur_1 IS SELECT RATIO_TO_REPORT ( sales_amount ) OVER () FROM sales_numeric ;

-- [PLSQL]
CALL PROC ();

-- [DQL]
SELECT d_moy , d_fy_week_seq , nth_value ( d_fy_week_seq , 6 ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 6 ORDER BY 1 , 2 ;


================================================================================
-- 来源: 2786_file_2786.txt
================================================================================

-- [DQL]
SELECT gs_encrypt_aes128 ( 'MPPDB' , 'Asdf1234' );

-- [DQL]
SELECT gs_encrypt('MPPDB', 'Asdf1234', 'sm4');

-- [DQL]
SELECT gs_encrypt_bytea('MPPDB', 'Asdf1234', 'sm4_ctr_sm3');

-- [DQL]
SELECT gs_decrypt_aes128 ( 'gwditQLQG8NhFw4OuoKhhQJoXojhFlYkjeG0aYdSCtLCnIUgkNwvYI04KbuhmcGZp8jWizBdR1vU9CspjuzI0lbz12A=' , '1234' );

-- [DQL]
select gs_decrypt('ZBzOmaGA4Bb+coyucJ0B8AkIShqc', 'Asdf1234', 'sm4');

-- [DQL]
select gs_decrypt_bytea('\x90e286971c2c70410def0a2814af4ac44c737926458b66271d9d1547bc937395ca018d7755672fa9dc3cdc6ec4a76001dc0e137f3bc5c8a5c51143561f1d09a848bfdebfec5e', 'Asdf1234', 'sm4_ctr_sm3');

-- [DQL]
select aes_encrypt('huwei123','123456vfhex4dyu,vdaladhjsadad','1234567890123456');

-- [DQL]
select aes_decrypt(aes_encrypt('huwei123','123456vfhex4dyu,vdaladhjsadad','1234567890123456'),'123456vfhex4dyu,vdaladhjsadad','1234567890123456');

-- [DQL]
SELECT pg_catalog . gs_digest ( 'gaussdb' , 'sha256' );

-- [DQL]
SELECT gs_password_deadline ();

-- [DQL]
SELECT inet_server_addr ();

-- [DQL]
SELECT inet_client_addr ();


================================================================================
-- 来源: 2788_file_2788.txt
================================================================================

-- [DQL]
SELECT * FROM generate_series ( 2 , 4 );

-- [DQL]
SELECT * FROM generate_series ( 5 , 1 , - 2 );

-- [DQL]
SELECT * FROM generate_series ( 4 , 3 );

-- [DQL]
SELECT current_date + s . a AS dates FROM generate_series ( 0 , 14 , 7 ) AS s ( a );

-- [DQL]
SELECT * FROM generate_series ( '2008-03-01 00:00' :: timestamp , '2008-03-04 12:00' , '10 hours' );

-- [DQL]
SELECT generate_subscripts ( '{NULL,1,NULL,2}' :: int [], 1 ) AS s ;

-- [DDL]
CREATE OR REPLACE FUNCTION unnest2 ( anyarray ) RETURNS SETOF anyelement AS $$ SELECT $ 1 [ i ][ j ] FROM generate_subscripts ( $ 1 , 1 ) g1 ( i ), generate_subscripts ( $ 1 , 2 ) g2 ( j );

-- [DQL]
SELECT * FROM unnest2 ( ARRAY [[ 1 , 2 ],[ 3 , 4 ]]);

-- [DDL]
DROP FUNCTION unnest2 ;


================================================================================
-- 来源: 2789_file_2789.txt
================================================================================

-- [DQL]
SELECT coalesce ( NULL , 'hello' );

-- [DQL]
SELECT decode ( 'A' , 'A' , 1 , 'B' , 2 , 0 );

-- [DQL]
SELECT nullif ( 'hello' , 'world' );

-- [DQL]
SELECT nullif ( '1234' :: VARCHAR , 123 :: INT4 );

-- [DQL]
SELECT nullif ( '1234' :: VARCHAR , '2012-12-24' :: DATE );

-- [DQL]
SELECT nullif ( 1 :: bit , '1' :: MONEY );

-- [DQL]
SELECT nvl ( 'hello' , 'world' );

-- [DQL]
SELECT nvl2 ( 'hello' , 'world' , 'other' );

-- [DQL]
SELECT greatest ( 1 * 2 , 2 - 3 , 4 - 1 );

-- [DQL]
SELECT greatest ( 'HARRY' , 'HARRIOT' , 'HAROLD' );

-- [DQL]
SELECT least ( 1 * 2 , 2 - 3 , 4 - 1 );

-- [DQL]
SELECT least ( 'HARRY' , 'HARRIOT' , 'HAROLD' );

-- [DDL]
CREATE TABLE blob_tb ( b blob , id int );

-- [DML_INSERT]
INSERT INTO blob_tb VALUES ( empty_blob (), 1 );

-- [DDL]
DROP TABLE blob_tb ;

-- [DDL]
CREATE TABLE clob_tb ( c clob , id int );

-- [DML_INSERT]
INSERT INTO clob_tb VALUES ( empty_clob (), 1 );

-- [DDL]
DROP TABLE clob_tb ;

-- [DDL]
CREATE TABLE student_demo ( name VARCHAR2 ( 20 ), grade NUMBER ( 10 , 2 ));

-- [DML_INSERT]
INSERT INTO student_demo VALUES ( 'name0' , 0 );

-- [DML_INSERT]
INSERT INTO student_demo VALUES ( 'name1' , 1 );

-- [DML_INSERT]
INSERT INTO student_demo VALUES ( 'name2' , 2 );

-- [DQL]
SELECT * FROM student_demo WHERE LNNVL ( name = 'name1' );

-- [DQL]
SELECT isnull ( null );

-- [DQL]
SELECT isnull ( 1 );

-- [DQL]
select if ( 2 > 3 , 'true' , 'false' );

-- [DQL]
select if ( null , 'not null' , 'is null' );

-- [DQL]
select ifnull ( '' , null ) is null as a ;

-- [DQL]
select ifnull ( null , null ) is null as a ;

-- [DQL]
select ifnull ( null , 'A' ) as a ;


================================================================================
-- 来源: 2790_file_2790.txt
================================================================================

-- [DQL]
SELECT current_query ();

-- [DQL]
SELECT current_schema ();

-- [DQL]
SELECT current_schemas ( true );

-- [DQL]
SELECT database ();

-- [DQL]
SELECT current_user ;

-- [DQL]
SELECT definer_current_user ();

-- [DQL]
SELECT pg_current_sessionid ();

-- [DQL]
select pg_current_sessid();

-- [DQL]
SELECT pg_current_userid();

-- [DQL]
SELECT working_version_num ();

-- [DQL]
select tablespace_oid_name ( 1663 );

-- [DQL]
SELECT inet_client_addr ();

-- [DQL]
SELECT inet_client_port ();

-- [DQL]
SELECT inet_server_addr ();

-- [DQL]
SELECT inet_server_port ();

-- [DQL]
SELECT pg_backend_pid ();

-- [DQL]
SELECT pg_conf_load_time ();

-- [DQL]
SELECT pg_my_temp_schema ();

-- [DQL]
SELECT pg_is_other_temp_schema ( 25356 );

-- [DQL]
SELECT pg_listening_channels ();

-- [DQL]
SELECT pg_postmaster_start_time ();

-- [DQL]
select * from pg_get_ruledef(24828);

-- [DQL]
select sessionid2pid ( sessid :: cstring ) from gs_session_stat limit 2 ;

-- [DQL]
SELECT session_context ( 'USERENV' , 'CURRENT_SCHEMA' );

-- [DQL]
SELECT pg_trigger_depth ();

-- [DQL]
SELECT session_user ;

-- [DQL]
SELECT user ;

-- [DQL]
select getpgusername ();

-- [DQL]
select getdatabaseencoding ();

-- [DQL]
select version();

-- [DQL]
select opengauss_version ();

-- [DQL]
select gs_deployment ();

-- [DQL]
SELECT get_hostname ();

-- [DQL]
SELECT get_nodename ();

-- [DQL]
SELECT get_nodeinfo ( 'node_type' );

-- [DQL]
SELECT get_nodeinfo ( 'node_name' );

-- [DQL]
SELECT get_schema_oid ( 'public' );

-- [DQL]
SELECT has_table_privilege ( 'tpcds.web_site' , 'select' );

-- [DQL]
SELECT has_table_privilege ( 'omm' , 'tpcds.web_site' , 'select,INSERT WITH GRANT OPTION ' );

-- [DQL]
SELECT relname FROM pg_class WHERE pg_table_is_visible ( oid );

-- [DQL]
SELECT format_type (( SELECT oid FROM pg_type WHERE typname = 'varchar' ), 10 );

-- [DQL]
select pg_check_authid(1);

-- [DQL]
select * from pg_get_functiondef(598);

-- [DQL]
select * from pg_get_indexdef(16416);

-- [DDL]
CREATE TABLE sales (prod_id NUMBER(6), cust_id NUMBER, time_id DATE, channel_id CHAR(1), promo_id NUMBER(6), quantity_sold NUMBER(3), amount_sold NUMBER(10,2) ) PARTITION BY RANGE( time_id) INTERVAL('1 day') ( partition p1 VALUES LESS THAN ('2019-02-01 00:00:00'), partition p2 VALUES LESS THAN ('2019-02-02 00:00:00') );

-- [DDL]
create index index_sales on sales(prod_id) local (PARTITION idx_p1 ,PARTITION idx_p2);

-- [DML_INSERT]
INSERT INTO sales VALUES(1, 12, '2019-02-05 00:00:00', 'a', 1, 1, 1);

-- [DQL]
select oid from pg_class where relname = 'index_sales';

-- [DQL]
select * from pg_get_indexdef(24632, true);

-- [DQL]
select * from pg_get_indexdef(24632, false);

-- [DQL]
select * from pg_get_indexdef(16416, 0, false);

-- [DQL]
select * from pg_get_indexdef(16416, 1, false);

-- [DQL]
select pg_check_authid(20);

-- [DQL]
select * from pg_get_tabledef(16384);

-- [DQL]
select * from pg_get_tabledef('t1');

-- [DQL]
SELECT pg_typeof ( 33 );

-- [DQL]
SELECT typlen FROM pg_type WHERE oid = pg_typeof ( 33 );

-- [DQL]
SELECT collation for ( description ) FROM pg_description LIMIT 1 ;

-- [DQL]
select * from pg_get_serial_sequence('t1', 'c1');

-- [DQL]
select * from pg_sequence_parameters(16420);

-- [DQL]
select * from gs_get_kernel_info();


================================================================================
-- 来源: 2792_file_2792.txt
================================================================================

-- [DQL]
SELECT current_setting ( 'datestyle' );

-- [DQL]
SELECT set_config ( 'log_statement_stats' , 'off' , false );


================================================================================
-- 来源: 2793_file_2793.txt
================================================================================

-- [DQL]
SELECT pg_ls_dir ( './' );

-- [DQL]
SELECT pg_read_file ( 'postmaster.pid' , 0 , 100 );

-- [DQL]
SELECT convert_from ( pg_read_binary_file ( 'filename' ), 'UTF8' );

-- [DQL]
SELECT * FROM pg_stat_file ( 'filename' );

-- [DQL]
SELECT ( pg_stat_file ( 'filename' )). modification ;

-- [DQL]
SELECT convert_from ( pg_read_binary_file ( 'postmaster.pid' ), 'UTF8' );

-- [DQL]
SELECT * FROM pg_stat_file ( 'postmaster.pid' );

-- [DQL]
SELECT ( pg_stat_file ( 'postmaster.pid' )). modification ;


================================================================================
-- 来源: 2794_file_2794.txt
================================================================================

-- [DQL]
SELECT pid from pg_stat_activity ;

-- [DQL]
SELECT pg_terminate_backend ( 140657876268816 );


================================================================================
-- 来源: 2795_file_2795.txt
================================================================================

-- [DQL]
SELECT pg_start_backup ( 'label_goes_here' );

-- [DQL]
SELECT * FROM pg_xlogfile_name_offset ( pg_stop_backup ());


================================================================================
-- 来源: 2796_file_2796.txt
================================================================================

-- [DQL]
select * from pg_create_physical_replication_slot_extern ( 'uuid' , false , 'OBS;

-- [DQL]
select * from pg_create_physical_replication_slot_extern ( 'uuid' , false , 'NAS;

-- [DQL]
select gs_set_obs_delete_location('0/54000000');


================================================================================
-- 来源: 2799_file_2799.txt
================================================================================

-- [DQL]
SELECT pg_column_size ( 1 );

-- [DQL]
SELECT pg_database_size ( 'testdb' );

-- [MAINTENANCE]
analyze ;

-- [DQL]
select get_db_source_datasize ();

-- [DQL]
SELECT datalength(1);


================================================================================
-- 来源: 2801_file_2801.txt
================================================================================

-- [DQL]
select * from pg_create_logical_replication_slot('slot_lsn','mppdb_decoding',0);

-- [DQL]
select * from pg_create_logical_replication_slot('slot_csn','mppdb_decoding',1);

-- [DQL]
select * from pg_logical_slot_peek_changes('slot_lsn',NULL,4096,'skip-empty-xacts','on');

-- [DQL]
select * from pg_logical_slot_peek_changes('slot_csn',NULL,4096,'skip-empty-xacts','on');

-- [DDL]
CREATE TABLE t1(a int, b int);

-- [DQL]
SELECT pg_current_xlog_location();

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,1);

-- [DML_UPDATE]
UPDATE t1 SET b = 2 WHERE a = 1;

-- [DML_DELETE]
DELETE FROM t1;

-- [DQL]
SELECT * FROM pg_logical_get_area_changes('0/5ECBCD48', NULL, NULL, 'sql_decoding', NULL);

-- [DDL]
CREATE TABLE t2(a int, b int GENERATED ALWAYS AS (a + 1) STORED);

-- [DQL]
SELECT pg_current_xlog_location();

-- [DML_INSERT]
INSERT INTO t2(a) VALUES(1);

-- [DML_UPDATE]
UPDATE t2 set a = 2 where a = 1;

-- [DML_DELETE]
DELETE FROM t2;

-- [DQL]
SELECT * FROM pg_logical_get_area_changes('0/5F62CFE8', NULL, NULL, 'sql_decoding', NULL, 'skip-generated-columns', 'on');

-- [DQL]
select * from pg_get_replication_slots();

-- [DQL]
select * from gs_get_parallel_decode_status();

-- [DQL]
select * from gs_get_slot_decoded_wal_time('replication_slot');

-- [DQL]
select * from gs_logical_parallel_decode_status('replication_slot');

-- [DQL]
select * from gs_logical_parallel_decode_status('replication_slot');

-- [DQL]
select * from gs_logical_parallel_decode_reset_status('replication_slot');

-- [DQL]
select * from gs_logical_parallel_decode_status('replication_slot');

-- [DQL]
select * from gs_logical_decode_start_observe('replication_slot',20,5);

-- [DQL]
select * from gs_logical_decode_start_observe('replication_slot',20,5);

-- [DQL]
select * from gs_logical_decode_stop_observe('replication_slot');

-- [DQL]
select * from gs_logical_decode_stop_observe('replication_slot');

-- [DQL]
select * from gs_logical_decode_observe_data('replication_slot');

-- [DQL]
select * from gs_logical_decode_observe('replication_slot');

-- [DQL]
select * from gs_logical_decode_observe_status('replication_slot');

-- [DQL]
select * from gs_logical_decode_observe_status('replication_slo');

-- [DQL]
select * from gs_logical_decode_stop_observe('replication_slot');

-- [DQL]
select * from gs_logical_decode_observe_status('replication_slot');

-- [DQL]
select * from gs_get_parallel_decode_thread_info();


================================================================================
-- 来源: 2802_file_2802.txt
================================================================================

-- [DQL]
select * from gs_seg_dump_page('pg_default', 1, 1024, 4157);

-- [DQL]
select * from gs_seg_dump_page(16788, 1024, 0);

-- [DQL]
select * from gs_seg_get_spc_location('pg_default', 1024, 4157, 0);

-- [DQL]
select * from gs_seg_get_spc_location(24578,1024,0);

-- [DQL]
select * from gs_seg_get_location(4157);

-- [DQL]
select * from gs_seg_get_segment_layout();

-- [DQL]
select * from gs_seg_get_datafile_layout();

-- [DQL]
select * from gs_seg_get_slice_layout(1,1024, 0);

-- [DQL]
select * from gs_seg_get_segment('pg_default', 1024, 4157);

-- [DQL]
select * from gs_seg_get_segment(16768, 1024);

-- [DQL]
select * from gs_seg_get_extents('pg_default', 1024, 4157);

-- [DQL]
select * from gs_seg_get_extents(16768, 1024);

-- [DQL]
select * from gs_seg_free_spc_remain_segment('pg_default', 1, 4159);

-- [DQL]
select * from gs_seg_free_spc_remain_extent('pg_default', 1, 0, 4159);

-- [DQL]
select * from gs_seg_get_datafiles();

-- [DQL]
select * from gs_seg_get_spc_extents('pg_default', 1,1024, 0);


================================================================================
-- 来源: 2804_file_2804.txt
================================================================================

-- [DQL]
select pg_stat_get_env();

-- [DQL]
select gs_parse_page_bypath('base/16603/16394', -1, 'btree', false);

-- [DQL]
select gs_parse_page_bypath('base/12828/16771_vm', -1, 'vm', false);

-- [DQL]
select gs_parse_page_bypath('000000000000', 0, 'clog', false);

-- [DQL]
select gs_parse_page_bypath('base/12828/16777', -10, 'heap', false);

-- [DQL]
select * from gs_stat_space(false);

-- [DQL]
select * from gs_index_dump_read(0, 'all');

-- [DQL]
select * from gs_index_dump_read(1, 'all');

-- [DQL]
select * from gs_parse_page_bypath('base/15833/16768', 0, 'uheap', false);

-- [DQL]
select * from gs_xlogdump_bylastlsn('0/4593570', -1, 'uheap');

-- [DQL]
select * from gs_xlogdump_bylastlsn('0/4593570', 0, 'ubtree');

-- [DDL]
CREATE TABLE test(a int,b int);

-- [DML_INSERT]
INSERT INTO test values(1,1);

-- [DDL]
CREATE PROCEDURE mypro1() as num int;

-- [SESSION]
SET instr_unique_sql_track_type = 'all';

-- [SESSION]
SET track_stmt_stat_level = 'L0,L0';

-- [PLSQL]
CALL mypro1();

-- [SESSION]
SET track_stmt_stat_level = 'off,L0';

-- [SESSION]
SET instr_unique_sql_track_type = 'top';

-- [DQL]
SELECT query,unique_query_id,start_time,finish_time FROM dbe_perf.statement_history;

-- [DQL]
SELECT query FROM dbe_perf.get_full_sql_by_parent_id_and_timestamp(536458473,'2023-06-02 17:40:59.028144+08','2023-06-02 17:40:59.032027+08');


================================================================================
-- 来源: 2805_Undo.txt
================================================================================

-- [DQL]
select * from gs_global_config where name like '%undostoragetype%';

-- [DQL]
select * from gs_stat_undo(true);

-- [DQL]
select * from gs_stat_undo(false);

-- [DQL]
select * from gs_undo_meta_dump_zone(-1,true);

-- [DQL]
select * from gs_undo_translot_dump_slot(-1,true);

-- [DQL]
select * from gs_undo_translot_dump_xid('15758',false);

-- [DQL]
select * from gs_undo_dump_record('0000000000000042');

-- [DQL]
select * from gs_undo_dump_xid('15779');

-- [DQL]
select * from gs_verify_undo_record('urp', 24, 24, 1);

-- [DQL]
select * from gs_verify_undo_record('zone', 0, 2, 1);

-- [DQL]
select * from gs_verify_undo_slot('zone', 0, 2, 1);

-- [DQL]
select * from gs_verify_undo_meta('all', 0, 2, 1);


================================================================================
-- 来源: 2808_file_2808.txt
================================================================================

-- [DQL]
select pg_stat_get_role_name(10);

-- [DQL]
select * from pg_stat_get_activity(139881386280704);

-- [DQL]
select * from gs_stack ( 139663481165568 );

-- [DQL]
select * from gs_stack ();

-- [DQL]
SELECT * FROM gs_perf_start ( 10 , 100 );

-- [DQL]
SELECT * FROM gs_perf_query () WHERE overhead > 2 AND level < 10 ;

-- [DQL]
SELECT * FROM gs_perf_clean ();

-- [DQL]
select sessionid from pg_stat_activity where usename = 'testuser';

-- [DQL]
select * from gs_session_all_settings(788861) where name = 'work_mem';

-- [DQL]
select * from gs_session_all_settings() where name = 'work_mem';

-- [DQL]
select * from gs_local_wal_preparse_statistics();

-- [DQL]
SELECT * FROM GS_WLM_RESPOOL_CPU_INFO ();

-- [DQL]
SELECT * FROM GS_WLM_RESPOOL_CONNECTION_INFO ();

-- [DQL]
SELECT * FROM GS_WLM_RESPOOL_MEMORY_INFO ();

-- [DQL]
SELECT * FROM GS_WLM_RESPOOL_CONCURRENCY_INFO();

-- [DQL]
SELECT * FROM GS_WLM_RESPOOL_IO_INFO();

-- [DQL]
SELECT * FROM GS_WLM_USER_SPACE_INFO ();

-- [DQL]
SELECT * FROM GS_WLM_SESSION_IO_INFO ();

-- [DQL]
SELECT * FROM GS_WLM_SESSION_MEMORY_INFO ();

-- [DQL]
select * from gs_hot_standby_space_info();

-- [DQL]
SELECT * FROM exrto_file_read_stat();

-- [DQL]
SELECT * FROM gs_exrto_recycle_info();

-- [DQL]
SELECT * FROM gs_stat_get_db_conflict_all(12738);

-- [DQL]
SELECT * FROM gs_redo_stat_info();

-- [DQL]
SELECT * FROM gs_recovery_conflict_waitevent_info();

-- [DQL]
SELECT * FROM gs_display_delay_ddl_info();

-- [DDL]
CREATE TABLE part_tab 1 ( a int, b int ) PARTITION BY RANGE(b) ( PARTITION P1 VALUES LESS THAN(10), PARTITION P2 VALUES LESS THAN(20), PARTITION P3 VALUES LESS THAN(MAXVALUE) );

-- [DDL]
CREATE TABLE subpart_tab 1 ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY RANGE (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES LESS THAN( '2' ), SUBPARTITION p_201901_b VALUES LESS THAN( '3' ) ), PARTITION p_201902 VALUES LESS THAN( '201904' ) ( SUBPARTITION p_201902_a VALUES LESS THAN( '2' ), SUBPARTITION p_201902_b VALUES LESS THAN( '3' ) ) );

-- [DDL]
CREATE INDEX index_part_tab1 ON part_tab1(b) LOCAL ( PARTITION b_index1, PARTITION b_index2, PARTITION b_index 3 );

-- [DDL]
CREATE INDEX idx_user_no ON subpart_tab1(user_no) LOCAL;

-- [DML_INSERT]
INSERT INTO part_tab1 VALUES(1, 1);

-- [DML_INSERT]
INSERT INTO part_tab1 VALUES(1, 11);

-- [DML_INSERT]
INSERT INTO part_tab1 VALUES(1, 21);

-- [DML_UPDATE]
UPDATE part_tab1 SET a = 2 WHERE b = 1;

-- [DML_UPDATE]
UPDATE part_tab1 SET a = 3 WHERE b = 11;

-- [DML_UPDATE]
UPDATE /*+ indexscan(part_tab1) */ part_tab1 SET a = 4 WHERE b = 21;

-- [DML_DELETE]
DELETE FROM part_tab1;

-- [MAINTENANCE]
ANALYZE part_tab1;

-- [MAINTENANCE]
VACUUM part_tab1;

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201902', '1', '1', 1);

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201902', '2', '2', 1);

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201903', '1', '3', 1);

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201903', '2', '4', 1);

-- [DML_UPDATE]
UPDATE subpart_tab1 SET sales_amt = 2 WHERE user_no='1';

-- [DML_UPDATE]
UPDATE subpart_tab1 SET sales_amt = 3 WHERE user_no='2';

-- [DML_UPDATE]
UPDATE subpart_tab1 SET sales_amt = 4 WHERE user_no='3';

-- [DML_UPDATE]
UPDATE /*+ indexscan(subpart_tab1) */ subpart_tab1 SET sales_amt = 5 WHERE user_no='4';

-- [DML_DELETE]
DELETE FROM subpart_tab1;

-- [MAINTENANCE]
ANALYZE subpart_tab1;

-- [MAINTENANCE]
VACUUM subpart_tab1;

-- [DQL]
SELECT * FROM gs_stat_all_partitions;

-- [DQL]
SELECT * FROM gs_statio_all_partitions;

-- [DQL]
SELECT * FROM gs_stat_get_partition_stats(16952);

-- [TCL]
BEGIN;

-- [DML_INSERT]
INSERT INTO part_tab1 VALUES(1, 1);

-- [DML_INSERT]
INSERT INTO part_tab1 VALUES(1, 11);

-- [DML_INSERT]
INSERT INTO part_tab1 VALUES(1, 21);

-- [DML_UPDATE]
UPDATE part_tab1 SET a = 2 WHERE b = 1;

-- [DML_UPDATE]
UPDATE part_tab1 SET a = 3 WHERE b = 11;

-- [DML_UPDATE]
UPDATE /*+ indexscan(part_tab1) */ part_tab1 SET a = 4 WHERE b = 21;

-- [DML_DELETE]
DELETE FROM part_tab1;

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201902', '1', '1', 1);

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201902', '2', '2', 1);

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201903', '1', '3', 1);

-- [DML_INSERT]
INSERT INTO subpart_tab1 VALUES('201903', '2', '4', 1);

-- [DML_UPDATE]
UPDATE subpart_tab1 SET sales_amt = 2 WHERE user_no='1';

-- [DML_UPDATE]
UPDATE subpart_tab1 SET sales_amt = 3 WHERE user_no='2';

-- [DML_UPDATE]
UPDATE subpart_tab1 SET sales_amt = 4 WHERE user_no='3';

-- [DML_UPDATE]
UPDATE /*+ indexscan(subpart_tab1) */ subpart_tab1 SET sales_amt = 5 WHERE user_no='4';

-- [DML_DELETE]
DELETE FROM subpart_tab1;

-- [DQL]
SELECT * FROM gs_stat_xact_all_partitions;

-- [DQL]
SELECT * FROM gs_stat_get_xact_partition_stats(16952);


================================================================================
-- 来源: 2809_file_2809.txt
================================================================================

-- [DQL]
select pg_get_triggerdef(oid) from pg_trigger;

-- [DQL]
select pg_get_triggerdef(oid,true) from pg_trigger;

-- [DQL]
select pg_get_triggerdef(oid,false) from pg_trigger;


================================================================================
-- 来源: 2810_HashFunc.txt
================================================================================

-- [DQL]
select ora_hash ( 123 );

-- [DQL]
select ora_hash ( '123' );

-- [DQL]
select ora_hash ( 'sample' );

-- [DQL]
select ora_hash ( to_date ( '2012-1-2' , 'yyyy-mm-dd' ));

-- [DQL]
select ora_hash ( 123 , 234 );

-- [DQL]
select ora_hash ( '123' , 234 );

-- [DQL]
select ora_hash ( 'sample' , 234 );

-- [DQL]
select ora_hash ( to_date ( '2012-1-2' , 'yyyy-mm-dd' ), 234 );

-- [DQL]
select hash_array ( ARRAY [[ 1 , 2 , 3 ],[ 1 , 2 , 3 ]]);

-- [DQL]
select hash_numeric ( 30 );

-- [DQL]
select hash_range ( numrange ( 1 . 1 , 2 . 2 ));

-- [DQL]
select hashbpchar ( 'hello' );

-- [DQL]
select hashbpchar ( 'hello' );

-- [DQL]
select hashchar ( 'true' );

-- [DDL]
CREATE TYPE b1 AS ENUM ( 'good' , 'bad' , 'ugly' );

-- [PLSQL]
call hashenum ( 'good' :: b1 );

-- [DQL]
select hashfloat4 ( 12 . 1234 );

-- [DQL]
select hashfloat8 ( 123456 . 1234 );

-- [DQL]
select hashinet ( '127.0.0.1' :: inet );

-- [DQL]
select hashint1 ( 20 );

-- [DQL]
select hashint2(20000);


================================================================================
-- 来源: 2812_file_2812.txt
================================================================================

-- [DQL]
select * from pg_get_gtt_relstats(74069);

-- [DQL]
select * from pg_get_gtt_statistics(74069,1,''::text);

-- [DQL]
select * from pg_gtt_attached_pid(74069);

-- [DQL]
select * from pg_list_gtt_relfrozenxids();


================================================================================
-- 来源: 2816_file_2816.txt
================================================================================

-- [DQL]
select * , sys_connect_by_path ( name , '-' ) from connect_table start with id = 1 connect by prior id = pid ;

-- [DQL]
select * , connect_by_root ( name ) from connect_table start with id = 1 connect by prior id = pid ;


================================================================================
-- 来源: 2819_Global SysCache.txt
================================================================================

-- [DQL]
select * from gs_gsc_catalog_detail(16574, 1260);

-- [DQL]
select * from gs_gsc_clean();

-- [DQL]
select * from gs_gsc_dbstat_info();


================================================================================
-- 来源: 2820_file_2820.txt
================================================================================

-- [DQL]
select * from gs_verify_data_file();

-- [DQL]
select * from gs_verify_data_file(true);

-- [DQL]
select * from gs_repair_file(16554,'base/16552/24745',360);

-- [DQL]
select * from local_bad_block_info();

-- [DQL]
select * from local_clear_bad_block_info();

-- [DQL]
select * from gs_verify_and_tryrepair_page('base/16552/24745',0,false,false);

-- [DQL]
select * from gs_repair_page('base/16552/24745',0,false,60);

-- [DQL]
select gs_edit_page_bypath('base/15808/25075',0,16,'0x1FFF', 2, false, 'page');

-- [DQL]
select gs_edit_page_bypath('base/15808/25075', 0,16,'@1231!', 8, false, 'page');

-- [DQL]
select gs_edit_page_bypath('/pg_log_dir/dump/1663_15808_25075_0.editpage', 0,16,'0x1FFF', 2, true, 'page');

-- [DQL]
select * from gs_repair_page_bypath('pg_log/dump/1663_15991_16767_0.editpage', 0, 'base/15991/16767', 0, 'page');

-- [DQL]
select * from gs_repair_page_bypath('standby', 0, 'base/15990/16768', 0, 'page');

-- [DQL]
select * from gs_repair_page_bypath('init_block', 0, 'base/15990/16768', 0, 'page');

-- [DQL]
select * from gs_repair_undo_byzone(4);

-- [DQL]
select * from gs_repair_undo_byzone(78);

-- [DQL]
select * from gs_repair_undo_byzone(0);

-- [DQL]
select * from gs_verify_urq(16387, 0, 1, 'free queue');

-- [DQL]
select * from gs_verify_urq(16387, 0, 1, 'empty queue');

-- [DQL]
SELECT * FROM gs_urq_dump_stat(16387, 0);

-- [DQL]
SELECT gs_urq_dump_stat(17260,0);

-- [DQL]
select * from gs_repair_urq(16387, 0);

-- [DQL]
select * from gs_get_standby_bad_block_info();


================================================================================
-- 来源: 2821_XML.txt
================================================================================

-- [DQL]
SELECT XMLPARSE ( DOCUMENT '<?xml version="1.0"?><book><title>Manual</title><chapter>...</chapter></book>' );

-- [DQL]
SELECT XMLPARSE ( CONTENT 'abc<foo>bar</foo><bar>foo</bar>' );

-- [DQL]
SELECT XMLPARSE ( CONTENT 'abc<foo>bar</foo' wellformed );

-- [DQL]
SELECT XMLSERIALIZE ( CONTENT 'good' AS CHAR ( 10 ));

-- [DQL]
SELECT xmlserialize ( DOCUMENT '<head>bad</head>' as text );

-- [DQL]
SELECT xmlcomment ( 'hello' );

-- [SESSION]
set xmloption=content;

-- [DQL]
select XMLCONCAT(('<?xml version="1.0" encoding="GB2312" standalone="no"?><bar>foo</bar>'),('<?xml version="1.0" encoding="GB2312" standalone="no" ?><bar>foo</bar>')) ;

-- [DQL]
select XMLCONCAT('abc>');

-- [SESSION]
set a_format_version='10c';

-- [SESSION]
set a_format_dev_version=s2;

-- [SESSION]
set xmloption=content;

-- [DQL]
select XMLCONCAT(('<?xml version="1.0" encoding="GB2312" standalone="no"?><bar>foo</bar>'),('<?xml version="1.0" encoding="GB2312" standalone="no" ?><bar>foo</bar>')) ;

-- [DQL]
select XMLCONCAT('abc>');

-- [DQL]
SELECT xmlelement ( name foo );

-- [DQL]
SELECT xmlelement ( "entityescaping<>" , 'a$><&"b' );

-- [DQL]
SELECT xmlelement ( entityescaping "entityescaping<>" , 'a$><&"b' );

-- [DQL]
SELECT xmlelement ( noentityescaping "entityescaping<>" , 'a$><&"b' );

-- [DQL]
SELECT xmlelement(" entityescaping <> ", '<abc/>' b);

-- [DQL]
SELECT xmlelement(" entityescaping <> ", '<abc/>' as b);

-- [DQL]
SELECT xmlelement(" entityescaping <> ", xml('<abc/>') b);

-- [DQL]
SELECT xmlelement(" entityescaping <> ", xml('<abc/>') as b);

-- [DQL]
SELECT xmlelement(" entityescaping <> ", xmlattributes('entityescaping<>' " entityescaping <> "));

-- [DQL]
SELECT xmlelement(name " entityescaping <> ", xmlattributes(entityescaping 'entityescaping<>' " entityescaping <> "));

-- [DQL]
SELECT xmlelement(" entityescaping <> ", xmlattributes(noentityescaping 'entityescaping<>' " entityescaping <> "));

-- [DQL]
SELECT xmlforest ( 'abc' AS foo , 123 AS bar );

-- [DQL]
SELECT xmlpi ( name php , 'echo "hello world";

-- [DQL]
SELECT xmlroot ( '<?xml version="1.1"?><content>abc</content>' , version '1.0' , standalone yes );

-- [DDL]
CREATE TABLE xmltest ( id int , data xml );

-- [DML_INSERT]
INSERT INTO xmltest VALUES ( 1 , '<value>one</value>' );

-- [DML_INSERT]
INSERT INTO xmltest VALUES ( 2 , '<value>two</value>' );

-- [DQL]
SELECT xmlagg ( data ) FROM xmltest ;

-- [SESSION]
set xmloption = document ;

-- [DQL]
SELECT xmlagg ( data ) FROM xmltest ;

-- [DML_DELETE]
DELETE FROM XMLTEST ;

-- [DML_INSERT]
INSERT INTO xmltest VALUES ( 1 , '<?xml version="1.0" encoding="GBK"?><value>one</value>' );

-- [DML_INSERT]
INSERT INTO xmltest VALUES ( 2 , '<?xml version="1.0" encoding="GBK"?><value>two</value>' );

-- [DQL]
SELECT xmlagg ( data ) FROM xmltest ;

-- [DQL]
SELECT xmlagg ( data order by id desc ) FROM xmltest ;

-- [DQL]
SELECT xmlexists ( '//town[text() = ''Toronto'']' PASSING BY REF '<towns><town>Toronto</town><town>Ottawa</town></towns>' );

-- [DQL]
SELECT xml_is_well_formed ( '<>' );

-- [DQL]
SELECT xml_is_well_formed_document ( '<pg:foo xmlns:pg="http://postgresql.org/stuff">bar</pg:foo>' );

-- [DQL]
select xml_is_well_formed_content ( 'k' );

-- [DQL]
SELECT xpath ( '/my:a/text()' , '<my:a xmlns:my="http://example.com">test</my:a>' , ARRAY [ ARRAY [ 'my' , 'http://example.com' ]]);

-- [DQL]
SELECT xpath_exists ( '/my:a/text()' , '<my:a xmlns:my="http://example.com">test</my:a>' , ARRAY [ ARRAY [ 'my' , 'http://example.com' ]]);

-- [DDL]
CREATE SCHEMA testxmlschema ;

-- [DDL]
CREATE TABLE testxmlschema . test1 ( a int , b text );

-- [DML_INSERT]
INSERT INTO testxmlschema . test1 VALUES ( 1 , 'one' ), ( 2 , 'two' ), ( - 1 , null );

-- [DDL]
create database test ;

-- [DQL]
SELECT query_to_xml ( 'SELECT * FROM testxmlschema.test1' , false , false , '' );

-- [DQL]
SELECT query_to_xmlschema ( 'SELECT * FROM testxmlschema.test1' , false , false , '' );

-- [DQL]
SELECT query_to_xml_and_xmlschema ( 'SELECT * FROM testxmlschema.test1' , true , true , '' );

-- [OTHER]
CURSOR xc WITH HOLD FOR SELECT * FROM testxmlschema . test1 ORDER BY 1 , 2 ;

-- [DQL]
SELECT cursor_to_xml ( 'xc' :: refcursor , 5 , false , true , '' );

-- [DQL]
SELECT cursor_to_xmlschema ( 'xc' :: refcursor , true , false , '' );

-- [DQL]
SELECT schema_to_xml ( 'testxmlschema' , false , true , '' );

-- [DQL]
SELECT schema_to_xmlschema ( 'testxmlschema' , false , true , '' );

-- [DQL]
SELECT schema_to_xml_and_xmlschema ( 'testxmlschema' , true , true , 'foo' );

-- [DQL]
SELECT database_to_xml ( true , true , 'test' );

-- [DQL]
SELECT database_to_xmlschema ( true , true , 'test' );

-- [DQL]
SELECT database_to_xml_and_xmlschema ( true , true , 'test' );

-- [DQL]
SELECT table_to_xml ( 'testxmlschema.test1' , false , false , '' );

-- [DQL]
SELECT table_to_xmlschema ( 'testxmlschema.test1' , false , false , '' );

-- [DQL]
SELECT table_to_xml_and_xmlschema ( 'testxmlschema.test1' , false , false , '' );

-- [SESSION]
SET a_format_version = '10c' ;

-- [SESSION]
SET a_format_dev_version = 's4' ;

-- [PLSQL]
DECLARE xmldata xml ;

-- [DQL]
SELECT getclobval ( xmlparse ( document '<a>123</a>' ));

-- [SESSION]
SET a_format_version='10c';

-- [SESSION]
SET a_format_dev_version='s4';

-- [PLSQL]
DECLARE xmldata xml;

-- [DQL]
SELECT getstringval(xmlparse(document '<a>123<b>456</b></a>'));

-- [DQL]
SELECT xmlsequence(xml('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>'));


================================================================================
-- 来源: 2822_XMLTYPE.txt
================================================================================

-- [DQL]
SELECT createxml ( '<a>123</a>' );

-- [DQL]
SELECT xmltype . createxml ( '<a>123</a>' );

-- [DQL]
select xmltype ( '<a>123<b>456</b></a>' ). extract ( '/a/b' ). getstringval ();

-- [DQL]
select getstringval ( extractxml ( xmltype ( '<a>123<b>456</b></a>' ), '/a/b' ));

-- [PLSQL]
declare a xmltype ;

-- [PLSQL]
declare xmltype_clob clob ;

-- [PLSQL]
declare xmltype_blob blob ;

-- [DQL]
SELECT getblobval ( xmltype ( '<asd/>' ), 7 );

-- [DQL]
select xmltype ( '<asd/>' ). getblobVal ( 7 );

-- [DQL]
SELECT getclobval ( xmltype ( '<a>123</a>' ));

-- [DQL]
SELECT xmltype ( '<a>123</a>' ). getclobval ();

-- [DQL]
SELECT getnumberval ( xmltype ( '<a>123</a>' ). extract ( '/a/text()' ));

-- [DQL]
SELECT xmltype ( '<a>123</a>' ). extract ( '/a/text()' ). getnumberval ();

-- [DQL]
SELECT isfragment ( xmltype ( '<a>123</a>' ));

-- [DQL]
SELECT xmltype ( '<a>123</a>' ). isfragment ();

-- [DQL]
SELECT xmltype ( '<a>123</a>' );

-- [PLSQL]
declare xmltype_clob clob ;

-- [PLSQL]
declare xmltype_blob blob ;

-- [DQL]
select getstringval('<a>123<b>456</b></a>');

-- [DQL]
select xmltype('<a>123<b>456</b></a>').getstringval();

-- [DQL]
select getrootelement('<a>123<b>456</b></a>');

-- [DQL]
select xmltype('<a>123<b>456</b></a>').getrootelement();

-- [DQL]
select getnamespace('<c:a xmlns:c="asd">123<d:b xmlns:d="qwe">456</d:b></c:a>');

-- [DQL]
select xmltype('<c:a xmlns:c="asd">123<d:b xmlns:d="qwe">456</d:b></c:a>').getnamespace();

-- [DQL]
select existsnode('<a>123<b>456</b></a>','/a/b');

-- [DQL]
select xmltype('<a>123<b>456</b></a>').existsnode('/a/b');

-- [DQL]
select existsnode('<a:b xmlns:a="asd">123<c>456</c></a:b>','/a:b/c','xmlns:a="asd"');

-- [DQL]
select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').existsnode('/a:b/c','xmlns:a="asd"');

-- [DQL]
select extractxml('<a>123<b>456</b></a>','/a/b');

-- [DQL]
select xmltype('<a>123<b>456</b></a>').extract('/a/b');

-- [DQL]
select xmltype('<a>123<b>456</b></a>').extractxml('/a/b');

-- [DQL]
select extractxml('<a:b xmlns:a="asd">123<c>456</c></a:b>','/a:b','xmlns:a="asd"');

-- [DQL]
select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').extract('/a:b','xmlns:a="asd"');

-- [DQL]
select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').extractxml('/a:b','xmlns:a="asd"');

-- [DQL]
SELECT xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>'));

-- [DQL]
SELECT unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//title/text()'))) AS title , unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//author/text()'))) AS author;

-- [DQL]
SELECT array_to_json(array_agg(row_to_json(t))) FROM ( SELECT unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//title/text()'))) AS title , unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinnger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//author/text()'))) AS author ) t;


================================================================================
-- 来源: 2823_Global Plsql Cache.txt
================================================================================

-- [DQL]
select * from gs_glc_memory_detail where type='func' or type='pkg';

-- [DQL]
select invalidate_plsql_object('public','f3','function');

-- [PLSQL]
call pg_catalog.invalidate_plsql_object('public','pkg1','package');

-- [DQL]
select invalidate_plsql_object();


================================================================================
-- 来源: 2824_file_2824.txt
================================================================================

-- [DQL]
select * from cross_test ;

-- [DDL]
create extension tablefunc ;

-- [DQL]
select * from crosstab ( 'select group_, id, var from cross_test order by 1, 2;

-- [DDL]
create extension tablefunc ;

-- [DQL]
select * from crosstab2 ( 'select group_, id, var from cross_test order by 1, 2;

-- [DDL]
create extension tablefunc ;

-- [DQL]
select * from crosstab ( 'select group_, id, var from cross_test order by 1, 2;


================================================================================
-- 来源: 2825_file_2825.txt
================================================================================

-- [DQL]
select uuid ();

-- [DQL]
SELECT uuid_short ();


================================================================================
-- 来源: 2826_SQL.txt
================================================================================

-- [DQL]
select gs_add_workload_rule ( 'sqlid' , 'rule for one query' , '' , now (), '' , 20 , '{id=32413214}' );

-- [DDL]
create database db1 ;

-- [DDL]
create database db2 ;

-- [DQL]
select gs_add_workload_rule ( 'select' , 'rule for select' , '{db1, db2}' , '' , '' , 100 , '{tb1, tb2}' );

-- [DQL]
select gs_add_workload_rule ( 'resource' , 'rule for resource' , '{}' , '' , '' , 20 , '{cpu-80}' );

-- [DDL]
create database db1 ;

-- [DQL]
select gs_update_workload_rule ( 2 , 'rule for select 2' , '{db1}' , now (), '' , 50 , '{tb1}' );

-- [DQL]
select gs_delete_workload_rule ( 3 );

-- [DQL]
select * from gs_get_workload_rule_stat ( 1 );

-- [DQL]
select * from gs_get_workload_rule_stat ( - 1 );


================================================================================
-- 来源: 2829_file_2829.txt
================================================================================

-- [DQL]
SELECT 2 BETWEEN 1 AND 3 AS RESULT ;

-- [DQL]
SELECT 2 >= 1 AND 2 <= 3 AS RESULT ;

-- [DQL]
SELECT 2 NOT BETWEEN 1 AND 3 AS RESULT ;

-- [DQL]
SELECT 2 < 1 OR 2 > 3 AS RESULT ;

-- [DQL]
SELECT 2 + 2 IS NULL AS RESULT ;

-- [DQL]
SELECT 2 + 2 IS NOT NULL AS RESULT ;

-- [DQL]
SELECT 2 + 2 ISNULL AS RESULT ;

-- [DQL]
SELECT 2 + 2 NOTNULL AS RESULT ;

-- [DQL]
SELECT 2 + 2 IS DISTINCT FROM NULL AS RESULT ;

-- [DQL]
SELECT 2 + 2 IS NOT DISTINCT FROM NULL AS RESULT ;

-- [DQL]
select 1 <=> 1 AS RESULT ;

-- [DQL]
select NULL <=> 1 AS RESULT ;

-- [DQL]
select NULL <=> NULL AS RESULT ;


================================================================================
-- 来源: 2830_file_2830.txt
================================================================================

-- [DDL]
CREATE TABLE tpcds . case_when_t1 ( CW_COL1 INT );

-- [DML_INSERT]
INSERT INTO tpcds . case_when_t1 VALUES ( 1 ), ( 2 ), ( 3 );

-- [DQL]
SELECT * FROM tpcds . case_when_t1 ;

-- [DQL]
SELECT CW_COL1 , CASE WHEN CW_COL1 = 1 THEN 'one' WHEN CW_COL1 = 2 THEN 'two' ELSE 'other' END FROM tpcds . case_when_t1 ORDER BY 1 ;

-- [DDL]
DROP TABLE tpcds . case_when_t1 ;

-- [DQL]
SELECT DECODE ( 'A' , 'A' , 1 , 'B' , 2 , 0 );

-- [DDL]
CREATE TABLE tpcds . c_tabl ( description varchar ( 10 ), short_description varchar ( 10 ), last_value varchar ( 10 )) ;

-- [DML_INSERT]
INSERT INTO tpcds . c_tabl VALUES ( 'abc' , 'efg' , '123' );

-- [DML_INSERT]
INSERT INTO tpcds . c_tabl VALUES ( NULL , 'efg' , '123' );

-- [DML_INSERT]
INSERT INTO tpcds . c_tabl VALUES ( NULL , NULL , '123' );

-- [DQL]
SELECT description , short_description , last_value , COALESCE ( description , short_description , last_value ) FROM tpcds . c_tabl ORDER BY 1 , 2 , 3 , 4 ;

-- [DDL]
DROP TABLE tpcds . c_tabl ;

-- [DQL]
SELECT COALESCE ( NULL , 'Hello World' );

-- [DDL]
CREATE TABLE tpcds . null_if_t1 ( NI_VALUE1 VARCHAR ( 10 ), NI_VALUE2 VARCHAR ( 10 ) );

-- [DML_INSERT]
INSERT INTO tpcds . null_if_t1 VALUES ( 'abc' , 'abc' );

-- [DML_INSERT]
INSERT INTO tpcds . null_if_t1 VALUES ( 'abc' , 'efg' );

-- [DQL]
SELECT NI_VALUE1 , NI_VALUE2 , NULLIF ( NI_VALUE1 , NI_VALUE2 ) FROM tpcds . null_if_t1 ORDER BY 1 , 2 , 3 ;

-- [DDL]
DROP TABLE tpcds . null_if_t1 ;

-- [DQL]
SELECT NULLIF ( 'Hello' , 'Hello World' );

-- [DQL]
SELECT greatest ( 9000 , 155555 , 2 . 01 );

-- [DQL]
SELECT least ( 9000 , 2 );

-- [DQL]
SELECT nvl ( null , 1 );

-- [DQL]
SELECT nvl ( 'Hello World' , 1 );


================================================================================
-- 来源: 2831_file_2831.txt
================================================================================

-- [DQL]
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE EXISTS ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom = store_returns . sr_reason_sk and sr_customer_sk < 10 );

-- [DQL]
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk IN ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom < 10 );

-- [DQL]
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk < ANY ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom < 10 );

-- [DQL]
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk < all ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom < 10 );


================================================================================
-- 来源: 2832_file_2832.txt
================================================================================

-- [DQL]
SELECT 8000 + 500 IN ( 10000 , 9000 ) AS RESULT ;

-- [DQL]
SELECT 8000 + 500 NOT IN ( 10000 , 9000 ) AS RESULT ;

-- [DQL]
SELECT 8000 + 500 < SOME ( array [ 10000 , 9000 ]) AS RESULT ;

-- [DQL]
SELECT 8000 + 500 < ANY ( array [ 10000 , 9000 ]) AS RESULT ;

-- [DQL]
SELECT 8000 + 500 < ALL ( array [ 10000 , 9000 ]) AS RESULT ;


================================================================================
-- 来源: 2833_file_2833.txt
================================================================================

-- [DQL]
SELECT ROW ( 1 , 2 , NULL ) < ROW ( 1 , 3 , 0 ) AS RESULT ;

-- [DQL]
select ( 4 , 5 , 6 ) > ( 3 , 2 , 1 ) as result ;

-- [DQL]
select ( 4 , 1 , 1 ) > ( 3 , 2 , 1 ) as result ;

-- [DQL]
select ( 'test' , 'data' ) > ( 'data' , 'data' ) as result ;

-- [DQL]
select ( 4 , 1 , 1 ) > ( 3 , 2 , null ) as result ;

-- [DQL]
select ( null , 1 , 1 ) > ( 3 , 2 , 1 ) as result ;

-- [DQL]
select ( null , 5 , 6 ) > ( null , 5 , 6 ) as result ;

-- [DQL]
select ( 4 , 5 , 6 ) > ( 4 , 5 , 6 ) as result ;

-- [DQL]
select ( 2 , 2 , 5 ) >= ( 2 , 2 , 3 ) as result ;

-- [DQL]
select ( 2 , 2 , 1 ) <= ( 2 , 2 , 3 ) as result ;

-- [DQL]
select ( 1 , 2 , 3 ) = ( 1 , 2 , 3 ) as result ;

-- [DQL]
select ( 1 , 2 , 3 ) <> ( 2 , 2 , 3 ) as result ;

-- [DQL]
select ( 2 , 2 , 3 ) <> ( 2 , 2 , null ) as result ;

-- [DQL]
select ( null , 5 , 6 ) <> ( null , 5 , 6 ) as result ;


================================================================================
-- 来源: 2834_file_2834.txt
================================================================================

-- [DQL]
SELECT DATE_ADD ( '2018-05-01' , INTERVAL 1 DAY );

-- [DQL]
SELECT DATE_SUB ( '2018-05-01' , INTERVAL 1 YEAR );

-- [DQL]
SELECT DATE '2023-01-10' - INTERVAL 1 DAY ;

-- [DQL]
SELECT DATE '2023-01-10' + INTERVAL 1 MONTH ;


================================================================================
-- 来源: 2835_file_2835.txt
================================================================================

-- [DDL]
CREATE TABLE Students ( name varchar ( 20 ), id int ) with ( STORAGE_TYPE = USTORE );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Jack' , 35 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Leon' , 15 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'James' , 24 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Taker' , 81 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Mary' , 25 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Rose' , 64 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Perl' , 18 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Under' , 57 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Angel' , 101 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Frank' , 20 );

-- [DML_INSERT]
INSERT INTO Students VALUES ( 'Charlie' , 40 );

-- [DQL]
SELECT * FROM Students WHERE rownum <= 10 ;

-- [DQL]
SELECT * FROM Students WHERE rownum < 5 order by 1 ;

-- [DQL]
SELECT rownum , * FROM ( SELECT * FROM Students order by 1 ) WHERE rownum <= 2 ;

-- [DQL]
SELECT * FROM Students WHERE rownum > 1 ;

-- [DQL]
SELECT * FROM Students ;

-- [DML_UPDATE]
update Students set id = id + 5 WHERE rownum < 4 ;

-- [DQL]
SELECT * FROM Students ;

-- [DDL]
drop table Students ;

-- [DDL]
create table test ( a int , b int );

-- [DML_INSERT]
insert into test select generate_series , generate_series from generate_series ( 1 , 10 );

-- [EXPLAIN]
EXPLAIN SELECT a , rownum FROM test group by a , rownum having rownum < 5 ;

-- [EXPLAIN]
EXPLAIN SELECT * FROM ( SELECT * FROM test WHERE rownum < 5 ) WHERE b < 5 ;

-- [DDL]
drop table test ;


================================================================================
-- 来源: 2837_file_2837.txt
================================================================================

-- [DQL]
SELECT text 'Origin' AS "label" , point '(0,0)' AS "value" ;


================================================================================
-- 来源: 2838_file_2838.txt
================================================================================

-- [DQL]
SELECT 40 ! AS "40 factorial" ;

-- [DQL]
SELECT CAST ( 40 AS bigint ) ! AS "40 factorial" ;

-- [DQL]
SELECT text 'abc' || 'def' AS "text and unknown" ;

-- [DQL]
SELECT 'abc' || 'def' AS "unspecified" ;

-- [DQL]
SELECT @ '-4.5' AS "abs" ;

-- [DQL]
SELECT array [ 1 , 2 ] <@ '{1,2,3}' as "is subset" ;


================================================================================
-- 来源: 2839_file_2839.txt
================================================================================

-- [DQL]
SELECT round ( 4 , 4 );

-- [DQL]
SELECT round ( CAST ( 4 AS numeric ), 4 );

-- [DQL]
SELECT round ( 4 . 0 , 4 );

-- [DQL]
SELECT substr ( '1234' , 3 );

-- [DQL]
SELECT substr ( varchar '1234' , 3 );

-- [DQL]
SELECT substr ( CAST ( varchar '1234' AS text ), 3 );

-- [DQL]
SELECT substr ( 1234 , 3 );

-- [DQL]
SELECT substr ( CAST ( 1234 AS text ), 3 );


================================================================================
-- 来源: 2840_file_2840.txt
================================================================================

-- [DDL]
CREATE TABLE tpcds . value_storage_t1 ( VS_COL1 CHARACTER ( 20 ) );

-- [DML_INSERT]
INSERT INTO tpcds . value_storage_t1 VALUES ( 'abcdef' );

-- [DQL]
SELECT VS_COL1 , octet_length ( VS_COL1 ) FROM tpcds . value_storage_t1 ;

-- [DDL]
DROP TABLE tpcds . value_storage_t1 ;


================================================================================
-- 来源: 2841_UNIONCASE.txt
================================================================================

-- [DQL]
SELECT text 'a' AS "text" UNION SELECT 'b' ;

-- [DQL]
SELECT 1 . 2 AS "numeric" UNION SELECT 1 ;

-- [DQL]
SELECT 1 AS "real" UNION SELECT CAST ( '2.2' AS REAL );

-- [DDL]
CREATE DATABASE a_1 dbcompatibility = 'A' ;

-- [OTHER]
\ c a_1 --创建表t1。 a_1 =# CREATE TABLE t1 ( a int , b varchar ( 10 ));

-- [DDL]
CREATE DATABASE td_1 dbcompatibility = 'C' ;

-- [OTHER]
\ c td_1 --创建表t2。 td_1 =# CREATE TABLE t2 ( a int , b varchar ( 10 ));

-- [DDL]
DROP DATABASE a_1 ;

-- [DDL]
DROP DATABASE td_1 ;

-- [DDL]
CREATE DATABASE ora_1 dbcompatibility = 'A';

--删除ORA模式的数据库。
-- [DDL]
DROP DATABASE ora_1;


================================================================================
-- 来源: 2845_file_2845.txt
================================================================================

-- [DQL]
SELECT d_dow || '-' || d_dom || '-' || d_fy_week_seq AS identify_serials FROM tpcds . date_dim WHERE d_fy_week_seq = 1 ;


================================================================================
-- 来源: 2846_file_2846.txt
================================================================================

-- [DQL]
SELECT 'a fat cat sat on a mat and ate a fat rat' :: tsvector @@ 'cat & rat' :: tsquery AS RESULT ;

-- [DQL]
SELECT 'fat & cow' :: tsquery @@ 'a fat cat sat on a mat and ate a fat rat' :: tsvector AS RESULT ;

-- [DQL]
SELECT to_tsvector ( 'fat cats ate fat rats' ) @@ to_tsquery ( 'fat & rat' ) AS RESULT ;

-- [DQL]
SELECT 'fat cats ate fat rats' :: tsvector @@ to_tsquery ( 'fat & rat' ) AS RESULT ;


================================================================================
-- 来源: 2849_file_2849.txt
================================================================================

-- [DDL]
DROP SCHEMA IF EXISTS tsearch CASCADE ;

-- [DDL]
CREATE SCHEMA tsearch ;

-- [DDL]
CREATE TABLE tsearch . pgweb ( id int , body text , title text , last_mod_date date ) with ( storage_type = ASTORE );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 1 , 'China, officially the People''s Republic of China (PRC), located in Asia, is the world''s most populous state.' , 'China' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 2 , 'America is a rock band, formed in England in 1970 by multi-instrumentalists Dewey Bunnell, Dan Peek, and Gerry Beckley.' , 'America' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 3 , 'England is a country that is part of the United Kingdom. It shares land borders with Scotland to the north and Wales to the west.' , 'England' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 4 , 'Australia, officially the Commonwealth of Australia, is a country comprising the mainland of the Australian continent, the island of Tasmania, and numerous smaller islands.' , 'Australia' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 6 , 'Japan is an island country in East Asia.' , 'Japan' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 7 , 'Germany, officially the Federal Republic of Germany, is a sovereign state and federal parliamentary republic in central-western Europe.' , 'Germany' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 8 , 'France, is a sovereign state comprising territory in western Europe and several overseas regions and territories.' , 'France' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 9 , 'Italy officially the Italian Republic, is a unitary parliamentary republic in Europe.' , 'Italy' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 10 , 'India, officially the Republic of India, is a country in South Asia.' , 'India' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 11 , 'Brazil, officially the Federative Republic of Brazil, is the largest country in both South America and Latin America.' , 'Brazil' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 12 , 'Canada is a country in the northern half of North America.' , 'Canada' , '2010-1-1' );

-- [DML_INSERT]
INSERT INTO tsearch . pgweb VALUES ( 13 , 'Mexico, officially the United Mexican States, is a federal republic in the southern part of North America.' , 'Mexico' , '2010-1-1' );

-- [DQL]
SELECT id , body , title FROM tsearch . pgweb WHERE to_tsvector ( 'english' , body ) @@ to_tsquery ( 'english' , 'america' );

-- [SESSION]
SHOW default_text_search_config ;

-- [DQL]
SELECT id , body , title FROM tsearch . pgweb WHERE to_tsvector ( body ) @@ to_tsquery ( 'america' );

-- [DQL]
SELECT title FROM tsearch . pgweb WHERE to_tsvector ( title || ' ' || body ) @@ to_tsquery ( 'north & america' ) ORDER BY last_mod_date DESC LIMIT 10 ;


================================================================================
-- 来源: 2850_file_2850.txt
================================================================================

-- [DDL]
CREATE INDEX pgweb_idx_1 ON tsearch . pgweb USING gin ( to_tsvector ( 'english' , body ));

-- [DDL]
CREATE INDEX pgweb_idx_2 ON tsearch . pgweb USING gin ( to_tsvector ( 'ngram' , body ));

-- [DDL]
CREATE INDEX pgweb_idx_3 ON tsearch . pgweb USING gin ( to_tsvector ( 'english' , title || ' ' || body ));

-- [DDL]
ALTER TABLE tsearch . pgweb ADD COLUMN textsearchable_index_col tsvector ;

-- [DML_UPDATE]
UPDATE tsearch . pgweb SET textsearchable_index_col = to_tsvector ( 'english' , coalesce ( title , '' ) || ' ' || coalesce ( body , '' ));

-- [DDL]
CREATE INDEX textsearch_idx_4 ON tsearch . pgweb USING gin ( textsearchable_index_col );

-- [DQL]
SELECT title FROM tsearch . pgweb WHERE textsearchable_index_col @@ to_tsquery ( 'north & america' ) ORDER BY last_mod_date DESC LIMIT 10 ;


================================================================================
-- 来源: 2851_file_2851.txt
================================================================================

-- [DDL]
create table table1 ( c_int int , c_bigint bigint , c_varchar varchar , c_text text ) with ( orientation = row , storage_type = ASTORE );

-- [DDL]
create text search configuration ts_conf_1 ( parser = POUND );

-- [DDL]
create text search configuration ts_conf_2 ( parser = POUND ) with ( split_flag = '%' );

-- [SESSION]
set default_text_search_config = 'ts_conf_1' ;

-- [DDL]
create index idx1 on table1 using gin ( to_tsvector ( c_text ));

-- [SESSION]
set default_text_search_config = 'ts_conf_2' ;

-- [DDL]
create index idx2 on table1 using gin ( to_tsvector ( c_text ));

-- [DQL]
select c_varchar , to_tsvector ( c_varchar ) from table1 where to_tsvector ( c_text ) @@ plainto_tsquery ( '￥#@……&**' ) and to_tsvector ( c_text ) @@ plainto_tsquery ( '某公司 ' ) and c_varchar is not null order by 1 desc limit 3 ;


================================================================================
-- 来源: 2853_file_2853.txt
================================================================================

-- [DQL]
SELECT to_tsvector ( 'english' , 'a fat cat sat on a mat - it ate a fat rats' );

-- [DDL]
CREATE TABLE tsearch . tt ( id int , title text , keyword text , abstract text , body text , ti tsvector );

-- [DML_INSERT]
INSERT INTO tsearch . tt ( id , title , keyword , abstract , body ) VALUES ( 1 , 'China' , 'Beijing' , 'China' , 'China, officially the People''s Republic of China (PRC), located in Asia, is the world''s most populous state.' );

-- [DML_UPDATE]
UPDATE tsearch . tt SET ti = setweight ( to_tsvector ( coalesce ( title , '' )), 'A' ) || setweight ( to_tsvector ( coalesce ( keyword , '' )), 'B' ) || setweight ( to_tsvector ( coalesce ( abstract , '' )), 'C' ) || setweight ( to_tsvector ( coalesce ( body , '' )), 'D' );

-- [DDL]
DROP TABLE tsearch . tt ;


================================================================================
-- 来源: 2854_file_2854.txt
================================================================================

-- [DQL]
SELECT to_tsquery ( 'english' , 'The & Fat & Rats' );

-- [DQL]
SELECT to_tsquery ( 'english' , 'Fat | Rats:AB' );

-- [DQL]
SELECT to_tsquery ( 'supern:*A & star:A*B' );

-- [DQL]
SELECT plainto_tsquery ( 'english' , 'The Fat Rats' );

-- [DQL]
SELECT plainto_tsquery ( 'english' , 'The Fat & Rats:C' );


================================================================================
-- 来源: 2855_file_2855.txt
================================================================================

-- [DQL]
SELECT id , title , ts_rank_cd ( to_tsvector ( body ), query ) AS rank FROM tsearch . pgweb , to_tsquery ( 'america' ) query WHERE query @@ to_tsvector ( body ) ORDER BY rank DESC LIMIT 10 ;

-- [DQL]
SELECT id , title , ts_rank_cd ( to_tsvector ( body ), query , 32 /* rank/(rank+1) */ ) AS rank FROM tsearch . pgweb , to_tsquery ( 'america' ) query WHERE query @@ to_tsvector ( body ) ORDER BY rank DESC LIMIT 10 ;

-- [DDL]
CREATE TABLE tsearch . ts_ngram ( id int , body text );

-- [DML_INSERT]
INSERT INTO tsearch . ts_ngram VALUES ( 1 , '中文' );

-- [DML_INSERT]
INSERT INTO tsearch . ts_ngram VALUES ( 2 , '中文检索' );

-- [DML_INSERT]
INSERT INTO tsearch . ts_ngram VALUES ( 3 , '检索中文' );

-- [DQL]
SELECT id , body , ts_rank_cd ( to_tsvector ( 'ngram' , body ), query ) AS rank FROM tsearch . ts_ngram , to_tsquery ( '中文' ) query WHERE query @@ to_tsvector ( body );

-- [DQL]
SELECT id , body , ts_rank_cd ( to_tsvector ( 'ngram' , body ), query ) AS rank FROM tsearch . ts_ngram , to_tsquery ( '中文' ) query WHERE query @@ to_tsvector ( 'ngram' , body );


================================================================================
-- 来源: 2856_file_2856.txt
================================================================================

-- [DQL]
SELECT ts_headline ( 'english' , 'The most common type of search is to find all documents containing given query terms and return them in order of their similarity to the query.' , to_tsquery ( 'english' , 'query & similarity' ));

-- [DQL]
SELECT ts_headline ( 'english' , 'The most common type of search is to find all documents containing given query terms and return them in order of their similarity to the query.' , to_tsquery ( 'english' , 'query & similarity' ), 'StartSel = <, StopSel = >' );


================================================================================
-- 来源: 2859_file_2859.txt
================================================================================

-- [DQL]
SELECT numnode ( plainto_tsquery ( 'the any' ));

-- [DQL]
SELECT numnode(' foo & bar ' :: tsquery );

-- [DQL]
SELECT querytree ( to_tsquery ( '!defined' ));


================================================================================
-- 来源: 2860_file_2860.txt
================================================================================

-- [DQL]
SELECT ts_rewrite ( 'a & b' :: tsquery , 'a' :: tsquery , 'c' :: tsquery );

-- [DDL]
CREATE TABLE tsearch . aliases ( id int , t tsquery , s tsquery );

-- [DML_INSERT]
INSERT INTO tsearch . aliases VALUES ( 1 , to_tsquery ( 'supernovae' ), to_tsquery ( 'supernovae|sn' ));

-- [DQL]
SELECT ts_rewrite ( to_tsquery ( 'supernovae & crab' ), 'SELECT t, s FROM tsearch.aliases' );

-- [DML_UPDATE]
UPDATE tsearch . aliases SET s = to_tsquery ( 'supernovae|sn & !nebulae' ) WHERE t = to_tsquery ( 'supernovae' );

-- [DQL]
SELECT ts_rewrite ( to_tsquery ( 'supernovae & crab' ), 'SELECT t, s FROM tsearch.aliases' );

-- [DQL]
SELECT ts_rewrite ( 'a & b' :: tsquery , 'SELECT t,s FROM tsearch.aliases WHERE ''a & b''::tsquery @> t' );

-- [DDL]
DROP TABLE tsearch . aliases ;


================================================================================
-- 来源: 2861_file_2861.txt
================================================================================

-- [DQL]
SELECT * FROM ts_stat ( 'SELECT to_tsvector(''english'', sr_reason_sk) FROM tpcds.store_returns WHERE sr_customer_sk < 10' ) ORDER BY nentry DESC , ndoc DESC , word LIMIT 10 ;

-- [DQL]
SELECT * FROM ts_stat ( 'SELECT to_tsvector(''english'', sr_reason_sk) FROM tpcds.store_returns WHERE sr_customer_sk < 10' , 'a' ) ORDER BY nentry DESC , ndoc DESC , word LIMIT 10 ;


================================================================================
-- 来源: 2862_file_2862.txt
================================================================================

-- [DQL]
SELECT alias , description , token FROM ts_debug ( 'english' , 'foo-bar-beta1' );

-- [DQL]
SELECT alias , description , token FROM ts_debug ( 'english' , 'http://example.com/stuff/index.html' );


================================================================================
-- 来源: 2864_file_2864.txt
================================================================================

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION astro_en ADD MAPPING FOR asciiword WITH astro_syn , english_ispell , english_stem ;


================================================================================
-- 来源: 2865_file_2865.txt
================================================================================

-- [DQL]
SELECT to_tsvector ( 'english' , 'in the list of stop words' );

-- [DQL]
SELECT ts_rank_cd ( to_tsvector ( 'english' , 'in the list of stop words' ), to_tsquery ( 'list & stop' ));

-- [DQL]
SELECT ts_rank_cd ( to_tsvector ( 'english' , 'list stop words' ), to_tsquery ( 'list & stop' ));


================================================================================
-- 来源: 2866_Simple.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH DICTIONARY public . simple_dict ( TEMPLATE = pg_catalog . simple , STOPWORDS = english );

-- [DQL]
SELECT ts_lexize ( 'public.simple_dict' , 'YeS' );

-- [DQL]
SELECT ts_lexize ( 'public.simple_dict' , 'The' );

-- [DDL]
ALTER TEXT SEARCH DICTIONARY public . simple_dict ( Accept = false );

-- [DQL]
SELECT ts_lexize ( 'public.simple_dict' , 'YeS' );

-- [DQL]
SELECT ts_lexize ( 'public.simple_dict' , 'The' );


================================================================================
-- 来源: 2867_Synonym.txt
================================================================================

-- [DQL]
SELECT * FROM ts_debug ( 'english' , 'Paris' );

-- [DDL]
CREATE TEXT SEARCH DICTIONARY my_synonym ( TEMPLATE = synonym , SYNONYMS = my_synonyms , FILEPATH = 'file:///home/dicts/' );

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION english ALTER MAPPING FOR asciiword WITH my_synonym , english_stem ;

-- [DQL]
SELECT * FROM ts_debug ( 'english' , 'Paris' );

-- [DQL]
SELECT * FROM ts_debug ( 'english' , 'paris' );

-- [DDL]
ALTER TEXT SEARCH DICTIONARY my_synonym ( CASESENSITIVE = true );

-- [DQL]
SELECT * FROM ts_debug ( 'english' , 'Paris' );

-- [DQL]
SELECT * FROM ts_debug ( 'english' , 'paris' );

-- [DDL]
CREATE TEXT SEARCH DICTIONARY syn ( TEMPLATE = synonym , SYNONYMS = synonym_sample );

-- [DQL]
SELECT ts_lexize ( 'syn' , 'indices' );

-- [DDL]
CREATE TEXT SEARCH CONFIGURATION tst ( copy = simple );

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION tst ALTER MAPPING FOR asciiword WITH syn ;

-- [DQL]
SELECT to_tsvector ( 'tst' , 'indices' );

-- [DQL]
SELECT to_tsquery ( 'tst' , 'indices' );

-- [DQL]
SELECT 'indexes are very useful' :: tsvector ;

-- [DQL]
SELECT 'indexes are very useful' :: tsvector @@ to_tsquery ( 'tst' , 'indices' );


================================================================================
-- 来源: 2868_Thesaurus.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH DICTIONARY thesaurus_astro ( TEMPLATE = thesaurus , DictFile = thesaurus_astro , Dictionary = pg_catalog . english_stem , FILEPATH = 'file:///home/dicts/' );

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION russian ALTER MAPPING FOR asciiword , asciihword , hword_asciipart WITH thesaurus_astro , english_stem ;

-- [DQL]
SELECT plainto_tsquery ( 'russian' , 'supernova star' );

-- [DQL]
SELECT to_tsvector ( 'russian' , 'supernova star' );

-- [DQL]
SELECT to_tsquery ( 'russian' , '''supernova star''' );

-- [DDL]
ALTER TEXT SEARCH DICTIONARY thesaurus_astro ( DictFile = thesaurus_astro , FILEPATH = 'file:///home/dicts/' );

-- [DQL]
SELECT plainto_tsquery ( 'russian' , 'supernova star' );


================================================================================
-- 来源: 2869_Ispell.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH DICTIONARY norwegian_ispell ( TEMPLATE = ispell , DictFile = nn_no , AffFile = nn_no , FilePath = 'file:///home/dicts' );

-- [DQL]
SELECT ts_lexize ( 'norwegian_ispell' , 'sjokoladefabrikk' );


================================================================================
-- 来源: 2871_file_2871.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH CONFIGURATION ts_conf ( COPY = pg_catalog . english );

-- [DDL]
CREATE TEXT SEARCH DICTIONARY gs_dict ( TEMPLATE = synonym , SYNONYMS = gs_dict , FILEPATH = 'file:///home/dicts' );

-- [DDL]
CREATE TEXT SEARCH DICTIONARY english_ispell ( TEMPLATE = ispell , DictFile = english , AffFile = english , StopWords = english , FILEPATH = 'file:///home/dicts' );

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION ts_conf ALTER MAPPING FOR asciiword , asciihword , hword_asciipart , word , hword , hword_part WITH gs_dict , english_ispell , english_stem ;

-- [DDL]
ALTER TEXT SEARCH CONFIGURATION ts_conf DROP MAPPING FOR email , url , url_path , sfloat , float ;

-- [DQL]
SELECT * FROM ts_debug ( 'ts_conf' , ' GaussDB, the highly scalable, SQL compliant, open source object-relational database management system, is now undergoing beta testing of the next version of our software. ' );

-- [OTHER]
\ dF + ts_conf Text search configuration "public.ts_conf" Parser : "pg_catalog.default" Token | Dictionaries -----------------+------------------------------------- asciihword | gs_dict , english_ispell , english_stem asciiword | gs_dict , english_ispell , english_stem file | simple host | simple hword | gs_dict , english_ispell , english_stem hword_asciipart | gs_dict , english_ispell , english_stem hword_numpart | simple hword_part | gs_dict , english_ispell , english_stem int | simple numhword | simple numword | simple uint | simple version | simple word | gs_dict , english_ispell , english_stem

-- [SESSION]
SET default_text_search_config = 'public.ts_conf' ;

-- [SESSION]
SHOW default_text_search_config ;


================================================================================
-- 来源: 2873_file_2873.txt
================================================================================

-- [DQL]
SELECT * FROM ts_debug ( 'english' , 'a fat cat sat on a mat - it ate a fat rats' );


================================================================================
-- 来源: 2874_file_2874.txt
================================================================================

-- [DQL]
SELECT * FROM ts_parse ( 'default' , '123 - a number' );

-- [DQL]
SELECT * FROM ts_token_type ( 'default' );


================================================================================
-- 来源: 2875_file_2875.txt
================================================================================

-- [DQL]
SELECT ts_lexize ( 'english_stem' , 'stars' );

-- [DQL]
SELECT ts_lexize ( 'english_stem' , 'a' );


================================================================================
-- 来源: 2882_ABORT.txt
================================================================================

-- [DDL]
CREATE TABLE customer_demographics_t1 ( CD_DEMO_SK INTEGER NOT NULL, CD_GENDER CHAR(1) , CD_MARITAL_STATUS CHAR(1) , CD_EDUCATION_STATUS CHAR(20) , CD_PURCHASE_ESTIMATE INTEGER , CD_CREDIT_RATING CHAR(10) , CD_DEP_COUNT INTEGER , CD_DEP_EMPLOYED_COUNT INTEGER , CD_DEP_COLLEGE_COUNT INTEGER ) ;

--插入记录。
-- [DML_INSERT]
INSERT INTO customer_demographics_t1 VALUES(1920801,'M', 'U', 'DOCTOR DEGREE', 200, 'GOOD', 1, 0,0);

--开启事务。
-- [TCL]
START TRANSACTION;

--更新字段值。
-- [DML_UPDATE]
UPDATE customer_demographics_t1 SET cd_education_status= 'Unknown';

--终止事务，上面所执行的更新会被撤销掉。
-- [TCL]
ABORT;

--查询数据。
-- [DQL]
SELECT * FROM customer_demographics_t1 WHERE cd_demo_sk = 1920801;

--删除表。
-- [DDL]
DROP TABLE customer_demographics_t1;


================================================================================
-- 来源: 2883_ALTER AGGREGATE.txt
================================================================================

-- [DDL]
CREATE OR REPLACE FUNCTION int_add(int,int) returns int as $BODY$ declare begin return $1 + $2;

-- 创建聚合函数。
-- [OTHER]
CREATE AGGREGATE myavg (int) ( sfunc = int_add, stype = int, initcond = '0' );

--把一个接受integer 类型参数的聚合函数myavg重命名为 my_average 。
-- [OTHER]
ALTER AGGREGATE myavg(integer) RENAME TO my_average;

--创建用户joe。
-- [DDL]
CREATE USER joe PASSWORD ' ******** ';

--把一个接受integer 类型参数的聚合函数myavg的所有者改为joe 。
-- [OTHER]
ALTER AGGREGATE my_average(integer) OWNER TO joe;

--创建SCHEMA。
-- [DDL]
CREATE SCHEMA myschema;

--把一个接受integer 类型参数的聚合函数myavg移动到模式myschema里。
-- [OTHER]
ALTER AGGREGATE my_average(integer) SET SCHEMA myschema;

--删除SCHEMA,用户及相关函数。
-- [DDL]
DROP SCHEMA myschema CASCADE;

-- [DDL]
DROP USER joe;

-- [DDL]
DROP FUNCTION int_add(int,int);


================================================================================
-- 来源: 2885_ALTER DATABASE.txt
================================================================================

-- [DDL]
CREATE DATABASE testdb;

--将testdb重命名为test_db1。
-- [DDL]
ALTER DATABASE testdb RENAME TO test_db1;

-- [DDL]
ALTER DATABASE test_db1 WITH CONNECTION LIMIT 100;

--查看test_db1信息。
-- [DQL]
SELECT datname,datconnlimit FROM pg_database WHERE datname = 'test_db1';

-- [DDL]
CREATE USER scott PASSWORD '********';

--将test_db1的所有者修改为jim。
-- [DDL]
ALTER DATABASE test_db1 OWNER TO scott;

--查看test_db1信息。
-- [DQL]
SELECT t1.datname, t2.usename FROM pg_database t1, pg_user t2 WHERE t1.datname='test_db1' AND t1.datdba=t2.usesysid;

-- [DDL]
CREATE TABLESPACE tbs_data1 RELATIVE LOCATION 'tablespace1/tbs_data1';

--修改test_db1默认表空间。
-- [DDL]
ALTER DATABASE test_db1 SET TABLESPACE tbs_data1;

--查看test_db1信息。
-- [DQL]
SELECT t1.datname AS database, t2.spcname AS tablespace FROM pg_database t1, pg_tablespace t2 WHERE t1.dattablespace = t2.oid AND t1.datname = 'test_db1';

-- [DDL]
CREATE USER jack PASSWORD '********';

-- [DDL]
CREATE TABLE test_tbl1(c1 int,c2 int);

-- [DQL]
SELECT tablename FROM pg_tables WHERE tablename = 'test_tbl1';

-- [DDL]
ALTER DATABASE test_db1 ENABLE PRIVATE OBJECT;

--由于隔离属性的原因，该查询只能查出0条数据。
-- [DQL]
SELECT tablename FROM pg_tables WHERE tablename = 'test_tbl1';

-- [DDL]
DROP TABLE public.test_tbl1;

-- [DDL]
DROP DATABASE test_db1;

-- [DDL]
DROP TABLESPACE tbs_data1;

-- [DDL]
DROP USER jack;

-- [DDL]
DROP USER scott;


================================================================================
-- 来源: 2886_ALTER DATABASE LINK.txt
================================================================================

-- [DDL]
CREATE USER user01 WITH SYSADMIN PASSWORD '********';

-- [SESSION]
SET ROLE user01 PASSWORD '********';

--创建公共dblink
-- [OTHER]
CREATE PUBLIC DATABASE LINK public_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

--创建普通用户
-- [DDL]
CREATE USER user2 PASSWORD '********';

-- 修改dblink对象信息
-- [OTHER]
ALTER PUBLIC DATABASE LINK public_dblink CONNECT TO 'user2' IDENTIFIED BY '********';

--删除公共dblink
-- [OTHER]
DROP PUBLIC DATABASE LINK public_dblink;

--删除创建出的用户
-- [SESSION]
RESET ROLE;

-- [DDL]
DROP USER user01 CASCADE;

-- [DDL]
DROP USER user02;


================================================================================
-- 来源: 2887_ALTER DATA SOURCE.txt
================================================================================

-- [DDL]
CREATE DATA SOURCE ds_test1;

--修改名称。
-- [OTHER]
ALTER DATA SOURCE ds_test1 RENAME TO ds_test;

--创建用户和修改所有者。
-- [DDL]
CREATE USER user_test1 IDENTIFIED BY '********';

-- [DDL]
ALTER USER user_test1 WITH SYSADMIN;

-- [OTHER]
ALTER DATA SOURCE ds_test OWNER TO user_test1;

--修改TYPE和VERSION。
-- [OTHER]
ALTER DATA SOURCE ds_test TYPE 'MPPDB_TYPE' VERSION 'XXX';

--添加字段。
-- [OTHER]
ALTER DATA SOURCE ds_test OPTIONS (add dsn ' gaussdb ', username 'test_user');

--修改字段。
-- [OTHER]
ALTER DATA SOURCE ds_test OPTIONS (set dsn 'unknown');

--删除字段。
-- [OTHER]
ALTER DATA SOURCE ds_test OPTIONS (drop username);

--删除Data Source和user对象。
-- [DDL]
DROP DATA SOURCE ds_test;

-- [DDL]
DROP USER user_test1;


================================================================================
-- 来源: 2888_ALTER DEFAULT PRIVILEGES.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--将创建在模式tpcds里的所有表（和视图）的SELECT权限授予每一个用户。
-- [DDL]
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds GRANT SELECT ON TABLES TO PUBLIC;

--创建用户普通用户jack。
-- [DDL]
CREATE USER jack PASSWORD ' ******** ';

--将tpcds下的所有表的插入权限授予用户jack。
-- [DDL]
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds GRANT INSERT ON TABLES TO jack;

--将tpcds下由jack创建的所有表的插入权限授予用户jack。
-- [DCL_GRANT]
GRANT USAGE,CREATE ON SCHEMA tpcds TO jack;

-- [DDL]
ALTER DEFAULT PRIVILEGES FOR ROLE jack IN SCHEMA tpcds GRANT INSERT ON TABLES TO jack;

--撤销上述权限。
-- [DDL]
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds REVOKE SELECT ON TABLES FROM PUBLIC;

-- [DDL]
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds REVOKE INSERT ON TABLES FROM jack;

--删除用户jack。
-- [DDL]
DROP USER jack CASCADE;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds;


================================================================================
-- 来源: 2889_ALTER DIRECTORY.txt
================================================================================

-- [DDL]
CREATE OR REPLACE DIRECTORY dir as '/tmp/';

--创建用户
-- [DDL]
CREATE USER jim PASSWORD '********';

--修改目录的owner。
-- [DDL]
ALTER DIRECTORY dir OWNER TO jim;

--删除目录。
-- [DDL]
DROP DIRECTORY dir;


================================================================================
-- 来源: 2892_ALTER FOREIGN TABLE.txt
================================================================================

-- [DDL]
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--创建外表
-- [DDL]
CREATE FOREIGN TABLE foreign_tbl (col1 text) SERVER my_server OPTIONS (logtype 'pg_log');

--修改外表属性
-- [DDL]
ALTER FOREIGN TABLE foreign_tbl OPTIONS (ADD latest_files '2');

-- [DDL]
ALTER FOREIGN TABLE foreign_tbl OPTIONS ( SET latest_files '5');

-- [DDL]
ALTER FOREIGN TABLE foreign_tbl OPTIONS ( DROP latest_files);

-- [DDL]
DROP FOREIGN TABLE foreign_tbl;

-- [DDL]
DROP SERVER my_server;


================================================================================
-- 来源: 2893_ALTER FUNCTION.txt
================================================================================

-- [SESSION]
SET behavior_compat_options ='plpgsql_dependency';

-- 创建函数
-- [DDL]
CREATE OR REPLACE FUNCTION test_func(a int) RETURN int IS proc_var int;

-- 用函数名重编译函数
-- [OTHER]
ALTER PROCEDURE test_func COMPILE;

-- 用函数带类型签名重编译存储过程
-- [OTHER]
ALTER PROCEDURE test_func(int) COMPILE;

-- 删除函数
-- [DDL]
DROP FUNCTION test_func;


================================================================================
-- 来源: 2894_ALTER GLOBAL CONFIGURATION.txt
================================================================================

-- [DDL]
ALTER GLOBAL CONFIGURATION with ( redis_is_ok = true );

-- [DQL]
SELECT * FROM gs_global_config ;

-- [DDL]
ALTER GLOBAL CONFIGURATION with ( redis_is_ok = false );

-- [DQL]
SELECT * FROM gs_global_config ;

-- [DDL]
DROP GLOBAL CONFIGURATION redis_is_ok ;

-- [DQL]
SELECT * FROM gs_global_config ;


================================================================================
-- 来源: 2896_ALTER INDEX.txt
================================================================================

-- [DDL]
CREATE TABLE test1(col1 int, col2 int);

-- [DDL]
CREATE INDEX aa ON test1(col1);

--将索引aa重命名为idx_test1_col1。
-- [DDL]
ALTER INDEX aa RENAME TO idx_test1_col1;

--查询test1表上的索引信息。
-- [DQL]
SELECT tablename,indexname,tablespace FROM pg_indexes WHERE tablename = 'test1';

-- [DDL]
CREATE TABLESPACE tbs_index1 RELATIVE LOCATION 'tablespace1/tbs_index1';

--修改索引idx_test1_col1的所属表空间为tbs_index1。
-- [DDL]
ALTER INDEX IF EXISTS idx_test1_col1 SET TABLESPACE tbs_index1;

--查询test1表上的索引信息。
-- [DQL]
SELECT tablename,indexname,tablespace FROM pg_indexes WHERE tablename = 'test1';

--修改索引idx_test1_col1 的填充因子。
-- [DDL]
ALTER INDEX IF EXISTS idx_test1_col1 SET (FILLFACTOR = 70);

-- [DDL]
ALTER INDEX IF EXISTS idx_test1_col1 RESET (FILLFACTOR);

-- [DDL]
ALTER INDEX IF EXISTS idx_test1_col1 UNUSABLE;

--查看索引idx_test1_col1的可用性。
-- [DQL]
SELECT indisusable FROM pg_index WHERE indexrelid = 'idx_test1_col1'::regclass;

--重建索引idx_test1_col1。
-- [DDL]
ALTER INDEX idx_test1_col1 REBUILD;

--查看索引idx_test1_col1的可用性。
-- [DQL]
SELECT indisusable FROM pg_index WHERE indexrelid = 'idx_test1_col1'::regclass;

--删除。
-- [DDL]
DROP INDEX idx_test1_col1;

-- [DDL]
DROP TABLE test1;

-- [DDL]
DROP TABLESPACE tbs_index1;

-- [DDL]
CREATE TABLE test2(col1 int, col2 int) PARTITION BY RANGE (col1)( PARTITION p1 VALUES LESS THAN (100), PARTITION p2 VALUES LESS THAN (200) );

--创建分区索引。
-- [DDL]
CREATE INDEX idx_test2_col1 ON test2(col1) LOCAL( PARTITION p1, PARTITION p2 );

--重命名索引分区。
-- [DDL]
ALTER INDEX idx_test2_col1 RENAME PARTITION p1 TO p1_test2_idx;

-- [DDL]
ALTER INDEX idx_test2_col1 RENAME PARTITION p2 TO p2_test2_idx;

--查询索引idx_test2_col1分区的名称。
-- [DQL]
SELECT relname FROM pg_partition WHERE parentid = 'idx_test2_col1'::regclass;

-- [DDL]
CREATE TABLESPACE tbs_index2 RELATIVE LOCATION 'tablespace1/tbs_index2';

-- [DDL]
CREATE TABLESPACE tbs_index3 RELATIVE LOCATION 'tablespace1/tbs_index3';

--修改索引idx_test2_col1分区的所属表空间。
-- [DDL]
ALTER INDEX idx_test2_col1 MOVE PARTITION p1_test2_idx TABLESPACE tbs_index2;

-- [DDL]
ALTER INDEX idx_test2_col1 MOVE PARTITION p2_test2_idx TABLESPACE tbs_index3;

--查询索引idx_test2_col1分区的所属表空间。
-- [DQL]
SELECT t1.relname index_name, t2.spcname tablespace_name FROM pg_partition t1, pg_tablespace t2 WHERE t1.parentid = 'idx_test2_col1'::regclass AND t1.reltablespace = t2.oid;

--删除。
-- [DDL]
DROP INDEX idx_test2_col1;

-- [DDL]
DROP TABLE test2;

-- [DDL]
DROP TABLESPACE tbs_index2;

-- [DDL]
DROP TABLESPACE tbs_index3;


================================================================================
-- 来源: 2899_ALTER MASKING POLICY.txt
================================================================================

-- [DDL]
CREATE USER dev_mask PASSWORD '********' ;

-- [DDL]
CREATE USER bob_mask PASSWORD '********' ;

-- [DDL]
CREATE TABLE tb_for_masking ( col1 text , col2 text , col3 text );

-- [DDL]
CREATE RESOURCE LABEL mask_lb1 ADD COLUMN ( tb_for_masking . col1 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb2 ADD COLUMN ( tb_for_masking . col2 );

-- [DDL]
CREATE MASKING POLICY maskpol1 maskall ON LABEL ( mask_lb1 );

-- [DDL]
ALTER MASKING POLICY maskpol1 COMMENTS 'masking policy for tb_for_masking.col1' ;

-- [DDL]
ALTER MASKING POLICY maskpol1 ADD randommasking ON LABEL ( mask_lb2 );

-- [DDL]
ALTER MASKING POLICY maskpol1 REMOVE randommasking ON LABEL ( mask_lb2 );

-- [DDL]
ALTER MASKING POLICY maskpol1 MODIFY randommasking ON LABEL ( mask_lb1 );

-- [DDL]
ALTER MASKING POLICY maskpol1 MODIFY ( FILTER ON ROLES ( dev_mask , bob_mask ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' ));

-- [DDL]
ALTER MASKING POLICY maskpol1 DROP FILTER ;

-- [DDL]
ALTER MASKING POLICY maskpol1 DISABLE ;

-- [DDL]
DROP MASKING POLICY maskpol1 ;

-- [DDL]
DROP RESOURCE LABEL mask_lb1 , mask_lb2 ;

-- [DDL]
DROP TABLE tb_for_masking ;

-- [DDL]
DROP USER dev_mask , bob_mask ;


================================================================================
-- 来源: 2900_ALTER MATERIALIZED VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建全量物化视图。
-- [DDL]
CREATE MATERIALIZED VIEW foo AS SELECT * FROM my_table;

--把物化视图foo重命名为bar。
-- [DDL]
ALTER MATERIALIZED VIEW foo RENAME TO bar;

--删除全量物化视图。
-- [DDL]
DROP MATERIALIZED VIEW bar;

--删除表my_table。
-- [DDL]
DROP TABLE my_table;


================================================================================
-- 来源: 2901_ALTER OPERATOR.txt
================================================================================

-- [OTHER]
ALTER OPERATOR @@ (text, text) OWNER TO omm;


================================================================================
-- 来源: 2903_ALTER PACKAGE.txt
================================================================================

-- [DDL]
CREATE DATABASE ora_compatible_db DBCOMPATIBILITY 'ORA';

-- 开启依赖功能
-- [SESSION]
SET behavior_compat_options ='plpgsql_dependency';

-- 创建包
-- [DDL]
CREATE OR REPLACE PACKAGE test_pkg AS pkg_var int := 1;

-- [DDL]
CREATE OR REPLACE PACKAGE body test_pkg AS procedure test_pkg_proc(var int) IS BEGIN pkg_var := 1;

-- 重编译包
-- [DDL]
ALTER PACKAGE test_pkg COMPILE;

--删除包
-- [DDL]
DROP PACKAGE test_pkg;


================================================================================
-- 来源: 2904_ALTER PROCEDURE.txt
================================================================================

-- [SESSION]
SET behavior_compat_options ='plpgsql_dependency';

-- 创建存储过程
-- [DDL]
CREATE OR REPLACE PROCEDURE test_proc(a int) IS proc_var int;

-- 用存储过程名重编译存储过程
-- [OTHER]
ALTER PROCEDURE test_proc COMPILE;

-- 用存储过程带类型签名重编译存储过程
-- [OTHER]
ALTER PROCEDURE test_proc(int) COMPILE;

-- 删除存储过程
-- [DDL]
DROP PROCEDURE test_proc;


================================================================================
-- 来源: 2905_ALTER RESOURCE LABEL.txt
================================================================================

-- [DDL]
CREATE TABLE table_for_label ( col1 int , col2 text );

-- [DDL]
CREATE RESOURCE LABEL table_label ADD COLUMN ( table_for_label . col1 );

-- [DDL]
ALTER RESOURCE LABEL table_label ADD COLUMN ( table_for_label . col2 );

-- [DDL]
ALTER RESOURCE LABEL table_label REMOVE COLUMN ( table_for_label . col1 );

-- [DDL]
DROP RESOURCE LABEL table_label ;

-- [DDL]
DROP TABLE table_for_label ;


================================================================================
-- 来源: 2906_ALTER RESOURCE POOL.txt
================================================================================

-- [DDL]
CREATE RESOURCE POOL pool1 ;

-- [DDL]
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "High" );

-- [DDL]
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "class1:Low" );

-- [DDL]
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "class1:wg1" );

-- [DDL]
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "class1:wg2:3" );

-- [DDL]
DROP RESOURCE POOL pool1 ;


================================================================================
-- 来源: 2908_ALTER ROW LEVEL SECURITY POLICY.txt
================================================================================

-- [DDL]
CREATE TABLE all_data(id int, role varchar(100), data varchar(100));

--创建行访问控制策略，当前用户只能查看用户自身的数据。
-- [OTHER]
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING(role = CURRENT_USER);

--创建用户alice, bob。
-- [DDL]
CREATE ROLE alice WITH PASSWORD "********";

-- [DDL]
CREATE ROLE bob WITH PASSWORD "********";

--修改行访问控制all_data_rls的名称。
-- [OTHER]
ALTER ROW LEVEL SECURITY POLICY all_data_rls ON all_data RENAME TO all_data_new_rls;

--修改行访问控制策略影响的用户。
-- [OTHER]
ALTER ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data TO alice, bob;

--修改行访问控制策略表达式。
-- [OTHER]
ALTER ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data USING (id > 100 AND role = current_user);

--删除访问控制策略。
-- [OTHER]
DROP ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data;

--删除用户alice, bob。
-- [DDL]
DROP ROLE alice, bob;

--删除数据表all_data。
-- [DDL]
DROP TABLE all_data;


================================================================================
-- 来源: 2909_ALTER SCHEMA.txt
================================================================================

-- [DDL]
CREATE DATABASE test1 WITH DBCOMPATIBILITY = 'b';

--创建模式ds。
-- [DDL]
CREATE SCHEMA ds;

--将当前模式ds更名为ds_new。
-- [DDL]
ALTER SCHEMA ds RENAME TO ds_new;

--创建用户jack。
-- [DDL]
CREATE USER jack PASSWORD ' ******** ';

--将DS_NEW的所有者修改为jack。
-- [DDL]
ALTER SCHEMA ds_new OWNER TO jack;

--将sch的默认字符集修改为utf8mb4，默认字符序修改为utf8mb4_bin。仅在B模式下（即sql_compatibility='B'）支持该语法。
-- [DDL]
CREATE SCHEMA sch;

-- [DDL]
ALTER SCHEMA sch CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;

--删除模式ds_new和sch。
-- [DDL]
DROP SCHEMA ds_new;

-- [DDL]
DROP SCHEMA sch;

--删除用户jack。
-- [DDL]
DROP USER jack;

-- [DDL]
DROP DATABASE test1;


================================================================================
-- 来源: 2910_ALTER SEQUENCE.txt
================================================================================

-- [DDL]
CREATE SEQUENCE serial START 101;

--创建一个表,定义默认值。
-- [DDL]
CREATE TABLE t1(c1 bigint default nextval('serial'));

--将序列serial的归属列变为T1.C1。
-- [DDL]
ALTER SEQUENCE serial OWNED BY t1.c1;

--删除序列和表。
-- [DDL]
DROP SEQUENCE serial CASCADE;

-- [DDL]
DROP TABLE t1;


================================================================================
-- 来源: 2911_ALTER SERVER.txt
================================================================================

-- [DDL]
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--修改外部服务的名称。
-- [DDL]
ALTER SERVER my_server RENAME TO my_server_1;

--删除my_server_1。
-- [DDL]
DROP SERVER my_server_1;


================================================================================
-- 来源: 2912_ALTER SESSION.txt
================================================================================

-- [DDL]
CREATE SCHEMA ds;

--设置模式搜索路径。
-- [SESSION]
SET SEARCH_PATH TO ds, public;

--设置日期时间风格为传统的POSTGRES风格（日在月前）。
-- [SESSION]
SET DATESTYLE TO postgres, dmy;

--设置当前会话的字符编码为UTF8。
-- [DDL]
ALTER SESSION SET NAMES 'UTF8';

--设置时区为加州伯克利。
-- [SESSION]
SET TIME ZONE 'PST8PDT';

--设置时区为意大利。
-- [SESSION]
SET TIME ZONE 'Europe/Rome';

--设置当前模式。
-- [DDL]
ALTER SESSION SET CURRENT_SCHEMA TO tpcds;

--设置XML OPTION为DOCUMENT。
-- [DDL]
ALTER SESSION SET XML OPTION DOCUMENT;

--创建角色joe，并设置会话的角色为joe。
-- [DDL]
CREATE ROLE joe WITH PASSWORD ' ******** ';

-- [DDL]
ALTER SESSION SET SESSION AUTHORIZATION joe PASSWORD ' ******** ';

--删除ds模式。
-- [DDL]
DROP SCHEMA ds;

--删除joe。
-- [DDL]
DROP ROLE joe;

--开启事务,设置事务级别
-- [TCL]
START TRANSACTION;

-- [DDL]
ALTER SESSION SET TRANSACTION READ ONLY;


================================================================================
-- 来源: 2914_ALTER SYNONYM.txt
================================================================================

-- [DDL]
CREATE USER sysadmin WITH SYSADMIN PASSWORD '********';

--创建同义词t1。
-- [DDL]
CREATE OR REPLACE SYNONYM t1 FOR ot.t1;

--创建新用户u1。
-- [DDL]
CREATE USER u1 PASSWORD '********';

--给新用户赋权限
-- [DCL_GRANT]
GRANT ALL ON SCHEMA sysadmin TO u1;

--修改同义词t1的owner为u1。
-- [DDL]
ALTER SYNONYM t1 OWNER TO u1;

--删除同义词t1。
-- [DDL]
DROP SYNONYM t1;

--收回用户u1权限
-- [DCL_REVOKE]
REVOKE ALL ON SCHEMA sysadmin FROM u1;

--删除用户u1。
-- [DDL]
DROP USER u1;

--删除用户sysadmin。
-- [DDL]
DROP USER sysadmin;


================================================================================
-- 来源: 2915_ALTER SYSTEM KILL SESSION.txt
================================================================================

-- [DQL]
SELECT sa.sessionid AS sid,0::integer AS serial#,ad.rolname AS username FROM pg_stat_get_activity(NULL) AS sa LEFT JOIN pg_authid ad ON(sa.usesysid = ad.oid)WHERE sa.application_name <> 'JobScheduler';

--结束SID为140131075880720的会话。
-- [DDL]
ALTER SYSTEM KILL SESSION '140131075880720,0' IMMEDIATE;


================================================================================
-- 来源: 2916_ALTER TABLE.txt
================================================================================

-- [DDL]
CREATE TABLE aa(c1 int, c2 int);

-- [DDL]
ALTER TABLE IF EXISTS aa RENAME TO test_alt1;

-- [DDL]
CREATE SCHEMA test_schema;

--把表test_alt1的所属模式修改为test_schema。
-- [DDL]
ALTER TABLE test_alt1 SET SCHEMA test_schema;

--查询表信息。
-- [DQL]
SELECT schemaname,tablename FROM pg_tables WHERE tablename = 'test_alt1';

-- [DDL]
CREATE USER test_user PASSWORD 'XXXXXXXXXX';

-- 修改test_alt1表的所有者为test_user;
-- [DDL]
ALTER TABLE IF EXISTS test_schema.test_alt1 OWNER TO test_user;

-- 查看
-- [DQL]
SELECT tablename, schemaname, tableowner FROM pg_tables WHERE tablename = 'test_alt1';

-- [DDL]
CREATE TABLESPACE tbs_data1 RELATIVE LOCATION 'tablespace1/tbs_data1';

--修改test_alt1表的空间为tbs_data1。
-- [DDL]
ALTER TABLE test_schema.test_alt1 SET TABLESPACE tbs_data1;

-- 查看。
-- [DQL]
SELECT tablename, tablespace FROM pg_tables WHERE tablename = 'test_alt1';

--删除。
-- [DDL]
DROP TABLE test_schema.test_alt1;

-- [DDL]
DROP TABLESPACE tbs_data1;

-- [DDL]
DROP SCHEMA test_schema;

-- [DDL]
DROP USER test_user;

-- [DDL]
CREATE TABLE test_alt2(c1 INT,c2 INT);

-- 修改列名
-- [DDL]
ALTER TABLE test_alt2 RENAME c1 TO id;

-- [DDL]
ALTER TABLE test_alt2 RENAME COLUMN c2 to areaid;

-- [DDL]
ALTER TABLE IF EXISTS test_alt2 ADD COLUMN name VARCHAR(20);

-- [DDL]
ALTER TABLE test_alt2 MODIFY name VARCHAR(50);

-- [DDL]
ALTER TABLE test_alt2 ALTER COLUMN name TYPE VARCHAR(25);

-- [DDL]
ALTER TABLE test_alt2 DROP COLUMN areaid;

--修改test_alt2表中name字段的存储模式。
-- [DDL]
ALTER TABLE test_alt2 ALTER COLUMN name SET STORAGE PLAIN;

--删除。
-- [DDL]
DROP TABLE test_alt2;

-- [DDL]
CREATE DATABASE test DBCOMPATIBILITY 'B';

-- [DDL]
CREATE TABLE tbl_test(id int, name varchar(20));

--修改tbl_test表中字段name类型，并指定位置到最前面。
-- [DDL]
ALTER TABLE tbl_test MODIFY COLUMN name varchar(25) FIRST;

--修改tbl_test字段name的类型，并指定位置在id字段的后面。
-- [DDL]
ALTER TABLE tbl_test MODIFY COLUMN name varchar(10) AFTER id;

--删除表tbl_test。
-- [DDL]
DROP TABLE tbl_test;

-- [DDL]
DROP DATABASE test;

-- [DDL]
CREATE TABLE test_alt3(pid INT, areaid CHAR(5), name VARCHAR(20));

--为pid添加非空约束。
-- [DDL]
ALTER TABLE test_alt3 MODIFY pid NOT NULL;

-- [DDL]
ALTER TABLE test_alt3 MODIFY pid NULL;

-- [DDL]
ALTER TABLE test_alt3 ALTER COLUMN areaid SET DEFAULT '00000';

-- [DDL]
ALTER TABLE test_alt3 ALTER COLUMN areaid DROP DEFAULT;

-- [DDL]
ALTER TABLE test_alt3 ADD CONSTRAINT pk_test3_pid PRIMARY KEY (pid);

-- [DDL]
CREATE TABLE test_alt4(c1 INT, c2 INT);

--建索引。
-- [DDL]
CREATE UNIQUE INDEX pk_test4_c1 ON test_alt4(c1);

--添加约束时关联已经创建的索引。
-- [DDL]
ALTER TABLE test_alt4 ADD CONSTRAINT pk_test4_c1 PRIMARY KEY USING INDEX pk_test4_c1;

--删除。
-- [DDL]
DROP TABLE test_alt4;

-- [DDL]
ALTER TABLE test_alt3 DROP CONSTRAINT IF EXISTS pk_test3_pid;

--删除。
-- [DDL]
DROP TABLE test_alt3;


================================================================================
-- 来源: 2920_ALTER TEXT SEARCH CONFIGURATION.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH CONFIGURATION english_1 (parser=default);

--增加文本搜索配置字串类型映射语法。
-- [DDL]
ALTER TEXT SEARCH CONFIGURATION english_1 ADD MAPPING FOR word WITH simple,english_stem;

--增加文本搜索配置字串类型映射语法。
-- [DDL]
ALTER TEXT SEARCH CONFIGURATION english_1 ADD MAPPING FOR email WITH english_stem, french_stem;

--查询文本搜索配置相关信息。
-- [DQL]
SELECT b.cfgname,a.maptokentype,a.mapseqno,a.mapdict,c.dictname FROM pg_ts_config_map a,pg_ts_config b, pg_ts_dict c WHERE a.mapcfg=b.oid AND a.mapdict=c.oid AND b.cfgname='english_1' ORDER BY 1,2,3,4,5;

--修改文本搜索配置字串类型映射语法。
-- [DDL]
ALTER TEXT SEARCH CONFIGURATION english_1 ALTER MAPPING REPLACE french_stem with german_stem;

--查询文本搜索配置相关信息。
-- [DQL]
SELECT b.cfgname,a.maptokentype,a.mapseqno,a.mapdict,c.dictname FROM pg_ts_config_map a,pg_ts_config b, pg_ts_dict c WHERE a.mapcfg=b.oid AND a.mapdict=c.oid AND b.cfgname='english_1' ORDER BY 1,2,3,4,5;

--删除文本搜索配置。
-- [DDL]
DROP TEXT SEARCH CONFIGURATION english_1;


================================================================================
-- 来源: 2921_ALTER TEXT SEARCH DICTIONARY.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH DICTIONARY my_dict ( TEMPLATE = Simple );

--更改Simple类型字典，将非停用词设置为已识别，其他参数保持不变。
-- [DDL]
ALTER TEXT SEARCH DICTIONARY my_dict ( Accept = true );

--更改Simple类型字典，重置Accept参数。
-- [DDL]
ALTER TEXT SEARCH DICTIONARY my_dict ( Accept );

--更新词典定义，不实际更改任何内容。
-- [DDL]
ALTER TEXT SEARCH DICTIONARY my_dict ( dummy );

--删除字典my_dict。
-- [DDL]
DROP TEXT SEARCH DICTIONARY my_dict;


================================================================================
-- 来源: 2926_ALTER VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE test_tb1(col1 INT,col2 INT);

--创建视图。
-- [DDL]
CREATE VIEW abc AS SELECT * FROM test_tb1;

--重命名视图。
-- [DDL]
ALTER VIEW IF EXISTS abc RENAME TO test_v1;

-- [DDL]
CREATE ROLE role_test PASSWORD '********';

--修改视图所有者。
-- [DDL]
ALTER VIEW IF EXISTS test_v1 OWNER TO role_test;

-- [DDL]
CREATE SCHEMA tcpds;

--修改视图所属模式。
-- [DDL]
ALTER VIEW test_v1 SET SCHEMA tcpds;

-- [DDL]
ALTER VIEW tcpds.test_v1 SET (security_barrier = TRUE);

--重置视图选项。
-- [DDL]
ALTER VIEW tcpds.test_v1 RESET (security_barrier);

--删除视图test_v1。
-- [DDL]
DROP VIEW tcpds.test_v1;

--删除表test_tb1。
-- [DDL]
DROP TABLE test_tb1;

--删除用户。
-- [DDL]
DROP ROLE role_test;

--删除schema。
-- [DDL]
DROP SCHEMA tcpds;


================================================================================
-- 来源: 2927_ANALYZE _ ANALYSE.txt
================================================================================

-- [DDL]
CREATE TABLE customer_info ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL, WR_REFUNDED_CUSTOMER_SK INTEGER );

-- [DDL]
CREATE TABLE customer_par ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL, WR_REFUNDED_CUSTOMER_SK INTEGER ) PARTITION BY RANGE(WR_RETURNED_DATE_SK) ( PARTITION P1 VALUES LESS THAN(2452275), PARTITION P2 VALUES LESS THAN(2452640), PARTITION P3 VALUES LESS THAN(2453000), PARTITION P4 VALUES LESS THAN(MAXVALUE) ) ENABLE ROW MOVEMENT;

-- [MAINTENANCE]
ANALYZE customer_info;

-- [MAINTENANCE]
ANALYZE customer_par;

-- [MAINTENANCE]
ANALYZE VERBOSE customer_info;

-- [DDL]
DROP TABLE customer_info;

-- [DDL]
DROP TABLE customer_par;


================================================================================
-- 来源: 2929_BEGIN.txt
================================================================================

-- [TCL]
BEGIN gaussdb $ # dbe_output . print_line ( 'Hello' );


================================================================================
-- 来源: 2931_CALL.txt
================================================================================

-- [DDL]
CREATE FUNCTION func_add_sql(num1 integer, num2 integer) RETURN integer AS BEGIN RETURN num1 + num2;

--按参数值传递。
-- [PLSQL]
CALL func_add_sql(1, 3);

--使用命名标记法传参。
-- [PLSQL]
CALL func_add_sql(num1 => 1,num2 => 3);

-- [PLSQL]
CALL func_add_sql(num2 := 2, num1 := 3);

--删除函数。
-- [DDL]
DROP FUNCTION func_add_sql;

--创建带出参的函数。
-- [DDL]
CREATE FUNCTION func_increment_sql(num1 IN integer, num2 IN integer, res OUT integer) RETURN integer AS BEGIN res := num1 + num2;

--出参传入常量。
-- [PLSQL]
CALL func_increment_sql(1,2,1);

--删除函数。
-- [DDL]
DROP FUNCTION func_increment_sql;


================================================================================
-- 来源: 2932_CHECKPOINT.txt
================================================================================

-- [MAINTENANCE]
CHECKPOINT;


================================================================================
-- 来源: 2933_CLEAN CONNECTION.txt
================================================================================

-- [DDL]
CREATE DATABASE test_clean_connection ;

-- [DDL]
CREATE USER jack PASSWORD '********' ;

-- [OTHER]
CLEAN CONNECTION TO ALL FOR DATABASE template1 TO USER jack ;

-- [OTHER]
CLEAN CONNECTION TO ALL TO USER jack ;

-- [OTHER]
CLEAN CONNECTION TO ALL FORCE FOR DATABASE test_clean_connection ;

-- [DDL]
DROP USER jack ;

-- [DDL]
DROP DATABASE test_clean_connection ;


================================================================================
-- 来源: 2935_CLUSTER.txt
================================================================================

-- [DDL]
CREATE TABLE test_c1 ( id int , name varchar ( 20 ));

-- [DDL]
CREATE INDEX idx_test_c1_id ON test_c1 ( id );

-- [DML_INSERT]
INSERT INTO test_c1 VALUES ( 3 , 'Joe' ),( 1 , 'Jack' ),( 2 , 'Scott' );

-- [DQL]
SELECT * FROM test_c1 ;

-- [MAINTENANCE]
CLUSTER test_c1 USING idx_test_c1_id ;

-- [DQL]
SELECT * FROM test_c1 ;

-- [DDL]
DROP TABLE test_c1 ;

-- [DDL]
CREATE TABLE test(col1 int,CONSTRAINT pk_test PRIMARY KEY (col1));

-- 第一次聚簇排序不带USING关键字报错
-- [MAINTENANCE]
CLUSTER test;

-- 聚簇排序
-- [MAINTENANCE]
CLUSTER test USING pk_test;

--对已做过聚簇的表重新进行聚簇
-- [MAINTENANCE]
CLUSTER VERBOSE test;

-- 删除
-- [DDL]
DROP TABLE test;

-- [DDL]
CREATE TABLE test_c2(id int, info varchar(4)) PARTITION BY RANGE (id)( PARTITION p1 VALUES LESS THAN (11), PARTITION p2 VALUES LESS THAN (21) );

-- [DDL]
CREATE INDEX idx_test_c2_id1 ON test_c2(id);

-- [DML_INSERT]
INSERT INTO test_c2 VALUES (6,'ABBB'),(2,'ABAB'),(9,'AAAA');

-- [DML_INSERT]
INSERT INTO test_c2 VALUES (11,'AAAB'),(19,'BBBA'),(16,'BABA');

-- 查看
-- [DQL]
SELECT * FROM test_c2;

-- 对分区p2进行聚簇排序
-- [MAINTENANCE]
CLUSTER test_c2 PARTITION (p2) USING idx_test_c2_id1;

-- 查看
-- [DQL]
SELECT * FROM test_c2;

-- 删除
-- [DDL]
DROP TABLE test_c2;


================================================================================
-- 来源: 2936_COMMENT.txt
================================================================================

-- [DDL]
CREATE TABLE emp( empno varchar(7), ename varchar(50), job varchar(50), mgr varchar(7), deptno int );

--表添加注释
-- [DDL]
COMMENT ON TABLE emp IS '部门表';

--字段添加注释
-- [DDL]
COMMENT ON COLUMN emp.empno IS '员工编号';

-- [DDL]
COMMENT ON COLUMN emp.ename IS '员工姓名';

-- [DDL]
COMMENT ON COLUMN emp.job IS '职务';

-- [DDL]
COMMENT ON COLUMN emp.mgr IS '上司编号';

-- [DDL]
COMMENT ON COLUMN emp.deptno IS '部门编号';

--删除
-- [DDL]
DROP TABLE emp;


================================================================================
-- 来源: 2937_COMMIT _ END.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建表。
-- [DDL]
CREATE TABLE tpcds. customer_demographics_t2 ( CD_DEMO_SK INTEGER NOT NULL, CD_GENDER CHAR(1) , CD_MARITAL_STATUS CHAR(1) , CD_EDUCATION_STATUS CHAR(20) , CD_PURCHASE_ESTIMATE INTEGER , CD_CREDIT_RATING CHAR(10) , CD_DEP_COUNT INTEGER , CD_DEP_EMPLOYED_COUNT INTEGER , CD_DEP_COLLEGE_COUNT INTEGER ) ;

--开启事务。
-- [TCL]
START TRANSACTION;

--插入数据。
-- [DML_INSERT]
INSERT INTO tpcds. customer_demographics_t2 VALUES(1,'M', 'U', 'DOCTOR DEGREE', 1200, 'GOOD', 1, 0, 0);

-- [DML_INSERT]
INSERT INTO tpcds. customer_demographics_t2 VALUES(2,'F', 'U', 'MASTER DEGREE', 300, 'BAD', 1, 0, 0);

--提交事务，让所有更改永久化。
-- [TCL]
COMMIT;

--查询数据。
-- [DQL]
SELECT * FROM tpcds. customer_demographics_t2;

--删除表 tpcds. customer_demographics_t2。
-- [DDL]
DROP TABLE tpcds. customer_demographics_t2;

-- 删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds;


================================================================================
-- 来源: 2938_COMMIT PREPARED.txt
================================================================================

-- [TCL]
BEGIN;

--准备标识符为的trans_test的事务。
-- [TCL]
PREPARE TRANSACTION 'trans_test';

--创建表。
-- [DDL]
CREATE TABLE item1(id int);

--提交标识符为的trans_test的事务。
-- [TCL]
COMMIT PREPARED 'trans_test';

--删除表。
-- [DDL]
DROP TABLE item1;


================================================================================
-- 来源: 2939_COPY.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建 tpcds. ship_mode表。
-- [DDL]
CREATE TABLE tpcds. ship_mode ( SM_SHIP_MODE_SK INTEGER NOT NULL, SM_SHIP_MODE_ID CHAR(16) NOT NULL, SM_TYPE CHAR(30) , SM_CODE CHAR(10) , SM_CARRIER CHAR(20) , SM_CONTRACT CHAR(20) ) ;

--向 tpcds. ship_mode表插入一条数据。
-- [DML_INSERT]
INSERT INTO tpcds. ship_mode VALUES (1,'a','b','c','d','e');

--将 tpcds. ship_mode中的数据拷贝到/home/ omm /ds_ship_mode.dat文件中。
-- [DML_COPY]
COPY tpcds. ship_mode TO '/home/ omm /ds_ship_mode.dat';

--将 tpcds. ship_mode 输出到STDOUT。
-- [DML_COPY]
COPY tpcds. ship_mode TO STDOUT;

--将 tpcds. ship_mode 的数据输出到STDOUT，使用参数如下：分隔符为','(delimiter ',')，编码格式为UTF8(encoding 'utf8')。
-- [DML_COPY]
COPY tpcds. ship_mode TO STDOUT WITH (delimiter ',', encoding 'utf8');

--将 tpcds. ship_mode 的数据输出到STDOUT，使用参数如下：导入格式为CSV（format 'CSV'），引号包围SM_SHIP_MODE_SK字段的导出内容(force_quote(SM_SHIP_MODE_SK))。
-- [DML_COPY]
COPY tpcds. ship_mode TO STDOUT WITH (format 'CSV', force_quote(SM_SHIP_MODE_SK));

--创建 tpcds. ship_mode_t1表。
-- [DDL]
CREATE TABLE tpcds. ship_mode_t1 ( SM_SHIP_MODE_SK INTEGER NOT NULL, SM_SHIP_MODE_ID CHAR(16) NOT NULL, SM_TYPE CHAR(30) , SM_CODE CHAR(10) , SM_CARRIER CHAR(20) , SM_CONTRACT CHAR(20) ) ;

--从STDIN拷贝数据到表 tpcds. ship_mode_t1。
-- [DML_COPY]
COPY tpcds. ship_mode_t1 FROM STDIN;

--从/home/ omm /ds_ship_mode.dat文件拷贝数据到表 tpcds. ship_mode_t1。
-- [DML_COPY]
COPY tpcds. ship_mode_t1 FROM '/home/ omm /ds_ship_mode.dat';

--从/home/ omm /ds_ship_mode.dat文件拷贝数据到表 tpcds. ship_mode_t1，应用TRANSFORM表达式转换，取SM_TYPE列左边10个字符插入到表中。
-- [DML_COPY]
COPY tpcds. ship_mode_t1 FROM '/home/ omm /ds_ship_mode.dat' TRANSFORM (SM_TYPE AS LEFT(SM_TYPE, 10));

--从/home/ omm /ds_ship_mode.dat文件拷贝数据到表 tpcds. ship_mode_t1，使用参数如下：导入格式为TEXT（format 'text'），分隔符为'\t'（delimiter E'\t'），忽略多余列（ignore_extra_data 'true'），不指定转义（noescaping 'true'）。
-- [DML_COPY]
COPY tpcds. ship_mode_t1 FROM '/home/ omm /ds_ship_mode.dat' WITH(format 'text', delimiter E'\t', ignore_extra_data 'true', noescaping 'true');

--从/home/ omm /ds_ship_mode.dat文件拷贝数据到表 tpcds. ship_mode_t1，使用参数如下：导入格式为FIXED（FIXED），指定定长格式（FORMATTER(SM_SHIP_MODE_SK(0, 2), SM_SHIP_MODE_ID(2,16), SM_TYPE(18,30), SM_CODE(50,10), SM_CARRIER(61,20), SM_CONTRACT(82,20))），忽略多余列（ignore_extra_data），有数据头（header）。
-- [DML_COPY]
COPY tpcds. ship_mode_t1 FROM '/home/ omm /ds_ship_mode.dat' FIXED FORMATTER(SM_SHIP_MODE_SK(0, 2), SM_SHIP_MODE_ID(2,16), SM_TYPE(18,30), SM_CODE(50,10), SM_CARRIER(61,20), SM_CONTRACT(82,20)) header ignore_extra_data;

--删除表和SCHEMA。
-- [DDL]
DROP TABLE tpcds. ship_mode;

-- [DDL]
DROP TABLE tpcds. ship_mode_t1;

-- [DDL]
DROP SCHEMA tpcds;


================================================================================
-- 来源: 2940_CREATE AGGREGATE.txt
================================================================================

-- [DDL]
CREATE OR REPLACE FUNCTION int_add(int,int) returns int as $BODY$ declare begin return $1 + $2;

-- 创建聚合函数
-- [OTHER]
CREATE AGGREGATE sum (int) ( sfunc = int_add, stype = int, initcond = '0' );

-- 创建测试表和添加数据
-- [DDL]
CREATE TABLE test_sum(a int,b int,c int);

-- [DML_INSERT]
INSERT INTO test_sum VALUES(1,2),(2,3),(3,4),(4,5);

-- 执行聚合函数
-- [DQL]
SELECT sum(a) FROM test_sum;

-- 删除聚合函数
-- [OTHER]
DROP AGGREGATE sum(int);

-- 删除自定义函数
-- [DDL]
DROP FUNCTION int_add(int,int);

-- 删除测试表
-- [DDL]
DROP TABLE test_sum;


================================================================================
-- 来源: 2941_CREATE AUDIT POLICY.txt
================================================================================

-- [DDL]
CREATE USER dev_audit PASSWORD '********' ;

-- [DDL]
CREATE USER bob_audit PASSWORD '********' ;

-- [DDL]
CREATE TABLE tb_for_audit ( col1 text , col2 text , col3 text );

-- [DDL]
CREATE RESOURCE LABEL adt_lb0 ADD TABLE ( tb_for_audit );

-- [DDL]
CREATE AUDIT POLICY adt1 PRIVILEGES CREATE ;

-- [DDL]
CREATE AUDIT POLICY adt2 ACCESS SELECT ;

-- [DDL]
CREATE AUDIT POLICY adt3 PRIVILEGES CREATE ON LABEL ( adt_lb0 ) FILTER ON ROLES ( dev_audit , bob_audit );

-- [DDL]
CREATE AUDIT POLICY adt4 ACCESS SELECT ON LABEL ( adt_lb0 ), INSERT ON LABEL ( adt_lb0 ), DELETE FILTER ON ROLES ( dev_audit , bob_audit ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' );

-- [DDL]
ALTER AUDIT POLICY adt4 REMOVE ACCESS ( SELECT ON LABEL ( adt_lb0 ));

-- [DDL]
DROP AUDIT POLICY adt1 , adt2 , adt3 , adt4 ;

-- [DDL]
DROP RESOURCE LABEL adt_lb0 ;

-- [DDL]
DROP TABLE tb_for_audit ;

-- [DDL]
DROP USER dev_audit , bob_audit ;


================================================================================
-- 来源: 2942_CREATE CAST.txt
================================================================================

-- [DDL]
CREATE OR REPLACE FUNCTION double_to_timestamp(double precision) RETURNS TIMESTAMP WITH TIME ZONE AS $$ SELECT to_timestamp($1);

--创建类型转换
-- [OTHER]
CREATE CAST(double precision AS timestamp with time zone) WITH FUNCTION double_to_timestamp(double precision) AS IMPLICIT;

--删除类型转换
-- [OTHER]
DROP CAST (double precision AS timestamp with time zone);


================================================================================
-- 来源: 2944_CREATE DATABASE.txt
================================================================================

-- [DDL]
CREATE USER jim PASSWORD '********';

--创建一个GBK编码的数据库testdb1。
-- [DDL]
CREATE DATABASE testdb1 ENCODING 'GBK' template = template0;

--查看数据库testdb1信息。
-- [DQL]
SELECT datname,pg_encoding_to_char(encoding) FROM pg_database WHERE datname = 'testdb1';

-- [DDL]
CREATE DATABASE testdb2 OWNER jim DBCOMPATIBILITY = 'A';

--查看testdb2信息。
-- [DQL]
SELECT t1.datname,t2.usename,t1.datcompatibility FROM pg_database t1,pg_user t2 WHERE t1.datname = 'testdb2' AND t1.datdba=t2.usesysid;

-- [SESSION]
SET a_format_version='10c';

-- [SESSION]
SET a_format_dev_version='s2';

--创建兼容A格式的数据库并指定时区。
-- [DDL]
CREATE DATABASE testdb3 DBCOMPATIBILITY 'A' DBTIMEZONE='+08:00';

--查看testdb3信息。
-- [DQL]
SELECT datname,datcompatibility,dattimezone FROM pg_database WHERE datname = 'testdb3';


================================================================================
-- 来源: 2945_CREATE DATABASE LINK.txt
================================================================================

-- [DDL]
CREATE USER user01 WITH SYSADMIN PASSWORD '********';

-- [SESSION]
SET ROLE user01 PASSWORD '********';

--创建私有dblink
-- [DDL]
CREATE DATABASE LINK private_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

--删除私有dblink
-- [DDL]
DROP DATABASE LINK private_dblink;

--创建公共dblink
-- [OTHER]
CREATE PUBLIC DATABASE LINK public_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

--删除公共dblink
-- [OTHER]
DROP PUBLIC DATABASE LINK public_dblink;

--删除创建出的用户
-- [SESSION]
RESET ROLE;

-- [DDL]
DROP USER user01 CASCADE;


================================================================================
-- 来源: 2946_CREATE DATA SOURCE.txt
================================================================================

-- [DDL]
CREATE DATA SOURCE ds_test1;

--创建一个Data Source对象，含TYPE信息，VERSION为NULL。
-- [DDL]
CREATE DATA SOURCE ds_test2 TYPE 'MPPDB' VERSION NULL;

--创建一个Data Source对象，仅含OPTIONS。
-- [DDL]
CREATE DATA SOURCE ds_test3 OPTIONS (dsn ' GaussDB ', encoding 'utf8');

--创建一个Data Source对象，含TYPE, VERSION, OPTIONS。
-- [DDL]
CREATE DATA SOURCE ds_test4 TYPE 'unknown' VERSION '11.2.3' OPTIONS (dsn ' GaussDB ', username 'userid', password '********', encoding '');

--删除Data Source对象。
-- [DDL]
DROP DATA SOURCE ds_test1;

-- [DDL]
DROP DATA SOURCE ds_test2;

-- [DDL]
DROP DATA SOURCE ds_test3;

-- [DDL]
DROP DATA SOURCE ds_test4;


================================================================================
-- 来源: 2947_CREATE DIRECTORY.txt
================================================================================

-- [DDL]
CREATE OR REPLACE DIRECTORY dir AS '/tmp/';

--删除目录。
-- [DDL]
DROP DIRECTORY dir;


================================================================================
-- 来源: 2948_CREATE EVENT.txt
================================================================================

-- [DDL]
CREATE DATABASE test_event WITH DBCOMPATIBILITY = 'b';

-- [DDL]
CREATE TABLE t_ev(num int);

--创建一个执行一次的定时任务。
-- [OTHER]
CREATE EVENT IF NOT EXISTS event_e1 ON SCHEDULE AT sysdate + interval 5 second + interval 33 minute DISABLE DO insert into t_ev values(0);

--创建一个每隔一分钟执行一次的定时任务。
-- [OTHER]
CREATE EVENT IF NOT EXISTS event_e2 ON SCHEDULE EVERY 1 minute DO insert into t_ev values(1);

--修改定时任务状态和待执行语句。
-- [OTHER]
ALTER EVENT event_e1 ENABLE DO select 1;

--修改定时任务名。
-- [OTHER]
ALTER EVENT event_e1 RENAME TO event_ee;

--查看定时任务。
-- [SESSION]
SHOW EVENTS;

--删除定时任务。
-- [OTHER]
DROP EVENT event_e1;

-- [OTHER]
DROP EVENT event_e2;

--删除表。
-- [DDL]
DROP TABLE t_ev;

-- [DDL]
DROP DATABASE test_event;


================================================================================
-- 来源: 2949_CREATE EXTENSION.txt
================================================================================

-- [DDL]
CREATE EXTENSION IF NOT EXISTS security_plugin;

-- [DDL]
DROP EXTENSION security_plugin;


================================================================================
-- 来源: 2950_CREATE FOREIGN TABLE.txt
================================================================================

-- [DDL]
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--创建外表
-- [DDL]
CREATE FOREIGN TABLE foreign_tbl (col1 text) SERVER my_server OPTIONS (logtype 'pg_log');

--删除外表
-- [DDL]
DROP FOREIGN TABLE foreign_tbl;

--删除server
-- [DDL]
DROP SERVER my_server;


================================================================================
-- 来源: 2951_CREATE FUNCTION.txt
================================================================================

-- [DDL]
CREATE FUNCTION func_add_sql(integer, integer) RETURNS integer AS 'select $1 + $2;

--利用参数名用 plpgsql 自增一个整数。
-- [DDL]
CREATE OR REPLACE FUNCTION func_increment_plsql(i integer) RETURNS integer AS $$ BEGIN RETURN i + 1;

--返回RECORD类型
-- [DDL]
CREATE OR REPLACE FUNCTION func_increment_sql(i int, out result_1 bigint, out result_2 bigint) RETURNS SETOF RECORD AS $$ BEGIN result_1 = i + 1;

--返回一个包含多个输出参数的记录。
-- [DDL]
CREATE FUNCTION func_dup_sql(in int, out f1 int, out f2 text) AS $$ SELECT $1, CAST($1 AS text) || ' is text' $$ LANGUAGE SQL;

-- 调用func_dup_sql函数
-- [DQL]
SELECT * FROM func_dup_sql(42);

--计算两个整数的和，并返回结果。如果输入为null，则返回null。
-- [DDL]
CREATE FUNCTION func_add_sql2(num1 integer, num2 integer) RETURN integer AS BEGIN RETURN num1 + num2;

--修改函数func_add_sql2的执行规则为IMMUTABLE，即参数不变时返回相同结果。
-- [DDL]
ALTER FUNCTION func_add_sql2(INTEGER, INTEGER) IMMUTABLE;

--将函数func_add_sql2的名称修改为add_two_number。
-- [DDL]
ALTER FUNCTION func_add_sql2(INTEGER, INTEGER) RENAME TO add_two_number;

--创建jim用户。
-- [DDL]
CREATE USER jim PASSWORD '********';

--将函数add_two_number的所有者改为 jim 。
-- [DDL]
ALTER FUNCTION add_two_number(INTEGER, INTEGER) OWNER TO jim ;

--删除函数。
-- [DDL]
DROP FUNCTION func_add_sql;

-- [DDL]
DROP FUNCTION func_increment_plsql;

-- [DDL]
DROP FUNCTION func_increment_sql;

-- [DDL]
DROP FUNCTION func_dup_sql;

-- [DDL]
DROP FUNCTION add_two_number;

--删除jim用户
-- [DDL]
DROP USER jim;

--设置参数
-- [SESSION]
SET behavior_compat_options='proc_outparam_override';

--创建函数
-- [DDL]
CREATE OR REPLACE FUNCTION func1(in a integer, out b integer) RETURNS int AS $$ DECLARE c int;

--同时返回return和出参
-- [PLSQL]
DECLARE result integer;

--不支持左赋值表达式
-- [PLSQL]
DECLARE result integer;

--存储过程中不支持out/inout传入常量
-- [PLSQL]
DECLARE result integer;

--存储过程中支持out/inout传入变量
-- [PLSQL]
DECLARE result integer;

--删除函数func
-- [DDL]
DROP FUNCTION func1;

-- 不打开参数set behavior_compat_options = 'proc_outparam_override'时，被匿名块或存储过程直接调用的函数的OUT、IN OUT出参不能使用复合类型，并且RETURN值会被当做OUT出参的第一个值导致调用失败
-- [DDL]
CREATE TYPE rec as(c1 int, c2 int);

-- [DDL]
CREATE OR REPLACE FUNCTION func(a in out rec, b in out int) RETURN int AS BEGIN a.c1:=100;

-- [PLSQL]
DECLARE r rec;

-- [DDL]
DROP FUNCTION func;

-- [DDL]
DROP TYPE rec;

--以下示例只有当数据库兼容模式为A时可以执行
-- [DDL]
CREATE OR REPLACE PACKAGE pkg_type AS type table_of_index_int is table of integer index by integer;

--创建一个返回table of integer index by integer类型结果的函数
-- [DDL]
CREATE OR REPLACE FUNCTION func_001(a in out pkg_type.table_of_index_int, b in out pkg_type.table_of_index_var) --#add in & inout #defult value RETURN pkg_type.table_of_index_int AS table_of_index_int_val pkg_type.table_of_index_int;

-- [PLSQL]
DECLARE table_of_index_int_val pkg_type.table_of_index_int;

--创建一个含有IN/OUT类型参数的函数
-- [DDL]
CREATE OR REPLACE FUNCTION func_001(a in out date, b in out date) --#add in & inout #defult value RETURN integer AS BEGIN raise info '%', a;

-- [PLSQL]
DECLARE date1 date := '2022-02-02';

--创建一个含有IN/OUT类型参数的函数
-- [DDL]
CREATE OR REPLACE FUNCTION func_001(a in out INT, b in out date) --#add in & inout #defult value RETURN INT AS BEGIN raise info '%', a;

-- [PLSQL]
DECLARE date1 int := 1;

--删除函数
-- [DDL]
DROP FUNCTION func_001;

--删除package
-- [DDL]
DROP PACKAGE pkg_type;


================================================================================
-- 来源: 2953_CREATE GROUP.txt
================================================================================

-- [DDL]
CREATE GROUP super_users WITH PASSWORD "********";

--创建用户。
-- [DDL]
CREATE ROLE lche WITH PASSWORD "********";

--创建用户。
-- [DDL]
CREATE ROLE jim WITH PASSWORD "********";

--向用户组中添加用户。
-- [DDL]
ALTER GROUP super_users ADD USER lche, jim;

--从用户组中删除用户。
-- [DDL]
ALTER GROUP super_users DROP USER jim;

--修改用户组的名称。
-- [DDL]
ALTER GROUP super_users RENAME TO normal_users;

--删除用户。
-- [DDL]
DROP ROLE lche, jim;

--删除用户组。
-- [DDL]
DROP GROUP normal_users;


================================================================================
-- 来源: 2954_CREATE INCREMENTAL MATERIALIZED VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建增量物化视图。
-- [DDL]
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--基表写入数据。
-- [DML_INSERT]
INSERT INTO my_table VALUES(1,1),(2,2);

--对增量物化视图my_imv进行增量刷新。
-- [OTHER]
REFRESH INCREMENTAL MATERIALIZED VIEW my_imv;

--删除增量物化视图。
-- [DDL]
DROP MATERIALIZED VIEW my_imv;

--删除普通表my_table。
-- [DDL]
DROP TABLE my_table;


================================================================================
-- 来源: 2955_CREATE INDEX.txt
================================================================================

-- [DDL]
CREATE TABLE tbl_test1( id int, --用户id name varchar(50), --用户姓名 postcode char(6) --邮编 );

--创建表空间tbs_index1。
-- [DDL]
CREATE TABLESPACE tbs_index1 RELATIVE LOCATION 'test_tablespace/tbs_index1';

--为表tbl_test1创建索引idx_test1指定表空间。
-- [DDL]
CREATE INDEX idx_test1 ON tbl_test1(name) TABLESPACE tbs_index1;

--查询索引idx_test1信息。
-- [DQL]
SELECT indexname,tablename,tablespace FROM pg_indexes WHERE indexname = 'idx_test1';

--删除索引。
-- [DDL]
DROP INDEX idx_test1;

--删除表空间
-- [DDL]
DROP TABLESPACE tbs_index1;

-- [DDL]
CREATE UNIQUE INDEX idx_test2 ON tbl_test1(id);

--删除索引。
-- [DDL]
DROP INDEX idx_test2;

-- [DDL]
CREATE INDEX idx_test3 ON tbl_test1(substr(postcode,2));

--删除索引。
-- [DDL]
DROP INDEX idx_test3;

-- [DDL]
CREATE INDEX idx_test4 ON tbl_test1(id) WHERE id IS NOT NULL;

-- 删除索引。
-- [DDL]
DROP INDEX idx_test4;

-- 删除表
-- [DDL]
DROP TABLE tbl_test1;

-- [DDL]
CREATE TABLE student(id int, name varchar(20)) PARTITION BY RANGE (id) ( PARTITION p1 VALUES LESS THAN (200), PARTITION pmax VALUES LESS THAN (MAXVALUE) );

--创建LOCAL分区索引不指定索引分区的名称。
-- [DDL]
CREATE INDEX idx_student1 ON student(id) LOCAL;

--查看索引分区信息，发现LOC索引分区数和表的分区数一致。
-- [DQL]
SELECT relname FROM pg_partition WHERE parentid = 'idx_student1'::regclass;

--删除LOCAL分区索引。
-- [DDL]
DROP INDEX idx_student1;

--创建GLOBAL索引。
-- [DDL]
CREATE INDEX idx_student2 ON student(name) GLOBAL;

--查看索引分区信息，发现GLOBAL索引分区数和表的分区数不一致。
-- [DQL]
SELECT relname FROM pg_partition WHERE parentid = 'idx_student2'::regclass;

--删除GLOBAL分区索引。
-- [DDL]
DROP INDEX idx_student2;

--删除表。
-- [DDL]
DROP TABLE student;


================================================================================
-- 来源: 2957_CREATE MASKING POLICY.txt
================================================================================

-- [DDL]
CREATE USER dev_mask PASSWORD '********' ;

-- [DDL]
CREATE USER bob_mask PASSWORD '********' ;

-- [DDL]
CREATE TABLE tb_for_masking ( idx int , col1 text , col2 text , col3 text , col4 text , col5 text , col6 text , col7 text , col8 text );

-- [DML_INSERT]
INSERT INTO tb_for_masking VALUES ( 1 , '9876543210' , 'usr321usr' , 'abc@huawei.com' , 'abc@huawei.com' , '1234-4567-7890-0123' , 'abcdef 123456 ui 323 jsfd321 j3k2l3' , '4880-9898-4545-2525' , 'this is a llt case' );

-- [DML_INSERT]
INSERT INTO tb_for_masking VALUES ( 2 , '0123456789' , 'lltc123llt' , 'abc@gmail.com' , 'abc@gmail.com' , '9876-5432-1012-3456' , '1234 abcd ef 56 gh78ijk90lm' , '4856-7654-1234-9865' , 'this,is.a!LLT?case' );

-- [DDL]
CREATE RESOURCE LABEL mask_lb1 ADD COLUMN ( tb_for_masking . col1 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb2 ADD COLUMN ( tb_for_masking . col2 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb3 ADD COLUMN ( tb_for_masking . col3 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb4 ADD COLUMN ( tb_for_masking . col4 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb5 ADD COLUMN ( tb_for_masking . col5 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb6 ADD COLUMN ( tb_for_masking . col6 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb7 ADD COLUMN ( tb_for_masking . col7 );

-- [DDL]
CREATE RESOURCE LABEL mask_lb8 ADD COLUMN ( tb_for_masking . col8 );

-- [DDL]
CREATE MASKING POLICY maskpol1 maskall ON LABEL ( mask_lb1 );

-- [DDL]
CREATE MASKING POLICY maskpol2 alldigitsmasking ON LABEL ( mask_lb2 );

-- [DDL]
CREATE MASKING POLICY maskpol3 basicemailmasking ON LABEL ( mask_lb3 );

-- [DDL]
CREATE MASKING POLICY maskpol4 fullemailmasking ON LABEL ( mask_lb4 );

-- [DDL]
CREATE MASKING POLICY maskpol5 creditcardmasking ON LABEL ( mask_lb5 );

-- [DDL]
CREATE MASKING POLICY maskpol6 shufflemasking ON LABEL ( mask_lb6 );

-- [DDL]
CREATE MASKING POLICY maskpol7 regexpmasking ( '[\d+]' , '*' , 2 , 9 ) ON LABEL ( mask_lb7 );

-- [DDL]
CREATE MASKING POLICY maskpol8 randommasking ON LABEL ( mask_lb8 ) FILTER ON ROLES ( dev_mask , bob_mask ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' );

-- [DQL]
SELECT * FROM tb_for_masking ;

-- [DCL_GRANT]
GRANT ALL PRIVILEGES TO dev_mask ;

-- [DCL_GRANT]
GRANT ALL PRIVILEGES TO bob_mask ;

-- [SESSION]
SET role dev_mask PASSWORD '********' ;

-- [DQL]
SELECT col8 FROM tb_for_masking ;

-- [SESSION]
SET role bob_mask PASSWORD '********' ;

-- [DQL]
SELECT col8 FROM tb_for_masking ;

-- [DDL]
DROP MASKING POLICY maskpol1 , maskpol2 , maskpol3 , maskpol4 , maskpol5 , maskpol6 , maskpol7 , maskpol8 ;

-- [DDL]
DROP RESOURCE LABEL mask_lb1 , mask_lb2 , mask_lb3 , mask_lb4 , mask_lb5 , mask_lb6 , mask_lb7 , mask_lb8 ;

-- [DDL]
DROP TABLE tb_for_masking ;

-- [DDL]
DROP USER dev_mask , bob_mask ;


================================================================================
-- 来源: 2958_CREATE MATERIALIZED VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建全量物化视图。
-- [DDL]
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--基表写入数据。
-- [DML_INSERT]
INSERT INTO my_table VALUES(1,1),(2,2);

--对全量物化视图my_mv进行全量刷新。
-- [OTHER]
REFRESH MATERIALIZED VIEW my_mv;

--删除全量物化视图。
-- [DDL]
DROP MATERIALIZED VIEW my_mv;

--删除普通表my_table。
-- [DDL]
DROP TABLE my_table;


================================================================================
-- 来源: 2959_CREATE MODEL.txt
================================================================================

-- [DDL]
CREATE TABLE houses ( id INTEGER, tax INTEGER, bedroom INTEGER, bath DOUBLE PRECISION, price INTEGER, size INTEGER, lot INTEGER, mark text );

--插入训练数据
-- [DML_INSERT]
INSERT INTO houses(id, tax, bedroom, bath, price, size, lot, mark) VALUES (1,590,2,1,50000,770,22100,'a+'), (2,1050,3,2,85000,1410,12000,'a+'), (3,20,2,1,22500,1060,3500,'a-'), (4,870,2,2,90000,1300,17500,'a+'), (5,1320,3,2,133000,1500,30000,'a+'), (6,1350,2,1,90500,850,25700,'a-'), (7,2790,3,2.5,260000,2130,25000,'a+'), (8,680,2,1,142500,1170,22000,'a-'), (9,1840,3,2,160000,1500,19000,'a+'), (10,3680,4,2,240000,2790,20000,'a-'), (11,1660,3,1,87000,1030,17500,'a+'), (12,1620,3,2,118500,1250,20000,'a-'), (13,3100,3,2,140000,1760,38000,'a+'), (14,2090,2,3,148000,1550,14000,'a-'), (15,650,3,1.5,65000,1450,12000,'a-');

--训练模型
-- [DDL]
CREATE MODEL price_model USING logistic_regression FEATURES size, lot TARGET mark FROM HOUSES WITH learning_rate=0.88, max_iterations=default;

--删除模型
-- [DDL]
DROP MODEL price_model;

--删除表
-- [DDL]
DROP TABLE houses;


================================================================================
-- 来源: 2962_CREATE PACKAGE.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PACKAGE emp_bonus IS var1 INT:=1;

-- [DDL]
DROP TABLE IF EXISTS test1;

-- [DDL]
CREATE OR REPLACE PACKAGE BODY emp_bonus IS var3 INT:=3;

-- [DDL]
ALTER PACKAGE emp_bonus OWNER TO omm;

--将PACKAGE emp_bonus的所属者改为omm 调用PACKAGE示例
-- [PLSQL]
CALL emp_bonus.testpro1(1);

-- [DDL]
DROP TABLE IF EXISTS test1;

-- [DQL]
SELECT emp_bonus.testpro1(1);

-- [DDL]
DROP TABLE IF EXISTS test1;

--匿名块里调用PACKAGE存储过程
-- [TCL]
BEGIN emp_bonus.testpro1(1);

-- [DDL]
DROP TABLE IF EXISTS test1;

-- [DDL]
DROP PACKAGE emp_bonus;


================================================================================
-- 来源: 2963_CREATE PROCEDURE.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE prc_add ( param1 IN INTEGER , param2 IN OUT INTEGER ) AS BEGIN param2 : = param1 + param2 ;

-- [DQL]
SELECT prc_add ( 2 , 3 );

-- [DDL]
CREATE OR REPLACE PROCEDURE pro_variadic ( var1 VARCHAR2 ( 10 ) DEFAULT 'hello!' , var4 VARIADIC int4 []) AS BEGIN dbe_output . print_line ( var1 );

-- [DQL]
SELECT pro_variadic ( var1 => 'hello' , VARIADIC var4 => array [ 1 , 2 , 3 , 4 ]);

-- [DDL]
CREATE TABLE tb1 ( a integer );

-- [DDL]
CREATE PROCEDURE insert_data ( v integer ) SECURITY INVOKER AS BEGIN INSERT INTO tb1 VALUES ( v );

-- [PLSQL]
CALL insert_data ( 1 );

-- [DDL]
CREATE OR REPLACE PROCEDURE package_func_overload ( col int , col2 out varchar ) package as declare col_type text ;

-- [DDL]
DROP PROCEDURE prc_add ;

-- [DDL]
DROP PROCEDURE pro_variadic ;

-- [DDL]
DROP PROCEDURE insert_data ;

-- [DDL]
DROP PROCEDURE package_func_overload ;

-- [DDL]
DROP TABLE tb1 ;


================================================================================
-- 来源: 2964_CREATE PUBLICATION.txt
================================================================================

-- [DDL]
CREATE TABLE users (c1 int, c2 int);

-- [DDL]
CREATE TABLE departments (c1 int, c2 int);

-- [DDL]
CREATE TABLE mydata (c1 int, c2 int);

-- [DDL]
CREATE TABLE mydata2 (c1 int, c2 int);

--创建一个发布，发布两个表中所有更改。
-- [OTHER]
CREATE PUBLICATION mypublication FOR TABLE users, departments;

--创建一个发布，发布所有表中的所有更改。
-- [OTHER]
CREATE PUBLICATION alltables FOR ALL TABLES;

--创建一个发布，只发布一个表中的INSERT操作。
-- [OTHER]
CREATE PUBLICATION insert_only FOR TABLE mydata WITH (publish = 'insert');

--修改发布的动作。
-- [OTHER]
ALTER PUBLICATION insert_only SET (publish='insert,update,delete');

--向发布中添加表。
-- [OTHER]
ALTER PUBLICATION insert_only ADD TABLE mydata2;

--删除发布。
-- [OTHER]
DROP PUBLICATION insert_only;

-- [OTHER]
DROP PUBLICATION alltables;

-- [OTHER]
DROP PUBLICATION mypublication;

--删除表。
-- [DDL]
DROP TABLE users;

-- [DDL]
DROP TABLE departments;

-- [DDL]
DROP TABLE mydata;

-- [DDL]
DROP TABLE mydata2;


================================================================================
-- 来源: 2965_CREATE RESOURCE LABEL.txt
================================================================================

-- [DDL]
CREATE TABLE tb_for_label ( col1 text , col2 text , col3 text );

-- [DDL]
CREATE SCHEMA schema_for_label ;

-- [DDL]
CREATE VIEW view_for_label AS SELECT 1 ;

-- [DDL]
CREATE FUNCTION func_for_label RETURNS TEXT AS $$ SELECT col1 FROM tb_for_label ;

-- [DDL]
CREATE RESOURCE LABEL IF NOT EXISTS table_label add TABLE ( public . tb_for_label );

-- [DDL]
CREATE RESOURCE LABEL IF NOT EXISTS column_label add COLUMN ( public . tb_for_label . col1 );

-- [DDL]
CREATE RESOURCE LABEL IF NOT EXISTS schema_label add SCHEMA ( schema_for_label );

-- [DDL]
CREATE RESOURCE LABEL IF NOT EXISTS view_label add VIEW ( view_for_label );

-- [DDL]
CREATE RESOURCE LABEL IF NOT EXISTS func_label add FUNCTION ( func_for_label );

-- [DDL]
DROP RESOURCE LABEL func_label , view_label , schema_label , column_label , table_label ;

-- [DDL]
DROP FUNCTION func_for_label ;

-- [DDL]
DROP VIEW view_for_label ;

-- [DDL]
DROP SCHEMA schema_for_label ;

-- [DDL]
DROP TABLE tb_for_label ;


================================================================================
-- 来源: 2966_CREATE RESOURCE POOL.txt
================================================================================

-- [DDL]
CREATE RESOURCE POOL pool1 ;

-- [DDL]
CREATE RESOURCE POOL pool2 WITH ( CONTROL_GROUP = "High" );

-- [DDL]
CREATE RESOURCE POOL pool3 WITH ( CONTROL_GROUP = "class1:Low" );

-- [DDL]
CREATE RESOURCE POOL pool4 WITH ( CONTROL_GROUP = "class1:wg1" );

-- [DDL]
CREATE RESOURCE POOL pool5 WITH ( CONTROL_GROUP = "class1:wg2:3" );

-- [DDL]
DROP RESOURCE POOL pool1 ;

-- [DDL]
DROP RESOURCE POOL pool2 ;

-- [DDL]
DROP RESOURCE POOL pool3 ;

-- [DDL]
DROP RESOURCE POOL pool4 ;

-- [DDL]
DROP RESOURCE POOL pool5 ;


================================================================================
-- 来源: 2967_CREATE ROLE.txt
================================================================================

-- [DDL]
CREATE ROLE manager IDENTIFIED BY ' ******** ';

--创建一个角色，从2015年1月1日开始生效，到2026年1月1日失效。
-- [DDL]
CREATE ROLE miriam WITH LOGIN PASSWORD ' ******** ' VALID BEGIN '2015-01-01' VALID UNTIL '2026-01-01';

--修改角色manager的密码为********。
-- [DDL]
ALTER ROLE manager IDENTIFIED BY '********' REPLACE ' ********** ';

--修改角色manager为系统管理员。
-- [DDL]
ALTER ROLE manager SYSADMIN;

--删除角色manager。
-- [DDL]
DROP ROLE manager;

--删除角色miriam。
-- [DDL]
DROP GROUP miriam;


================================================================================
-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY.txt
================================================================================

-- [DDL]
CREATE USER alice PASSWORD '********';

--创建用户bob。
-- [DDL]
CREATE USER bob PASSWORD '********';

--创建数据表all_data。
-- [DDL]
CREATE TABLE public.all_data(id int, role varchar(100), data varchar(100));

--向数据表插入数据。
-- [DML_INSERT]
INSERT INTO all_data VALUES(1, 'alice', 'alice data');

-- [DML_INSERT]
INSERT INTO all_data VALUES(2, 'bob', 'bob data');

-- [DML_INSERT]
INSERT INTO all_data VALUES(3, 'peter', 'peter data');

--将表all_data的读取权限赋予alice和bob用户。
-- [DCL_GRANT]
GRANT SELECT ON all_data TO alice, bob;

--打开行访问控制策略开关。
-- [DDL]
ALTER TABLE all_data ENABLE ROW LEVEL SECURITY;

--创建行访问控制策略，当前用户只能查看用户自身的数据。
-- [OTHER]
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING(role = CURRENT_USER);

--当前用户执行SELECT操作
-- [DQL]
SELECT * FROM all_data;

-- [EXPLAIN]
EXPLAIN(COSTS OFF) SELECT * FROM all_data;

--切换至alice用户执行SELECT操作。
-- [DQL]
SELECT * FROM all_data;

-- [EXPLAIN]
EXPLAIN(COSTS OFF) SELECT * FROM all_data;

--删除行访问控制策略。
-- [OTHER]
DROP ROW LEVEL SECURITY POLICY all_data_rls ON all_data;

--删除数据表all_data。
-- [DDL]
DROP TABLE public.all_data;

--删除用户alice, bob。
-- [DDL]
DROP USER alice, bob;


================================================================================
-- 来源: 2969_CREATE RULE.txt
================================================================================

-- [DDL]
CREATE TABLE def_test ( c1 int4 DEFAULT 5, c2 text DEFAULT 'initial_default' );

-- [DDL]
CREATE VIEW def_view_test AS SELECT * FROM def_test;

--创建RULE def_view_test_ins。
-- [OTHER]
CREATE RULE def_view_test_ins AS ON INSERT TO def_view_test DO INSTEAD INSERT INTO def_test SELECT new.*;

--删除RULE def_view_test_ins。
-- [OTHER]
DROP RULE def_view_test_ins ON def_view_test;

--删除表def_test、视图def_view_test。
-- [DDL]
DROP VIEW def_view_test;

-- [DDL]
DROP TABLE def_test;


================================================================================
-- 来源: 2970_CREATE SCHEMA.txt
================================================================================

-- [DDL]
CREATE DATABASE test1 WITH DBCOMPATIBILITY = 'b';

--创建一个角色role1。
-- [DDL]
CREATE ROLE role1 IDENTIFIED BY ' ******** ';

-- 为用户role1创建一个同名schema，子命令创建的表films和winners的拥有者为role1。
-- [DDL]
CREATE SCHEMA AUTHORIZATION role1 CREATE TABLE films (title text, release date, awards text[]) CREATE VIEW winners AS SELECT title, release FROM films WHERE awards IS NOT NULL;

-- 创建一个schema ds，指定schema的默认字符集为utf8mb4，默认字符序为utf8mb4_bin。仅在B模式下（即sql_compatibility='B'）支持该语法。
-- [DDL]
CREATE SCHEMA ds CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;

--删除schema。
-- [DDL]
DROP SCHEMA role1 CASCADE;

-- [DDL]
DROP SCHEMA ds CASCADE;

--删除用户。
-- [DDL]
DROP USER role1 CASCADE;

-- [DDL]
DROP DATABASE test1;


================================================================================
-- 来源: 2971_CREATE SECURITY LABEL.txt
================================================================================

-- [DDL]
CREATE SECURITY LABEL sec_label 'L1:G4' ;

-- [DDL]
DROP SECURITY LABEL sec_label ;


================================================================================
-- 来源: 2972_CREATE SEQUENCE.txt
================================================================================

-- [DDL]
CREATE SEQUENCE seq1 START 101 INCREMENT 10 ;

-- [DQL]
SELECT nextval ( 'seq1' );

-- [DQL]
SELECT nextval ( 'seq1' );

-- [DDL]
DROP SEQUENCE seq1 ;

-- [DDL]
CREATE TABLE test1 ( id int PRIMARY KEY , name varchar ( 20 ));

-- [DDL]
CREATE SEQUENCE test_seq2 START 1 NO CYCLE OWNED BY test1 . id ;

-- [DDL]
ALTER TABLE test1 ALTER COLUMN id SET DEFAULT nextval ( 'test_seq2' :: regclass );

-- [DML_INSERT]
INSERT INTO test1 ( name ) values ( 'Joe' ),( 'Scott' ),( 'Ben' );

-- [DQL]
SELECT * FROM test1 ;

-- [DDL]
DROP SEQUENCE test_seq2 CASCADE ;

-- [DDL]
DROP TABLE test1 ;


================================================================================
-- 来源: 2973_CREATE SERVER.txt
================================================================================

-- [DDL]
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--删除my_server。
-- [DDL]
DROP SERVER my_server;


================================================================================
-- 来源: 2974_CREATE SUBSCRIPTION.txt
================================================================================

-- [DDL]
CREATE TABLE users (c1 int, c2 int);

-- [DDL]
CREATE TABLE departments (c1 int, c2 int);

-- [DDL]
CREATE TABLE mydata (c1 int, c2 int);

--创建一个发布，发布两个表中所有更改。
-- [OTHER]
CREATE PUBLICATION mypublication FOR TABLE users, departments;

--创建一个发布，只发布一个表中的INSERT操作。
-- [OTHER]
CREATE PUBLICATION insert_only FOR TABLE mydata WITH (publish = 'insert');

--创建一个到远程服务器的订阅，复制发布mypublication和insert_only中的表，并在提交时立即开始复制。其中ip和port为发布端的ip地址和端口，user、dbname、password为发布端数据库的用户信息。
-- [OTHER]
CREATE SUBSCRIPTION mysub CONNECTION 'host=192.168.11.11 port=5432 user=foo dbname=foodb password=********' PUBLICATION mypublication, insert_only;

--创建一个到远程服务器的订阅，复制insert_only发布中的表， 并且不开始复制直到稍后启用复制。其中ip和port为发布端的ip地址和端口，user、dbname、password为发布端数据库的用户信息。
-- [OTHER]
CREATE SUBSCRIPTION mysub CONNECTION 'host=192.168.11.11 port=5432 user=foo dbname=foodb password=********' PUBLICATION insert_only WITH (enabled = false);

--修改订阅的连接信息。其中ip和port为发布端的ip地址和端口，user、dbname、password为发布端数据库的用户信息。
-- [OTHER]
ALTER SUBSCRIPTION mysub CONNECTION 'host=192.168.11.11 port=5432 user=foo dbname=foodb password=********';

--激活订阅。
-- [OTHER]
ALTER SUBSCRIPTION mysub SET(enabled=true);

--删除订阅。
-- [OTHER]
DROP SUBSCRIPTION mysub;

--删除发布。
-- [OTHER]
DROP PUBLICATION insert_only;

-- [OTHER]
DROP PUBLICATION mypublication;

--删除表。
-- [DDL]
DROP TABLE users;

-- [DDL]
DROP TABLE departments;

-- [DDL]
DROP TABLE mydata;


================================================================================
-- 来源: 2975_CREATE SYNONYM.txt
================================================================================

-- [DDL]
CREATE SCHEMA ot;

--创建表ot.t1及其同义词t1。
-- [DDL]
CREATE TABLE ot.t1(id int, name varchar2(10));

-- [DDL]
CREATE OR REPLACE SYNONYM t1 FOR ot.t1;

--使用同义词t1。
-- [DQL]
SELECT * FROM t1;

-- [DML_INSERT]
INSERT INTO t1 VALUES (1, 'ada'), (2, 'bob');

-- [DML_UPDATE]
UPDATE t1 SET t1.name = 'cici' WHERE t1.id = 2;

--创建同义词v1及其关联视图ot.v_t1。
-- [DDL]
CREATE SYNONYM v1 FOR ot.v_t1;

-- [DDL]
CREATE VIEW ot.v_t1 AS SELECT * FROM ot.t1;

--使用同义词v1。
-- [DQL]
SELECT * FROM v1;

--创建重载函数ot.add及其同义词add。
-- [DDL]
CREATE OR REPLACE FUNCTION ot.add(a integer, b integer) RETURNS integer AS $$ SELECT $1 + $2 $$ LANGUAGE sql;

-- [DDL]
CREATE OR REPLACE FUNCTION ot.add(a decimal(5,2), b decimal(5,2)) RETURNS decimal(5,2) AS $$ SELECT $1 + $2 $$ LANGUAGE sql;

-- [DDL]
CREATE OR REPLACE SYNONYM add FOR ot.add;

--使用同义词add。
-- [DQL]
SELECT add(1,2);

-- [DQL]
SELECT add(1.2,2.3);

--创建存储过程ot.register及其同义词register。
-- [DDL]
CREATE PROCEDURE ot.register(n_id integer, n_name varchar2(10)) SECURITY INVOKER AS BEGIN INSERT INTO ot.t1 VALUES(n_id, n_name);

-- [DDL]
CREATE OR REPLACE SYNONYM register FOR ot.register;

--使用同义词register，调用存储过程。
-- [PLSQL]
CALL register(3,'mia');

--删除同义词。
-- [DDL]
DROP SYNONYM t1;

-- [DDL]
DROP SYNONYM IF EXISTS v1;

-- [DDL]
DROP SYNONYM IF EXISTS add;

-- [DDL]
DROP SYNONYM register;

-- [DDL]
DROP SCHEMA ot CASCADE;


================================================================================
-- 来源: 2976_CREATE TABLE.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t1 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- [DDL]
CREATE TABLE tpcds. warehouse_t2 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60), W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- [DDL]
DROP TABLE tpcds.warehouse_t2;

-- [DDL]
DROP TABLE tpcds.warehouse_t1;

-- [DDL]
DROP SCHEMA tpcds;

-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t3 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) DEFAULT 'GA', W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- [DDL]
DROP TABLE tpcds.warehouse_t3;

-- [DDL]
DROP SCHEMA tpcds;

--创建表，并在事务结束时检查W_WAREHOUSE_NAME字段是否有重复。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t4 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) UNIQUE DEFERRABLE, W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- [DDL]
DROP TABLE tpcds.warehouse_t4;

-- [DDL]
DROP SCHEMA tpcds;

-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t5 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2), UNIQUE(W_WAREHOUSE_NAME) WITH(fillfactor=70) );

-- [DDL]
DROP TABLE tpcds.warehouse_t5;

-- [DDL]
DROP SCHEMA tpcds;

--或者用下面的语法。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t6 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) UNIQUE, W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) ) WITH(fillfactor=70);

-- [DDL]
DROP TABLE tpcds.warehouse_t6;

-- [DDL]
DROP SCHEMA tpcds;

--创建表，并指定该表数据不写入预写日志。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE UNLOGGED TABLE tpcds. warehouse_t7 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- [DDL]
DROP TABLE tpcds.warehouse_t7;

-- [DDL]
DROP SCHEMA tpcds;

--创建表临时表。
-- [DDL]
CREATE TEMPORARY TABLE warehouse_t24 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

--创建本地临时表，并指定提交事务时删除该临时表数据。
-- [DDL]
CREATE TEMPORARY TABLE warehouse_t25 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) ) ON COMMIT DELETE ROWS;

--创建全局临时表，并指定会话结束时删除该临时表数据，当前Ustore存储引擎不支持全局临时表。
-- [DDL]
CREATE GLOBAL TEMPORARY TABLE gtt1 ( ID INTEGER NOT NULL, NAME CHAR(16) NOT NULL, ADDRESS VARCHAR(50) , POSTCODE CHAR(6) ) ON COMMIT PRESERVE ROWS;

--创建表时，不希望因为表已存在而报错。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE IF NOT EXISTS tpcds. warehouse_t8 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- [DDL]
DROP TABLE tpcds.warehouse_t8;

-- [DDL]
DROP SCHEMA tpcds;

--创建普通表空间。
-- [DDL]
CREATE TABLESPACE DS_TABLESPACE1 RELATIVE LOCATION 'tablespace/tablespace_1';

--创建表时，指定表空间。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t9 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) ) TABLESPACE DS_TABLESPACE1;

-- [DDL]
DROP TABLE tpcds.warehouse_t9;

-- [DDL]
DROP SCHEMA tpcds;

--创建表时，单独指定W_WAREHOUSE_NAME的索引表空间。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t10 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) UNIQUE USING INDEX TABLESPACE DS_TABLESPACE1, W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- [DDL]
DROP TABLE tpcds.warehouse_t10;

-- [DDL]
DROP SCHEMA tpcds;

-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t11 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- [DDL]
DROP TABLE tpcds.warehouse_t11;

-- [DDL]
DROP SCHEMA tpcds;

---或是用下面的语法，效果完全一样。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t12 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2), PRIMARY KEY(W_WAREHOUSE_SK) );

-- [DDL]
DROP TABLE tpcds.warehouse_t12;

-- [DDL]
DROP SCHEMA tpcds;

--或是用下面的语法，指定约束的名称。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t13 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2), CONSTRAINT W_CSTR_KEY1 PRIMARY KEY(W_WAREHOUSE_SK) );

-- [DDL]
DROP TABLE tpcds.warehouse_t13;

-- [DDL]
DROP SCHEMA tpcds;

--创建一个有主键约束并且指定约束类型及排序方式的表。
-- [DDL]
CREATE TABLE tpcds. warehouse_t13_1 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2), CONSTRAINT PRIMARY KEY USING BTREE (W_WAREHOUSE_SK DESC) );

-- [DDL]
DROP TABLE tpcds.warehouse_t13_1;

-- [DDL]
DROP SCHEMA tpcds;

--创建一个有复合主键约束的表。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t14 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2), CONSTRAINT W_CSTR_KEY2 PRIMARY KEY(W_WAREHOUSE_SK, W_WAREHOUSE_ID) );

-- [DDL]
DROP TABLE tpcds.warehouse_t14;

-- [DDL]
DROP SCHEMA tpcds;

--定义一个检查列约束。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t19 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY CHECK (W_WAREHOUSE_SK > 0), W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) CHECK (W_WAREHOUSE_NAME IS NOT NULL), W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- [DDL]
CREATE TABLE tpcds. warehouse_t20 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) CHECK (W_WAREHOUSE_NAME IS NOT NULL), W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2), CONSTRAINT W_CONSTR_KEY2 CHECK(W_WAREHOUSE_SK > 0 AND W_WAREHOUSE_NAME IS NOT NULL) );

--向 tpcds. warehouse_t19表中增加一个varchar列。
-- [DDL]
ALTER TABLE tpcds. warehouse_t19 ADD W_GOODS_CATEGORY varchar(30);

--给 tpcds. warehouse_t19表增加一个检查约束。
-- [DDL]
ALTER TABLE tpcds. warehouse_t19 ADD CONSTRAINT W_CONSTR_KEY4 CHECK (W_STATE IS NOT NULL);

--在一个操作中改变两个现存字段的类型。
-- [DDL]
ALTER TABLE tpcds. warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY TYPE varchar(80), ALTER COLUMN W_STREET_NAME TYPE varchar(100);

--此语句与上面语句等效。
-- [DDL]
ALTER TABLE tpcds. warehouse_t19 MODIFY (W_GOODS_CATEGORY varchar(30), W_STREET_NAME varchar(60));

--给一个已存在字段添加非空约束。
-- [DDL]
ALTER TABLE tpcds. warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY SET NOT NULL;

--移除已存在字段的非空约束。
-- [DDL]
ALTER TABLE tpcds. warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY DROP NOT NULL;

--将表移动到另一个表空间。
-- [DDL]
ALTER TABLE tpcds. warehouse_t19 SET TABLESPACE PG_DEFAULT;

--创建模式joe。
-- [DDL]
CREATE SCHEMA joe;

--将表移动到另一个模式中。
-- [DDL]
ALTER TABLE tpcds. warehouse_t19 SET SCHEMA joe;

--重命名已存在的表。
-- [DDL]
ALTER TABLE joe.warehouse_t19 RENAME TO warehouse_t23;

--从warehouse_t23表中删除一个字段。
-- [DDL]
ALTER TABLE joe.warehouse_t23 DROP COLUMN W_STREET_NAME;

--删除表空间、模式joe
-- [DDL]
DROP TABLESPACE DS_TABLESPACE1;

-- [DDL]
DROP SCHEMA IF EXISTS joe CASCADE;

-- [DDL]
DROP TABLE tpcds.warehouse_t20;

-- [DDL]
DROP SCHEMA tpcds;

--定义一个有ALWAYS属性IDENTITY列的表。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t26 ( W_WAREHOUSE_SK INTEGER GENERATED ALWAYS AS IDENTITY , W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- [DDL]
DROP TABLE tpcds.warehouse_t26;

-- [DDL]
DROP SCHEMA tpcds;

--定义一个有BY DEFAULT属性IDENTITY列的表。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t27 ( W_WAREHOUSE_SK INTEGER GENERATED BY DEFAULT AS IDENTITY (INCREMENT BY 10 MINVALUE 200 SCALE) , W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- [DDL]
DROP TABLE tpcds.warehouse_t27;

-- [DDL]
DROP SCHEMA tpcds;

--定义一个有BY DEFAULT ON NULL属性IDENTITY列的表。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. warehouse_t28 ( W_WAREHOUSE_SK INTEGER GENERATED BY DEFAULT ON NULL AS IDENTITY (START WITH 10 MAXVALUE 200 CYCLE) , W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- [DDL]
DROP TABLE tpcds.warehouse_t28;

-- [DDL]
DROP SCHEMA tpcds;

--创建一个有外键约束的表。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds.city_t23 ( W_CITY VARCHAR(60) PRIMARY KEY, W_ADDRESS TEXT );

-- [DDL]
CREATE TABLE tpcds.warehouse_t23 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) REFERENCES tpcds.city_t23(W_CITY), W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- [DDL]
DROP TABLE tpcds.city_t23;

-- [DDL]
DROP TABLE tpcds.warehouse_t23;

-- [DDL]
DROP SCHEMA tpcds;

--或是用下面的语法，效果完全一样。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds.warehouse_t23 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) , FOREIGN KEY(W_CITY) REFERENCES tpcds.city_t23(W_CITY) );

-- [DDL]
DROP TABLE tpcds.warehouse_t23;

-- [DDL]
DROP SCHEMA tpcds;

--或是用下面的语法，指定约束的名称。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds.warehouse_t23 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) , CONSTRAINT W_FORE_KEY1 FOREIGN KEY(W_CITY) REFERENCES tpcds.city_t23(W_CITY) );

-- [DDL]
DROP TABLE tpcds.warehouse_t23;

-- [DDL]
DROP SCHEMA tpcds;

-- [DDL]
CREATE TABLE t1(C1 VARCHAR(20) CHARSET utf8mb4 COLLATE utf8mb4_unicode_ci) CHARSET = utf8mb4 COLLATE = utf8mb4_ bin ;

-- [DDL]
ALTER TABLE t1 charset utf8mb4 collate utf8mb4_general_ci;

-- 为t1表新增字段c3，并设置字段的字符集为utf8mb4，字符序为utf8mb4_bin
-- [DDL]
ALTER TABLE t1 add c3 varchar(20) charset utf8mb4 collate utf8mb4_bin;

-- 创建开启ILM的表
-- [DDL]
ALTER DATABASE SET ILM=ON;

-- [DDL]
CREATE TABLE ilm_table (a int) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION ON (a != 0);

-- [DDL]
CREATE TABLE ilm_table (a int);

-- [DDL]
ALTER TABLE ilm_table ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION;

-- [DDL]
ALTER TABLE ilm_table ILM DISABLE_ALL;

-- [DDL]
ALTER TABLE ilm_table ILM ENABLE_ALL;

-- [DDL]
ALTER TABLE ilm_table ILM DELETE_ALL;

-- 创建B模式数据
-- [DDL]
CREATE DATABASE test_on_update DBCOMPATIBILITY 'b';

-- 设置数据库兼容性
-- [SESSION]
SET b_format_version='5.7';

-- [SESSION]
SET b_format_dev_version='s1';

-- 创建 t1_on_update 表，设置ON UPDATE属性
-- [DDL]
CREATE TABLE t1_on_update ( TS0 TIMESTAMP ON UPDATE CURRENT_TIMESTAMP , TS1 TIMESTAMP ON UPDATE CURRENT_TIMESTAMP() , TS2 TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6) , DT0 DATETIME ON UPDATE LOCALTIMESTAMP , DT1 DATETIME ON UPDATE NOW()


================================================================================
-- 来源: 2977_CREATE TABLE AS.txt
================================================================================

-- [DDL]
CREATE TABLE test1(col1 int PRIMARY KEY,col2 varchar(10));

-- [DML_INSERT]
INSERT INTO test1 VALUES (1,'col1'),(101,'col101');

-- 查询表中col1<100的数据。
-- [DQL]
SELECT * FROM test1 WHERE col1 < 100;

-- 创建test2表并向表中插入上面查询的数据。
-- [DDL]
CREATE TABLE test2 AS SELECT * FROM test1 WHERE col1 < 100;

-- [DDL]
CREATE TABLE test3(c1,c2) AS SELECT * FROM test1;

-- 删除。
-- [DDL]
DROP TABLE test1,test2,test3;


================================================================================
-- 来源: 2978_CREATE TABLE PARTITION.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建表 tpcds. web_returns。
-- [DDL]
CREATE TABLE tpcds. web_returns ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

--创建分区表 tpcds. web_returns_p1。
-- [DDL]
CREATE TABLE tpcds. web_returns_p1 ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL, WR_REFUNDED_CUSTOMER_SK INTEGER , WR_REFUNDED_CDEMO_SK INTEGER , WR_REFUNDED_HDEMO_SK INTEGER , WR_REFUNDED_ADDR_SK INTEGER , WR_RETURNING_CUSTOMER_SK INTEGER , WR_RETURNING_CDEMO_SK INTEGER , WR_RETURNING_HDEMO_SK INTEGER , WR_RETURNING_ADDR_SK INTEGER , WR_WEB_PAGE_SK INTEGER , WR_REASON_SK INTEGER , WR_ORDER_NUMBER BIGINT NOT NULL, WR_RETURN_QUANTITY INTEGER , WR_RETURN_AMT DECIMAL(7,2) , WR_RETURN_TAX DECIMAL(7,2) , WR_RETURN_AMT_INC_TAX DECIMAL(7,2) , WR_FEE DECIMAL(7,2) , WR_RETURN_SHIP_COST DECIMAL(7,2) , WR_REFUNDED_CASH DECIMAL(7,2) , WR_REVERSED_CHARGE DECIMAL(7,2) , WR_ACCOUNT_CREDIT DECIMAL(7,2) , WR_NET_LOSS DECIMAL(7,2) ) PARTITION BY RANGE(WR_RETURNED_DATE_SK) ( PARTITION P1 VALUES LESS THAN(2450815), PARTITION P2 VALUES LESS THAN(2451179), PARTITION P3 VALUES LESS THAN(2451544), PARTITION P4 VALUES LESS THAN(2451910), PARTITION P5 VALUES LESS THAN(2452275), PARTITION P6 VALUES LESS THAN(2452640), PARTITION P7 VALUES LESS THAN(2453005), PARTITION P8 VALUES LESS THAN(MAXVALUE) );

--从示例数据表导入数据。
-- [DML_INSERT]
INSERT INTO tpcds. web_returns_p1 SELECT * FROM tpcds. web_returns;

--删除分区P8。
-- [DDL]
ALTER TABLE tpcds. web_returns_p1 DROP PARTITION P8;

--增加分区WR_RETURNED_DATE_SK介于2453005和2453105之间。
-- [DDL]
ALTER TABLE tpcds. web_returns_p1 ADD PARTITION P8 VALUES LESS THAN (2453105);

--增加分区WR_RETURNED_DATE_SK介于2453105和MAXVALUE之间。
-- [DDL]
ALTER TABLE tpcds. web_returns_p1 ADD PARTITION P9 VALUES LESS THAN (MAXVALUE);

-- [DDL]
ALTER TABLE tpcds. web_returns_p1 RENAME PARTITION P7 TO P10;

--分区P6重命名为P11。
-- [DDL]
ALTER TABLE tpcds. web_returns_p1 RENAME PARTITION FOR (2452639) TO P11;

--查询分区P10的行数。
-- [DQL]
SELECT count(*) FROM tpcds. web_returns_p1 PARTITION (P10);

--查询分区P1的行数。
-- [DQL]
SELECT COUNT(*) FROM tpcds. web_returns_p1 PARTITION FOR (2450815);

--删除表tpcds.web_returns_p1。
-- [DDL]
DROP TABLE tpcds.web_returns_p1;

--删除表tpcds.web_returns。
-- [DDL]
DROP TABLE tpcds.web_returns;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;

--创建开启ILM策略的表ilm_part并分区
-- [DDL]
CREATE TABLE ilm_part (a int) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION PARTITION BY RANGE (a) ( PARTITION p1 VALUES LESS THAN (10), PARTITION p2 VALUES LESS THAN (20), PARTITION p3 VALUES LESS THAN (30));

-- [DDL]
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1/tablespace_1';

-- [DDL]
CREATE TABLESPACE example2 RELATIVE LOCATION 'tablespace2/tablespace_2';

-- [DDL]
CREATE TABLESPACE example3 RELATIVE LOCATION 'tablespace3/tablespace_3';

-- [DDL]
CREATE TABLESPACE example4 RELATIVE LOCATION 'tablespace4/tablespace_4';

--创建SCHEMA。
-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds. web_returns_p2 ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL, WR_REFUNDED_CUSTOMER_SK INTEGER , WR_REFUNDED_CDEMO_SK INTEGER , WR_REFUNDED_HDEMO_SK INTEGER , WR_REFUNDED_ADDR_SK INTEGER , WR_RETURNING_CUSTOMER_SK INTEGER , WR_RETURNING_CDEMO_SK INTEGER , WR_RETURNING_HDEMO_SK INTEGER , WR_RETURNING_ADDR_SK INTEGER , WR_WEB_PAGE_SK INTEGER , WR_REASON_SK INTEGER , WR_ORDER_NUMBER BIGINT NOT NULL, WR_RETURN_QUANTITY INTEGER , WR_RETURN_AMT DECIMAL(7,2) , WR_RETURN_TAX DECIMAL(7,2) , WR_RETURN_AMT_INC_TAX DECIMAL(7,2) , WR_FEE DECIMAL(7,2) , WR_RETURN_SHIP_COST DECIMAL(7,2) , WR_REFUNDED_CASH DECIMAL(7,2) , WR_REVERSED_CHARGE DECIMAL(7,2) , WR_ACCOUNT_CREDIT DECIMAL(7,2) , WR_NET_LOSS DECIMAL(7,2) ) TABLESPACE example1 PARTITION BY RANGE(WR_RETURNED_DATE_SK) ( PARTITION P1 VALUES LESS THAN(2450815), PARTITION P2 VALUES LESS THAN(2451179), PARTITION P3 VALUES LESS THAN(2451544), PARTITION P4 VALUES LESS THAN(2451910), PARTITION P5 VALUES LESS THAN(2452275), PARTITION P6 VALUES LESS THAN(2452640), PARTITION P7 VALUES LESS THAN(2453005), PARTITION P8 VALUES LESS THAN(MAXVALUE) TABLESPACE example2 ) ENABLE ROW MOVEMENT;

--以like方式创建一个分区表。
-- [DDL]
CREATE TABLE tpcds. web_returns_p3 (LIKE tpcds. web_returns_p2 INCLUDING PARTITION);

--修改分区P1的表空间为example2。
-- [DDL]
ALTER TABLE tpcds. web_returns_p2 MOVE PARTITION P1 TABLESPACE example2;

--修改分区P2的表空间为example3。
-- [DDL]
ALTER TABLE tpcds. web_returns_p2 MOVE PARTITION P2 TABLESPACE example3;

--以2453010为分割点切分P8。
-- [DDL]
ALTER TABLE tpcds. web_returns_p2 SPLIT PARTITION P8 AT (2453010) INTO ( PARTITION P9, PARTITION P10 );

--将P6，P7合并为一个分区。
-- [DDL]
ALTER TABLE tpcds. web_returns_p2 MERGE PARTITIONS P6, P7 INTO PARTITION P8;

--修改分区表迁移属性。
-- [DDL]
ALTER TABLE tpcds. web_returns_p2 DISABLE ROW MOVEMENT;

--删除表和表空间。
-- [DDL]
DROP TABLE tpcds. web_returns_p1;

-- [DDL]
DROP TABLE tpcds. web_returns_p2;

-- [DDL]
DROP TABLE tpcds. web_returns_p3;

-- [DDL]
DROP TABLESPACE example1;

-- [DDL]
DROP TABLESPACE example2;

-- [DDL]
DROP TABLESPACE example3;

-- [DDL]
DROP TABLESPACE example4;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;

-- [DDL]
CREATE TABLESPACE startend_tbs1 LOCATION '/home/ omm /startend_tbs1';

-- [DDL]
CREATE TABLESPACE startend_tbs2 LOCATION '/home/ omm /startend_tbs2';

-- [DDL]
CREATE TABLESPACE startend_tbs3 LOCATION '/home/ omm /startend_tbs3';

-- [DDL]
CREATE TABLESPACE startend_tbs4 LOCATION '/home/ omm /startend_tbs4';

-- 创建临时schema
-- [DDL]
CREATE SCHEMA tpcds;

-- [SESSION]
SET CURRENT_SCHEMA TO tpcds;

-- 创建分区表，分区键是integer类型
-- [DDL]
CREATE TABLE tpcds.startend_pt (c1 INT, c2 INT) TABLESPACE startend_tbs1 PARTITION BY RANGE (c2) ( PARTITION p1 START(1) END(1000) EVERY(200) TABLESPACE startend_tbs2, PARTITION p2 END(2000), PARTITION p3 START(2000) END(2500) TABLESPACE startend_tbs3, PARTITION p4 START(2500), PARTITION p5 START(3000) END(5000) EVERY(1000) TABLESPACE startend_tbs4 ) ENABLE ROW MOVEMENT;

-- 查看分区表信息
-- [DQL]
SELECT relname, boundaries, spcname FROM pg_partition p JOIN pg_tablespace t ON p.reltablespace=t.oid and p.parentid='tpcds.startend_pt'::regclass ORDER BY 1;

-- 导入数据，查看分区数据量
-- [DML_INSERT]
INSERT INTO tpcds.startend_pt VALUES (GENERATE_SERIES(0, 4999), GENERATE_SERIES(0, 4999));

-- [DQL]
SELECT COUNT(*) FROM tpcds.startend_pt PARTITION FOR (0);

-- [DQL]
SELECT COUNT(*) FROM tpcds.startend_pt PARTITION (p3);

-- 增加分区: [5000, 5300), [5300, 5600), [5600, 5900), [5900, 6000)
-- [DDL]
ALTER TABLE tpcds.startend_pt ADD PARTITION p6 START(5000) END(6000) EVERY(300) TABLESPACE startend_tbs4;

-- 增加MAXVALUE分区: p
-- [DDL]
ALTER TABLE tpcds.startend_pt ADD PARTITION p7 END(MAXVALUE);

-- 重命名分区p7为p
-- [DDL]
ALTER TABLE tpcds.startend_pt RENAME PARTITION p7 TO p8;

-- 删除分区p
-- [DDL]
ALTER TABLE tpcds.startend_pt DROP PARTITION p8;

-- 重命名5950所在的分区为：p
-- [DDL]
ALTER TABLE tpcds.startend_pt RENAME PARTITION FOR(5950) TO p71;

-- 分裂4500所在的分区[4000, 5000)
-- [DDL]
ALTER TABLE tpcds.startend_pt SPLIT PARTITION FOR(4500) INTO(PARTITION q1 START(4000) END(5000) EVERY(250) TABLESPACE startend_tbs3);

-- 修改分区p2的表空间为startend_tbs
-- [DDL]
ALTER TABLE tpcds.startend_pt MOVE PARTITION p2 TABLESPACE startend_tbs4;

-- 查看分区情形
-- [DQL]
SELECT relname, boundaries, spcname FROM pg_partition p JOIN pg_tablespace t ON p.reltablespace=t.oid and p.parentid='tpcds.startend_pt'::regclass ORDER BY 1;

-- 删除表和表空间
-- [DDL]
DROP SCHEMA tpcds CASCADE;

-- [DDL]
DROP TABLESPACE startend_tbs1;

-- [DDL]
DROP TABLESPACE startend_tbs2;

-- [DDL]
DROP TABLESPACE startend_tbs3;

-- [DDL]
DROP TABLESPACE startend_tbs4;

-- [DDL]
CREATE TABLE sales ( prod_id NUMBER ( 6 ), cust_id NUMBER , time_id DATE , channel_id CHAR ( 1 ), promo_id NUMBER ( 6 ), quantity_sold NUMBER ( 3 ), amount_sold NUMBER ( 10 , 2 ) ) PARTITION BY RANGE ( time_id ) INTERVAL ( '1 day' ) ( PARTITION p1 VALUES LESS THAN ( '2019-02-01 00:00:00' ), PARTITION p2 VALUES LESS THAN ( '2019-02-02 00:00:00' ) );

-- [DML_INSERT]
INSERT INTO sales VALUES ( 1 , 12 , '2019-01-10 00:00:00' , 'a' , 1 , 1 , 1 );

-- [DML_INSERT]
INSERT INTO sales VALUES ( 1 , 12 , '2019-02-01 00:00:00' , 'a' , 1 , 1 , 1 );

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'sales' AND t1 . parttype = 'p' ;

-- [DML_INSERT]
INSERT INTO sales VALUES ( 1 , 12 , '2019-02-05 00:00:00' , 'a' , 1 , 1 , 1 );

-- [DML_INSERT]
INSERT INTO sales VALUES ( 1 , 12 , '2019-02-03 00:00:00' , 'a' , 1 , 1 , 1 );

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'sales' AND t1 . parttype = 'p' ;

-- [DDL]
create table test_list ( col1 int , col2 int ) partition by list ( col1 ) ( partition p1 values ( 2000 ), partition p2 values ( 3000 ), partition p3 values ( 4000 ), partition p4 values ( 5000 ) );

-- [DML_INSERT]
INSERT INTO test_list VALUES ( 2000 , 2000 );

-- [DML_INSERT]
INSERT INTO test_list VALUES ( 3000 , 3000 );

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- [DML_INSERT]
INSERT INTO test_list VALUES ( 6000 , 6000 );

-- [DDL]
alter table test_list add partition p5 values ( 6000 );

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- [DML_INSERT]
INSERT INTO test_list VALUES ( 6000 , 6000 );

-- [DDL]
create table t1 ( col1 int , col2 int );

-- [DQL]
select * from test_list partition ( p1 );

-- [DDL]
alter table test_list exchange partition ( p1 ) with table t1 ;

-- [DQL]
select * from test_list partition ( p1 );

-- [DQL]
select * from t1 ;

-- [DQL]
select * from test_list partition ( p2 );

-- [DDL]
alter table test_list truncate partition p2 ;

-- [DQL]
select * from test_list partition ( p2 );

-- [DDL]
alter table test_list drop partition p5 ;

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- [DML_INSERT]
INSERT INTO test_list VALUES ( 6000 , 6000 );

-- [DDL]
alter table test_list merge partitions p1 , p2 into partition p2 ;

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- [DDL]
alter table test_list split partition p2 values ( 2000 ) into ( partition p1 , partition p2 );

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- [DDL]
drop table test_list ;

-- [DDL]
create table test_hash ( col1 int , col2 int ) partition by hash ( col1 ) ( partition p1 , partition p2 );

-- [DML_INSERT]
INSERT INTO test_hash VALUES ( 1 , 1 );

-- [DML_INSERT]
INSERT INTO test_hash VALUES ( 2 , 2 );

-- [DML_INSERT]
INSERT INTO test_hash VALUES ( 3 , 3 );

-- [DML_INSERT]
INSERT INTO test_hash VALUES ( 4 , 4 );

-- [DQL]
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_hash' AND t1 . parttype = 'p' ;

-- [DQL]
select * from test_hash partition ( p1 );

-- [DQL]
select * from test_hash partition ( p2 );

-- [DDL]
create table t1 ( col1 int , col2 int );

-- [DDL]
alter table test_hash exchange partition ( p1 ) with table t1 ;

-- [DQL]
select * from test_hash partition ( p1 );

-- [DQL]
select * from t1 ;

-- [DDL]
alter table test_hash truncate partition p2 ;

-- [DQL]
select * from test_hash partition ( p2 );

-- [DDL]
drop table test_hash ;

-- [DDL]
CREATE TABLE t_multi_keys_list ( a int , b varchar ( 4 ), c int ) PARTITION BY LIST ( a , b ) ( PARTITION p1 VALUES ( ( 0 , NULL ) ), PARTITION p2 VALUES ( ( 0 , '1' ), ( 0 , '2' ), ( 0 , '3' ), ( 1 , '1' ), ( 1 , '2' ) ), PARTITION p3 VALUES ( ( NULL , '0' ), ( 2 , '1' ) ), PARTITION p4 VALUES ( ( 3 , '2' ), ( NULL , NULL ) ), PARTITION pd VALUES ( DEFAULT ) );

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_part ( a int ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION , PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- [DDL]
DROP TABLE ilm_part ;

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- [DDL]
DROP TABLE ilm_part ;

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- [DDL]
DROP TABLE ilm_part ;

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- [DDL]
DROP TABLE ilm_part ;

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- [DDL]
DROP TABLE ilm_part ;


================================================================================
-- 来源: 2979_CREATE TABLESPACE.txt
================================================================================

-- [DDL]
CREATE TABLESPACE ds_location1 RELATIVE LOCATION 'tablespace/tablespace_1';

--创建用户joe。
-- [DDL]
CREATE ROLE joe IDENTIFIED BY ' ******** ';

--创建用户jay。
-- [DDL]
CREATE ROLE jay IDENTIFIED BY ' ******** ';

--创建表空间，且所有者指定为用户joe。
-- [DDL]
CREATE TABLESPACE ds_location2 OWNER joe RELATIVE LOCATION 'tablespace/tablespace_2';

--把表空间ds_location1重命名为ds_location3。
-- [DDL]
ALTER TABLESPACE ds_location1 RENAME TO ds_location3;

--改变表空间ds_location2的所有者。
-- [DDL]
ALTER TABLESPACE ds_location2 OWNER TO jay;

--删除表空间。
-- [DDL]
DROP TABLESPACE ds_location2;

-- [DDL]
DROP TABLESPACE ds_location3;

--删除用户。
-- [DDL]
DROP ROLE joe;

-- [DDL]
DROP ROLE jay;


================================================================================
-- 来源: 2980_CREATE TABLE SUBPARTITION.txt
================================================================================

-- [DDL]
CREATE TABLE list_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( '2' ) ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( '2' ) ) );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201902' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201903' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- [DQL]
select * from list_list ;

-- [DDL]
DROP TABLE list_list ;

-- [DDL]
CREATE TABLE list_hash ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY HASH ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a , SUBPARTITION p_201901_b ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a , SUBPARTITION p_201902_b ) );

-- [DML_INSERT]
INSERT INTO list_hash VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_hash VALUES ( '201902' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_hash VALUES ( '201902' , '3' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_hash VALUES ( '201903' , '4' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_hash VALUES ( '201903' , '5' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_hash VALUES ( '201903' , '6' , '1' , 1 );

-- [DQL]
select * from list_hash ;

-- [DDL]
DROP TABLE list_hash ;

-- [DDL]
CREATE TABLE list_range ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY RANGE ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a VALUES less than ( '4' ), SUBPARTITION p_201901_b VALUES less than ( '6' ) ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a VALUES less than ( '3' ), SUBPARTITION p_201902_b VALUES less than ( '6' ) ) );

-- [DML_INSERT]
INSERT INTO list_range VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_range VALUES ( '201902' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_range VALUES ( '201902' , '3' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_range VALUES ( '201903' , '4' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_range VALUES ( '201903' , '5' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_range VALUES ( '201903' , '6' , '1' , 1 );

-- [DQL]
select * from list_range ;

-- [DDL]
DROP TABLE list_range ;

-- [DDL]
CREATE TABLE range_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 VALUES LESS THAN ( '201903' ) ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( '2' ) ), PARTITION p_201902 VALUES LESS THAN ( '201904' ) ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( '2' ) ) );

-- [DML_INSERT]
INSERT INTO range_list VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_list VALUES ( '201902' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_list VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_list VALUES ( '201903' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_list VALUES ( '201903' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_list VALUES ( '201903' , '2' , '1' , 1 );

-- [DQL]
select * from range_list ;

-- [DDL]
DROP TABLE range_list ;

-- [DDL]
CREATE TABLE range_hash ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE ( month_code ) SUBPARTITION BY HASH ( dept_code ) ( PARTITION p_201901 VALUES LESS THAN ( '201903' ) ( SUBPARTITION p_201901_a , SUBPARTITION p_201901_b ), PARTITION p_201902 VALUES LESS THAN ( '201904' ) ( SUBPARTITION p_201902_a , SUBPARTITION p_201902_b ) );

-- [DML_INSERT]
INSERT INTO range_hash VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_hash VALUES ( '201902' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_hash VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_hash VALUES ( '201903' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_hash VALUES ( '201903' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_hash VALUES ( '201903' , '2' , '1' , 1 );

-- [DQL]
select * from range_hash ;

-- [DDL]
DROP TABLE range_hash ;

-- [DDL]
CREATE TABLE range_range ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE ( month_code ) SUBPARTITION BY RANGE ( dept_code ) ( PARTITION p_201901 VALUES LESS THAN ( '201903' ) ( SUBPARTITION p_201901_a VALUES LESS THAN ( '2' ), SUBPARTITION p_201901_b VALUES LESS THAN ( '3' ) ), PARTITION p_201902 VALUES LESS THAN ( '201904' ) ( SUBPARTITION p_201902_a VALUES LESS THAN ( '2' ), SUBPARTITION p_201902_b VALUES LESS THAN ( '3' ) ) );

-- [DML_INSERT]
INSERT INTO range_range VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_range VALUES ( '201902' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_range VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_range VALUES ( '201903' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_range VALUES ( '201903' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO range_range VALUES ( '201903' , '2' , '1' , 1 );

-- [DQL]
select * from range_range ;

-- [DDL]
DROP TABLE range_range ;

-- [DDL]
CREATE TABLE hash_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY hash ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( '2' ) ), PARTITION p_201902 ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( '2' ) ) );

-- [DML_INSERT]
INSERT INTO hash_list VALUES ( '201901' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_list VALUES ( '201901' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_list VALUES ( '201901' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_list VALUES ( '201903' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_list VALUES ( '201903' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_list VALUES ( '201903' , '2' , '1' , 1 );

-- [DQL]
select * from hash_list ;

-- [DDL]
DROP TABLE hash_list ;

-- [DDL]
CREATE TABLE hash_hash ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY hash ( month_code ) SUBPARTITION BY hash ( dept_code ) ( PARTITION p_201901 ( SUBPARTITION p_201901_a , SUBPARTITION p_201901_b ), PARTITION p_201902 ( SUBPARTITION p_201902_a , SUBPARTITION p_201902_b ) );

-- [DML_INSERT]
INSERT INTO hash_hash VALUES ( '201901' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_hash VALUES ( '201901' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_hash VALUES ( '201901' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_hash VALUES ( '201903' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_hash VALUES ( '201903' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_hash VALUES ( '201903' , '2' , '1' , 1 );

-- [DQL]
select * from hash_hash ;

-- [DDL]
DROP TABLE hash_hash ;

-- [DDL]
CREATE TABLE hash_range ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY hash ( month_code ) SUBPARTITION BY range ( dept_code ) ( PARTITION p_201901 ( SUBPARTITION p_201901_a VALUES LESS THAN ( '2' ), SUBPARTITION p_201901_b VALUES LESS THAN ( '3' ) ), PARTITION p_201902 ( SUBPARTITION p_201902_a VALUES LESS THAN ( '2' ), SUBPARTITION p_201902_b VALUES LESS THAN ( '3' ) ) );

-- [DML_INSERT]
INSERT INTO hash_range VALUES ( '201901' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_range VALUES ( '201901' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_range VALUES ( '201901' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_range VALUES ( '201903' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_range VALUES ( '201903' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO hash_range VALUES ( '201903' , '2' , '1' , 1 );

-- [DQL]
select * from hash_range ;

-- [DDL]
DROP TABLE hash_range ;

-- [DDL]
CREATE TABLE range_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY LIST (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES ('1'), SUBPARTITION p_201901_b VALUES ('2') ), PARTITION p_201902 VALUES LESS THAN( '201910' ) ( SUBPARTITION p_201902_a VALUES ('1'), SUBPARTITION p_201902_b VALUES ('2') ) );

--指定一级分区插入数据
-- [DML_INSERT]
INSERT INTO range_list partition (p_201901) VALUES('201902', '1', '1', 1);

--实际分区和指定分区不一致，报错
-- [DML_INSERT]
INSERT INTO range_list partition (p_201902) VALUES('201902', '1', '1', 1);

--指定二级分区插入数据
-- [DML_INSERT]
INSERT INTO range_list subpartition (p_201901_a) VALUES('201902', '1', '1', 1);

--实际分区和指定分区不一致，报错
-- [DML_INSERT]
INSERT INTO range_list subpartition (p_201901_b) VALUES('201902', '1', '1', 1);

-- [DML_INSERT]
INSERT INTO range_list partition for ('201902') VALUES('201902', '1', '1', 1);

-- [DML_INSERT]
INSERT INTO range_list subpartition for ('201902','1') VALUES('201902', '1', '1', 1);

--指定分区查询数据
-- [DQL]
select * from range_list partition (p_201901);

-- [DQL]
select * from range_list subpartition (p_201901_a);

-- [DQL]
select * from range_list partition for ('201902');

-- [DQL]
select * from range_list subpartition for ('201902','1');

--指定分区更新数据
-- [DML_UPDATE]
update range_list partition (p_201901) set user_no = '2';

-- [DQL]
select * from range_list;

-- [DML_UPDATE]
update range_list subpartition (p_201901_a) set user_no = '3';

-- [DQL]
select * from range_list;

-- [DML_UPDATE]
update range_list partition for ('201902') set user_no = '4';

-- [DQL]
select * from range_list;

-- [DML_UPDATE]
update range_list subpartition for ('201902','2') set user_no = '5';

-- [DQL]
select *from range_list;

-- [DQL]
select * from range_list;

--指定分区删除数据
-- [DML_DELETE]
delete from range_list partition (p_201901);

-- [DML_DELETE]
delete from range_list partition for ('201903');

-- [DML_DELETE]
delete from range_list subpartition (p_201901_a);

-- [DML_DELETE]
delete from range_list subpartition for ('201903','2');

--参数sql_compatibility='B'时，可指定多分区删除数据
-- [DDL]
CREATE DATABASE db dbcompatibility 'B';

-- [DDL]
CREATE TABLE range_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY LIST (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES ('1'), SUBPARTITION p_201901_b VALUES ('2') ), PARTITION p_201902 VALUES LESS THAN( '201910' ) ( SUBPARTITION p_201902_a VALUES ('1'), SUBPARTITION p_201902_b VALUES ('2') ) );

-- [DML_DELETE]
delete from range_list as t partition (p_201901_a, p_201901);

--删除数据库
-- [DDL]
DROP DATABASE db;

--指定分区insert数据
-- [DML_INSERT]
INSERT INTO range_list partition (p_201901) VALUES('201902', '1', '1', 1) ON DUPLICATE KEY UPDATE sales_amt = 5;

-- [DML_INSERT]
INSERT INTO range_list subpartition (p_201901_a) VALUES('201902', '1', '1', 1) ON DUPLICATE KEY UPDATE sales_amt = 10;

-- [DML_INSERT]
INSERT INTO range_list partition for ('201902') VALUES('201902', '1', '1', 1) ON DUPLICATE KEY UPDATE sales_amt = 30;

-- [DML_INSERT]
INSERT INTO range_list subpartition for ('201902','1') VALUES('201902', '1', '1', 1) ON DUPLICATE KEY UPDATE sales_amt = 40;

-- [DQL]
select * from range_list;

--指定分区merge into数据
-- [DDL]
CREATE TABLE newrange_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY LIST (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES ('1'), SUBPARTITION p_201901_b VALUES ('2') ), PARTITION p_201902 VALUES LESS THAN( '201910' ) ( SUBPARTITION p_201902_a VALUES ('1'), SUBPARTITION p_201902_b VALUES ('2') ) );

-- [DML_INSERT]
INSERT INTO newrange_list VALUES('201902', '1', '1', 1);

-- [DML_INSERT]
INSERT INTO newrange_list VALUES('201903', '1', '1', 2);

-- [DML_MERGE]
MERGE INTO range_list partition (p_201901) p USING newrange_list partition (p_201901) np ON p.month_code= np.month_code WHEN MATCHED THEN UPDATE SET dept_code = np.dept_code, user_no = np.user_no, sales_amt = np.sales_amt WHEN NOT MATCHED THEN INSERT VALUES (np.month_code, np.dept_code, np.user_no, np.sales_amt);

-- [DQL]
select * from range_list;

-- [DML_MERGE]
MERGE INTO range_list partition for ('201901') p USING newrange_list partition for ('201901') np ON p.month_code= np.month_code WHEN MATCHED THEN UPDATE SET dept_code = np.dept_code, user_no = np.user_no, sales_amt = np.sales_amt WHEN NOT MATCHED THEN INSERT VALUES (np.month_code, np.dept_code, np.user_no, np.sales_amt);

-- [DQL]
select * from range_list;

-- [DML_MERGE]
MERGE INTO range_list subpartition (p_201901_a) p USING newrange_list subpartition (p_201901_a) np ON p.month_code= np.month_code WHEN MATCHED THEN UPDATE SET dept_code = np.dept_code, user_no = np.user_no, sales_amt = np.sales_amt WHEN NOT MATCHED THEN INSERT VALUES (np.month_code, np.dept_code, np.user_no, np.sales_amt);

-- [DQL]
select * from range_list;

-- [DML_MERGE]
MERGE INTO range_list subpartition for ('201901', '1') p USING newrange_list subpartition for ('201901', '1') np ON p.month_code= np.month_code WHEN MATCHED THEN UPDATE SET dept_code = np.dept_code, user_no = np.user_no, sales_amt = np.sales_amt WHEN NOT MATCHED THEN INSERT VALUES (np.month_code, np.dept_code, np.user_no, np.sales_amt);

-- [DQL]
select * from range_list;

-- [DDL]
DROP TABLE range_list;

-- [DDL]
DROP TABLE newrange_list;

-- [DDL]
CREATE TABLE list_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( default ) ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( '2' ) ) );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201902' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201903' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- [DQL]
select * from list_list ;

-- [DQL]
select * from list_list partition ( p_201901 );

-- [DDL]
alter table list_list truncate partition p_201901 ;

-- [DQL]
select * from list_list partition ( p_201901 );

-- [DQL]
select * from list_list partition ( p_201902 );

-- [DDL]
alter table list_list truncate partition p_201902 ;

-- [DQL]
select * from list_list partition ( p_201902 );

-- [DQL]
select * from list_list ;

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201902' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201903' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- [DQL]
select * from list_list subpartition ( p_201901_a );

-- [DDL]
alter table list_list truncate subpartition p_201901_a ;

-- [DQL]
select * from list_list subpartition ( p_201901_a );

-- [DQL]
select * from list_list subpartition ( p_201901_b );

-- [DDL]
alter table list_list truncate subpartition p_201901_b ;

-- [DQL]
select * from list_list subpartition ( p_201901_b );

-- [DQL]
select * from list_list subpartition ( p_201902_a );

-- [DDL]
alter table list_list truncate subpartition p_201902_a ;

-- [DQL]
select * from list_list subpartition ( p_201902_a );

-- [DQL]
select * from list_list subpartition ( p_201902_b );

-- [DDL]
alter table list_list truncate subpartition p_201902_b ;

-- [DQL]
select * from list_list subpartition ( p_201902_b );

-- [DQL]
select * from list_list ;

-- [DDL]
DROP TABLE list_list ;

-- [DDL]
CREATE TABLE list_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( default ) ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( default ) ) );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201902' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201903' , '1' , '1' , 1 );

-- [DML_INSERT]
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- [DQL]
select * from list_list ;

-- [DQL]
select * from list_list subpartition ( p_201901_a );

-- [DQL]
select * from list_list subpartition ( p_201901_b );

-- [DDL]
alter table list_list split subpartition p_201901_b VALUES ( 2 ) into ( subpartition p_201901_b , subpartition p_201901_c );

-- [DQL]
select * from list_list subpartition ( p_201901_a );

-- [DQL]
select * from list_list subpartition ( p_201901_b );

-- [DQL]
select * from list_list subpartition ( p_201901_c );

-- [DQL]
select * from list_list partition ( p_201901 );

-- [DQL]
select * from list_list subpartition ( p_201902_a );

-- [DQL]
select * from list_list subpartition ( p_201902_b );

-- [DDL]
alter table list_list split subpartition p_201902_b VALUES ( 3 ) into ( subpartition p_201902_b , subpartition p_201902_c );

-- [DQL]
select * from list_list subpartition ( p_201902_a );

-- [DQL]
select * from list_list subpartition ( p_201902_b );

-- [DQL]
select * from list_list subpartition ( p_201902_c );

-- [DDL]
DROP TABLE list_list ;

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_subpart ( a int , b int ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION PARTITION BY RANGE ( a ) SUBPARTITION BY RANGE ( b ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ( SUBPARTITION p1_s1 VALUES LESS THAN ( 10 ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION , SUBPARTITION p1_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p1_s3 VALUES LESS THAN ( 30 )), PARTITION p2 VALUES LESS THAN ( 20 ) ( SUBPARTITION p2_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p2_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p2_s3 VALUES LESS THAN ( 30 )), PARTITION p3 VALUES LESS THAN ( 30 ) ( SUBPARTITION p3_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p3_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p3_s3 VALUES LESS THAN ( 30 )));

-- [DDL]
DROP TABLE ilm_subpart ;

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_subpart ( a int , b int ) PARTITION BY RANGE ( a ) SUBPARTITION BY RANGE ( b ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ( SUBPARTITION p1_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p1_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p1_s3 VALUES LESS THAN ( 30 )), PARTITION p2 VALUES LESS THAN ( 20 ) ( SUBPARTITION p2_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p2_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p2_s3 VALUES LESS THAN ( 30 )), PARTITION p3 VALUES LESS THAN ( 30 ) ( SUBPARTITION p3_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p3_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p3_s3 VALUES LESS THAN ( 30 )));

-- [DDL]
ALTER TABLE ilm_subpart MODIFY SUBPARTITION p2_s1 ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION ;

-- [DDL]
ALTER TABLE ilm_subpart MODIFY SUBPARTITION p2_s1 ILM DISABLE_ALL ;

-- [DDL]
ALTER TABLE ilm_subpart MODIFY SUBPARTITION p2_s1 ILM ENABLE_ALL ;

-- [DDL]
ALTER TABLE ilm_subpart MODIFY SUBPARTITION p2_s1 ILM DELETE_ALL ;

-- [DDL]
DROP TABLE ilm_subpart ;

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_subpart ( a int , b int ) PARTITION BY RANGE ( a ) SUBPARTITION BY RANGE ( b ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ( SUBPARTITION p1_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p1_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p1_s3 VALUES LESS THAN ( 30 )), PARTITION p2 VALUES LESS THAN ( 20 ) ( SUBPARTITION p2_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p2_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p2_s3 VALUES LESS THAN ( 30 )), PARTITION p3 VALUES LESS THAN ( 30 ) ( SUBPARTITION p3_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p3_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p3_s3 VALUES LESS THAN ( 30 )));

-- [DDL]
ALTER TABLE ilm_subpart MODIFY PARTITION p2 ADD SUBPARTITION p2_s4 VALUES LESS THAN ( 40 ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION ;

-- [DDL]
DROP TABLE ilm_subpart ;

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE TABLE ilm_subpart ( a int , b int ) PARTITION BY RANGE ( a ) SUBPARTITION BY RANGE ( b ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ( SUBPARTITION p1_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p1_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p1_s3 VALUES LESS THAN ( 30 )), PARTITION p2 VALUES LESS THAN ( 20 ) ( SUBPARTITION p2_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p2_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p2_s3 VALUES LESS THAN ( 30 )), PARTITION p3 VALUES LESS THAN ( 30 ) ( SUBPARTITION p3_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p3_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p3_s3 VALUES LESS THAN ( 30 )));

-- [DDL]
ALTER TABLE ilm_subpart SPLIT SUBPARTITION p1_s2 AT ( '15' ) INTO ( SUBPARTITION p1_s2_1 ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION , SUBPARTITION p1_s2_2 );

-- [DDL]
DROP TABLE ilm_subpart ;


================================================================================
-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH CONFIGURATION ngram2 (parser=ngram) WITH (gram_size = 2, grapsymbol_ignore = false);

--创建文本搜索配置。
-- [DDL]
CREATE TEXT SEARCH CONFIGURATION ngram3 (copy=ngram2) WITH (gram_size = 2, grapsymbol_ignore = false);

--添加类型映射。
-- [DDL]
ALTER TEXT SEARCH CONFIGURATION ngram2 ADD MAPPING FOR multisymbol WITH simple;

--创建用户joe。
-- [DDL]
CREATE USER joe IDENTIFIED BY ' ******** ';

--修改文本搜索配置的所有者。
-- [DDL]
ALTER TEXT SEARCH CONFIGURATION ngram2 OWNER TO joe;

--修改文本搜索配置的schema。
-- [DDL]
ALTER TEXT SEARCH CONFIGURATION ngram2 SET SCHEMA joe;

--重命名文本搜索配置。
-- [DDL]
ALTER TEXT SEARCH CONFIGURATION joe.ngram2 RENAME TO ngram_2;

--删除类型映射。
-- [DDL]
ALTER TEXT SEARCH CONFIGURATION joe.ngram_2 DROP MAPPING IF EXISTS FOR multisymbol;

--删除文本搜索配置。
-- [DDL]
DROP TEXT SEARCH CONFIGURATION joe.ngram_2;

-- [DDL]
DROP TEXT SEARCH CONFIGURATION ngram3;

--删除Schema及用户joe。
-- [DDL]
DROP SCHEMA IF EXISTS joe CASCADE;

-- [DDL]
DROP ROLE IF EXISTS joe;


================================================================================
-- 来源: 2983_CREATE TRIGGER.txt
================================================================================

-- [DDL]
CREATE TABLE test_trigger_src_tbl(id1 INT, id2 INT, id3 INT);

-- [DDL]
CREATE TABLE test_trigger_des_tbl(id1 INT, id2 INT, id3 INT);

--创建触发器函数
-- [DDL]
CREATE OR REPLACE FUNCTION tri_insert_func() RETURNS TRIGGER AS $$ DECLARE BEGIN INSERT INTO test_trigger_des_tbl VALUES(NEW.id1, NEW.id2, NEW.id3);

-- [DDL]
CREATE OR REPLACE FUNCTION tri_update_func() RETURNS TRIGGER AS $$ DECLARE BEGIN UPDATE test_trigger_des_tbl SET id3 = NEW.id3 WHERE id1=OLD.id1;

-- [DDL]
CREATE OR REPLACE FUNCTION TRI_DELETE_FUNC() RETURNS TRIGGER AS $$ DECLARE BEGIN DELETE FROM test_trigger_des_tbl WHERE id1=OLD.id1;

--创建INSERT触发器
-- [DDL]
CREATE TRIGGER insert_trigger BEFORE INSERT ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_insert_func();

--创建UPDATE触发器
-- [DDL]
CREATE TRIGGER update_trigger AFTER UPDATE ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_update_func();

--创建DELETE触发器
-- [DDL]
CREATE TRIGGER delete_trigger BEFORE DELETE ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_delete_func();

--执行INSERT触发事件并检查触发结果
-- [DML_INSERT]
INSERT INTO test_trigger_src_tbl VALUES(100,200,300);

-- [DQL]
SELECT * FROM test_trigger_src_tbl;

-- [DQL]
SELECT * FROM test_trigger_des_tbl;

--执行UPDATE触发事件并检查触发结果
-- [DML_UPDATE]
UPDATE test_trigger_src_tbl SET id3=400 WHERE id1=100;

-- [DQL]
SELECT * FROM test_trigger_src_tbl;

-- [DQL]
SELECT * FROM test_trigger_des_tbl;

--执行DELETE触发事件并检查触发结果
-- [DML_DELETE]
DELETE FROM test_trigger_src_tbl WHERE id1=100;

-- [DQL]
SELECT * FROM test_trigger_src_tbl;

-- [DQL]
SELECT * FROM test_trigger_des_tbl;

--修改触发器
-- [DDL]
ALTER TRIGGER delete_trigger ON test_trigger_src_tbl RENAME TO delete_trigger_renamed;

--禁用insert_trigger触发器
-- [DDL]
ALTER TABLE test_trigger_src_tbl DISABLE TRIGGER insert_trigger;

--禁用当前表上所有触发器
-- [DDL]
ALTER TABLE test_trigger_src_tbl DISABLE TRIGGER ALL;

--删除触发器
-- [DDL]
DROP TRIGGER insert_trigger ON test_trigger_src_tbl;

-- [DDL]
DROP TRIGGER update_trigger ON test_trigger_src_tbl;

-- [DDL]
DROP TRIGGER delete_trigger_renamed ON test_trigger_src_tbl;

--删除函数。
-- [DDL]
DROP FUNCTION tri_insert_func();

-- [DDL]
DROP FUNCTION tri_update_func();

-- [DDL]
DROP FUNCTION tri_delete_func();

--删除源表及触发表。
-- [DDL]
DROP TABLE test_trigger_src_tbl;

-- [DDL]
DROP TABLE test_trigger_des_tbl;


================================================================================
-- 来源: 2984_CREATE TYPE.txt
================================================================================

-- [DDL]
CREATE TYPE compfoo AS (f1 int, f2 text);

-- [DDL]
CREATE TABLE t1_compfoo(a int, b compfoo);

-- [DDL]
CREATE TABLE t2_compfoo(a int, b compfoo);

-- [DML_INSERT]
INSERT INTO t1_compfoo values(1,(1,'demo'));

-- [DML_INSERT]
INSERT INTO t2_compfoo select * from t1_compfoo;

-- [DQL]
SELECT (b).f1 FROM t1_compfoo;

-- [DQL]
SELECT * FROM t1_compfoo t1 join t2_compfoo t2 on (t1.b).f1=(t1.b).f1;

--重命名数据类型。
-- [DDL]
ALTER TYPE compfoo RENAME TO compfoo1;

--要改变一个用户定义类型compfoo1的所有者为usr1。
-- [DDL]
CREATE USER usr1 PASSWORD ' ******** ';

-- [DDL]
ALTER TYPE compfoo1 OWNER TO usr1;

--把用户定义类型compfoo1的模式改变为usr1。
-- [DDL]
ALTER TYPE compfoo1 SET SCHEMA usr1;

--给一个数据类型增加一个新的属性。
-- [DDL]
ALTER TYPE usr1.compfoo1 ADD ATTRIBUTE f3 int;

--删除compfoo1类型。
-- [DDL]
DROP TYPE usr1.compfoo1 CASCADE;

--删除相关表和用户。
-- [DDL]
DROP TABLE t1_compfoo;

-- [DDL]
DROP TABLE t2_compfoo;

-- [DDL]
DROP SCHEMA usr1;

-- [DDL]
DROP USER usr1;

--创建一个枚举类型。
-- [DDL]
CREATE TYPE bugstatus AS ENUM ('create', 'modify', 'closed');

--添加一个标签值。
-- [DDL]
ALTER TYPE bugstatus ADD VALUE IF NOT EXISTS 'regress' BEFORE 'closed';

--重命名一个标签值。
-- [DDL]
ALTER TYPE bugstatus RENAME VALUE 'create' TO 'new';

--创建一个集合类型
-- [DDL]
CREATE TYPE bugstatus_table AS TABLE OF bugstatus;

--删除集合类型及枚举类型。
-- [DDL]
DROP TYPE bugstatus_table;

-- [DDL]
DROP TYPE bugstatus CASCADE;


================================================================================
-- 来源: 2985_CREATE USER.txt
================================================================================

-- [DDL]
CREATE USER jim PASSWORD ' ******** ';

--下面语句与上面的等价。
-- [DDL]
CREATE USER kim IDENTIFIED BY ' ******** ';

--如果创建有“创建数据库”权限的用户，则需要加CREATEDB关键字。
-- [DDL]
CREATE USER dim CREATEDB PASSWORD ' ******** ';

--将用户jim的登录密码由 ******** 修改为**********。
-- [DDL]
ALTER USER jim IDENTIFIED BY '**********' REPLACE ' ******** ';

--为用户jim追加CREATEROLE权限。
-- [DDL]
ALTER USER jim CREATEROLE;

--将enable_seqscan的值设置为on， 设置成功后，在下一会话中生效。
-- [DDL]
ALTER USER jim SET enable_seqscan TO on;

--重置jim的enable_seqscan参数。
-- [DDL]
ALTER USER jim RESET enable_seqscan;

--锁定jim账户。
-- [DDL]
ALTER USER jim ACCOUNT LOCK;

--删除用户。
-- [DDL]
DROP USER kim CASCADE;

-- [DDL]
DROP USER jim CASCADE;

-- [DDL]
DROP USER dim CASCADE;


================================================================================
-- 来源: 2986_CREATE USER MAPPING.txt
================================================================================

-- [DDL]
CREATE ROLE bob PASSWORD '********';

-- 创建外部服务器
-- [DDL]
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

-- 创建USER MAPPING。
-- [DDL]
CREATE USER MAPPING FOR bob SERVER my_server OPTIONS (user 'bob', password '********');

-- 修改USER MAPPING。
-- [DDL]
ALTER USER MAPPING FOR bob SERVER my_server OPTIONS (SET password '********');

-- 删除USER MAPPING。
-- [DDL]
DROP USER MAPPING FOR bob SERVER my_server;

-- 删除外部服务器。
-- [DDL]
DROP SERVER my_server;

-- 删除角色。
-- [DDL]
DROP ROLE bob;


================================================================================
-- 来源: 2987_CREATE VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE test_tb1(col1 int, col2 int);

-- [DML_INSERT]
INSERT INTO test_tb1 VALUES (generate_series(1,100),generate_series(1,100));

--创建一个col1小于5的视图。
-- [DDL]
CREATE VIEW test_v1 AS SELECT * FROM test_tb1 WHERE col1 < 3;

--查看视图。
-- [DQL]
SELECT * FROM test_v1;

--删除表和视图。
-- [DDL]
DROP VIEW test_v1;

-- [DDL]
DROP TABLE test_tb1;

-- [DDL]
CREATE TABLE test_tb2(c1 int, c2 int);

-- [DDL]
CREATE TEMP VIEW test_v2 AS SELECT * FROM test_tb2;

--删除视图和表。
-- [DDL]
DROP VIEW test_v2 ;

-- [DDL]
DROP TABLE test_tb2;


================================================================================
-- 来源: 2988_CREATE WEAK PASSWORD DICTIONARY.txt
================================================================================

-- [DDL]
CREATE WEAK PASSWORD DICTIONARY WITH VALUES ('password1');

--向gs_global_config系统表中插入多个弱口令。
-- [DDL]
CREATE WEAK PASSWORD DICTIONARY WITH VALUES ('password2'),('password3');

--清空gs_global_config系统表中所有弱口令。
-- [DDL]
DROP WEAK PASSWORD DICTIONARY;

--查看现有弱口令。
-- [DQL]
SELECT * FROM gs_global_config WHERE NAME LIKE 'weak_password';


================================================================================
-- 来源: 2991_DEALLOCATE.txt
================================================================================

-- [DQL]
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- [PREPARED_STMT]
PREPARE q1 AS SELECT 1 AS a ;

-- [PREPARED_STMT]
PREPARE q2 AS SELECT 1 AS a ;

-- [PREPARED_STMT]
PREPARE q3 AS SELECT 1 AS a ;

-- [DQL]
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- [PREPARED_STMT]
DEALLOCATE q1 ;

-- [DQL]
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- [PREPARED_STMT]
DEALLOCATE ALL ;

-- [DQL]
SELECT name , statement , parameter_types FROM pg_prepared_statements ;


================================================================================
-- 来源: 2993_DELETE.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建表tpcds.customer_address。
-- [DDL]
CREATE TABLE tpcds.customer_address ( ca_address_sk INTEGER NOT NULL, ca_address_id CHARACTER(16) NOT NULL, ca_street_number INTEGER , ca_street_name CHARACTER (20) );

--向表中插入多条记录。
-- [DML_INSERT]
INSERT INTO tpcds.customer_address VALUES (1, 'AAAAAAAABAAAAAAA', '18', 'Jackson'),(10000, 'AAAAAAAACAAAAAAA', '362', 'Washington 6th'),(15000, 'AAAAAAAADAAAAAAA', '585', 'Dogwood Washington');

--创建表 tpcds. customer_address_bak。
-- [DDL]
CREATE TABLE tpcds. customer_address_bak AS TABLE tpcds. customer_address;

--删除 tpcds. customer_address_bak中ca_address_sk大于14888的职员。
-- [DML_DELETE]
DELETE FROM tpcds. customer_address_bak WHERE ca_address_sk > 14888;

--同时删除 tpcds. customer_address和 tpcds. customer_address_bak中ca_address_sk小于50的职员。
-- [DML_DELETE]
DELETE FROM tpcds. customer_address a, tpcds. customer_address_bak b where a.ca_address_sk = b.ca_address_sk and a.ca_address_sk < 50;

--删除 tpcds. customer_address_bak中所有数据。
-- [DML_DELETE]
DELETE FROM tpcds. customer_address_bak;

--删除 tpcds. customer_address_bak表。
-- [DDL]
DROP TABLE tpcds. customer_address_bak;

--删除tpcds.customer_address表。
-- [DDL]
DROP TABLE tpcds.customer_address;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 2994_DO.txt
================================================================================

-- [DDL]
CREATE USER webuser PASSWORD ' ******** ';

--授予用户webuser对模式tpcds下视图的所有操作权限。
-- [PLSQL]
DO $$DECLARE r record;

--删除用户webuser。
-- [DDL]
DROP USER webuser CASCADE;


================================================================================
-- 来源: 2995_DROP AGGREGATE.txt
================================================================================

-- [DDL]
CREATE OR REPLACE FUNCTION int_add(int,int) returns int as $BODY$ declare begin return $1 + $2;

-- 创建聚合函数。
-- [OTHER]
CREATE AGGREGATE myavg (int) ( sfunc = int_add, stype = int, initcond = '0' );

--将integer类型的聚合函数myavg删除。
-- [OTHER]
DROP AGGREGATE myavg(integer);

-- 删除自定义函数
-- [DDL]
DROP FUNCTION int_add(int,int);


================================================================================
-- 来源: 3000_DROP DATA SOURCE.txt
================================================================================

-- [DDL]
CREATE DATA SOURCE ds_tst1;

--删除Data Source对象。
-- [DDL]
DROP DATA SOURCE ds_tst1 CASCADE;

-- [DDL]
DROP DATA SOURCE IF EXISTS ds_tst1 RESTRICT;


================================================================================
-- 来源: 3001_DROP DIRECTORY.txt
================================================================================

-- [DDL]
CREATE OR REPLACE DIRECTORY dir as '/tmp/';

--删除目录。
-- [DDL]
DROP DIRECTORY dir;


================================================================================
-- 来源: 3003_DROP EXTENSION.txt
================================================================================

-- [DDL]
DROP EXTENSION plpgsql ;


================================================================================
-- 来源: 3010_DROP MASKING POLICY.txt
================================================================================

-- [DDL]
DROP MASKING POLICY IF EXISTS maskpol1 ;

-- [DDL]
DROP MASKING POLICY IF EXISTS maskpol1 , maskpol2 , maskpol3 ;


================================================================================
-- 来源: 3011_DROP MATERIALIZED VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建名为my_mv的物化视图。
-- [DDL]
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--删除名为my_mv的物化视图。
-- [DDL]
DROP MATERIALIZED VIEW my_mv;

--删除表。
-- [DDL]
DROP TABLE my_table;


================================================================================
-- 来源: 3014_DROP OWNED.txt
================================================================================

-- [DDL]
CREATE USER jim PASSWORD '********' ;

-- [DDL]
DROP OWNED BY jim ;

-- [DDL]
DROP USER jim ;


================================================================================
-- 来源: 3015_DROP PACKAGE.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PACKAGE PCK1 IS a int;

--删除PACKAGE。
-- [DDL]
DROP PACKAGE PCK1;


================================================================================
-- 来源: 3020_DROP ROW LEVEL SECURITY POLICY.txt
================================================================================

-- [DDL]
CREATE TABLE all_data(id int, role varchar(100), data varchar(100));

--创建行访问控制策略。
-- [OTHER]
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING(role = CURRENT_USER);

--删除行访问控制策略。
-- [OTHER]
DROP ROW LEVEL SECURITY POLICY all_data_rls ON all_data;

--删除数据表all_data。
-- [DDL]
DROP TABLE all_data;


================================================================================
-- 来源: 3021_DROP RULE.txt
================================================================================

-- [DDL]
CREATE TABLE def_test ( c1 int4 DEFAULT 5, c2 text DEFAULT 'initial_default' );

-- [DDL]
CREATE VIEW def_view_test AS SELECT * FROM def_test;

--创建RULE def_view_test_ins
-- [OTHER]
CREATE RULE def_view_test_ins AS

-- [OTHER]
ON INSERT TO def_view_test

-- [PLSQL]
DO INSTEAD INSERT INTO def_test SELECT new.*;

--删除RULE def_view_test_ins
-- [OTHER]
DROP RULE def_view_test_ins ON def_view_test;

--删除表def_test、视图def_view_test
-- [DDL]
DROP VIEW def_view_test;

-- [DDL]
DROP TABLE def_test;


================================================================================
-- 来源: 3024_DROP SECURITY LABEL.txt
================================================================================

-- [DDL]
CREATE SECURITY LABEL sec_label 'L1:G4' ;

-- [DDL]
DROP SECURITY LABEL sec_label ;


================================================================================
-- 来源: 3025_DROP SEQUENCE.txt
================================================================================

-- [DDL]
CREATE SEQUENCE serial START 101;

--删除序列。
-- [DDL]
DROP SEQUENCE serial;


================================================================================
-- 来源: 3026_DROP SERVER.txt
================================================================================

-- [DDL]
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--删除my_server。
-- [DDL]
DROP SERVER my_server;


================================================================================
-- 来源: 3032_DROP TEXT SEARCH DICTIONARY.txt
================================================================================

-- [DDL]
CREATE TEXT SEARCH DICTIONARY english ( TEMPLATE = simple );

--删除词典english。
-- [DDL]
DROP TEXT SEARCH DICTIONARY english;


================================================================================
-- 来源: 3040_EXECUTE.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建表reason。
-- [DDL]
CREATE TABLE tpcds. reason ( CD_DEMO_SK INTEGER NOT NULL, CD_GENDER character(16) , CD_MARITAL_STATUS character(100) );

--插入数据。
-- [DML_INSERT]
INSERT INTO tpcds. reason VALUES(51, 'AAAAAAAADDAAAAAA', 'reason 51');

--创建表reason_t1。
-- [DDL]
CREATE TABLE tpcds. reason_t1 AS TABLE tpcds.reason;

--为一个INSERT语句创建一个预备语句然后执行它。
-- [PREPARED_STMT]
PREPARE insert_reason(integer,character(16),character(100)) AS INSERT INTO tpcds. reason_t1 VALUES($1,$2,$3);

-- [PREPARED_STMT]
EXECUTE insert_reason(52, 'AAAAAAAADDAAAAAA', 'reason 52');

--删除表reason和reason_t1。
-- [DDL]
DROP TABLE tpcds. reason;

-- [DDL]
DROP TABLE tpcds. reason_t1;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 3041_EXPDP DATABASE.txt
================================================================================

-- [OTHER]
EXPDP DATABASE test LOCATION = '/data1/expdp/database';


================================================================================
-- 来源: 3042_EXPDP TABLE.txt
================================================================================

-- [OTHER]
EXPDP TABLE test_t LOCATION = '/data1/expdp/table0';


================================================================================
-- 来源: 3043_EXPLAIN.txt
================================================================================

-- [DDL]
CREATE TABLE student(id int, name char(20));

-- [EXPLAIN]
EXPLAIN (NODE true) INSERT INTO student VALUES(5,'a'),(6,'b');

-- [EXPLAIN]
EXPLAIN (NUM_NODES true) INSERT INTO student VALUES(5,'a'),(6,'b');

-- [DDL]
CREATE SCHEMA tpcds;

--创建表tpcds.customer_address。
-- [DDL]
CREATE TABLE tpcds.customer_address ( ca_address_sk INTEGER NOT NULL, ca_address_id CHARACTER(16) NOT NULL );

--向表中插入多条记录。
-- [DML_INSERT]
INSERT INTO tpcds.customer_address VALUES (5000, 'AAAAAAAABAAAAAAA'),(10000, 'AAAAAAAACAAAAAAA');

--创建一个表 tpcds. customer_address_p1。
-- [DDL]
CREATE TABLE tpcds. customer_address_p1 AS TABLE tpcds. customer_address;

--修改explain_perf_mode为normal。
-- [SESSION]
SET explain_perf_mode=normal;

--显示表简单查询的执行计划。
-- [EXPLAIN]
EXPLAIN SELECT * FROM tpcds. customer_address_p1;

--以JSON格式输出的执行计划（explain_perf_mode为normal时）。
-- [EXPLAIN]
EXPLAIN(FORMAT JSON) SELECT * FROM tpcds. customer_address_p1;

--如果有一个索引，当使用一个带索引WHERE条件的查询，可能会显示一个不同的计划。
-- [EXPLAIN]
EXPLAIN SELECT * FROM tpcds. customer_address_p1 WHERE ca_address_sk=10000;

--以YAML格式输出的执行计划（explain_perf_mode为normal时）。
-- [EXPLAIN]
EXPLAIN(FORMAT YAML) SELECT * FROM tpcds. customer_address_p1 WHERE ca_address_sk=10000;

--禁止开销估计的执行计划。
-- [EXPLAIN]
EXPLAIN(COSTS FALSE) SELECT * FROM tpcds. customer_address_p1 WHERE ca_address_sk=10000;

--带有聚集函数查询的执行计划。
-- [EXPLAIN]
EXPLAIN SELECT SUM(ca_address_sk) FROM tpcds. customer_address_p1 WHERE ca_address_sk<10000;

--创建一个二级分区表。
-- [DDL]
CREATE TABLE range_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY LIST (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a values ('1'), SUBPARTITION p_201901_b values ('2') ), PARTITION p_201902 VALUES LESS THAN( '201910' ) ( SUBPARTITION p_201902_a values ('1'), SUBPARTITION p_201902_b values ('2') ) );

--执行带有二级分区表的查询语句。
--Iterations 和 Sub Iterations分别标识遍历了几个一级分区和二级分区。
--Selected Partitions标识哪些一级分区被实际扫描，Selected Subpartitions: (p:s)标识第p个一级分区下s个二级分区被实际扫描，如果一级分区下所有二级分区都被扫描则s显示为ALL。
-- [EXPLAIN]
EXPLAIN SELECT * FROM range_list WHERE dept_code = '1';

--删除表 tpcds. customer_address_p1。
-- [DDL]
DROP TABLE tpcds. customer_address_p1;

--删除表tpcds.customer_address。
-- [DDL]
DROP TABLE tpcds.customer_address;

--删除表range_list。
-- [DDL]
DROP TABLE range_list;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;

-- [DDL]
CREATE TABLE tb_a(c1 int);

-- [DML_INSERT]
INSERT INTO tb_a VALUES(1),(2),(3);

-- [DDL]
CREATE TABLE tb_b AS SELECT * FROM tb_a;

-- [EXPLAIN]
EXPLAIN (OPTEVAL on )SELECT * FROM tb_a a, tb_b b WHERE a.c1=b.c1 AND a.c1=1;

--删除表tb_a，tb_b。
-- [DDL]
DROP TABLE tb_a;

-- [DDL]
DROP TABLE tb_b;


================================================================================
-- 来源: 3044_EXPLAIN PLAN.txt
================================================================================

-- [DDL]
CREATE TABLE foo1(f1 int, f2 text, f3 text[]);

-- [DDL]
CREATE TABLE foo2(f1 int, f2 text, f3 text[]);

--执行explain plan。
-- [EXPLAIN]
EXPLAIN PLAN SET STATEMENT_ID = 'TPCH-Q4' FOR SELECT f1, count(*) FROM foo1 WHERE f1 > 1 AND f1 < 3 AND EXISTS (SELECT * FROM foo2) GROUP BY f1;

-- [DQL]
SELECT * FROM plan_table;

-- [DML_DELETE]
DELETE FROM plan_table WHERE STATEMENT_ID = 'TPCH-Q4' ;

-- [DDL]
DROP TABLE foo1 ;

-- [DDL]
DROP TABLE foo2 ;


================================================================================
-- 来源: 3046_FETCH.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建表tpcds.customer_address。
-- [DDL]
CREATE TABLE tpcds.customer_address ( ca_address_sk INTEGER NOT NULL, ca_address_id CHARACTER(16) NOT NULL, ca_street_number INTEGER , ca_street_name CHARACTER (20) );

--向表中插入多条记录。
-- [DML_INSERT]
INSERT INTO tpcds.customer_address VALUES (1, 'AAAAAAAABAAAAAAA', '18', 'Jackson'),(2, 'AAAAAAAACAAAAAAA', '362', 'Washington 6th'),(3, 'AAAAAAAADAAAAAAA', '585', 'Dogwood Washington');

--SELECT语句，用一个游标读取一个表。开始一个事务。
-- [TCL]
START TRANSACTION;

--建立一个名为cursor1的游标。
-- [OTHER]
CURSOR cursor1 FOR SELECT * FROM tpcds. customer_address ORDER BY 1;

--抓取头3行到游标cursor1里。
-- [CURSOR]
FETCH FORWARD 3 FROM cursor1;

--关闭游标并提交事务。
-- [CURSOR]
CLOSE cursor1;

--VALUES子句，用一个游标读取VALUES子句中的内容。开始一个事务。
-- [TCL]
START TRANSACTION;

--建立一个名为cursor2的游标。
-- [OTHER]
CURSOR cursor2 FOR VALUES(1,2),(0,3) ORDER BY 1;

--抓取头2行到游标cursor2里。
-- [CURSOR]
FETCH FORWARD 2 FROM cursor2;

--关闭游标并提交事务。
-- [CURSOR]
CLOSE cursor2;

--WITH HOLD游标的使用，开启事务。
-- [TCL]
START TRANSACTION;

--创建一个with hold游标。
-- [PLSQL]
DECLARE cursor1 CURSOR WITH HOLD FOR SELECT * FROM tpcds. customer_address ORDER BY 1;

--抓取头2行到游标cursor1里。
-- [CURSOR]
FETCH FORWARD 2 FROM cursor1;

--抓取下一行到游标cursor1里。
-- [CURSOR]
FETCH FORWARD 1 FROM cursor1;

--关闭游标。
-- [CURSOR]
CLOSE cursor1;

--删除表tpcds.customer_address。
-- [DDL]
DROP TABLE tpcds.customer_address;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 3048_GRANT.txt
================================================================================

-- [DDL]
CREATE USER joe PASSWORD ' ******** ';

-- [DCL_GRANT]
GRANT ALL PRIVILEGES TO joe;

-- [DDL]
CREATE SCHEMA tpcds;

-- [DDL]
CREATE TABLE tpcds.reason ( r_reason_sk INTEGER NOT NULL, r_reason_id CHAR(16) NOT NULL, r_reason_desc VARCHAR(20) );

-- [DCL_REVOKE]
REVOKE ALL PRIVILEGES FROM joe;

-- [DCL_GRANT]
GRANT USAGE ON SCHEMA tpcds TO joe;

-- [DCL_GRANT]
GRANT ALL PRIVILEGES ON tpcds. reason TO joe;

-- [DCL_GRANT]
GRANT SELECT (r_reason_sk,r_reason_id,r_reason_desc),UPDATE (r_reason_desc) ON tpcds. reason TO joe;

-- [DCL_GRANT]
GRANT SELECT (r_reason_sk, r_reason_id) ON tpcds. reason TO joe WITH GRANT OPTION;

-- [DDL]
CREATE DATABASE testdb;

-- [DCL_GRANT]
GRANT CREATE,CONNECT ON DATABASE testdb TO joe WITH GRANT OPTION;

-- [DDL]
CREATE ROLE tpcds_manager PASSWORD ' ******** ';

-- [DCL_GRANT]
GRANT USAGE,CREATE ON SCHEMA tpcds TO tpcds_manager;

-- [DDL]
CREATE TABLESPACE tpcds_tbspc RELATIVE LOCATION 'tablespace/tablespace_1';

-- [DCL_GRANT]
GRANT ALL ON TABLESPACE tpcds_tbspc TO joe;

-- [DDL]
CREATE or replace FUNCTION tpcds.fun1() RETURN boolean AS BEGIN SELECT current_user;

-- [DCL_GRANT]
GRANT ALTER ON FUNCTION tpcds.fun1() TO joe;

-- [DDL]
CREATE ROLE manager PASSWORD ' ******** ';

-- [DCL_GRANT]
GRANT joe TO manager WITH ADMIN OPTION;

-- [DDL]
CREATE ROLE senior_manager PASSWORD ' ******** ';

-- [DCL_GRANT]
GRANT manager TO senior_manager;

-- [DCL_REVOKE]
REVOKE joe FROM manager;

-- [DCL_REVOKE]
REVOKE manager FROM senior_manager;

-- [DDL]
DROP USER manager;

-- [DDL]
DROP DATABASE testdb;


================================================================================
-- 来源: 3050_IMPDP DATABASE CREATE.txt
================================================================================

-- [OTHER]
IMPDP DATABASE test CREATE SOURCE = '/data1/impdp/database' OWNER=admin;


================================================================================
-- 来源: 3051_IMPDP RECOVER.txt
================================================================================

-- [OTHER]
IMPDP DATABASE RECOVER SOURCE = '/data1/impdp/database' OWNER=admin;


================================================================================
-- 来源: 3052_IMPDP TABLE.txt
================================================================================

-- [OTHER]
IMPDP TABLE SOURCE = '/data1/impdp/table0' OWNER=admin;


================================================================================
-- 来源: 3053_IMPDP TABLE PREPARE.txt
================================================================================

-- [OTHER]
IMPDP TABLE PREPARE SOURCE = '/data1/impdp/table0' OWNER=admin;


================================================================================
-- 来源: 3054_INSERT.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- [DDL]
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--向表中插入一条记录。
-- [DML_INSERT]
INSERT INTO tpcds.reason(r_reason_sk, r_reason_id, r_reason_desc) VALUES (0, 'AAAAAAAAAAAAAAAA', 'reason0');

--创建表 tpcds. reason_t2。
-- [DDL]
CREATE TABLE tpcds. reason_t2 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--向表中插入一条记录。
-- [DML_INSERT]
INSERT INTO tpcds. reason_t2(r_reason_sk, r_reason_id, r_reason_desc) VALUES (1, 'AAAAAAAABAAAAAAA', 'reason1');

--向表中插入一条记录，和上一条语法等效。
-- [DML_INSERT]
INSERT INTO tpcds. reason_t2 VALUES (2, 'AAAAAAAABAAAAAAA', 'reason2');

--向表中插入多条记录。
-- [DML_INSERT]
INSERT INTO tpcds. reason_t2 VALUES (3, 'AAAAAAAACAAAAAAA','reason3'),(4, 'AAAAAAAADAAAAAAA', 'reason4'),(5, 'AAAAAAAAEAAAAAAA','reason5');

--向表中插入 tpcds. reason中r_reason_sk小于5的记录。
-- [DML_INSERT]
INSERT INTO tpcds. reason_t2 SELECT * FROM tpcds. reason WHERE r_reason_sk <5;

--对表创建唯一索引。
-- [DDL]
CREATE UNIQUE INDEX reason_t2_u_index ON tpcds. reason_t2(r_reason_sk);

--向表中插入多条记录，如果冲突则更新冲突数据行中r_reason_id字段为'BBBBBBBBCAAAAAAA'。
-- [DML_INSERT]
INSERT INTO tpcds. reason_t2 VALUES (5, 'BBBBBBBBCAAAAAAA','reason5'),(6, 'AAAAAAAADAAAAAAA', 'reason6') ON DUPLICATE KEY UPDATE r_reason_id = 'BBBBBBBBCAAAAAAA';

--更新已有记录并返回
-- [DML_INSERT]
INSERT INTO tpcds. reason_t2 VALUES ( 5, 'BBBBBBBBCAAAAAAA','reason5') ON DUPLICATE KEY UPDATE r_reason_desc='reason5_new' RETURNING *;

--删除表 tpcds. reason_t2。
-- [DDL]
DROP TABLE tpcds. reason_t2;

--删除表tpcds.reason。
-- [DDL]
DROP TABLE tpcds.reason;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 3056_LOCK.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- [DDL]
CREATE TABLE tpcds.reason ( r_reason_sk INTEGER NOT NULL, r_reason_id CHAR(16) NOT NULL, r_reason_desc INTEGER );

--向表中插入多条记录。
-- [DML_INSERT]
INSERT INTO tpcds.reason VALUES (1, 'AAAAAAAABAAAAAAA', '18'),(5, 'AAAAAAAACAAAAAAA', '362'),(7, 'AAAAAAAADAAAAAAA', '585');

--在执行删除操作时对一个有主键的表进行 SHARE ROW EXCLUSIVE 锁。
-- [DDL]
CREATE TABLE tpcds. reason_t1 AS TABLE tpcds. reason;

-- [TCL]
START TRANSACTION;

-- [MAINTENANCE]
LOCK TABLE tpcds. reason_t1 IN SHARE ROW EXCLUSIVE MODE;

-- [DML_DELETE]
DELETE FROM tpcds. reason_t1 WHERE r_reason_desc IN(SELECT r_reason_desc FROM tpcds. reason_t1 WHERE r_reason_sk < 6 );

-- [DML_DELETE]
DELETE FROM tpcds. reason_t1 WHERE r_reason_sk = 7;

-- [TCL]
COMMIT;

--删除表 tpcds. reason_t1。
-- [DDL]
DROP TABLE tpcds. reason_t1;

--删除表。
-- [DDL]
DROP TABLE tpcds.reason;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 3060_MERGE INTO.txt
================================================================================

-- [DDL]
CREATE TABLE products ( product_id INTEGER, product_name VARCHAR2(60), category VARCHAR2(60) );

-- [DML_INSERT]
INSERT INTO products VALUES (1501, 'vivitar 35mm', 'electrncs');

-- [DML_INSERT]
INSERT INTO products VALUES (1502, 'olympus is50', 'electrncs');

-- [DML_INSERT]
INSERT INTO products VALUES (1600, 'play gym', 'toys');

-- [DML_INSERT]
INSERT INTO products VALUES (1601, 'lamaze', 'toys');

-- [DML_INSERT]
INSERT INTO products VALUES (1666, 'harry potter', 'dvd');

-- [DDL]
CREATE TABLE newproducts ( product_id INTEGER, product_name VARCHAR2(60), category VARCHAR2(60) );

-- [DML_INSERT]
INSERT INTO newproducts VALUES (1502, 'olympus camera', 'electrncs');

-- [DML_INSERT]
INSERT INTO newproducts VALUES (1601, 'lamaze', 'toys');

-- [DML_INSERT]
INSERT INTO newproducts VALUES (1666, 'harry potter', 'toys');

-- [DML_INSERT]
INSERT INTO newproducts VALUES (1700, 'wait interface', 'books');

-- 进行MERGE INTO操作
-- [DML_MERGE]
MERGE INTO products p USING newproducts np ON (p.product_id = np.product_id) WHEN MATCHED THEN UPDATE SET p.product_name = np.product_name, p.category = np.category WHERE p.product_name != 'play gym' WHEN NOT MATCHED THEN INSERT VALUES (np.product_id, np.product_name, np.category) WHERE np.category = 'books';

-- 查询更新后的结果
-- [DQL]
SELECT * FROM products ORDER BY product_id;

-- 删除表
-- [DDL]
DROP TABLE products;

-- [DDL]
DROP TABLE newproducts;


================================================================================
-- 来源: 3061_MOVE.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- [DDL]
CREATE TABLE tpcds.reason ( r_reason_sk INTEGER NOT NULL, r_reason_id CHAR(16) NOT NULL, r_reason_desc VARCHAR(40) );

--向表中插入多条记录。
-- [DML_INSERT]
INSERT INTO tpcds.reason VALUES (1, 'AAAAAAAABAAAAAAA', 'Xxxxxxxxx'),(2, 'AAAAAAAACAAAAAAA', ' Xxxxxxxxx'),(3, 'AAAAAAAADAAAAAAA', ' Xxxxxxxxx'),(4, 'AAAAAAAAEAAAAAAA', 'Not the product that was ordered'),(5, 'AAAAAAAAFAAAAAAA', 'Parts missing'),(6, 'AAAAAAAAGAAAAAAA', 'Does not work with a product that I have'),(7, 'AAAAAAAAHAAAAAAA', 'Gift exchange');

--开始一个事务。
-- [TCL]
START TRANSACTION;

--定义一个名为cursor1的游标。
-- [OTHER]
CURSOR cursor1 FOR SELECT * FROM tpcds. reason;

--忽略游标cursor1的前3行。
-- [OTHER]
MOVE FORWARD 3 FROM cursor1;

--抓取游标cursor1的前4行。
-- [CURSOR]
FETCH 4 FROM cursor1;

--关闭游标。
-- [CURSOR]
CLOSE cursor1;

--删除表。
-- [DDL]
DROP TABLE tpcds.reason;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 3063_PREDICT BY.txt
================================================================================

-- [DDL]
CREATE TABLE houses ( id INTEGER, tax INTEGER, bedroom INTEGER, bath DOUBLE PRECISION, price INTEGER, size INTEGER, lot INTEGER, mark text );

--插入训练数据
-- [DML_INSERT]
INSERT INTO houses(id, tax, bedroom, bath, price, size, lot, mark) VALUES (1,590,2,1,50000,770,22100,'a+'), (2,1050,3,2,85000,1410,12000,'a+'), (3,20,2,1,22500,1060,3500,'a-'), (4,870,2,2,90000,1300,17500,'a+'), (5,1320,3,2,133000,1500,30000,'a+'), (6,1350,2,1,90500,850,25700,'a-'), (7,2790,3,2.5,260000,2130,25000,'a+'), (8,680,2,1,142500,1170,22000,'a-'), (9,1840,3,2,160000,1500,19000,'a+'), (10,3680,4,2,240000,2790,20000,'a-'), (11,1660,3,1,87000,1030,17500,'a+'), (12,1620,3,2,118500,1250,20000,'a-'), (13,3100,3,2,140000,1760,38000,'a+'), (14,2090,2,3,148000,1550,14000,'a-'), (15,650,3,1.5,65000,1450,12000,'a-');

--训练模型
-- [DDL]
CREATE MODEL price_model USING logistic_regression FEATURES size, lot TARGET mark FROM HOUSES WITH learning_rate=0.88, max_iterations=default;

-- [DQL]
SELECT id, PREDICT BY price_model (FEATURES size,lot) FROM houses;

--删除模型
-- [DDL]
DROP MODEL price_model;

--删除表
-- [DDL]
DROP TABLE houses;


================================================================================
-- 来源: 3065_PREPARE TRANSACTION.txt
================================================================================

-- [TCL]
BEGIN;

--准备标识符为的trans_test的事务。
-- [TCL]
PREPARE TRANSACTION 'trans_test';

--取消标识符为的trans_test的事务。
-- [TCL]
ROLLBACK PREPARED 'trans_test';


================================================================================
-- 来源: 3066_PURGE.txt
================================================================================

-- [DDL]
CREATE ROLE tpcds IDENTIFIED BY '*********';

-- 创建表空间reason_table_space
-- [DDL]
CREATE TABLESPACE REASON_TABLE_SPACE1 owner tpcds RELATIVE location 'tablespace/tsp_reason1';

-- 创建SCHEMA。
-- [DDL]
CREATE SCHEMA tpcds;

-- 在表空间创建表tpcds.reason_t
-- [DDL]
CREATE TABLE tpcds.reason_t1 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) tablespace reason_table_space1;

-- 在表空间创建表tpcds.reason_t
-- [DDL]
CREATE TABLE tpcds.reason_t2 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) tablespace reason_table_space1;

-- 在表空间创建表tpcds.reason_t
-- [DDL]
CREATE TABLE tpcds.reason_t3 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) tablespace reason_table_space1;

-- 对表tpcds.reason_t1创建索引
-- [DDL]
CREATE INDEX index_t1 on tpcds.reason_t1(r_reason_id);

-- [DDL]
DROP TABLE tpcds.reason_t1;

-- [DDL]
DROP TABLE tpcds.reason_t2;

-- [DDL]
DROP TABLE tpcds.reason_t3;

--查看回收站
-- [DQL]
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

--PURGE清除表
-- [OTHER]
PURGE TABLE tpcds.reason_t3;

-- [DQL]
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

--PURGE清除索引
-- [OTHER]
PURGE INDEX tpcds.index_t1;

-- [DQL]
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

--PURGE清除回收站所有对象
-- [OTHER]
PURGE recyclebin;

-- [DQL]
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

-- 删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 3068_REASSIGN OWNED.txt
================================================================================

-- [DDL]
CREATE USER jim PASSWORD '********' ;

-- [DDL]
CREATE USER tom PASSWORD '********' ;

-- [OTHER]
REASSIGN OWNED BY jim TO tom ;

-- [DDL]
DROP USER jim , tom CASCADE ;


================================================================================
-- 来源: 3069_REFRESH INCREMENTAL MATERIALIZED VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建增量物化视图。
-- [DDL]
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--基表写入数据。
-- [DML_INSERT]
INSERT INTO my_table VALUES(1,1),(2,2);

--对增量物化视图my_imv进行增量刷新。
-- [OTHER]
REFRESH INCREMENTAL MATERIALIZED VIEW my_imv;

--删除增量物化视图。
-- [DDL]
DROP MATERIALIZED VIEW my_imv;

--删除表my_table。
-- [DDL]
DROP TABLE my_table;


================================================================================
-- 来源: 3070_REFRESH MATERIALIZED VIEW.txt
================================================================================

-- [DDL]
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建全量物化视图。
-- [DDL]
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--创建增量物化视图。
-- [DDL]
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--基表写入数据。
-- [DML_INSERT]
INSERT INTO my_table VALUES(1,1),(2,2);

--对全量物化视图my_mv进行全量刷新。
-- [OTHER]
REFRESH MATERIALIZED VIEW my_mv;

--对增量物化视图my_imv进行全量刷新。
-- [OTHER]
REFRESH MATERIALIZED VIEW my_imv;

--删除增量物化视图。
-- [DDL]
DROP MATERIALIZED VIEW my_imv;

--删除全量物化视图。
-- [DDL]
DROP MATERIALIZED VIEW my_mv;

--删除表my_table。
-- [DDL]
DROP TABLE my_table;


================================================================================
-- 来源: 3071_REINDEX.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建表tpcds. customer。
-- [DDL]
CREATE TABLE tpcds.customer ( c_customer_sk INTEGER NOT NULL, c_customer_id CHAR(16) NOT NULL );

--向表中插入多条记录。
-- [DML_INSERT]
INSERT INTO tpcds.customer VALUES (1, 'AAAAAAAABAAAAAAA'),(5, 'AAAAAAAACAAAAAAA'),(10, 'AAAAAAAADAAAAAAA');

--创建一个行存表 tpcds. customer_t1，并在 tpcds. customer_t1表上的c_customer_sk字段创建索引。
-- [DDL]
CREATE TABLE tpcds. customer_t1 ( c_customer_sk integer not null, c_customer_id char(16) not null, c_current_cdemo_sk integer , c_current_hdemo_sk integer , c_current_addr_sk integer , c_first_shipto_date_sk integer , c_first_sales_date_sk integer , c_salutation char(10) , c_first_name char(20) , c_last_name char(30) , c_preferred_cust_flag char(1) , c_birth_day integer , c_birth_month integer , c_birth_year integer , c_birth_country varchar(20) , c_login char(13) , c_email_address char(50) , c_last_review_date char(10) ) WITH (orientation = row);

-- [DDL]
CREATE INDEX tpcds_customer_index1 ON tpcds. customer_t1 (c_customer_sk);

-- [DML_INSERT]
INSERT INTO tpcds. customer_t1 SELECT * FROM tpcds. customer WHERE c_customer_sk < 10;

--重建一个单独索引。
-- [MAINTENANCE]
REINDEX INDEX tpcds. tpcds_customer_index1;

--在线重建一个单独索引。
-- [MAINTENANCE]
REINDEX INDEX CONCURRENTLY tpcds. tpcds_customer_index1;

--重建表 tpcds. customer_t1上的所有索引。
-- [MAINTENANCE]
REINDEX TABLE tpcds. customer_t1;

--在线重建表 tpcds. customer_t1上的所有索引。
-- [MAINTENANCE]
REINDEX TABLE CONCURRENTLY tpcds. customer_t1;

--删除 tpcds. customer_t1表。
-- [DDL]
DROP TABLE tpcds. customer_t1;

--删除表。
-- [DDL]
DROP TABLE tpcds.customer;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 3072_RELEASE SAVEPOINT.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建一个新表。
-- [DDL]
CREATE TABLE tpcds. table1(a int);

--开启事务。
-- [TCL]
START TRANSACTION;

--插入数据。
-- [DML_INSERT]
INSERT INTO tpcds. table1 VALUES (3);

--建立保存点。
-- [TCL]
SAVEPOINT my_savepoint;

--插入数据。
-- [DML_INSERT]
INSERT INTO tpcds. table1 VALUES (4);

--删除保存点。
-- [TCL]
RELEASE SAVEPOINT my_savepoint;

--提交事务。
-- [TCL]
COMMIT;

--查询表的内容，会同时看到3和4。
-- [DQL]
SELECT * FROM tpcds. table1;

--删除表。
-- [DDL]
DROP TABLE tpcds. table1;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 3073_REPLACE.txt
================================================================================

-- [DDL]
CREATE TABLE test(f1 int primary key, f2 int, f3 int);

--插入数据。
-- [DML_INSERT]
INSERT INTO test VALUES(1, 1, 1), (2, 2, 2), (3, 3, 3);

--值替换插入数据。
-- [OTHER]
REPLACE INTO test VALUES(1, 11, 11);

--查询值替换插入的结果
-- [DQL]
SELECT * FROM test WHERE f1 = 1;

--查询替换插入数据。
-- [OTHER]
REPLACE INTO test SELECT 2, 22, 22;

-- [DQL]
SELECT * FROM test WHERE f1 = 2;

--设置指定字段替换插入数据。
-- [OTHER]
REPLACE INTO test SET f1 = f1 + 3, f2 = f1 * 10 + 3, f3 = f2;

-- [DQL]
SELECT * FROM test WHERE f1 = 3;

-- [DDL]
DROP TABLE test;


================================================================================
-- 来源: 3074_RESET.txt
================================================================================

-- [SESSION]
RESET timezone;

--把所有参数设置为缺省值。
-- [SESSION]
RESET ALL;


================================================================================
-- 来源: 3075_REVOKE.txt
================================================================================

-- [DCL_REVOKE]
REVOKE jerry FROM tom ;

-- [DCL_REVOKE]
REVOKE SELECT ON TABLE jerry . t1 FROM tom ;

-- [DCL_REVOKE]
REVOKE EXECUTE ON FUNCTION jerry . fun1 () FROM tom ;

-- [DCL_REVOKE]
REVOKE CONNECT ON database DB1 FROM tom ;


================================================================================
-- 来源: 3076_ROLLBACK.txt
================================================================================

-- [TCL]
START TRANSACTION;

--取消所有更改
-- [TCL]
ROLLBACK;


================================================================================
-- 来源: 3077_ROLLBACK PREPARED.txt
================================================================================

-- [TCL]
BEGIN;

--准备标识符为的trans_test的事务。
-- [TCL]
PREPARE TRANSACTION 'trans_test';

--取消标识符为的trans_test的事务。
-- [TCL]
ROLLBACK PREPARED 'trans_test';


================================================================================
-- 来源: 3078_ROLLBACK TO SAVEPOINT.txt
================================================================================

-- [TCL]
START TRANSACTION;

-- [TCL]
SAVEPOINT my_savepoint;

-- [TCL]
ROLLBACK TO SAVEPOINT my_savepoint;

--游标位置不受保存点回滚的影响。
-- [PLSQL]
DECLARE foo CURSOR FOR SELECT 1 UNION SELECT 2;

-- [TCL]
SAVEPOINT foo;

-- [CURSOR]
FETCH 1 FROM foo;

-- [TCL]
ROLLBACK TO SAVEPOINT foo;

-- [CURSOR]
FETCH 1 FROM foo;

-- [TCL]
RELEASE SAVEPOINT my_savepoint;

-- [TCL]
COMMIT;


================================================================================
-- 来源: 3080_SAVEPOINT.txt
================================================================================

-- [DDL]
CREATE TABLE table1(a int);

--开启事务。
-- [TCL]
START TRANSACTION;

--插入数据。
-- [DML_INSERT]
INSERT INTO table1 VALUES (1);

--建立保存点。
-- [TCL]
SAVEPOINT my_savepoint;

--插入数据。
-- [DML_INSERT]
INSERT INTO table1 VALUES (2);

--回滚保存点。
-- [TCL]
ROLLBACK TO SAVEPOINT my_savepoint;

--插入数据。
-- [DML_INSERT]
INSERT INTO table1 VALUES (3);

--提交事务。
-- [TCL]
COMMIT;

--查询表的内容，会同时看到1和3,不能看到2，因为2被回滚。
-- [DQL]
SELECT * FROM table1;

--删除表。
-- [DDL]
DROP TABLE table1;

--创建一个新表。
-- [DDL]
CREATE TABLE table2(a int);

--开启事务。
-- [TCL]
START TRANSACTION;

--插入数据。
-- [DML_INSERT]
INSERT INTO table2 VALUES (3);

--建立保存点。
-- [TCL]
SAVEPOINT my_savepoint;

--插入数据。
-- [DML_INSERT]
INSERT INTO table2 VALUES (4);

--回滚保存点。
-- [TCL]
RELEASE SAVEPOINT my_savepoint;

--提交事务。
-- [TCL]
COMMIT;

--查询表的内容，会同时看到3和4。
-- [DQL]
SELECT * FROM table2;

--删除表。
-- [DDL]
DROP TABLE table2;


================================================================================
-- 来源: 3081_SECURITY LABEL ON.txt
================================================================================

-- [DDL]
CREATE SECURITY LABEL sec_label 'L1:G4' ;

-- [DDL]
CREATE TABLE tbl ( c1 int , c2 int );

-- [DDL]
CREATE USER bob WITH PASSWORD '********' ;

-- [OTHER]
SECURITY LABEL ON ROLE bob IS 'sec_label' ;

-- [OTHER]
SECURITY LABEL ON TABLE tbl IS 'sec_label' ;

-- [OTHER]
SECURITY LABEL ON COLUMN tbl . c1 IS 'sec_label' ;

-- [OTHER]
SECURITY LABEL ON ROLE bob IS NULL ;

-- [OTHER]
SECURITY LABEL ON TABLE tbl IS NULL ;

-- [OTHER]
SECURITY LABEL ON COLUMN tbl . c1 IS NULL ;

-- [DDL]
DROP SECURITY LABEL sec_label ;

-- [DDL]
DROP TABLE tbl ;

-- [DDL]
DROP USER bob ;


================================================================================
-- 来源: 3082_SELECT.txt
================================================================================

--创建自定义变量
-- [DDL]
CREATE DATABASE user_var dbcompatibility 'b';

--删除数据库
-- [DDL]
DROP DATABASE user_var;

-- [DQL]
SELECT * FROM XMLTABLE( XMLNAMESPACES('nspace1' AS "ns1", 'nspace2' AS "ns2"), -- 声明两个XML的命名空间'nspace1'和'nspace2'及对应的别名"ns1"和"ns2" '/ns1:root/*:child' -- 经row_expression从传入的数据中选取命名空间为'nspace1'的root节点，在选取其下面的所有child节点，忽略child的命名空间；其中ns1为'nspace1'的别名 PASSING xmltype( '<root xmlns="nspace1"> <child> <name>peter</name> <age>11</age> </child> <child xmlns="nspace1"> <name>qiqi</name> <age>12</age> </child> <child xmlns="nspace2"> <name>hacker</name> <age>15</age> </child> </root>') COLUMNS column FOR ORDINALITY, -- 该列为行号列 name varchar(10) path 'ns1:name', -- 从row_expression获取的每个child节点中选取命名空间为'nspace1'的name节点，并将节点中的值转换为varchar(10)返回；其中ns1为'nspace1'的别名 age int);

-- [DDL]
CREATE TABLE test(name varchar, id int, fatherid int);

-- [DML_INSERT]
INSERT INTO test VALUES('A', 1, 0), ('B', 2, 1),('C',3,1),('D',4,1),('E',5,2);

-- [DQL]
SELECT * FROM TEST START WITH id = 1 CONNECT BY prior id = fatherid ORDER SIBLINGS BY id DESC;

-- [DDL]
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- [DDL]
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--向表中插入多条记录。
-- [DML_INSERT]
INSERT INTO tpcds.reason values(3,'AAAAAAAABAAAAAAA','reason 1'),(10,'AAAAAAAABAAAAAAA','reason 2'),(4,'AAAAAAAABAAAAAAA','reason 3'),(10,'AAAAAAAABAAAAAAA','reason 4'),(10,'AAAAAAAABAAAAAAA','reason 5'),(20,'AAAAAAAACAAAAAAA','N%reason 6'),(30,'AAAAAAAACAAAAAAA','W%reason 7');

--先通过子查询得到一张临时表temp_t，然后查询表temp_t中的所有数据。
-- [DQL]
WITH temp_t(name,isdba) AS (SELECT usename,usesuper FROM pg_user) SELECT * FROM temp_t;

--查询 tpcds. reason表的所有r_reason_sk记录，且去除重复。
-- [DQL]
SELECT DISTINCT(r_reason_sk) FROM tpcds. reason;

--LIMIT子句示例：获取表中一条记录。
-- [DQL]
SELECT * FROM tpcds. reason LIMIT 1;

--查询所有记录，且按字母升序排列。
-- [DQL]
SELECT r_reason_desc FROM tpcds. reason ORDER BY r_reason_desc;

--通过表别名，从pg_user和pg_user_status这两张表中获取数据。
-- [DQL]
SELECT a.usename,b.locktime FROM pg_user a,pg_user_status b WHERE a.usesysid=b.roloid;

--FULL JOIN子句示例：将pg_user和pg_user_status这两张表的数据进行全连接显示，即数据的合集。
-- [DQL]
SELECT a.usename,b.locktime,a.usesuper FROM pg_user a FULL JOIN pg_user_status b ON a.usesysid=b.roloid;

--GROUP BY子句示例：根据查询条件过滤，并对结果进行分组。
-- [DQL]
SELECT r_reason_id, AVG(r_reason_sk) FROM tpcds. reason GROUP BY r_reason_id HAVING AVG(r_reason_sk) > 25;

--GROUP BY CUBE子句示例：根据查询条件过滤，并对结果进行分组汇总。
-- [DQL]
SELECT r_reason_id,AVG(r_reason_sk) FROM tpcds. reason GROUP BY CUBE(r_reason_id,r_reason_sk);

--GROUP BY GROUPING SETS子句示例:根据查询条件过滤，并对结果进行分组汇总。
-- [DQL]
SELECT r_reason_id,AVG(r_reason_sk) FROM tpcds. reason GROUP BY GROUPING SETS((r_reason_id,r_reason_sk),r_reason_sk);

--UNION子句示例：将表 tpcds. reason里r_reason_desc字段中的内容以W开头和以N开头的进行合并。
-- [DQL]
SELECT r_reason_sk, tpcds. reason.r_reason_desc FROM tpcds. reason WHERE tpcds. reason.r_reason_desc LIKE 'W%' UNION SELECT r_reason_sk, tpcds. reason.r_reason_desc FROM tpcds. reason WHERE tpcds. reason.r_reason_desc LIKE 'N%';

--NLS_SORT子句示例：中文拼音排序。
-- [DQL]
SELECT * FROM tpcds. reason ORDER BY NLSSORT( r_reason_desc, 'NLS_SORT = SCHINESE_PINYIN_M');

--不区分大小写排序（可选，仅支持纯英文不区分大小写排序）:
-- [DQL]
SELECT * FROM tpcds. reason ORDER BY NLSSORT( r_reason_desc, 'NLS_SORT = generic_m_ci');

--创建分区表 tpcds. reason_p
-- [DDL]
CREATE TABLE tpcds. reason_p ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) PARTITION BY RANGE (r_reason_sk) ( partition P_05_BEFORE values less than (05), partition P_15 values less than (15), partition P_25 values less than (25), partition P_35 values less than (35), partition P_45_AFTER values less than (MAXVALUE) );

--插入数据。
-- [DML_INSERT]
INSERT INTO tpcds. reason_p values(3,'AAAAAAAABAAAAAAA','reason 1'),(10,'AAAAAAAABAAAAAAA','reason 2'),(4,'AAAAAAAABAAAAAAA','reason 3'),(10,'AAAAAAAABAAAAAAA','reason 4'),(10,'AAAAAAAABAAAAAAA','reason 5'),(20,'AAAAAAAACAAAAAAA','reason 6'),(30,'AAAAAAAACAAAAAAA','reason 7');

--PARTITION子句示例：从 tpcds. reason_p的表分区P_05_BEFORE中获取数据。
-- [DQL]
SELECT * FROM tpcds. reason_p PARTITION (P_05_BEFORE);

--PARTITION子句指定多分区示例：从 tpcds. reason_p的表分区P_05_BEFORE，P_15，P_25中获取数据。
-- [DQL]
SELECT * FROM tpcds. reason_p PARTITION (P_05_BEFORE, P_15, P_25) ORDER BY 1;

--GROUP BY子句示例：按r_reason_id分组统计 tpcds. reason_p表中的记录数。
-- [DQL]
SELECT COUNT(*),r_reason_id FROM tpcds. reason_p GROUP BY r_reason_id;

--GROUP BY CUBE子句示例：根据查询条件过滤，并对查询结果分组汇总。
-- [DQL]
SELECT * FROM tpcds. reason GROUP BY CUBE (r_reason_id,r_reason_sk,r_reason_desc);

--GROUP BY GROUPING SETS子句示例：根据查询条件过滤，并对查询结果分组汇总。
-- [DQL]
SELECT * FROM tpcds. reason GROUP BY GROUPING SETS ((r_reason_id,r_reason_sk),r_reason_desc);

--HAVING子句示例：按r_reason_id分组统计 tpcds. reason_p表中的记录，并只显示r_reason_id个数大于2的信息。
-- [DQL]
SELECT COUNT(*) c,r_reason_id FROM tpcds. reason_p GROUP BY r_reason_id HAVING c>2;

--IN子句示例：按r_reason_id分组统计 tpcds. reason_p表中的r_reason_id个数，并只显示r_reason_id值为 AAAAAAAABAAAAAAA或AAAAAAAADAAAAAAA的个数。
-- [DQL]
SELECT COUNT(*),r_reason_id FROM tpcds. reason_p GROUP BY r_reason_id HAVING r_reason_id IN('AAAAAAAABAAAAAAA','AAAAAAAADAAAAAAA');

--INTERSECT子句示例：查询r_reason_id等于AAAAAAAABAAAAAAA，并且r_reason_sk小于5的信息。
-- [DQL]
SELECT * FROM tpcds. reason_p WHERE r_reason_id='AAAAAAAABAAAAAAA' INTERSECT SELECT * FROM tpcds. reason_p WHERE r_reason_sk<5;

--EXCEPT子句示例：查询r_reason_id等于AAAAAAAABAAAAAAA，并且去除r_reason_sk小于4的信息。
-- [DQL]
SELECT * FROM tpcds. reason_p WHERE r_reason_id='AAAAAAAABAAAAAAA' EXCEPT SELECT * FROM tpcds. reason_p WHERE r_reason_sk<4;

--创建表store_returns、customer
-- [DDL]
CREATE TABLE tpcds.store_returns (sr_item_sk int, sr_customer_id varchar(50),sr_customer_sk int);

-- [DDL]
CREATE TABLE tpcds.customer (c_item_sk int, c_customer_id varchar(50),c_customer_sk int);

--通过在where子句中指定"(+)"来实现左连接。
-- [DQL]
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE t1.sr_customer_sk = t2.c_customer_sk(+) ORDER BY 1 DESC LIMIT 1;

--通过在where子句中指定"(+)"来实现右连接。
-- [DQL]
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE t1.sr_customer_sk(+) = t2.c_customer_sk ORDER BY 1 DESC LIMIT 1;

--通过在where子句中指定"(+)"来实现左连接，并且增加连接条件。
-- [DQL]
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE t1.sr_customer_sk = t2.c_customer_sk(+) AND t2.c_customer_sk(+) < 1 ORDER BY 1 LIMIT 1;

--不支持在where子句中指定"(+)"的同时使用内层嵌套AND/OR的表达式。
-- [DQL]
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE NOT(t1.sr_customer_sk = t2.c_customer_sk(+) AND t2.c_customer_sk(+) < 1);

--where子句在不支持表达式宏指定"(+)"会报错。
-- [DQL]
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE (t1.sr_customer_sk = t2.c_customer_sk(+))::bool;

--where子句在表达式的两边都指定"(+)"会报错。
-- [DQL]
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE t1.sr_customer_sk(+) = t2.c_customer_sk(+);

--删除表。
-- [DDL]
DROP TABLE tpcds. reason_p;

--闪回查询示例，使用闪回功能需要设置undo_retention_time参数
--创建表tpcds.time_table
-- [DDL]
CREATE TABLE tpcds.time_table(idx integer, snaptime timestamp, snapcsn bigint, timeDesc character(100));

--向表tpcds.time_table中插入记录
-- [DML_INSERT]
INSERT INTO tpcds.time_table select 1, now(),int8in(xidout(next_csn)), 'time1' from gs_get_next_xid_csn();

-- [DML_INSERT]
INSERT INTO tpcds.time_table select 2, now(),int8in(xidout(next_csn)), 'time2' from gs_get_next_xid_csn();

-- [DML_INSERT]
INSERT INTO tpcds.time_table select 3, now(),int8in(xidout(next_csn)), 'time3' from gs_get_next_xid_csn();

-- [DML_INSERT]
INSERT INTO tpcds.time_table select 4, now(),int8in(xidout(next_csn)), 'time4' from gs_get_next_xid_csn();

-- [DQL]
SELECT * FROM tpcds.time_table;

-- [DML_DELETE]
DELETE tpcds.time_table;

--2021-04-25 17:50:22.311176应该使用tpcds.time_table中第四条snaptime字段值
-- [DQL]
SELECT * FROM tpcds.time_table TIMECAPSULE TIMESTAMP to_timestamp('2021-04-25 17:50:22.311176','YYYY-MM-DD HH24:MI:SS.FF');

--107330 csn应该使用tpcds.time_table中第四条snapcsn字段值
-- [DQL]
SELECT * FROM tpcds.time_table TIMECAPSULE CSN 107330;

--WITH RECURSIVE查询示例：计算从1到100的累加值。
-- [DQL]
WITH RECURSIVE t1(a) AS ( SELECT 100 ), t(n) AS ( VALUES (1) UNION ALL SELECT n+1 FROM t WHERE n < (SELECT max(a) FROM t1) ) SELECT sum(n) FROM t;

-- [DDL]
DROP TABLE t ;

--UNPIVOT子句示例：将表p1的math列和phy列转置为（class，score）行
-- [DDL]
CREATE TABLE p1(id int, math int, phy int);

-- [DML_INSERT]
INSERT INTO p1 values(1,20,30);

-- [DML_INSERT]
INSERT INTO p1 values(2,30,40);

-- [DML_INSERT]
INSERT INTO p1 values(3,40,50);

-- [DQL]
SELECT * FROM p1;

-- [DQL]
SELECT * FROM p1 UNPIVOT(score FOR class IN(math, phy));

--PIVOT子句示例：将表p2的（class，score）行转置为'MATH'列和 'PHY'列
-- [DDL]
CREATE TABLE p2(id int, class varchar(10), score int);

-- [DML_INSERT]
INSERT INTO p2 SELECT * FROM p1 UNPIVOT(score FOR class IN(math, phy));

-- [DQL]
SELECT * FROM p2;

-- [DQL]
SELECT * FROM p2 PIVOT(max(score) FOR class IN ('MATH', 'PHY'));

-- [DDL]
DROP TABLE p1;

-- [DDL]
DROP TABLE p2;

--SKIP LOCKED示例
--step 1:创建astore表并插入数据
-- [DDL]
CREATE TABLE skiplocked_astore(id int, info text) WITH (storage_type=astore);

-- [DML_INSERT]
INSERT INTO skiplocked_astore VALUES (1, 'abc'), (2, 'bcd'), (3, 'cdf'),(3, 'dfg');

--step 2:session1开启事务通过UPDATE锁住skiplocked_astore中id等于1的行
-- [TCL]
BEGIN;

-- [DQL]
SELECT * FROM skiplocked_astore WHERE id = 1 FOR UPDATE;

--STEP 3:session2 使用SKIP LOCKED会跳过被锁行，仅返回加锁成功的行
-- [DQL]
SELECT * FROM skiplocked_astore FOR UPDATE SKIP LOCKED;

--删除表。
-- [DDL]
DROP TABLE tpcds.reason;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 3083_SELECT INTO.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- [DDL]
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--向表中插入多条记录。
-- [DML_INSERT]
INSERT INTO tpcds.reason values(1,'AAAAAAAABAAAAAAA','reason 1'),(2,'AAAAAAAABAAAAAAA','reason 2'),(3,'AAAAAAAABAAAAAAA','reason 3'),(4,'AAAAAAAABAAAAAAA','reason 4'),(4,'AAAAAAAABAAAAAAA','reason 5'),(4,'AAAAAAAACAAAAAAA','reason 6'),(5,'AAAAAAAACAAAAAAA','reason 7');

--将 tpcds. reason表中r_reason_sk小于5的值加入到新建表中。
-- [DQL]
SELECT * INTO tpcds. reason_t1 FROM tpcds. reason WHERE r_reason_sk < 5;

--删除 tpcds. reason_t1表。
-- [DDL]
DROP TABLE tpcds. reason_t1;

--删除表。
-- [DDL]
DROP TABLE tpcds.reason;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 3084_SET.txt
================================================================================

-- [SESSION]
SET search_path TO tpcds, public;

--把日期时间风格设置为传统的 POSTGRES 风格(日在月前)。
-- [SESSION]
SET datestyle TO postgres,dmy;

--SET自定义用户变量的功能。
-- [DDL]
CREATE DATABASE user_var dbcompatibility 'b';

--删除数据库。
-- [DDL]
DROP DATABASE user_var;

-- [DDL]
CREATE DATABASE test_set dbcompatibility 'B';

--删除数据库。
-- [DDL]
DROP DATABASE test_set;


================================================================================
-- 来源: 3085_SET CONSTRAINTS.txt
================================================================================

-- [SESSION]
SET CONSTRAINTS ALL DEFERRED;


================================================================================
-- 来源: 3086_SET ROLE.txt
================================================================================

-- [DDL]
CREATE ROLE paul IDENTIFIED BY ' ******** ';

--设置当前用户为paul。
-- [SESSION]
SET ROLE paul PASSWORD ' ******** ';

--删除用户。
-- [DDL]
DROP USER paul;


================================================================================
-- 来源: 3087_SET SESSION AUTHORIZATION.txt
================================================================================

-- [DDL]
CREATE ROLE paul IDENTIFIED BY ' ******** ';

--设置当前用户为paul。
-- [SESSION]
SET SESSION AUTHORIZATION paul password ' ******** ';

--删除用户。
-- [DDL]
DROP USER paul;


================================================================================
-- 来源: 3088_SET TRANSACTION.txt
================================================================================

-- [DDL]
CREATE DATABASE mysql_compatible_db DBCOMPATIBILITY 'B';

--开启一个事务，设置事务的隔离级别为READ COMMITTED，访问模式为READ ONLY。
-- [TCL]
START TRANSACTION;

-- [SESSION]
SET LOCAL TRANSACTION ISOLATION LEVEL READ COMMITTED READ ONLY;

-- [TCL]
COMMIT;

--设置当前会话的事务隔离级别、读写模式。
--在sql_compatibility = 'B'场景下,b_format_behavior_compat_options设置为set_session_transaction。
-- [SESSION]
SET SESSION TRANSACTION ISOLATION LEVEL READ COMMITTED;

-- [SESSION]
SET SESSION TRANSACTION READ ONLY;

--给sql_compatibility = 'B'的数据库设置全局会话的事务隔离级别、读写模式(当前只能在sql_compatibility = 'B'场景下)。
-- [SESSION]
SET GLOBAL TRANSACTION ISOLATION LEVEL READ COMMITTED;

-- [SESSION]
SET GLOBAL TRANSACTION READ ONLY;

-- [DDL]
DROP DATABASE mysql_compatible_db;


================================================================================
-- 来源: 3089_SHOW.txt
================================================================================

-- [SESSION]
SHOW timezone;

--显示所有参数。
-- [SESSION]
SHOW ALL;

--显示参数名中包含”var”的所有参数
-- [SESSION]
SHOW VARIABLES LIKE var;


================================================================================
-- 来源: 3091_SHRINK.txt
================================================================================

-- [DDL]
CREATE TABLE row_compression ( id int ) WITH (compresstype=2, compress_chunk_size = 512, compress_level = 1);

--插入数据
-- [DML_INSERT]
INSERT INTO row_compression SELECT generate_series(1,1000);

--查看数据
-- [DQL]
SELECT * FROM row_compression;

--shrink整理
-- [OTHER]
SHRINK TABLE row_compression;

--删除表
-- [DDL]
DROP TABLE row_compression;


================================================================================
-- 来源: 3092_SHUTDOWN.txt
================================================================================

-- [OTHER]
SHUTDOWN;

--使用fast模式关闭当前数据库节点。
-- [OTHER]
SHUTDOWN FAST;


================================================================================
-- 来源: 3093_SNAPSHOT.txt
================================================================================

-- [DDL]
CREATE TABLE t1 (id int, name varchar);

-- [DML_INSERT]
INSERT INTO t1 VALUES (1, 'zhangsan');

-- [DML_INSERT]
INSERT INTO t1 VALUES (2, 'lisi');

-- [DML_INSERT]
INSERT INTO t1 VALUES (3, 'wangwu');

-- [DML_INSERT]
INSERT INTO t1 VALUES (4, 'lisa');

-- [DML_INSERT]
INSERT INTO t1 VALUES (5, 'jack');

-- [OTHER]
CREATE SNAPSHOT s1@1.0 comment is 'first version' AS SELECT * FROM t1;

-- [OTHER]
CREATE SNAPSHOT s1@2.0 FROM @1.0 comment is 'inherits from @1.0' USING (INSERT VALUES(6, 'john'), (7, 'tim');

-- [DQL]
SELECT * FROM DB4AISHOT(s1@1.0);

-- [OTHER]
SAMPLE SNAPSHOT s1@2.0 stratify by name as nick at ratio .5;

-- [OTHER]
PURGE SNAPSHOT s1@2.0;

-- [OTHER]
PURGE SNAPSHOT s1nick@2.0;

-- [OTHER]
PURGE SNAPSHOT s1@1.0;

-- [DDL]
DROP TABLE t1;


================================================================================
-- 来源: 3094_START TRANSACTION.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建表 tpcds. reason。
-- [DDL]
CREATE TABLE tpcds. reason (c1 int, c2 int);

--以默认方式启动事务。
-- [TCL]
START TRANSACTION;

-- [DQL]
SELECT * FROM tpcds. reason;

--以默认方式启动事务。
-- [TCL]
BEGIN;

-- [DQL]
SELECT * FROM tpcds. reason;

--以隔离级别为READ COMMITTED，读/写方式启动事务。
-- [TCL]
START TRANSACTION ISOLATION LEVEL READ COMMITTED READ WRITE;

-- [DQL]
SELECT * FROM tpcds. reason;

-- [TCL]
COMMIT;

--删除表 tpcds. reason。
-- [DDL]
DROP TABLE tpcds. reason;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds;


================================================================================
-- 来源: 3096_TIMECAPSULE TABLE.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

-- 删除表tpcds.reason_t
-- [DDL]
DROP TABLE IF EXISTS tpcds.reason_t2;

-- 创建表tpcds.reason_t
-- [DDL]
CREATE TABLE tpcds.reason_t2 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) )with(storage_type = ustore);

--向表tpcds.reason_t2中插入记录
-- [DML_INSERT]
INSERT INTO tpcds.reason_t2 VALUES (1, 'AA', 'reason1'),(2, 'AB', 'reason2'),(3, 'AC', 'reason3');

--清空tpcds.reason_t2表中的数据
-- [DML_TRUNCATE]
TRUNCATE TABLE tpcds.reason_t2;

--查询tpcds.reason_t2表中的数据
-- [DQL]
SELECT * FROM tpcds.reason_t2;

--执行闪回TRUNCATE
-- [OTHER]
TIMECAPSULE TABLE tpcds.reason_t2 to BEFORE TRUNCATE;

-- [DQL]
SELECT * FROM tpcds.reason_t2;

--删除表tpcds.reason_t
-- [DDL]
DROP TABLE tpcds.reason_t2;

--执行闪回DROP
-- [OTHER]
TIMECAPSULE TABLE tpcds.reason_t2 to BEFORE DROP;

-- 清空回收站，删除SCHEMA。
-- [OTHER]
PURGE RECYCLEBIN;

-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 3097_TRUNCATE.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- [DDL]
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--向表中插入多条记录。
-- [DML_INSERT]
INSERT INTO tpcds.reason values(1,'AAAAAAAABAAAAAAA','reason 1'),(5,'AAAAAAAABAAAAAAA','reason 2'),(15,'AAAAAAAABAAAAAAA','reason 3'),(25,'AAAAAAAABAAAAAAA','reason 4'),(35,'AAAAAAAABAAAAAAA','reason 5'),(45,'AAAAAAAACAAAAAAA','reason 6'),(55,'AAAAAAAACAAAAAAA','reason 7');

--创建表。
-- [DDL]
CREATE TABLE tpcds. reason_t1 AS TABLE tpcds. reason;

--清空表 tpcds. reason_t1。
-- [DML_TRUNCATE]
TRUNCATE TABLE tpcds. reason_t1;

--删除表。
-- [DDL]
DROP TABLE tpcds. reason_t1;

-- [DDL]
CREATE TABLE tpcds. reason_p ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) )PARTITION BY RANGE (r_reason_sk) ( partition p_05_before values less than (05), partition p_15 values less than (15), partition p_25 values less than (25), partition p_35 values less than (35), partition p_45_after values less than (MAXVALUE) );

--插入数据。
-- [DML_INSERT]
INSERT INTO tpcds. reason_p SELECT * FROM tpcds. reason;

--清空分区p_05_before。
-- [DDL]
ALTER TABLE tpcds. reason_p TRUNCATE PARTITION p_05_before;

--清空分区p_15。
-- [DDL]
ALTER TABLE tpcds. reason_p TRUNCATE PARTITION for (15);

--清空分区表。
-- [DML_TRUNCATE]
TRUNCATE TABLE tpcds. reason_p;

--删除表。
-- [DDL]
DROP TABLE tpcds. reason_p;

--删除表。
-- [DDL]
DROP TABLE tpcds.reason;

--删除SCHEMA。
-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 3099_UPDATE.txt
================================================================================

-- [DDL]
CREATE TABLE tbl_test1(id int, info varchar(10));

-- [DML_INSERT]
INSERT INTO tbl_test1 VALUES (1, 'A'), (2, 'B');

--修改tbl_test1表中所有数据的info列。
-- [DML_UPDATE]
UPDATE tbl_test1 SET info = 'aa';

--查询tbl_test1表。
-- [DQL]
SELECT * FROM tbl_test1;

-- [DML_UPDATE]
UPDATE tbl_test1 SET info = 'bb' WHERE id = 2;

--查询tbl_test1表。
-- [DQL]
SELECT * FROM tbl_test1;

-- [DML_UPDATE]
UPDATE tbl_test1 SET info = 'ABC' WHERE id = 1 RETURNING info;

-- 删除tbl_test1表。
-- [DDL]
DROP TABLE tbl_test1;

-- [DDL]
CREATE TABLE test_grade ( sid int, --学号 name varchar(50), --姓名 score char, --成绩 examtime date, --考试时间 last_exam boolean --是否是最后一次考试 );

--插入数据。
-- [DML_INSERT]
INSERT INTO test_grade VALUES (1,'Scott','A','2008-07-08',1),(2,'Ben','D','2008-07-08',1),(3,'Jack','D','2008-07-08',1);

--查询。
-- [DQL]
SELECT * FROM test_grade;

--2008-08-25 Ben参加了补考,成绩为B，正常步骤需要先修改last_exam为否,然后插入2008-08-25这一天的成绩。
-- [DQL]
WITH old_exam AS ( UPDATE test_grade SET last_exam = 0 WHERE sid = 2 AND examtime = '2008-07-08' RETURNING sid, name ) INSERT INTO test_grade VALUES ( ( SELECT sid FROM old_exam ), ( SELECT name FROM old_exam ), 'B', '2008-08-25', 1 );

--查询。
-- [DQL]
SELECT * FROM test_grade;

--删除。
-- [DDL]
DROP TABLE test_grade;


================================================================================
-- 来源: 3101_VACUUM.txt
================================================================================

-- [DDL]
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- [DDL]
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--向表中插入多条记录。
-- [DML_INSERT]
INSERT INTO tpcds.reason values(1,'AAAAAAAABAAAAAAA','reason 1'),(2,'AAAAAAAABAAAAAAA','reason 2');

--在表 tpcds. reason上创建索引。
-- [DDL]
CREATE UNIQUE INDEX ds_reason_index1 ON tpcds. reason(r_reason_sk);

--对带索引的表 tpcds. reason执行VACUUM操作。
-- [MAINTENANCE]
VACUUM (VERBOSE, ANALYZE) tpcds. reason;

--删除索引。
-- [DDL]
DROP INDEX tpcds.ds_reason_index1 CASCADE;

-- [DDL]
DROP TABLE tpcds. reason;

-- [DDL]
DROP SCHEMA tpcds CASCADE;


================================================================================
-- 来源: 3111_file_3111.txt
================================================================================

-- [DQL]
select $$it's an example$$;


================================================================================
-- 来源: 3120_SQL.txt
================================================================================

-- [EXPLAIN]
explain select * from t1 where not exists(select * from t2 where t1.c1 = t2.c1);


================================================================================
-- 来源: 3130_file_3130.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE array_proc AS DECLARE TYPE ARRAY_INTEGER IS VARRAY ( 1024 ) OF INTEGER ;

-- [PLSQL]
CALL array_proc ();

-- [DDL]
DROP PROCEDURE array_proc ;


================================================================================
-- 来源: 3131_file_3131.txt
================================================================================

-- [SESSION]
set behavior_compat_options = '';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = '';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set a_format_version='10c';

-- [SESSION]
set a_format_dev_version='s1';

-- [SESSION]
set behavior_compat_options = '';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set a_format_version='10c';

-- [SESSION]
set a_format_dev_version='s1';

-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set a_format_version='10c';

-- [SESSION]
set a_format_dev_version='s1';

-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = '';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = '';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- n大于数组元素个数，清空数组元素
-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = '';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- 数组未初始化报错
-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = '';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- 数组未初始化
-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- 数组未初始化报错
-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = '';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = '';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- 开启参数varray_compat后报错
-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type array_integer is varray(10) of integer;

-- [SESSION]
set behavior_compat_options = '';

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回NULL
-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 开启varray_copmat参数后，数组未初始化报错
-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- [SESSION]
set behavior_compat_options = '';

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回NULL
-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 开启varray_copmat参数后，数组未初始化报错
-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- [SESSION]
set behavior_compat_options = '';

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 下标越界，大于数组范围
-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 开启varray_copmat参数后，数组未初始化报错
-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- [SESSION]
set behavior_compat_options = '';

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 下标越界，大于数组范围
-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 开启varray_copmat参数后，数组未初始化报错
-- [SESSION]
set behavior_compat_options = 'varray_compat';

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- [PLSQL]
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回false
-- [PLSQL]
declare type varr is varray(10) of varchar(3);


================================================================================
-- 来源: 3133_file_3133.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER ;

-- [PLSQL]
CALL table_proc ();

-- [DDL]
DROP PROCEDURE table_proc ;

-- [DDL]
CREATE OR REPLACE PROCEDURE nest_table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER ;

-- [PLSQL]
CALL nest_table_proc ();

-- [DDL]
DROP PROCEDURE nest_table_proc ;

-- [DDL]
CREATE OR REPLACE PROCEDURE index_table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER INDEX BY INTEGER ;

-- [PLSQL]
CALL index_table_proc ();

-- [DDL]
DROP PROCEDURE index_table_proc ;

-- [DDL]
CREATE OR REPLACE PROCEDURE nest_table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER INDEX BY INTEGER ;

-- [PLSQL]
CALL nest_table_proc ();

-- [DDL]
DROP PROCEDURE nest_table_proc ;


================================================================================
-- 来源: 3134_file_3134.txt
================================================================================

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of varchar2;

-- [PLSQL]
declare type nest is table of varchar2 index by varchar2;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type t1 is table of int index by int;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type t1 is table of int index by int;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type t1 is table of int index by varchar;

-- [PLSQL]
declare type nest is table of int;

-- [PLSQL]
declare type t1 is table of int index by int;

-- [PLSQL]
declare type nest is table of int;

-- [DDL]
create or replace procedure p1 () gaussdb -# as gaussdb $ # type t1 is table of int ;

-- [PLSQL]
call p1 ();

-- [DDL]
drop procedure if exists p1 ();

-- [DDL]
create or replace procedure p1 () is gaussdb $ # type rec is record ( c1 int , c2 int );

-- [PLSQL]
call p1 ();

-- [DDL]
drop procedure if exists p1 ();

-- [DDL]
create or replace procedure p1 () gaussdb -# as gaussdb $ # type t1 is table of int index by int ;

-- [PLSQL]
call p1 ();

-- [DDL]
drop procedure if exists p1 ();

-- [DDL]
create or replace procedure p1 () is gaussdb $ # type rec is record ( c1 int , c2 int );

-- [PLSQL]
call p1 ();

-- [DDL]
drop procedure if exists p1 ();


================================================================================
-- 来源: 3135_record.txt
================================================================================

-- [DDL]
create table emp_rec ( gaussdb ( # empno numeric ( 4 , 0 ) not null , gaussdb ( # ename varchar ( 10 ) gaussdb ( # );

-- [DML_INSERT]
insert into emp_rec values ( 111 , 'aaa' ), ( 222 , 'bbb' ), ( 333 , 'ccc' );

-- [OTHER]
\ d emp_rec Table "public.emp_rec" Column | Type | Modifiers --------+-----------------------+----------- empno | numeric ( 4 , 0 ) | not null ename | character varying ( 10 ) | --演示在函数中对record进行操作。

-- [DDL]
CREATE OR REPLACE FUNCTION regress_record ( p_w VARCHAR2 ) RETURNS VARCHAR2 AS $$ gaussdb $ # DECLARE gaussdb $ # --声明一个record类型. gaussdb $ # type rec_type is record ( name varchar2 ( 100 ), epno int );

-- [PLSQL]
CALL regress_record ( 'abc' );

-- [DDL]
DROP FUNCTION regress_record ;

-- [DDL]
DROP TABLE emp_rec ;

-- [DDL]
create type rec_type is ( c1 int , c2 int );

-- [SESSION]
set behavior_compat_options = 'proc_outparam_override' ;

-- [DDL]
create or replace function func ( a in int ) return rec_type is gaussdb $ # r rec_type ;

-- [PLSQL]
call func ( 0 );

-- [DDL]
drop function func ;

-- [DDL]
drop type rec_type ;

-- [SESSION]
set behavior_compat_options = 'proc_outparam_override' ;

-- [DDL]
create or replace function func ( a out int ) return record is gaussdb $ # type rc is record ( c1 int , c2 int );

-- [PLSQL]
call func ( 1 );

-- [DDL]
drop function func ;


================================================================================
-- 来源: 3138_file_3138.txt
================================================================================

-- [TCL]
BEGIN NULL ;

-- [TCL]
BEGIN dbe_output . print_line ( 'hello world!' );

-- [PLSQL]
DECLARE my_var VARCHAR2 ( 30 );


================================================================================
-- 来源: 3144_file_3144.txt
================================================================================

-- [PLSQL]
DECLARE emp_id INTEGER : = 7788 ;

-- [PLSQL]
DECLARE emp_id INTEGER :=7788;


================================================================================
-- 来源: 3145_file_3145.txt
================================================================================

-- [DDL]
CREATE TYPE o1 AS ( a int , b int );

-- [PLSQL]
DECLARE TYPE r1 is VARRAY ( 10 ) of o1 ;

-- [DDL]
DROP TABLE IF EXISTS customers;

-- [PLSQL]
DECLARE type id_list is varray(6) of customers.id%type;

-- [DDL]
CREATE TABLE test(a integer);

-- [DDL]
CREATE OR REPLACE FUNCTION check_test() RETURNS integer language plpgsql AS $function$ DECLARE b integer;

-- [DQL]
SELECT check_test();


================================================================================
-- 来源: 3146_file_3146.txt
================================================================================

-- [DDL]
CREATE SCHEMA hr ;

-- [SESSION]
SET CURRENT_SCHEMA = hr ;

-- [DDL]
CREATE TABLE staffs ( section_id INTEGER , salary INTEGER );

-- [DML_INSERT]
INSERT INTO staffs VALUES ( 30 , 10 );

-- [DML_INSERT]
INSERT INTO staffs VALUES ( 30 , 20 );

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_staffs ( section NUMBER ( 6 ), salary_sum out NUMBER ( 8 , 2 ), staffs_count out INTEGER ) IS BEGIN SELECT sum ( salary ), count ( * ) INTO salary_sum , staffs_count FROM hr . staffs where section_id = section ;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_return AS v_num NUMBER ( 8 , 2 );

-- [PLSQL]
CALL proc_return ();

-- [DDL]
DROP PROCEDURE proc_staffs ;

-- [DDL]
DROP PROCEDURE proc_return ;


================================================================================
-- 来源: 3148_file_3148.txt
================================================================================

-- [DDL]
DROP SCHEMA IF EXISTS hr CASCADE;

-- [DDL]
CREATE SCHEMA hr;

-- [SESSION]
SET CURRENT_SCHEMA = hr;

-- [DDL]
CREATE TABLE staffs ( staff_id NUMBER, first_name VARCHAR2, salary NUMBER );

-- [DML_INSERT]
INSERT INTO staffs VALUES (200, 'mike', 5800);

-- [DML_INSERT]
INSERT INTO staffs VALUES (201, 'lily', 3000);

-- [DML_INSERT]
INSERT INTO staffs VALUES (202, 'john', 4400);

--从动态语句检索值（INTO 子句）：
-- [PLSQL]
DECLARE staff_count VARCHAR2(20);

--传递并检索值（INTO子句用在USING子句前）：
-- [DDL]
CREATE OR REPLACE PROCEDURE dynamic_proc AS staff_id NUMBER(6) := 200;

--调用存储过程
-- [PLSQL]
CALL dynamic_proc();

--删除存储过程
-- [DDL]
DROP PROCEDURE dynamic_proc;

-- [DDL]
CREATE SCHEMA hr;

-- [SESSION]
SET CURRENT_SCHEMA = hr;

-- [DDL]
CREATE TABLE staffs ( section_id NUMBER, first_name VARCHAR2, phone_number VARCHAR2, salary NUMBER );

-- [DML_INSERT]
INSERT INTO staffs VALUES (30, 'mike', '13567829252', 5800);

-- [DML_INSERT]
INSERT INTO staffs VALUES (40, 'john', '17896354637', 4000);

-- [PLSQL]
DECLARE name VARCHAR2(20);


================================================================================
-- 来源: 3149_file_3149.txt
================================================================================

-- [DDL]
CREATE TABLE sections_t1 ( section NUMBER ( 4 ) , section_name VARCHAR2 ( 30 ), manager_id NUMBER ( 6 ), place_id NUMBER ( 4 ) );

-- [PLSQL]
DECLARE section NUMBER ( 4 ) : = 280 ;

-- [DQL]
SELECT * FROM sections_t1 ;

-- [DDL]
DROP TABLE sections_t1 ;


================================================================================
-- 来源: 3150_file_3150.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_add ( param1 in INTEGER, param2 out INTEGER, param3 in INTEGER ) AS BEGIN param2:= param1 + param3;

-- [PLSQL]
DECLARE input1 INTEGER:=1;

--删除存储过程
-- [DDL]
DROP PROCEDURE proc_add;


================================================================================
-- 来源: 3151_file_3151.txt
================================================================================

-- [DDL]
DROP SCHEMA IF EXISTS hr CASCADE;

-- [DDL]
CREATE SCHEMA hr;

-- [SESSION]
SET CURRENT_SCHEMA = hr;

-- [DDL]
CREATE TABLE staffs ( staff_id NUMBER, first_name VARCHAR2, salary NUMBER );

-- [DML_INSERT]
INSERT INTO staffs VALUES (200, 'mike', 5800);

-- [DML_INSERT]
INSERT INTO staffs VALUES (201, 'lily', 3000);

-- [DML_INSERT]
INSERT INTO staffs VALUES (202, 'john', 4400);

--创建存储过程dynamic_proc
-- [DDL]
CREATE OR REPLACE PROCEDURE dynamic_proc AS staff_id NUMBER(6) := 200;

--调用存储过程
-- [PLSQL]
CALL dynamic_proc();

--删除存储过程
-- [DDL]
DROP PROCEDURE dynamic_proc;


================================================================================
-- 来源: 3155_RETURN NEXTRETURN QUERY.txt
================================================================================

-- [DDL]
DROP TABLE t1 ;

-- [DDL]
CREATE TABLE t1 ( a int );

-- [DML_INSERT]
INSERT INTO t1 VALUES ( 1 ),( 10 );

-- [DDL]
CREATE OR REPLACE FUNCTION fun_for_return_next () RETURNS SETOF t1 AS $$ DECLARE r t1 % ROWTYPE ;

-- [PLSQL]
call fun_for_return_next ();

-- [DDL]
CREATE OR REPLACE FUNCTION fun_for_return_query () RETURNS SETOF t1 AS $$ DECLARE r t1 % ROWTYPE ;

-- [PLSQL]
call fun_for_return_query ();


================================================================================
-- 来源: 3156_file_3156.txt
================================================================================

-- [PLSQL]
DECLARE v_user_id integer default 1 ;

-- [PLSQL]
DECLARE v_user_id integer default 1 ;

-- [PLSQL]
DECLARE v_user_id integer default 1 ;

-- [PLSQL]
DECLARE v_user_id integer default NULL ;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_control_structure ( i in integer ) AS BEGIN IF i > 0 THEN raise info 'i:% is greater than 0. ' , i ;

-- [PLSQL]
CALL proc_control_structure ( 3 );

-- [DDL]
DROP PROCEDURE proc_control_structure ;


================================================================================
-- 来源: 3157_file_3157.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_loop ( i in integer , count out integer ) AS BEGIN count : = 0 ;

-- [PLSQL]
CALL proc_loop ( 10 , 5 );

-- [DDL]
CREATE TABLE integertable ( c1 integer ) ;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_while_loop ( maxval in integer ) AS DECLARE i int : = 1 ;

-- [PLSQL]
CALL proc_while_loop ( 10 );

-- [DDL]
DROP PROCEDURE proc_while_loop ;

-- [DDL]
DROP TABLE integertable ;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_for_loop () AS BEGIN FOR I IN 0 .. 5 LOOP DBE_OUTPUT . PRINT_LINE ( 'It is ' || to_char ( I ) || ' time;

-- [PLSQL]
CALL proc_for_loop ();

-- [DDL]
DROP PROCEDURE proc_for_loop ;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_for_loop_query () AS record VARCHAR2 ( 50 );

-- [PLSQL]
CALL proc_for_loop_query ();

-- [DDL]
DROP PROCEDURE proc_for_loop_query ;

-- [DDL]
CREATE TABLE hdfs_t1 ( title NUMBER ( 6 ), did VARCHAR2 ( 20 ), data_period VARCHAR2 ( 25 ), kind VARCHAR2 ( 25 ), interval VARCHAR2 ( 20 ), time DATE , isModified VARCHAR2 ( 10 ) );

-- [DML_INSERT]
INSERT INTO hdfs_t1 VALUES ( 8 , 'Donald' , 'OConnell' , 'DOCONNEL' , '650.507.9833' , to_date ( '21-06-1999' , 'dd-mm-yyyy' ), 'SH_CLERK' );

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_forall () AS BEGIN FORALL i IN 100 .. 120 update hdfs_t1 set title = title + 100 * i ;

-- [PLSQL]
CALL proc_forall ();

-- [DQL]
SELECT * FROM hdfs_t1 ;

-- [DDL]
DROP PROCEDURE proc_forall ;

-- [DDL]
DROP TABLE hdfs_t1 ;


================================================================================
-- 来源: 3158_file_3158.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_case_branch ( pi_result in integer , pi_return out integer ) AS BEGIN CASE pi_result WHEN 1 THEN pi_return : = 111 ;

-- [PLSQL]
CALL proc_case_branch ( 3 , 0 );

-- [DDL]
DROP PROCEDURE proc_case_branch ;


================================================================================
-- 来源: 3159_file_3159.txt
================================================================================

-- [PLSQL]
DECLARE v_num integer default NULL;


================================================================================
-- 来源: 3160_file_3160.txt
================================================================================

-- [DDL]
CREATE TABLE mytab ( id INT , firstname VARCHAR ( 20 ), lastname VARCHAR ( 20 )) ;

-- [DML_INSERT]
INSERT INTO mytab ( firstname , lastname ) VALUES ( 'Tom' , 'Jones' );

-- [DDL]
CREATE FUNCTION fun_exp () RETURNS INT AS $$ DECLARE x INT : = 0 ;

-- [PLSQL]
call fun_exp ();

-- [DQL]
select * from mytab ;

-- [DDL]
DROP FUNCTION fun_exp ();

-- [DDL]
DROP TABLE mytab ;

-- [DDL]
CREATE TABLE db ( a INT , b TEXT );

-- [DDL]
CREATE FUNCTION merge_db ( key INT , data TEXT ) RETURNS VOID AS $$ BEGIN LOOP --第一次尝试更新key UPDATE db SET b = data WHERE a = key ;

-- [DQL]
SELECT merge_db ( 1 , 'david' );

-- [DQL]
SELECT merge_db ( 1 , 'dennis' );

-- [DDL]
DROP FUNCTION merge_db ;

-- [DDL]
DROP TABLE db ;


================================================================================
-- 来源: 3161_GOTO.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE GOTO_test () AS DECLARE v1 int ;

-- [PLSQL]
call GOTO_test ();


================================================================================
-- 来源: 3162_file_3162.txt
================================================================================

-- [DDL]
DROP TABLE IF EXISTS EXAMPLE1;

-- [DDL]
CREATE TABLE EXAMPLE1(COL1 INT);

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE() AS BEGIN FOR i IN 0..20 LOOP INSERT INTO EXAMPLE1(COL1) VALUES (i);

-- [PLSQL]
call TRANSACTION_EXAMPLE();

-- [DDL]
CREATE OR REPLACE PROCEDURE TEST_COMMIT_INSERT_EXCEPTION_ROLLBACK() AS BEGIN DROP TABLE IF EXISTS TEST_COMMIT;

-- [PLSQL]
call TEST_COMMIT_INSERT_EXCEPTION_ROLLBACK();

-- [TCL]
BEGIN;

-- [DDL]
CREATE OR REPLACE PROCEDURE TEST_COMMIT2() IS BEGIN DROP TABLE IF EXISTS TEST_COMMIT;

-- [PLSQL]
call TEST_COMMIT2();

-- [DDL]
CREATE OR REPLACE PROCEDURE exec_func3(RET_NUM OUT INT) AS BEGIN RET_NUM := 1+1;

-- [PLSQL]
call exec_func3('');

-- [DDL]
CREATE OR REPLACE PROCEDURE exec_func4(ADD_NUM IN INT) AS SUM_NUM INT;

-- [PLSQL]
call exec_func4();

-- [SESSION]
SHOW explain_perf_mode;

-- [SESSION]
SHOW enable_force_vector_engine;

-- [DDL]
CREATE OR REPLACE PROCEDURE GUC_ROLLBACK() AS BEGIN SET enable_force_vector_engine = on;

-- [PLSQL]
call GUC_ROLLBACK();

-- [SESSION]
SHOW explain_perf_mode;

-- [SESSION]
SHOW enable_force_vector_engine;

-- [SESSION]
SET enable_force_vector_engine = off;

-- [DDL]
CREATE OR REPLACE FUNCTION FUNCTION_EXAMPLE1() RETURN INT AS EXP INT;

-- [DDL]
CREATE OR REPLACE FUNCTION FUNCTION_EXAMPLE2() RETURN INT AS EXP INT;

-- [DDL]
CREATE OR REPLACE FUNCTION FUNCTION_TRI_EXAMPLE2() RETURN TRIGGER AS EXP INT;

-- [DDL]
CREATE TRIGGER TRIGGER_EXAMPLE AFTER DELETE ON EXAMPLE1 FOR EACH ROW EXECUTE PROCEDURE FUNCTION_TRI_EXAMPLE2();

-- [DML_DELETE]
DELETE FROM EXAMPLE1;

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE1() IMMUTABLE AS EXP INT;

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE2(EXP_OUT OUT INT) AS EXP INT;

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE3() AS BEGIN FOR i IN 0..20 LOOP INSERT INTO EXAMPLE1 (col1) VALUES (i);

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE4() SET ARRAY_NULLS TO "ON" AS BEGIN FOR i IN 0..20 LOOP INSERT INTO EXAMPLE1 (col1) VALUES (i);

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE5(INTIN IN INT, INTOUT OUT INT) AS BEGIN INTOUT := INTIN + 1;

-- [DDL]
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE6() AS CURSOR CURSOR1(EXPIN INT) IS SELECT TRANSACTION_EXAMPLE5(EXPIN);

-- [DDL]
CREATE OR REPLACE PROCEDURE exec_func1() AS BEGIN CREATE TABLE TEST_exec(A INT);

-- [DDL]
CREATE OR REPLACE PROCEDURE exec_func2() AS BEGIN EXECUTE exec_func1();

-- [DDL]
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE1() AS BEGIN INSERT INTO EXAMPLE1 VALUES(1);

-- [PLSQL]
call STP_SAVEPOINT_EXAMPLE1();

-- [DDL]
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE2() AS BEGIN INSERT INTO EXAMPLE1 VALUES(2);

-- [TCL]
BEGIN;

-- [DDL]
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE3() AS BEGIN INSERT INTO EXAMPLE1 VALUES(2);

-- [TCL]
BEGIN;

-- [DDL]
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE4() AS BEGIN INSERT INTO EXAMPLE1 VALUES(1);

-- [TCL]
BEGIN;


================================================================================
-- 来源: 3168_file_3168.txt
================================================================================

-- [DDL]
DROP SCHEMA IF EXISTS hr CASCADE;

-- [DDL]
CREATE SCHEMA hr;

-- [SESSION]
SET current_schema = hr;

-- [DDL]
DROP TABLE IF EXISTS sections;

-- [DDL]
DROP TABLE IF EXISTS staffs;

-- [DDL]
DROP TABLE IF EXISTS department;

--创建部门表
-- [DDL]
CREATE TABLE sections( section_name varchar(100), place_id int, section_id int );

-- [DML_INSERT]
INSERT INTO sections VALUES ('hr',1,1);

--创建员工表
-- [DDL]
CREATE TABLE staffs( staff_id number(6), salary number(8,2), section_id int, first_name varchar(20) );

-- [DML_INSERT]
INSERT INTO staffs VALUES (1,100,1,'Tom');

--创建部门表
-- [DDL]
CREATE TABLE department( section_id int );

-- [DDL]
CREATE OR REPLACE PROCEDURE cursor_proc1 () AS DECLARE DEPT_NAME VARCHAR ( 100 );

-- [PLSQL]
CALL cursor_proc1 ();

-- [OTHER]
hr ---1 hr ---1 hr ---1 cursor_proc1 -------------- ( 1 row )

-- [DDL]
DROP PROCEDURE cursor_proc1 ;

-- [DDL]
CREATE TABLE hr . staffs_t1 AS TABLE hr . staffs ;

-- [DDL]
CREATE OR REPLACE PROCEDURE cursor_proc2 () AS DECLARE V_EMPNO NUMBER ( 6 );

-- [PLSQL]
CALL cursor_proc2 ();

-- [DDL]
DROP PROCEDURE cursor_proc2 ;

-- [DDL]
DROP TABLE hr . staffs_t1 ;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_sys_ref ( O OUT SYS_REFCURSOR ) IS C1 SYS_REFCURSOR ;

-- [PLSQL]
DECLARE C1 SYS_REFCURSOR ;

-- [OTHER]
1 1 ANONYMOUS BLOCK EXECUTE --删除存储过程

-- [DDL]
DROP PROCEDURE proc_sys_ref ;


================================================================================
-- 来源: 3169_file_3169.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_cursor3 () AS DECLARE V_DEPTNO NUMBER ( 4 ) : = 100 ;

-- [PLSQL]
CALL proc_cursor3 ();

-- [DDL]
DROP PROCEDURE proc_cursor3 ;


================================================================================
-- 来源: 3170_file_3170.txt
================================================================================

-- [TCL]
BEGIN FOR ROW_TRANS IN SELECT first_name FROM hr . staffs LOOP DBE_OUTPUT . PRINT_LINE ( ROW_TRANS . first_name );

-- [OTHER]
Tom ANONYMOUS BLOCK EXECUTE --创建表

-- [DDL]
CREATE TABLE integerTable1 ( A INTEGER );

-- [DDL]
CREATE TABLE integerTable2 ( B INTEGER );

-- [DML_INSERT]
INSERT INTO integerTable2 VALUES ( 2 );

-- [PLSQL]
DECLARE CURSOR C1 IS SELECT A FROM integerTable1 ;

-- [DDL]
DROP TABLE integerTable1 ;

-- [DDL]
DROP TABLE integerTable2 ;


================================================================================
-- 来源: 3178_DBE_COMPRESSION.txt
================================================================================

-- [DDL]
alter database set ilm = on ;

-- [DDL]
CREATE user user1 IDENTIFIED BY 'Gauss_zzy123' ;

-- [DDL]
CREATE user user2 IDENTIFIED BY 'Gauss_zzy123' ;

-- [SESSION]
SET ROLE user1 PASSWORD 'Gauss_zzy123' ;

-- [DDL]
CREATE TABLE TEST_DATA ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- [DQL]
SELECT DBE_COMPRESSION . GET_COMPRESSION_TYPE ( 'user1' , 'test_data' , '(0,1)' , NULL );


================================================================================
-- 来源: 3180_DBE_HEAT_MAP.txt
================================================================================

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE Schema HEAT_MAP_DATA ;

-- [SESSION]
SET current_schema = HEAT_MAP_DATA ;

-- [DDL]
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1' ;

-- [DDL]
CREATE TABLE HEAT_MAP_DATA . heat_map_table ( id INT , value TEXT ) TABLESPACE example1 ;

-- [DML_INSERT]
INSERT INTO HEAT_MAP_DATA . heat_map_table VALUES ( 1 , 'test_data_row_1' );

-- [DQL]
SELECT * from DBE_HEAT_MAP . ROW_HEAT_MAP ( owner => 'heat_map_data' , segment_name => 'heat_map_table' , partition_name => NULL , ctid => '(0,1)' );


================================================================================
-- 来源: 3181_DBE_ILM.txt
================================================================================

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE Schema ILM_DATA ;

-- [SESSION]
SET current_schema = ILM_DATA ;

-- [DDL]
CREATE SEQUENCE ILM_DATA . ORDER_TABLE_SE_ORDER_ID MINVALUE 1 ;

-- [DDL]
CREATE OR REPLACE PROCEDURE ILM_DATA . ORDER_TABLE_CREATE_DATA ( NUM INTEGER ) IS BEGIN FOR X IN 1 .. NUM LOOP INSERT INTO ORDER_TABLE VALUES ( ORDER_TABLE_SE_ORDER_ID . nextval , '零食大礼包A' , NOW ());

-- [DDL]
CREATE TABLE ILM_DATA . ORDER_TABLE ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) WITH ( STORAGE_TYPE = ASTORE ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- [DQL]
SELECT ORDER_ID , DBE_COMPRESSION . GET_COMPRESSION_TYPE ( 'ilm_data' , 'order_table' , ctid :: text , NULL ) FROM ILM_DATA . ORDER_TABLE ;

-- [DQL]
SELECT ORDER_ID , DBE_HEAT_MAP . ROW_HEAT_MAP ( 'ilm_data' , 'order_table' , NULL , ctid :: text ) FROM ILM_DATA . ORDER_TABLE ;

-- [DQL]
SELECT ORDER_ID , DBE_COMPRESSION . GET_COMPRESSION_TYPE ( 'ilm_data' , 'order_table' , ctid :: text , NULL ) FROM ILM_DATA . ORDER_TABLE ;

-- [PLSQL]
CALL DBE_ILM . STOP_ILM ( - 1 , true , NULL );


================================================================================
-- 来源: 3182_DBE_ILM_ADMIN.txt
================================================================================

-- [OTHER]
DBE_ILM_ADMIN . DISABLE_ILM ();

-- [OTHER]
DBE_ILM_ADMIN . ENABLE_ILM ();

-- [PLSQL]
CALL DBE_ILM_ADMIN . CUSTOMIZE_ILM ( 1 , 15 );

-- [DQL]
select * from gs_adm_ilmparameters ;


================================================================================
-- 来源: 3186_DBE_PROFILER.txt
================================================================================

-- [DDL]
DROP TABLE IF EXISTS t1 ;

-- [DDL]
CREATE TABLE t1 ( i int );

-- [DDL]
CREATE OR REPLACE PROCEDURE p1 () AS sql_stmt varchar2 ( 200 );

-- [DDL]
CREATE OR REPLACE PROCEDURE p2 () AS BEGIN p1 ();

-- [DDL]
CREATE OR REPLACE PROCEDURE p3 () AS BEGIN p2 ();

-- [DQL]
SELECT dbe_profiler . pl_start_profiling ( '123' );

-- [PLSQL]
CALL p3 ();

-- [DQL]
SELECT * FROM dbe_profiler . pl_profiling_functions ORDER BY run_id , funcoid ;

-- [DQL]
SELECT * FROM dbe_profiler . pl_profiling_details WHERE funcoid = 16770 ORDER BY run_id , funcoid , line # ;

-- [DQL]
SELECT * FROM dbe_profiler . pl_profiling_callgraph ORDER BY run_id , stack ;

-- [DQL]
SELECT step_name , loops_count FROM dbe_profiler . pl_profiling_trackinfo WHERE funcoid = 16770 ;

-- [DQL]
SELECT dbe_profiler . pl_clear_profiling ( '' );

-- [DQL]
SELECT step_name , loops_count FROM dbe_profiler . pl_profiling_trackinfo WHERE funcoid = 16770 ;

-- [DDL]
DROP TABLE t1 ;

-- [DDL]
CREATE TABLE t2 ( a int , b int );

-- [DDL]
CREATE OR REPLACE PROCEDURE autonomous ( a int , b int ) AS DECLARE num3 int : = a ;

-- [DDL]
CREATE OR REPLACE PROCEDURE autonomous_1 ( a int , b int ) AS DECLARE BEGIN dbe_output . print_line ( 'just no use call.' );

-- [DQL]
SELECT dbe_profiler . pl_start_profiling ( '100' );

-- [PLSQL]
CALL autonomous ( 11 , 22 );

-- [DQL]
SELECT * FROM dbe_profiler . pl_profiling_functions ORDER BY run_id , funcoid ;

-- [DQL]
SELECT * FROM dbe_profiler . pl_profiling_details ORDER BY run_id , funcoid , line # ;

-- [DQL]
SELECT * FROM dbe_profiler . pl_profiling_callgraph ORDER BY run_id , stack ;

-- [DQL]
SELECT * FROM dbe_profiler . pl_profiling_trackinfo ORDER BY run_id , funcoid ;

-- [DQL]
SELECT dbe_profiler . pl_start_profiling ( '101' );

-- [PLSQL]
CALL autonomous_1 ( 11 , 22 );

-- [DQL]
SELECT * FROM dbe_profiler . pl_profiling_functions ORDER BY run_id , funcoid ;

-- [DQL]
SELECT * FROM dbe_profiler . pl_profiling_details ORDER BY run_id , funcoid , line # ;

-- [DQL]
SELECT * FROM dbe_profiler . pl_profiling_callgraph ORDER BY run_id , stack ;

-- [DQL]
SELECT * FROM dbe_profiler . pl_profiling_trackinfo ORDER BY run_id , funcoid ;

-- [DQL]
SELECT dbe_profiler . pl_clear_profiling ( '' );

-- [DQL]
SELECT * FROM dbe_profiler . pl_profiling_functions ;

-- [DDL]
DROP TABLE t2 ;


================================================================================
-- 来源: 3189_DBE_SCHEDULER.txt
================================================================================

-- [PLSQL]
CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER . create_schedule ( 'schedule1' , NULL , 'sysdate' , NULL , 'test' );

-- [PLSQL]
CALL DBE_SCHEDULER . create_job ( job_name => 'job1' , program_name => 'program1' , schedule_name => 'schedule1' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_schedule ( 'schedule1' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

-- [PLSQL]
CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER . create_schedule ( 'schedule1' , NULL , 'sysdate' , NULL , 'test' );

-- [PLSQL]
CALL DBE_SCHEDULER . create_job ( job_name => 'job1' , program_name => 'program1' , schedule_name => 'schedule1' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_schedule ( 'schedule1' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER.create_job('job1', 'program1', '2021-07-20', 'interval ''3 minute''', '2121-07-20', 'DEFAULT_JOB_CLASS', false, false,'test', 'style', NULL, NULL);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_single_job('job1', false, false);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_program('program1', false);

-- [PLSQL]
CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER . set_attribute ( 'program1' , 'number_of_arguments' , 0 );

-- [PLSQL]
CALL DBE_SCHEDULER . set_attribute ( 'program1' , 'program_type' , 'STORED_PROCEDURE' );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

-- [DQL]
SELECT dbe_scheduler . create_job ( 'job1' , 'PLSQL_BLOCK' , 'begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER . run_job ( 'job1' , false );

-- [PLSQL]
CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

-- [DQL]
SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.run_backend_job('job1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [DDL]
create user test1 identified by '*********';

-- [DQL]
select DBE_SCHEDULER.create_credential('cre_1', 'test1', '*********');

-- [DQL]
select DBE_SCHEDULER.create_job(job_name=>'job1', job_type=>'EXTERNAL_SCRIPT', job_action=>'/usr/bin/pwd', enabled=>true, auto_drop=>false, credential_name => 'cre_1');

-- [PLSQL]
CALL DBE_SCHEDULER.run_foreground_job('job1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_credential('cre_1', false);

-- [DDL]
drop user test1;

-- [DQL]
SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.stop_job('job1', true, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [DQL]
SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.stop_single_job('job1', true);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.generate_job_name();

-- [PLSQL]
CALL DBE_SCHEDULER.generate_job_name();

-- [PLSQL]
CALL DBE_SCHEDULER.generate_job_name('job');

-- [PLSQL]
CALL DBE_SCHEDULER.generate_job_name('job');

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_program('program1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER.define_program_argument('program1', 1, 'pa1', 'type1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.define_program_argument('program1', 1, 'pa1', 'type1', 'value1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_program('program1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_program('program1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_single_program('program1', false);

-- [PLSQL]
CALL dbe_scheduler.create_job('job1','EXTERNAL_SCRIPT','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.set_job_argument_value('job1', 1, 'value1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

-- [PLSQL]
CALL DBE_SCHEDULER.drop_schedule('schedule1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_schedule('schedule2', false);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_schedule('schedule3', true);

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

-- [PLSQL]
CALL DBE_SCHEDULER.drop_schedule('schedule1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_schedule('schedule2', false);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_schedule('schedule3', true);

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

-- [PLSQL]
CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

-- [PLSQL]
CALL DBE_SCHEDULER.drop_single_schedule('schedule1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_single_schedule('schedule2', false);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_single_schedule('schedule3', true);

-- [PLSQL]
CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job_class('jc1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job_class('jc1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_single_job_class('jc1', false);

-- [DDL]
create user user1 password '1*s*****';

-- [PLSQL]
CALL DBE_SCHEDULER.grant_user_authorization('user1', 'create job');

-- [DDL]
drop user user1;

-- [DDL]
create user user1 password '1*s*****';

-- [PLSQL]
CALL DBE_SCHEDULER.grant_user_authorization('user1', 'create job');

-- [PLSQL]
CALL DBE_SCHEDULER.revoke_user_authorization('user1', 'create job');

-- [DDL]
drop user user1;

-- [PLSQL]
CALL DBE_SCHEDULER.create_credential('cre_1', 'user1', '');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_credential('cre_1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.create_credential('cre_1', 'user1', '');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_credential('cre_1', false);

-- [PLSQL]
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'stored_procedure', 'insert into tb_job_test(key) values(null);

-- [PLSQL]
CALL DBE_SCHEDULER.enable('job1');

-- [PLSQL]
CALL DBE_SCHEDULER.enable('program1', 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_program('program1', false);

-- [PLSQL]
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.enable_single('job1');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.create_program('program1', 'stored_procedure', 'insert into tb_job_test(key) values(null);

-- [PLSQL]
CALL DBE_SCHEDULER.disable('job1');

-- [PLSQL]
CALL DBE_SCHEDULER.disable('program1', false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.drop_program('program1', false);

-- [PLSQL]
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- [PLSQL]
CALL DBE_SCHEDULER.disable_single('job1', false);

-- [PLSQL]
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- [PLSQL]
CALL DBE_SCHEDULER.eval_calendar_string('FREQ=DAILY;

-- [DDL]
CREATE OR REPLACE PROCEDURE pr1(calendar_str text) as DECLARE start_date timestamp with time zone;

-- [PLSQL]
CALL pr1('FREQ=hourly;


================================================================================
-- 来源: 3192_DBE_STATS.txt
================================================================================

-- [DDL]
CREATE SCHEMA dbe_stats_lock;

-- [SESSION]
SET CURRENT_SCHEMA=dbe_stats_lock;

-- [DDL]
CREATE TABLE t1(a int,b int);

-- 锁定表，查看其锁定状态
-- [PLSQL]
CALL DBE_STATS.LOCK_TABLE_STATS(ownname=>'dbe_stats_lock',tabname=>'t1');

-- [DQL]
SELECT relname,instr(reloptions::text,'stat_state=locked',1,1) <> 0 as exist_lock FROM PG_CLASS WHERE relname='t1' and relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_lock');

-- 锁定后analyze, 发生报错
-- [MAINTENANCE]
ANALYZE t1;

-- 删除表、删除命名空间
-- [DDL]
DROP TABLE t1;

-- [DDL]
DROP SCHEMA dbe_stats_lock;

-- [DDL]
CREATE SCHEMA dbe_stats_lock;

-- [SESSION]
SET CURRENT_SCHEMA=dbe_stats_lock;

-- [DDL]
CREATE TABLE upart_table(a int, b int, c int) PARTITION BY RANGE(a) ( PARTITION p1 VALUES LESS THAN(1200), PARTITION p2 VALUES LESS THAN(2400), PARTITION p3 VALUES LESS THAN(MAXVALUE) );

-- 锁定一个分区，其他分区及表不受影响
-- [PLSQL]
CALL DBE_STATS.LOCK_PARTITION_STATS(ownname=>'dbe_stats_lock',tabname=>'upart_table',partname=>'p1');

-- [DQL]
SELECT relname,instr(reloptions::text,'stat_state=locked',1,1) <> 0 as exist_lock FROM PG_CLASS WHERE relname='upart_table';

-- [DQL]
SELECT relname,instr(reloptions::text,'stat_state=locked',1,1) <> 0 as exist_lock FROM PG_PARTITION WHERE parentid='upart_table'::REGCLASS;

-- 删除表、命名空间
-- [DDL]
DROP TABLE upart_table;

-- [DDL]
DROP SCHEMA dbe_stats_lock;

-- [DDL]
CREATE SCHEMA dbe_stats_lock;

-- [SESSION]
SET CURRENT_SCHEMA=dbe_stats_lock;

-- [DDL]
CREATE TABLE t1(a int,b int);

-- [DML_INSERT]
INSERT INTO t1 VALUES(generate_series(1,100),1);

-- [MAINTENANCE]
ANALYZE t1;

-- 锁定列后，查看列的锁定状态
-- [PLSQL]
CALL DBE_STATS.LOCK_COLUMN_STATS(ownname=>'dbe_stats_lock',tabname=>'t1',colname=>'a');

-- [DQL]
SELECT staattnum,stastate FROM PG_STATISTIC WHERE starelid='t1'::REGCLASS;

-- 删除表、命名空间
-- [DDL]
DROP TABLE t1;

-- [DDL]
DROP SCHEMA dbe_stats_lock;

-- [PLSQL]
CALL DBE_STATS.LOCK_SCHEMA_STATS(ownname=>'dbe_stats_lock');

-- [PLSQL]
CALL DBE_STATS.UNLOCK_TABLE_STATS(ownname=>'dbe_stats_lock',tabname=>'t1');

-- [PLSQL]
CALL DBE_STATS.UNLOCK_PARTITION_STATS(ownname=>'dbe_stats_lock',tabname=>'upart_table',partname=>'p1');

-- [PLSQL]
CALL DBE_STATS.UNLOCK_COLUMN_STATS(ownname=>'dbe_stats_lock',tabname=>'t1',colname=>'a');

-- [PLSQL]
CALL DBE_STATS.UNLOCK_SCHEMA_STATS(ownname=>'dbe_stats_lock');

-- [DDL]
CREATE SCHEMA dbe_stats_restore;

-- [SESSION]
SET CURRENT_SCHEMA=dbe_stats_restore;

-- [DDL]
CREATE TABLE t1(a int, b int);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,1);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,1);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,1);

-- [MAINTENANCE]
ANALYZE t1;

-- [DML_INSERT]
INSERT INTO t1 VALUES(2,2);

-- [DML_INSERT]
INSERT INTO t1 VALUES(2,2);

-- [DML_INSERT]
INSERT INTO t1 VALUES(2,2);

-- [MAINTENANCE]
ANALYZE t1;

-- 查看历史表
-- [DQL]
SELECT relname,reltuples FROM GS_TABLESTATS_HISTORY WHERE relname='t1';

-- 查看当前系统表中的统计信息
-- [DQL]
SELECT relname,reltuples FROM PG_CLASS WHERE relname='t1' AND relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_restore');

-- 回退到最早的统计信息，查看系统表
-- [PLSQL]
CALL DBE_STATS.RESTORE_TABLE_STATS(ownname=>'dbe_stats_restore',tabname=>'t1',as_of_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

-- [DQL]
SELECT relname,reltuples FROM PG_CLASS WHERE relname='t1' AND relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_restore');

-- 删除表、删除命名空间
-- [DDL]
DROP TABLE t1;

-- [DDL]
DROP SCHEMA dbe_stats_restore;

-- [PLSQL]
CALL DBE_STATS.RESTORE_PARTITION_STATS(ownname=>'dbe_stats_restore',tabname=>'t1',partname=>'p1',as_of_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

-- [DDL]
CREATE SCHEMA dbe_stats_restore;

-- [SESSION]
SET CURRENT_SCHEMA=dbe_stats_restore;

-- [DDL]
CREATE TABLE t1(a int, b int);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,1);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,1);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,1);

-- [MAINTENANCE]
ANALYZE t1;

-- [DML_INSERT]
INSERT INTO t1 VALUES(2,2);

-- [DML_INSERT]
INSERT INTO t1 VALUES(2,2);

-- [DML_INSERT]
INSERT INTO t1 VALUES(2,2);

-- [MAINTENANCE]
ANALYZE t1;

-- 查看历史表里的统计信息
-- [DQL]
SELECT staattnum,stadistinct,stakind1,stanumbers1,stavalues1 FROM GS_STATISTIC_HISTORY WHERE starelid='t1'::REGCLASS ORDER BY statimestamp;

-- 查询当前系统表中的统计信息
-- [DQL]
SELECT staattnum,stadistinct,stakind1,stanumbers1,stavalues1 FROM PG_STATISTIC WHERE starelid='t1'::REGCLASS;

-- 回退到时间较早的时间节点，查询系统表中的统计信息
-- [PLSQL]
CALL DBE_STATS.RESTORE_COLUMN_STATS(ownname=>'dbe_stats_restore',tabname=>'t1',colname=>'a',as_of_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

-- [DQL]
SELECT staattnum,stadistinct,stakind1,stanumbers1,stavalues1 FROM PG_STATISTIC WHERE starelid='t1'::REGCLASS;

-- 删除表、命名空间
-- [DDL]
DROP TABLE t1;

-- [DDL]
DROP SCHEMA dbe_stats_restore;

-- [DDL]
CREATE SCHEMA dbe_stats_restore;

-- [SESSION]
SET CURRENT_SCHEMA=dbe_stats_restore;

-- [DDL]
CREATE TABLE t1(a int, b int);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,1);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,1);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,1);

-- [MAINTENANCE]
ANALYZE t1;

-- [DML_INSERT]
INSERT INTO t1 VALUES(2,2);

-- [DML_INSERT]
INSERT INTO t1 VALUES(2,2);

-- [DML_INSERT]
INSERT INTO t1 VALUES(2,2);

-- [MAINTENANCE]
ANALYZE t1;

-- [DQL]
SELECT relname,reltuples FROM GS_TABLESTATS_HISTORY WHERE relname='t1';

-- [DQL]
SELECT relname,reltuples FROM PG_CLASS WHERE relname='t1' AND relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_restore');

-- [PLSQL]
CALL DBE_STATS.RESTORE_SCHEMA_STATS(ownname=>'dbe_stats_restore',as_of_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

-- [DQL]
SELECT relname,reltuples FROM PG_CLASS WHERE relname='t1' AND relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_restore');

-- 删除表、命名空间
-- [DDL]
DROP TABLE t1;

-- [DDL]
DROP SCHEMA dbe_stats_restore;

-- [DDL]
CREATE SCHEMA dbe_stats_purge;

-- [SESSION]
SET CURRENT_SCHEMA=dbe_stats_purge;

-- [DDL]
CREATE TABLE t1(a int, b int);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,1);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,1);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,1);

-- [MAINTENANCE]
ANALYZE t1;

-- [DML_INSERT]
INSERT INTO t1 VALUES(2,2);

-- [DML_INSERT]
INSERT INTO t1 VALUES(2,2);

-- [DML_INSERT]
INSERT INTO t1 VALUES(2,2);

-- [MAINTENANCE]
ANALYZE t1;

-- 查看历史表
-- [DQL]
SELECT relname,reltuples FROM GS_TABLESTATS_HISTORY WHERE relname='t1';

-- 清除时间较早的历史统计信息，查看历史表
-- [PLSQL]
CALL DBE_STATS.PURGE_STATS(before_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

-- [DQL]
SELECT relname,reltuples FROM GS_TABLESTATS_HISTORY WHERE relname='t1';

-- 删除表、命名空间
-- [DDL]
DROP TABLE t1;

-- [DDL]
DROP SCHEMA dbe_stats_purge;

-- [PLSQL]
CALL DBE_STATS.GET_STATS_HISTORY_RETENTION();

-- [MAINTENANCE]
ANALYZE;

-- [PLSQL]
CALL DBE_STATS.GET_STATS_HISTORY_AVAILABILITY();


================================================================================
-- 来源: 3193_DBE_TASK.txt
================================================================================

-- [DQL]
SELECT DBE_TASK . SUBMIT ( 'call pro_xxx();

-- [DQL]
SELECT DBE_TASK . SUBMIT ( 'call pro_xxx();

-- [PLSQL]
DECLARE gaussdb -# jobid int ;

-- [PLSQL]
DECLARE gaussdb -# id integer ;

-- [PLSQL]
CALL dbe_task . id_submit ( 101 , 'insert_msg_statistic1;

-- [PLSQL]
CALL dbe_task.cancel(101);

-- [TCL]
BEGIN gaussdb $ # DBE_TASK . ID_SUBMIT ( 12345 , 'insert_msg_statistic1;

-- [PLSQL]
CALL dbe_task . id_submit ( 101 , 'insert_msg_statistic1;

-- [PLSQL]
CALL dbe_task . finish ( 101 , true );

-- [PLSQL]
CALL dbe_task . update ( 101 , 'call userproc();

-- [PLSQL]
CALL dbe_task . update ( 101 , 'insert into tbl_a values(sysdate);

-- [TCL]
BEGIN gaussdb $ # DBE_TASK . CHANGE ( gaussdb $ # job => 101 , gaussdb $ # what => 'insert into t2 values (2);

-- [PLSQL]
CALL dbe_task . content ( 101 , 'call userproc();

-- [PLSQL]
CALL dbe_task . content ( 101 , 'insert into tbl_a values(sysdate);

-- [PLSQL]
CALL dbe_task . next_time ( 101 , sysdate );

-- [PLSQL]
CALL dbe_task . interval ( 101 , 'sysdate + 1.0/1440' );

-- [PLSQL]
CALL dbe_task . cancel ( 101 );


================================================================================
-- 来源: 3198_Retry.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE retry_basic ( IN x INT ) AS BEGIN INSERT INTO t1 ( a ) VALUES ( x );

-- [PLSQL]
CALL retry_basic ( 1 );


================================================================================
-- 来源: 3202_file_3202.txt
================================================================================

-- [DDL]
CREATE TABLE test_trigger_des_tbl(id1 int, id2 int, id3 int);

-- [DDL]
CREATE OR REPLACE FUNCTION tri_insert_func() RETURNS TRIGGER AS $$ DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
DROP TABLE test_trigger_des_tbl;

-- [DDL]
CREATE TABLE t1(a INT ,b TEXT);

-- [PLSQL]
DECLARE

-- [DQL]
SELECT * FROM t1;

-- [DDL]
DROP TABLE t1;

-- [DDL]
CREATE TABLE sections(section_id INT);

-- [DML_INSERT]
INSERT INTO sections VALUES(1);

-- [DML_INSERT]
INSERT INTO sections VALUES(1);

-- [DML_INSERT]
INSERT INTO sections VALUES(1);

-- [DML_INSERT]
INSERT INTO sections VALUES(1);

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_sys_ref(OUT c1 REFCURSOR) IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_sys_call() AS DECLARE c1 SYS_REFCURSOR;

-- [PLSQL]
CALL proc_sys_call();

-- [DDL]
DROP PROCEDURE proc_sys_ref;

-- [DDL]
DROP PROCEDURE proc_sys_call;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_sys_ref(IN c1 REFCURSOR) IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_sys_call() AS DECLARE c1 SYS_REFCURSOR;

-- [PLSQL]
CALL proc_sys_call();

-- [DDL]
DROP PROCEDURE proc_sys_ref;

-- [DDL]
DROP PROCEDURE IF EXISTS proc_sys_ref;

-- [DDL]
CREATE OR REPLACE FUNCTION proc_sys_ref() RETURN SYS_REFCURSOR IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
DROP FUNCTION IF EXISTS proc_sys_ref;

-- [DDL]
CREATE OR REPLACE FUNCTION proc_sys_ref(C1 out SYS_REFCURSOR) return SYS_REFCURSOR IS gaussdb$# declare gaussdb$# PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
CREATE OR REPLACE PROCEDURE proc_sys_ref(OUT c1 REFCURSOR) IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [TCL]
begin;

-- [PLSQL]
call proc_sys_ref(null);

-- [CURSOR]
fetch "<unnamed portal 1>";

-- [DDL]
DROP PROCEDURE proc_sys_ref;

-- [DDL]
DROP TABLE sections;

-- [DDL]
CREATE TABLE test_in (id INT,a DATE);

-- [DDL]
CREATE OR REPLACE FUNCTION autonomous_out() RETURNS RECORD LANGUAGE PLPGSQL AS $$ DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DQL]
SELECT ok.id,ok.a FROM autonomous_out() AS ok(id INT,a DATE);

-- [DDL]
CREATE TYPE rec IS (e1 INTEGER, e2 VARCHAR2);

-- [DDL]
CREATE OR REPLACE FUNCTION func(ele3 INOUT VARCHAR2) RETURN rec AS i INTEGER;

-- [PLSQL]
CALL func(1);

-- [DDL]
DROP TABLE test_in;

-- [DDL]
DROP FUNCTION autonomous_out;

-- [DDL]
DROP FUNCTION func;

-- [DDL]
CREATE OR REPLACE PROCEDURE auto_func(r INT) AS DECLARE a INT;

-- [PLSQL]
call auto_func(1);

-- [DDL]
DROP FUNCTION auto_func;

-- [DDL]
CREATE TABLE test_in (id INT,a DATE);

-- [DDL]
CREATE TABLE test_main (id INT,a DATE);

-- [DML_INSERT]
INSERT INTO test_main VALUES (1111,'2021-01-01'),(2222,'2021-02-02');

-- [DML_TRUNCATE]
TRUNCATE test_in,test_main;

-- [DDL]
CREATE OR REPLACE FUNCTION autonomous_f_022(num1 INT) RETURNS SETOF test_in LANGUAGE PLPGSQL AS $$ DECLARE count INT :=3;

-- [DDL]
DROP TABLE test_main;

-- [DDL]
DROP TABLE test_in;


================================================================================
-- 来源: 3203_file_3203.txt
================================================================================

-- [DDL]
CREATE TABLE t2(a INT, b INT);

-- [DML_INSERT]
INSERT INTO t2 VALUES(1,2);

-- [DQL]
SELECT * FROM t2;

-- [DDL]
CREATE OR REPLACE PROCEDURE autonomous_4(a INT, b INT) AS DECLARE num3 INT := a;

-- [DDL]
CREATE OR REPLACE PROCEDURE autonomous_5(a INT, b INT) AS DECLARE BEGIN DBE_OUTPUT.PRINT_LINE('JUST NO USE CALL.');

-- [DQL]
SELECT autonomous_5(11,22);

-- [DQL]
SELECT * FROM t2 ORDER BY a;

-- [DDL]
DROP TABLE t2;

-- [DDL]
DROP PROCEDURE autonomous_4;

-- [DDL]
DROP PROCEDURE autonomous_5;


================================================================================
-- 来源: 3204_file_3204.txt
================================================================================

-- [DDL]
CREATE TABLE t1(a INT ,B TEXT);

-- [TCL]
START TRANSACTION;

-- [PLSQL]
DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DML_INSERT]
INSERT INTO t1 VALUES(1,'YOU WILL ROLLBACK!');

-- [TCL]
ROLLBACK;

-- [DQL]
SELECT * FROM t1;

-- [DDL]
DROP TABLE t1;


================================================================================
-- 来源: 3205_file_3205.txt
================================================================================

-- [DDL]
CREATE TABLE t4(a INT, b INT, c TEXT);

-- [DDL]
CREATE OR REPLACE FUNCTION autonomous_32(a INT ,b INT ,c TEXT) RETURN INT AS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- [DDL]
CREATE OR REPLACE FUNCTION autonomous_33(num1 INT) RETURN INT AS DECLARE num3 INT := 220;

-- [DQL]
SELECT autonomous_33(0);

-- [DQL]
SELECT * FROM t4;

-- [DDL]
DROP TABLE t4;

-- [DDL]
DROP FUNCTION autonomous_32;

-- [DDL]
DROP FUNCTION autonomous_32;


================================================================================
-- 来源: 3206_Package.txt
================================================================================

-- [DDL]
CREATE TABLE t2(a INT, b INT);

-- [DML_INSERT]
INSERT INTO t2 VALUES(1,2);

-- [DQL]
SELECT * FROM t2;

-- [DDL]
CREATE OR REPLACE PACKAGE autonomous_pkg AS PROCEDURE autonomous_4(a INT, b INT);

-- [DDL]
CREATE OR REPLACE PACKAGE BODY autonomous_pkg AS PROCEDURE autonomous_4(a INT, b INT) AS DECLARE num3 INT := a;

-- [DDL]
CREATE OR REPLACE PROCEDURE autonomous_5(a INT, b INT) AS DECLARE va INT;

-- [DQL]
SELECT autonomous_5(11,22);

-- [DQL]
SELECT * FROM t2 ORDER BY a;

-- [DDL]
DROP TABLE t2;


================================================================================
-- 来源: 3626_PG_REPLICATION_SLOTS.txt
================================================================================

-- [DQL]
SELECT * FROM pg_replication_slots;


================================================================================
-- 来源: 3806_SESSION_STAT_ACTIVITY.txt
================================================================================

-- [DQL]
SELECT datname, usename, usesysid,state,pid FROM pg_stat_activity;


================================================================================
-- 来源: 3807_GLOBAL_SESSION_STAT_ACTIVITY.txt
================================================================================

-- [DQL]
SELECT datname, usename, usesysid,state,pid FROM pg_stat_activity;


================================================================================
-- 来源: 3929_DBE_PLDEBUGGER Schema.txt
================================================================================

-- [DDL]
CREATE OR REPLACE PROCEDURE test_debug ( IN x INT ) AS BEGIN INSERT INTO t1 ( a ) VALUES ( x );

-- [DQL]
SELECT OID FROM PG_PROC WHERE PRONAME = 'test_debug' ;

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . turn_on ( 16389 );

-- [PLSQL]
call test_debug ( 1 );

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . attach ( 'datanode' , 0 );

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . next ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . info_locals ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . set_var ( 'x' , 2 );

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . print_var ( 'x' );

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . continue ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . continue ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . error_end ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . abort ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . info_code ( 16389 );

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . add_breakpoint ( 16389 , 4 );

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . info_breakpoints ();

-- [DQL]
SELECT * FROM DBE_PLDEBUGGER . continue ();


================================================================================
-- 来源: 3977_file_3977.txt
================================================================================

-- [SESSION]
SHOW server_version ;

-- [SESSION]
SHOW ALL ;

-- [DQL]
SELECT * FROM pg_settings WHERE NAME = 'server_version' ;

-- [DQL]
SELECT * FROM pg_settings ;

-- [SESSION]
SHOW client_encoding ;


================================================================================
-- 来源: 3978_file_3978.txt
================================================================================

-- [DDL]
ALTER DATABASE dbname SET paraname TO value ;

-- [DDL]
ALTER USER username SET paraname TO value ;

-- [SESSION]
SET paraname TO value ;

-- [SESSION]
SHOW hot_standby ;

-- [SESSION]
SHOW authentication_timeout ;

-- [SESSION]
SHOW explain_perf_mode ;

-- [DDL]
ALTER DATABASE postgres SET explain_perf_mode TO pretty ;

-- [DDL]
ALTER USER omm SET explain_perf_mode TO pretty ;

-- [SESSION]
SET explain_perf_mode TO pretty ;

-- [SESSION]
SHOW explain_perf_mode ;

-- [SESSION]
SHOW max_connections ;

-- [OTHER]
\ q 修改 GaussDB 数据库主节点的最大连接数。 gs_guc set -Z datanode -N all -I all -c "max_connections = 800" 重启 数据库 。 gs_om -t stop && gs_om -t start 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看最大连接数。 1 2 3 4

-- [SESSION]
SHOW max_connections ;

-- [SESSION]
SHOW authentication_timeout ;

-- [OTHER]
\ q 修改数据库主节点的客户端认证最长时间。 gs_guc reload -Z datanode -N all -I all -c "authentication_timeout = 59s" 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看客户端认证的最长时间。 1 2 3 4

-- [SESSION]
SHOW authentication_timeout ;

-- [SESSION]
SHOW max_connections ;

-- [OTHER]
\ q 修改 GaussDB 数据库节点的最大连接数。 gs_guc set -Z datanode -N all -I all -c "max_connections = 500" 重启 数据库 。 gs_om -t stop gs_om -t start 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看最大连接数。 1 2 3 4

-- [SESSION]
SHOW max_connections ;

-- [SESSION]
SHOW authentication_timeout ;

-- [OTHER]
\ q 修改 GaussDB 数据库节点的客户端认证最长时间。 gs_guc reload -Z datanode -N all -I all -c "authentication_timeout = 30s" 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看客户端认证的最长时间。 1 2 3 4

-- [SESSION]
SHOW authentication_timeout ;


================================================================================
-- 来源: 4012_file_4012.txt
================================================================================

-- [SESSION]
show logging_module ;

-- [SESSION]
set logging_module = 'on(SSL)' ;

-- [SESSION]
show logging_module ;

-- [SESSION]
set logging_module = 'off(ALL)' ;

-- [SESSION]
show logging_module ;

-- [SESSION]
set logging_module = 'on(ALL)' ;

-- [SESSION]
show logging_module ;


================================================================================
-- 来源: 4027_file_4027.txt
================================================================================

-- [DQL]
select 0 . 1231243 as a , 0 . 1231243 :: numeric as b , 0 . 1231243 :: integer ( 10 , 3 ) as c , length ( 0 . 1242343 ) as d ;

-- [DQL]
select 0 . 1231243 as a , 0 . 1231243 :: numeric as b , 0 . 1231243 :: integer ( 10 , 3 ) as c , length ( 0 . 1242343 ) as d ;

-- [DQL]
select add_months ( '2018-02-28' , 3 ) from sys_dummy ;

-- [DQL]
select add_months ( '2018-02-28' , 3 ) from sys_dummy ;

-- [DQL]
select '' AS to_number_14 , to_number ( '34,50' , '999,99' );

-- [DQL]
select '' AS to_number_14 , to_number ( '34,50' , '999,99' );

-- [DQL]
select ( - 2147483648 ):: int4 / ( - 1 ):: int4 ;

-- [DQL]
select ( - 2147483648 ):: int4 / ( - 1 ):: int4 ;

-- [DDL]
create table test1 ( c1 int , c2 varchar );

-- [DML_INSERT]
insert into test1 values ( 2 , '1.1' );

-- [SESSION]
set behavior_compat_options = '' ;

-- [DQL]
select * from test1 where c2 > 1 ;

-- [SESSION]
set behavior_compat_options = 'convert_string_digit_to_numeric' ;

-- [DQL]
select * from test1 where c2 > 1 ;

-- [DQL]
select length ( lpad ( '123' , 0 , '*' )) from sys_dummy ;

-- [DQL]
select length ( lpad ( '123' , 0 , '*' )) from sys_dummy ;

-- [DQL]
select concat ( variadic NULL :: int []) is NULL ;

-- [DQL]
select concat ( variadic NULL :: int []) is NULL ;

-- [DQL]
select concat ( variadic NULL :: int []) is NULL ;

-- [SESSION]
set behavior_compat_options='hide_tailing_zero';

-- [DQL]
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- [SESSION]
set behavior_compat_options='';

-- [DQL]
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- [SESSION]
set behavior_compat_options='';

-- [DDL]
create table tb_test(c1 int,c2 varchar2,c3 varchar2);

-- [DML_INSERT]
insert into tb_test values(1,'a','b');

-- [DDL]
create or replace view v_test as select rownum from tb_test;

-- [SESSION]
set behavior_compat_options = 'rownum_type_compat';

-- [DDL]
create or replace view v_test1 as select rownum from tb_test;

-- [SESSION]
set behavior_compat_options='aformat_null_test';

-- [DQL]
select r, r is null as isnull, r is not null as isnotnull from (values (1,row(1,2)), (1,row(null,null)), (1,null), (null,row(1,2)), (null,row(null,null)), (null,null) ) r(a,b);

-- [SESSION]
set behavior_compat_options='';

-- [DQL]
select r, r is null as isnull, r is not null as isnotnull from (values (1,row(1,2)), (1,row(null,null)), (1,null), (null,row(1,2)), (null,row(null,null)), (null,null) ) r(a,b);

-- [SESSION]
set behavior_compat_options='';

-- [DDL]
create table tab_1(col1 varchar(3));

-- [DDL]
create table tab_2(col2 char(3));

-- [DML_INSERT]
insert into tab_2 values(' ');

-- [DML_INSERT]
insert into tab_1 select col2 from tab_2;

-- [DQL]
select * from tab_1 where col1 is null;

-- [DQL]
select * from tab_1 where col1=' ';

-- [DML_DELETE]
delete from tab_1;

-- [SESSION]
set behavior_compat_options = 'char_coerce_compat';

-- [DML_INSERT]
insert into tab_1 select col2 from tab_2;

-- [DQL]
select * from tab_1 where col1 is null;

-- [DQL]
select * from tab_1 where col1=' ';

-- [SESSION]
set behavior_compat_options='truncate_numeric_tail_zero';

-- [DQL]
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- [SESSION]
set behavior_compat_options='';

-- [DQL]
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- [DDL]
create or replace function test(f1 int, f2 int default 20, f3 int, f4 int default 40, f5 int default 50) return int gaussdb -# as gaussdb $# begin gaussdb $# raise info 'f1:%',f1;

-- [DQL]
select test(1,2);

-- [DDL]
create or replace function test(f1 int, f2 int default 20, f3 int, f4 int default 40, f5 int default 50) return int gaussdb -# as gaussdb $# begin gaussdb $# raise info 'f1:%',f1;

-- [DQL]
select test(1,2);

-- [DQL]
select power(2,3);

-- [DQL]
select count(*) from db_ind_columns;

-- [DQL]
select count(index_name) from db_ind_columns;

-- [DQL]
SELECT left('abcde', 2);

-- [DQL]
SELECT pg_client_encoding();

-- [SESSION]
set behavior_compat_options = 'enable_funcname_with_argsname';

-- [DQL]
select power(2,3);

-- [DQL]
select count(*) from db_ind_columns;

-- [DQL]
select count(index_name) from db_ind_columns;

-- [DQL]
SELECT left('abcde', 2);

-- [DQL]
SELECT pg_client_encoding();

-- [SESSION]
SET behavior_compat_options='proc_outparam_override,proc_outparam_transfer_length';

-- [DDL]
CREATE OR REPLACE PROCEDURE out_param_test1(m in int, v inout varchar2,v1 inout varchar2) is gaussdb$# begin gaussdb$# v := 'aaaddd';

-- [DDL]
CREATE OR REPLACE PROCEDURE call_out_param_test1 is gaussdb$# v varchar2(5) := 'aabbb';

-- [PLSQL]
CALL call_out_param_test1();

-- [DDL]
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of varchar(5);

-- [PLSQL]
CALL p1();

-- [SESSION]
SET behavior_compat_options = 'tableof_elem_constraints';

-- [DDL]
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of varchar(5);

-- [PLSQL]
CALL p1();

-- [DDL]
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of int index by varchar(5);

-- [PLSQL]
CALL p1();

-- [SESSION]
SET behavior_compat_options = 'tableof_elem_constraints';

-- [DDL]
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of int index by varchar(5);

-- [PLSQL]
CALL p1();

-- [SESSION]
set behavior_compat_options='current_sysdate';

-- [DQL]
select sysdate;

-- [DDL]
create or replace function proc_test return varchar2 as gaussdb$# begin gaussdb$# return '1';

-- [DDL]
create or replace procedure proc_test as gaussdb$# begin gaussdb$# null;

-- 设置参数后允许替换类型
-- [SESSION]
set behavior_compat_options='allow_function_procedure_replace';

-- [DDL]
create or replace procedure proc_test as gaussdb$# begin gaussdb$# null;

-- [DDL]
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- [PLSQL]
call p1();

-- [DDL]
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- [PLSQL]
call p1();

-- [DDL]
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- [PLSQL]
call p1();

-- [SESSION]
set behavior_compat_options = 'collection_exception_backcompat';

-- [DDL]
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- [PLSQL]
call p1();

-- [DDL]
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- [PLSQL]
call p1();

-- [DDL]
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- [PLSQL]
call p1();

-- [SESSION]
set behavior_compat_options='enable_case_when_alias';

-- [DDL]
create table test(c1 varchar2);

-- [DML_INSERT]
insert into test values('x');

-- [DQL]
select decode(c1,'x','0','default') from test;

-- [DQL]
select (case c1 when 'x' then '0' else 'default' end) from test;

-- [DDL]
create user plsql_rollback1 PASSWORD '********';

-- [DDL]
create user plsql_rollback2 PASSWORD '********';

-- [DCL_GRANT]
grant plsql_rollback1 to plsql_rollback2;

-- [DDL]
create or replace procedure plsql_rollback1.p1 () authid definer as gaussdb$# va int;

-- [SESSION]
set session AUTHORIZATION plsql_rollback2 PASSWORD '********';

-- [DDL]
CREATE SCHEMA sch1;

-- [DDL]
CREATE PACKAGE pck1 IS PROCEDURE sch1.pck1();

-- [DQL]
select timestamp '2024-03-20 01:30:00’ at time zone 'Europe/Moscow' from dual;

-- [SESSION]
set behavior_compat_options='enable_use_ora_timestamptz';

-- [DQL]
select timestamp '2024-03-20 01:30:00’ at time zone 'Europe/Moscow' from dual;

-- [SESSION]
set gs_format_behavior_compat_options='allow_textconcat_null';

-- [DQL]
select 'a' || null || 'b';


================================================================================
-- 来源: 4152_key.txt
================================================================================

-- [DQL]
select * from gs_stat_get_hotkeys_info () order by count , hash_value ;

-- [DQL]
select * from global_stat_get_hotkeys_info () order by count , hash_value ;


================================================================================
-- 来源: 4155_file_4155.txt
================================================================================

-- [SESSION]
show product_version ;

-- [SESSION]
show hotpatch_version ;


================================================================================
-- 来源: 4233_file_4233.txt
================================================================================

-- [SESSION]
show product_version ;

-- [SESSION]
show hotpatch_version ;


================================================================================
-- 来源: 4273_file_4273.txt
================================================================================

-- 创建全量物化视图
-- [DDL]
CREATE MATERIALIZED VIEW mv AS select count(*) from t1;

-- 查询物化视图结果
-- [DQL]
SELECT * FROM mv;

-- 再次向物化视图中基表插入数据
-- [DML_INSERT]
INSERT INTO t1 VALUES(3, 3);

-- 对全量物化视图做全量刷新
-- [OTHER]
REFRESH MATERIALIZED VIEW mv;

-- 查询物化视图结果
-- [DQL]
SELECT * FROM mv;

-- 删除物化视图，删除表
-- [DDL]
DROP MATERIALIZED VIEW mv;

-- [DDL]
DROP TABLE t1;


================================================================================
-- 来源: 4277_file_4277.txt
================================================================================

-- 创建增量物化视图
-- [DDL]
CREATE INCREMENTAL MATERIALIZED VIEW mv AS SELECT * FROM t1;

-- 插入数据
-- [DML_INSERT]
INSERT INTO t1 VALUES(3, 3);

-- 增量刷新物化视图
-- [OTHER]
REFRESH INCREMENTAL MATERIALIZED VIEW mv;

-- 查询物化视图结果
-- [DQL]
SELECT * FROM mv;

-- 插入数据
-- [DML_INSERT]
INSERT INTO t1 VALUES(4, 4);

-- 全量刷新物化视图
-- [OTHER]
REFRESH MATERIALIZED VIEW mv;

-- 查询物化视图结果
-- [DQL]
select * from mv;

-- 删除物化视图，删除表
-- [DDL]
DROP MATERIALIZED VIEW mv;

-- [DDL]
DROP TABLE t1;


================================================================================
-- 来源: 4280_gsql.txt
================================================================================

-- [OTHER]
CREATE CLIENT MASTER KEY cmk1 WITH ( KEY_STORE = hcs_kms , KEY_PATH = '{KMS服务器地址}/{密钥ID}', ALGORITHM = AES_256);

-- [OTHER]
CREATE COLUMN ENCRYPTION KEY cek1 WITH VALUES (CLIENT_MASTER_KEY = cmk1, ALGORITHM = AES_256_GCM);

-- [DDL]
CREATE TABLE creditcard_info ( id_number int, name text encrypted with (column_encryption_key = cek1, encryption_type = DETERMINISTIC), credit_card varchar(19) encrypted with (column_encryption_key = cek1, encryption_type = DETERMINISTIC));

-- [DML_INSERT]
INSERT INTO creditcard_info VALUES (1,'joe','6217986500001288393');

-- [DML_INSERT]
INSERT INTO creditcard_info VALUES (2, 'joy','6219985678349800033');

-- 从加密表中查询数据
-- [DQL]
select * from creditcard_info where name = 'joe';

-- 更新加密表中数据
-- [DML_UPDATE]
update creditcard_info set credit_card = '80000000011111111' where name = 'joy';

-- 向表中新增一列加密列
-- [DDL]
ALTER TABLE creditcard_info ADD COLUMN age int ENCRYPTED WITH (COLUMN_ENCRYPTION_KEY = cek1, ENCRYPTION_TYPE = DETERMINISTIC);

-- 从表中删除一列加密列
-- [DDL]
ALTER TABLE creditcard_info DROP COLUMN age;

-- 从系统表中查询主密钥信息
-- [DQL]
SELECT * FROM gs_client_global_keys;

-- 从系统表中查询列密钥信息
-- [DQL]
SELECT column_key_name,column_key_distributed_id ,global_key_id,key_owner FROM gs_column_keys;

-- [DDL]
DROP TABLE creditcard_info;

-- 删除列密钥
-- [OTHER]
DROP COLUMN ENCRYPTION KEY cek1;

-- 删除主密钥
-- [OTHER]
DROP CLIENT MASTER KEY cmk1;


================================================================================
-- 来源: 4283__.txt
================================================================================

-- [DDL]
CREATE TABLE creditcard_info ( id_number int , name text , credit_card varchar ( 19 ) encrypted with ( column_encryption_key = cek1 , encryption_type = DETERMINISTIC ) ) with ( orientation = row ) distribute by hash ( id_number );

-- [DML_INSERT]
insert into creditcard_info values ( 1 , 'Avi' , '1234567890123456' );

-- [DML_INSERT]
insert into creditcard_info values ( 2 , 'Eli' , '2345678901234567' );

-- [DDL]
CREATE FUNCTION f_encrypt_in_sql ( val1 text , val2 varchar ( 19 )) RETURNS text AS 'SELECT name from creditcard_info where name=$1 or credit_card=$2 LIMIT 1' LANGUAGE SQL ;

-- [DDL]
CREATE FUNCTION f_encrypt_in_plpgsql ( val1 text , val2 varchar ( 19 ), OUT c text ) AS $$ BEGIN SELECT into c name from creditcard_info where name = $ 1 or credit_card = $ 2 LIMIT 1 ;

-- [DQL]
SELECT f_encrypt_in_sql ( 'Avi' , '1234567890123456' );

-- [DQL]
SELECT f_encrypt_in_plpgsql ( 'Avi' , val2 => '1234567890123456' );


================================================================================
-- 来源: 4284_file_4284.txt
================================================================================

-- [SESSION]
SHOW enable_tde;

-- [SESSION]
show tde_key_info;

-- [DDL]
CREATE TABLE t1 (c1 INT, c2 TEXT) WITH (enable_tde = on);

-- [DDL]
CREATE TABLE t2 (c1 INT, c2 TEXT) WITH (enable_tde = on, encrypt_algo = 'SM4_CTR');

-- [DQL]
SELECT relname,reloptions FROM pg_class WHERE relname = 't1';

-- [DML_INSERT]
INSERT INTO t1 VALUES (1, 'tde plain 123');

-- [DQL]
SELECT * FROM t1;

-- [DDL]
ALTER TABLE t1 ENCRYPTION KEY ROTATION;

-- [DDL]
ALTER TABLE t1 SET (enable_tde = off);

-- [MAINTENANCE]
VACUUM FULL t1;

-- [DDL]
ALTER TABLE t1 SET (enable_tde = on);

-- [MAINTENANCE]
VACUUM FULL t1;

-- [DDL]
DROP TABLE t1;


================================================================================
-- 来源: 4286_file_4286.txt
================================================================================

-- [DDL]
CREATE SECURITY LABEL label1 'L1:G2,G4' ;

-- [DDL]
CREATE SECURITY LABEL label2 'L2:G2-G4' ;

-- [DDL]
CREATE SECURITY LABEL label3 'L3:G1-G5' ;

-- [DQL]
SELECT * FROM gs_security_label ;

-- [DQL]
SELECT * FROM gs_security_label;

-- [DDL]
DROP SECURITY LABEL label1 ;

-- [DDL]
DROP SECURITY LABEL label2 ;

-- [DDL]
DROP SECURITY LABEL label3 ;


================================================================================
-- 来源: 4287_file_4287.txt
================================================================================

-- [OTHER]
SECURITY LABEL ON USER user1 is 'label1' ;

-- [OTHER]
SECURITY LABEL ON USER user2 is 'label3' ;

-- [OTHER]
SECURITY LABEL ON TABLE tbl is 'label2' ;

-- [DQL]
SELECT * FROM pg_seclabels ;

-- [DQL]
SELECT * FROM pg_seclabels;


================================================================================
-- 来源: 4288_file_4288.txt
================================================================================

-- [DCL_GRANT]
GRANT SELECT , INSERT ON tbl TO user1 , user2 ;


================================================================================
-- 来源: 4303_file_4303.txt
================================================================================

--查询t1_hash分区类型
-- [DQL]
SELECT relname, parttype FROM pg_class WHERE relname = 't1_hash';


================================================================================
-- 来源: 4304_file_4304.txt
================================================================================

--查询t1_hash分区类型
-- [DQL]
SELECT oid, relname, parttype FROM pg_class WHERE relname = 't1_hash';

--查询t1_hash的分区信息
-- [DQL]
SELECT oid, relname, parttype, parentid FROM pg_partition WHERE parentid = 16685;


================================================================================
-- 来源: 4307_file_4307.txt
================================================================================

-- [DQL]
SELECT * FROM range_sales PARTITION (p1);

-- [DQL]
SELECT * FROM range_sales PARTITION (p2);

-- [DQL]
SELECT * FROM range_sales PARTITION (p3);

-- 查看分区表信息
-- [DQL]
SELECT relname, boundaries, spcname FROM pg_partition p JOIN pg_tablespace t ON p.reltablespace=t.oid and p.parentid='tpcds.startend_pt'::regclass ORDER BY 1;


================================================================================
-- 来源: 4313_DQL_DML.txt
================================================================================

-- [DQL]
SELECT * FROM list_02 ORDER BY data;

-- 查询分区p_list_2数据
-- [DQL]
SELECT * FROM list_02 PARTITION (p_list_2) ORDER BY data;

-- 查询(100)所对应的分区的数据，即分区p_list_
-- [DQL]
SELECT * FROM list_02 PARTITION FOR (100) ORDER BY data;

-- [DML_DELETE]
DELETE FROM list_02 PARTITION (p_list_5);

-- 指定分区p_list_7插入数据，由于数据不符合该分区约束，插入报错
-- [DML_INSERT]
INSERT INTO list_02 PARTITION (p_list_7) VALUES(null, 'cherry', 'cherry data');

-- 将分区值100所属分区，即分区p_list_4的数据进行更新
-- [DML_UPDATE]
UPDATE list_02 PARTITION FOR (100) SET data = '';


================================================================================
-- 来源: 4316_file_4316.txt
================================================================================

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = 1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 < 1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 > 11;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 is NULL;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = 1 AND c2 = 2;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = 1 OR c1 = 2;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE NOT c1 = 1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 IN (1, 2, 3);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = ALL(ARRAY[1, 2, 3]);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = ANY(ARRAY[1, 2, 3]);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = SOME(ARRAY[1, 2, 3]);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = ALL(SELECT c2 FROM t1 WHERE c1 > 10);


================================================================================
-- 来源: 4318_PBE.txt
================================================================================

-- [PREPARED_STMT]
PREPARE p1(int) AS SELECT * FROM t1 WHERE c1 = $1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p1(1);

-- [PREPARED_STMT]
PREPARE p2(INT, INT) AS SELECT * FROM t1 WHERE c1 = $1 AND c2 = $2;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p2(1, 2);

-- [SESSION]
set plan_cache_mode = 'force_generic_plan';

-- [PREPARED_STMT]
PREPARE p3(TEXT) AS SELECT * FROM t1 WHERE c1 = $1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p3('12');

-- [PREPARED_STMT]
PREPARE p4(INT) AS SELECT * FROM t1 WHERE c1 = ALL(SELECT c2 FROM t1 WHERE c1 > $1);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p4(1);

-- [PREPARED_STMT]
PREPARE p5(name) AS SELECT * FROM t1 WHERE c1 = $1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p5('12');

-- [DDL]
create sequence seq;

-- [PREPARED_STMT]
PREPARE p6(TEXT) AS SELECT * FROM t1 WHERE c1 = currval($1);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p6('seq');


================================================================================
-- 来源: 4319_file_4319.txt
================================================================================

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) indexscan(t1) indexscan(t2) */ * FROM t2 JOIN t1 ON t1.c1 = t2.c1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) indexscan(t1) indexscan(t2) */ * FROM t2 JOIN t1 ON t1.c1 < t2.c1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) indexscan(t1) indexscan(t2) */ * FROM t2 JOIN t1 ON t1.c1 < t2.c1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) indexscan(t1) indexscan(t2) */ * FROM t2 JOIN t1 ON t1.c1 = t2.c1 AND t1.c2 = 2;

-- [SESSION]
set enable_seqscan=off;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) */ * FROM t2 JOIN t1 ON t1.c1 = t2.c1 OR t1.c2 = 2;

-- [DDL]
CREATE TABLE t3(c1 TEXT, c2 INT);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 JOIN t3 ON t1.c1 = t3.c1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 JOIN t3 ON t1.c1 = LENGTHB(t3.c1);


================================================================================
-- 来源: 4320_file_4320.txt
================================================================================

-- [DDL]
CREATE INDEX tpcds_web_returns_p2_index1 ON web_returns_p2 (ca_address_id) LOCAL;

-- [DDL]
CREATE INDEX tpcds_web_returns_p2_index2 ON web_returns_p2 (ca_address_sk) LOCAL ( PARTITION web_returns_p2_P1_index, PARTITION web_returns_p2_P2_index TABLESPACE example3, PARTITION web_returns_p2_P3_index TABLESPACE example4, PARTITION web_returns_p2_P4_index, PARTITION web_returns_p2_P5_index, PARTITION web_returns_p2_P6_index, PARTITION web_returns_p2_P7_index, PARTITION web_returns_p2_P8_index ) TABLESPACE example2;

-- [DDL]
CREATE INDEX tpcds_web_returns_p2_global_index ON web_returns_p2 (ca_street_number) GLOBAL;

-- [DDL]
CREATE INDEX tpcds_web_returns_for_p1 ON web_returns_p2 (ca_address_id) LOCAL(partition ind_part for p1);

-- [DDL]
CREATE INDEX tpcds_web_returns_for_p2 ON web_returns_p2 (ca_address_id) LOCAL(partition ind_part for (5000));

-- [DDL]
ALTER INDEX tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P2_index TABLESPACE example1;

-- [DDL]
ALTER INDEX tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P3_index TABLESPACE example2;

-- [DDL]
ALTER INDEX tpcds_web_returns_p2_index2 RENAME PARTITION web_returns_p2_P8_index TO web_returns_p2_P8_index_new;

-- [DQL]
SELECT RELNAME FROM PG_CLASS WHERE RELKIND='i' or RELKIND='I';

-- [DDL]
DROP INDEX tpcds_web_returns_p2_index1;


================================================================================
-- 来源: 4322_file_4322.txt
================================================================================

-- [DDL]
create table t1_range_int ( c1 int, c2 int, c3 int, c4 int ) partition by range(c1) ( partition range_p00 values less than(10), partition range_p01 values less than(20), partition range_p02 values less than(30), partition range_p03 values less than(40), partition range_p04 values less than(50) );

-- [DML_INSERT]
insert into t1_range_int select v,v,v,v from generate_series(0, 49) as v;

-- [MAINTENANCE]
analyze t1_range_int with all;

-- [DQL]
select relname, parttype, relpages, reltuples from pg_partition where parentid=(select oid from pg_class where relname='t1_range_int') order by relname;

-- [DQL]
select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_stats where tablename='t1_range_int' order by tablename, partitionname, attname;

-- [DDL]
ALTER TABLE t1_range_int ADD STATISTICS ((c2, c3));

-- [MAINTENANCE]
analyze t1_range_int with all;

-- [DQL]
select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_ext_stats where tablename='t1_range_int' order by tablename,partitionname,attname;

-- [DDL]
create index t1_range_int_index on t1_range_int(text(c1)) local;

-- [MAINTENANCE]
analyze t1_range_int with all;

-- [DQL]
select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_stats where tablename='t1_range_int_index' order by tablename,partitionname,attname;


================================================================================
-- 来源: 4323_file_4323.txt
================================================================================

-- [DQL]
select relname, relpages, reltuples from pg_partition where relname in ('id11', 'id22', 'max_id1');

-- [DQL]
select * from pg_stats where tablename ='only_fisrt_part' and partitionname ='id11';

-- [EXPLAIN]
EXPLAIN SELECT * FROM test_int4_maxvalue PARTITION(id1);

-- 查询指定分区max_id
-- [EXPLAIN]
EXPLAIN SELECT * FROM test_int4_maxvalue PARTITION(max_id);

-- 查询指定分区p_
-- [EXPLAIN]
EXPLAIN SELECT * FROM test_default PARTITION(p_1);

-- 查询指定分区p_
-- [EXPLAIN]
EXPLAIN SELECT * FROM test_default PARTITION(p_3);


================================================================================
-- 来源: 4351_file_4351.txt
================================================================================

-- [DDL]
CREATE TABLE TEST(a int);

-- [DDL]
CREATE TABLE TEST1(a int) with(orientation=row, storage_type=ustore);

-- [DDL]
CREATE TABLE TEST2(a int) with(orientation=row, storage_type=astore);

-- [DDL]
create table test4(a int) with(orientation=row);

-- [SESSION]
show enable_default_ustore_table;


================================================================================
-- 来源: 4363_Ustore.txt
================================================================================

-- [DDL]
CREATE TABLE ustore_table(a INT PRIMARY KEY, b CHAR (20)) WITH (STORAGE_TYPE=USTORE);

-- [DDL]
drop table ustore_table;

-- [DDL]
CREATE INDEX UB_tree_index ON test(a);

-- [DDL]
drop index ub_tree_index;


================================================================================
-- 来源: 4365_init_td.txt
================================================================================

-- [DDL]
CREATE TABLE test1(name varchar) WITH(storage_type = ustore, init_td=2);

-- [DDL]
ALTER TABLE test1 SET(init_td=8);

-- [DQL]
SELECT * FROM pg_thread_wait_status;

-- [DDL]
DROP TABLE test1;


================================================================================
-- 来源: 4366_fillfactor.txt
================================================================================

-- [DDL]
create table test(a int) with(fillfactor=100);

-- [DDL]
alter table test set(fillfactor=92);

-- [DDL]
drop table test;


================================================================================
-- 来源: 4387_file_4387.txt
================================================================================

-- [DQL]
select * from gs_global_config where name like '%undostoragetype%';


================================================================================
-- 来源: 4393_file_4393.txt
================================================================================

-- [DDL]
DROP TABLE IF EXISTS "public".flashtest;

-- [DDL]
CREATE TABLE "public".flashtest (col1 INT,col2 TEXT) with(storage_type=ustore);

-- [DQL]
select int8in(xidout(next_csn)) from gs_get_next_xid_csn();

-- [DQL]
select now();

-- [DML_INSERT]
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- [DQL]
SELECT * FROM flashtest;

-- [DQL]
SELECT * FROM flashtest TIMECAPSULE CSN 79351682;

-- [DQL]
SELECT * FROM flashtest;

-- [DQL]
SELECT * FROM flashtest TIMECAPSULE TIMESTAMP '2023-09-13 19:35:26.011986';

-- [DQL]
SELECT * FROM flashtest;

-- [DQL]
SELECT * FROM flashtest TIMECAPSULE TIMESTAMP to_timestamp ('2023-09-13 19:35:26.011986', 'YYYY-MM-DD HH24:MI:SS.FF');

-- [DQL]
SELECT * FROM flashtest AS ft TIMECAPSULE CSN 79351682;

-- [DDL]
drop TABLE IF EXISTS "public".flashtest;


================================================================================
-- 来源: 4394_file_4394.txt
================================================================================

-- [DDL]
DROP TABLE IF EXISTS "public".flashtest;

-- [DDL]
CREATE TABLE "public".flashtest (col1 INT,col2 TEXT) with(storage_type=ustore);

-- [DQL]
select int8in(xidout(next_csn)) from gs_get_next_xid_csn();

-- [DQL]
select now();

-- [DQL]
SELECT * FROM flashtest;

-- [DML_INSERT]
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- [DQL]
SELECT * FROM flashtest;

-- [OTHER]
TIMECAPSULE TABLE flashtest TO CSN 79352065;

-- [DQL]
SELECT * FROM flashtest;

-- [DQL]
select now();

-- [DML_INSERT]
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- [DQL]
SELECT * FROM flashtest;

-- [OTHER]
TIMECAPSULE TABLE flashtest TO TIMESTAMP to_timestamp ('2023-09-13 19:52:21.551028', 'YYYY-MM-DD HH24:MI:SS.FF');

-- [DQL]
SELECT * FROM flashtest;

-- [DQL]
select now();

-- [DML_INSERT]
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- [DQL]
SELECT * FROM flashtest;

-- [OTHER]
TIMECAPSULE TABLE flashtest TO TIMESTAMP '2023-09-13 19:54:00.641506';

-- [DQL]
SELECT * FROM flashtest;

-- [DDL]
drop TABLE IF EXISTS "public".flashtest;


================================================================================
-- 来源: 4395_DROP_TRUNCATE.txt
================================================================================

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- [DML_INSERT]
insert into flashtest values(1, 'A');

-- [DQL]
select * from flashtest;

-- [DDL]
drop table if EXISTS flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DQL]
select * from flashtest;

-- [OTHER]
PURGE TABLE flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest;

-- [DDL]
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- [DDL]
create index flashtest_index on flashtest(id);

-- [DDL]
drop table if EXISTS flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [OTHER]
PURGE index flashtest_index;

-- [DQL]
select * from gs_recyclebin;

-- [OTHER]
PURGE RECYCLEBIN;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest;

-- [DDL]
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- [DML_INSERT]
insert into flashtest values(1, 'A');

-- [DQL]
select * from flashtest;

-- [DDL]
drop table if EXISTS flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DQL]
select * from flashtest;

-- [OTHER]
timecapsule table flashtest to before drop;

-- [DQL]
select * from flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest;

-- [DQL]
select * from flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [OTHER]
timecapsule table "BIN$31C14EB48DC$9B4E$0==$0" to before drop;

-- [DQL]
select * from gs_recyclebin;

-- [DQL]
select * from flashtest;

-- [DDL]
drop table if EXISTS flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DQL]
select * from flashtest;

-- [OTHER]
timecapsule table flashtest to before drop rename to flashtest_rename;

-- [DQL]
select * from flashtest;

-- [DQL]
select * from flashtest_rename;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest_rename;

-- [OTHER]
PURGE RECYCLEBIN;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest;

-- [DDL]
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- [DML_INSERT]
insert into flashtest values(1, 'A');

-- [DQL]
select * from flashtest;

-- [DML_TRUNCATE]
truncate table flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DQL]
select * from flashtest;

-- [OTHER]
timecapsule table flashtest to before truncate;

-- [DQL]
select * from flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest;

-- [OTHER]
PURGE RECYCLEBIN;

-- [DQL]
select * from gs_recyclebin;


================================================================================
-- 来源: 4407_file_4407.txt
================================================================================

-- [DDL]
ALTER DATABASE SET ilm = on;

-- [OTHER]
List of relations Schema | Name | Type | Owner | Storage

-- [DQL]
SELECT a.oid, a.relname FROM pg_class a inner join pg_namespace b on a.relnamespace = b.oid WHERE (a.relname = 'gsilmpolicy_seq' OR a.relname = 'gsilmtask_seq') AND b.nspname = 'public';

-- [DDL]
CREATE TABLE ilm_table_1 (col1 int, col2 text) ilm add policy row store compress advanced row after 3 days of no modification on (col1 < 1000);

-- [DDL]
CREATE TABLE ilm_table_2 (col1 int, col2 text);

-- [DDL]
ALTER TABLE ilm_table_2 ilm add policy row store compress advanced row after 3 days of no modification;

-- [DQL]
SELECT * FROM gs_my_ilmpolicies;

-- [DQL]
SELECT * FROM gs_my_ilmdatamovementpolicies;

-- [DQL]
SELECT * FROM gs_my_ilmobjects;

-- [PLSQL]
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(11, 1);

-- [DML_INSERT]
INSERT INTO ilm_table_1 select *, 'test_data' FROM generate_series(1, 10000);

-- [PLSQL]
DECLARE v_taskid number;

-- [TCL]
BEGIN DBE_ILM.EXECUTE_ILM(OWNER => 'public', OBJECT_NAME => 'ilm_table_1', TASK_ID => v_taskid, SUBOBJECT_NAME => NULL, POLICY_NAME => 'ALL POLICIES', EXECUTION_MODE => 2);

-- [DQL]
SELECT * FROM gs_my_ilmtasks;

-- [DQL]
SELECT * FROM gs_my_ilmevaluationdetails;

-- [DQL]
SELECT * FROM gs_my_ilmresults;

-- [PLSQL]
DECLARE V_HOUR INT := 22;

-- [DQL]
SELECT * FROM gs_adm_ilmparameters;


================================================================================
-- 来源: 4409_TIPS.txt
================================================================================

-- [DDL]
DROP TABLE IF EXISTS ILM_TABLE;

-- [DDL]
CREATE TABLE ILM_TABLE(a int);

-- [DDL]
ALTER TABLE ILM_TABLE ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION;

-- [DQL]
SELECT * FROM gs_adm_ilmresults ORDER BY task_id desc;

-- [OTHER]
DBE_ILM.STOP_ILM (task_id => V_TASK, p_drop_running_Jobs => FALSE, p_Jobname => V_JOBNAME);

-- [DDL]
DROP TABLE IF EXISTS ILM_TABLE;

-- [DDL]
CREATE TABLE ILM_TABLE(a int);

-- [DDL]
ALTER TABLE ILM_TABLE ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION;

-- [PLSQL]
CALL DBE_ILM_ADMIN.DISABLE_ILM();

-- [PLSQL]
CALL DBE_ILM_ADMIN.ENABLE_ILM();

-- [PLSQL]
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(11, 1);

-- [PLSQL]
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(12, 10);

-- [PLSQL]
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(1, 1);

-- [PLSQL]
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(13, 512);

-- [OTHER]
DBE_COMPRESSION.GET_COMPRESSION_RATIO ( scratchtbsname IN VARCHAR2, ownname IN VARCHAR2, objname IN VARCHAR2, subobjname IN VARCHAR2, comptype IN NUMBER, blkcnt_cmp OUT PLS_INTEGER, blkcnt_uncmp OUT PLS_INTEGER, row_cmp OUT PLS_INTEGER, row_uncmp OUT PLS_INTEGER, cmp_ratio OUT NUMBER, comptype_str OUT VARCHAR2, sample_ratio IN INTEGER DEFAULT 20, objtype IN PLS_INTEGER DEFAULT OBJTYPE_TABLE);

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE user user1 IDENTIFIED BY '********' ;

-- [DDL]
CREATE user user2 IDENTIFIED BY '********' ;

-- [SESSION]
SET ROLE user1 PASSWORD '********' ;

-- [DDL]
CREATE TABLE TEST_DATA ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- [OTHER]
DBE_HEAT_MAP.ROW_HEAT_MAP( owner IN VARCHAR2, segment_name IN VARCHAR2, partition_name IN VARCHAR2 DEFAULT NULL, ctid IN VARCHAR2,);

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE Schema HEAT_MAP_DATA ;

-- [SESSION]
SET current_schema = HEAT_MAP_DATA ;

-- [DDL]
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1' ;

-- [DDL]
CREATE TABLE HEAT_MAP_DATA . heat_map_table ( id INT , value TEXT ) TABLESPACE example1 ;

-- [DML_INSERT]
INSERT INTO HEAT_MAP_DATA . heat_map_table VALUES ( 1 , 'test_data_row_1' );

-- [DQL]
SELECT * from DBE_HEAT_MAP . ROW_HEAT_MAP ( owner => 'heat_map_data' , segment_name => 'heat_map_table' , partition_name => NULL , ctid => '(0,1)' );

-- [DQL]
SELECT * FROM GS_ADM_ILMPARAMETERS;

-- [DQL]
SELECT * FROM GS_ADM_ILMPOLICIES;

-- [DQL]
SELECT * FROM GS_MY_ILMPOLICIES;

-- [DQL]
SELECT * FROM GS_ADM_ILMDATAMOVEMENTPOLICIES;

-- [DQL]
SELECT * FROM GS_MY_ILMDATAMOVEMENTPOLICIES;

-- [DQL]
SELECT * FROM GS_ADM_ILMOBJECTS;

-- [DQL]
SELECT * FROM GS_MY_ILMOBJECTS;

-- [DQL]
SELECT * FROM GS_ADM_ILMTASKS;

-- [DQL]
SELECT * FROM GS_MY_ILMTASKS;

-- [DQL]
SELECT * FROM GS_ADM_ILMEVALUATIONDETAILS;

-- [DQL]
SELECT * FROM GS_MY_ILMEVALUATIONDETAILS;

-- [DQL]
SELECT * FROM GS_ADM_ILMRESULTS;

-- [DQL]
SELECT * FROM GS_MY_ILMRESULTS;


================================================================================
-- 来源: 4433_query.txt
================================================================================

-- [DQL]
select "table", "column" from gs_index_advise('SELECT c_discount from bmsql_customer where c_w_id = 10');

-- [DQL]
select "table", "column" from gs_index_advise('select name, age, sex from t1 where age >= 18 and age < 35 and sex = ' 'f ' ';

-- [DQL]
select "table", "column", "indextype" from gs_index_advise('select name, age, sex from range_table where age = 20;


================================================================================
-- 来源: 4434_file_4434.txt
================================================================================

-- [DQL]
select * from hypopg_create_index('create index on bmsql_customer(c_w_id)');

-- [SESSION]
set enable_hypo_index = on;

-- [EXPLAIN]
explain SELECT c_discount from bmsql_customer where c_w_id = 10;

-- [EXPLAIN]
explain SELECT c_discount from bmsql_customer where c_w_id = 10;

-- [DQL]
select * from hypopg_display_index();

-- [DQL]
select * from hypopg_estimate_size(329729);

-- [DQL]
select * from hypopg_drop_index(329726);

-- [DQL]
select * from hypopg_reset_index();


================================================================================
-- 来源: 4493_file_4493.txt
================================================================================

-- [DDL]
CREATE TABLE t1(c1 int, c2 int);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1, 1);

-- [DML_INSERT]
INSERT INTO t1 VALUES(2, 2);

--创建全量物化视图。
-- [DDL]
CREATE MATERIALIZED VIEW mv AS select count(*) from t1;

--查询物化视图结果。
-- [DQL]
SELECT * FROM mv;

--向物化视图中基表插入数据。
-- [DML_INSERT]
INSERT INTO t1 VALUES(3, 3);

--对全量物化视图做全量刷新。
-- [OTHER]
REFRESH MATERIALIZED VIEW mv;

--查询物化视图结果。
-- [DQL]
SELECT * FROM mv;

--删除物化视图，删除表。
-- [DDL]
DROP MATERIALIZED VIEW mv;

-- [DDL]
DROP TABLE t1;


================================================================================
-- 来源: 4497_file_4497.txt
================================================================================

-- [DDL]
CREATE TABLE t1(c1 int, c2 int);

-- [DML_INSERT]
INSERT INTO t1 VALUES(1, 1);

-- [DML_INSERT]
INSERT INTO t1 VALUES(2, 2);

--创建增量物化视图。
-- [DDL]
CREATE INCREMENTAL MATERIALIZED VIEW mv AS SELECT * FROM t1;

--插入数据。
-- [DML_INSERT]
INSERT INTO t1 VALUES(3, 3);

--增量刷新物化视图。
-- [OTHER]
REFRESH INCREMENTAL MATERIALIZED VIEW mv;

--查询物化视图结果。
-- [DQL]
SELECT * FROM mv;

--插入数据。
-- [DML_INSERT]
INSERT INTO t1 VALUES(4, 4);

--全量刷新物化视图。
-- [OTHER]
REFRESH MATERIALIZED VIEW mv;

--查询物化视图结果。
-- [DQL]
select * from mv;

--删除物化视图，删除表。
-- [DDL]
DROP MATERIALIZED VIEW mv;

-- [DDL]
DROP TABLE t1;


================================================================================
-- 来源: 4500_gsql.txt
================================================================================

-- [OTHER]
CREATE CLIENT MASTER KEY cmk1 WITH ( KEY_STORE = hcs_kms , KEY_PATH = '{KMS服务器地址}/{密钥ID}', ALGORITHM = AES_256);

-- [OTHER]
CREATE COLUMN ENCRYPTION KEY cek1 WITH VALUES (CLIENT_MASTER_KEY = cmk1, ALGORITHM = AES_256_GCM);

-- [DDL]
CREATE TABLE creditcard_info ( id_number int, name text encrypted with (column_encryption_key = cek1, encryption_type = DETERMINISTIC), credit_card varchar(19) encrypted with (column_encryption_key = cek1, encryption_type = DETERMINISTIC));

-- [DML_INSERT]
INSERT INTO creditcard_info VALUES (1,'joe','6217986500001288393');

-- [DML_INSERT]
INSERT INTO creditcard_info VALUES (2, 'joy','6219985678349800033');

-- 从加密表中查询数据
-- [DQL]
select * from creditcard_info where name = 'joe';

-- 更新加密表中数据
-- [DML_UPDATE]
update creditcard_info set credit_card = '80000000011111111' where name = 'joy';

-- 向表中新增一列加密列
-- [DDL]
ALTER TABLE creditcard_info ADD COLUMN age int ENCRYPTED WITH (COLUMN_ENCRYPTION_KEY = cek1, ENCRYPTION_TYPE = DETERMINISTIC);

-- 从表中删除一列加密列
-- [DDL]
ALTER TABLE creditcard_info DROP COLUMN age;

-- 从系统表中查询主密钥信息
-- [DQL]
SELECT * FROM gs_client_global_keys;

-- 从系统表中查询列密钥信息
-- [DQL]
SELECT column_key_name,column_key_distributed_id ,global_key_id,key_owner FROM gs_column_keys;

-- [DDL]
DROP TABLE creditcard_info;

-- 删除列密钥
-- [OTHER]
DROP COLUMN ENCRYPTION KEY cek1;

-- 删除主密钥
-- [OTHER]
DROP CLIENT MASTER KEY cmk1;


================================================================================
-- 来源: 4504__.txt
================================================================================

-- [DDL]
CREATE TABLE creditcard_info ( id_number int , name text , credit_card varchar ( 19 ) encrypted with ( column_encryption_key = cek1 , encryption_type = DETERMINISTIC ) ) with ( orientation = row );

-- [DML_INSERT]
insert into creditcard_info values ( 1 , 'Avi' , '1234567890123456' );

-- [DML_INSERT]
insert into creditcard_info values ( 2 , 'Eli' , '2345678901234567' );

-- [DDL]
CREATE FUNCTION f_encrypt_in_sql ( val1 text , val2 varchar ( 19 )) RETURNS text AS 'SELECT name from creditcard_info where name=$1 or credit_card=$2 LIMIT 1' LANGUAGE SQL ;

-- [DDL]
CREATE FUNCTION f_encrypt_in_plpgsql ( val1 text , val2 varchar ( 19 ), OUT c text ) AS $$ BEGIN SELECT into c name from creditcard_info where name = $ 1 or credit_card = $ 2 LIMIT 1 ;

-- [DQL]
SELECT f_encrypt_in_sql ( 'Avi' , '1234567890123456' );

-- [DQL]
SELECT f_encrypt_in_plpgsql ( 'Avi' , val2 => '1234567890123456' );


================================================================================
-- 来源: 4507_gsql.txt
================================================================================

-- [OTHER]
CREATE CLIENT MASTER KEY cmk1 WITH ( KEY_STORE = hcs_kms , KEY_PATH = '{KMS服务器地址}/{密钥ID}', ALGORITHM = AES_256);

-- [OTHER]
CREATE COLUMN ENCRYPTION KEY cek1 WITH VALUES (CLIENT_MASTER_KEY = cmk1, ALGORITHM = AES_256_GCM);

-- [DDL]
CREATE TABLE contacts ( id int unique , credit float8 encrypted with ( column_encryption_key = cek1 , encryption_type = DETERMINISTIC ), name text encrypted with ( column_encryption_key = cek1 , encryption_type = DETERMINISTIC ));

-- [DDL]
CREATE TABLE contacts_plain ( id int unique , credit float8 , name text );

-- [DML_INSERT]
INSERT INTO contacts VALUES ( 1 , 8000 , 'zhangsan' );

-- [DML_INSERT]
INSERT INTO contacts VALUES ( 2 , 7056 . 6 , 'lisi' );

-- [DML_INSERT]
INSERT INTO contacts VALUES ( 3 , 16050 , 'wangwu' );

-- [DQL]
select id,credit from contacts where credit > 10000;

-- [DQL]
select id,credit from contacts where credit < 10000;

-- [DQL]
select id,credit from contacts where credit >= 8000;

-- [DQL]
select id,credit from contacts where credit <= 8000;

-- [DQL]
select id,credit from contacts order by credit;

-- [DQL]
select id,credit from contacts order by credit DESC;

-- [DQL]
select credit*2 from contacts limit 1;

-- [DQL]
select sum(credit) from contacts;

-- [DQL]
select case when credit > 9000 then name end from contacts;

-- [DQL]
select credit::text, credit::int from contacts offset 1 limit 1;

-- [DQL]
select credit from contacts where name like 'zhang%';

-- [DML_INSERT]
insert into contacts_plain (id, name) select id, ce_decrypt_deterministic(name, (select column_key_distributed_id from gs_column_keys where column_key_name=' cek1 ')) from contacts;

-- [DML_DELETE]
delete from contacts;

-- [DML_INSERT]
insert into contacts (id, name) select id, ce_encrypt_deterministic(name, (select column_key_distributed_id from gs_column_keys where column_key_name=' cek1 ')) from contacts_plain;

-- [DDL]
DROP TABLE contacts, contacts_plain;

-- [OTHER]
DROP COLUMN ENCRYPTION KEY cek1;

-- [OTHER]
DROP CLIENT MASTER KEY cmk1;


================================================================================
-- 来源: 4511_file_4511.txt
================================================================================

-- [SESSION]
SHOW enable_tde;

-- [SESSION]
show tde_key_info;

-- [DDL]
CREATE TABLE t1 (c1 INT, c2 TEXT) WITH (enable_tde = on);

-- [DDL]
CREATE TABLE t2 (c1 INT, c2 TEXT) WITH (enable_tde = on, encrypt_algo = 'SM4_CTR');

-- [DQL]
SELECT relname,reloptions FROM pg_class WHERE relname = 't1';

-- [DML_INSERT]
INSERT INTO t1 VALUES (1, 'tde plain 123');

-- [DQL]
SELECT * FROM t1;

-- [DDL]
ALTER TABLE t1 ENCRYPTION KEY ROTATION;

-- [DDL]
ALTER TABLE t1 SET (enable_tde = off);

-- [MAINTENANCE]
VACUUM FULL t1;

-- [DDL]
ALTER TABLE t1 SET (enable_tde = on);

-- [MAINTENANCE]
VACUUM FULL t1;

-- [DDL]
DROP TABLE t1;


================================================================================
-- 来源: 4513_file_4513.txt
================================================================================

-- [DDL]
CREATE SECURITY LABEL label1 'L1:G2,G4' ;

-- [DDL]
CREATE SECURITY LABEL label2 'L2:G2-G4' ;

-- [DDL]
CREATE SECURITY LABEL label3 'L3:G1-G5' ;

-- [DQL]
SELECT * FROM gs_security_label ;

-- [DQL]
SELECT * FROM gs_security_label;

-- [DDL]
DROP SECURITY LABEL label1 ;

-- [DDL]
DROP SECURITY LABEL label2 ;

-- [DDL]
DROP SECURITY LABEL label3 ;


================================================================================
-- 来源: 4514_file_4514.txt
================================================================================

-- [OTHER]
SECURITY LABEL ON USER user1 is 'label1' ;

-- [OTHER]
SECURITY LABEL ON USER user2 is 'label3' ;

-- [OTHER]
SECURITY LABEL ON TABLE tbl is 'label2' ;

-- [DQL]
SELECT * FROM pg_seclabels ;

-- [DQL]
SELECT * FROM pg_seclabels;


================================================================================
-- 来源: 4515_file_4515.txt
================================================================================

-- [DCL_GRANT]
GRANT SELECT , INSERT ON tbl TO user1 , user2 ;


================================================================================
-- 来源: 4522_DDL.txt
================================================================================

-- [DDL]
CREATE TABLE test_create_table_partition2 (c1 INT, c2 INT) PARTITION BY RANGE (c2) ( PARTITION p1 START(1) END(1000) EVERY(200) , PARTITION p2 END(2000), PARTITION p3 START(2000) END(2500), PARTITION p4 START(2500), PARTITION p5 START(3000) END(5000) EVERY(1000) );

-- [DDL]
CREATE TABLE test_create_table_partition2 (c1 INT, c2 INT) PARTITION BY RANGE (c2) ( PARTITION p1_0 VALUES LESS THAN ('1'), PARTITION p1_1 VALUES LESS THAN ('201'), PARTITION p1_2 VALUES LESS THAN ('401'), PARTITION p1_3 VALUES LESS THAN ('601'), PARTITION p1_4 VALUES LESS THAN ('801'), PARTITION p1_5 VALUES LESS THAN ('1000'), PARTITION p2 VALUES LESS THAN ('2000'), PARTITION p3 VALUES LESS THAN ('2500'), PARTITION p4 VALUES LESS THAN ('3000'), PARTITION p5_1 VALUES LESS THAN ('4000'), PARTITION p5_2 VALUES LESS THAN ('5000') );

-- [DDL]
CREATE TABLE IF NOT EXISTS tb5 (c1 int,c2 int) with (ORIENTATION=ROW, STORAGE_TYPE=USTORE);

-- [DDL]
ALTER TABLE IF EXISTS tb5 * ADD COLUMN IF NOT EXISTS c2 char(5) after c1;

-- [DDL]
ALTER TABLE IF EXISTS public.tb5 ADD COLUMN IF NOT EXISTS c2 pg_catalog.int4 AFTER c1;

-- [DDL]
ALTER TABLE IF EXISTS tb5 * ADD COLUMN IF NOT EXISTS c2 char(5) after c1, ADD COLUMN IF NOT EXISTS c3 char(5) after c1;

-- [DDL]
ALTER TABLE IF EXISTS public.tb5 ADD COLUMN IF NOT EXISTS c2 pg_catalog.int4 AFTER c1, ADD COLUMN IF NOT EXISTS c3 pg_catalog.bpchar(5) AFTER c1;

-- [DDL]
ALTER TABLE IF EXISTS tb5 * ADD COLUMN c2 char(5) after c1, ADD COLUMN IF NOT EXISTS c4 int after c1;

-- [DDL]
ALTER TABLE tbl_28 ADD COLUMN b1 TIMESTAMP DEFAULT NOW();

-- [DDL]
ALTER TABLE tbl_28 ADD COLUMN b2 INT DEFAULT RANDOM();

-- [DDL]
ALTER TABLE tbl_28 ADD COLUMN b3 INT DEFAULT ABS(1);

-- [DDL]
CREATE TABLE IF NOT EXISTS tb1 (c1 time without time zone ON UPDATE CURRENT_TIMESTAMP) with (ORIENTATION=ROW, STORAGE_TYPE=USTORE);

-- B兼容模式下修改表，列字段添加ON UPDATE事件
-- [DDL]
ALTER TABLE IF EXISTS ONLY tb2 MODIFY COLUMN c2 time without time zone ON UPDATE LOCALTIMESTAMP;

-- [DDL]
CREATE TABLE IF NOT EXISTS public.tb1 (c1 TIME) WITH (orientation = 'row', storage_type = 'ustore', compression = 'no') NOCOMPRESS;

-- [DDL]
ALTER TABLE IF EXISTS ONLY public.tb2 MODIFY COLUMN c2 TIME;

-- [DDL]
CREATE TABLE IF NOT EXISTS tb3 (c1 int) with (storage_type=USTORE,ORIENTATION=ROW) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 7 day OF NO MODIFICATION;

-- [DDL]
CREATE TABLE IF NOT EXISTS public.tb3 (c1 pg_catalog.int4) WITH (storage_type = 'ustore', orientation = 'row', compression = 'no') NOCOMPRESS;

-- [DDL]
CREATE TABLE IF NOT EXISTS tb6 (c1 integer comment 'Mysql兼容注释语法') with (ORIENTATION=ROW, STORAGE_TYPE=USTORE);

-- [DDL]
CREATE TABLE IF NOT EXISTS public.tb6 (c1 pg_catalog.int4) WITH (storage_type = 'ustore', orientation = 'row', compression = 'no') NOCOMPRESS;

-- [TCL]
BEGIN;

-- [OTHER]
GAINT ALL PRIVILEGES to u01;

-- [DML_INSERT]
INSERT INTO test1(col1) values(1);

-- [TCL]
COMMIT;

-- 只反解析第一句和第三句SQL语句
-- [TCL]
BEGIN;

-- [DDL]
CREATE TABLE mix_tran_t4(id int);

-- [DML_INSERT]
INSERT INTO mix_tran_t4 VALUES(111);

-- [DDL]
CREATE TABLE mix_tran_t5(id int);

-- [TCL]
COMMIT;

-- 只反解析第一句和第二句SQL语句
-- [TCL]
BEGIN;

-- [DML_INSERT]
INSERT INTO mix_tran_t4 VALUES(111);

-- [DDL]
CREATE TABLE mix_tran_t6(id int);

-- [DML_INSERT]
INSERT INTO mix_tran_t4 VALUES(111);

-- [TCL]
COMMIT;

-- 全反解析
-- [TCL]
BEGIN;

-- [DML_INSERT]
INSERT INTO mix_tran_t4 VALUES(111);

-- [DDL]
CREATE TABLE mix_tran_t7(id int);

-- [DDL]
CREATE TABLE mix_tran_t8(id int);

-- [TCL]
COMMIT;

-- 只反解析第一句和第三句SQL语句
-- [TCL]
BEGIN;

-- [DDL]
CREATE TABLE mix_tran_t7(id int);

-- [DDL]
CREATE TYPE compfoo AS (f1 int, f2 text);

-- [DDL]
CREATE TABLE mix_tran_t8(id int);

-- [TCL]
COMMIT;

-- 全反解析
-- [TCL]
BEGIN;

-- [DML_INSERT]
INSERT INTO mix_tran_t4 VALUES(111);

-- [DML_INSERT]
INSERT INTO mix_tran_t4 VALUES(111);

-- [DML_INSERT]
INSERT INTO mix_tran_t4 VALUES(111);

-- [TCL]
COMMIT;

-- 只反解析第一句SQL语句
-- [TCL]
BEGIN;

-- [DML_INSERT]
INSERT INTO mix_tran_t4 VALUES(111);

-- [DDL]
CREATE TYPE compfoo AS (f1 int, f2 text);

-- [DML_INSERT]
INSERT INTO mix_tran_t4 VALUES(111);

-- [TCL]
COMMIT;

-- 只反解析第一句和第三句SQL语句
-- [TCL]
BEGIN;

-- [DML_INSERT]
INSERT INTO mix_tran_t4 VALUES(111);

-- [DDL]
CREATE TYPE compfoo AS (f1 int, f2 text);

-- [DDL]
CREATE TABLE mix_tran_t9(id int);

-- [TCL]
COMMIT;

-- [OTHER]
gs_guc set -Z datanode -D $node_dir -c "wal_level = logical" 其中，$node_dir为数据库节点路径，用户可根据实际情况替换。 使用如下命令连接数据库。

-- [OTHER]
gsql -d gaussdb -p 20000 -r 其中，gaussdb为需要连接的数据库名称，20000为数据库端口号，用户可根据实际情况替换。 创建名称为slot1的逻辑复制槽。 1 2 3 4

-- [DQL]
SELECT * FROM pg_create_logical_replication_slot ( 'slot1' , 'mppdb_decoding' );

-- [DDL]
CREATE OR REPLACE PACKAGE ldp_pkg1 IS var1 int : = 1 ;

-- [DQL]
SELECT data FROM pg_logical_slot_peek_changes ( 'ldp_ddl_replica_slot' , NULL , NULL , 'enable-ddl-decoding' , 'true' , 'enable-ddl-json-format' , 'false' ) WHERE data not like 'BEGIN%' AND data not like 'COMMIT%' AND data not like '%dbe_pldeveloper.gs_source%' ;

-- [DQL]
SELECT * FROM pg_drop_replication_slot ( 'slot1' );


================================================================================
-- 来源: 4531_file_4531.txt
================================================================================

--查询t1_hash分区类型
-- [DQL]
SELECT relname, parttype FROM pg_class WHERE relname = 't1_hash';

--查询t1_sub_rr分区类型
-- [DQL]
SELECT relname, parttype FROM pg_class WHERE relname = 't1_sub_rr';


================================================================================
-- 来源: 4532_file_4532.txt
================================================================================

--查询t1_hash分区类型
-- [DQL]
SELECT oid, relname, parttype FROM pg_class WHERE relname = 't1_hash';

--查询t1_hash的分区信息
-- [DQL]
SELECT oid, relname, parttype, parentid FROM pg_partition WHERE parentid = 16685;


================================================================================
-- 来源: 4535_file_4535.txt
================================================================================

-- [DQL]
SELECT * FROM range_sales PARTITION (p1);

-- [DQL]
SELECT * FROM range_sales PARTITION (p2);

-- [DQL]
SELECT * FROM range_sales PARTITION (p3);

-- 查看分区表信息
-- [DQL]
SELECT relname, boundaries, spcname FROM pg_partition p JOIN pg_tablespace t ON p.reltablespace=t.oid and p.parentid='tpcds.startend_pt'::regclass ORDER BY 1;


================================================================================
-- 来源: 4543_DQL_DML.txt
================================================================================

-- [DQL]
SELECT * FROM list_list_02 ORDER BY data;

-- 查询分区p_list_4数据
-- [DQL]
SELECT * FROM list_list_02 PARTITION (p_list_4) ORDER BY data;

-- 查询(100, 100)所对应的二级分区的数据，即二级分区p_list_4_subpartdefault1，这个分区是在p_list_4下自动创建的一个分区范围定义为DEFAULT的分区
-- [DQL]
SELECT * FROM list_list_02 SUBPARTITION FOR(100, 100) ORDER BY data;

-- 查询分区p_list_2 数据
-- [DQL]
SELECT * FROM list_list_02 PARTITION (p_list_2) ORDER BY data;

-- 查询(0, 100)所对应的二级分区的数据，即二级分区p_list_2_
-- [DQL]
SELECT * FROM list_list_02 SUBPARTITION FOR (0, 100) ORDER BY data;

-- [DML_DELETE]
DELETE FROM list_list_02 PARTITION (p_list_5);

-- 指定分区p_list_7_1插入数据，由于数据不符合该分区约束，插入报错
-- [DML_INSERT]
INSERT INTO list_list_02 SUBPARTITION (p_list_7_1) VALUES(null, 'cherry', 'cherry data');

-- 将一级分区值100所属分区的数据进行更新
-- [DML_UPDATE]
UPDATE list_list_02 PARTITION FOR (100) SET id = 1;


================================================================================
-- 来源: 4546_file_4546.txt
================================================================================

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = 1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 < 1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 > 11;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 is NULL;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = 1 AND c2 = 2;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = 1 OR c1 = 2;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE NOT c1 = 1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 IN (1, 2, 3);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = ALL(ARRAY[1, 2, 3]);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = ANY(ARRAY[1, 2, 3]);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = SOME(ARRAY[1, 2, 3]);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 WHERE c1 = ALL(SELECT c2 FROM t1 WHERE c1 > 10);


================================================================================
-- 来源: 4548_PBE.txt
================================================================================

--设置参数
-- [SESSION]
set plan_cache_mode = 'force_generic_plan';

-- [PREPARED_STMT]
PREPARE p1(int) AS SELECT * FROM t1 WHERE c1 = $1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p1(1);

-- [PREPARED_STMT]
PREPARE p2(int) AS SELECT * FROM t1 WHERE c1 < $1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p2(1);

-- [PREPARED_STMT]
PREPARE p3(int) AS SELECT * FROM t1 WHERE c1 > $1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p3(1);

-- [PREPARED_STMT]
PREPARE p5(INT, INT) AS SELECT * FROM t1 WHERE c1 = $1 AND c2 = $2;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p5(1, 2);

-- [PREPARED_STMT]
PREPARE p6(INT, INT) AS SELECT * FROM t1 WHERE c1 = $1 OR c2 = $2;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p6(1, 2);

-- [PREPARED_STMT]
PREPARE p7(INT) AS SELECT * FROM t1 WHERE NOT c1 = $1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) execute p7(1);

-- [PREPARED_STMT]
PREPARE p8(INT, INT, INT) AS SELECT * FROM t1 WHERE c1 IN ($1, $2, $3);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p8(1, 2, 3);

-- [PREPARED_STMT]
PREPARE p9(INT, INT, INT) AS SELECT * FROM t1 WHERE c1 NOT IN ($1, $2, $3);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p9(1, 2, 3);

-- [PREPARED_STMT]
PREPARE p10(INT, INT, INT) AS SELECT * FROM t1 WHERE c1 = ALL(ARRAY[$1, $2, $3]);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p10(1, 2, 3);

-- [PREPARED_STMT]
PREPARE p11(INT, INT, INT) AS SELECT * FROM t1 WHERE c1 = ANY(ARRAY[$1, $2, $3]);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p11(1, 2, 3);

-- [PREPARED_STMT]
PREPARE p12(INT, INT, INT) AS SELECT * FROM t1 WHERE c1 = SOME(ARRAY[$1, $2, $3]);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p12(1, 2, 3);

-- [SESSION]
set plan_cache_mode = 'force_generic_plan';

-- [PREPARED_STMT]
PREPARE p13(TEXT) AS SELECT * FROM t1 WHERE c1 = $1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p13('12');

-- [PREPARED_STMT]
PREPARE p14(TEXT) AS SELECT * FROM t1 WHERE c1 = LENGTHB($1);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p14('hello');

-- [PREPARED_STMT]
PREPARE p15(INT) AS SELECT * FROM t1 WHERE c1 = ALL(SELECT c2 FROM t1 WHERE c1 > $1);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p15(1);

-- [PREPARED_STMT]
PREPARE p16(name) AS SELECT * FROM t1 WHERE c1 = $1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p16('12');

-- [DDL]
create sequence seq;

-- [PREPARED_STMT]
PREPARE p17(TEXT) AS SELECT * FROM t1 WHERE c1 = currval($1);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) EXECUTE p17('seq');


================================================================================
-- 来源: 4549_file_4549.txt
================================================================================

-- [DDL]
CREATE TABLE t1 (c1 INT, c2 INT) PARTITION BY RANGE (c1) ( PARTITION p1 VALUES LESS THAN(10), PARTITION p2 VALUES LESS THAN(20), PARTITION p3 VALUES LESS THAN(MAXVALUE) );

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t2 JOIN t1 ON t1.c1 = t2.c2;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t2 JOIN t1 ON t1.c1 < t2.c2;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t2 JOIN t1 ON t1.c1 > t2.c2;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t2 JOIN t1 ON t1.c1 = t2.c2 AND t1.c2 = 2;

-- [SESSION]
set enable_seqscan=off;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t2 JOIN t1 ON t1.c1 = t2.c2 OR t1.c1 = 2;

-- [DDL]
CREATE TABLE t3(c1 TEXT, c2 INT);

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 JOIN t3 ON t1.c1 = t3.c1;

-- [EXPLAIN]
EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 JOIN t3 ON t1.c1 = LENGTHB(t3.c1);


================================================================================
-- 来源: 4551_Partition Iterator.txt
================================================================================

-- [EXPLAIN]
EXPLAIN SELECT * FROM test_range_pt WHERE a = 3000;

-- [SESSION]
SET partition_iterator_elimination = on;

-- [EXPLAIN]
EXPLAIN SELECT * FROM test_range_pt WHERE a = 3000;


================================================================================
-- 来源: 4552_Merge Append.txt
================================================================================

-- [EXPLAIN]
EXPLAIN ANALYZE SELECT * FROM test_range_pt WHERE b >10 AND b < 5000 ORDER BY b LIMIT 10;

--关闭分区表Merge Append算子
-- [SESSION]
SET sql_beta_feature = 'disable_merge_append_partition';

-- [EXPLAIN]
EXPLAIN ANALYZE SELECT * FROM test_range_pt WHERE b >10 AND b < 5000 ORDER BY b LIMIT 10;


================================================================================
-- 来源: 4553_Max_Min.txt
================================================================================

-- [EXPLAIN]
explain analyze select min(b) from test_range_pt;

-- [EXPLAIN]
explain analyze select min(b) from test_range_pt;


================================================================================
-- 来源: 4554_file_4554.txt
================================================================================

--INSERT常量，执行FastPath优化
-- [EXPLAIN]
explain insert into fastpath_t1 values (0, 'test_insert');

--INSERT带参数/简单表达式，执行FastPath优化
-- [PREPARED_STMT]
prepare insert_t1 as insert into fastpath_t1 values($1 + 1 + $2, $2);

-- [EXPLAIN]
explain execute insert_t1(10, '0');

--INSERT为子查询，无法执行FastPath优化，走标准执行器模块
-- [DDL]
create table test_1(col1 int, col3 text);

-- [EXPLAIN]
explain insert into fastpath_t1 select * from test_1;


================================================================================
-- 来源: 4555_file_4555.txt
================================================================================

-- [DDL]
CREATE INDEX tpcds_web_returns_p2_index1 ON web_returns_p2 (ca_address_id) LOCAL;

-- [DDL]
CREATE INDEX tpcds_web_returns_p2_index2 ON web_returns_p2 (ca_address_sk) LOCAL ( PARTITION web_returns_p2_P1_index, PARTITION web_returns_p2_P2_index TABLESPACE example3, PARTITION web_returns_p2_P3_index TABLESPACE example4, PARTITION web_returns_p2_P4_index, PARTITION web_returns_p2_P5_index, PARTITION web_returns_p2_P6_index, PARTITION web_returns_p2_P7_index, PARTITION web_returns_p2_P8_index ) TABLESPACE example2;

-- [DDL]
CREATE INDEX tpcds_web_returns_p2_global_index ON web_returns_p2 (ca_street_number) GLOBAL;

-- [DDL]
CREATE INDEX tpcds_web_returns_for_p1 ON web_returns_p2 (ca_address_id) LOCAL(partition ind_part for p1);

-- [DDL]
CREATE INDEX tpcds_web_returns_for_p2 ON web_returns_p2 (ca_address_id) LOCAL(partition ind_part for (5000));

-- [DDL]
ALTER INDEX tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P2_index TABLESPACE example1;

-- [DDL]
ALTER INDEX tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P3_index TABLESPACE example2;

-- [DDL]
ALTER INDEX tpcds_web_returns_p2_index2 RENAME PARTITION web_returns_p2_P8_index TO web_returns_p2_P8_index_new;

-- [DQL]
SELECT RELNAME FROM PG_CLASS WHERE RELKIND='i' or RELKIND='I';

-- [DDL]
DROP INDEX tpcds_web_returns_p2_index1;


================================================================================
-- 来源: 4557_file_4557.txt
================================================================================

-- [DDL]
create table t1_range_int ( c1 int, c2 int, c3 int, c4 int ) partition by range(c1) ( partition range_p00 values less than(10), partition range_p01 values less than(20), partition range_p02 values less than(30), partition range_p03 values less than(40), partition range_p04 values less than(50) );

-- [DML_INSERT]
insert into t1_range_int select v,v,v,v from generate_series(0, 49) as v;

-- [MAINTENANCE]
analyze t1_range_int with all;

-- [DQL]
select relname, parttype, relpages, reltuples from pg_partition where parentid=(select oid from pg_class where relname='t1_range_int') order by relname;

-- [DQL]
select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_stats where tablename='t1_range_int' order by tablename, partitionname, attname;

-- [DDL]
ALTER TABLE t1_range_int ADD STATISTICS ((c2, c3));

-- [MAINTENANCE]
analyze t1_range_int with all;

-- [DQL]
select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_ext_stats where tablename='t1_range_int' order by tablename,partitionname,attname;

-- [DDL]
create index t1_range_int_index on t1_range_int(text(c1)) local;

-- [MAINTENANCE]
analyze t1_range_int with all;

-- [DQL]
select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_stats where tablename='t1_range_int_index' order by tablename,partitionname,attname;


================================================================================
-- 来源: 4558_file_4558.txt
================================================================================

-- [DQL]
select relname, relpages, reltuples from pg_partition where relname in ('id11', 'id22', 'max_id1');

-- [DQL]
select * from pg_stats where tablename ='only_fisrt_part' and partitionname ='id11';

-- 查询指定分区id
-- [EXPLAIN]
EXPLAIN SELECT * FROM test_int4_maxvalue PARTITION(id1);

-- 查询指定分区max_id
-- [EXPLAIN]
EXPLAIN SELECT * FROM test_int4_maxvalue PARTITION(max_id);

-- 查询指定分区p_
-- [EXPLAIN]
EXPLAIN SELECT * FROM test_default PARTITION(p_1);

-- [EXPLAIN]
EXPLAIN SELECT * FROM test_default PARTITION(p_3);


================================================================================
-- 来源: 4611_file_4611.txt
================================================================================

-- [DDL]
CREATE TABLE TEST(a int);

-- [DDL]
CREATE TABLE TEST1(a int) with(orientation=row, storage_type=ustore);

-- [DDL]
CREATE TABLE TEST2(a int) with(orientation=row, storage_type=astore);

-- [DDL]
create table test4(a int) with(orientation=row);

-- [SESSION]
show enable_default_ustore_table;


================================================================================
-- 来源: 4623_Ustore.txt
================================================================================

-- [DDL]
CREATE TABLE ustore_table(a INT PRIMARY KEY, b CHAR (20)) WITH (STORAGE_TYPE=USTORE);

-- [DDL]
drop table ustore_table;

-- [DDL]
CREATE INDEX UB_tree_index ON test(a);

-- [DDL]
drop index ub_tree_index;


================================================================================
-- 来源: 4625_init_td.txt
================================================================================

-- [DDL]
create table test1(name varchar) with(storage_type = ustore, init_td=2);

-- [DDL]
alter table test1 set(init_td=8);

-- [DQL]
select * from pg_thread_wait_status;

-- [DDL]
drop table test1;


================================================================================
-- 来源: 4626_fillfactor.txt
================================================================================

-- [DDL]
create table test(a int) with(fillfactor=100);

-- [DDL]
alter table test set(fillfactor=92);

-- [DDL]
drop table test;


================================================================================
-- 来源: 4647_file_4647.txt
================================================================================

-- [DQL]
select * from gs_global_config where name like '%undostoragetype%';


================================================================================
-- 来源: 4653_file_4653.txt
================================================================================

-- [DDL]
drop TABLE IF EXISTS "public".flashtest;

-- [DDL]
CREATE TABLE "public".flashtest (col1 INT,col2 TEXT) with(storage_type=ustore);

-- [DQL]
select int8in(xidout(next_csn)) from gs_get_next_xid_csn();

-- [DQL]
select now();

-- [DML_INSERT]
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- [DQL]
SELECT * FROM flashtest;

-- [DQL]
SELECT * FROM flashtest TIMECAPSULE CSN 79351682;

-- [DQL]
SELECT * FROM flashtest;

-- [DQL]
SELECT * FROM flashtest TIMECAPSULE TIMESTAMP '2023-09-13 19:35:26.011986';

-- [DQL]
SELECT * FROM flashtest;

-- [DQL]
SELECT * FROM flashtest TIMECAPSULE TIMESTAMP to_timestamp ('2023-09-13 19:35:26.011986', 'YYYY-MM-DD HH24:MI:SS.FF');

-- [DQL]
SELECT * FROM flashtest AS ft TIMECAPSULE CSN 79351682;

-- [DDL]
drop TABLE IF EXISTS "public".flashtest;


================================================================================
-- 来源: 4654_file_4654.txt
================================================================================

-- [DDL]
drop TABLE IF EXISTS "public".flashtest;

-- [DDL]
CREATE TABLE "public".flashtest (col1 INT,col2 TEXT) with(storage_type=ustore);

-- [DQL]
select int8in(xidout(next_csn)) from gs_get_next_xid_csn();

-- [DQL]
select now();

-- [DQL]
SELECT * FROM flashtest;

-- [DML_INSERT]
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- [DQL]
SELECT * FROM flashtest;

-- [OTHER]
TIMECAPSULE TABLE flashtest TO TIMESTAMP to_timestamp ('2023-09-13 19:52:21.551028', 'YYYY-MM-DD HH24:MI:SS.FF');

-- [DQL]
SELECT * FROM flashtest;

-- [DQL]
select now();

-- [DML_INSERT]
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- [DQL]
SELECT * FROM flashtest;

-- [OTHER]
TIMECAPSULE TABLE flashtest TO TIMESTAMP '2023-09-13 19:54:00.641506';

-- [DQL]
SELECT * FROM flashtest;

-- [DDL]
drop TABLE IF EXISTS "public".flashtest;


================================================================================
-- 来源: 4655_DROP_TRUNCATE.txt
================================================================================

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- [DML_INSERT]
insert into flashtest values(1, 'A');

-- [DQL]
select * from flashtest;

-- [DDL]
drop table if EXISTS flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DQL]
select * from flashtest;

-- [OTHER]
PURGE TABLE flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest;

-- [DDL]
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- [DDL]
create index flashtest_index on flashtest(id);

-- [DDL]
drop table if EXISTS flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [OTHER]
PURGE index flashtest_index;

-- [DQL]
select * from gs_recyclebin;

-- [OTHER]
PURGE RECYCLEBIN;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest;

-- [DDL]
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- [DML_INSERT]
insert into flashtest values(1, 'A');

-- [DQL]
select * from flashtest;

-- [DDL]
drop table if EXISTS flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DQL]
select * from flashtest;

-- [OTHER]
timecapsule table flashtest to before drop;

-- [DQL]
select * from flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest;

-- [DQL]
select * from flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [OTHER]
timecapsule table "BIN$31C14EB48DC$9B4E$0==$0" to before drop;

-- [DQL]
select * from gs_recyclebin;

-- [DQL]
select * from flashtest;

-- [DDL]
drop table if EXISTS flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DQL]
select * from flashtest;

-- [OTHER]
timecapsule table flashtest to before drop rename to flashtest_rename;

-- [DQL]
select * from flashtest;

-- [DQL]
select * from flashtest_rename;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest_rename;

-- [OTHER]
PURGE RECYCLEBIN;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest;

-- [DDL]
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- [DML_INSERT]
insert into flashtest values(1, 'A');

-- [DQL]
select * from flashtest;

-- [DML_TRUNCATE]
truncate table flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DQL]
select * from flashtest;

-- [OTHER]
timecapsule table flashtest to before truncate;

-- [DQL]
select * from flashtest;

-- [DQL]
select * from gs_recyclebin;

-- [DDL]
drop table if EXISTS flashtest;

-- [OTHER]
PURGE RECYCLEBIN;

-- [DQL]
select * from gs_recyclebin;


================================================================================
-- 来源: 4667_file_4667.txt
================================================================================

-- [DDL]
ALTER DATABASE SET ilm = on;

-- [OTHER]
List of relations Schema | Name | Type | Owner | Storage

-- [DQL]
SELECT a.oid, a.relname from pg_class a inner join pg_namespace b on a.relnamespace = b.oid WHERE (a.relname = 'gsilmpolicy_seq' OR a.relname = 'gsilmtask_seq') AND b.nspname = 'public';

-- [DDL]
CREATE TABLE ilm_table_1 (col1 int, col2 text) ilm add policy row store compress advanced row after 3 days of no modification on (col1 < 1000);

-- [DDL]
CREATE TABLE ilm_table_2 (col1 int, col2 text);

-- [DDL]
ALTER TABLE ilm_table_2 ilm add policy row store compress advanced row after 3 days of no modification;

-- [DQL]
SELECT * FROM gs_my_ilmpolicies;

-- [DQL]
SELECT * FROM gs_my_ilmdatamovementpolicies;

-- [DQL]
SELECT * FROM gs_my_ilmobjects;

-- [PLSQL]
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(11, 1);

-- [DML_INSERT]
INSERT INTO ilm_table_1 select *, 'test_data' FROM generate_series(1, 10000);

-- [PLSQL]
DECLARE v_taskid number;

-- [TCL]
BEGIN DBE_ILM.EXECUTE_ILM(OWNER => 'public', OBJECT_NAME => 'ilm_table_1', TASK_ID => v_taskid, SUBOBJECT_NAME => NULL, POLICY_NAME => 'ALL POLICIES', EXECUTION_MODE => 2);

-- [DQL]
SELECT * FROM gs_my_ilmtasks;

-- [DQL]
SELECT * FROM gs_my_ilmevaluationdetails;

-- [DQL]
SELECT * FROM gs_my_ilmresults;

-- [PLSQL]
DECLARE V_HOUR INT := 22;

-- [DQL]
SELECT * FROM gs_adm_ilmparameters;


================================================================================
-- 来源: 4669_TIPS.txt
================================================================================

-- [DDL]
DROP TABLE IF EXISTS ILM_TABLE;

-- [DDL]
CREATE TABLE ILM_TABLE(a int);

-- [DDL]
ALTER TABLE ILM_TABLE ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION;

-- [DQL]
SELECT * FROM gs_adm_ilmresults ORDER BY task_id desc;

-- [OTHER]
DBE_ILM.STOP_ILM (task_id => V_TASK, p_drop_running_Jobs => FALSE, p_Jobname => V_JOBNAME);

-- [DDL]
DROP TABLE IF EXISTS ILM_TABLE;

-- [DDL]
CREATE TABLE ILM_TABLE(a int);

-- [DDL]
ALTER TABLE ILM_TABLE ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION;

-- [PLSQL]
CALL DBE_ILM_ADMIN.DISABLE_ILM();

-- [PLSQL]
CALL DBE_ILM_ADMIN.ENABLE_ILM();

-- [PLSQL]
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(11， 1);

-- [PLSQL]
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(12, 10);

-- [PLSQL]
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(1, 1);

-- [PLSQL]
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(13, 512);

-- [OTHER]
DBE_COMPRESSION.GET_COMPRESSION_RATIO ( scratchtbsname IN VARCHAR2, ownname IN VARCHAR2, objname IN VARCHAR2, subobjname IN VARCHAR2, comptype IN NUMBER, blkcnt_cmp OUT PLS_INTEGER, blkcnt_uncmp OUT PLS_INTEGER, row_cmp OUT PLS_INTEGER, row_uncmp OUT PLS_INTEGER, cmp_ratio OUT NUMBER, comptype_str OUT VARCHAR2, sample_ratio IN INTEGER DEFAULT 20, objtype IN PLS_INTEGER DEFAULT OBJTYPE_TABLE);

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE user user1 IDENTIFIED BY '********' ;

-- [DDL]
CREATE user user2 IDENTIFIED BY '********' ;

-- [SESSION]
SET ROLE user1 PASSWORD '********' ;

-- [DDL]
CREATE TABLE TEST_DATA ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- [OTHER]
DBE_HEAT_MAP.ROW_HEAT_MAP( owner IN VARCHAR2, segment_name IN VARCHAR2, partition_name IN VARCHAR2 DEFAULT NULL, ctid IN VARCHAR2,);

-- [DDL]
ALTER DATABASE set ilm = on ;

-- [DDL]
CREATE Schema HEAT_MAP_DATA ;

-- [SESSION]
SET current_schema = HEAT_MAP_DATA ;

-- [DDL]
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1' ;

-- [DDL]
CREATE TABLE HEAT_MAP_DATA . heat_map_table ( id INT , value TEXT ) TABLESPACE example1 ;

-- [DML_INSERT]
INSERT INTO HEAT_MAP_DATA . heat_map_table VALUES ( 1 , 'test_data_row_1' );

-- [DQL]
SELECT * from DBE_HEAT_MAP . ROW_HEAT_MAP ( owner => 'heat_map_data' , segment_name => 'heat_map_table' , partition_name => NULL , ctid => '(0,1)' );

-- [DQL]
SELECT * FROM GS_ADM_ILMPARAMETERS;

-- [DQL]
SELECT * FROM GS_ADM_ILMPOLICIES;

-- [DQL]
SELECT * FROM GS_MY_ILMPOLICIES;

-- [DQL]
SELECT * FROM GS_ADM_ILMDATAMOVEMENTPOLICIES;

-- [DQL]
SELECT * FROM GS_MY_ILMDATAMOVEMENTPOLICIES;

-- [DQL]
SELECT * FROM GS_ADM_ILMOBJECTS;

-- [DQL]
SELECT * FROM GS_MY_ILMOBJECTS;

-- [DQL]
SELECT * FROM GS_ADM_ILMTASKS;

-- [DQL]
SELECT * FROM GS_MY_ILMTASKS;

-- [DQL]
SELECT * FROM GS_ADM_ILMEVALUATIONDETAILS;

-- [DQL]
SELECT * FROM GS_MY_ILMEVALUATIONDETAILS;

-- [DQL]
SELECT * FROM GS_ADM_ILMRESULTS;

-- [DQL]
SELECT * FROM GS_MY_ILMRESULTS;


================================================================================
-- 来源: 4693_query.txt
================================================================================

-- [DQL]
select "table", "column" from gs_index_advise('select name, age, sex from t1 where age >= 18 and age < 35 and sex = ' 'f ' ';

-- [DQL]
select "table", "column", "indextype" from gs_index_advise('select name, age, sex from range_table where age = 20;


================================================================================
-- 来源: 4694_file_4694.txt
================================================================================

-- [DQL]
select * from hypopg_create_index('create index on bmsql_customer(c_w_id)');

-- [SESSION]
set enable_hypo_index = on;

-- [EXPLAIN]
explain SELECT c_discount from bmsql_customer where c_w_id = 10;

-- [EXPLAIN]
explain SELECT c_discount from bmsql_customer where c_w_id = 10;

-- [DQL]
select * from hypopg_display_index();


================================================================================
-- 来源: 4733_DB4AI.txt
================================================================================

-- [DDL]
CREATE MODEL iris_classification_model USING xgboost_regression_logistic FEATURES sepal_length, sepal_width,petal_length,petal_width TARGET target_type < 2 FROM tb_iris_1 WITH nthread=4, max_depth=8;

-- [DQL]
select gs_explain_model('iris_classification_model');

-- [DQL]
SELECT id , PREDICT BY iris_classification (FEATURES sepal_length,sepal_width,petal_length,petal_width ) as " PREDICT" FROM tb_iris limit 3;

-- [EXPLAIN]
Explain CREATE MODEL patient_logisitic_regression USING logistic_regression FEATURES second_attack, treatment TARGET trait_anxiety > 50 FROM patients WITH batch_size=10, learning_rate = 0.05;

-- [DDL]
CREATE MODEL patient_linear_regression USING linear_regression FEATURES second_attack,treatment TARGET trait_anxiety FROM patients WITH optimizer='aa';

-- [DDL]
CREATE MODEL patient_linear_regression USING linear_regression FEATURES second_attack,treatment TARGET trait_anxiety FROM patients;

-- [DDL]
CREATE MODEL patient_linear_regression USING linear_regression FEATURES * TARGET trait_anxiety FROM patients;

-----------------------------------------------------------------------------------------------------------------------
-- [DDL]
CREATE MODEL patient_linear_regression USING linear_regression FEATURES second_attack,treatment TARGET * FROM patients;

-- [DDL]
CREATE MODEL patient_linear_regression USING linear_regression FEATURES second_attack,treatment FROM patients;

-- [DDL]
CREATE MODEL ecoli_svmc USING multiclass FEATURES f1, f2, f3, f4, f5, f6, f7 TARGET cat FROM (SELECT * FROM db4ai_ecoli WHERE cat='cp');

-- [DDL]
create model iris_classification_model using xgboost_regression_logistic features message_regular target error_level from error_code;

-- [DDL]
CREATE MODEL ecoli_svmc USING multiclass FEATURES f1, f2, f3, f4, f5, f6, f7, cat TARGET cat FROM db4ai_ecoli ;

-- [DQL]
select gs_explain_model("ecoli_svmc");

-- [DQL]
select id, PREDICT BY patient_logistic_regression (FEATURES second_attack,treatment) FROM patients;

-- [DQL]
select id, PREDICT BY patient_linear_regression (FEATURES second_attack) FROM patients;

-------------------------------------------------------------------------------------------------------------------------------------
-- [DQL]
select id, PREDICT BY patient_linear_regression (FEATURES 1,second_attack,treatment) FROM patients;


================================================================================
-- 来源: 5279_drop user.txt
================================================================================

-- [DDL]
drop user test1 cascade;

-- [DQL]
select d.datname,s.classid,s.objid from pg_roles r join pg_shdepend s on r.oid=s.refobjid join pg_database d on s.dbid=d.oid where rolname=' test1 ';

-- [DDL]
drop user test1 cascade;


================================================================================
-- 来源: 5295_file_5295.txt
================================================================================

-- [DQL]
select gs_create_log_tables();

-- [DDL]
alter foreign table gs_pg_log_ft options (set master_only 'false');

-- [DDL]
alter foreign table gs_profile_log_ft options (set latest_files '10');


================================================================================
-- 来源: 5778_gsql.txt
================================================================================

-- [OTHER]
\ set foo bar 要引用变量的值，在变量前面加冒号。例如查看变量的值： 1

-- [OTHER]
\ echo : foo bar 这种变量的引用方法适用于规则的SQL语句和除\copy、\ef、\help、\sf、\!以外的元命令。 gsql预定义了一些特殊变量，同时也规划了变量的取值。为了保证和后续版本最大限度地兼容，请避免以其他目的使用这些变量。所有特殊变量见 表2 。 所有特殊变量都由大写字母、数字和下划线组成。 要查看特殊变量的默认值，请使用元命令 \echo : varname （例如\echo :DBNAME）。 表2 特殊变量设置 变量 设置方法 变量说明 DBNAME \set DBNAME dbname 当前连接的数据库的名称。每次连接数据库时都会被重新设置。 ECHO \set ECHO all | queries 如果设置为all，只显示查询信息。等效于使用gsql连接数据库时指定-a参数。 如果设置为queries，显示命令行和查询信息。等效于使用gsql连接数据库时指定-e参数。 ECHO_HIDDEN \set ECHO_HIDDEN on | off | noexec 当使用元命令查询数据库信息（例如\dg）时，此变量的取值决定了查询的行为： 设置为on，先显示元命令实际调用的查询语句，然后显示查询结果。等效于使用gsql连接数据库时指定-E参数。 设置为off，则只显示查询结果。 设置为noexec，则只显示查询信息，不执行查询操作。 ENCODING \set ENCODING encoding 当前客户端的字符集编码。 FETCH_COUNT \set FETCH_COUNT variable 如果该变量的值为大于0的整数，假设为n，则执行SELECT语句时每次从结果集中取n行到缓存并显示到屏幕。 如果不设置此变量，或设置的值小于等于0，则执行SELECT语句时一次性把结果都取到缓存。 说明： 设置合理的变量值，将减少内存使用量。一般来说，设为100到1000之间的值比较合理。 HISTCONTROL \set HISTCONTROL ignorespace | ignoredups | ignoreboth | none ignorespace：以空格开始的行将不会写入历史列表。 ignoredups：与以前历史记录里匹配的行不会写入历史记录。 ignoreboth、none或者其他值：所有以交互模式读入的行都被保存到历史列表。 说明： none表示不设置HISTCONTROL。 HISTFILE \set HISTFILE filename 此文件用于存储历史名列表。缺省值是~/.bash_history。 HISTSIZE \set HISTSIZE size 保存在历史命令里命令的个数。缺省值是500。 HOST \set HOST hostname 已连接的数据库主机名称。 IGNOREEOF \set IGNOREEOF variable 若设置此变量为数值，假设为10，则在gsql中输入的前9次EOF字符（通常是Ctrl+C）都会被忽略，在第10次按Ctrl+C才能退出gsql程序。 若设置此变量为非数值，则缺省为10。 若删除此变量，则向交互的gsql会话发送一个EOF终止应用。 LASTOID \set LASTOID oid 最后影响的oid值，即为从一条INSERT或lo_import命令返回的值。此变量只保证在下一条SQL语句的结果显示之前有效。 ON_ERROR_ROLLBACK \set ON_ERROR_ROLLBACK on | interactive | off 如果是on，当一个事务块里的语句产生错误的时候，这个错误将被忽略而事务继续。 如果是interactive，这样的错误只是在交互的会话里忽略。 如果是off（缺省），事务块里一个语句生成的错误将会回滚整个事务。on_error_rollback-on模式是通过在一个事务块的每个命令前隐含地发出一个SAVEPOINT的方式来工作的，在发生错误的时候回滚到该事务块。 ON_ERROR_STOP \set ON_ERROR_STOP on | off on：命令执行错误时会立即停止，在交互模式下，gsql会立即返回已执行命令的结果。 off（缺省）：命令执行错误时将会跳过错误继续执行。 PORT \set PORT port 正连接数据库的端口号。 USER \set USER username 当前用于连接的数据库用户。 VERBOSITY \set VERBOSITY terse | default | verbose 这个选项可以设置为值terse、default、verbose之一以控制错误报告的冗余行。 terse：仅返回严重且主要的错误文本以及文本位置（一般适合于单行错误信息）。 default：返回严重且主要的错误文本及其位置，还包括详细的错误细节、错误提示（可能会跨越多行）。 verbose：返回所有的错误信息。 SQL代换 像元命令的参数一样，gsql变量的一个关键特性是可以把gsql变量替换成正规的SQL语句。此外，gsql还提供为变量更换新的别名或其他标识符等功能。使用SQL代换方式替换一个变量的值可在变量前加冒号。例如： 1 2 3 4 5 6 7 8

-- [OTHER]
\ set foo 'HR.areaS'

-- [DQL]
select * from : foo ;

-- [OTHER]
\ set PROMPT2 TEST

-- [DQL]
select * from HR . areaS TEST ;

-- [OTHER]
\ set PROMPT3 '>>>>'

-- [DML_COPY]
copy HR . areaS from STDIN ;


================================================================================
-- 来源: 5779_file_5779.txt
================================================================================

-- [OTHER]
\ l List of databases Name | Owner | Encoding | Collate | Ctype | Access privileges ----------------+----------+-----------+---------+-------+----------------------- human_resource | omm | SQL_ASCII | C | C | postgres | omm | SQL_ASCII | C | C | template0 | omm | SQL_ASCII | C | C | = c / omm + | | | | | omm = CTc / omm template1 | omm | SQL_ASCII | C | C | = c / omm + | | | | | omm = CTc / omm human_staff | omm | SQL_ASCII | C | C | ( 5 rows ) 更多gsql元命令请参见 元命令参考 。 示例 以把一个查询分成多行输入为例。注意提示符的变化： 1 2 3 4

-- [DDL]
CREATE TABLE HR . areaS ( gaussdb ( # area_ID NUMBER , gaussdb ( # area_NAME VARCHAR2 ( 25 ) gaussdb ( # ) tablespace EXAMPLE ;

-- [OTHER]
\ d HR . areaS Table "hr.areas" Column | Type | Modifiers -----------+-----------------------+----------- area_id | numeric | not null area_name | character varying ( 25 ) | 向HR.areaS表插入四行数据： 1 2 3 4 5 6 7

-- [DML_INSERT]
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 1 , 'Europe' );

-- [DML_INSERT]
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 2 , 'Americas' );

-- [DML_INSERT]
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 3 , 'Asia' );

-- [DML_INSERT]
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 4 , 'Middle East and Africa' );

-- [OTHER]
\ set PROMPT1 '%n@%m %~%R%#' omm @ [ local ]

-- [OTHER]
查看表： 1 2 3 4 5 6 7 8 omm @ [ local ]

-- [DQL]
SELECT * FROM HR . areaS ;

-- [OTHER]
\ pset border 2 Border style is 2 . omm @ [ local ]

-- [DQL]
SELECT * FROM HR . areaS ;

-- [OTHER]
\ pset border 0 Border style is 0 . omm @ [ local ]

-- [DQL]
SELECT * FROM HR . areaS ;

-- [OTHER]
\ a \ t \ x Output format is unaligned . Showing only tuples . Expanded display is on . omm @ [ local ]

-- [DQL]
SELECT * FROM HR . areaS ;

-- [OTHER]
父主题： gsql


================================================================================
-- 来源: 5780_file_5780.txt
================================================================================

-- [OTHER]
\ h Available help : ABORT ALTER AGGREGATE ALTER APP WORKLOAD GROUP ... ... 例如，查看CREATE DATABASE命令的参数可使用下面的命令： 1 2 3 4 5 6 7 8 9 10 11 12

-- [OTHER]
\ help CREATE DATABASE Command : CREATE DATABASE Description : create a new database Syntax : CREATE DATABASE database_name [ [ WITH ] { [ OWNER [ = ] user_name ] | [ TEMPLATE [ = ] template ] | [ ENCODING [ = ] encoding ] | [ LC_COLLATE [ = ] lc_collate ] | [ LC_CTYPE [ = ] lc_ctype ] | [ DBCOMPATIBILITY [ = ] compatibility_type ] | [ TABLESPACE [ = ] tablespace_name ] | [ CONNECTION LIMIT [ = ] connlimit ] } [...] ];

-- [OTHER]
\ ? General \ copyright show GaussDB Kernel usage and distribution terms \ g [ FILE ] or ;


================================================================================
-- 来源: 5782_file_5782.txt
================================================================================

-- [OTHER]
\ d \d[S+] NAME 列出指定表、视图和索引的结构。 - 假设存在表a，列出指定表a的结构。

-- [OTHER]
\ dtable + a \d+ [PATTERN] 列出所有表、视图和索引。 如果声明了PATTERN，只显示名称匹配PATTERN的表、视图和索引。 列出所有名称以f开头的表、视图和索引。

-- [OTHER]
\ d + f * \da[S] [PATTERN] 列出所有可用的聚集函数，以及它们操作的数据类型和返回值类型。 如果声明了PATTERN，只显示名称匹配PATTERN的聚集函数。 列出所有名称以f开头可用的聚集函数，以及它们操作的数据类型和返回值类型。

-- [OTHER]
\ da f * \db[+] [PATTERN] 列出所有可用的表空间。 如果声明了PATTERN，只显示名称匹配PATTERN的表空间。 列出所有名称以p开头的可用表空间。

-- [OTHER]
\ db p * \dc[S+] [PATTERN] 列出所有字符集之间的可用转换。 如果声明了PATTERN，只显示名称匹配PATTERN的转换。 列出所有字符集之间的可用转换。

-- [OTHER]
\ dc * \dC[+] [PATTERN] 列出所有类型转换。 PATTERN需要使用实际类型名，不能使用别名。 如果声明了PATTERN，只显示名称匹配PATTERN的转换。 列出所有名称以c开头的类型转换。

-- [OTHER]
\ dC c * \dd[S] [PATTERN] 显示所有匹配PATTERN的描述。 如果没有给出参数，则显示所有可视对象。“对象”包括：聚集、函数、操作符、类型、关系(表、视图、索引、序列、大对象)、规则。 列出所有可视对象。

-- [OTHER]
\ dd \ddp [PATTERN] 显示所有默认的使用权限。 如果指定了PATTERN，只显示名称匹配PATTERN的使用权限。 列出所有默认的使用权限。

-- [OTHER]
\ ddp \dD[S+] [PATTERN] 列出所有可用域。 如果声明了PATTERN，只显示名称匹配PATTERN的域。 列出所有可用域。

-- [OTHER]
\ dD \det[+] [PATTERN] 列出所有的外部表。 如果声明了PATTERN，只显示名称匹配PATTERN的表。 列出所有的外部表。

-- [OTHER]
\ det \des[+] [PATTERN] 列出所有的外部服务器。 如果声明了PATTERN，只显示名称匹配PATTERN的服务器。 列出所有的外部服务器。

-- [OTHER]
\ des \deu[+] [PATTERN] 列出用户映射信息。 如果声明了PATTERN，只显示名称匹配PATTERN的信息。 列出用户映射信息。

-- [OTHER]
\ deu \dew[+] [PATTERN] 列出封装的外部数据。 如果声明了PATTERN，只显示名称匹配PATTERN的数据。 列出封装的外部数据。

-- [OTHER]
\ dew \df[antw][S+] [PATTERN] 列出所有可用函数，以及它们的参数和返回的数据类型。a代表聚集函数，n代表普通函数，t代表触发器，w代表窗口函数。 如果声明了PATTERN，只显示名称匹配PATTERN的函数。 列出所有可用函数，以及它们的参数和返回的数据类型。

-- [OTHER]
\ df \dF[+] [PATTERN] 列出所有的文本搜索配置信息。 如果声明了PATTERN，只显示名称匹配PATTERN的配置信息。 列出所有的文本搜索配置信息。

-- [OTHER]
\ dF + \dFd[+] [PATTERN] 列出所有的文本搜索字典。 如果声明了PATTERN，只显示名称匹配PATTERN的字典。 列出所有的文本搜索字典。

-- [OTHER]
\ dFd \dFp[+] [PATTERN] 列出所有的文本搜索分析器。 如果声明了PATTERN，只显示名称匹配PATTERN的分析器。 列出所有的文本搜索分析器。

-- [OTHER]
\ dFp \dFt[+] [PATTERN] 列出所有的文本搜索模板。 如果声明了PATTERN，只显示名称匹配PATTERN的模板。 列出所有的文本搜索模板。

-- [OTHER]
\ dFt \dg[+] [PATTERN] 列出所有数据库角色。 说明： 因为用户和群组的概念被统一为角色，所以这个命令等价于\du。为了和以前兼容，所以保留两个命令。 如果指定了PATTERN，只显示名称匹配PATTERN的角色。 列出名称为“j?e”所有数据库角色（“?”表示任一字符）。

-- [OTHER]
\ dg j ? e \dl \lo_list的别名，显示一个大对象的列表。 - 列出所有的大对象。

-- [OTHER]
\ dl \dL[S+] [PATTERN] 列出可用的程序语言。 如果指定了PATTERN，只列出名称匹配PATTERN的语言。 列出可用的程序语言。

-- [OTHER]
\ dL \dm[S+] [PATTERN] 列出物化视图。 如果指定了PATTERN，只列出名称匹配PATTERN的物化视图。 列出物化视图。

-- [OTHER]
\ dm \dn[S+] [PATTERN] 列出所有模式（名称空间）。如果向命令追加+，会列出每个模式相关的权限及描述。 如果声明了PATTERN，只列出名称匹配PATTERN的模式名。缺省时，只列出用户创建的模式。 列出所有名称以d开头的模式以及相关信息。

-- [OTHER]
\ dn + d * \do[S] [PATTERN] 列出所有可用的操作符，以及它们的操作数和返回的数据类型。 如果声明了PATTERN，只列出名称匹配PATTERN的操作符。缺省时，只列出用户创建的操作符。 列出所有可用的操作符，以及它们的操作数和返回的数据类型。

-- [OTHER]
\ do \dO[S+] [PATTERN] 列出排序规则。 如果声明了PATTERN，只列出名称匹配PATTERN的规则。缺省时，只列出用户创建的规则。 列出排序规则。

-- [OTHER]
\ dO \dp [PATTERN] 列出一列可用的表、视图以及相关的权限信息。 \dp显示结果如下： rolename=xxxx/yyyy --赋予一个角色的权限 =xxxx/yyyy --赋予public的权限 xxxx表示赋予的权限，yyyy表示授予这个权限的角色。权限的参数说明请参见 表5 。 如果指定了PATTERN，只列出名称匹配PATTERN的表、视图。 列出一列可用的表、视图以及相关的权限信息。

-- [OTHER]
\ dp \drds [PATTERN1 [PATTERN2]] 列出所有修改过的配置参数。这些设置可以是针对角色的、针对数据库的或者同时针对两者的。PATTERN1和PATTERN2表示要列出的角色PATTERN和数据库PATTERN。 如果声明了PATTERN，只列出名称匹配PATTERN的规则。缺省或指定*时，则会列出所有设置。 列出数据库所有修改过的配置参数。

-- [OTHER]
\ drds * dbname \dT[S+] [PATTERN] 列出所有的数据类型。 如果指定了PATTERN，只列出名称匹配PATTERN的类型。 列出所有的数据类型。

-- [OTHER]
\ dT \du[+] [PATTERN] 列出所有数据库角色。 说明： 因为用户和群组的概念被统一为角色，所以这个命令等价于\dg。为了和以前兼容，所以保留两个命令。 如果指定了PATTERN，则只列出名称匹配PATTERN的角色。 列出所有数据库角色。

-- [OTHER]
\ du \dE[S+] [PATTERN] \di[S+] [PATTERN] \ds[S+] [PATTERN] \dt[S+] [PATTERN] \dv[S+] [PATTERN] 这一组命令，字母E，i，s，t和v分别代表着外部表，索引，序列，表和视图。可以以任意顺序指定其中一个或者它们的组合来列出这些对象。例如：\dit列出所有的索引和表。在命令名称后面追加+，则每一个对象的物理尺寸以及相关的描述也会被列出。 如果指定了PATTERN，只列出名称匹配该PATTERN的对象。默认情况下只会显示用户创建的对象。通过PATTERN或者S修饰符可以把系统对象包括在内。 列出所有的索引和视图。

-- [OTHER]
\ div \dx[+] [PATTERN] 列出安装数据库的扩展信息。 如果指定了PATTERN，则只列出名称匹配PATTERN的扩展信息。 列出安装数据库的扩展信息。

-- [OTHER]
\ dx \l[+] 列出服务器上所有数据库的名称、所有者、字符集编码以及使用权限。 - 列出服务器上所有数据库的名称、所有者、字符集编码以及使用权限。

-- [OTHER]
\ l \sf[+] FUNCNAME 显示函数的定义。 说明： 对于带圆括号的函数名，需要在函数名两端添加双引号，并且在双引号后面加上参数类型列表。参数类型列表两端添加圆括号。 - 假设存在函数function_a和函数名带圆括号的函数func()name，列出函数的定义。 1 2

-- [OTHER]
\ sf function_a

-- [OTHER]
\ sf "func()name" ( argtype1 , argtype2 ) \z [PATTERN] 列出数据库中所有表、视图和序列，以及它们相关的访问特权。 如果给出任何pattern ，则被当成一个正则表达式，只显示匹配的表、视图、序列。 列出数据库中所有表、视图和序列，以及它们相关的访问特权。

-- [OTHER]
\ z 表5 权限的参数说明 参数 参数说明 r SELECT：允许对指定的表、视图读取数据。 w UPDATE：允许对指定表更新字段。 a INSERT：允许对指定表插入数据。 d DELETE：允许删除指定表中的数据。 D TRUNCATE：允许清理指定表中的数据。 x REFERENCES：允许创建外键约束，分布式场景暂不支持。 t TRIGGER：允许在指定表上创建触发器。 X EXECUTE：允许使用指定的函数，以及利用这些函数实现的操作符。 U USAGE： 对于过程语言，允许用户在创建函数时，指定过程语言。 对于模式，允许访问包含在指定模式中的对象。 对于序列，允许使用nextval函数。 C CREATE： 对于数据库，允许在该数据库里创建新的模式。 对于模式，允许在该模式中创建新的对象。 对于表空间，允许在其中创建表，以及允许创建数据库和模式的时候把该表空间指定为其缺省表空间。 c CONNECT：允许用户连接到指定的数据库。 T TEMPORARY：允许创建临时表。 A ALTER：允许用户修改指定对象的属性。 P DROP：允许用户删除指定的对象。 m COMMENT：允许用户定义或修改指定对象的注释。 i INDEX：允许用户在指定表上创建索引。 v VACUUM：允许用户对指定的表执行ANALYZE和VACUUM操作。 * 给前面权限的授权选项。 表6 格式化元命令 参数 参数说明 \a 对齐模式和非对齐模式之间的切换。 \C [STRING] 把正在打印的表的标题设置为一个查询的结果或者取消这样的设置。 \f [STRING] 对于不对齐的查询输出，显示或者设置域分隔符。 \H 若当前模式为文本格式，则切换为HTML输出格式。 若当前模式为HTML格式，则切换回文本格式。 \pset NAME [VALUE] 设置影响查询结果表输出的选项。NAME的取值见 表7 。 \t [on|off] 切换输出的字段名的信息和行计数脚注。 \T [STRING] 指定在使用HTML输出格式时放在table标签里的属性。如果参数为空，不设置。 \x [on|off|auto] 切换扩展行格式。 表7 可调节的打印选项 选项 选项说明 取值范围 border value必须是一个数字。通常这个数字越大，表的边界就越宽线就越多，但是这个取决于特定的格式。 在HTML格式下，取值范围为大于0的整数。 在其他格式下，取值范围： 0：无边框 1：内部分隔线 2：台架 expanded (或x) 在正常和扩展格式之间切换。 当打开扩展格式时，查询结果用两列显示，字段名称在左、数据在右。这个模式在数据无法放进通常的“水平”模式的屏幕时很有用。 在正常格式下，当查询输出的格式比屏幕宽时，用扩展格式。正常格式只对aligned和wrapped格式有用。 fieldsep 声明域分隔符来实现非对齐输出。这样就可以创建其他程序希望的制表符或逗号分隔的输出。要设置制表符域分隔符，键入\pset fieldsep '\t'。缺省域分隔符是'|'(竖条符)。 - fieldsep_zero 声明域分隔符来实现非对齐输出到零字节。 - footer 用来切换脚注。 - format 设置输出格式。允许使用唯一缩写（这意味着一个字母就够了）。 取值范围： unaligned：写一行的所有列在一条直线上中，当前活动字段分隔符分隔。 aligned：此格式是标准的，可读性好的文本输出。 wrapped：类似aligned，但是包装跨行的宽数据值，使其适应目标字段的宽度输出。 html：把表输出为可用于文档里的对应标记语言。输出不是完整的文档。 latex：把表输出为可用于文档里的对应标记语言。输出不是完整的文档。 troff-ms：把表输出为可用于文档里的对应标记语言。输出不是完整的文档。 null 打印一个字符串，用来代替一个null值。 缺省是什么都不打印，这样很容易和空字符串混淆。 numericlocale 切换分隔小数点左边的数值的区域相关的分组符号。 on：显示指定的分隔符。 off：不显示分隔符。 忽略此参数，显示默认的分隔符。 pager 控制查询和gsql帮助输出的分页器。如果设置了环境变量PAGER，输出将被指向到指定程序，否则使用系统缺省。 on：当输出到终端且不适合屏幕显示时，使用分页器。 off：不使用分页器。 always：当输出到终端无论是否符合屏幕显示时，都使用分页器。 recordsep 声明在非对齐输出格式时的记录分隔符。 - recordsep_zero 声明在非对齐输出到零字节时的记录分隔符。 - tableattr（或T） 声明放在html输出格式中HTML table标签的属性（例如：cellpadding或bgcolor）。注意：这里可能不需要声明border，因为已经在\pset border里用过了。如果没有给出value，则不设置表的属性。 - title 为随后打印的表设置标题。这个可以用于给输出一个描述性标签。如果没有给出value，不设置标题。 - tuples_only（或者t） 在完全显示和只显示实际的表数据之间切换。完全显示将输出像列头、标题、各种脚注等信息。在tuples_only模式下，只显示实际的表数据。 - feedback 切换是否输出结果行数。 - 表8 连接元命令 参数 参数说明 取值范围 \c[onnect] [DBNAME|- USER|- HOST|- PORT|-] 连接到一个新的数据库。当数据库名称长度超过63个字节时，默认前63个字节有效，连接到前63个字节对应的数据库，但是gsql的命令提示符中显示的数据库对象名仍为截断前的名称。 说明： 重新建立连接时，如果切换数据库登录用户，将可能会出现交互式输入，要求输入新用户的连接密码。该密码最长长度为999字节，受限于GUC参数password_max_length的最大值。 - \encoding [ENCODING] 设置客户端字符编码格式。 不带参数时，显示当前的编码格式。 \conninfo 输出当前连接的数据库的信息。 - 表9 操作系统元命令 参数 参数说明 取值范围 \cd [DIR] 切换当前的工作目录。 绝对路径或相对路径，且满足操作系统路径命名规则。 \setenv NAME [VALUE] 设置环境变量NAME为VALUE，如果没有给出VALUE值，则不设置环境变量。 - \timing [on|off] 以毫秒为单位显示每条SQL语句的执行时间（不包括屏显打印时间）。 on表示打开显示。 off表示关闭显示。 \! [COMMAND] 返回到一个单独的Unix shell或者执行Unix命令COMMAND。 - 表10 变量元命令 参数 参数说明 \prompt [TEXT] NAME 提示用户用文本格式来指定变量名称。 \set [NAME [VALUE]] 设置内部变量NAME为VALUE或者如果给出了多于一个值，设置为所有这些值的连接结果。如果没有给出第二个参数，只设变量不设值。 有一些常用变量被gsql特殊对待，它们是一些选项设置，通常所有特殊对待的变量都是由大写字母组成(可能还有数字和下划线)。 表11 是一个所有特殊对待的变量列表。 \unset NAME 不设置（或删除）gsql变量名。 表11 \set常用命令 名称 命令说明 取值范围 \set VERBOSITY value 这个选项可以设置为值default，verbose，terse之一以控制错误报告的冗余行。 value取值范围：default，verbose，terse \set ON_ERROR_STOP value 如果设置了这个变量，脚本处理将马上停止。如果该脚本是从另外一个脚本调用的，那个脚本也会按同样的方式停止。如果最外层的脚本不是从一次交互的gsql会话中调用的而是用-f选项调用的，gsql将返回错误代码3，以示这个情况与致命错误条件的区别（错误代码为1）。 value取值范围为：on/off，true/false，yes/no，1/0 \set AUTOCOMMIT [on|off] 设置当前gsql连接的自动提交行为，on为打开自动提交，off为关闭自动提交。默认情况下，gsql连接处于自动提交模式，每个单独的语句都被隐式提交。如果基于性能或者其它方面考虑，需要关闭自动提交时，需要用户自己显示的发出COMMIT命令来保证事务的提交。例如，在指定的业务SQL执行完之后发送COMMIT语句显式提交，特别是gsql客户端退出之前务必保证所有的事务已经提交。 说明： gsql默认使用自动提交模式，若关闭自动提交，将会导致后面执行的语句都受到隐式事务包裹，数据库中不支持在事务中执行的语句不能在此模式下执行。 on表示打开自动提交。 off表示关闭自动提交。 表12 大对象元命令 参数 参数说明 \lo_list 显示一个目前存储在该数据库里的所有 GaussDB 大对象和提供给他们的注释。 表13 全密态元命令 参数 参数说明 \send_token 全密态功能，传输密钥到服务端缓存，只在开启内存解密逃生通道的情况下使用。 \st 全密态功能，传输密钥到服务端缓存，只在开启内存解密逃生通道的情况下使用。 \clear_token 全密态功能，销毁服务端缓存的密钥，只在开启内存解密逃生通道的情况下使用。 \ct 全密态功能，销毁服务端缓存的密钥，只在开启内存解密逃生通道的情况下使用。 \key_info KEY_INFO 在全密态数据库特性中，用于设置用于访问外部密钥管理者的参数。 分布式暂不支持 全密态内存解密逃生通道 。 PATTERN 很多\d命令都可以用一个PATTERN参数来指定要被显示的对象名称。在最简单的情况下，PATTERN正好就是该对象的准确名称。在PATTERN中的字符通常会被变成小写形式（就像在SQL名称中那样），例如\dt FOO将会显示名为foo的表。就像在SQL名称中那样，把PATTERN放在双引号中可以阻止它被转换成小写形式。如果需要在一个PATTERN中包括一个真正的双引号字符，则需要把它写成两个相邻的双引号，这同样是符合SQL引用标识符的规则。例如，\dt "FOO""BAR"将显示名为FOO"BAR（不是foo"bar）的表。和普通的SQL名称规则不同，不能只在PATTERN的一部分周围放上双引号，例如\dt FOO"FOO"BAR将会显示名为fooFOObar的表。 不使用PATTERN参数时，\d命令会显示当前schema搜索路径中可见的全部对象——这等价于用*作为PATTERN。所谓对象可见是指可以直接用名称引用该对象，而不需要用schema来进行限定。要查看数据库中所有的对象而不管它们的可见性，可以把*.*用作PATTERN。 如果放在一个PATTERN中，*将匹配任意字符序列（包括空序列），而?会匹配任意的单个字符（这种记号方法就像 Unix shell 的文件名PATTERN一样）。例如，\dt int*会显示名称以int开始的表。但是如果被放在双引号内，*和?就会失去这些特殊含义而变成普通的字符。 包含一个点号（.）的PATTERN被解释为一个schema名称模式后面跟上一个对象名称模式。例如，\dt foo*.*bar*会显示名称以foo开始的schema中所有名称包括bar的表。如果没有出现点号，那么模式将只匹配当前schema搜索路径中可见的对象。同样，双引号内的点号会失去其特殊含义并且变成普通的字符。 高级用户可以使用字符类等正则表达式记法，如[0-9]可以匹配任意数字。所有的正则表达式特殊字符都按照POSIX正则表达式所说的工作。以下字符除外： .会按照上面所说的作为一种分隔符。 *会被翻译成正则表达式记号.*。 ?会被翻译成.。 $则按字面意思匹配。 根据需要，可以通过书写?、( R +|)、( R |)和 R ?来分别模拟PATTERN字符.、 R *和 R ?。$不需要作为一个正则表达式字符，因为PATTERN必须匹配整个名称，而不是像正则表达式的常规用法那样解释（换句话说，$会被自动地追加到PATTERN上）。如果不希望该PATTERN的匹配位置被固定，可以在开头或者结尾写上*。注意在双引号内，所有的正则表达式特殊字符会失去其特殊含义并且按照其字面意思进行匹配。另外，在操作符名称PATTERN中（即\do的PATTERN参数），正则表达式特殊字符也按照字面意思进行匹配。


================================================================================
-- 来源: 5824_gs_expand.txt
================================================================================

-------------------------------------------------- 每个表的重分布执行时间（redis_progress_detail）： 由于该表由重分布线程创建记录，当重分布异常退出或者session连接异常时可能导致记录的时间不准确，只能作为参考，需要获取准确时间需要通过日志进行读取； 当用户表在pgxc_redistb中的redistributed字段为'y'时，用户再修改表名，该表中的table_name不会再进行更新。
-- [DQL]
select * from redis_progress_detail;


================================================================================
-- 来源: 5839_gs_rescue.txt
================================================================================

-- [DDL]
create table original(col1 integer);

-- [DML_COPY]
copy original from '/data2/file01';

-- [DDL]
create table amend(col1 integer,col2 integer default 0);

-- [DML_COPY]
copy amend from '/data2/file02';

-- [DML_INSERT]
insert into amend(col1) select * from original;


================================================================================
-- 来源: 5891_gsql.txt
================================================================================

-- [OTHER]
\ set foo bar 要引用变量的值，在变量前面加冒号。例如查看变量的值： 1

-- [OTHER]
\ echo : foo bar 这种变量的引用方法适用于规则的SQL语句和除\copy、\ef、\help、\sf、\!以外的元命令。 gsql预定义了一些特殊变量，同时也规划了变量的取值。为了保证和后续版本最大限度地兼容，请避免以其他目的使用这些变量。所有特殊变量见 表2 。 所有特殊变量都由大写字母、数字和下划线组成。 要查看特殊变量的默认值，请使用元命令 \echo : varname （例如\echo :DBNAME）。 表2 特殊变量设置 变量 设置方法 变量说明 DBNAME \set DBNAME dbname 当前连接的数据库的名称。每次连接数据库时都会被重新设置。 ECHO \set ECHO all | queries 如果设置为all，只显示查询信息。等效于使用gsql连接数据库时指定-a参数。 如果设置为queries，显示命令行和查询信息。等效于使用gsql连接数据库时指定-e参数。 ECHO_HIDDEN \set ECHO_HIDDEN on | off | noexec 当使用元命令查询数据库信息（例如\dg）时，此变量的取值决定了查询的行为： 设置为on，先显示元命令实际调用的查询语句，然后显示查询结果。等效于使用gsql连接数据库时指定-E参数。 设置为off，则只显示查询结果。 设置为noexec，则只显示查询信息，不执行查询操作。 ENCODING \set ENCODING encoding 当前客户端的字符集编码。 FETCH_COUNT \set FETCH_COUNT variable 如果该变量的值为大于0的整数，假设为n，则执行SELECT语句时每次从结果集中取n行到缓存并显示到屏幕。 如果不设置此变量，或设置的值小于等于0，则执行SELECT语句时一次性把结果都取到缓存。 说明： 设置合理的变量值，将减少内存使用量。一般来说，设为100到1000之间的值比较合理。 HISTCONTROL \set HISTCONTROL ignorespace | ignoredups | ignoreboth | none ignorespace：以空格开始的行将不会写入历史列表。 ignoredups：与以前历史记录里匹配的行不会写入历史记录。 ignoreboth、none或者其他值：所有以交互模式读入的行都被保存到历史列表。 说明： none表示不设置HISTCONTROL。 HISTFILE \set HISTFILE filename 此文件用于存储历史名列表。缺省值是~/.bash_history。 HISTSIZE \set HISTSIZE size 保存在历史命令里命令的个数。缺省值是500。 HOST \set HOST hostname 已连接的数据库主机名称。 IGNOREEOF \set IGNOREEOF variable 若设置此变量为数值，假设为10，则在gsql中输入的前9次EOF字符（通常是Ctrl+C）都会被忽略，在第10次按Ctrl+C才能退出gsql程序。 若设置此变量为非数值，则缺省为10。 若删除此变量，则向交互的gsql会话发送一个EOF终止应用。 LASTOID \set LASTOID oid 最后影响的oid值，即为从一条INSERT或lo_import命令返回的值。此变量只保证在下一条SQL语句的结果显示之前有效。 ON_ERROR_ROLLBACK \set ON_ERROR_ROLLBACK on | interactive | off 如果是on，当一个事务块里的语句产生错误的时候，这个错误将被忽略而事务继续。 如果是interactive，这样的错误只是在交互的会话里忽略。 如果是off（缺省），事务块里一个语句生成的错误将会回滚整个事务。on_error_rollback-on模式是通过在一个事务块的每个命令前隐含地发出一个SAVEPOINT的方式来工作的，在发生错误的时候回滚到该事务块。 ON_ERROR_STOP \set ON_ERROR_STOP on | off on：命令执行错误时会立即停止，在交互模式下，gsql会立即返回已执行命令的结果。 off（缺省）：命令执行错误时将会跳过错误继续执行。 PORT \set PORT port 正连接数据库的端口号。 USER \set USER username 当前用于连接的数据库用户。 VERBOSITY \set VERBOSITY terse | default | verbose 这个选项可以设置为值terse、default、verbose之一以控制错误报告的冗余行。 terse：仅返回严重且主要的错误文本以及文本位置（一般适合于单行错误信息）。 default：返回严重且主要的错误文本及其位置，还包括详细的错误细节、错误提示（可能会跨越多行）。 verbose：返回所有的错误信息。 SQL代换 像元命令的参数一样，gsql变量的一个关键特性是可以把gsql变量替换成正规的SQL语句。此外，gsql还提供为变量更换新的别名或其他标识符等功能。使用SQL代换方式替换一个变量的值可在变量前加冒号。例如： 1 2 3 4 5 6 7 8

-- [OTHER]
\ set foo 'HR.areaS'

-- [DQL]
select * from : foo ;

-- [OTHER]
\ set PROMPT2 TEST

-- [DQL]
select * from HR . areaS TEST ;

-- [OTHER]
\ set PROMPT3 '>>>>'

-- [DML_COPY]
copy HR . areaS from STDIN ;


================================================================================
-- 来源: 5892_file_5892.txt
================================================================================

-- [OTHER]
\ l List of databases Name | Owner | Encoding | Collate | Ctype | Access privileges ----------------+----------+-----------+---------+-------+----------------------- human_resource | omm | SQL_ASCII | C | C | postgres | omm | SQL_ASCII | C | C | template0 | omm | SQL_ASCII | C | C | = c / omm + | | | | | omm = CTc / omm template1 | omm | SQL_ASCII | C | C | = c / omm + | | | | | omm = CTc / omm human_staff | omm | SQL_ASCII | C | C | ( 5 rows ) 更多gsql元命令请参见 元命令参考 。 示例 以把一个查询分成多行输入为例。注意提示符的变化： 1 2 3 4

-- [DDL]
CREATE TABLE HR . areaS ( gaussdb ( # area_ID NUMBER , gaussdb ( # area_NAME VARCHAR2 ( 25 ) gaussdb -# ) tablespace EXAMPLE ;

-- [OTHER]
\ d HR . areaS Table "hr.areas" Column | Type | Modifiers -----------+-----------------------+----------- area_id | numeric | not null area_name | character varying ( 25 ) | 向HR.areaS表插入四行数据： 1 2 3 4 5 6 7

-- [DML_INSERT]
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 1 , 'Europe' );

-- [DML_INSERT]
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 2 , 'Americas' );

-- [DML_INSERT]
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 3 , 'Asia' );

-- [DML_INSERT]
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 4 , 'Middle East and Africa' );

-- [OTHER]
\ set PROMPT1 '%n@%m %~%R%#' omm @ [ local ]

-- [OTHER]
查看表： 1 2 3 4 5 6 7 8 omm @ [ local ]

-- [DQL]
SELECT * FROM HR . areaS ;

-- [OTHER]
\ pset border 2 Border style is 2 . omm @ [ local ]

-- [DQL]
SELECT * FROM HR . areaS ;

-- [OTHER]
\ pset border 0 Border style is 0 . omm @ [ local ]

-- [DQL]
SELECT * FROM HR . areaS ;

-- [OTHER]
\ a \ t \ x Output format is unaligned . Showing only tuples . Expanded display is on . omm @ [ local ]

-- [DQL]
SELECT * FROM HR . areaS ;

-- [OTHER]
父主题： gsql


================================================================================
-- 来源: 5893_file_5893.txt
================================================================================

-- [OTHER]
\ h Available help : ABORT ALTER AGGREGATE ALTER APP WORKLOAD GROUP ... ... 例如，查看CREATE DATABASE命令的参数可使用下面的命令： 1 2 3 4 5 6 7 8 9 10 11 12

-- [OTHER]
\ help CREATE DATABASE Command : CREATE DATABASE Description : create a new database Syntax : CREATE DATABASE database_name [ [ WITH ] { [ OWNER [ = ] user_name ] | [ TEMPLATE [ = ] template ] | [ ENCODING [ = ] encoding ] | [ LC_COLLATE [ = ] lc_collate ] | [ LC_CTYPE [ = ] lc_ctype ] | [ DBCOMPATIBILITY [ = ] compatibility_type ] | [ TABLESPACE [ = ] tablespace_name ] | [ CONNECTION LIMIT [ = ] connlimit ] } [...] ];

-- [OTHER]
\ ? General \ copyright show GaussDB Kernel usage and distribution terms \ g [ FILE ] or ;


================================================================================
-- 来源: 5895_file_5895.txt
================================================================================

-- [OTHER]
\ d \d[S+] NAME 列出指定表、视图和索引的结构。 - 假设存在表a，列出指定表a的结构。

-- [OTHER]
\ dtable + a \d+ [PATTERN] 列出所有表、视图和索引。 如果声明了PATTERN，只显示名称匹配PATTERN的表、视图和索引。 列出所有名称以f开头的表、视图和索引。

-- [OTHER]
\ d + f * \da[S] [PATTERN] 列出所有可用的聚集函数，以及它们操作的数据类型和返回值类型。 如果声明了PATTERN，只显示名称匹配PATTERN的聚集函数。 列出所有名称以f开头可用的聚集函数，以及它们操作的数据类型和返回值类型。

-- [OTHER]
\ da f * \db[+] [PATTERN] 列出所有可用的表空间。 如果声明了PATTERN，只显示名称匹配PATTERN的表空间。 列出所有名称以p开头的可用表空间。

-- [OTHER]
\ db p * \dc[S+] [PATTERN] 列出所有字符集之间的可用转换。 如果声明了PATTERN，只显示名称匹配PATTERN的转换。 列出所有字符集之间的可用转换。

-- [OTHER]
\ dc * \dC[+] [PATTERN] 列出所有类型转换。 PATTERN需要使用实际类型名，不能使用别名。 如果声明了PATTERN，只显示名称匹配PATTERN的转换。 列出所有名称以c开头的类型转换。

-- [OTHER]
\ dC c * \dd[S] [PATTERN] 显示所有匹配PATTERN的描述。 如果没有给出参数，则显示所有可视对象。“对象”包括：聚集、函数、操作符、类型、关系(表、视图、索引、序列、大对象)、规则。 列出所有可视对象。

-- [OTHER]
\ dd \ddp [PATTERN] 显示所有默认的使用权限。 如果指定了PATTERN，只显示名称匹配PATTERN的使用权限。 列出所有默认的使用权限。

-- [OTHER]
\ ddp \dD[S+] [PATTERN] 列出所有可用域。 如果声明了PATTERN，只显示名称匹配PATTERN的域。 列出所有可用域。

-- [OTHER]
\ dD \det[+] [PATTERN] 列出所有的外部表。 如果声明了PATTERN，只显示名称匹配PATTERN的表。 列出所有的外部表。

-- [OTHER]
\ det \des[+] [PATTERN] 列出所有的外部服务器。 如果声明了PATTERN，只显示名称匹配PATTERN的服务器。 列出所有的外部服务器。

-- [OTHER]
\ des \deu[+] [PATTERN] 列出用户映射信息。 如果声明了PATTERN，只显示名称匹配PATTERN的信息。 列出用户映射信息。

-- [OTHER]
\ deu \dew[+] [PATTERN] 列出封装的外部数据。 如果声明了PATTERN，只显示名称匹配PATTERN的数据。 列出封装的外部数据。

-- [OTHER]
\ dew \df[antw][S+] [PATTERN] 列出所有可用函数，以及它们的参数和返回的数据类型。a代表聚集函数，n代表普通函数，t代表触发器，w代表窗口函数。 如果声明了PATTERN，只显示名称匹配PATTERN的函数。 列出所有可用函数，以及它们的参数和返回的数据类型。

-- [OTHER]
\ df \dF[+] [PATTERN] 列出所有的文本搜索配置信息。 如果声明了PATTERN，只显示名称匹配PATTERN的配置信息。 列出所有的文本搜索配置信息。

-- [OTHER]
\ dF + \dFd[+] [PATTERN] 列出所有的文本搜索字典。 如果声明了PATTERN，只显示名称匹配PATTERN的字典。 列出所有的文本搜索字典。

-- [OTHER]
\ dFd \dFp[+] [PATTERN] 列出所有的文本搜索分析器。 如果声明了PATTERN，只显示名称匹配PATTERN的分析器。 列出所有的文本搜索分析器。

-- [OTHER]
\ dFp \dFt[+] [PATTERN] 列出所有的文本搜索模板。 如果声明了PATTERN，只显示名称匹配PATTERN的模板。 列出所有的文本搜索模板。

-- [OTHER]
\ dFt \dg[+] [PATTERN] 列出所有数据库角色。 说明： 因为用户和群组的概念被统一为角色，所以这个命令等价于\du。为了和以前兼容，所以保留两个命令。 如果指定了PATTERN，只显示名称匹配PATTERN的角色。 列出名称为“j?e”所有数据库角色（“?”表示任一字符）。

-- [OTHER]
\ dg j ? e \dl \lo_list的别名，显示一个大对象的列表。 - 列出所有的大对象。

-- [OTHER]
\ dl \dL[S+] [PATTERN] 列出可用的程序语言。 如果指定了PATTERN，只列出名称匹配PATTERN的语言。 列出可用的程序语言。

-- [OTHER]
\ dL \dm[S+] [PATTERN] 列出物化视图。 如果指定了PATTERN，只列出名称匹配PATTERN的物化视图。 列出物化视图。

-- [OTHER]
\ dm \dn[S+] [PATTERN] 列出所有模式（名称空间）。如果向命令追加+，会列出每个模式相关的权限及描述。 如果声明了PATTERN，只列出名称匹配PATTERN的模式名。缺省时，只列出用户创建的模式。 列出所有名称以d开头的模式以及相关信息。

-- [OTHER]
\ dn + d * \do[S] [PATTERN] 列出所有可用的操作符，以及它们的操作数和返回的数据类型。 如果声明了PATTERN，只列出名称匹配PATTERN的操作符。缺省时，只列出用户创建的操作符。 列出所有可用的操作符，以及它们的操作数和返回的数据类型。

-- [OTHER]
\ do \dO[S+] [PATTERN] 列出排序规则。 如果声明了PATTERN，只列出名称匹配PATTERN的规则。缺省时，只列出用户创建的规则。 列出排序规则。

-- [OTHER]
\ dO \dp [PATTERN] 列出一列可用的表、视图以及相关的权限信息。 \dp显示结果如下： rolename=xxxx/yyyy --赋予一个角色的权限 =xxxx/yyyy --赋予public的权限 xxxx表示赋予的权限，yyyy表示授予这个权限的角色。权限的参数说明请参见 表5 。 如果指定了PATTERN，只列出名称匹配PATTERN的表、视图。 列出一列可用的表、视图以及相关的权限信息。

-- [OTHER]
\ dp \drds [PATTERN1 [PATTERN2]] 列出所有修改过的配置参数。这些设置可以是针对角色的、针对数据库的或者同时针对两者的。PATTERN1和PATTERN2表示要列出的角色PATTERN和数据库PATTERN。 如果声明了PATTERN，只列出名称匹配PATTERN的规则。缺省或指定*时，则会列出所有设置。 列出数据库所有修改过的配置参数。

-- [OTHER]
\ drds * dbname \dT[S+] [PATTERN] 列出所有的数据类型。 如果指定了PATTERN，只列出名称匹配PATTERN的类型。 列出所有的数据类型。

-- [OTHER]
\ dT \du[+] [PATTERN] 列出所有数据库角色。 说明： 因为用户和群组的概念被统一为角色，所以这个命令等价于\dg。为了和以前兼容，所以保留两个命令。 如果指定了PATTERN，则只列出名称匹配PATTERN的角色。 列出所有数据库角色。

-- [OTHER]
\ du \dE[S+] [PATTERN] \di[S+] [PATTERN] \ds[S+] [PATTERN] \dt[S+] [PATTERN] \dv[S+] [PATTERN] 这一组命令，字母E，i，s，t和v分别代表着外部表，索引，序列，表和视图。可以以任意顺序指定其中一个或者它们的组合来列出这些对象。例如：\dit列出所有的索引和表。在命令名称后面追加+，则每一个对象的物理尺寸以及相关的描述也会被列出。 如果指定了PATTERN，只列出名称匹配该PATTERN的对象。默认情况下只会显示用户创建的对象。通过PATTERN或者S修饰符可以把系统对象包括在内。 列出所有的索引和视图。

-- [OTHER]
\ div \dx[+] [PATTERN] 列出安装数据库的扩展信息。 如果指定了PATTERN，则只列出名称匹配PATTERN的扩展信息。 列出安装数据库的扩展信息。

-- [OTHER]
\ dx \l[+] 列出服务器上所有数据库的名称、所有者、字符集编码以及使用权限。 - 列出服务器上所有数据库的名称、所有者、字符集编码以及使用权限。

-- [OTHER]
\ l \sf[+] FUNCNAME 显示函数的定义。 说明： 对于带圆括号的函数名，需要在函数名两端添加双引号，并且在双引号后面加上参数类型列表。参数类型列表两端添加圆括号。 - 假设存在函数function_a和函数名带圆括号的函数func()name，列出函数的定义。 1 2

-- [OTHER]
\ sf function_a

-- [OTHER]
\ sf "func()name" ( argtype1 , argtype2 ) \z [PATTERN] 列出数据库中所有表、视图和序列，以及它们相关的访问特权。 如果给出任何pattern ，则被当成一个正则表达式，只显示匹配的表、视图、序列。 列出数据库中所有表、视图和序列，以及它们相关的访问特权。

-- [OTHER]
\ z 表5 权限的参数说明 参数 参数说明 r SELECT：允许对指定的表、视图读取数据。 w UPDATE：允许对指定表更新字段。 a INSERT：允许对指定表插入数据。 d DELETE：允许删除指定表中的数据。 D TRUNCATE：允许清理指定表中的数据。 x REFERENCES：允许创建外键约束。 t TRIGGER：允许在指定表上创建触发器。 X EXECUTE：允许使用指定的函数，以及利用这些函数实现的操作符。 U USAGE： 对于过程语言，允许用户在创建函数时，指定过程语言。 对于模式，允许访问包含在指定模式中的对象。 对于序列，允许使用nextval函数。 C CREATE： 对于数据库，允许在该数据库里创建新的模式。 对于模式，允许在该模式中创建新的对象。 对于表空间，允许在其中创建表，以及允许创建数据库和模式的时候把该表空间指定为其缺省表空间。 c CONNECT：允许用户连接到指定的数据库。 T TEMPORARY：允许创建临时表。 A ALTER：允许用户修改指定对象的属性。 P DROP：允许用户删除指定的对象。 m COMMENT：允许用户定义或修改指定对象的注释。 i INDEX：允许用户在指定表上创建索引。 v VACUUM：允许用户对指定的表执行ANALYZE和VACUUM操作。 * 给前面权限的授权选项。 表6 格式化元命令 参数 参数说明 \a 对齐模式和非对齐模式之间的切换。 \C [STRING] 把正在打印的表的标题设置为一个查询的结果或者取消这样的设置。 \f [STRING] 对于不对齐的查询输出，显示或者设置域分隔符。 \H 若当前模式为文本格式，则切换为HTML输出格式。 若当前模式为HTML格式，则切换回文本格式。 \pset NAME [VALUE] 设置影响查询结果表输出的选项。NAME的取值见 表7 。 \t [on|off] 切换输出的字段名的信息和行计数脚注。 \T [STRING] 指定在使用HTML输出格式时放在table标签里的属性。如果参数为空，不设置。 \x [on|off|auto] 切换扩展行格式。 表7 可调节的打印选项 选项 选项说明 取值范围 border value必须是一个数字。通常这个数字越大，表的边界就越宽线就越多，但是这个取决于特定的格式。 在HTML格式下，取值范围为大于0的整数。 在其他格式下，取值范围： 0：无边框 1：内部分隔线 2：台架 expanded (或x) 在正常和扩展格式之间切换。 当打开扩展格式时，查询结果用两列显示，字段名称在左、数据在右。这个模式在数据无法放进通常的“水平”模式的屏幕时很有用。 在正常格式下，当查询输出的格式比屏幕宽时，用扩展格式。正常格式只对aligned和wrapped格式有用。 fieldsep 声明域分隔符来实现非对齐输出。这样就可以创建其他程序希望的制表符或逗号分隔的输出。要设置制表符域分隔符，键入\pset fieldsep '\t'。缺省域分隔符是 '|' (竖条符)。 - fieldsep_zero 声明域分隔符来实现非对齐输出到零字节。 - footer 用来切换脚注。 - format 设置输出格式。允许使用唯一缩写（这意味着一个字母就够了）。 取值范围： unaligned：写一行的所有列在一条直线上中，当前活动字段分隔符分隔。 aligned：此格式是标准的，可读性好的文本输出。 wrapped：类似aligned，但是包装跨行的宽数据值，使其适应目标字段的宽度输出。 html：把表输出为可用于文档里的对应标记语言。输出不是完整的文档。 latex：把表输出为可用于文档里的对应标记语言。输出不是完整的文档。 troff-ms：把表输出为可用于文档里的对应标记语言。输出不是完整的文档。 null 打印一个字符串，用来代替一个null值。 缺省是什么都不打印，这样很容易和空字符串混淆。 numericlocale 切换分隔小数点左边的数值的区域相关的分组符号。 on：显示指定的分隔符。 off：不显示分隔符。 忽略此参数，显示默认的分隔符。 pager 控制查询和gsql帮助输出的分页器。如果设置了环境变量PAGER，输出将被指向到指定程序，否则使用系统缺省。 on：当输出到终端且不适合屏幕显示时，使用分页器。 off：不使用分页器。 always：当输出到终端无论是否符合屏幕显示时，都使用分页器。 recordsep 声明在非对齐输出格式时的记录分隔符。 - recordsep_zero 声明在非对齐输出到零字节时的记录分隔符。 - tableattr（或T） 声明放在html输出格式中HTML table标签的属性（例如：cellpadding或bgcolor）。注意：这里可能不需要声明border，因为已经在\pset border里用过了。如果没有给出value，则不设置表的属性。 - title 为随后打印的表设置标题。这个可以用于给输出一个描述性标签。如果没有给出value，不设置标题。 - tuples_only （或者t） 在完全显示和只显示实际的表数据之间切换。完全显示将输出像列头、标题、各种脚注等信息。在tuples_only模式下，只显示实际的表数据。 - feedback 切换是否输出结果行数 - 表8 连接元命令 参数 参数说明 取值范围 \c[onnect] [DBNAME|- USER|- HOST|- PORT|-] 连接到一个新的数据库。当数据库名称长度超过63个字节时，默认前63个字节有效，连接到前63个字节对应的数据库，但是gsql的命令提示符中显示的数据库对象名仍为截断前的名称。 说明： 重新建立连接时，如果切换数据库登录用户，将可能会出现交互式输入，要求输入新用户的连接密码。该密码最长长度为999字节，受限于GUC参数password_max_length的最大值。 - \encoding [ENCODING] 设置客户端字符编码格式。 不带参数时，显示当前的编码格式。 \conninfo 输出当前连接的数据库的信息。 - 表9 操作系统元命令 参数 参数说明 取值范围 \cd [DIR] 切换当前的工作目录。 绝对路径或相对路径，且满足操作系统路径命名规则。 \setenv NAME [VALUE] 设置环境变量NAME为VALUE，如果没有给出VALUE值，则不设置环境变量。 - \timing [on|off] 以毫秒为单位显示每条SQL语句的执行时间 （不包括屏显打印时间） 。 on表示打开显示。 off表示关闭显示。 \! [COMMAND] 返回到一个单独的Unix shell或者执行Unix命令COMMAND。 - 表10 变量元命令 参数 参数说明 \prompt [TEXT] NAME 提示用户用文本格式来指定变量名称。 \set [NAME [VALUE]] 设置内部变量NAME为VALUE或者如果给出了多于一个值，设置为所有这些值的连接结果。如果没有给出第二个参数，只设变量不设值。 有一些常用变量被gsql特殊对待，它们是一些选项设置，通常所有特殊对待的变量都是由大写字母组成(可能还有数字和下划线)。 表11 是一个所有特殊对待的变量列表。 \unset NAME 不设置（或删除）gsql变量名。 表11 \set常用命令 名称 命令说明 取值范围 \set VERBOSITY value 这个选项可以设置为值default，verbose，terse之一以控制错误报告的冗余行。 value取值范围：default， verbose，terse \set ON_ERROR_STOP value 如果设置了这个变量，脚本处理将马上停止。如果该脚本是从另外一个脚本调用的，那个脚本也会按同样的方式停止。如果最外层的脚本不是从一次交互的gsql会话中调用的而是用-f选项调用的，gsql将返回错误代码3，以示这个情况与致命错误条件的区别（错误代码为1）。 value取值范围为：on/off，true/false，yes/no，1/0 \set AUTOCOMMIT [on|off] 设置当前gsql连接的自动提交行为，on为打开自动提交，off为关闭自动提交。默认情况下，gsql连接处于自动提交模式，每个单独的语句都被隐式提交。如果基于性能或者其它方面考虑，需要关闭自动提交时，需要用户自己显示的发出COMMIT命令来保证事务的提交。例如，在指定的业务SQL执行完之后发送COMMIT语句显式提交，特别是gsql客户端退出之前务必保证所有的事务已经提交。 说明： gsql默认使用自动提交模式，若关闭自动提交，将会导致后面执行的语句都受到隐式事务包裹，数据库中不支持在事务中执行的语句不能在此模式下执行。 on表示打开自动提交。 off表示关闭自动提交。 表12 大对象元命令 参数 参数说明 \lo_list 显示一个目前存储在该数据库里的所有 GaussDB 大对象和提供给他们的注释。 表13 全密态元命令 参数 参数说明 \send_token 全密态功能，传输密钥到服务端缓存，只在开启内存解密逃生通道的情况下使用。 \st 全密态功能，传输密钥到服务端缓存，只在开启内存解密逃生通道的情况下使用。 \clear_token 全密态功能，销毁服务端缓存的密钥，只在开启内存解密逃生通道的情况下使用。 \ct 全密态功能，销毁服务端缓存的密钥，只在开启内存解密逃生通道的情况下使用。 \key_info KEY_INFO 在全密态数据库特性中，用于设置访问外部密钥管理者的参数。 PATTERN 很多\d命令都可以用一个PATTERN参数来指定要被显示的对象名称。在最简单的情况下，PATTERN正好就是该对象的准确名称。在PATTERN中的字符通常会被变成小写形式（就像在SQL名称中那样），例如\dt FOO将会显示名为foo的表。就像在SQL名称中那样，把PATTERN放在双引号中可以阻止它被转换成小写形式。如果需要在一个PATTERN中包括一个真正的双引号字符，则需要把它写成两个相邻的双引号，这同样是符合SQL引用标识符的规则。例如，\dt "FOO""BAR"将显示名为FOO"BAR（不是foo"bar）的表。和普通的SQL名称规则不同，不能只在PATTERN的一部分周围放上双引号，例如\dt FOO"FOO"BAR将会显示名为fooFOObar的表。 不使用PATTERN参数时，\d命令会显示当前schema搜索路径中可见的全部对象——这等价于用*作为PATTERN。所谓对象可见是指可以直接用名称引用该对象，而不需要用schema来进行限定。要查看数据库中所有的对象而不管它们的可见性，可以把*.*用作PATTERN。 如果放在一个PATTERN中，*将匹配任意字符序列（包括空序列），而?会匹配任意的单个字符（这种记号方法就像 Unix shell 的文件名PATTERN一样）。例如，\dt int*会显示名称以int开始的表。但是如果被放在双引号内，*和?就会失去这些特殊含义而变成普通的字符。 包含一个点号（.）的PATTERN被解释为一个schema名称模式后面跟上一个对象名称模式。例如，\dt foo*.*bar*会显示名称以foo开始的schema中所有名称包括bar的表。如果没有出现点号，那么模式将只匹配当前schema搜索路径中可见的对象。同样，双引号内的点号会失去其特殊含义并且变成普通的字符。 高级用户可以使用字符类等正则表达式记法，如[0-9]可以匹配任意数字。所有的正则表达式特殊字符都按照POSIX正则表达式所说的工作。以下字符除外： .会按照上面所说的作为一种分隔符。 *会被翻译成正则表达式记号.*。 ?会被翻译成.。 $则按字面意思匹配。 根据需要，可以通过书写?、( R +|)、( R |)和 R ?来分别模拟PATTERN字符.、 R *和 R ?。$不需要作为一个正则表达式字符，因为PATTERN必须匹配整个名称，而不是像正则表达式的常规用法那样解释（换句话说，$会被自动地追加到PATTERN上）。如果不希望该PATTERN的匹配位置被固定，可以在开头或者结尾写上*。注意在双引号内，所有的正则表达式特殊字符会失去其特殊含义并且按照其字面意思进行匹配。另外，在操作符名称PATTERN中（即\do的PATTERN参数），正则表达式特殊字符也按照字面意思进行匹配。 DELIMITER 更改SQL语句之间分隔符命令，分隔符默认值为“;


================================================================================
-- 来源: 5949_gs_rescue.txt
================================================================================

-- [DDL]
create table original(col1 integer);

-- [DML_COPY]
copy original from '/data2/file01';

-- [DDL]
create table amend(col1 integer,col2 integer default 0);

-- [DML_COPY]
copy amend from '/data2/file02';

-- [DML_INSERT]
insert into amend(col1) select * from original;


================================================================================
-- 来源: 733_file_733.txt
================================================================================

-- [DDL]
create table a(id int, value int);

-- [DML_INSERT]
insert into a values(1,4);

-- [DML_INSERT]
insert into a values(2,4);

-- [TCL]
start transaction isolation level repeatable read;

-- [DQL]
select * from a;

-- [DML_UPDATE]
update a set value = 6 where id = 1;

-- [DQL]
select * from a;

-- [TCL]
start transaction isolation level repeatable read;

-- [DQL]
select * from a;

-- [DML_UPDATE]
update a set value = 6 where id = 2;

-- [DQL]
select * from a;

-- [TCL]
commit;

-- [TCL]
commit;

-- [DQL]
select * from a;

-- [DDL]
create table a(id int primary key, value int);

-- [DML_INSERT]
insert into a values(1,10);

-- [TCL]
start transaction isolation level repeatable read;

-- [DML_DELETE]
delete a where id = 1;

-- [TCL]
start transaction isolation level repeatable read;

-- [DQL]
select * from a;

-- [TCL]
commit;

-- [DML_INSERT]
insert into a values(1, 100);

-- [DQL]
select * from a;


================================================================================
-- 来源: 738_file_738.txt
================================================================================

-- [DDL]
CREATE USER sysadmin WITH SYSADMIN password "********" ;

-- [DDL]
ALTER USER joe SYSADMIN ;

-- [DDL]
CREATE USER createrole WITH CREATEROLE password "********" ;

-- [DDL]
ALTER USER joe CREATEROLE ;

-- [DDL]
CREATE USER auditadmin WITH AUDITADMIN password "********" ;

-- [DDL]
ALTER USER joe AUDITADMIN ;

-- [DDL]
CREATE USER monadmin WITH MONADMIN password "********" ;

-- [DDL]
ALTER USER joe MONADMIN ;

-- [DDL]
CREATE USER opradmin WITH OPRADMIN password "********" ;

-- [DDL]
ALTER USER joe OPRADMIN ;

-- [DDL]
CREATE USER poladmin WITH POLADMIN password "********" ;

-- [DDL]
ALTER USER joe POLADMIN ;


================================================================================
-- 来源: 740_file_740.txt
================================================================================

-- [DDL]
CREATE USER joe WITH CREATEDB PASSWORD "********" ;

-- [DQL]
SELECT * FROM pg_user ;

-- [DQL]
SELECT * FROM pg_authid ;

-- [DDL]
CREATE USER user_persistence WITH PERSISTENCE IDENTIFIED BY "********" ;


================================================================================
-- 来源: 742_Schema.txt
================================================================================

-- [DQL]
SELECT s . nspname , u . usename AS nspowner FROM pg_namespace s , pg_user u WHERE nspname = 'schema_name' AND s . nspowner = u . usesysid ;

-- [DQL]
SELECT * FROM pg_namespace ;

-- [DQL]
SELECT distinct ( tablename ), schemaname from pg_tables where schemaname = 'pg_catalog' ;

-- [SESSION]
SHOW SEARCH_PATH ;

-- [SESSION]
SET SEARCH_PATH TO myschema , public ;


================================================================================
-- 来源: 743_file_743.txt
================================================================================

-- [DCL_GRANT]
GRANT USAGE ON SCHEMA tpcds TO joe ;

-- [DCL_GRANT]
GRANT SELECT ON TABLE tpcds . web_returns to joe ;

-- [DDL]
CREATE ROLE lily WITH CREATEDB PASSWORD "********" ;

-- [DCL_GRANT]
GRANT USAGE ON SCHEMA tpcds TO lily ;

-- [DCL_GRANT]
GRANT SELECT ON TABLE tpcds . web_returns to lily ;

-- [DCL_GRANT]
GRANT lily to joe ;


================================================================================
-- 来源: 744_file_744.txt
================================================================================

-- [DDL]
CREATE USER alice PASSWORD '********' ;

-- [DDL]
CREATE USER bob PASSWORD '********' ;

-- [DDL]
CREATE USER peter PASSWORD '********' ;

-- [DDL]
CREATE TABLE all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

-- [DML_INSERT]
INSERT INTO all_data VALUES ( 1 , 'alice' , 'alice data' );

-- [DML_INSERT]
INSERT INTO all_data VALUES ( 2 , 'bob' , 'bob data' );

-- [DML_INSERT]
INSERT INTO all_data VALUES ( 3 , 'peter' , 'peter data' );

-- [DCL_GRANT]
GRANT SELECT ON all_data TO alice , bob , peter ;

-- [DDL]
ALTER TABLE all_data ENABLE ROW LEVEL SECURITY ;

-- [OTHER]
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

-- [OTHER]
\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_rls" FOR ALL TO public USING ((( role ):: name = "current_user" ())) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --切换至用户alice，执行SQL"SELECT * FROM public.all_data"

-- [DQL]
SELECT * FROM public . all_data ;

-- [EXPLAIN]
EXPLAIN ( COSTS OFF ) SELECT * FROM public . all_data ;

-- [DQL]
SELECT * FROM public . all_data ;

-- [EXPLAIN]
EXPLAIN ( COSTS OFF ) SELECT * FROM public . all_data ;


================================================================================
-- 来源: 751_file_751.txt
================================================================================

-- [DDL]
CREATE USER joe WITH PASSWORD "********" ;

-- [DCL_GRANT]
GRANT ALL PRIVILEGES TO joe;


================================================================================
-- 来源: 752_file_752.txt
================================================================================

-- [DDL]
CREATE DATABASE db_tpcds ;

-- [DDL]
CREATE DATABASE db_tpcds WITH TABLESPACE = hr_local ;

-- [OTHER]
gsql((GaussDB Kernel XXX.XXX.XXX build f521c606) compiled at 2021-09-16 14:55:22 commit 2935 last mr 6385 release) Non-SSL connection (SSL connection is recommended when requiring high-security) Type "help" for help. db_tpcds=> 查看数据库 使用\l元命令查看数据库系统的数据库列表。

-- [OTHER]
\ l 使用如下命令通过系统表pg_database查询数据库列表。

-- [DQL]
SELECT datname FROM pg_database ;

-- [DDL]
ALTER DATABASE db_tpcds RENAME TO human_tpcds ;

-- [DDL]
DROP DATABASE human_tpcds ;


================================================================================
-- 来源: 753_file_753.txt
================================================================================

-- [DDL]
CREATE TABLE customer_t1 ( state_ID CHAR ( 2 ), state_NAME VARCHAR2 ( 40 ), area_ID NUMBER );

-- [DDL]
DROP TABLE customer_t1 ;

-- [DDL]
CREATE TABLE customer_t2 ( state_ID CHAR ( 2 ), state_NAME VARCHAR2 ( 40 ), area_ID NUMBER ) WITH ( ORIENTATION = COLUMN );

-- [DDL]
DROP TABLE customer_t2 ;


================================================================================
-- 来源: 754_file_754.txt
================================================================================

-- [DDL]
CREATE TABLESPACE ds_location1 LOCATION '/pg_location/mount1/path1' MAXSIZE '500G' ;

-- [DDL]
CREATE USER jack IDENTIFIED BY '********' ;

-- [DDL]
CREATE TABLESPACE fastspace RELATIVE LOCATION 'my_tablespace/tablespace1' ;

-- [DCL_GRANT]
GRANT CREATE ON TABLESPACE fastspace TO jack ;

-- [DDL]
CREATE TABLE foo ( i int ) TABLESPACE fastspace ;

-- [SESSION]
SET default_tablespace = 'fastspace' ;

-- [DDL]
CREATE TABLE foo2 ( i int );

-- [DQL]
SELECT spcname FROM pg_tablespace ;

-- [DQL]
SELECT PG_TABLESPACE_SIZE ( 'fastspace' );

-- [DDL]
ALTER TABLESPACE fastspace RENAME TO fspace ;

-- [DDL]
DROP USER jack CASCADE ;

-- [DDL]
DROP TABLE foo ;

-- [DDL]
DROP TABLE foo2 ;

-- [DDL]
DROP TABLESPACE fspace ;


================================================================================
-- 来源: 756_file_756.txt
================================================================================

-- [DDL]
CREATE TABLE customer_t1 ( c_customer_sk integer , c_customer_id char ( 5 ), c_first_name char ( 6 ), c_last_name char ( 8 ) ) distribute by hash ( c_last_name );


================================================================================
-- 来源: 757_file_757.txt
================================================================================

-- [DDL]
CREATE TABLE table1 ( id int , a char ( 6 ), b varchar ( 6 ), c varchar ( 6 ));

-- [DDL]
CREATE TABLE table2 ( id int , a char ( 20 ), b varchar ( 20 ), c varchar ( 20 ));

-- [DML_INSERT]
INSERT INTO table1 VALUES ( 1 , reverse ( '123AAA78' ), reverse ( '123AA78' ), reverse ( '123AA78' ));

-- [DML_INSERT]
INSERT INTO table1 VALUES ( 2 , reverse ( '123A78' ), reverse ( '123A78' ), reverse ( '123A78' ));

-- [DML_INSERT]
INSERT INTO table1 VALUES ( 3 , '87A123' , '87A123' , '87A123' );

-- [DML_INSERT]
INSERT INTO table2 VALUES ( 1 , reverse ( '123AA78' ), reverse ( '123AA78' ), reverse ( '123AA78' ));

-- [DML_INSERT]
INSERT INTO table2 VALUES ( 2 , reverse ( '123A78' ), reverse ( '123A78' ), reverse ( '123A78' ));

-- [DML_INSERT]
INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 3769 , 'hello' , 'Grace' );

-- [DML_INSERT]
INSERT INTO customer_t1 VALUES ( 3769 , 'hello' , 'Grace' );

-- [DML_INSERT]
INSERT INTO customer_t1 ( c_customer_sk , c_first_name ) VALUES ( 3769 , 'Grace' );

-- [DML_INSERT]
INSERT INTO customer_t1 VALUES ( 3769 , 'hello' );

-- [DML_INSERT]
INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 3769 , 'hello' , DEFAULT );

-- [DML_INSERT]
INSERT INTO customer_t1 DEFAULT VALUES ;

-- [DML_INSERT]
INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 6885 , 'maps' , 'Joes' ), ( 4321 , 'tpcds' , 'Lily' ), ( 9527 , 'world' , 'James' );

-- [DDL]
CREATE TABLE customer_t2 ( c_customer_sk integer , c_customer_id char ( 5 ), c_first_name char ( 6 ), c_last_name char ( 8 ) );

-- [DML_INSERT]
INSERT INTO customer_t2 SELECT * FROM customer_t1 ;

-- [DDL]
DROP TABLE customer_t2 CASCADE ;


================================================================================
-- 来源: 758_file_758.txt
================================================================================

-- [DML_UPDATE]
UPDATE customer_t1 SET c_customer_sk = 9876 WHERE c_customer_sk = 9527 ;

-- [DML_UPDATE]
UPDATE customer_t1 SET c_customer_sk = c_customer_sk + 100 ;

-- [DML_UPDATE]
UPDATE customer_t1 SET c_customer_id = 'Admin' , c_first_name = 'Local' WHERE c_customer_sk = 4421 ;


================================================================================
-- 来源: 759_file_759.txt
================================================================================

-- [DQL]
SELECT * FROM pg_tables ;

-- [OTHER]
\ d + customer_t1 ;

-- [DQL]
SELECT count ( * ) FROM customer_t1 ;

-- [DQL]
SELECT * FROM customer_t1 ;

-- [DQL]
SELECT c_customer_sk FROM customer_t1 ;

-- [DQL]
SELECT DISTINCT ( c_customer_sk ) FROM customer_t1 ;

-- [DQL]
SELECT * FROM customer_t1 WHERE c_customer_sk = 3869 ;

-- [DQL]
SELECT * FROM customer_t1 ORDER BY c_customer_sk ;


================================================================================
-- 来源: 760_file_760.txt
================================================================================

-- [DML_DELETE]
DELETE FROM customer_t1 WHERE c_customer_sk = 3869 ;

-- [DML_DELETE]
DELETE FROM customer_t1 ;

-- [DML_TRUNCATE]
TRUNCATE TABLE customer_t1 ;

-- [DDL]
DROP TABLE customer_t1 ;


================================================================================
-- 来源: 761_file_761.txt
================================================================================

-- [DDL]
CREATE TABLE public.search_table_t1(a int) distribute by hash(a);

-- [DDL]
CREATE TABLE public.search_table_t2(b int) distribute by hash(b);

-- [DDL]
CREATE TABLE public.search_table_t3(c int) distribute by hash(c);

-- [DDL]
CREATE TABLE public.search_table_t4(d int) distribute by hash(d);

-- [DDL]
CREATE TABLE public.search_table_t5(e int) distribute by hash(e);

-- [DQL]
SELECT distinct ( tablename ) FROM pg_tables WHERE SCHEMANAME = 'public' AND TABLENAME LIKE 'search_table%' ;


================================================================================
-- 来源: 763_schema.txt
================================================================================

-- [DDL]
CREATE SCHEMA myschema ;

-- [DDL]
CREATE SCHEMA myschema AUTHORIZATION omm ;

-- [DDL]
CREATE TABLE myschema . mytable ( id int , name varchar ( 20 ));

-- [DQL]
SELECT * FROM myschema . mytable ;

-- [SESSION]
SHOW SEARCH_PATH ;

-- [SESSION]
SET SEARCH_PATH TO myschema , public ;

-- [DCL_REVOKE]
REVOKE CREATE ON SCHEMA public FROM PUBLIC ;

-- [DQL]
SELECT current_schema ();

-- [DDL]
CREATE USER jack IDENTIFIED BY '********' ;

-- [DCL_GRANT]
GRANT USAGE ON schema myschema TO jack ;

-- [DCL_REVOKE]
REVOKE USAGE ON schema myschema FROM jack ;

-- [DDL]
DROP SCHEMA IF EXISTS nullschema ;

-- [DDL]
DROP SCHEMA myschema CASCADE ;

-- [DDL]
DROP USER jack ;


================================================================================
-- 来源: 764_file_764.txt
================================================================================

-- [DDL]
CREATE TABLE tpcds . customer_address ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) DISTRIBUTE BY HASH ( ca_address_sk ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) ) ENABLE ROW MOVEMENT ;

-- [DDL]
CREATE TABLE tpcds . web_returns_p2 ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) DISTRIBUTE BY HASH ( ca_address_sk ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) ) ENABLE ROW MOVEMENT ;

-- [DML_INSERT]
INSERT INTO tpcds . web_returns_p2 SELECT * FROM tpcds . customer_address ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 DISABLE ROW MOVEMENT ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 DROP PARTITION P8 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 ADD PARTITION P8 VALUES LESS THAN ( MAXVALUE );

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION P8 TO P_9 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION FOR ( 40000 ) TO P8 ;

-- [DQL]
SELECT * FROM tpcds . web_returns_p2 PARTITION ( P6 );

-- [DQL]
SELECT * FROM tpcds . web_returns_p2 PARTITION FOR ( 35888 );

-- [DDL]
DROP TABLE tpcds . customer_address ;

-- [DDL]
DROP TABLE tpcds . web_returns_p2 ;

-- [DDL]
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1/tablespace_1' ;

-- [DDL]
CREATE TABLESPACE example2 RELATIVE LOCATION 'tablespace2/tablespace_2' ;

-- [DDL]
CREATE TABLESPACE example3 RELATIVE LOCATION 'tablespace3/tablespace_3' ;

-- [DDL]
CREATE TABLESPACE example4 RELATIVE LOCATION 'tablespace4/tablespace_4' ;

-- [DDL]
CREATE TABLE tpcds . customer_address ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) TABLESPACE example1 DISTRIBUTE BY HASH ( ca_address_sk ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) TABLESPACE example2 ) ENABLE ROW MOVEMENT ;

-- [DDL]
CREATE TABLE tpcds . web_returns_p2 ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) TABLESPACE example1 DISTRIBUTE BY HASH ( ca_address_sk ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) TABLESPACE example2 ) ENABLE ROW MOVEMENT ;

-- [DML_INSERT]
INSERT INTO tpcds . web_returns_p2 SELECT * FROM tpcds . customer_address ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 DISABLE ROW MOVEMENT ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 DROP PARTITION P8 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 ADD PARTITION P8 VALUES LESS THAN ( MAXVALUE );

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION P8 TO P_9 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION FOR ( 40000 ) TO P8 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P6 TABLESPACE example3 ;

-- [DDL]
ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P4 TABLESPACE example4 ;

-- [DQL]
SELECT * FROM tpcds . web_returns_p2 PARTITION ( P6 );

-- [DQL]
SELECT * FROM tpcds . web_returns_p2 PARTITION FOR ( 35888 );

-- [DDL]
DROP TABLE tpcds . customer_address ;

-- [DDL]
DROP TABLE tpcds . web_returns_p2 ;

-- [DDL]
DROP TABLESPACE example1 ;

-- [DDL]
DROP TABLESPACE example2 ;

-- [DDL]
DROP TABLESPACE example3 ;

-- [DDL]
DROP TABLESPACE example4 ;


================================================================================
-- 来源: 765_file_765.txt
================================================================================

-- [DDL]
CREATE INDEX tpcds_web_returns_p2_index1 ON tpcds . web_returns_p2 ( ca_address_id ) LOCAL ;

-- [DDL]
CREATE INDEX tpcds_web_returns_p2_index2 ON tpcds . web_returns_p2 ( ca_address_sk ) LOCAL ( PARTITION web_returns_p2_P1_index , PARTITION web_returns_p2_P2_index TABLESPACE example3 , PARTITION web_returns_p2_P3_index TABLESPACE example4 , PARTITION web_returns_p2_P4_index , PARTITION web_returns_p2_P5_index , PARTITION web_returns_p2_P6_index , PARTITION web_returns_p2_P7_index , PARTITION web_returns_p2_P8_index ) TABLESPACE example2 ;

-- [DDL]
ALTER INDEX tpcds . tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P2_index TABLESPACE example1 ;

-- [DDL]
ALTER INDEX tpcds . tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P3_index TABLESPACE example2 ;

-- [DDL]
ALTER INDEX tpcds . tpcds_web_returns_p2_index2 RENAME PARTITION web_returns_p2_P8_index TO web_returns_p2_P8_index_new ;

-- [DQL]
SELECT RELNAME FROM PG_CLASS WHERE RELKIND = 'i' ;

-- [OTHER]
\ di + tpcds . tpcds_web_returns_p2_index2 删除索引 1

-- [DDL]
DROP INDEX tpcds . tpcds_web_returns_p2_index1 ;

-- [DDL]
DROP INDEX tpcds . tpcds_web_returns_p2_index2 ;

-- [DDL]
CREATE TABLE tpcds . customer_address_bak AS TABLE tpcds . customer_address ;

-- [DQL]
SELECT ca_address_sk FROM tpcds . customer_address_bak WHERE ca_address_sk = 14888 ;

-- [DDL]
CREATE INDEX index_wr_returned_date_sk ON tpcds . customer_address_bak ( ca_address_sk );

-- [DDL]
CREATE UNIQUE INDEX ds_ship_mode_t1_index1 ON tpcds. ship_mode_t1(SM_SHIP_MODE_SK);

-- [DQL]
SELECT ca_address_sk , ca_address_id FROM tpcds . customer_address_bak WHERE ca_address_sk = 5050 AND ca_street_number < 1000 ;

-- [DDL]
CREATE INDEX more_column_index ON tpcds . customer_address_bak ( ca_address_sk , ca_street_number );

-- [DDL]
CREATE INDEX part_index ON tpcds . customer_address_bak ( ca_address_sk ) WHERE ca_address_sk = 5050 ;

-- [DQL]
SELECT * FROM tpcds . customer_address_bak WHERE trunc ( ca_street_number ) < 1000 ;

-- [DDL]
CREATE INDEX para_index ON tpcds . customer_address_bak ( trunc ( ca_street_number ));

-- [DDL]
DROP TABLE tpcds . customer_address_bak ;


================================================================================
-- 来源: 766_file_766.txt
================================================================================

-- [DDL]
CREATE OR REPLACE VIEW MyView AS SELECT * FROM tpcds . web_returns WHERE trunc ( wr_refunded_cash ) > 10000 ;

-- [DQL]
SELECT * FROM MyView ;

-- [DQL]
SELECT * FROM my_views ;

-- [DQL]
SELECT * FROM adm_views ;

-- [OTHER]
\ d + MyView View "PG_CATALOG.MyView" Column | Type | Modifiers | Storage | Description ----------+-----------------------+-----------+----------+------------- USERNAME | CHARACTER VARYING ( 64 ) | | extended | View definition : SELECT PG_AUTHID . ROLNAME :: CHARACTER VARYING ( 64 ) AS USERNAME FROM PG_AUTHID ;

-- [DDL]
DROP VIEW MyView ;


================================================================================
-- 来源: 767_file_767.txt
================================================================================

-- [DDL]
CREATE TABLE T1 ( id serial , name text );

-- [DDL]
CREATE SEQUENCE seq1 cache 100 ;

-- [DDL]
CREATE TABLE T2 ( id int not null default nextval ( 'seq1' ), name text );

-- [DDL]
ALTER SEQUENCE seq1 OWNED BY T2 . id ;

-- [DDL]
CREATE SEQUENCE newSeq1 ;

-- [DDL]
CREATE TABLE newT1 ( id int not null default nextval ( 'newSeq1' ), name text );

-- [DML_INSERT]
INSERT INTO newT1 ( name ) SELECT name FROM T1 ;

-- [DML_INSERT]
INSERT INTO newT1 ( id , name ) SELECT id , name FROM T1 ;

-- [DQL]
SELECT SETVAL ( 'newSeq1' , 10000 );


================================================================================
-- 来源: 768_file_768.txt
================================================================================

-- [DDL]
CREATE TABLE test ( id int , time date );

-- [DDL]
CREATE OR REPLACE PROCEDURE PRC_JOB_1 () AS N_NUM integer : = 1 ;

-- [PLSQL]
call dbe_task . submit ( 'call public.prc_job_1();

-- [PLSQL]
call dbe_task . id_submit ( 1 , 'call public.prc_job_1();

-- [DQL]
select job , dbname , start_date , last_date , this_date , next_date , broken , status , interval , failures , what from my_jobs ;

-- [PLSQL]
call dbe_task . finish ( 1 , true );

-- [PLSQL]
call dbe_task . finish ( 1 , false );

-- [PLSQL]
call dbe_task . next_date ( 1 , sysdate + 1 . 0 / 24 );

-- [PLSQL]
call dbe_task . interval ( 1 , 'sysdate + 1.0/24' );

-- [PLSQL]
call dbe_task . content ( 1 , 'insert into public.test values(333, sysdate+5);

-- [PLSQL]
call dbe_task . update ( 1 , 'call public.prc_job_1();

-- [PLSQL]
call dbe_task . cancel ( 1 );


================================================================================
-- 来源: 780_SQL.txt
================================================================================

-- [DQL]
SELECT CURRENT_DATE ;

-- [DQL]
SELECT CURRENT_TIME ;

-- [DQL]
SELECT CURRENT_TIMESTAMP ( 6 );


================================================================================
-- 来源: 977_SQL.txt
================================================================================

-- [EXPLAIN]
explain select * from t1,t2 where t1.c1=t2.c2;

-- [EXPLAIN]
explain select * from t1,t2 where t1.c1=t2.c2;


================================================================================
-- 来源: 978_file_978.txt
================================================================================

-- [EXPLAIN]
EXPLAIN SELECT * FROM t1,t2 WHERE t1.c1 = t2.c2;

-- [EXPLAIN]
explain select c1 , count ( 1 ) from t1 group by c1 ;

-- [SESSION]
set enable_fast_query_shipping=off;

-- [EXPLAIN]
explain select c1,count(1) from t1 group by c1;

-- [EXPLAIN]
explain performance select count(1) from t1;


================================================================================
-- 来源: 991_file_991.txt
================================================================================

-- [EXPLAIN]
explain select * from t where c1 > 1;

-- [EXPLAIN]
explain select * from t limit 1;

-- [EXPLAIN]
explain select sum(c1), count(*) from t;

-- [DDL]
create table t(c1 int, c2 int, c3 int)distribute by hash(c1);

-- [DDL]
create table t1(c1 int, c2 int, c3 int)distribute by hash(c1);

-- [EXPLAIN]
explain select * from t1 join t on t.c1 = t1.c1;

-- [EXPLAIN]
explain select * from t1 join t on t.c1 = t1.c2;

-- [DDL]
CREATE NODE GROUP ng WITH(datanode1, datanode2, datanode3, datanode4, datanode5, datanode6);

-- [DDL]
CREATE TABLE t1(a int, b int, c int) DISTRIBUTE BY HASH(a) TO GROUP ng;

-- [DDL]
CREATE TABLE t2(a int, b int, c int) DISTRIBUTE BY HASH(a) TO GROUP ng;

-- [EXPLAIN]
EXPLAIN (COSTS OFF) SELECT * FROM t1 UNION ALL SELECT * FROM t2;

-- [EXPLAIN]
EXPLAIN (COSTS OFF) SELECT * FROM t1 UNION SELECT * FROM t2;

-- [EXPLAIN]
EXPLAIN (COSTS OFF) SELECT * FROM t1 WHERE a = 1 UNION SELECT * FROM t2 WHERE a = 1;

-- [EXPLAIN]
EXPLAIN (COSTS OFF) SELECT * FROM t1 WHERE a = 1;

-- [EXPLAIN]
EXPLAIN (COSTS OFF) SELECT * FROM t2 WHERE a = 3;

-- [EXPLAIN]
EXPLAIN (COSTS OFF) SELECT * FROM t1 WHERE a = 1 UNION SELECT * FROM t2 WHERE a = 3;

-- [EXPLAIN]
EXPLAIN ( COSTS OFF ) WITH cte AS ( SELECT * FROM t1 ) SELECT * FROM cte ;

-- [EXPLAIN]
EXPLAIN ( COSTS OFF ) WITH cte AS ( SELECT * FROM t1 WHERE a = 1 ) SELECT * FROM cte ;

-- [EXPLAIN]
EXPLAIN ( COSTS OFF ) WITH cte AS ( SELECT * FROM t1 ORDER BY a ) SELECT * FROM cte ;

-- [EXPLAIN]
EXPLAIN ( COSTS OFF ) WITH RECURSIVE cte AS ( SELECT * FROM t1 UNION ALL SELECT t2 . * FROM t2 , cte WHERE cte . a = t2 . a ) SELECT * FROM cte ;

-- [EXPLAIN]
EXPLAIN ( COSTS OFF ) WITH RECURSIVE cte AS ( SELECT * FROM t1 WHERE a = 1 UNION ALL SELECT t2 . * FROM t2 , cte WHERE cte . a = t2 . a AND t2 . a = 1 ) SELECT * FROM cte ;

-- [EXPLAIN]
EXPLAIN ( COSTS OFF ) WITH RECURSIVE cte AS ( SELECT * FROM t1 UNION SELECT t2 . * FROM t2 , cte WHERE cte . a = t2 . a ) SELECT * FROM cte ;

-- [DDL]
DROP TABLE t1 ;

-- [DDL]
DROP TABLE t2 ;

-- [DDL]
DROP NODE GROUP ng ;

-- [DDL]
CREATE TABLE CUSTOMER1 ( C_CUSTKEY BIGINT NOT NULL , C_NAME VARCHAR ( 25 ) NOT NULL , C_ADDRESS VARCHAR ( 40 ) NOT NULL , C_NATIONKEY INT NOT NULL , C_PHONE CHAR ( 15 ) NOT NULL , C_ACCTBAL DECIMAL ( 15 , 2 ) NOT NULL , C_MKTSEGMENT CHAR ( 10 ) NOT NULL , C_COMMENT VARCHAR ( 117 ) NOT NULL ) DISTRIBUTE BY hash ( C_CUSTKEY );

-- [DDL]
CREATE TABLE test_stream ( a int , b float );

-- [DDL]
CREATE TABLE sal_emp ( c1 integer [] ) DISTRIBUTE BY replication ;

-- [EXPLAIN]
explain update customer1 set C_NAME = 'a' returning c_name ;

-- [EXPLAIN]
explain verbose select count ( c_custkey order by c_custkey ) from customer1 ;

-- [EXPLAIN]
explain verbose select count ( distinct b ) from test_stream ;

-- [EXPLAIN]
explain verbose select distinct on ( c_custkey ) c_custkey from customer1 order by c_custkey ;

-- [EXPLAIN]
explain verbose select array [ c_custkey , 1 ] from customer1 order by c_custkey ;


================================================================================
-- 来源: 994_file_994.txt
================================================================================

-- [EXPLAIN]
explain ( analyze on , costs off ) select * from t1 where c2 = 10004 ;

-- [DDL]
create index idx on t1 ( c2 );

-- [EXPLAIN]
explain ( analyze on , costs off ) select * from t1 where c2 = 10004 ;

-- [EXPLAIN]
explain analyze select count(*) from t2,t1 where t1.c1=t2.c2;

-- [SESSION]
set enable_mergejoin=off;

-- [SESSION]
set enable_nestloop=off;

-- [EXPLAIN]
explain analyze select count(*) from t2,t1 where t1.c1=t2.c2;

-- [EXPLAIN]
explain analyze select count(*) from t1 group by c2;

-- [SESSION]
set enable_sort=off;

-- [EXPLAIN]
explain analyze select count(*) from t1 group by c2;


================================================================================
-- 来源: 995_file_995.txt
================================================================================

-- [EXPLAIN]
explain performance select count ( * ) from inventory ;

-- [DQL]
select table_skewness ( 'inventory' );

-- [DQL]
select table_skewness ( 'inventory' );

