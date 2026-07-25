-- 来源: 1436_file_1436.txt
-- SQL 数量: 14

CREATE SCHEMA hr ;

SET CURRENT_SCHEMA = hr ;

CREATE TABLE staffs ( section_id INTEGER , salary INTEGER );

INSERT INTO staffs VALUES ( 30 , 10 );

INSERT INTO staffs VALUES ( 30 , 20 );

CREATE OR REPLACE PROCEDURE proc_staffs ( section NUMBER ( 6 ), salary_sum out NUMBER ( 8 , 2 ), staffs_count out INTEGER ) IS BEGIN SELECT sum ( salary ), count ( * ) INTO salary_sum , staffs_count FROM hr . staffs where section_id = section ;

CREATE OR REPLACE PROCEDURE proc_return AS v_num NUMBER ( 8 , 2 );

CALL proc_return ();

DROP PROCEDURE proc_staffs ;

DROP PROCEDURE proc_return ;

CREATE OR REPLACE FUNCTION func_return returns void language plpgsql AS $$ DECLARE v_num INTEGER : = 1 ;

CALL func_return ();

DROP FUNCTION func_return ;

DROP SCHEMA hr CASCADE ;

