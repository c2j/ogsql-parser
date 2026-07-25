-- 来源: 1089_file_1089.txt
-- SQL 数量: 12

SELECT gs_encrypt_aes128 ( 'MPPDB' , '1234@abc' );

SELECT gs_decrypt_aes128 ( 'OF1g3+70oeqFfyKiWlpxfYxPnpeitNc6+7nAe02Ttt37fZF8Q+bbEYhdw/YG+0c9tHKRWM6OcTzlB3HnqvX+1d8Bflo=' , '1234@abc' );

select aes_encrypt('huwei123','123456vfhex4dyu,vdaladhjsadad','1234567890123456');

select aes_decrypt(aes_encrypt('huwei123','123456vfhex4dyu,vdaladhjsadad','1234567890123456'),'123456vfhex4dyu,vdaladhjsadad','1234567890123456');

SELECT pg_catalog . gs_digest ( 'gaussdb' , 'sha256' );

SELECT gs_password_deadline ();

SELECT inet_server_addr ();

SELECT inet_client_addr ();

SELECT gs_encrypt('MPPDB', 'Asdf1234', 'sm4');

select gs_decrypt('ZBzOmaGA4Bb+coyucJ0B8AkIShqc', 'Asdf1234', 'sm4');

SELECT gs_encrypt_bytea('MPPDB', 'Asdf1234', 'sm4_ctr_sm3');

select gs_decrypt_bytea('\x90e286971c2c70410def0a2814af4ac44c737926458b66271d9d1547bc937395ca018d7755672fa9dc3cdc6ec4a76001dc0e137f3bc5c8a5c51143561f1d09a848bfdebfec5e', 'Asdf1234', 'sm4_ctr_sm3');

