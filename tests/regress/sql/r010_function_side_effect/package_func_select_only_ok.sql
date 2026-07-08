-- description: package body function with only SELECT should NOT trigger R010
-- nowarn: R010
CREATE OR REPLACE PACKAGE BODY pkg_r010_test AS
  FUNCTION fn_select_only RETURN INTEGER IS
    v_count INTEGER;
  BEGIN
    SELECT COUNT(*) INTO v_count FROM t1;
    RETURN v_count;
  END fn_select_only;
END pkg_r010_test;
