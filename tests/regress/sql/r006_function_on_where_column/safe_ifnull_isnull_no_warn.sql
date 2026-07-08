-- description: IFNULL and ISNULL are safe functions on column, should not trigger R006
-- nowarn: R006
SELECT * FROM t WHERE IFNULL(col, 0) = 1;
