-- 来源: 1072_file_1072.txt
-- SQL 数量: 199

SELECT bit_length ( 'world' );

SELECT btrim ( 'sring' , 'ing' );

SELECT char_length ( 'hello' );

select dump ( 'abc测试' );

SELECT instr ( 'abcdabcdabcd' , 'bcd' , 2 , 2 );

SELECT instrb ( 'abcdabcdabcd' , 'bcd' , 2 , 2 );

SELECT lengthb ( 'hello' );

SELECT left ( 'abcde' , 2 );

SELECT length ( 'jose' , 'UTF8' );

SELECT lpad ( 'hi' , 5 , 'xyza' );

select lpad ( 'expr1' , 7 , '中国' );

select lpad ( 'expr1' , 8 , '中国' );

SELECT notlike ( 1 , 2 );

SELECT notlike ( 1 , 1 );

SELECT octet_length ( 'jose' );

SELECT overlay ( 'hello' placing 'world' from 2 for 3 );

SELECT position ( 'ing' in 'string' );

SELECT pg_client_encoding ();

SELECT quote_ident ( 'hello world' );

SELECT quote_literal ( 'hello' );

SELECT quote_literal ( E 'O\' hello ');

SELECT quote_literal ( 'O\hello' );

SELECT quote_literal ( NULL );

SELECT quote_literal ( 42 . 5 );

SELECT quote_literal ( E 'O\' 42 . 5 ');

SELECT quote_literal ( 'O\42.5' );

SELECT quote_nullable ( 'hello' );

SELECT quote_nullable ( E 'O\' hello ');

SELECT quote_nullable ( 'O\hello' );

SELECT quote_nullable ( NULL );

SELECT quote_nullable ( 42 . 5 );

SELECT quote_nullable ( E 'O\' 42 . 5 ');

SELECT quote_nullable ( 'O\42.5' );

SELECT quote_nullable ( NULL );

select substring_inner ( 'adcde' , 2 , 3 );

SELECT substring ( 'Thomas' from 2 for 3 );

select substring ( 'substrteststring' , - 5 , 5 );

SELECT substring_index ( 'Test1splitTest2splitTest3splitTest4' , 'split' , 2 );

SELECT substring_index ( 'Test1splitTest2splitTest3splitTest4' , 'split' , - 2 );

SELECT substring ( 'Thomas' from '...$' );

SELECT substring ( 'foobar' from 'o(.)b' );

SELECT substring ( 'foobar' from '(o(.)b)' );

SELECT substring ( 'Thomas' from '%#"o_a#"_' for '#' );

SELECT rawcat ( 'ab' , 'cd' );

SELECT regexp_like ( 'str' , '[ac]' );

SELECT regexp_substr ( 'str' , '[ac]' );

SELECT regexp_substr ( 'foobarbaz' , 'b(..)' , 3 , 2 ) AS RESULT ;

SELECT regexp_count('foobarbaz','b(..)', 5) AS RESULT;

SELECT regexp_instr('foobarbaz','b(..)', 1, 1, 0) AS RESULT;

SELECT regexp_instr('foobarbaz','b(..)', 1, 2, 0) AS RESULT;

SELECT regexp_matches ( 'foobarbequebaz' , '(bar)(beque)' );

SELECT regexp_matches ( 'foobarbequebaz' , 'barbeque' );

SELECT regexp_matches ( 'foobarbequebazilbarfbonk' , '(b[^b]+)(b[^b]+)' , 'g' );

SELECT regexp_match('foobarbequebaz', '(bar)(beque)');

SELECT (regexp_match('foobarbequebaz', 'bar.*que'))[1];

SELECT regexp_match('Learning #PostgreSQL', 'R', 'c');

SELECT regexp_match('hello world', 'h e l l o', 'x');

SELECT regexp_split_to_array ( 'hello world' , E '\\s+' );

SELECT regexp_split_to_table ( 'hello world' , E '\\s+' );

