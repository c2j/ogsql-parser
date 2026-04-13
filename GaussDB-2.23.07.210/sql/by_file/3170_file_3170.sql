-- 来源: 3170_file_3170.txt
-- SQL 数量: 8

BEGIN FOR ROW_TRANS IN SELECT first_name FROM hr . staffs LOOP DBE_OUTPUT . PRINT_LINE ( ROW_TRANS . first_name );

Tom ANONYMOUS BLOCK EXECUTE --创建表

CREATE TABLE integerTable1 ( A INTEGER );

CREATE TABLE integerTable2 ( B INTEGER );

INSERT INTO integerTable2 VALUES ( 2 );

DECLARE CURSOR C1 IS SELECT A FROM integerTable1 ;

DROP TABLE integerTable1 ;

DROP TABLE integerTable2 ;

