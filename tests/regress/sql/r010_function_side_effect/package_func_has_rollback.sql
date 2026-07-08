-- description: package body function with ROLLBACK should trigger R010
-- warn: R010
CREATE OR REPLACE PACKAGE BODY pkg_r010_test AS
  FUNCTION fn_with_rollback RETURN INTEGER IS
  BEGIN
    INSERT INTO t1 VALUES (1);
    ROLLBACK;
    RETURN 1;
  END fn_with_rollback;
END pkg_r010_test;
