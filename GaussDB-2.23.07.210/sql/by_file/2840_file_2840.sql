-- 来源: 2840_file_2840.txt
-- SQL 数量: 4

CREATE TABLE tpcds . value_storage_t1 ( VS_COL1 CHARACTER ( 20 ) );

INSERT INTO tpcds . value_storage_t1 VALUES ( 'abcdef' );

SELECT VS_COL1 , octet_length ( VS_COL1 ) FROM tpcds . value_storage_t1 ;

DROP TABLE tpcds . value_storage_t1 ;

