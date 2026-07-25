-- 来源: 744_file_744.txt
-- SQL 数量: 15

CREATE USER alice PASSWORD '********' ;

CREATE USER bob PASSWORD '********' ;

CREATE USER peter PASSWORD '********' ;

CREATE TABLE all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

INSERT INTO all_data VALUES ( 1 , 'alice' , 'alice data' );

INSERT INTO all_data VALUES ( 2 , 'bob' , 'bob data' );

INSERT INTO all_data VALUES ( 3 , 'peter' , 'peter data' );

GRANT SELECT ON all_data TO alice , bob , peter ;

ALTER TABLE all_data ENABLE ROW LEVEL SECURITY ;

CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_rls" FOR ALL TO public USING ((( role ):: name = "current_user" ())) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --切换至用户alice，执行SQL"SELECT * FROM public.all_data"

SELECT * FROM public . all_data ;

EXPLAIN ( COSTS OFF ) SELECT * FROM public . all_data ;

SELECT * FROM public . all_data ;

EXPLAIN ( COSTS OFF ) SELECT * FROM public . all_data ;

