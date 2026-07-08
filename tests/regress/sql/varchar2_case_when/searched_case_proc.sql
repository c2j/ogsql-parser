-- description: procedure with varchar2 variable declared and initialized via searched CASE WHEN expression
CREATE OR REPLACE PROCEDURE test_searched_case(
    p_status IN VARCHAR2
) AS
    v_result VARCHAR2(100) := CASE WHEN p_status = 'A' THEN 'Active' ELSE 'Inactive' END;
BEGIN
    RAISE NOTICE 'Result: %', v_result;
END;
