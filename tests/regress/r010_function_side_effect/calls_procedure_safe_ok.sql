-- description: function calling safe function (no transactions) should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION inner_s() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    PERFORM * FROM t1;
END;
$$;

CREATE OR REPLACE FUNCTION outer_s() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    inner_s();
END;
$$;
