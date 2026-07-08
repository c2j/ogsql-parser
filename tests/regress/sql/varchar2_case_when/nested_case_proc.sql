-- description: procedure with varchar2 variable initialized via nested CASE WHEN expression
CREATE OR REPLACE PROCEDURE test_nested_case(
    p_type  IN VARCHAR2,
    p_value IN INTEGER
) AS
    v_category VARCHAR2(200) := CASE
        WHEN p_type = 'A' THEN CASE
            WHEN p_value > 100 THEN 'A-High'
            WHEN p_value >= 50 THEN 'A-Medium'
            ELSE 'A-Low'
        END
        WHEN p_type = 'B' THEN CASE
            WHEN p_value > 200 THEN 'B-High'
            ELSE 'B-Low'
        END
        ELSE 'Other'
    END;
BEGIN
    RAISE NOTICE 'Category: %', v_category;
END;
