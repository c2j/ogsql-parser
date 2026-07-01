-- description: function wrapping column in WHERE should trigger R006
-- warn: R006
SELECT * FROM t WHERE LENGTH(name) > 5;
