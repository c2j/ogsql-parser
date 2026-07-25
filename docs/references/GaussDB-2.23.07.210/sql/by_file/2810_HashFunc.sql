-- 来源: 2810_HashFunc.txt
-- SQL 数量: 21

select ora_hash ( 123 );

select ora_hash ( '123' );

select ora_hash ( 'sample' );

select ora_hash ( to_date ( '2012-1-2' , 'yyyy-mm-dd' ));

select ora_hash ( 123 , 234 );

select ora_hash ( '123' , 234 );

select ora_hash ( 'sample' , 234 );

select ora_hash ( to_date ( '2012-1-2' , 'yyyy-mm-dd' ), 234 );

select hash_array ( ARRAY [[ 1 , 2 , 3 ],[ 1 , 2 , 3 ]]);

select hash_numeric ( 30 );

select hash_range ( numrange ( 1 . 1 , 2 . 2 ));

select hashbpchar ( 'hello' );

select hashbpchar ( 'hello' );

select hashchar ( 'true' );

CREATE TYPE b1 AS ENUM ( 'good' , 'bad' , 'ugly' );

call hashenum ( 'good' :: b1 );

select hashfloat4 ( 12 . 1234 );

select hashfloat8 ( 123456 . 1234 );

select hashinet ( '127.0.0.1' :: inet );

select hashint1 ( 20 );

select hashint2(20000);

