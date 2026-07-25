-- 来源: 2761_XMLTYPE.txt
-- SQL 数量: 4

CREATE TABLE xmltypetest(id int, data xmltype);

INSERT INTO xmltypetest VALUES (1, '<ss/>');

INSERT INTO xmltypetest VALUES (2, '<xx/>');

SELECT * FROM xmltypetest ORDER BY 1;

