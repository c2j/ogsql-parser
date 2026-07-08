-- description: function with DML in LOOP without transaction should NOT trigger R010
-- nowarn: R010
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
