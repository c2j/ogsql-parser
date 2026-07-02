-- description: dbe_raw.bit_or with 2 args currently warns (BUG - should not, dbe_raw.bit_or takes exactly 2)
-- parse-warn: bit_or, exactly 1
SELECT dbe_raw.bit_or(r1, r2) FROM t;
