-- description: function with UPDATE should trigger R010
-- warn: R010
CREATE OR REPLACE FUNCTION fn_update() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    UPDATE t1 SET name = 'x';
END;
$$;
