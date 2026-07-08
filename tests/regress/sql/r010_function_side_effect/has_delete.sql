-- description: function with DELETE without transaction should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION fn_delete() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    DELETE FROM t1;
END;
$$;
