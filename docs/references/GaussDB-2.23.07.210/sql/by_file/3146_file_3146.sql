-- 来源: 3146_file_3146.txt
-- SQL 数量: 10

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

