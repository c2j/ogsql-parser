-- 来源: 1439_file_1439.txt
-- SQL 数量: 4

CREATE TABLE sections_t1 ( section NUMBER ( 4 ) , section_name VARCHAR2 ( 30 ), manager_id NUMBER ( 6 ), place_id NUMBER ( 4 ) ) DISTRIBUTE BY hash ( manager_id );

DECLARE section NUMBER ( 4 ) : = 280 ;

SELECT * FROM sections_t1 ;

DROP TABLE sections_t1 ;

