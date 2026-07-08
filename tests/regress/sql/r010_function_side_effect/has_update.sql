-- description: function with UPDATE without transaction should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION fn_update() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    UPDATE t1 SET name = 'x';
END;
$$;
