-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY.txt
-- SQL 数量: 16

CREATE USER alice PASSWORD '********';

--创建用户bob。
CREATE USER bob PASSWORD '********';

--创建数据表all_data。
CREATE TABLE public.all_data(id int, role varchar(100), data varchar(100));

--向数据表插入数据。
INSERT INTO all_data VALUES(1, 'alice', 'alice data');

INSERT INTO all_data VALUES(2, 'bob', 'bob data');

INSERT INTO all_data VALUES(3, 'peter', 'peter data');

--将表all_data的读取权限赋予alice和bob用户。
GRANT SELECT ON all_data TO alice, bob;

--打开行访问控制策略开关。
ALTER TABLE all_data ENABLE ROW LEVEL SECURITY;

--创建行访问控制策略，当前用户只能查看用户自身的数据。
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING(role = CURRENT_USER);

--当前用户执行SELECT操作
SELECT * FROM all_data;

EXPLAIN(COSTS OFF) SELECT * FROM all_data;

--切换至alice用户执行SELECT操作。
SELECT * FROM all_data;

EXPLAIN(COSTS OFF) SELECT * FROM all_data;

--删除行访问控制策略。
DROP ROW LEVEL SECURITY POLICY all_data_rls ON all_data;

--删除数据表all_data。
DROP TABLE public.all_data;

--删除用户alice, bob。
DROP USER alice, bob;

