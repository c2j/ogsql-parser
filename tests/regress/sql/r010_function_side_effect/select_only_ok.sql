-- description: function with only SELECT should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION fn_select() RETURNS SETOF t1
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY SELECT * FROM t1;
END;
$$;
