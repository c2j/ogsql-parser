-- description: plain column reference in WHERE without function should not trigger R006
-- nowarn: R006
SELECT * FROM t WHERE name = 'abc';
