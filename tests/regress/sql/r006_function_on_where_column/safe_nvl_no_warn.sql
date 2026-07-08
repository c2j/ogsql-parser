-- description: NVL and NVL2 are safe functions on column, should not trigger R006
-- nowarn: R006
SELECT * FROM t WHERE NVL(name, 'x') = 'abc';
