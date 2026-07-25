-- 来源: 1276_CREATE TRIGGER.txt
-- SQL 数量: 28

CREATE TABLE test_trigger_src_tbl ( id1 INT , id2 INT , id3 INT );

CREATE TABLE test_trigger_des_tbl ( id1 INT , id2 INT , id3 INT );

CREATE OR REPLACE FUNCTION tri_insert_func () RETURNS TRIGGER AS $$ DECLARE BEGIN INSERT INTO test_trigger_des_tbl VALUES ( NEW . id1 , NEW . id2 , NEW . id3 );

CREATE OR REPLACE FUNCTION tri_update_func () RETURNS TRIGGER AS $$ DECLARE BEGIN UPDATE test_trigger_des_tbl SET id3 = NEW . id3 WHERE id1 = OLD . id1 ;

CREATE OR REPLACE FUNCTION tri_delete_func () RETURNS TRIGGER AS $$ DECLARE BEGIN DELETE FROM test_trigger_des_tbl WHERE id1 = OLD . id1 ;

CREATE TRIGGER insert_trigger BEFORE INSERT ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_insert_func ();

CREATE TRIGGER update_trigger AFTER UPDATE ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_update_func ();

CREATE TRIGGER delete_trigger BEFORE DELETE ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_delete_func ();

INSERT INTO test_trigger_src_tbl VALUES ( 100 , 200 , 300 );

SELECT * FROM test_trigger_src_tbl ;

SELECT * FROM test_trigger_des_tbl ;

UPDATE test_trigger_src_tbl SET id3 = 400 WHERE id1 = 100 ;

SELECT * FROM test_trigger_src_tbl ;

SELECT * FROM test_trigger_des_tbl ;

DELETE FROM test_trigger_src_tbl WHERE id1 = 100 ;

SELECT * FROM test_trigger_src_tbl ;

SELECT * FROM test_trigger_des_tbl ;

ALTER TRIGGER delete_trigger ON test_trigger_src_tbl RENAME TO delete_trigger_renamed ;

ALTER TABLE test_trigger_src_tbl DISABLE TRIGGER insert_trigger ;

ALTER TABLE test_trigger_src_tbl DISABLE TRIGGER ALL ;

DROP TRIGGER insert_trigger ON test_trigger_src_tbl ;

DROP TRIGGER update_trigger ON test_trigger_src_tbl ;

DROP TRIGGER delete_trigger_renamed ON test_trigger_src_tbl ;

DROP FUNCTION tri_insert_func ();

DROP FUNCTION tri_update_func ();

DROP FUNCTION tri_delete_func ();

DROP TABLE test_trigger_src_tbl ;

DROP TABLE test_trigger_des_tbl ;

