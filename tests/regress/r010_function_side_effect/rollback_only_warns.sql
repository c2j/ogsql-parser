-- description: function with only ROLLBACK (no DML) should trigger R010
-- warn: R010
CREATE OR REPLACE FUNCTION fn_rollback_only() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    ROLLBACK;
END;
$$;
