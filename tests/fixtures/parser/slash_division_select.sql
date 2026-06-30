-- Issue: slash-division-operator
-- Description: / is treated as statement terminator in SELECT list, DDL, subqueries, and PL/pgSQL
-- Expect: parse
-- Split: semicolon

SELECT a / 1000 FROM tab;

SELECT a / 1000, b / 200 FROM tab;

SELECT 1 / 2 FROM tab;

SELECT 1.5 / 2.0 FROM tab;

SELECT a / -1000 FROM tab;

SELECT a / (1000 + 1) FROM tab;

SELECT c2 / c1 c FROM tab;

SELECT c2 / c1 AS c FROM tab;

SELECT a / b c FROM tab;
