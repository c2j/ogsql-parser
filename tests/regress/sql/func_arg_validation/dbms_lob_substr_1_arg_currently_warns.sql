-- description: dbms_lob.substr with 1 arg currently warns (BUG - dbms_lob.substr takes 1-3, builtin substr takes 2-3)
-- parse-warn: substr, at least 2
SELECT dbms_lob.substr(s) FROM t;
