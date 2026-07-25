-- 来源: 1050_file_1050.txt
-- SQL 数量: 17

CREATE TABLE varchar_maxlength_test1 (a int, b varchar, c int) DISTRIBUTE BY HASH (a);

-- varchar为1073741728，超过规定长度，插入失败
insert into varchar_maxlength_test1 values(1, repeat('a', 1073741728), 1);

-- varchar为1073741727，长度符合要求，插入成功
insert into varchar_maxlength_test1 values(1, repeat('a', 1073741727), 1);

-- 创建表，表中仅varchar一列，根据计算规则，varchar最大存储长度为1GB-85-4=
CREATE TABLE varchar_maxlength_test2 (a varchar) DISTRIBUTE BY HASH (a);

CREATE TABLE char_type_t1 ( CT_COL1 CHARACTER(4) )DISTRIBUTE BY HASH (CT_COL1);

--插入数据。
INSERT INTO char_type_t1 VALUES ('ok');

--查询表中的数据。
SELECT ct_col1, char_length(ct_col1) FROM char_type_t1;

--删除表。
DROP TABLE char_type_t1;

CREATE TABLE char_type_t2 ( CT_COL1 VARCHAR ( 5 ) ) DISTRIBUTE BY HASH ( CT_COL1 );

INSERT INTO char_type_t2 VALUES ( 'ok' );

INSERT INTO char_type_t2 VALUES ( 'good' );

INSERT INTO char_type_t2 VALUES ( 'too long' );

INSERT INTO char_type_t2 VALUES ( 'too long' :: varchar ( 5 ));

SELECT ct_col1 , char_length ( ct_col1 ) FROM char_type_t2 ;

DROP TABLE char_type_t2 ;

create database gaussdb_m with dbcompatibility 'MYSQL' ;

\ c gaussdb_m -- 设置兼容版本控制参数 gaussdb_m =# set b_format_version = '5.7' ;

