-- description: CALL inside PL/pgSQL DO block should parse
DO $$
BEGIN
    CALL pkg_xxx.proc_yyy('arg1', 42);
END;
$$;
