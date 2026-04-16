-- 来源: 1142_file_1142.txt
-- SQL 数量: 4

CREATE TABLE tpcds . value_storage_t1 ( VS_COL1 CHARACTER ( 20 ) ) DISTRIBUTE BY HASH ( VS_COL1 );

INSERT INTO tpcds . value_storage_t1 VALUES ( 'abcdef' );

SELECT VS_COL1 , octet_length ( VS_COL1 ) FROM tpcds . value_storage_t1 ;

DROP TABLE tpcds . value_storage_t1 ;

