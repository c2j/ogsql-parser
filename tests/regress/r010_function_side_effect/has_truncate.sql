-- description: function with TRUNCATE should trigger R010
-- warn: R010
CREATE OR REPLACE FUNCTION fn_truncate() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    TRUNCATE TABLE t1;
END;
$$;
