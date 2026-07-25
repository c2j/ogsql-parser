-- 来源: 3020_DROP ROW LEVEL SECURITY POLICY.txt
-- SQL 数量: 4

CREATE TABLE all_data(id int, role varchar(100), data varchar(100));

--创建行访问控制策略。
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING(role = CURRENT_USER);

--删除行访问控制策略。
DROP ROW LEVEL SECURITY POLICY all_data_rls ON all_data;

--删除数据表all_data。
DROP TABLE all_data;

