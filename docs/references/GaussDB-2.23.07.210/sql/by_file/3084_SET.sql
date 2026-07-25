-- 来源: 3084_SET.txt
-- SQL 数量: 6

SET search_path TO tpcds, public;

--把日期时间风格设置为传统的 POSTGRES 风格(日在月前)。
SET datestyle TO postgres,dmy;

--SET自定义用户变量的功能。
CREATE DATABASE user_var dbcompatibility 'b';

--删除数据库。
DROP DATABASE user_var;

CREATE DATABASE test_set dbcompatibility 'B';

--删除数据库。
DROP DATABASE test_set;

