-- description: function with DML in LOOP should trigger R010
-- warn: R010
CREATE OR REPLACE FUNCTION fn_insert_in_loop() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    LOOP
        INSERT INTO t1 VALUES (1);
        EXIT;
    END LOOP;
END;
$$;
