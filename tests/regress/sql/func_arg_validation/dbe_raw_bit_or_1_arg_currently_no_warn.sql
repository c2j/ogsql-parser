-- description: dbe_raw.bit_or with 1 arg currently no warn (BUG - should warn, dbe_raw.bit_or takes exactly 2)
-- parse-nowarn: bit_or
SELECT dbe_raw.bit_or(r1) FROM t;
