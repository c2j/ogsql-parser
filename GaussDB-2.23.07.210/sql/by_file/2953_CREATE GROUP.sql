-- 来源: 2953_CREATE GROUP.txt
-- SQL 数量: 8

CREATE GROUP super_users WITH PASSWORD "********";

--创建用户。
CREATE ROLE lche WITH PASSWORD "********";

--创建用户。
CREATE ROLE jim WITH PASSWORD "********";

--向用户组中添加用户。
ALTER GROUP super_users ADD USER lche, jim;

--从用户组中删除用户。
ALTER GROUP super_users DROP USER jim;

--修改用户组的名称。
ALTER GROUP super_users RENAME TO normal_users;

--删除用户。
DROP ROLE lche, jim;

--删除用户组。
DROP GROUP normal_users;