SELECT repeat ( 'Pg' , 4 );

SELECT replace ( 'abcdefabcdef' , 'cd' , 'XXX' );

SELECT replace ( 'abcdefabcdef' , 'cd' );

SELECT reverse ( 'abcde' );

SELECT right ( 'abcde' , 2 );

SELECT right ( 'abcde' , - 2 );

SELECT rpad ( 'hi' , 5 , 'xy' );

select rpad ( 'expr1' , 7 , '中国' ) || '*' ;

select rpad ( 'expr1' , 8 , '中国' ) || '*' ;

SELECT substr ( 'stringtest' FROM 4 );

SELECT substr ( 'stringtest' , 4 );

SELECT substr ( 'stringtest' , - 4 );

SELECT substr ( 'stringtest' , 11 );

SELECT substr ( 'teststring' FROM 5 FOR 2 );

SELECT substr ( 'teststring' , 5 , 2 );

SELECT substr ( 'teststring' , 5 , 10 );

SELECT substrb ( 'string' , 2 );

SELECT substrb ( 'string' , - 2 );

SELECT substrb ( 'string' , 10 );

SELECT substrb ( '数据库' , 1 );

SELECT substrb ( '数据库' , 2 );

SELECT substrb ( 'string' , 2 , 3 );

SELECT substrb ( 'string' , 2 , 10 );

SELECT substrb ( '数据库' , 4 , 3 );

SELECT substrb ( '数据库' , 2 , 6 ) = ' 据' as result ;

SELECT substrb ( '数据库' , 2 , 6 ) = ' 据 ' as result ;

SELECT 'MPP' || 'DB' AS RESULT ;

SELECT 'Value: ' || 42 AS RESULT ;

SELECT split_part ( 'abc~@~def~@~ghi' , '~@~' , 2 );

SELECT strpos ( 'source' , 'rc' );

SELECT to_hex ( 2147483647 );

SELECT translate ( '12345' , '143' , 'ax' );

SELECT length ( 'abcd' );

SELECT length ( '汉字abc' );

CREATE DATABASE gaussdb_m WITH dbcompatibility 'MYSQL' ;

\ c gaussdb_m -- 设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

SELECT lengthb ( 'Chinese' );

select to_single_byte ( 'AB123' );

select to_multi_byte ( 'ABC123' );

SELECT trim ( BOTH 'x' FROM 'xTomxx' );

SELECT trim ( LEADING 'x' FROM 'xTomxx' );

SELECT trim ( TRAILING 'x' FROM 'xTomxx' );

SELECT rtrim ( 'TRIMxxxx' , 'x' );

SELECT ltrim ( 'xxxxTRIM' , 'x' );

SELECT upper ( 'tom' );

SELECT lower ( 'TOM' );

SELECT nls_upper ( 'gro?e' );

SELECT nls_upper ( 'gro?e' , 'nls_sort = XGerman' );

SELECT nls_lower ( 'INDIVISIBILITY' );

SELECT nls_lower ( 'INDIVISIBILITY' , 'nls_sort = XTurkish' );

SELECT instr ( 'corporate floor' , 'or' , 3 );

SELECT instr ( 'corporate floor' , 'or' , - 3 , 2 );

SELECT initcap ( 'hi THOMAS' );

SELECT ascii ( 'xyz' );

SELECT ascii2 ( 'xyz' );

select ascii2 ( '中xyz' );

SELECT asciistr ( 'xyz中' );

select unistr ( 'abc\0041\4E2D' );

select vsize ( 'abc测试' );

SELECT replace ( 'jack and jue' , 'j' , 'bl' );

SELECT concat ( 'Hello' , ' World!' );

SELECT concat ( 'Hello' , NULL );

CREATE TABLE test_space ( c char ( 10 ));

INSERT INTO test_space values ( 'a' );

SELECT * FROM test_space WHERE c = 'a ' ;

