-- 来源: 1460_file_1460.txt
-- SQL 数量: 7

BEGIN FOR ROW_TRANS IN SELECT first_name FROM hr . staffs LOOP DBE_OUTPUT . PRINT_LINE ( ROW_TRANS . first_name );

Tom ANONYMOUS BLOCK EXECUTE --创建表

CREATE TABLE integerTable1 ( A INTEGER ) DISTRIBUTE BY hash ( A );

CREATE TABLE integerTable2 ( B INTEGER ) DISTRIBUTE BY hash ( B );

INSERT INTO integerTable2 VALUES ( 2 );

DROP TABLE integerTable1 ;

DROP TABLE integerTable2 ;

