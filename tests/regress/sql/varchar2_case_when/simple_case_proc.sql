-- description: procedure with varchar2 variable declared and initialized via simple CASE expression (expr-based matching)
CREATE OR REPLACE PROCEDURE test_simple_case(
    p_level IN INTEGER
) AS
    v_level_name VARCHAR2(50) := CASE p_level
        WHEN 1 THEN 'Low'
        WHEN 2 THEN 'Medium'
        WHEN 3 THEN 'High'
        ELSE 'Unknown'
    END;
BEGIN
    RAISE NOTICE 'Level: %', v_level_name;
END;
