-- description: UPDATE without WHERE triggers C007, should not trigger R001
-- warn: C007
-- nowarn: R001
UPDATE t1 SET name = 'x';
