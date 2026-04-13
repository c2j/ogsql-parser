-- 来源: 4320_file_4320.txt
-- SQL 数量: 10

CREATE INDEX tpcds_web_returns_p2_index1 ON web_returns_p2 (ca_address_id) LOCAL;

CREATE INDEX tpcds_web_returns_p2_index2 ON web_returns_p2 (ca_address_sk) LOCAL ( PARTITION web_returns_p2_P1_index, PARTITION web_returns_p2_P2_index TABLESPACE example3, PARTITION web_returns_p2_P3_index TABLESPACE example4, PARTITION web_returns_p2_P4_index, PARTITION web_returns_p2_P5_index, PARTITION web_returns_p2_P6_index, PARTITION web_returns_p2_P7_index, PARTITION web_returns_p2_P8_index ) TABLESPACE example2;

CREATE INDEX tpcds_web_returns_p2_global_index ON web_returns_p2 (ca_street_number) GLOBAL;

CREATE INDEX tpcds_web_returns_for_p1 ON web_returns_p2 (ca_address_id) LOCAL(partition ind_part for p1);

CREATE INDEX tpcds_web_returns_for_p2 ON web_returns_p2 (ca_address_id) LOCAL(partition ind_part for (5000));

ALTER INDEX tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P2_index TABLESPACE example1;

ALTER INDEX tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P3_index TABLESPACE example2;

ALTER INDEX tpcds_web_returns_p2_index2 RENAME PARTITION web_returns_p2_P8_index TO web_returns_p2_P8_index_new;

SELECT RELNAME FROM PG_CLASS WHERE RELKIND='i' or RELKIND='I';

DROP INDEX tpcds_web_returns_p2_index1;

