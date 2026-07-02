-- description: dbe_lob.trim with 1 arg currently no warn (BUG - dbe_lob.trim takes exactly 2, builtin trim takes 1-3)
-- parse-nowarn: trim
SELECT dbe_lob.trim(s) FROM t;
