-- 来源: 4514_file_4514.txt
-- SQL 数量: 5

SECURITY LABEL ON USER user1 is 'label1' ;

SECURITY LABEL ON USER user2 is 'label3' ;

SECURITY LABEL ON TABLE tbl is 'label2' ;

SELECT * FROM pg_seclabels ;

SELECT * FROM pg_seclabels;

