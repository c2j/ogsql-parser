-- 来源: 1311_DROP ROW LEVEL SECURITY POLICY.txt
-- SQL 数量: 4

CREATE TABLE all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

DROP ROW LEVEL SECURITY POLICY all_data_rls ON all_data ;

DROP TABLE all_data ;

