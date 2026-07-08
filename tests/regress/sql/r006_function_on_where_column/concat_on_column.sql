-- description: string concatenation on column in WHERE should trigger R006
-- warn: R006
SELECT * FROM t WHERE first_name || last_name = 'JohnDoe';
