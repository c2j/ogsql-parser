-- 类别: DCL_REVOKE
-- SQL 数量: 20

-- 来源: 1213_ALTER SYNONYM
REVOKE ALL ON SCHEMA sysadmin FROM u1 ;

-- 来源: 1337_GRANT
REVOKE ALL PRIVILEGES FROM joe ;

-- 来源: 1337_GRANT
REVOKE joe FROM manager ;

-- 来源: 1337_GRANT
REVOKE manager FROM senior_manager ;

-- 来源: 1364_REVOKE
REVOKE jerry FROM tom ;

-- 来源: 1364_REVOKE
REVOKE SELECT ON TABLE jerry . t1 FROM tom ;

-- 来源: 1364_REVOKE
REVOKE EXECUTE ON FUNCTION jerry . fun1 () FROM tom ;

-- 来源: 1364_REVOKE
REVOKE CONNECT ON database DB1 FROM tom ;

-- 来源: 2461_schema
REVOKE CREATE ON SCHEMA public FROM PUBLIC ;

-- 来源: 2461_schema
REVOKE USAGE ON schema myschema FROM jack ;

--收回用户u1权限
-- 来源: 2914_ALTER SYNONYM
REVOKE ALL ON SCHEMA sysadmin FROM u1;

-- 来源: 3048_GRANT
REVOKE ALL PRIVILEGES FROM joe;

-- 来源: 3048_GRANT
REVOKE joe FROM manager;

-- 来源: 3048_GRANT
REVOKE manager FROM senior_manager;

-- 来源: 3075_REVOKE
REVOKE jerry FROM tom ;

-- 来源: 3075_REVOKE
REVOKE SELECT ON TABLE jerry . t1 FROM tom ;

-- 来源: 3075_REVOKE
REVOKE EXECUTE ON FUNCTION jerry . fun1 () FROM tom ;

-- 来源: 3075_REVOKE
REVOKE CONNECT ON database DB1 FROM tom ;

-- 来源: 763_schema
REVOKE CREATE ON SCHEMA public FROM PUBLIC ;

-- 来源: 763_schema
REVOKE USAGE ON schema myschema FROM jack ;

