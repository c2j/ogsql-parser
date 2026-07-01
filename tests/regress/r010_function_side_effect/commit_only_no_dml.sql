-- description: function with only COMMIT (no DML) should trigger R010
-- warn: R010
CREATE OR REPLACE FUNCTION fn_commit_only() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    COMMIT;
END;
$$;
