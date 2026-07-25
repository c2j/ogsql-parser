-- 来源: 4507_gsql.txt
-- SQL 数量: 24

CREATE CLIENT MASTER KEY cmk1 WITH ( KEY_STORE = hcs_kms , KEY_PATH = '{KMS服务器地址}/{密钥ID}', ALGORITHM = AES_256);

CREATE COLUMN ENCRYPTION KEY cek1 WITH VALUES (CLIENT_MASTER_KEY = cmk1, ALGORITHM = AES_256_GCM);

CREATE TABLE contacts ( id int unique , credit float8 encrypted with ( column_encryption_key = cek1 , encryption_type = DETERMINISTIC ), name text encrypted with ( column_encryption_key = cek1 , encryption_type = DETERMINISTIC ));

CREATE TABLE contacts_plain ( id int unique , credit float8 , name text );

INSERT INTO contacts VALUES ( 1 , 8000 , 'zhangsan' );

INSERT INTO contacts VALUES ( 2 , 7056 . 6 , 'lisi' );

INSERT INTO contacts VALUES ( 3 , 16050 , 'wangwu' );

select id,credit from contacts where credit > 10000;

select id,credit from contacts where credit < 10000;

select id,credit from contacts where credit >= 8000;

select id,credit from contacts where credit <= 8000;

select id,credit from contacts order by credit;

select id,credit from contacts order by credit DESC;

select credit*2 from contacts limit 1;

select sum(credit) from contacts;

select case when credit > 9000 then name end from contacts;

select credit::text, credit::int from contacts offset 1 limit 1;

select credit from contacts where name like 'zhang%';

insert into contacts_plain (id, name) select id, ce_decrypt_deterministic(name, (select column_key_distributed_id from gs_column_keys where column_key_name=' cek1 ')) from contacts;

delete from contacts;

insert into contacts (id, name) select id, ce_encrypt_deterministic(name, (select column_key_distributed_id from gs_column_keys where column_key_name=' cek1 ')) from contacts_plain;

DROP TABLE contacts, contacts_plain;

DROP COLUMN ENCRYPTION KEY cek1;

DROP CLIENT MASTER KEY cmk1;

