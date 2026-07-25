-- 来源: 2986_CREATE USER MAPPING.txt
-- SQL 数量: 7

CREATE ROLE bob PASSWORD '********';

-- 创建外部服务器
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

-- 创建USER MAPPING。
CREATE USER MAPPING FOR bob SERVER my_server OPTIONS (user 'bob', password '********');

-- 修改USER MAPPING。
ALTER USER MAPPING FOR bob SERVER my_server OPTIONS (SET password '********');

-- 删除USER MAPPING。
DROP USER MAPPING FOR bob SERVER my_server;

-- 删除外部服务器。
DROP SERVER my_server;

-- 删除角色。
DROP ROLE bob;

