-- 来源: 1364_REVOKE.txt
-- SQL 数量: 4

REVOKE jerry FROM tom ;

REVOKE SELECT ON TABLE jerry . t1 FROM tom ;

REVOKE EXECUTE ON FUNCTION jerry . fun1 () FROM tom ;

REVOKE CONNECT ON database DB1 FROM tom ;

