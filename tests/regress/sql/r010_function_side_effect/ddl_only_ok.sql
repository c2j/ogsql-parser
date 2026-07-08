-- description: function with DDL (CREATE TABLE) should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE FUNCTION fn_ddl() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    CREATE TEMP TABLE tmp (id INTEGER);
    DROP TABLE tmp;
END;
$$;
