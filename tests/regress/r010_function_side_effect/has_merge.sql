-- description: function with MERGE without transaction should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION fn_merge() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    MERGE INTO t1 USING t2 ON t1.id = t2.id
    WHEN MATCHED THEN UPDATE SET name = t2.name
    WHEN NOT MATCHED THEN INSERT VALUES (t2.id, t2.name);
END;
$$;
