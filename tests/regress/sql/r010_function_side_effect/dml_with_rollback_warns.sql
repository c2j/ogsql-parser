-- description: function with DML + ROLLBACK should trigger R010
-- warn: R010
CREATE OR REPLACE FUNCTION fn_dml_rollback() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO t1 VALUES (1);
    ROLLBACK;
END;
$$;
