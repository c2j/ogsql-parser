-- description: PL/pgSQL anonymous block with FOR loop, DML, and COMMIT
BEGIN
   DELETE FROM test_data_sync WHERE id LIKE 'chkdbt%' AND LENGTH(id) <= 8;

   FOR i IN 1..99 LOOP
       INSERT INTO test_data_sync(id, test_number, test_varchar)
       VALUES ('chkdbt' || i, i, 'check测试drs同步');
   END LOOP;

   COMMIT;
END;
