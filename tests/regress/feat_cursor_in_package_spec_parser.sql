-- Issue: cursor-in-package-spec
-- Description: Package spec with cursor declarations (no-param, param, multi-param, IN OUT)
-- Expect: all parse without error

-- 场景1: 包规范中声明无参数游标
CREATE OR REPLACE PACKAGE pkg_cursor1 IS
  CURSOR c_no_param IS
    SELECT * FROM t;
END pkg_cursor1;

-- 场景2: 包规范中声明带单个 IN 参数游标
CREATE OR REPLACE PACKAGE pkg_cursor2 IS
  CURSOR c_one_param(v_code IN VARCHAR2) IS
    SELECT * FROM dat_contract t WHERE t.step_code = v_code;
END pkg_cursor2;

-- 场景3: 包规范中声明带多个参数游标（IN / OUT）
CREATE OR REPLACE PACKAGE pkg_cursor3 IS
  CURSOR c_multi_param(v_code IN VARCHAR2, v_flag OUT INTEGER) IS
    SELECT t1.seq_no FROM dat_inst t1 WHERE t1.code = v_code;
END pkg_cursor3;

-- 场景4: 包规范中声明带 IN OUT 参数游标
CREATE OR REPLACE PACKAGE pkg_cursor4 IS
  CURSOR c_inout_param(p1 IN OUT VARCHAR2, p2 INTEGER) IS
    SELECT * FROM t WHERE col1 = p1 AND col2 = p2;
END pkg_cursor4;

-- 场景5: 包规范中同时包含变量、游标、过程声明
CREATE OR REPLACE PACKAGE pkg_cursor5 IS
  v_flag NUMBER;
  CURSOR c_data(v_code IN VARCHAR2) IS
    SELECT * FROM t WHERE code = v_code;
  PROCEDURE prc_main;
END pkg_cursor5;

-- 场景6: 包体引用包规范中声明的游标（OPEN / CLOSE）
CREATE OR REPLACE PACKAGE BODY pkg_cursor5 IS
  PROCEDURE prc_main IS
  BEGIN
    v_flag := 1;
    OPEN c_data('X');
    FETCH c_data INTO v_rec;
    CLOSE c_data;
  END prc_main;
END pkg_cursor5;

-- 场景7: 包规范中游标查询带 JOIN
CREATE OR REPLACE PACKAGE pkg_cursor6 IS
  CURSOR c_joined(v_id IN INTEGER) IS
    SELECT a.name, b.amount
    FROM users a
    INNER JOIN orders b ON a.id = b.user_id
    WHERE a.id = v_id;
END pkg_cursor6;
