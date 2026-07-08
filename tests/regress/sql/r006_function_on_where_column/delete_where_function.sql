-- description: function on WHERE column in DELETE should trigger R006
-- warn: R006
DELETE FROM t WHERE LENGTH(name) > 5;
