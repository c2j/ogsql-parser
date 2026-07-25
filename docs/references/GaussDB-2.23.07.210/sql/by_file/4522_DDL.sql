-- 来源: 4522_DDL.txt
-- SQL 数量: 64

CREATE TABLE test_create_table_partition2 (c1 INT, c2 INT) PARTITION BY RANGE (c2) ( PARTITION p1 START(1) END(1000) EVERY(200) , PARTITION p2 END(2000), PARTITION p3 START(2000) END(2500), PARTITION p4 START(2500), PARTITION p5 START(3000) END(5000) EVERY(1000) );

CREATE TABLE test_create_table_partition2 (c1 INT, c2 INT) PARTITION BY RANGE (c2) ( PARTITION p1_0 VALUES LESS THAN ('1'), PARTITION p1_1 VALUES LESS THAN ('201'), PARTITION p1_2 VALUES LESS THAN ('401'), PARTITION p1_3 VALUES LESS THAN ('601'), PARTITION p1_4 VALUES LESS THAN ('801'), PARTITION p1_5 VALUES LESS THAN ('1000'), PARTITION p2 VALUES LESS THAN ('2000'), PARTITION p3 VALUES LESS THAN ('2500'), PARTITION p4 VALUES LESS THAN ('3000'), PARTITION p5_1 VALUES LESS THAN ('4000'), PARTITION p5_2 VALUES LESS THAN ('5000') );

CREATE TABLE IF NOT EXISTS tb5 (c1 int,c2 int) with (ORIENTATION=ROW, STORAGE_TYPE=USTORE);

ALTER TABLE IF EXISTS tb5 * ADD COLUMN IF NOT EXISTS c2 char(5) after c1;

ALTER TABLE IF EXISTS public.tb5 ADD COLUMN IF NOT EXISTS c2 pg_catalog.int4 AFTER c1;

ALTER TABLE IF EXISTS tb5 * ADD COLUMN IF NOT EXISTS c2 char(5) after c1, ADD COLUMN IF NOT EXISTS c3 char(5) after c1;

ALTER TABLE IF EXISTS public.tb5 ADD COLUMN IF NOT EXISTS c2 pg_catalog.int4 AFTER c1, ADD COLUMN IF NOT EXISTS c3 pg_catalog.bpchar(5) AFTER c1;

ALTER TABLE IF EXISTS tb5 * ADD COLUMN c2 char(5) after c1, ADD COLUMN IF NOT EXISTS c4 int after c1;

ALTER TABLE tbl_28 ADD COLUMN b1 TIMESTAMP DEFAULT NOW();

ALTER TABLE tbl_28 ADD COLUMN b2 INT DEFAULT RANDOM();

ALTER TABLE tbl_28 ADD COLUMN b3 INT DEFAULT ABS(1);

CREATE TABLE IF NOT EXISTS tb1 (c1 time without time zone ON UPDATE CURRENT_TIMESTAMP) with (ORIENTATION=ROW, STORAGE_TYPE=USTORE);

-- B兼容模式下修改表，列字段添加ON UPDATE事件
ALTER TABLE IF EXISTS ONLY tb2 MODIFY COLUMN c2 time without time zone ON UPDATE LOCALTIMESTAMP;

CREATE TABLE IF NOT EXISTS public.tb1 (c1 TIME) WITH (orientation = 'row', storage_type = 'ustore', compression = 'no') NOCOMPRESS;

ALTER TABLE IF EXISTS ONLY public.tb2 MODIFY COLUMN c2 TIME;

CREATE TABLE IF NOT EXISTS tb3 (c1 int) with (storage_type=USTORE,ORIENTATION=ROW) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 7 day OF NO MODIFICATION;

CREATE TABLE IF NOT EXISTS public.tb3 (c1 pg_catalog.int4) WITH (storage_type = 'ustore', orientation = 'row', compression = 'no') NOCOMPRESS;

CREATE TABLE IF NOT EXISTS tb6 (c1 integer comment 'Mysql兼容注释语法') with (ORIENTATION=ROW, STORAGE_TYPE=USTORE);

CREATE TABLE IF NOT EXISTS public.tb6 (c1 pg_catalog.int4) WITH (storage_type = 'ustore', orientation = 'row', compression = 'no') NOCOMPRESS;

BEGIN;

GAINT ALL PRIVILEGES to u01;

INSERT INTO test1(col1) values(1);

COMMIT;

-- 只反解析第一句和第三句SQL语句
BEGIN;

CREATE TABLE mix_tran_t4(id int);

INSERT INTO mix_tran_t4 VALUES(111);

CREATE TABLE mix_tran_t5(id int);

COMMIT;

-- 只反解析第一句和第二句SQL语句
BEGIN;

INSERT INTO mix_tran_t4 VALUES(111);

CREATE TABLE mix_tran_t6(id int);

INSERT INTO mix_tran_t4 VALUES(111);

COMMIT;

-- 全反解析
BEGIN;

INSERT INTO mix_tran_t4 VALUES(111);

CREATE TABLE mix_tran_t7(id int);

CREATE TABLE mix_tran_t8(id int);

COMMIT;

-- 只反解析第一句和第三句SQL语句
BEGIN;

CREATE TABLE mix_tran_t7(id int);

CREATE TYPE compfoo AS (f1 int, f2 text);

CREATE TABLE mix_tran_t8(id int);

COMMIT;

-- 全反解析
BEGIN;

INSERT INTO mix_tran_t4 VALUES(111);

INSERT INTO mix_tran_t4 VALUES(111);

INSERT INTO mix_tran_t4 VALUES(111);

COMMIT;

-- 只反解析第一句SQL语句
BEGIN;

INSERT INTO mix_tran_t4 VALUES(111);

CREATE TYPE compfoo AS (f1 int, f2 text);

INSERT INTO mix_tran_t4 VALUES(111);

COMMIT;

-- 只反解析第一句和第三句SQL语句
BEGIN;

INSERT INTO mix_tran_t4 VALUES(111);

CREATE TYPE compfoo AS (f1 int, f2 text);

CREATE TABLE mix_tran_t9(id int);

COMMIT;

gs_guc set -Z datanode -D $node_dir -c "wal_level = logical" 其中，$node_dir为数据库节点路径，用户可根据实际情况替换。 使用如下命令连接数据库。

gsql -d gaussdb -p 20000 -r 其中，gaussdb为需要连接的数据库名称，20000为数据库端口号，用户可根据实际情况替换。 创建名称为slot1的逻辑复制槽。 1 2 3 4

SELECT * FROM pg_create_logical_replication_slot ( 'slot1' , 'mppdb_decoding' );

CREATE OR REPLACE PACKAGE ldp_pkg1 IS var1 int : = 1 ;

SELECT data FROM pg_logical_slot_peek_changes ( 'ldp_ddl_replica_slot' , NULL , NULL , 'enable-ddl-decoding' , 'true' , 'enable-ddl-json-format' , 'false' ) WHERE data not like 'BEGIN%' AND data not like 'COMMIT%' AND data not like '%dbe_pldeveloper.gs_source%' ;

SELECT * FROM pg_drop_replication_slot ( 'slot1' );

