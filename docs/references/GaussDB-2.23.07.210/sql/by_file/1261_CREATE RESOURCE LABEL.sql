-- 来源: 1261_CREATE RESOURCE LABEL.txt
-- SQL 数量: 14

CREATE TABLE tb_for_label ( col1 text , col2 text , col3 text );

CREATE SCHEMA schema_for_label ;

CREATE VIEW view_for_label AS SELECT 1 ;

CREATE FUNCTION func_for_label RETURNS TEXT AS $$ SELECT col1 FROM tb_for_label ;

CREATE RESOURCE LABEL IF NOT EXISTS table_label add TABLE ( public . tb_for_label );

CREATE RESOURCE LABEL IF NOT EXISTS column_label add COLUMN ( public . tb_for_label . col1 );

CREATE RESOURCE LABEL IF NOT EXISTS schema_label add SCHEMA ( schema_for_label );

CREATE RESOURCE LABEL IF NOT EXISTS view_label add VIEW ( view_for_label );

CREATE RESOURCE LABEL IF NOT EXISTS func_label add FUNCTION ( func_for_label );

DROP RESOURCE LABEL func_label , view_label , schema_label , column_label , table_label ;

DROP FUNCTION func_for_label ;

DROP VIEW view_for_label ;

DROP SCHEMA schema_for_label ;

DROP TABLE tb_for_label ;

