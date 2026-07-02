-- description: regexp_substr with 4 args should be OK
-- parse-nowarn: regexp_substr
SELECT regexp_substr('s', 'p', 1, 1) FROM t;
