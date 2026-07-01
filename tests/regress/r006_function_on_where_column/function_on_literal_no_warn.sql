-- description: function on literal (not column) in WHERE should not trigger R006
-- nowarn: R006
SELECT * FROM t WHERE LENGTH('hello') > 3;
