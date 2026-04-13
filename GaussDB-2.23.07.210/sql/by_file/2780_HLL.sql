-- 来源: 2780_HLL.txt
-- SQL 数量: 66

SELECT hll_hash_boolean ( FALSE );

SELECT hll_hash_boolean ( FALSE , 10 );

SELECT hll_hash_smallint ( 100 :: smallint );

SELECT hll_hash_smallint ( 100 :: smallint , 10 );

SELECT hll_hash_integer ( 0 );

SELECT hll_hash_integer ( 0 , 10 );

SELECT hll_hash_bigint ( 100 :: bigint );

SELECT hll_hash_bigint ( 100 :: bigint , 10 );

SELECT hll_hash_bytea ( E '\\x' );

SELECT hll_hash_bytea ( E '\\x' , 10 );

SELECT hll_hash_text ( 'AB' );

SELECT hll_hash_text ( 'AB' , 10 );

SELECT hll_hash_any ( 1 );

SELECT hll_hash_any ( '08:00:2b:01:02:03' :: macaddr );

SELECT hll_hash_any ( 1 , 10 );

SELECT hll_hashval_eq ( hll_hash_integer ( 1 ), hll_hash_integer ( 1 ));

SELECT hll_hashval_ne ( hll_hash_integer ( 1 ), hll_hash_integer ( 1 ));

SELECT hll_print ( hll_empty ());

SELECT hll_type ( hll_empty ());

SELECT hll_log2m ( hll_empty ());

SELECT hll_log2m ( hll_empty ( 10 ));

SELECT hll_log2m ( hll_empty ( - 1 ));

SELECT hll_log2explicit ( hll_empty ());

SELECT hll_log2explicit ( hll_empty ( 12 , 8 ));

SELECT hll_log2explicit ( hll_empty ( 12 , - 1 ));

SELECT hll_log2sparse ( hll_empty ());

SELECT hll_log2sparse ( hll_empty ( 12 , 8 , 10 ));

SELECT hll_log2sparse ( hll_empty ( 12 , 8 , - 1 ));

SELECT hll_duplicatecheck ( hll_empty ());

SELECT hll_duplicatecheck ( hll_empty ( 12 , 8 , 10 , 1 ));

SELECT hll_duplicatecheck ( hll_empty ( 12 , 8 , 10 , - 1 ));

SELECT hll_empty ();

SELECT hll_empty ( 10 );

SELECT hll_empty ( - 1 );

SELECT hll_empty ( 10 , 4 );

SELECT hll_empty ( 10 , - 1 );

SELECT hll_empty ( 10 , 4 , 8 );

SELECT hll_empty ( 10 , 4 , - 1 );

SELECT hll_empty ( 10 , 4 , 8 , 0 );

SELECT hll_empty ( 10 , 4 , 8 , - 1 );

SELECT hll_add ( hll_empty (), hll_hash_integer ( 1 ));

SELECT hll_add_rev ( hll_hash_integer ( 1 ), hll_empty ());

SELECT hll_eq ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

SELECT hll_ne ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

SELECT hll_cardinality ( hll_empty () || hll_hash_integer ( 1 ));

SELECT hll_union ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

CREATE TABLE t_id ( id int );

INSERT INTO t_id VALUES ( generate_series ( 1 , 500 ));

CREATE TABLE t_data ( a int , c text );

INSERT INTO t_data SELECT mod ( id , 2 ), id FROM t_id ;

CREATE TABLE t_a_c_hll ( a int , c hll );

INSERT INTO t_a_c_hll SELECT a , hll_add_agg ( hll_hash_text ( c )) FROM t_data GROUP BY a ;

SELECT a , # c AS cardinality FROM t_a_c_hll ORDER BY a ;

SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), 12 )) FROM t_data ;

SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 1 )) FROM t_data ;

SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 6 , 10 )) FROM t_data ;

SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 6 , 10 , - 1 )) FROM t_data ;

SELECT # hll_union_agg ( c ) AS cardinality FROM t_a_c_hll ;

SELECT ( hll_empty () || hll_hash_integer ( 1 )) = ( hll_empty () || hll_hash_integer ( 1 ));

SELECT hll_hash_integer ( 1 ) = hll_hash_integer ( 1 );

SELECT ( hll_empty () || hll_hash_integer ( 1 )) <> ( hll_empty () || hll_hash_integer ( 2 ));

SELECT hll_hash_integer ( 1 ) <> hll_hash_integer ( 2 );

SELECT hll_empty () || hll_hash_integer ( 1 );

SELECT hll_hash_integer ( 1 ) || hll_empty ();

SELECT ( hll_empty () || hll_hash_integer ( 1 )) || ( hll_empty () || hll_hash_integer ( 2 ));

SELECT # ( hll_empty () || hll_hash_integer ( 1 ));

