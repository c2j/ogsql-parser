-- description: built-in bit_and with 1 arg should be OK
-- parse-nowarn: bit_and
SELECT bit_and(c1) FROM t;
