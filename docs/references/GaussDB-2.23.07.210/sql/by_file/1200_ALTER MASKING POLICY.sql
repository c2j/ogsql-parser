-- 来源: 1200_ALTER MASKING POLICY.txt
-- SQL 数量: 17

CREATE USER dev_mask PASSWORD '********' ;

CREATE USER bob_mask PASSWORD '********' ;

CREATE TABLE tb_for_masking ( col1 text , col2 text , col3 text );

CREATE RESOURCE LABEL mask_lb1 ADD COLUMN ( tb_for_masking . col1 );

CREATE RESOURCE LABEL mask_lb2 ADD COLUMN ( tb_for_masking . col2 );

CREATE MASKING POLICY maskpol1 maskall ON LABEL ( mask_lb1 );

ALTER MASKING POLICY maskpol1 COMMENTS 'masking policy for tb_for_masking.col1' ;

ALTER MASKING POLICY maskpol1 ADD randommasking ON LABEL ( mask_lb2 );

ALTER MASKING POLICY maskpol1 REMOVE randommasking ON LABEL ( mask_lb2 );

ALTER MASKING POLICY maskpol1 MODIFY randommasking ON LABEL ( mask_lb1 );

ALTER MASKING POLICY maskpol1 MODIFY ( FILTER ON ROLES ( dev_mask , bob_mask ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' ));

ALTER MASKING POLICY maskpol1 DROP FILTER ;

ALTER MASKING POLICY maskpol1 DISABLE ;

DROP MASKING POLICY maskpol1 ;

DROP RESOURCE LABEL mask_lb1 , mask_lb2 ;

DROP TABLE tb_for_masking ;

DROP USER dev_mask , bob_mask ;

