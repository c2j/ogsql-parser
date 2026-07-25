-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY.txt
-- SQL 数量: 18

CREATE USER alice PASSWORD '*********' ;

CREATE USER bob PASSWORD '*********' ;

CREATE TABLE public . all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

INSERT INTO all_data VALUES ( 1 , 'alice' , 'alice data' );

INSERT INTO all_data VALUES ( 2 , 'bob' , 'bob data' );

INSERT INTO all_data VALUES ( 3 , 'peter' , 'peter data' );

GRANT SELECT ON all_data TO alice , bob ;

ALTER TABLE all_data ENABLE ROW LEVEL SECURITY ;

CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_rls" FOR ALL TO public USING ((( role ):: name = "current_user" ())) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --当前用户执行SELECT操作。

SELECT * FROM all_data ;

ALTER USER alice LOGIN ;

EXPLAIN ( COSTS OFF ) SELECT * FROM all_data ;

SELECT * FROM all_data ;

EXPLAIN ( COSTS OFF ) SELECT * FROM all_data ;

DROP ROW LEVEL SECURITY POLICY all_data_rls ON all_data ;

DROP TABLE public . all_data ;

DROP USER alice , bob ;

