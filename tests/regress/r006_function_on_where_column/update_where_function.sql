-- description: function on WHERE column in UPDATE should trigger R006
-- warn: R006
UPDATE t SET x = 1 WHERE LENGTH(name) > 5;
