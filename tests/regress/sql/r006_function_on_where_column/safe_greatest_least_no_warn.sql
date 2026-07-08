-- description: GREATEST and LEAST are safe functions on column, should not trigger R006
-- nowarn: R006
SELECT * FROM t WHERE GREATEST(a, b) > 10 AND LEAST(x, y) < 5;
