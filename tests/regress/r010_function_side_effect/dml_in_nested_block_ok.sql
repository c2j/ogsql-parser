-- description: function with DML in nested BEGIN block (no transaction) should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION fn_nested_dml() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    BEGIN
        INSERT INTO t1 VALUES (1);
    END;
END;
$$;
