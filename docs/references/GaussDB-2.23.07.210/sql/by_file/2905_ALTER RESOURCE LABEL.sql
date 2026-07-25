-- 来源: 2905_ALTER RESOURCE LABEL.txt
-- SQL 数量: 6

CREATE TABLE table_for_label ( col1 int , col2 text );

CREATE RESOURCE LABEL table_label ADD COLUMN ( table_for_label . col1 );

ALTER RESOURCE LABEL table_label ADD COLUMN ( table_for_label . col2 );

ALTER RESOURCE LABEL table_label REMOVE COLUMN ( table_for_label . col1 );

DROP RESOURCE LABEL table_label ;

DROP TABLE table_for_label ;