SELECT * FROM test_space WHERE c = 'a' || ' ' ;

SELECT chr ( 65 );

select chr ( 19968 );

SELECT chr ( 65 );

select chr ( 16705 );

select chr ( 4259905 );

SELECT nchr ( 65 );

select nchr ( 14989440 );

select nchr ( 14989440 );

select nchr ( 4321090 );

select nchr ( 14989440 );

select nchr ( 14989440 );

SELECT regexp_substr ( '500 Hello World, Redwood Shores, CA' , ',[^,]+,' ) "REGEXPR_SUBSTR" ;

SELECT regexp_replace ( 'Thomas' , '.[mN]a.' , 'M' );

SELECT regexp_replace ( 'foobarbaz' , 'b(..)' , E 'X\\1Y' , 'g' ) AS RESULT ;

SELECT regexp_replace('foobarbaz','b(..)', E'X\\1Y', 2, 2, 'n') AS RESULT;

SELECT concat_ws ( ',' , 'ABCDE' , 2 , NULL , 22 );

create table test ( a text );

insert into test ( a ) values ( 'abC 不' );

insert into test ( a ) values ( 'abC 啊' );

insert into test ( a ) values ( 'abc 啊' );

select * from test order by nlssort ( a , 'nls_sort=schinese_pinyin_m' );

select * from test order by nlssort ( a , 'nls_sort=generic_m_ci' );

SELECT convert ( 'text_in_utf8' , 'UTF8' , 'GBK' );

show server_encoding ;

SELECT convert_from ( 'some text' , 'GBK' );

SELECT convert ( 'asdas' using 'gbk' );

SELECT convert_from ( 'text_in_utf8' , 'UTF8' );

SELECT convert_to ( 'some text' , 'UTF8' );

SELECT 'AA_BBCC' LIKE '%A@_B%' ESCAPE '@' AS RESULT ;

SELECT 'AA_BBCC' LIKE '%A@_B%' AS RESULT ;

SELECT 'AA@_BBCC' LIKE '%A@_B%' AS RESULT ;

SELECT regexp_like ( 'ABC' , '[A-Z]' );

SELECT regexp_like ( 'ABC' , '[D-Z]' );

SELECT regexp_like ( 'ABC' , '[A-Z]' , 'i' );

SELECT regexp_like ( 'ABC' , '[A-Z]' );

SELECT format ( 'Hello %s, %1$s' , 'World' );

SELECT md5 ( 'ABC' );

select sha ( 'ABC' );

select sha1 ( 'ABC' );

select sha2 ( 'ABC' , 224 );

select sha2 ( 'ABC' , 256 );

select sha2 ( 'ABC' , 0 );

SELECT decode ( 'MTIzAAE=' , 'base64' );

select similar_escape('\s+ab','2');

select find_in_set ( 'ee' , 'a,ee,c' );

SELECT encode ( E '123\\000\\001' , 'base64' );

SET max_datanode_for_plan = 64 ;

EXPLAIN SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

CREATE EXTENSION pkg_bpchar_opc ;

SET max_datanode_for_plan = 64 ;

EXPLAIN SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

DROP EXTENSION pkg_bpchar_opc ;

SET max_datanode_for_plan = 64 ;

EXPLAIN SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: text ;

CREATE EXTENSION pkg_bpchar_opc ;

SET max_datanode_for_plan = 64 ;

explain select * from logs_text t1 where t1 . log_id = 'FE306991300002 ' :: bpchar ;

SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: text ;

DROP EXTENSION pkg_bpchar_opc ;

SET max_datanode_for_plan = 64 ;

SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

EXPLAIN SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = 'FE306991300002 ' ;

CREATE EXTENSION pkg_bpchar_opc ;

SET max_datanode_for_plan = 64 ;

SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

EXPLAIN SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = 'FE306991300002 ' ;

DROP EXTENSION pkg_bpchar_opc ;

