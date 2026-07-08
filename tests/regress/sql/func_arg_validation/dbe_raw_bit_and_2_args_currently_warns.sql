-- description: dbe_raw.bit_and with 2 args currently warns (BUG - should not, dbe_raw.bit_and takes exactly 2)
-- parse-warn: bit_and, exactly 1
SELECT dbe_raw.bit_and(r1, r2) FROM t;
