-- description: COALESCE is a safe function on column, should not trigger R006
-- nowarn: R006
SELECT * FROM t WHERE COALESCE(name, 'default') = 'abc';
