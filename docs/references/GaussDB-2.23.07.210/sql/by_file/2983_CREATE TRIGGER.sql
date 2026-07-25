-- 来源: 2983_CREATE TRIGGER.txt
-- SQL 数量: 28

CREATE TABLE test_trigger_src_tbl(id1 INT, id2 INT, id3 INT);

CREATE TABLE test_trigger_des_tbl(id1 INT, id2 INT, id3 INT);

--创建触发器函数
CREATE OR REPLACE FUNCTION tri_insert_func() RETURNS TRIGGER AS $$ DECLARE BEGIN INSERT INTO test_trigger_des_tbl VALUES(NEW.id1, NEW.id2, NEW.id3);

CREATE OR REPLACE FUNCTION tri_update_func() RETURNS TRIGGER AS $$ DECLARE BEGIN UPDATE test_trigger_des_tbl SET id3 = NEW.id3 WHERE id1=OLD.id1;

CREATE OR REPLACE FUNCTION TRI_DELETE_FUNC() RETURNS TRIGGER AS $$ DECLARE BEGIN DELETE FROM test_trigger_des_tbl WHERE id1=OLD.id1;

--创建INSERT触发器
CREATE TRIGGER insert_trigger BEFORE INSERT ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_insert_func();

--创建UPDATE触发器
CREATE TRIGGER update_trigger AFTER UPDATE ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_update_func();

--创建DELETE触发器
CREATE TRIGGER delete_trigger BEFORE DELETE ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_delete_func();

--执行INSERT触发事件并检查触发结果
INSERT INTO test_trigger_src_tbl VALUES(100,200,300);

SELECT * FROM test_trigger_src_tbl;

SELECT * FROM test_trigger_des_tbl;

--执行UPDATE触发事件并检查触发结果
UPDATE test_trigger_src_tbl SET id3=400 WHERE id1=100;

SELECT * FROM test_trigger_src_tbl;

SELECT * FROM test_trigger_des_tbl;

--执行DELETE触发事件并检查触发结果
DELETE FROM test_trigger_src_tbl WHERE id1=100;

SELECT * FROM test_trigger_src_tbl;

SELECT * FROM test_trigger_des_tbl;

--修改触发器
ALTER TRIGGER delete_trigger ON test_trigger_src_tbl RENAME TO delete_trigger_renamed;

--禁用insert_trigger触发器
ALTER TABLE test_trigger_src_tbl DISABLE TRIGGER insert_trigger;

--禁用当前表上所有触发器
ALTER TABLE test_trigger_src_tbl DISABLE TRIGGER ALL;

--删除触发器
DROP TRIGGER insert_trigger ON test_trigger_src_tbl;

DROP TRIGGER update_trigger ON test_trigger_src_tbl;

DROP TRIGGER delete_trigger_renamed ON test_trigger_src_tbl;

--删除函数。
DROP FUNCTION tri_insert_func();

DROP FUNCTION tri_update_func();

DROP FUNCTION tri_delete_func();

--删除源表及触发表。
DROP TABLE test_trigger_src_tbl;

DROP TABLE test_trigger_des_tbl;

