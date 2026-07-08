-- description: dbe_lob.trim with 2 args should be OK (takes exactly 2)
-- parse-nowarn: trim
SELECT dbe_lob.trim(s, n) FROM t;
