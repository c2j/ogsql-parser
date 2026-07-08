-- description: function calling function without side effects should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION fn_caller() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    PERFORM fn_select();
END;
$$;
