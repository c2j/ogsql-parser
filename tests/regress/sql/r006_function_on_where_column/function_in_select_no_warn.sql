-- description: function in SELECT list (not WHERE) should not trigger R006
-- nowarn: R006
SELECT LENGTH(name) FROM t WHERE id = 1;
