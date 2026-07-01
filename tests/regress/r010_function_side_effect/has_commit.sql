-- description: function with COMMIT should trigger R010
-- warn: R010
CREATE OR REPLACE FUNCTION fn_commit() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO t1 VALUES (1);
    COMMIT;
END;
$$;
