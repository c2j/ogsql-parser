-- description: arithmetic operation on column in WHERE should trigger R006
-- warn: R006
SELECT * FROM t WHERE col + 1 > 5;
