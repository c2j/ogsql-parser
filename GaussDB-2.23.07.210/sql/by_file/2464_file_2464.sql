-- 来源: 2464_file_2464.txt
-- SQL 数量: 4

CREATE OR REPLACE VIEW MyView AS SELECT * FROM tpcds . web_returns WHERE trunc ( wr_refunded_cash ) > 10000 ;

SELECT * FROM MyView ;

\ d + MyView View "PG_CATALOG.MyView" Column | Type | Modifiers | Storage | Description ----------+-----------------------+-----------+----------+------------- USERNAME | CHARACTER VARYING ( 64 ) | | extended | View definition : SELECT PG_AUTHID . ROLNAME :: CHARACTER VARYING ( 64 ) AS USERNAME FROM PG_AUTHID ;

DROP VIEW MyView ;

