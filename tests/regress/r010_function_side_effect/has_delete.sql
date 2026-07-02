-- description: function with DELETE should trigger R010
-- warn: R010
CREATE OR REPLACE FUNCTION fn_delete() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    DELETE FROM t1;
END;
$$;
