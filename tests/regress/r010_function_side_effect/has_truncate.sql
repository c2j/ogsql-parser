-- description: function with TRUNCATE without transaction should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION fn_truncate() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    TRUNCATE TABLE t1;
END;
$$;
