-- 来源: 1239_CREATE AUDIT POLICY.txt
-- SQL 数量: 13

CREATE USER dev_audit PASSWORD '*********' ;

CREATE USER bob_audit PASSWORD '*********' ;

CREATE TABLE tb_for_audit ( col1 text , col2 text , col3 text );

CREATE RESOURCE LABEL adt_lb0 ADD TABLE ( tb_for_audit );

CREATE AUDIT POLICY adt1 PRIVILEGES CREATE ;

CREATE AUDIT POLICY adt2 ACCESS SELECT ;

CREATE AUDIT POLICY adt3 PRIVILEGES CREATE ON LABEL ( adt_lb0 ) FILTER ON ROLES ( dev_audit , bob_audit );

CREATE AUDIT POLICY adt4 ACCESS SELECT ON LABEL ( adt_lb0 ), INSERT ON LABEL ( adt_lb0 ), DELETE FILTER ON ROLES ( dev_audit , bob_audit ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' );

ALTER AUDIT POLICY adt4 REMOVE ACCESS ( SELECT ON LABEL ( adt_lb0 ));

DROP AUDIT POLICY adt1 , adt2 , adt3 , adt4 ;

DROP RESOURCE LABEL adt_lb0 ;

DROP TABLE tb_for_audit ;

DROP USER dev_audit , bob_audit ;

