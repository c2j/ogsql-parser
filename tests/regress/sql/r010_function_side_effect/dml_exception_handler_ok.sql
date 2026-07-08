-- description: function with DML in exception handler (no transaction) should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION fn_dml_exception() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO t1 VALUES (1);
EXCEPTION
    WHEN OTHERS THEN
        INSERT INTO t_err VALUES (SQLERRM);
END;
$$;
