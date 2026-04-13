-- 类别: DCL_GRANT
-- SQL 数量: 54

-- 来源: 1191_ALTER DEFAULT PRIVILEGES
GRANT USAGE , CREATE ON SCHEMA tpcds TO jack ;

-- 来源: 1213_ALTER SYNONYM
GRANT ALL ON SCHEMA sysadmin TO u1 ;

-- 来源: 1254_CREATE MASKING POLICY
GRANT ALL PRIVILEGES TO dev_mask ;

-- 来源: 1254_CREATE MASKING POLICY
GRANT ALL PRIVILEGES TO bob_mask ;

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
GRANT SELECT ON all_data TO alice , bob ;

-- 来源: 1337_GRANT
GRANT ALL PRIVILEGES TO joe ;

-- 来源: 1337_GRANT
GRANT USAGE ON SCHEMA tpcds TO joe ;

-- 来源: 1337_GRANT
GRANT ALL PRIVILEGES ON tpcds . reason TO joe ;

-- 来源: 1337_GRANT
GRANT select ( r_reason_sk , r_reason_id , r_reason_desc ), update ( r_reason_desc ) ON tpcds . reason TO joe ;

-- 来源: 1337_GRANT
GRANT select ( r_reason_sk , r_reason_id ) ON tpcds . reason TO joe WITH GRANT OPTION ;

-- 来源: 1337_GRANT
GRANT create , connect on database testdb TO joe WITH GRANT OPTION ;

-- 来源: 1337_GRANT
GRANT USAGE , CREATE ON SCHEMA tpcds TO tpcds_manager ;

-- 来源: 1337_GRANT
GRANT ALL ON TABLESPACE tpcds_tbspc TO joe ;

-- 来源: 1337_GRANT
GRANT ALTER ON FUNCTION tpcds.fun1() TO joe;

-- 来源: 1337_GRANT
GRANT joe TO manager WITH ADMIN OPTION ;

-- 来源: 1337_GRANT
GRANT manager TO senior_manager ;

-- 来源: 2366_file_2366
grant plsql_rollback1 to plsql_rollback2;

-- 来源: 2441_file_2441
GRANT USAGE ON SCHEMA tpcds TO joe ;

-- 来源: 2441_file_2441
GRANT SELECT ON TABLE tpcds . web_returns to joe ;

-- 来源: 2441_file_2441
GRANT USAGE ON SCHEMA tpcds TO lily ;

-- 来源: 2441_file_2441
GRANT SELECT ON TABLE tpcds . web_returns to lily ;

-- 来源: 2441_file_2441
GRANT lily to joe ;

-- 来源: 2442_file_2442
GRANT SELECT ON all_data TO alice , bob , peter ;

-- 来源: 2449_file_2449
GRANT ALL PRIVILEGES TO joe;

-- 来源: 2452_file_2452
GRANT CREATE ON TABLESPACE fastspace TO jack ;

-- 来源: 2461_schema
GRANT USAGE ON schema myschema TO jack ;

--将tpcds下由jack创建的所有表的插入权限授予用户jack。
-- 来源: 2888_ALTER DEFAULT PRIVILEGES
GRANT USAGE,CREATE ON SCHEMA tpcds TO jack;

--给新用户赋权限
-- 来源: 2914_ALTER SYNONYM
GRANT ALL ON SCHEMA sysadmin TO u1;

-- 来源: 2957_CREATE MASKING POLICY
GRANT ALL PRIVILEGES TO dev_mask ;

-- 来源: 2957_CREATE MASKING POLICY
GRANT ALL PRIVILEGES TO bob_mask ;

--将表all_data的读取权限赋予alice和bob用户。
-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
GRANT SELECT ON all_data TO alice, bob;

-- 来源: 3048_GRANT
GRANT ALL PRIVILEGES TO joe;

-- 来源: 3048_GRANT
GRANT USAGE ON SCHEMA tpcds TO joe;

-- 来源: 3048_GRANT
GRANT ALL PRIVILEGES ON tpcds. reason TO joe;

-- 来源: 3048_GRANT
GRANT SELECT (r_reason_sk,r_reason_id,r_reason_desc),UPDATE (r_reason_desc) ON tpcds. reason TO joe;

-- 来源: 3048_GRANT
GRANT SELECT (r_reason_sk, r_reason_id) ON tpcds. reason TO joe WITH GRANT OPTION;

-- 来源: 3048_GRANT
GRANT CREATE,CONNECT ON DATABASE testdb TO joe WITH GRANT OPTION;

-- 来源: 3048_GRANT
GRANT USAGE,CREATE ON SCHEMA tpcds TO tpcds_manager;

-- 来源: 3048_GRANT
GRANT ALL ON TABLESPACE tpcds_tbspc TO joe;

-- 来源: 3048_GRANT
GRANT ALTER ON FUNCTION tpcds.fun1() TO joe;

-- 来源: 3048_GRANT
GRANT joe TO manager WITH ADMIN OPTION;

-- 来源: 3048_GRANT
GRANT manager TO senior_manager;

-- 来源: 4027_file_4027
grant plsql_rollback1 to plsql_rollback2;

-- 来源: 4288_file_4288
GRANT SELECT , INSERT ON tbl TO user1 , user2 ;

-- 来源: 4515_file_4515
GRANT SELECT , INSERT ON tbl TO user1 , user2 ;

-- 来源: 743_file_743
GRANT USAGE ON SCHEMA tpcds TO joe ;

-- 来源: 743_file_743
GRANT SELECT ON TABLE tpcds . web_returns to joe ;

-- 来源: 743_file_743
GRANT USAGE ON SCHEMA tpcds TO lily ;

-- 来源: 743_file_743
GRANT SELECT ON TABLE tpcds . web_returns to lily ;

-- 来源: 743_file_743
GRANT lily to joe ;

-- 来源: 744_file_744
GRANT SELECT ON all_data TO alice , bob , peter ;

-- 来源: 751_file_751
GRANT ALL PRIVILEGES TO joe;

-- 来源: 754_file_754
GRANT CREATE ON TABLESPACE fastspace TO jack ;

-- 来源: 763_schema
GRANT USAGE ON schema myschema TO jack ;

