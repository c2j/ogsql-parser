-- description: function with INSERT without transaction should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION fn_insert() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO t1 VALUES (1);
END;
$$;
