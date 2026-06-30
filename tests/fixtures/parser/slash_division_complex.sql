-- Issue: slash-division-operator
-- Description: / is treated as terminator in complex expression contexts
-- Expect: parse
-- Split: semicolon

SELECT * FROM tab WHERE a / 1000 > 1;

SELECT * FROM (SELECT a / 1000 AS r FROM tab) sub WHERE sub.r > 1;

WITH cte AS (SELECT a / 1000 AS r FROM tab) SELECT * FROM cte;

SELECT a / 1000, COUNT(*) FROM tab GROUP BY a / 1000 HAVING COUNT(*) / 1000 > 1;

SELECT * FROM tab ORDER BY a / 1000;

SELECT CASE WHEN a / 1000 > 1 THEN 'big' ELSE 'small' END FROM tab;

SELECT COUNT(*) / 100 FROM tab;

SELECT CAST(a AS FLOAT) / 1000 FROM tab;

SELECT a / b / c FROM tab;

SELECT (a + b) * (c / 1000) FROM tab;
