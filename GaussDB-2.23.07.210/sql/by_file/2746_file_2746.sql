-- 来源: 2746_file_2746.txt
-- SQL 数量: 16

CREATE TABLE varchar_maxlength_test1 (a int, b varchar, c int);

-- varchar为1073741728，超过规定长度，插入失败
insert into varchar_maxlength_test1 values(1, repeat('a', 1073741728), 1);

-- varchar为1073741727，长度符合要求，插入成功
insert into varchar_maxlength_test1 values(1, repeat('a', 1073741727), 1);

-- 创建表，表中仅varchar一列，根据计算规则，varchar最大存储长度为1GB-85-4=
CREATE TABLE varchar_maxlength_test2 (a varchar);

CREATE TABLE char_type_t1 ( CT_COL1 CHARACTER(4) );

--插入数据。
INSERT INTO char_type_t1 VALUES ('ok');

--查询表中的数据。
SELECT ct_col1, char_length(ct_col1) FROM char_type_t1;

--删除表。
DROP TABLE char_type_t1;

--创建表。
CREATE TABLE char_type_t2 ( CT_COL1 VARCHAR(5) );

--插入数据。
INSERT INTO char_type_t2 VALUES ('ok');

INSERT INTO char_type_t2 VALUES ('good');

--插入的数据长度超过类型规定的长度报错。
INSERT INTO char_type_t2 VALUES ('too long');

--明确类型的长度，超过数据类型长度后会自动截断。
INSERT INTO char_type_t2 VALUES ('too long'::varchar(5));

--查询数据。
SELECT ct_col1, char_length(ct_col1) FROM char_type_t2;

--删除数据。
DROP TABLE char_type_t2;

create database gaussdb_m with dbcompatibility 'b';

