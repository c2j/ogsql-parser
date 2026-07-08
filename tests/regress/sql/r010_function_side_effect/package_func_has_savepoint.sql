-- description: package body function with SAVEPOINT should trigger R010
-- warn: R010
CREATE OR REPLACE PACKAGE BODY pkg_r010_test AS
  FUNCTION fn_with_savepoint RETURN INTEGER IS
  BEGIN
    SAVEPOINT sp1;
    INSERT INTO t1 VALUES (1);
    RETURN 1;
  END fn_with_savepoint;
END pkg_r010_test;
