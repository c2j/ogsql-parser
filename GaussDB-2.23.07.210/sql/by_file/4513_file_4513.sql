-- 来源: 4513_file_4513.txt
-- SQL 数量: 8

CREATE SECURITY LABEL label1 'L1:G2,G4' ;

CREATE SECURITY LABEL label2 'L2:G2-G4' ;

CREATE SECURITY LABEL label3 'L3:G1-G5' ;

SELECT * FROM gs_security_label ;

SELECT * FROM gs_security_label;

DROP SECURITY LABEL label1 ;

DROP SECURITY LABEL label2 ;

DROP SECURITY LABEL label3 ;

