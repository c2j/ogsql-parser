-- description: function with INSERT should trigger R010
-- warn: R010
CREATE OR REPLACE FUNCTION fn_insert() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO t1 VALUES (1);
END;
$$;
