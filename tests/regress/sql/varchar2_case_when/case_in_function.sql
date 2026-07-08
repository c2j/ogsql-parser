-- description: function with varchar2 variable declared and initialized via CASE WHEN expression
CREATE OR REPLACE FUNCTION get_status_label(p_status VARCHAR2) RETURN VARCHAR2 AS
    v_label VARCHAR2(200) := CASE
        WHEN p_status IS NULL THEN 'N/A'
        WHEN LENGTH(p_status) = 0 THEN 'Empty'
        ELSE p_status || '-Processed'
    END;
BEGIN
    RETURN v_label;
END;
