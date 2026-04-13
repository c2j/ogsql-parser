-- 来源: 1143_UNIONCASE.txt
-- SQL 数量: 9

SELECT text 'a' AS "text" UNION SELECT 'b' ;

SELECT 1 . 2 AS "numeric" UNION SELECT 1 ;

SELECT 1 AS "real" UNION SELECT CAST ( '2.2' AS REAL );

CREATE DATABASE oracle_1 dbcompatibility = 'ORA';

--在TD模式下，创建TD兼容模式的数据库td_1。
CREATE DATABASE td_1 dbcompatibility = 'TD';

--删除Oracle和TD模式的数据库。
DROP DATABASE oracle_1;

DROP DATABASE td_1;

CREATE DATABASE ora_1 dbcompatibility = 'A';

--删除ORA模式的数据库。
DROP DATABASE ora_1;

