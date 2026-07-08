-- description: built-in bit_and with 2 args warns (takes exactly 1)
-- parse-warn: bit_and, exactly 1
SELECT bit_and(c1, c2) FROM t;
