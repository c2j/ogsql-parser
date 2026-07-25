-- 来源: 3099_UPDATE.txt
-- SQL 数量: 14

CREATE TABLE tbl_test1(id int, info varchar(10));

INSERT INTO tbl_test1 VALUES (1, 'A'), (2, 'B');

--修改tbl_test1表中所有数据的info列。
UPDATE tbl_test1 SET info = 'aa';

--查询tbl_test1表。
SELECT * FROM tbl_test1;

UPDATE tbl_test1 SET info = 'bb' WHERE id = 2;

--查询tbl_test1表。
SELECT * FROM tbl_test1;

UPDATE tbl_test1 SET info = 'ABC' WHERE id = 1 RETURNING info;

-- 删除tbl_test1表。
DROP TABLE tbl_test1;

CREATE TABLE test_grade ( sid int, --学号 name varchar(50), --姓名 score char, --成绩 examtime date, --考试时间 last_exam boolean --是否是最后一次考试 );

--插入数据。
INSERT INTO test_grade VALUES (1,'Scott','A','2008-07-08',1),(2,'Ben','D','2008-07-08',1),(3,'Jack','D','2008-07-08',1);

--查询。
SELECT * FROM test_grade;

--2008-08-25 Ben参加了补考,成绩为B，正常步骤需要先修改last_exam为否,然后插入2008-08-25这一天的成绩。
WITH old_exam AS ( UPDATE test_grade SET last_exam = 0 WHERE sid = 2 AND examtime = '2008-07-08' RETURNING sid, name ) INSERT INTO test_grade VALUES ( ( SELECT sid FROM old_exam ), ( SELECT name FROM old_exam ), 'B', '2008-08-25', 1 );

--查询。
SELECT * FROM test_grade;

--删除。
DROP TABLE test_grade;

