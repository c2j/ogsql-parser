-- 来源: 2908_ALTER ROW LEVEL SECURITY POLICY.txt
-- SQL 数量: 10

CREATE TABLE all_data(id int, role varchar(100), data varchar(100));

--创建行访问控制策略，当前用户只能查看用户自身的数据。
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING(role = CURRENT_USER);

--创建用户alice, bob。
CREATE ROLE alice WITH PASSWORD "********";

CREATE ROLE bob WITH PASSWORD "********";

--修改行访问控制all_data_rls的名称。
ALTER ROW LEVEL SECURITY POLICY all_data_rls ON all_data RENAME TO all_data_new_rls;

--修改行访问控制策略影响的用户。
ALTER ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data TO alice, bob;

--修改行访问控制策略表达式。
ALTER ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data USING (id > 100 AND role = current_user);

--删除访问控制策略。
DROP ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data;

--删除用户alice, bob。
DROP ROLE alice, bob;

--删除数据表all_data。
DROP TABLE all_data;

