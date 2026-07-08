-- description: regexp_substr with 2 args should be OK
-- parse-nowarn: regexp_substr
SELECT regexp_substr('s', 'p') FROM t;
