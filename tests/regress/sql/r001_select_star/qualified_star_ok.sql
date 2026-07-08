-- description: qualified t1.* should NOT trigger R001
-- nowarn: R001
SELECT t1.* FROM t1;
