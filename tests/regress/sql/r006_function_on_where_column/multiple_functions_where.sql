-- description: multiple function calls on different columns in WHERE should trigger R006 multiple times
-- warn: R006
SELECT * FROM t WHERE LENGTH(a) > 5 AND UPPER(b) = 'X';
