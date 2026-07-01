-- description: function calling function with transaction (cross-statement) should trigger R010
-- warn: R010
CREATE OR REPLACE FUNCTION inner_f() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO t1 VALUES (1);
    COMMIT;
END;
$$;

CREATE OR REPLACE FUNCTION outer_f() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    inner_f();
END;
$$;
