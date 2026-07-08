-- description: procedure with constant varchar2 variable declared and initialized via simple CASE expression referencing a parameter
CREATE OR REPLACE PROCEDURE test_const_case(
    p_i_insert_flag IN VARCHAR2
) AS
    v_name CONSTANT VARCHAR2(10) := CASE p_i_insert_flag
        WHEN '0' THEN 'Inserted'
        ELSE 'updated'
    END;
BEGIN
    RAISE NOTICE '%', v_name;
END;
