-- 来源: 2786_file_2786.txt
-- SQL 数量: 12

SELECT gs_encrypt_aes128 ( 'MPPDB' , 'Asdf1234' );

SELECT gs_encrypt('MPPDB', 'Asdf1234', 'sm4');

SELECT gs_encrypt_bytea('MPPDB', 'Asdf1234', 'sm4_ctr_sm3');

SELECT gs_decrypt_aes128 ( 'gwditQLQG8NhFw4OuoKhhQJoXojhFlYkjeG0aYdSCtLCnIUgkNwvYI04KbuhmcGZp8jWizBdR1vU9CspjuzI0lbz12A=' , '1234' );

select gs_decrypt('ZBzOmaGA4Bb+coyucJ0B8AkIShqc', 'Asdf1234', 'sm4');

select gs_decrypt_bytea('\x90e286971c2c70410def0a2814af4ac44c737926458b66271d9d1547bc937395ca018d7755672fa9dc3cdc6ec4a76001dc0e137f3bc5c8a5c51143561f1d09a848bfdebfec5e', 'Asdf1234', 'sm4_ctr_sm3');

select aes_encrypt('huwei123','123456vfhex4dyu,vdaladhjsadad','1234567890123456');

select aes_decrypt(aes_encrypt('huwei123','123456vfhex4dyu,vdaladhjsadad','1234567890123456'),'123456vfhex4dyu,vdaladhjsadad','1234567890123456');

SELECT pg_catalog . gs_digest ( 'gaussdb' , 'sha256' );

SELECT gs_password_deadline ();

SELECT inet_server_addr ();

SELECT inet_client_addr ();

