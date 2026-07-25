-- 来源: 2889_ALTER DIRECTORY.txt
-- SQL 数量: 4

CREATE OR REPLACE DIRECTORY dir as '/tmp/';

--创建用户
CREATE USER jim PASSWORD '********';

--修改目录的owner。
ALTER DIRECTORY dir OWNER TO jim;

--删除目录。
DROP DIRECTORY dir;

