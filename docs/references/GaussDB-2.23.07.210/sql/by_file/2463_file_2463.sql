-- 来源: 2463_file_2463.txt
-- SQL 数量: 19

CREATE INDEX tpcds_web_returns_p2_index1 ON tpcds . web_returns_p2 ( ca_address_id ) LOCAL ;

CREATE INDEX tpcds_web_returns_p2_index2 ON tpcds . web_returns_p2 ( ca_address_sk ) LOCAL ( PARTITION web_returns_p2_P1_index , PARTITION web_returns_p2_P2_index TABLESPACE example3 , PARTITION web_returns_p2_P3_index TABLESPACE example4 , PARTITION web_returns_p2_P4_index , PARTITION web_returns_p2_P5_index , PARTITION web_returns_p2_P6_index , PARTITION web_returns_p2_P7_index , PARTITION web_returns_p2_P8_index ) TABLESPACE example2 ;

ALTER INDEX tpcds . tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P2_index TABLESPACE example1 ;

ALTER INDEX tpcds . tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P3_index TABLESPACE example2 ;

ALTER INDEX tpcds . tpcds_web_returns_p2_index2 RENAME PARTITION web_returns_p2_P8_index TO web_returns_p2_P8_index_new ;

SELECT RELNAME FROM PG_CLASS WHERE RELKIND = 'i' or RELKIND = 'I' ;

\ di + tpcds . tpcds_web_returns_p2_index2 删除索引 1

DROP INDEX tpcds . tpcds_web_returns_p2_index1 ;

DROP INDEX tpcds . tpcds_web_returns_p2_index2 ;

CREATE TABLE tpcds . customer_address_bak AS TABLE tpcds . customer_address ;

SELECT ca_address_sk FROM tpcds . customer_address_bak WHERE ca_address_sk = 14888 ;

CREATE INDEX index_wr_returned_date_sk ON tpcds . customer_address_bak ( ca_address_sk );

CREATE UNIQUE INDEX ds_ship_mode_t1_index1 ON tpcds. ship_mode_t1(SM_SHIP_MODE_SK);

SELECT ca_address_sk , ca_address_id FROM tpcds . customer_address_bak WHERE ca_address_sk = 5050 AND ca_street_number < 1000 ;

CREATE INDEX more_column_index ON tpcds . customer_address_bak ( ca_address_sk , ca_street_number );

CREATE INDEX part_index ON tpcds . customer_address_bak ( ca_address_sk ) WHERE ca_address_sk = 5050 ;

SELECT * FROM tpcds . customer_address_bak WHERE trunc ( ca_street_number ) < 1000 ;

CREATE INDEX para_index ON tpcds . customer_address_bak ( trunc ( ca_street_number ));

DROP TABLE tpcds . customer_address_bak ;

