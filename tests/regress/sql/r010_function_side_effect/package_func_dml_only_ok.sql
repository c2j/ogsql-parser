-- description: package body function with DML only (no transaction control) should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE PACKAGE BODY pkg_r010_test AS
  FUNCTION fn_dml_only RETURN INTEGER IS
  BEGIN
    INSERT INTO t1 VALUES (1);
    UPDATE t2 SET c = 1;
    DELETE FROM t3;
    RETURN 1;
  END fn_dml_only;
END pkg_r010_test;
