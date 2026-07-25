-- 来源: 2957_CREATE MASKING POLICY.txt
-- SQL 数量: 32

CREATE USER dev_mask PASSWORD '********' ;

CREATE USER bob_mask PASSWORD '********' ;

CREATE TABLE tb_for_masking ( idx int , col1 text , col2 text , col3 text , col4 text , col5 text , col6 text , col7 text , col8 text );

INSERT INTO tb_for_masking VALUES ( 1 , '9876543210' , 'usr321usr' , 'abc@huawei.com' , 'abc@huawei.com' , '1234-4567-7890-0123' , 'abcdef 123456 ui 323 jsfd321 j3k2l3' , '4880-9898-4545-2525' , 'this is a llt case' );

INSERT INTO tb_for_masking VALUES ( 2 , '0123456789' , 'lltc123llt' , 'abc@gmail.com' , 'abc@gmail.com' , '9876-5432-1012-3456' , '1234 abcd ef 56 gh78ijk90lm' , '4856-7654-1234-9865' , 'this,is.a!LLT?case' );

CREATE RESOURCE LABEL mask_lb1 ADD COLUMN ( tb_for_masking . col1 );

CREATE RESOURCE LABEL mask_lb2 ADD COLUMN ( tb_for_masking . col2 );

CREATE RESOURCE LABEL mask_lb3 ADD COLUMN ( tb_for_masking . col3 );

CREATE RESOURCE LABEL mask_lb4 ADD COLUMN ( tb_for_masking . col4 );

CREATE RESOURCE LABEL mask_lb5 ADD COLUMN ( tb_for_masking . col5 );

CREATE RESOURCE LABEL mask_lb6 ADD COLUMN ( tb_for_masking . col6 );

CREATE RESOURCE LABEL mask_lb7 ADD COLUMN ( tb_for_masking . col7 );

CREATE RESOURCE LABEL mask_lb8 ADD COLUMN ( tb_for_masking . col8 );

CREATE MASKING POLICY maskpol1 maskall ON LABEL ( mask_lb1 );

CREATE MASKING POLICY maskpol2 alldigitsmasking ON LABEL ( mask_lb2 );

CREATE MASKING POLICY maskpol3 basicemailmasking ON LABEL ( mask_lb3 );

CREATE MASKING POLICY maskpol4 fullemailmasking ON LABEL ( mask_lb4 );

CREATE MASKING POLICY maskpol5 creditcardmasking ON LABEL ( mask_lb5 );

CREATE MASKING POLICY maskpol6 shufflemasking ON LABEL ( mask_lb6 );

CREATE MASKING POLICY maskpol7 regexpmasking ( '[\d+]' , '*' , 2 , 9 ) ON LABEL ( mask_lb7 );

CREATE MASKING POLICY maskpol8 randommasking ON LABEL ( mask_lb8 ) FILTER ON ROLES ( dev_mask , bob_mask ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' );

SELECT * FROM tb_for_masking ;

GRANT ALL PRIVILEGES TO dev_mask ;

GRANT ALL PRIVILEGES TO bob_mask ;

SET role dev_mask PASSWORD '********' ;

SELECT col8 FROM tb_for_masking ;

SET role bob_mask PASSWORD '********' ;

SELECT col8 FROM tb_for_masking ;

DROP MASKING POLICY maskpol1 , maskpol2 , maskpol3 , maskpol4 , maskpol5 , maskpol6 , maskpol7 , maskpol8 ;

DROP RESOURCE LABEL mask_lb1 , mask_lb2 , mask_lb3 , mask_lb4 , mask_lb5 , mask_lb6 , mask_lb7 , mask_lb8 ;

DROP TABLE tb_for_masking ;

DROP USER dev_mask , bob_mask ;

