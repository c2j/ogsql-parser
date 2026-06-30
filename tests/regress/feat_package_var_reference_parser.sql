-- Issue: package-var-reference
-- Description: Package-level variables referenced inside procedures and functions
-- Expect: all parse without error

-- 场景1: 包级变量在过程中赋值和读取
CREATE OR REPLACE PACKAGE BODY pkg_var_ref AS
  v_status VARCHAR := 'ACTIVE';
  v_counter INTEGER := 0;
  PROCEDURE prc_update_status(p_new_status VARCHAR) IS
  BEGIN
    v_status := p_new_status;
    v_counter := v_counter + 1;
    RAISE NOTICE 'status: %, count: %', v_status, v_counter;
  END prc_update_status;
  FUNCTION get_status RETURN VARCHAR IS
  BEGIN
    RETURN v_status;
  END get_status;
END pkg_var_ref;

-- 场景2: 包级变量在条件判断中引用
CREATE OR REPLACE PACKAGE BODY pkg_cond_ref AS
  v_threshold INTEGER := 100;
  PROCEDURE prc_check(p_value INTEGER) IS
  BEGIN
    IF p_value > v_threshold THEN
      RAISE NOTICE 'value % exceeds threshold %', p_value, v_threshold;
    ELSE
      RAISE NOTICE 'value % within limit', p_value;
    END IF;
  END prc_check;
END pkg_cond_ref;

-- 场景3: 包级变量在 FOR/WHILE 循环中使用
CREATE OR REPLACE PACKAGE BODY pkg_loop_ref AS
  v_total INTEGER := 0;
  v_limit INTEGER := 10;
  PROCEDURE prc_accumulate IS
    i INTEGER;
  BEGIN
    v_total := 0;
    FOR i IN 1 .. v_limit LOOP
      v_total := v_total + i;
    END LOOP;
  END prc_accumulate;
END pkg_loop_ref;

-- 场景4: 包级变量在 SELECT INTO 中作为目标
CREATE OR REPLACE PACKAGE BODY pkg_select_ref AS
  v_current_user VARCHAR := '';
  PROCEDURE prc_fetch_user IS
  BEGIN
    SELECT current_user INTO v_current_user;
    RAISE NOTICE 'current user: %', v_current_user;
  END prc_fetch_user;
END pkg_select_ref;

-- 场景5: 包级变量在 EXECUTE IMMEDIATE 中引用
CREATE OR REPLACE PACKAGE BODY pkg_dynamic_ref AS
  v_table_name VARCHAR := 'employees';
  v_sql VARCHAR(1000);
  PROCEDURE prc_dynamic_delete IS
  BEGIN
    v_sql := 'DELETE FROM ' || v_table_name || ' WHERE status = ''INACTIVE''';
    EXECUTE IMMEDIATE v_sql;
  END prc_dynamic_delete;
END pkg_dynamic_ref;

-- 场景6: 包级变量作为 EXCEPTION 处理中的参数
CREATE OR REPLACE PACKAGE BODY pkg_exception_ref AS
  v_retry_count INTEGER := 0;
  v_max_retries INTEGER := 3;
  PROCEDURE prc_with_retry IS
  BEGIN
    v_retry_count := 0;
    LOOP
      BEGIN
        NULL;
      EXCEPTION
        WHEN OTHERS THEN
          v_retry_count := v_retry_count + 1;
          IF v_retry_count >= v_max_retries THEN
            RAISE;
          END IF;
      END;
      EXIT;
    END LOOP;
  END prc_with_retry;
END pkg_exception_ref;

-- 场景7: spec 无 schema，body 带 schema — 仍应关联变量
CREATE OR REPLACE PACKAGE pkg_schema_test AS
  v_flag NUMBER;
  PROCEDURE prc_test;
END pkg_schema_test;

CREATE OR REPLACE PACKAGE BODY myschema.pkg_schema_test AS
  PROCEDURE prc_test IS
  BEGIN
    v_flag := 1;
    RAISE NOTICE '%', v_flag;
  END prc_test;
END pkg_schema_test;

-- 场景8: spec 带 schema，body 无 schema — 仍应关联变量
CREATE OR REPLACE PACKAGE myschema.pkg_schema_rev AS
  v_flag NUMBER;
  PROCEDURE prc_test;
END pkg_schema_rev;

CREATE OR REPLACE PACKAGE BODY pkg_schema_rev AS
  PROCEDURE prc_test IS
  BEGIN
    v_flag := 1;
    RAISE NOTICE '%', v_flag;
  END prc_test;
END pkg_schema_rev;
