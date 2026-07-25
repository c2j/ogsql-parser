-- 来源: 766_file_766.txt
-- SQL 数量: 6

CREATE OR REPLACE VIEW MyView AS SELECT * FROM tpcds . web_returns WHERE trunc ( wr_refunded_cash ) > 10000 ;

SELECT * FROM MyView ;

SELECT * FROM my_views ;

SELECT * FROM adm_views ;

\ d + MyView View "PG_CATALOG.MyView" Column | Type | Modifiers | Storage | Description ----------+-----------------------+-----------+----------+------------- USERNAME | CHARACTER VARYING ( 64 ) | | extended | View definition : SELECT PG_AUTHID . ROLNAME :: CHARACTER VARYING ( 64 ) AS USERNAME FROM PG_AUTHID ;

DROP VIEW MyView ;

