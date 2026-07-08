-- description: anonymous block with multiple varchar2 variables declared and initialized via CASE WHEN expressions
DECLARE
    v_grade  VARCHAR2(20) := CASE WHEN 85 >= 90 THEN 'A' ELSE 'B' END;
    v_status VARCHAR2(30) := CASE WHEN TRUE THEN 'Passed' ELSE 'Failed' END;
    v_label  VARCHAR2(100) := CASE
        WHEN 1=1 AND 2=2 THEN 'Valid'
        WHEN 1=2 OR 3=3 THEN 'Maybe'
        ELSE 'Invalid'
    END;
    v_id     INTEGER := 42;
BEGIN
    RAISE NOTICE 'Grade: %, Status: %, Label: %, ID: %', v_grade, v_status, v_label, v_id;
END;
