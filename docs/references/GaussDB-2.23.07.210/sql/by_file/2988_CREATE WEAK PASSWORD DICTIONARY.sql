-- 来源: 2988_CREATE WEAK PASSWORD DICTIONARY.txt
-- SQL 数量: 4

CREATE WEAK PASSWORD DICTIONARY WITH VALUES ('password1');

--向gs_global_config系统表中插入多个弱口令。
CREATE WEAK PASSWORD DICTIONARY WITH VALUES ('password2'),('password3');

--清空gs_global_config系统表中所有弱口令。
DROP WEAK PASSWORD DICTIONARY;

--查看现有弱口令。
SELECT * FROM gs_global_config WHERE NAME LIKE 'weak_password';

