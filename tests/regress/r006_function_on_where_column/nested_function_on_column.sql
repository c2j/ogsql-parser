-- description: nested function wrapping column in WHERE should trigger R006
-- warn: R006
SELECT * FROM t WHERE TRIM(LOWER(name)) = 'abc';
