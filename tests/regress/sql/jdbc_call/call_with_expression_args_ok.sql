-- description: CALL with expression arguments (function calls, arithmetic) should parse
CALL pkg_xxx.proc_yyy(upper('hello'), 1 + 2, now());
