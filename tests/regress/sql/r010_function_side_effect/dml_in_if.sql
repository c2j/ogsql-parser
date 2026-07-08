-- description: function with DML in IF without transaction should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION fn_insert_in_if() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    IF 1 > 0 THEN
        INSERT INTO t1 VALUES (1);
    END IF;
END;
$$;
