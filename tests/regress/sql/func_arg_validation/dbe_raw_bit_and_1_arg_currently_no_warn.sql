-- description: dbe_raw.bit_and with 1 arg currently no warn (BUG - should warn, dbe_raw.bit_and takes exactly 2)
-- parse-nowarn: bit_and
SELECT dbe_raw.bit_and(r1) FROM t;
