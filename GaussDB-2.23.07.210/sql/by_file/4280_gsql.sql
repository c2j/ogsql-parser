-- 来源: 4280_gsql.txt
-- SQL 数量: 14

CREATE CLIENT MASTER KEY cmk1 WITH ( KEY_STORE = hcs_kms , KEY_PATH = '{KMS服务器地址}/{密钥ID}', ALGORITHM = AES_256);

CREATE COLUMN ENCRYPTION KEY cek1 WITH VALUES (CLIENT_MASTER_KEY = cmk1, ALGORITHM = AES_256_GCM);

CREATE TABLE creditcard_info ( id_number int, name text encrypted with (column_encryption_key = cek1, encryption_type = DETERMINISTIC), credit_card varchar(19) encrypted with (column_encryption_key = cek1, encryption_type = DETERMINISTIC));

INSERT INTO creditcard_info VALUES (1,'joe','6217986500001288393');

INSERT INTO creditcard_info VALUES (2, 'joy','6219985678349800033');

-- 从加密表中查询数据
select * from creditcard_info where name = 'joe';

-- 更新加密表中数据
update creditcard_info set credit_card = '80000000011111111' where name = 'joy';

-- 向表中新增一列加密列
ALTER TABLE creditcard_info ADD COLUMN age int ENCRYPTED WITH (COLUMN_ENCRYPTION_KEY = cek1, ENCRYPTION_TYPE = DETERMINISTIC);

-- 从表中删除一列加密列
ALTER TABLE creditcard_info DROP COLUMN age;

-- 从系统表中查询主密钥信息
SELECT * FROM gs_client_global_keys;

-- 从系统表中查询列密钥信息
SELECT column_key_name,column_key_distributed_id ,global_key_id,key_owner FROM gs_column_keys;

DROP TABLE creditcard_info;

-- 删除列密钥
DROP COLUMN ENCRYPTION KEY cek1;

-- 删除主密钥
DROP CLIENT MASTER KEY cmk1;

