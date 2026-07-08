-- description: function with SAVEPOINT should trigger R010
-- warn: R010
CREATE OR REPLACE FUNCTION fn_savepoint() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    SAVEPOINT sp1;
    INSERT INTO t VALUES (1);
    RELEASE SAVEPOINT sp1;
END;
$$;
