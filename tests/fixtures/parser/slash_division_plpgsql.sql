-- Issue: slash-division-operator
-- Description: / is treated as terminator in PL/pgSQL blocks
-- Expect: parse
-- Split: blank-line

DO $$ DECLARE x INT := 10 / 2; BEGIN RAISE NOTICE '%', x; END $$;

CREATE FUNCTION f_div() RETURNS INT AS $$
DECLARE
    result INT;
BEGIN
    result := 100 / 3;
    RETURN result;
END;
$$ LANGUAGE plpgsql;

CREATE PROCEDURE p_div() AS $$
DECLARE
    x INT := a / 1000;
BEGIN
    UPDATE tab SET col = col / 2 WHERE id = x;
END;
$$ LANGUAGE plpgsql;

DO $$
DECLARE
    r RECORD;
BEGIN
    FOR r IN SELECT a / 1000 AS ratio FROM tab LOOP
        RAISE NOTICE '%', r.ratio;
    END LOOP;
END $$;
