-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY.txt
-- SQL 数量: 13

CREATE TABLE all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_rls" FOR ALL TO public USING ((( role ):: name = "current_user" ())) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no --修改行访问控制all_data_rls的名称。

ALTER ROW LEVEL SECURITY POLICY all_data_rls ON all_data RENAME TO all_data_new_rls ;

CREATE ROLE alice WITH PASSWORD "********" ;

CREATE ROLE bob WITH PASSWORD "********" ;

ALTER ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data TO alice , bob ;

\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_new_rls" FOR ALL TO alice , bob USING ((( role ):: name = "current_user" ())) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --修改行访问控制策略表达式。

ALTER ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data USING ( id > 100 AND role = current_user );

\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_new_rls" FOR ALL TO alice , bob USING ((( id > 100 ) AND (( role ):: name = "current_user" ()))) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --删除访问控制策略。

DROP ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data ;

DROP ROLE alice , bob ;

DROP TABLE all_data ;

