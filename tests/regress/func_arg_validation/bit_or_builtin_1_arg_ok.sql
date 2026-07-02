-- description: built-in bit_or with 1 arg should be OK
-- parse-nowarn: bit_or
SELECT bit_or(c1) FROM t;
