-- 来源: 1036_GUCbest_agg_plan.txt
-- SQL 数量: 7

explain select a , count ( 1 ) from agg_t1 group by a ;

set best_agg_plan to 1 ;

explain select b , count ( 1 ) from t1 group by b ;

set best_agg_plan to 2 ;

explain select b , count ( 1 ) from t1 group by b ;

set best_agg_plan to 3 ;

explain select b , count ( 1 ) from t1 group by b ;

