-- 来源: 1076_file_1076.txt
-- SQL 数量: 84

SELECT 2 + 3 AS RESULT ;

SELECT 2 - 3 AS RESULT ;

SELECT 2 * 3 AS RESULT ;

SELECT 4 / 2 AS RESULT ;

SELECT 4 / 3 AS RESULT ;

SELECT - 2 AS RESULT ;

SELECT 5 % 4 AS RESULT ;

SELECT @ - 5 . 0 AS RESULT ;

SELECT 2 . 0 ^ 3 . 0 AS RESULT ;

SELECT |/ 25 . 0 AS RESULT ;

SELECT ||/ 27 . 0 AS RESULT ;

SELECT 5 ! AS RESULT ;

SELECT !! 5 AS RESULT ;

SELECT 91 & 15 AS RESULT ;

SELECT 32 | 3 AS RESULT ;

SELECT 17 # 5 AS RESULT ;

SELECT ~ 1 AS RESULT ;

SELECT 1 << 4 AS RESULT ;

SELECT 8 >> 2 AS RESULT ;

SELECT abs ( - 17 . 4 );

SELECT acos ( - 1 );

SELECT asin ( 0 . 5 );

SELECT atan ( 1 );

SELECT atan2 ( 2 , 1 );

SELECT bitand ( 127 , 63 );

SELECT cbrt ( 27 . 0 );

SELECT ceil ( - 42 . 8 );

SELECT ceiling ( - 95 . 3 );

SELECT cos ( - 3 . 1415927 );

SELECT cosh ( 4 );

SELECT cot ( 1 );

SELECT degrees ( 0 . 5 );

SELECT div ( 9 , 4 );

SELECT exp ( 1 . 0 );

SELECT floor ( - 42 . 8 );

select int1 ( '123' );

select int1 ( '1.1' );

select int2 ( '1234' );

select int2 ( 25 . 3 );

select int4 ( '789' );

select int4 ( 99 . 9 );

select int8 ( '789' );

select int8 ( 99 . 9 );

select float4 ( '789' );

select float4 ( 99 . 9 );

select float8 ( '789' );

select float8 ( 99 . 9 );

select int16 ( '789' );

select int16 ( 99 . 9 );

select "numeric" ( '789' );

select "numeric" ( 99 . 9 );

SELECT radians ( 45 . 0 );

SELECT random ();

SELECT multiply ( 9 . 0 , '3.0' );

SELECT multiply ( '9.0' , 3 . 0 );

SELECT ln ( 2 . 0 );

SELECT log ( 100 . 0 );

SELECT log ( 2 . 0 , 64 . 0 );

SELECT mod ( 9 , 4 );

SELECT mod ( 9 , 0 );

SELECT pi ();

SELECT power ( 9 . 0 , 3 . 0 );

SELECT remainder ( 11 , 4 );

SELECT remainder ( 9 , 0 );

SELECT round ( 42 . 4 );

SELECT round ( 42 . 6 );

SELECT round ( - 0 . 2 :: float8 );

SELECT round ( 42 . 4382 , 2 );

SELECT setseed ( 0 . 54823 );

SELECT sign ( - 8 . 4 );

SELECT sin ( 1 . 57079 );

SELECT sinh ( 4 );

SELECT sqrt ( 2 . 0 );

SELECT tan ( 20 );

SELECT tanh ( 0 . 1 );

SELECT trunc ( 42 . 8 );

SELECT trunc ( 42 . 4382 , 2 );

SELECT width_bucket ( 5 . 35 , 0 . 024 , 10 . 06 , 5 );

SELECT width_bucket ( 5 . 35 , 0 . 024 , 10 . 06 , 5 );

SELECT nanvl('NaN', 1.1);

SELECT numeric_eq_text(1, '1');

SELECT text_eq_numeric('1', 1);

SELECT bigint_eq_text(1, '1');

SELECT text_eq_bigint('1', 1);

