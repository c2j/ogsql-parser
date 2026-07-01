-- description: procedure without transactions should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE PROCEDURE proc_select()
LANGUAGE plpgsql
AS $$
BEGIN
    PERFORM * FROM t1;
END;
$$;
