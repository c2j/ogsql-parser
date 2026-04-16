-- 来源: 2441_file_2441.txt
-- SQL 数量: 6

GRANT USAGE ON SCHEMA tpcds TO joe ;

GRANT SELECT ON TABLE tpcds . web_returns to joe ;

CREATE ROLE lily WITH CREATEDB PASSWORD "********" ;

GRANT USAGE ON SCHEMA tpcds TO lily ;

GRANT SELECT ON TABLE tpcds . web_returns to lily ;

GRANT lily to joe ;

