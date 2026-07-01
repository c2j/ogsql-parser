-- description: function with DML nested in IF block should trigger R010
-- warn: R010
CREATE OR REPLACE FUNCTION fn_insert_in_if() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    IF 1 > 0 THEN
        INSERT INTO t1 VALUES (1);
    END IF;
END;
$$;
