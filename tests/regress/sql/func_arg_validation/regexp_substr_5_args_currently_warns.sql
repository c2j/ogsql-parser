-- description: regexp_substr with 5 args currently warns (BUG - GaussDB accepts 5, registry says max 4)
-- parse-warn: regexp_substr, at most 4
SELECT regexp_substr('s', 'p', 1, 1, 'i') FROM t;
