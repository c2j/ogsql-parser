-- description: function with PERFORM SELECT should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION fn_perform() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    PERFORM * FROM t1 WHERE id = 1;
END;
$$;
