-- description: standalone function calling package function with transaction should trigger R010
-- warn: R010
CREATE OR REPLACE PACKAGE BODY pkg_tx AS
  FUNCTION fn_tx RETURN INTEGER IS
  BEGIN
    INSERT INTO t1 VALUES (1);
    COMMIT;
    RETURN 1;
  END fn_tx;
END pkg_tx;
/
CREATE OR REPLACE FUNCTION fn_caller() RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    pkg_tx.fn_tx();
END;
$$;
