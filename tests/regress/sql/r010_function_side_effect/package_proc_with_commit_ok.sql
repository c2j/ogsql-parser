-- description: package body procedure with COMMIT should NOT trigger R010 (only functions)
-- nowarn: R010
CREATE OR REPLACE PACKAGE BODY pkg_r010_test AS
  PROCEDURE prc_with_commit IS
  BEGIN
    INSERT INTO t1 VALUES (1);
    COMMIT;
  END prc_with_commit;
END pkg_r010_test;
