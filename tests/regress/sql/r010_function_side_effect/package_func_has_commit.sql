-- description: package body function with COMMIT should trigger R010
-- warn: R010
CREATE OR REPLACE PACKAGE BODY pkg_r010_test AS
  FUNCTION fn_with_commit RETURN INTEGER IS
  BEGIN
    INSERT INTO t1 VALUES (1);
    COMMIT;
    RETURN 1;
  END fn_with_commit;
END pkg_r010_test;
