-- description: function with ROLLBACK TO SAVEPOINT should trigger R010
-- warn: R010
CREATE OR REPLACE FUNCTION fn_rollback_sp() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    ROLLBACK TO SAVEPOINT sp1;
END;
$$;
