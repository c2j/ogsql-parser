-- description: regexp_substr with 1 arg warns (requires at least 2)
-- parse-warn: regexp_substr, at least 2
SELECT regexp_substr('s') FROM t;
