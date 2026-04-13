-- 来源: 2760_XML.txt
-- SQL 数量: 6

CREATE TABLE xmltest ( id int, data xml );

INSERT INTO xmltest VALUES (1, 'one');

INSERT INTO xmltest VALUES (2, 'two');

SELECT * FROM xmltest ORDER BY 1;

SELECT xmlconcat(xmlcomment('hello'), xmlelement(NAME qux, 'xml'), xmlcomment('world'));

DROP TABLE xmltest;

