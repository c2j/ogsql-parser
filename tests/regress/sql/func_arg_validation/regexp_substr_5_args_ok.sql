-- description: regexp_substr with 5 args is valid (fixed - registry now says max 5)
-- parse-nowarn: regexp_substr
SELECT regexp_substr('s', 'p', 1, 1, 'i') FROM t;
