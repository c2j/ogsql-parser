-- description: function with multiple DML statements (no transaction) should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION fn_multi_dml() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO t1 VALUES (1);
    UPDATE t2 SET name = 'x' WHERE id = 1;
    DELETE FROM t3 WHERE id = 1;
END;
$$;
