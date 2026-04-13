-- 类别: DQL
-- SQL 数量: 4180

-- 来源: 1015_Hint
select * from dbe_perf.global_plancache_status where schema_name='public' order by 1,2;

-- 来源: 1024_SQL PATCH
select * from hint_t1 where hint_t1 . a = 1 ;

-- 来源: 1024_SQL PATCH
select unique_query_id , query , query_plan from dbe_perf . statement_history where query like '%hint_t1%' ;

-- 来源: 1024_SQL PATCH
select * from dbe_sql_util . create_hint_sql_patch ( 'patch1' , 3929365485 , 'indexscan(hint_t1)' );

-- 来源: 1024_SQL PATCH
select * from hint_t1 where hint_t1 . a = 1 ;

-- 来源: 1024_SQL PATCH
select unique_query_id , query , query_plan from dbe_perf . statement_history where query like '%hint_t1%' ;

-- 来源: 1024_SQL PATCH
select * from dbe_sql_util.drop_sql_patch('patch1');

-- 来源: 1024_SQL PATCH
select * from dbe_sql_util.create_abort_sql_patch('patch2', 3929365485);

-- 来源: 1024_SQL PATCH
select * from hint_t1 t1 where t1.a = 1;

-- 来源: 1024_SQL PATCH
select b from test_proc_patch where a = 1;

-- 来源: 1024_SQL PATCH
select unique_query_id, query, query_plan, parent_unique_sql_id from dbe_perf.statement_history where query like '%call mypro();

-- 来源: 1024_SQL PATCH
select * from dbe_sql_util.create_abort_sql_patch('patch1',2859505004,2502737203);

-- 来源: 1024_SQL PATCH
select patch_name,unique_sql_id,parent_unique_sql_id,enable,abort,hint_string from gs_sql_patch where patch_name = 'patch1';

-- 来源: 1024_SQL PATCH
select b from test_proc_patch where a = 1;

--查看数据。
-- 来源: 1047_file_1047
SELECT * FROM int_type_t1;

-- 来源: 1047_file_1047
SELECT * FROM int_type_t2 ;

--查询表中的数据。
-- 来源: 1047_file_1047
SELECT * FROM decimal_type_t1;

-- 来源: 1047_file_1047
SELECT * FROM numeric_type_t1 ;

-- 来源: 1047_file_1047
SELECT * FROM smallserial_type_tab ;

-- 来源: 1047_file_1047
SELECT * FROM serial_type_tab ;

-- 来源: 1047_file_1047
SELECT * FROM bigserial_type_tab ;

-- 来源: 1047_file_1047
SELECT * FROM float_type_t2 ;

-- 来源: 1048_file_1048
SELECT '12.34' :: float8 :: numeric :: money ;

-- 来源: 1048_file_1048
SELECT '52093.89' :: money :: numeric :: float8 ;

-- 来源: 1049_file_1049
SELECT * FROM bool_type_t1 ;

-- 来源: 1049_file_1049
SELECT * FROM bool_type_t1 WHERE bt_col1 = 't' ;

--查询表中的数据。
-- 来源: 1050_file_1050
SELECT ct_col1, char_length(ct_col1) FROM char_type_t1;

-- 来源: 1050_file_1050
SELECT ct_col1 , char_length ( ct_col1 ) FROM char_type_t2 ;

-- 来源: 1051_file_1051
SELECT * FROM blob_type_t1 ;

-- 来源: 1052__
SELECT * FROM date_type_tab ;

-- 来源: 1052__
SELECT * FROM time_type_tab ;

-- 来源: 1052__
SELECT * FROM day_type_tab ;

-- 来源: 1052__
SELECT * FROM year_type_tab ;

-- 来源: 1052__
SELECT TIME 'allballs' ;

-- 来源: 1052__
SELECT * FROM date_type_tab ;

-- 来源: 1052__
SELECT * FROM date_type_tab ;

-- 来源: 1052__
SELECT time '04:05:06' ;

-- 来源: 1052__
SELECT time '04:05:06 PST' ;

-- 来源: 1052__
SELECT time with time zone '04:05:06 PST' ;

--查看数据。
-- 来源: 1052__
SELECT * FROM realtime_type_special;

-- 来源: 1052__
SELECT * FROM realtime_type_special WHERE col3 < 'infinity';

-- 来源: 1052__
SELECT * FROM realtime_type_special WHERE col3 > '-infinity';

-- 来源: 1052__
SELECT * FROM realtime_type_special WHERE col3 > 'now';

-- 来源: 1052__
SELECT * FROM realtime_type_special WHERE col3 = 'today';

-- 来源: 1052__
SELECT * FROM realtime_type_special WHERE col3 = 'tomorrow';

-- 来源: 1052__
SELECT * FROM realtime_type_special WHERE col3 > 'yesterday';

-- 来源: 1052__
SELECT * FROM reltime_type_tab ;

-- 来源: 1053_file_1053
SELECT point(1.1, 2.2);

-- 来源: 1053_file_1053
SELECT lseg(point(1.1, 2.2), point(3.3, 4.4));

-- 来源: 1053_file_1053
SELECT box(point(1.1, 2.2), point(3.3, 4.4));

-- 来源: 1053_file_1053
SELECT path(polygon '((0,0),(1,1),(2,0))');

-- 来源: 1053_file_1053
SELECT polygon(box '((0,0),(1,1))');

-- 来源: 1053_file_1053
SELECT circle(point(0,0),1);

-- 来源: 1055_file_1055
SELECT * FROM bit_type_t1 ;

-- 来源: 1056_file_1056
SELECT 'a fat cat sat on a mat and ate a fat rat' :: tsvector ;

-- 来源: 1056_file_1056
SELECT $$ the lexeme ' ' contains spaces $$ :: tsvector ;

-- 来源: 1056_file_1056
SELECT $$ the lexeme 'Joe''s' contains a quote $$ :: tsvector ;

-- 来源: 1056_file_1056
SELECT 'a:1 fat:2 cat:3 sat:4 on:5 a:6 mat:7 and:8 ate:9 a:10 fat:11 rat:12' :: tsvector ;

-- 来源: 1056_file_1056
SELECT 'a:1A fat:2B,4C cat:5D' :: tsvector ;

-- 来源: 1056_file_1056
SELECT 'The Fat Rats' :: tsvector ;

-- 来源: 1056_file_1056
SELECT to_tsvector ( 'english' , 'The Fat Rats' );

-- 来源: 1056_file_1056
SELECT 'fat & rat' :: tsquery ;

-- 来源: 1056_file_1056
SELECT 'fat & (rat | cat)' :: tsquery ;

-- 来源: 1056_file_1056
SELECT 'fat & rat & ! cat' :: tsquery ;

-- 来源: 1056_file_1056
SELECT 'fat:ab & cat' :: tsquery ;

-- 来源: 1056_file_1056
SELECT 'super:*' :: tsquery ;

-- 来源: 1056_file_1056
SELECT to_tsvector ( 'seriousness' ) @@ to_tsquery ( 'series:*' ) AS RESULT ;

-- 来源: 1056_file_1056
SELECT to_tsquery ( 'series:*' );

-- 来源: 1056_file_1056
SELECT to_tsquery ( 'Fat:ab & Cats' );

-- 来源: 1058_JSON_JSONB
SELECT 'null'::json;

-- 来源: 1058_JSON_JSONB
SELECT 'NULL'::jsonb;

-- 来源: 1058_JSON_JSONB
SELECT '1'::json;

-- 来源: 1058_JSON_JSONB
SELECT '-1.5'::json;

-- 来源: 1058_JSON_JSONB
SELECT '-1.5e-5'::jsonb, '-1.5e+2'::jsonb;

-- 来源: 1058_JSON_JSONB
SELECT '001'::json, '+15'::json, 'NaN'::json;

-- 来源: 1058_JSON_JSONB
SELECT 'true'::json;

-- 来源: 1058_JSON_JSONB
SELECT '"a"'::json;

-- 来源: 1058_JSON_JSONB
SELECT '[1, 2, "foo", null]'::json;

-- 来源: 1058_JSON_JSONB
SELECT '[]'::json;

-- 来源: 1058_JSON_JSONB
SELECT '[1, 2, "foo", null, [[]], {}]'::jsonb;

-- 来源: 1058_JSON_JSONB
SELECT '{}'::json;

-- 来源: 1058_JSON_JSONB
SELECT '{"foo": [true, "bar"], "tags": {"a": 1, "b": null}}'::jsonb;

-- 来源: 1058_JSON_JSONB
SELECT ' [1, " a ", {"a" :1 }] '::jsonb;

-- 来源: 1058_JSON_JSONB
SELECT '{"a" : 1, "a" : 2}'::jsonb;

-- 来源: 1058_JSON_JSONB
SELECT '{"aa" : 1, "b" : 2, "a" : 3}'::jsonb;

-- 来源: 1059_HLL
SELECT hll_cardinality ( set ) FROM helloworld WHERE id = 1 ;

-- 来源: 1059_HLL
SELECT date , hll_cardinality ( users ) FROM daily_uniques ORDER BY date ;

-- 来源: 1059_HLL
SELECT hll_cardinality ( hll_union_agg ( users )) FROM daily_uniques WHERE date >= '2019-02-20' :: date AND date <= '2019-02-26' :: date ;

-- 来源: 1059_HLL
SELECT date , ( # hll_union_agg ( users ) OVER two_days ) - # users AS lost_uniques FROM daily_uniques WINDOW two_days AS ( ORDER BY date ASC ROWS 1 PRECEDING );

-- 包含 。
-- 来源: 1060_file_1060
SELECT int4range(10, 20) @> 3;

-- 判断是否重叠
-- 来源: 1060_file_1060
SELECT numrange(11.1, 22.2) && numrange(20.0, 30.0);

-- 抽取上界 。
-- 来源: 1060_file_1060
SELECT upper(int8range(15, 25));

-- 计算交集 。
-- 来源: 1060_file_1060
SELECT int4range(10, 20) * int4range(15, 25);

-- 判断范围是否为空 。
-- 来源: 1060_file_1060
SELECT isempty(numrange(1, 5));

-- 来源: 1060_file_1060
SELECT '[3,7)'::int4range;

-- 既不包括 3 也不包括 7，但是包括之间的所有点 。
-- 来源: 1060_file_1060
SELECT '(3,7)'::int4range;

-- 只包括单独一个点 4 。
-- 来源: 1060_file_1060
SELECT '[4,4]'::int4range;

-- 不包括点（并且将被标准化为 '空'） 。
-- 来源: 1060_file_1060
SELECT '[4,4)'::int4range;

-- 来源: 1060_file_1060
SELECT numrange(1.0, 14.0, '(]');

-- 如果第三个参数被忽略，则假定为 '[)'。
-- 来源: 1060_file_1060
SELECT numrange(1.0, 14.0);

-- 尽管这里指定了 '(]'，显示时该值将被转换成标准形式，因为 int8range 是一种离散范围类型（见下文）。
-- 来源: 1060_file_1060
SELECT int8range(1, 14, '(]');

-- 为一个界限使用 NULL 导致范围在那一边是无界的。
-- 来源: 1060_file_1060
SELECT numrange(NULL, 2.2);

-- 来源: 1061_file_1061
SELECT oid FROM pg_class WHERE relname = 'pg_type' ;

-- 来源: 1061_file_1061
SELECT attrelid , attname , atttypid , attstattarget FROM pg_attribute WHERE attrelid = 'pg_type' :: REGCLASS ;

-- 来源: 1062_file_1062
SELECT showall ();

-- 来源: 1065_XML
SELECT * FROM xmltest ORDER BY 1;

-- 来源: 1065_XML
SELECT xmlconcat(xmlcomment('hello'), xmlelement(NAME qux, 'xml'), xmlcomment('world'));

-- 来源: 1066_XMLTYPE
SELECT * FROM xmltypetest ORDER BY 1;

-- 来源: 1067_aclitem
SELECT * FROM table_acl;

-- 来源: 1068_file_1068
SELECT CURRENT_ROLE ;

-- 来源: 1068_file_1068
SELECT CURRENT_SCHEMA ;

-- 来源: 1068_file_1068
SELECT CURRENT_USER ;

-- 来源: 1068_file_1068
SELECT LOCALTIMESTAMP ;

-- 来源: 1068_file_1068
SELECT SESSION_USER ;

-- 来源: 1068_file_1068
SELECT SYSDATE ;

-- 来源: 1068_file_1068
SELECT USER ;

-- 来源: 1072_file_1072
SELECT bit_length ( 'world' );

-- 来源: 1072_file_1072
SELECT btrim ( 'sring' , 'ing' );

-- 来源: 1072_file_1072
SELECT char_length ( 'hello' );

-- 来源: 1072_file_1072
select dump ( 'abc测试' );

-- 来源: 1072_file_1072
SELECT instr ( 'abcdabcdabcd' , 'bcd' , 2 , 2 );

-- 来源: 1072_file_1072
SELECT instrb ( 'abcdabcdabcd' , 'bcd' , 2 , 2 );

-- 来源: 1072_file_1072
SELECT lengthb ( 'hello' );

-- 来源: 1072_file_1072
SELECT left ( 'abcde' , 2 );

-- 来源: 1072_file_1072
SELECT length ( 'jose' , 'UTF8' );

-- 来源: 1072_file_1072
SELECT lpad ( 'hi' , 5 , 'xyza' );

-- 来源: 1072_file_1072
select lpad ( 'expr1' , 7 , '中国' );

-- 来源: 1072_file_1072
select lpad ( 'expr1' , 8 , '中国' );

-- 来源: 1072_file_1072
SELECT notlike ( 1 , 2 );

-- 来源: 1072_file_1072
SELECT notlike ( 1 , 1 );

-- 来源: 1072_file_1072
SELECT octet_length ( 'jose' );

-- 来源: 1072_file_1072
SELECT overlay ( 'hello' placing 'world' from 2 for 3 );

-- 来源: 1072_file_1072
SELECT position ( 'ing' in 'string' );

-- 来源: 1072_file_1072
SELECT pg_client_encoding ();

-- 来源: 1072_file_1072
SELECT quote_ident ( 'hello world' );

-- 来源: 1072_file_1072
SELECT quote_literal ( 'hello' );

-- 来源: 1072_file_1072
SELECT quote_literal ( E 'O\' hello ');

-- 来源: 1072_file_1072
SELECT quote_literal ( 'O\hello' );

-- 来源: 1072_file_1072
SELECT quote_literal ( NULL );

-- 来源: 1072_file_1072
SELECT quote_literal ( 42 . 5 );

-- 来源: 1072_file_1072
SELECT quote_literal ( E 'O\' 42 . 5 ');

-- 来源: 1072_file_1072
SELECT quote_literal ( 'O\42.5' );

-- 来源: 1072_file_1072
SELECT quote_nullable ( 'hello' );

-- 来源: 1072_file_1072
SELECT quote_nullable ( E 'O\' hello ');

-- 来源: 1072_file_1072
SELECT quote_nullable ( 'O\hello' );

-- 来源: 1072_file_1072
SELECT quote_nullable ( NULL );

-- 来源: 1072_file_1072
SELECT quote_nullable ( 42 . 5 );

-- 来源: 1072_file_1072
SELECT quote_nullable ( E 'O\' 42 . 5 ');

-- 来源: 1072_file_1072
SELECT quote_nullable ( 'O\42.5' );

-- 来源: 1072_file_1072
SELECT quote_nullable ( NULL );

-- 来源: 1072_file_1072
select substring_inner ( 'adcde' , 2 , 3 );

-- 来源: 1072_file_1072
SELECT substring ( 'Thomas' from 2 for 3 );

-- 来源: 1072_file_1072
select substring ( 'substrteststring' , - 5 , 5 );

-- 来源: 1072_file_1072
SELECT substring_index ( 'Test1splitTest2splitTest3splitTest4' , 'split' , 2 );

-- 来源: 1072_file_1072
SELECT substring_index ( 'Test1splitTest2splitTest3splitTest4' , 'split' , - 2 );

-- 来源: 1072_file_1072
SELECT substring ( 'Thomas' from '...$' );

-- 来源: 1072_file_1072
SELECT substring ( 'foobar' from 'o(.)b' );

-- 来源: 1072_file_1072
SELECT substring ( 'foobar' from '(o(.)b)' );

-- 来源: 1072_file_1072
SELECT substring ( 'Thomas' from '%#"o_a#"_' for '#' );

-- 来源: 1072_file_1072
SELECT rawcat ( 'ab' , 'cd' );

-- 来源: 1072_file_1072
SELECT regexp_like ( 'str' , '[ac]' );

-- 来源: 1072_file_1072
SELECT regexp_substr ( 'str' , '[ac]' );

-- 来源: 1072_file_1072
SELECT regexp_substr ( 'foobarbaz' , 'b(..)' , 3 , 2 ) AS RESULT ;

-- 来源: 1072_file_1072
SELECT regexp_count('foobarbaz','b(..)', 5) AS RESULT;

-- 来源: 1072_file_1072
SELECT regexp_instr('foobarbaz','b(..)', 1, 1, 0) AS RESULT;

-- 来源: 1072_file_1072
SELECT regexp_instr('foobarbaz','b(..)', 1, 2, 0) AS RESULT;

-- 来源: 1072_file_1072
SELECT regexp_matches ( 'foobarbequebaz' , '(bar)(beque)' );

-- 来源: 1072_file_1072
SELECT regexp_matches ( 'foobarbequebaz' , 'barbeque' );

-- 来源: 1072_file_1072
SELECT regexp_matches ( 'foobarbequebazilbarfbonk' , '(b[^b]+)(b[^b]+)' , 'g' );

-- 来源: 1072_file_1072
SELECT regexp_match('foobarbequebaz', '(bar)(beque)');

-- 来源: 1072_file_1072
SELECT (regexp_match('foobarbequebaz', 'bar.*que'))[1];

-- 来源: 1072_file_1072
SELECT regexp_match('Learning #PostgreSQL', 'R', 'c');

-- 来源: 1072_file_1072
SELECT regexp_match('hello world', 'h e l l o', 'x');

-- 来源: 1072_file_1072
SELECT regexp_split_to_array ( 'hello world' , E '\\s+' );

-- 来源: 1072_file_1072
SELECT regexp_split_to_table ( 'hello world' , E '\\s+' );

-- 来源: 1072_file_1072
SELECT repeat ( 'Pg' , 4 );

-- 来源: 1072_file_1072
SELECT replace ( 'abcdefabcdef' , 'cd' , 'XXX' );

-- 来源: 1072_file_1072
SELECT replace ( 'abcdefabcdef' , 'cd' );

-- 来源: 1072_file_1072
SELECT reverse ( 'abcde' );

-- 来源: 1072_file_1072
SELECT right ( 'abcde' , 2 );

-- 来源: 1072_file_1072
SELECT right ( 'abcde' , - 2 );

-- 来源: 1072_file_1072
SELECT rpad ( 'hi' , 5 , 'xy' );

-- 来源: 1072_file_1072
select rpad ( 'expr1' , 7 , '中国' ) || '*' ;

-- 来源: 1072_file_1072
select rpad ( 'expr1' , 8 , '中国' ) || '*' ;

-- 来源: 1072_file_1072
SELECT substr ( 'stringtest' FROM 4 );

-- 来源: 1072_file_1072
SELECT substr ( 'stringtest' , 4 );

-- 来源: 1072_file_1072
SELECT substr ( 'stringtest' , - 4 );

-- 来源: 1072_file_1072
SELECT substr ( 'stringtest' , 11 );

-- 来源: 1072_file_1072
SELECT substr ( 'teststring' FROM 5 FOR 2 );

-- 来源: 1072_file_1072
SELECT substr ( 'teststring' , 5 , 2 );

-- 来源: 1072_file_1072
SELECT substr ( 'teststring' , 5 , 10 );

-- 来源: 1072_file_1072
SELECT substrb ( 'string' , 2 );

-- 来源: 1072_file_1072
SELECT substrb ( 'string' , - 2 );

-- 来源: 1072_file_1072
SELECT substrb ( 'string' , 10 );

-- 来源: 1072_file_1072
SELECT substrb ( '数据库' , 1 );

-- 来源: 1072_file_1072
SELECT substrb ( '数据库' , 2 );

-- 来源: 1072_file_1072
SELECT substrb ( 'string' , 2 , 3 );

-- 来源: 1072_file_1072
SELECT substrb ( 'string' , 2 , 10 );

-- 来源: 1072_file_1072
SELECT substrb ( '数据库' , 4 , 3 );

-- 来源: 1072_file_1072
SELECT substrb ( '数据库' , 2 , 6 ) = ' 据' as result ;

-- 来源: 1072_file_1072
SELECT substrb ( '数据库' , 2 , 6 ) = ' 据 ' as result ;

-- 来源: 1072_file_1072
SELECT 'MPP' || 'DB' AS RESULT ;

-- 来源: 1072_file_1072
SELECT 'Value: ' || 42 AS RESULT ;

-- 来源: 1072_file_1072
SELECT split_part ( 'abc~@~def~@~ghi' , '~@~' , 2 );

-- 来源: 1072_file_1072
SELECT strpos ( 'source' , 'rc' );

-- 来源: 1072_file_1072
SELECT to_hex ( 2147483647 );

-- 来源: 1072_file_1072
SELECT translate ( '12345' , '143' , 'ax' );

-- 来源: 1072_file_1072
SELECT length ( 'abcd' );

-- 来源: 1072_file_1072
SELECT length ( '汉字abc' );

-- 来源: 1072_file_1072
SELECT lengthb ( 'Chinese' );

-- 来源: 1072_file_1072
select to_single_byte ( 'AB123' );

-- 来源: 1072_file_1072
select to_multi_byte ( 'ABC123' );

-- 来源: 1072_file_1072
SELECT trim ( BOTH 'x' FROM 'xTomxx' );

-- 来源: 1072_file_1072
SELECT trim ( LEADING 'x' FROM 'xTomxx' );

-- 来源: 1072_file_1072
SELECT trim ( TRAILING 'x' FROM 'xTomxx' );

-- 来源: 1072_file_1072
SELECT rtrim ( 'TRIMxxxx' , 'x' );

-- 来源: 1072_file_1072
SELECT ltrim ( 'xxxxTRIM' , 'x' );

-- 来源: 1072_file_1072
SELECT upper ( 'tom' );

-- 来源: 1072_file_1072
SELECT lower ( 'TOM' );

-- 来源: 1072_file_1072
SELECT nls_upper ( 'gro?e' );

-- 来源: 1072_file_1072
SELECT nls_upper ( 'gro?e' , 'nls_sort = XGerman' );

-- 来源: 1072_file_1072
SELECT nls_lower ( 'INDIVISIBILITY' );

-- 来源: 1072_file_1072
SELECT nls_lower ( 'INDIVISIBILITY' , 'nls_sort = XTurkish' );

-- 来源: 1072_file_1072
SELECT instr ( 'corporate floor' , 'or' , 3 );

-- 来源: 1072_file_1072
SELECT instr ( 'corporate floor' , 'or' , - 3 , 2 );

-- 来源: 1072_file_1072
SELECT initcap ( 'hi THOMAS' );

-- 来源: 1072_file_1072
SELECT ascii ( 'xyz' );

-- 来源: 1072_file_1072
SELECT ascii2 ( 'xyz' );

-- 来源: 1072_file_1072
select ascii2 ( '中xyz' );

-- 来源: 1072_file_1072
SELECT asciistr ( 'xyz中' );

-- 来源: 1072_file_1072
select unistr ( 'abc\0041\4E2D' );

-- 来源: 1072_file_1072
select vsize ( 'abc测试' );

-- 来源: 1072_file_1072
SELECT replace ( 'jack and jue' , 'j' , 'bl' );

-- 来源: 1072_file_1072
SELECT concat ( 'Hello' , ' World!' );

-- 来源: 1072_file_1072
SELECT concat ( 'Hello' , NULL );

-- 来源: 1072_file_1072
SELECT * FROM test_space WHERE c = 'a ' ;

-- 来源: 1072_file_1072
SELECT * FROM test_space WHERE c = 'a' || ' ' ;

-- 来源: 1072_file_1072
SELECT chr ( 65 );

-- 来源: 1072_file_1072
select chr ( 19968 );

-- 来源: 1072_file_1072
SELECT chr ( 65 );

-- 来源: 1072_file_1072
select chr ( 16705 );

-- 来源: 1072_file_1072
select chr ( 4259905 );

-- 来源: 1072_file_1072
SELECT nchr ( 65 );

-- 来源: 1072_file_1072
select nchr ( 14989440 );

-- 来源: 1072_file_1072
select nchr ( 14989440 );

-- 来源: 1072_file_1072
select nchr ( 4321090 );

-- 来源: 1072_file_1072
select nchr ( 14989440 );

-- 来源: 1072_file_1072
select nchr ( 14989440 );

-- 来源: 1072_file_1072
SELECT regexp_substr ( '500 Hello World, Redwood Shores, CA' , ',[^,]+,' ) "REGEXPR_SUBSTR" ;

-- 来源: 1072_file_1072
SELECT regexp_replace ( 'Thomas' , '.[mN]a.' , 'M' );

-- 来源: 1072_file_1072
SELECT regexp_replace ( 'foobarbaz' , 'b(..)' , E 'X\\1Y' , 'g' ) AS RESULT ;

-- 来源: 1072_file_1072
SELECT regexp_replace('foobarbaz','b(..)', E'X\\1Y', 2, 2, 'n') AS RESULT;

-- 来源: 1072_file_1072
SELECT concat_ws ( ',' , 'ABCDE' , 2 , NULL , 22 );

-- 来源: 1072_file_1072
select * from test order by nlssort ( a , 'nls_sort=schinese_pinyin_m' );

-- 来源: 1072_file_1072
select * from test order by nlssort ( a , 'nls_sort=generic_m_ci' );

-- 来源: 1072_file_1072
SELECT convert ( 'text_in_utf8' , 'UTF8' , 'GBK' );

-- 来源: 1072_file_1072
SELECT convert_from ( 'some text' , 'GBK' );

-- 来源: 1072_file_1072
SELECT convert ( 'asdas' using 'gbk' );

-- 来源: 1072_file_1072
SELECT convert_from ( 'text_in_utf8' , 'UTF8' );

-- 来源: 1072_file_1072
SELECT convert_to ( 'some text' , 'UTF8' );

-- 来源: 1072_file_1072
SELECT 'AA_BBCC' LIKE '%A@_B%' ESCAPE '@' AS RESULT ;

-- 来源: 1072_file_1072
SELECT 'AA_BBCC' LIKE '%A@_B%' AS RESULT ;

-- 来源: 1072_file_1072
SELECT 'AA@_BBCC' LIKE '%A@_B%' AS RESULT ;

-- 来源: 1072_file_1072
SELECT regexp_like ( 'ABC' , '[A-Z]' );

-- 来源: 1072_file_1072
SELECT regexp_like ( 'ABC' , '[D-Z]' );

-- 来源: 1072_file_1072
SELECT regexp_like ( 'ABC' , '[A-Z]' , 'i' );

-- 来源: 1072_file_1072
SELECT regexp_like ( 'ABC' , '[A-Z]' );

-- 来源: 1072_file_1072
SELECT format ( 'Hello %s, %1$s' , 'World' );

-- 来源: 1072_file_1072
SELECT md5 ( 'ABC' );

-- 来源: 1072_file_1072
select sha ( 'ABC' );

-- 来源: 1072_file_1072
select sha1 ( 'ABC' );

-- 来源: 1072_file_1072
select sha2 ( 'ABC' , 224 );

-- 来源: 1072_file_1072
select sha2 ( 'ABC' , 256 );

-- 来源: 1072_file_1072
select sha2 ( 'ABC' , 0 );

-- 来源: 1072_file_1072
SELECT decode ( 'MTIzAAE=' , 'base64' );

-- 来源: 1072_file_1072
select similar_escape('\s+ab','2');

-- 来源: 1072_file_1072
select find_in_set ( 'ee' , 'a,ee,c' );

-- 来源: 1072_file_1072
SELECT encode ( E '123\\000\\001' , 'base64' );

-- 来源: 1072_file_1072
SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- 来源: 1072_file_1072
SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- 来源: 1072_file_1072
SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- 来源: 1072_file_1072
SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: text ;

-- 来源: 1072_file_1072
SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- 来源: 1072_file_1072
SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: text ;

-- 来源: 1072_file_1072
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- 来源: 1072_file_1072
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = 'FE306991300002 ' ;

-- 来源: 1072_file_1072
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- 来源: 1072_file_1072
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = 'FE306991300002 ' ;

-- 来源: 1073_file_1073
SELECT octet_length ( E 'jo\\000se' :: bytea ) AS RESULT ;

-- 来源: 1073_file_1073
SELECT overlay ( E 'Th\\000omas' :: bytea placing E '\\002\\003' :: bytea from 2 for 3 ) AS RESULT ;

-- 来源: 1073_file_1073
SELECT position ( E '\\000om' :: bytea in E 'Th\\000omas' :: bytea ) AS RESULT ;

-- 来源: 1073_file_1073
SELECT substring ( E 'Th\\000omas' :: bytea from 2 for 3 ) AS RESULT ;

-- 来源: 1073_file_1073
select substr ( E 'Th\\000omas' :: bytea , 2 , 3 ) as result ;

-- 来源: 1073_file_1073
SELECT trim ( E '\\000' :: bytea from E '\\000Tom\\000' :: bytea ) AS RESULT ;

-- 来源: 1073_file_1073
SELECT btrim ( E '\\000trim\\000' :: bytea , E '\\000' :: bytea ) AS RESULT ;

-- 来源: 1073_file_1073
SELECT get_bit ( E 'Th\\000omas' :: bytea , 45 ) AS RESULT ;

-- 来源: 1073_file_1073
SELECT get_byte ( E 'Th\\000omas' :: bytea , 4 ) AS RESULT ;

-- 来源: 1073_file_1073
SELECT set_bit ( E 'Th\\000omas' :: bytea , 45 , 0 ) AS RESULT ;

-- 来源: 1073_file_1073
SELECT set_byte ( E 'Th\\000omas' :: bytea , 4 , 64 ) AS RESULT ;

-- 来源: 1074_file_1074
SELECT B '10001' || B '011' AS RESULT ;

-- 来源: 1074_file_1074
SELECT B '10001' & B '01101' AS RESULT ;

-- 来源: 1074_file_1074
SELECT B '10001' | B '01101' AS RESULT ;

-- 来源: 1074_file_1074
SELECT B '10001' # B '01101' AS RESULT ;

-- 来源: 1074_file_1074
SELECT ~ B '10001' AS RESULT ;

-- 来源: 1074_file_1074
SELECT B '10001' << 3 AS RESULT ;

-- 来源: 1074_file_1074
SELECT B '10001' >> 2 AS RESULT ;

-- 来源: 1074_file_1074
SELECT 44 :: bit ( 10 ) AS RESULT ;

-- 来源: 1074_file_1074
SELECT 44 :: bit ( 3 ) AS RESULT ;

-- 来源: 1074_file_1074
SELECT cast ( - 44 as bit ( 12 )) AS RESULT ;

-- 来源: 1074_file_1074
SELECT '1110' :: bit ( 4 ):: integer AS RESULT ;

-- 来源: 1074_file_1074
select substring ( '10101111' :: bit ( 8 ), 2 );

-- 来源: 1075_file_1075
SELECT 'abc' LIKE 'abc' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' LIKE 'a%' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' LIKE '_b_' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' LIKE 'c' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' SIMILAR TO 'abc' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' SIMILAR TO 'a' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' SIMILAR TO '%(b|d)%' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' SIMILAR TO '(b|c)%' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' ~ 'Abc' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' ~* 'Abc' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' !~ 'Abc' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' !~* 'Abc' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' ~ '^a' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' ~ '(b|d)' AS RESULT ;

-- 来源: 1075_file_1075
SELECT 'abc' ~ '^(b|c)' AS RESULT ;

-- 来源: 1076_file_1076
SELECT 2 + 3 AS RESULT ;

-- 来源: 1076_file_1076
SELECT 2 - 3 AS RESULT ;

-- 来源: 1076_file_1076
SELECT 2 * 3 AS RESULT ;

-- 来源: 1076_file_1076
SELECT 4 / 2 AS RESULT ;

-- 来源: 1076_file_1076
SELECT 4 / 3 AS RESULT ;

-- 来源: 1076_file_1076
SELECT - 2 AS RESULT ;

-- 来源: 1076_file_1076
SELECT 5 % 4 AS RESULT ;

-- 来源: 1076_file_1076
SELECT @ - 5 . 0 AS RESULT ;

-- 来源: 1076_file_1076
SELECT 2 . 0 ^ 3 . 0 AS RESULT ;

-- 来源: 1076_file_1076
SELECT |/ 25 . 0 AS RESULT ;

-- 来源: 1076_file_1076
SELECT ||/ 27 . 0 AS RESULT ;

-- 来源: 1076_file_1076
SELECT 5 ! AS RESULT ;

-- 来源: 1076_file_1076
SELECT !! 5 AS RESULT ;

-- 来源: 1076_file_1076
SELECT 91 & 15 AS RESULT ;

-- 来源: 1076_file_1076
SELECT 32 | 3 AS RESULT ;

-- 来源: 1076_file_1076
SELECT 17 # 5 AS RESULT ;

-- 来源: 1076_file_1076
SELECT ~ 1 AS RESULT ;

-- 来源: 1076_file_1076
SELECT 1 << 4 AS RESULT ;

-- 来源: 1076_file_1076
SELECT 8 >> 2 AS RESULT ;

-- 来源: 1076_file_1076
SELECT abs ( - 17 . 4 );

-- 来源: 1076_file_1076
SELECT acos ( - 1 );

-- 来源: 1076_file_1076
SELECT asin ( 0 . 5 );

-- 来源: 1076_file_1076
SELECT atan ( 1 );

-- 来源: 1076_file_1076
SELECT atan2 ( 2 , 1 );

-- 来源: 1076_file_1076
SELECT bitand ( 127 , 63 );

-- 来源: 1076_file_1076
SELECT cbrt ( 27 . 0 );

-- 来源: 1076_file_1076
SELECT ceil ( - 42 . 8 );

-- 来源: 1076_file_1076
SELECT ceiling ( - 95 . 3 );

-- 来源: 1076_file_1076
SELECT cos ( - 3 . 1415927 );

-- 来源: 1076_file_1076
SELECT cosh ( 4 );

-- 来源: 1076_file_1076
SELECT cot ( 1 );

-- 来源: 1076_file_1076
SELECT degrees ( 0 . 5 );

-- 来源: 1076_file_1076
SELECT div ( 9 , 4 );

-- 来源: 1076_file_1076
SELECT exp ( 1 . 0 );

-- 来源: 1076_file_1076
SELECT floor ( - 42 . 8 );

-- 来源: 1076_file_1076
select int1 ( '123' );

-- 来源: 1076_file_1076
select int1 ( '1.1' );

-- 来源: 1076_file_1076
select int2 ( '1234' );

-- 来源: 1076_file_1076
select int2 ( 25 . 3 );

-- 来源: 1076_file_1076
select int4 ( '789' );

-- 来源: 1076_file_1076
select int4 ( 99 . 9 );

-- 来源: 1076_file_1076
select int8 ( '789' );

-- 来源: 1076_file_1076
select int8 ( 99 . 9 );

-- 来源: 1076_file_1076
select float4 ( '789' );

-- 来源: 1076_file_1076
select float4 ( 99 . 9 );

-- 来源: 1076_file_1076
select float8 ( '789' );

-- 来源: 1076_file_1076
select float8 ( 99 . 9 );

-- 来源: 1076_file_1076
select int16 ( '789' );

-- 来源: 1076_file_1076
select int16 ( 99 . 9 );

-- 来源: 1076_file_1076
select "numeric" ( '789' );

-- 来源: 1076_file_1076
select "numeric" ( 99 . 9 );

-- 来源: 1076_file_1076
SELECT radians ( 45 . 0 );

-- 来源: 1076_file_1076
SELECT random ();

-- 来源: 1076_file_1076
SELECT multiply ( 9 . 0 , '3.0' );

-- 来源: 1076_file_1076
SELECT multiply ( '9.0' , 3 . 0 );

-- 来源: 1076_file_1076
SELECT ln ( 2 . 0 );

-- 来源: 1076_file_1076
SELECT log ( 100 . 0 );

-- 来源: 1076_file_1076
SELECT log ( 2 . 0 , 64 . 0 );

-- 来源: 1076_file_1076
SELECT mod ( 9 , 4 );

-- 来源: 1076_file_1076
SELECT mod ( 9 , 0 );

-- 来源: 1076_file_1076
SELECT pi ();

-- 来源: 1076_file_1076
SELECT power ( 9 . 0 , 3 . 0 );

-- 来源: 1076_file_1076
SELECT remainder ( 11 , 4 );

-- 来源: 1076_file_1076
SELECT remainder ( 9 , 0 );

-- 来源: 1076_file_1076
SELECT round ( 42 . 4 );

-- 来源: 1076_file_1076
SELECT round ( 42 . 6 );

-- 来源: 1076_file_1076
SELECT round ( - 0 . 2 :: float8 );

-- 来源: 1076_file_1076
SELECT round ( 42 . 4382 , 2 );

-- 来源: 1076_file_1076
SELECT setseed ( 0 . 54823 );

-- 来源: 1076_file_1076
SELECT sign ( - 8 . 4 );

-- 来源: 1076_file_1076
SELECT sin ( 1 . 57079 );

-- 来源: 1076_file_1076
SELECT sinh ( 4 );

-- 来源: 1076_file_1076
SELECT sqrt ( 2 . 0 );

-- 来源: 1076_file_1076
SELECT tan ( 20 );

-- 来源: 1076_file_1076
SELECT tanh ( 0 . 1 );

-- 来源: 1076_file_1076
SELECT trunc ( 42 . 8 );

-- 来源: 1076_file_1076
SELECT trunc ( 42 . 4382 , 2 );

-- 来源: 1076_file_1076
SELECT width_bucket ( 5 . 35 , 0 . 024 , 10 . 06 , 5 );

-- 来源: 1076_file_1076
SELECT width_bucket ( 5 . 35 , 0 . 024 , 10 . 06 , 5 );

-- 来源: 1076_file_1076
SELECT nanvl('NaN', 1.1);

-- 来源: 1076_file_1076
SELECT numeric_eq_text(1, '1');

-- 来源: 1076_file_1076
SELECT text_eq_numeric('1', 1);

-- 来源: 1076_file_1076
SELECT bigint_eq_text(1, '1');

-- 来源: 1076_file_1076
SELECT text_eq_bigint('1', 1);

-- 来源: 1077_file_1077
SELECT date '2001-10-01' - '7' AS RESULT ;

-- 来源: 1077_file_1077
SELECT date '2001-9-28' + integer '7' AS RESULT ;

-- 来源: 1077_file_1077
SELECT date '2001-09-28' + interval '1 hour' AS RESULT ;

-- 来源: 1077_file_1077
SELECT date '2001-09-28' + time '03:00' AS RESULT ;

-- 来源: 1077_file_1077
SELECT interval '1 day' + interval '1 hour' AS RESULT ;

-- 来源: 1077_file_1077
SELECT timestamp '2001-09-28 01:00' + interval '23 hours' AS RESULT ;

-- 来源: 1077_file_1077
SELECT time '01:00' + interval '3 hours' AS RESULT ;

-- 来源: 1077_file_1077
SELECT date '2001-10-01' - date '2001-09-28' AS RESULT ;

-- 来源: 1077_file_1077
SELECT date '2001-10-01' - integer '7' AS RESULT ;

-- 来源: 1077_file_1077
SELECT date '2001-09-28' - interval '1 hour' AS RESULT ;

-- 来源: 1077_file_1077
SELECT time '05:00' - time '03:00' AS RESULT ;

-- 来源: 1077_file_1077
SELECT time '05:00' - interval '2 hours' AS RESULT ;

-- 来源: 1077_file_1077
SELECT timestamp '2001-09-28 23:00' - interval '23 hours' AS RESULT ;

-- 来源: 1077_file_1077
SELECT interval '1 day' - interval '1 hour' AS RESULT ;

-- 来源: 1077_file_1077
SELECT timestamp '2001-09-29 03:00' - timestamp '2001-09-27 12:00' AS RESULT ;

-- 来源: 1077_file_1077
SELECT 900 * interval '1 second' AS RESULT ;

-- 来源: 1077_file_1077
SELECT 21 * interval '1 day' AS RESULT ;

-- 来源: 1077_file_1077
SELECT double precision '3.5' * interval '1 hour' AS RESULT ;

-- 来源: 1077_file_1077
SELECT interval '1 hour' / double precision '1.5' AS RESULT ;

-- 来源: 1077_file_1077
SELECT age ( timestamp '2001-04-10' , timestamp '1957-06-13' );

-- 来源: 1077_file_1077
SELECT age ( timestamp '1957-06-13' );

-- 来源: 1077_file_1077
SELECT clock_timestamp ();

-- 来源: 1077_file_1077
SELECT current_date ;

-- 来源: 1077_file_1077
SELECT current_time ;

-- 来源: 1077_file_1077
SELECT current_timestamp ;

-- 来源: 1077_file_1077
SELECT current_timestamp ;

-- 来源: 1077_file_1077
SELECT current_timestamp ();

-- 来源: 1077_file_1077
SELECT current_timestamp ( 1 );

-- 来源: 1077_file_1077
SELECT current_timestamp ( 1 );

-- 来源: 1077_file_1077
SELECT pg_systimestamp ();

-- 来源: 1077_file_1077
SELECT date_part ( 'hour' , timestamp '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT date_part ( 'month' , interval '2 years 3 months' );

-- 来源: 1077_file_1077
SELECT date_trunc ( 'hour' , timestamp '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT trunc ( timestamp '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT trunc ( timestamp '2001-02-16 20:38:40' , 'hour' );

-- 来源: 1077_file_1077
SELECT round ( timestamp '2001-02-16 20:38:40' , 'hour' );

-- 来源: 1077_file_1077
SELECT daterange ( '2000-05-06' , '2000-08-08' );

-- 来源: 1077_file_1077
SELECT daterange ( '2000-05-06' , '2000-08-08' , '[]' );

-- 来源: 1077_file_1077
SELECT isfinite ( date '2001-02-16' );

-- 来源: 1077_file_1077
SELECT isfinite ( date 'infinity' );

-- 来源: 1077_file_1077
SELECT isfinite ( timestamp '2001-02-16 21:28:30' );

-- 来源: 1077_file_1077
SELECT isfinite ( timestamp 'infinity' );

-- 来源: 1077_file_1077
SELECT isfinite ( interval '4 hours' );

-- 来源: 1077_file_1077
SELECT justify_days ( interval '35 days' );

-- 来源: 1077_file_1077
SELECT JUSTIFY_HOURS ( INTERVAL '27 HOURS' );

-- 来源: 1077_file_1077
SELECT JUSTIFY_INTERVAL ( INTERVAL '1 MON -1 HOUR' );

-- 来源: 1077_file_1077
SELECT localtime AS RESULT ;

-- 来源: 1077_file_1077
SELECT localtimestamp ;

-- 来源: 1077_file_1077
SELECT maketime ( 8 , 15 , 26 . 53 );

-- 来源: 1077_file_1077
SELECT maketime ( - 888 , 15 , 26 . 53 );

-- 来源: 1077_file_1077
SELECT now ();

-- 来源: 1077_file_1077
SELECT timenow ();

-- 来源: 1077_file_1077
SELECT dbtimezone ;

-- 来源: 1077_file_1077
SELECT numtodsinterval ( 100 , 'HOUR' );

-- 来源: 1077_file_1077
SELECT numtodsinterval ( 100 , 'HOUR' );

-- 来源: 1077_file_1077
SELECT numtoyminterval ( 100 , 'MONTH' );

-- 来源: 1077_file_1077
SELECT numtodsinterval ( 100 , 'MONTH' );

-- 来源: 1077_file_1077
SELECT new_time ( '1997-10-10' , 'AST' , 'EST' );

-- 来源: 1077_file_1077
SELECT NEW_TIME ( TO_TIMESTAMP ( '10-Sep-02 14:10:10.123000' , 'DD-Mon-RR HH24:MI:SS.FF' ), 'AST' , 'PST' );

-- 来源: 1077_file_1077
SELECT SESSIONTIMEZONE ;

-- 来源: 1077_file_1077
SELECT LOWER ( SESSIONTIMEZONE );

-- 来源: 1077_file_1077
SELECT SYS_EXTRACT_UTC ( TIMESTAMP '2000-03-28 11:30:00.00' );

-- 来源: 1077_file_1077
SELECT SYS_EXTRACT_UTC ( TIMESTAMPTZ '2000-03-28 11:30:00.00 -08:00' );

-- 来源: 1077_file_1077
SELECT TZ_OFFSET ( 'US/Pacific' );

-- 来源: 1077_file_1077
SELECT TZ_OFFSET ( sessiontimezone );

-- 来源: 1077_file_1077
SELECT pg_sleep ( 10 );

-- 来源: 1077_file_1077
SELECT statement_timestamp ();

-- 来源: 1077_file_1077
SELECT sysdate ;

-- 来源: 1077_file_1077
SELECT current_sysdate ();

-- 来源: 1077_file_1077
SELECT timeofday ();

-- 来源: 1077_file_1077
SELECT transaction_timestamp ();

-- 来源: 1077_file_1077
SELECT transaction_timestamp ();

-- 来源: 1077_file_1077
SELECT add_months ( to_date ( '2017-5-29' , 'yyyy-mm-dd' ), 11 ) FROM sys_dummy ;

-- 来源: 1077_file_1077
SELECT last_day ( to_date ( '2017-01-01' , 'YYYY-MM-DD' )) AS cal_result ;

-- 来源: 1077_file_1077
SELECT months_between(to_date('2022-10-31', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

-- 来源: 1077_file_1077
SELECT months_between(to_date('2022-10-30', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

-- 来源: 1077_file_1077
SELECT months_between(to_date('2022-10-29', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

-- 来源: 1077_file_1077
SELECT next_day ( timestamp '2017-05-25 00:00:00' , 'Sunday' ) AS cal_result ;

-- 来源: 1077_file_1077
SELECT next_day ( timestamp '2024-01-17 00:00:00' , 7 . 9999999 );

-- 来源: 1077_file_1077
SELECT next_day ( timestamp '2024-01-17 00:00:00' , 7 . 9999999 );

-- 来源: 1077_file_1077
SELECT tintervalend ( '["Sep 4, 1983 23:59:12" "Oct4, 1983 23:59:12"]' );

-- 来源: 1077_file_1077
SELECT tintervalrel ( '["Sep 4, 1983 23:59:12" "Oct4, 1983 23:59:12"]' );

-- 来源: 1077_file_1077
SELECT convert_tz ( cast ( '2023-01-01 10:10:10' as datetime ), '+00:00' , '+01:00' );

-- 来源: 1077_file_1077
SELECT convert_tz ( cast ( '2023-01-01' as date ), '+00:00' , '+01:00' );

-- 来源: 1077_file_1077
SELECT convert_tz ( '2023-01-01 10:10:10' , '+00:00' , '+01:00' );

-- 来源: 1077_file_1077
SELECT convert_tz ( '2023-01-01' , '+00:00' , '+01:00' );

-- 来源: 1077_file_1077
SELECT convert_tz ( 20230101101010 , '+00:00' , '+01:00' );

-- 来源: 1077_file_1077
SELECT convert_tz ( 20230101 , '+00:00' , '+01:00' );

-- 来源: 1077_file_1077
SELECT convert_tz ( '2023-01-01 10:10:10' , 'UTC' , 'PRC' );

-- 来源: 1077_file_1077
SELECT sec_to_time ( 2000 );

-- 来源: 1077_file_1077
SELECT sec_to_time ( '-2000' );

-- 来源: 1077_file_1077
SELECT ADDDATE ( '2018-05-01' , INTERVAL 1 DAY );

-- 来源: 1077_file_1077
SELECT ADDDATE('2018-05-01', 1);

-- 来源: 1077_file_1077
SELECT curdate ();

-- 来源: 1077_file_1077
SELECT curtime ( 3 );

-- 来源: 1077_file_1077
SELECT DATE_ADD('2018-05-01', INTERVAL 1 DAY);

-- 来源: 1077_file_1077
SELECT DATE_ADD('2018-05-01', 1);

-- 来源: 1077_file_1077
SELECT date_format('2023-10-11 12:13:14.151617','%b %c %M %m');

-- 来源: 1077_file_1077
SELECT DATE_SUB('2018-05-01', INTERVAL 1 YEAR);

-- 来源: 1077_file_1077
SELECT DATE_SUB('2023-1-1', 20);

-- 来源: 1077_file_1077
SELECT datediff('2021-11-12','2021-11-13');

-- 来源: 1077_file_1077
SELECT day('2023-01-02');

-- 来源: 1077_file_1077
SELECT dayofmonth('23-05-22');

-- 来源: 1077_file_1077
SELECT dayname('2023-10-11');

-- 来源: 1077_file_1077
SELECT dayofweek('2023-04-16');

-- 来源: 1077_file_1077
SELECT dayofyear('2000-12-31');

-- 来源: 1077_file_1077
SELECT extract(YEAR FROM '2023-10-11');

-- 来源: 1077_file_1077
SELECT extract(QUARTER FROM '2023-10-11');

-- 来源: 1077_file_1077
SELECT extract(MONTH FROM '2023-10-11');

-- 来源: 1077_file_1077
SELECT extract(WEEK FROM '2023-10-11');

-- 来源: 1077_file_1077
SELECT extract(DAY FROM '2023-10-11');

-- 来源: 1077_file_1077
SELECT extract(HOUR FROM '2023-10-11 12:13:14');

-- 来源: 1077_file_1077
SELECT from_days(36524);

-- 来源: 1077_file_1077
SELECT from_unixtime(1111885200);

-- 来源: 1077_file_1077
SELECT get_format(date, 'eur');

-- 来源: 1077_file_1077
SELECT get_format(date, 'usa');

-- 来源: 1077_file_1077
SELECT HOUR('10:10:10.1');

-- 来源: 1077_file_1077
SELECT makedate(2000, 60);

-- 来源: 1077_file_1077
SELECT MICROSECOND('2023-5-5 10:10:10.24485');

-- 来源: 1077_file_1077
SELECT MINUTE(time'10:10:10');

-- 来源: 1077_file_1077
SELECT month('2021-11-30');

-- 来源: 1077_file_1077
SELECT monthname('2023-02-28');

-- 来源: 1077_file_1077
SELECT period_add(202205, -12);

-- 来源: 1077_file_1077
SELECT period_diff('202101', '202102');

-- 来源: 1077_file_1077
SELECT SECOND('2023-5-5 10:10:10');

-- 来源: 1077_file_1077
SELECT QUARTER('2012-1-1');

-- 来源: 1077_file_1077
SELECT str_to_date('May 1, 2013','%M %d,%Y');

-- 来源: 1077_file_1077
SELECT SUBDATE('2023-1-1', 20);

-- 来源: 1077_file_1077
SELECT SUBDATE('2018-05-01', INTERVAL 1 YEAR);

-- 来源: 1077_file_1077
SELECT subtime('2000-03-01 20:59:59', '22:58');

-- 来源: 1077_file_1077
SELECT addtime('2000-03-01 20:59:59', '00:00:01');

-- 来源: 1077_file_1077
SELECT TIME_FORMAT('25:30:30', '%T|%r|%H|%h|%I|%i|%S|%f|%p|%k');

-- 来源: 1077_file_1077
SELECT time_to_sec('00:00:01');

-- 来源: 1077_file_1077
SELECT timediff(date'2022-12-30',20221229);

-- 来源: 1077_file_1077
SELECT TIMESTAMPADD(DAY,-2,'2022-07-27');

-- 来源: 1077_file_1077
SELECT to_days('2000-1-1');

-- 来源: 1077_file_1077
SELECT TO_SECONDS('2009-11-29 13:43:32');

-- 来源: 1077_file_1077
SELECT UNIX_TIMESTAMP('2022-12-22');

-- 来源: 1077_file_1077
SELECT utc_date();

-- 来源: 1077_file_1077
SELECT utc_time();

-- 来源: 1077_file_1077
SELECT utc_timestamp();

-- 来源: 1077_file_1077
SELECT week(date'2000-01-01', 1);

-- 来源: 1077_file_1077
SELECT week('2000-01-01', 2);

-- 来源: 1077_file_1077
SELECT weekday('1970-01-01 12:00:00');

-- 来源: 1077_file_1077
SELECT weekofyear('1970-05-22');

-- 来源: 1077_file_1077
SELECT year('23-05-22');

-- 来源: 1077_file_1077
SELECT yearweek(datetime'2000-01-01', 3);

-- 来源: 1077_file_1077
SELECT timestamp_diff ( 'year' , '2018-01-01' , '2020-04-01' );

-- 来源: 1077_file_1077
SELECT timestamp_diff ( 'month' , '2018-01-01' , '2020-04-01' );

-- 来源: 1077_file_1077
SELECT timestamp_diff ( 'quarter' , '2018-01-01' , '2020-04-01' );

-- 来源: 1077_file_1077
SELECT timestamp_diff ( 'week' , '2018-01-01' , '2020-04-01' );

-- 来源: 1077_file_1077
SELECT timestamp_diff ( 'day' , '2018-01-01' , '2020-04-01' );

-- 来源: 1077_file_1077
SELECT timestamp_diff ( 'hour' , '2018-01-01 10:10:10' , '2018-01-01 12:12:12' );

-- 来源: 1077_file_1077
SELECT timestamp_diff ( 'minute' , '2018-01-01 10:10:10' , '2018-01-01 12:12:12' );

-- 来源: 1077_file_1077
SELECT timestamp_diff ( 'second' , '2018-01-01 10:10:10' , '2018-01-01 10:12:12' );

-- 来源: 1077_file_1077
SELECT timestamp_diff ( 'microsecond' , '2018-01-01 10:10:10' , '2018-01-01 10:12:12' );

-- 来源: 1077_file_1077
SELECT TIMESTAMPDIFF ( YEAR , '2018-01-01' , '2020-01-01' );

-- 来源: 1077_file_1077
SELECT TIMESTAMPDIFF ( QUARTER , '2018-01-01' , '2020-01-01' );

-- 来源: 1077_file_1077
SELECT TIMESTAMPDIFF ( MONTH , '2018-01-01' , '2020-01-01' );

-- 来源: 1077_file_1077
SELECT TIMESTAMPDIFF ( WEEK , '2018-01-01' , '2020-01-01' );

-- 来源: 1077_file_1077
SELECT TIMESTAMPDIFF ( DAY , '2018-01-01' , '2020-01-01' );

-- 来源: 1077_file_1077
SELECT TIMESTAMPDIFF ( HOUR , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

-- 来源: 1077_file_1077
SELECT TIMESTAMPDIFF ( MINUTE , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

-- 来源: 1077_file_1077
SELECT TIMESTAMPDIFF ( SECOND , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

-- 来源: 1077_file_1077
SELECT TIMESTAMPDIFF ( MICROSECOND , '2020-01-01 10:10:10.000000' , '2020-01-01 10:10:10.111111' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( CENTURY FROM TIMESTAMP '2000-12-16 12:21:13' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( DAY FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( DAY FROM INTERVAL '40 days 1 minute' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( DECADE FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( DOW FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( DOY FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( EPOCH FROM TIMESTAMP WITH TIME ZONE '2001-02-16 20:38:40.12-08' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( EPOCH FROM INTERVAL '5 days 3 hours' );

-- 来源: 1077_file_1077
SELECT TIMESTAMP WITH TIME ZONE 'epoch' + 982384720 . 12 * INTERVAL '1 second' AS RESULT ;

-- 来源: 1077_file_1077
SELECT EXTRACT ( HOUR FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( ISODOW FROM TIMESTAMP '2001-02-18 20:38:40' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( ISOYEAR FROM DATE '2006-01-01' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( WEEK FROM TIMESTAMP '2006-01-01 00:00:40' );

-- 来源: 1077_file_1077
SELECT EXTRACT(ISOYEAR FROM DATE '2006-01-02');

-- 来源: 1077_file_1077
SELECT EXTRACT(WEEK FROM TIMESTAMP '2006-01-02 00:00:40');

-- 来源: 1077_file_1077
SELECT EXTRACT ( MICROSECONDS FROM TIME '17:12:28.5' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( MILLENNIUM FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( MILLISECONDS FROM TIME '17:12:28.5' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( MINUTE FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( MONTH FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( MONTH FROM INTERVAL '2 years 13 months' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( QUARTER FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( SECOND FROM TIME '17:12:28.5' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( ISOYEAR FROM DATE '2006-01-01' );

-- 来源: 1077_file_1077
SELECT EXTRACT ( WEEK FROM TIMESTAMP '2006-01-01 00:00:40' );

-- 来源: 1077_file_1077
SELECT EXTRACT(ISOYEAR FROM DATE '2006-01-02');

-- 来源: 1077_file_1077
SELECT EXTRACT(WEEK FROM TIMESTAMP '2006-01-02 00:00:40');

-- 来源: 1077_file_1077
SELECT EXTRACT ( YEAR FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT date_part ( 'day' , TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 1077_file_1077
SELECT date_part ( 'hour' , INTERVAL '4 hours 3 minutes' );

-- 来源: 1078_file_1078
SELECT cash_words ( '1.23' );

-- 来源: 1078_file_1078
SELECT cast ( '22-oct-1997' as timestamp );

-- 来源: 1078_file_1078
SELECT cast ( '22-ocX-1997' as timestamp DEFAULT '22-oct-1997' ON CONVERSION ERROR , 'DD-Mon-YYYY' );

-- 来源: 1078_file_1078
SELECT CAST ( 12 AS UNSIGNED );

-- 来源: 1078_file_1078
SELECT hextoraw ( '7D' );

-- 来源: 1078_file_1078
SELECT numtoday ( 2 );

-- 来源: 1078_file_1078
SELECT rawtohex ( '1234567' );

-- 来源: 1078_file_1078
SELECT to_blob ( '0AADD343CDBBD' :: RAW ( 10 ));

-- 来源: 1078_file_1078
SELECT to_bigint ( '123364545554455' );

-- 来源: 1078_file_1078
SELECT to_binary_double ( '12345678' );

-- 来源: 1078_file_1078
SELECT to_binary_double ( '1,2,3' , '9,9,9' );

-- 来源: 1078_file_1078
SELECT to_binary_double ( 1 e2 default 12 on conversion error );

-- 来源: 1078_file_1078
SELECT to_binary_double ( 'aa' default 12 on conversion error );

-- 来源: 1078_file_1078
SELECT to_binary_double ( '12-' default 10 on conversion error , '99S' );

-- 来源: 1078_file_1078
SELECT to_binary_double ( 'aa-' default 12 on conversion error , '99S' );

-- 来源: 1078_file_1078
SELECT to_binary_float ( '12345678' );

-- 来源: 1078_file_1078
SELECT to_binary_float ( '1,2,3' , '9,9,9' );

-- 来源: 1078_file_1078
SELECT to_binary_float ( 1 e2 default 12 on conversion error );

-- 来源: 1078_file_1078
SELECT to_binary_float ( 'aa' default 12 on conversion error );

-- 来源: 1078_file_1078
SELECT to_binary_float ( '12-' default 10 on conversion error , '99S' );

-- 来源: 1078_file_1078
SELECT to_binary_float ( 'aa-' default 12 on conversion error , '99S' );

-- 来源: 1078_file_1078
SELECT to_char ( current_timestamp , 'HH12:MI:SS' );

-- 来源: 1078_file_1078
SELECT to_char ( current_timestamp , 'FMHH12:FMMI:FMSS' );

-- 来源: 1078_file_1078
SELECT to_char ( 125 . 8 :: real , '999D99' );

-- 来源: 1078_file_1078
SELECT to_char ( 1485 , '9,999' );

-- 来源: 1078_file_1078
SELECT to_char ( 1148 . 5 , '9,999.999' );

-- 来源: 1078_file_1078
SELECT to_char ( 148 . 5 , '990999.909' );

-- 来源: 1078_file_1078
SELECT to_char ( 123 , 'XXX' );

-- 来源: 1078_file_1078
SELECT to_char ( interval '15h 2m 12s' , 'HH24:MI:SS' );

-- 来源: 1078_file_1078
SELECT to_char ( 125 , '999' );

-- 来源: 1078_file_1078
SELECT to_char ( - 125 . 8 , '999D99S' );

-- 来源: 1078_file_1078
SELECT to_char ( '01110' );

-- 来源: 1078_file_1078
SELECT to_char ( current_timestamp , 'HH12:MI:SS' );

-- 来源: 1078_file_1078
SELECT to_nchar ( current_timestamp , 'HH12:MI:SS' );

-- 来源: 1078_file_1078
SELECT to_nchar ( current_timestamp , 'FMHH12:FMMI:FMSS' );

-- 来源: 1078_file_1078
SELECT to_nchar ( 125 . 8 :: real , '999D99' );

-- 来源: 1078_file_1078
SELECT to_nchar ( 1485 , '9,999' );

-- 来源: 1078_file_1078
SELECT to_nchar ( 1148 . 5 , '9,999.999' );

-- 来源: 1078_file_1078
SELECT to_nchar ( 148 . 5 , '990999.909' );

-- 来源: 1078_file_1078
SELECT to_nchar ( 123 , 'XXX' );

-- 来源: 1078_file_1078
SELECT to_nchar ( interval '15h 2m 12s' , 'HH24:MI:SS' );

-- 来源: 1078_file_1078
SELECT to_nchar ( 125 , '999' );

-- 来源: 1078_file_1078
SELECT to_nchar ( - 125 . 8 , '999D99S' );

-- 来源: 1078_file_1078
SELECT to_nchar ( '01110' );

-- 来源: 1078_file_1078
SELECT to_nchar ( current_timestamp , 'HH12:MI:SS' );

-- 来源: 1078_file_1078
SELECT to_clob ( 'ABCDEF' :: RAW ( 10 ));

-- 来源: 1078_file_1078
SELECT to_clob ( 'hello111' :: CHAR ( 15 ));

-- 来源: 1078_file_1078
SELECT to_clob ( 'gauss123' :: NCHAR ( 10 ));

-- 来源: 1078_file_1078
SELECT to_clob ( 'gauss234' :: VARCHAR ( 10 ));

-- 来源: 1078_file_1078
SELECT to_clob ( 'gauss345' :: VARCHAR2 ( 10 ));

-- 来源: 1078_file_1078
SELECT to_clob ( 'gauss456' :: NVARCHAR2 ( 10 ));

-- 来源: 1078_file_1078
SELECT to_clob ( 'World222!' :: TEXT );

-- 来源: 1078_file_1078
SELECT to_date ( '2015-08-14' );

-- 来源: 1078_file_1078
SELECT to_date ( '05 Dec 2000' , 'DD Mon YYYY' );

-- 来源: 1078_file_1078
SELECT to_date ( '2015-08-14' );

-- 来源: 1078_file_1078
SELECT to_date ( '05 Dec 2000' , 'DD Mon YYYY' );

-- 来源: 1078_file_1078
select to_date('12-jan-2022' default '12-apr-2022' on conversion error);

-- 来源: 1078_file_1078
select to_date('12-ja-2022' default '12-apr-2022' on conversion error);

-- 来源: 1078_file_1078
select to_date('2022-12-12' default '2022-01-01' on conversion error, 'yyyy-mm-dd');

-- 来源: 1078_file_1078
SELECT to_number ( '12,454.8-' , '99G999D9S' );

-- 来源: 1078_file_1078
SELECT to_number ( '12,454.8-' , '99G999D9S' );

-- 来源: 1078_file_1078
select to_number ( '1e2' );

-- 来源: 1078_file_1078
select to_number ( '123.456' );

-- 来源: 1078_file_1078
select to_number ( '123' , '999' );

-- 来源: 1078_file_1078
select to_number ( '123-' , '999MI' );

-- 来源: 1078_file_1078
select to_number ( '123' default '456-' on conversion error , '999MI' );

-- 来源: 1078_file_1078
SELECT to_timestamp ( 1284352323 );

-- 来源: 1078_file_1078
SELECT to_timestamp ( '12-sep-2014' );

-- 来源: 1078_file_1078
SELECT to_timestamp ( '12-Sep-10 14:10:10.123000' , 'DD-Mon-YY HH24:MI:SS.FF' );

-- 来源: 1078_file_1078
SELECT to_timestamp ( '-1' , 'SYYYY' );

-- 来源: 1078_file_1078
SELECT to_timestamp ( '98' , 'RR' );

-- 来源: 1078_file_1078
SELECT to_timestamp ( '01' , 'RR' );

-- 来源: 1078_file_1078
SELECT to_timestamp('11-Sep-11' DEFAULT '12-Sep-10 14:10:10.123000' ON CONVERSION ERROR,'DD-Mon-YY HH24:MI:SS.FF');

-- 来源: 1078_file_1078
SELECT to_timestamp('12-Sep-10 14:10:10.123000','DD-Mon-YY HH24:MI:SSXFF');

-- 来源: 1078_file_1078
SELECT to_timestamp ( '05 Dec 2000' , 'DD Mon YYYY' );

-- 来源: 1078_file_1078
SELECT to_timestamp_tz ( '05 DeX 2000' DEFAULT '05 Dec 2001' ON CONVERSION ERROR , 'DD Mon YYYY' );

-- 来源: 1078_file_1078
SELECT to_timestamp_tz ( '05 DeX 2000' DEFAULT '05 Dec 2001' ON CONVERSION ERROR , 'DD Mon YYYY' , 'nls_date_language=AMERICAN' );

-- 来源: 1078_file_1078
select to_dsinterval ( '12 1:2:3.456' );

-- 来源: 1078_file_1078
select to_dsinterval ( 'P3DT4H5M6S' );

-- 来源: 1078_file_1078
select to_yminterval ( '1-1' );

-- 来源: 1078_file_1078
select to_yminterval ( 'P13Y3M4DT4H2M5S' );

-- 来源: 1078_file_1078
select treat ( data as json ) from json_doc ;

-- 来源: 1078_file_1078
select cast ( t1 ( 1 , 2 , 3 ) as int []) result ;

-- 来源: 1078_file_1078
SELECT convert_to_nocase ( '12345' , 'GBK' );

-- 来源: 1079_file_1079
SELECT box '((0,0),(1,1))' + point '(2.0,0)' AS RESULT ;

-- 来源: 1079_file_1079
SELECT box '((0,0),(1,1))' - point '(2.0,0)' AS RESULT ;

-- 来源: 1079_file_1079
SELECT box '((0,0),(1,1))' * point '(2.0,0)' AS RESULT ;

-- 来源: 1079_file_1079
SELECT box '((0,0),(2,2))' / point '(2.0,0)' AS RESULT ;

-- 来源: 1079_file_1079
SELECT box '((1,-1),(-1,1))' # box '((1,1),(-2,-2))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT # path '((1,0),(0,1),(-1,0))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT @-@ path '((0,0),(1,0))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT @@ circle '((0,0),10)' AS RESULT ;

-- 来源: 1079_file_1079
SELECT circle '((0,0),1)' <-> circle '((5,0),1)' AS RESULT ;

-- 来源: 1079_file_1079
SELECT box '((0,0),(1,1))' && box '((0,0),(2,2))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT circle '((0,0),1)' << circle '((5,0),1)' AS RESULT ;

-- 来源: 1079_file_1079
SELECT circle '((5,0),1)' >> circle '((0,0),1)' AS RESULT ;

-- 来源: 1079_file_1079
SELECT box '((0,0),(1,1))' &< box '((0,0),(2,2))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT box '((0,0),(3,3))' &> box '((0,0),(2,2))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT box '((0,0),(3,3))' <<| box '((3,4),(5,5))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT box '((3,4),(5,5))' |>> box '((0,0),(3,3))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT box '((0,0),(1,1))' &<| box '((0,0),(2,2))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT box '((0,0),(3,3))' |&> box '((0,0),(2,2))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT box '((0,0),(-3,-3))' <^ box '((0,0),(2,2))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT box '((0,0),(2,2))' >^ box '((0,0),(-3,-3))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT lseg '((-1,0),(1,0))' ?# box '((-2,-2),(2,2))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT ?- lseg '((-1,0),(1,0))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT point '(1,0)' ?- point '(0,0)' AS RESULT ;

-- 来源: 1079_file_1079
SELECT ?| lseg '((-1,0),(1,0))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT point '(0,1)' ?| point '(0,0)' AS RESULT ;

-- 来源: 1079_file_1079
SELECT lseg '((0,0),(0,1))' ?-| lseg '((0,0),(1,0))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT lseg '((-1,0),(1,0))' ?|| lseg '((-1,2),(1,2))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT circle '((0,0),2)' @> point '(1,1)' AS RESULT ;

-- 来源: 1079_file_1079
SELECT point '(1,1)' <@ circle '((0,0),2)' AS RESULT ;

-- 来源: 1079_file_1079
SELECT polygon '((0,0),(1,1))' ~= polygon '((1,1),(0,0))' AS RESULT ;

-- 来源: 1079_file_1079
SELECT area ( box '((0,0),(1,1))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT center ( box '((0,0),(1,2))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT diameter ( circle '((0,0),2.0)' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT height ( box '((0,0),(1,1))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT isclosed ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT isopen ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT length ( path '((-1,0),(1,0))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT npoints ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT npoints ( polygon '((1,1),(0,0))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT pclose ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT popen ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT radius ( circle '((0,0),2.0)' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT width ( box '((0,0),(1,1))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT box ( circle '((0,0),2.0)' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT box ( point '(0,0)' , point '(1,1)' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT box ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT circle ( box '((0,0),(1,1))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT circle ( point '(0,0)' , 2 . 0 ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT circle ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT lseg ( box '((-1,0),(1,0))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT lseg ( point '(-1,0)' , point '(1,0)' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT slope(point '(1,1)', point '(0,0)') AS RESULT;

-- 来源: 1079_file_1079
SELECT path ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT point ( 23 . 4 , - 44 . 5 ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT point ( box '((-1,0),(1,0))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT point ( circle '((0,0),2.0)' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT point ( lseg '((-1,0),(1,0))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT point ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT polygon ( box '((0,0),(1,1))' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT polygon ( circle '((0,0),2.0)' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT polygon ( 12 , circle '((0,0),2.0)' ) AS RESULT ;

-- 来源: 1079_file_1079
SELECT polygon ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1.5' < inet '192.168.1.6' AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1.5' <= inet '192.168.1.5' AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1.5' = inet '192.168.1.5' AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1.5' >= inet '192.168.1.5' AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1.5' > inet '192.168.1.4' AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1.5' <> inet '192.168.1.4' AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1.5' << inet '192.168.1/24' AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1/24' <<= inet '192.168.1/24' AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1/24' >> inet '192.168.1.5' AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1/24' >>= inet '192.168.1/24' AS RESULT ;

-- 来源: 1080_file_1080
SELECT ~ inet '192.168.1.6' AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1.6' & inet '10.0.0.0' AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1.6' | inet '10.0.0.0' AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1.6' + 25 AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1.43' - 36 AS RESULT ;

-- 来源: 1080_file_1080
SELECT inet '192.168.1.43' - inet '192.168.1.19' AS RESULT ;

-- 来源: 1080_file_1080
SELECT abbrev ( inet '10.1.0.0/16' ) AS RESULT ;

-- 来源: 1080_file_1080
SELECT abbrev ( cidr '10.1.0.0/16' ) AS RESULT ;

-- 来源: 1080_file_1080
SELECT broadcast ( '192.168.1.5/24' ) AS RESULT ;

-- 来源: 1080_file_1080
SELECT family ( '127.0.0.1' ) AS RESULT ;

-- 来源: 1080_file_1080
SELECT host ( '192.168.1.5/24' ) AS RESULT ;

-- 来源: 1080_file_1080
SELECT hostmask ( '192.168.23.20/30' ) AS RESULT ;

-- 来源: 1080_file_1080
SELECT masklen ( '192.168.1.5/24' ) AS RESULT ;

-- 来源: 1080_file_1080
SELECT netmask ( '192.168.1.5/24' ) AS RESULT ;

-- 来源: 1080_file_1080
SELECT network ( '192.168.1.5/24' ) AS RESULT ;

-- 来源: 1080_file_1080
SELECT set_masklen ( '192.168.1.5/24' , 16 ) AS RESULT ;

-- 来源: 1080_file_1080
SELECT set_masklen ( '192.168.1.0/24' :: cidr , 16 ) AS RESULT ;

-- 来源: 1080_file_1080
SELECT text ( inet '192.168.1.5' ) AS RESULT ;

-- 来源: 1080_file_1080
SELECT trunc ( macaddr '12:34:56:78:90:ab' ) AS RESULT ;

-- 来源: 1081_file_1081
SELECT to_tsvector ( 'fat cats ate rats' ) @@ to_tsquery ( 'cat & rat' ) AS RESULT ;

-- 来源: 1081_file_1081
SELECT to_tsvector ( 'fat cats ate rats' ) @@@ to_tsquery ( 'cat & rat' ) AS RESULT ;

-- 来源: 1081_file_1081
SELECT 'a:1 b:2' :: tsvector || 'c:1 d:2 b:3' :: tsvector AS RESULT ;

-- 来源: 1081_file_1081
SELECT 'fat | rat' :: tsquery && 'cat' :: tsquery AS RESULT ;

-- 来源: 1081_file_1081
SELECT 'fat | rat' :: tsquery || 'cat' :: tsquery AS RESULT ;

-- 来源: 1081_file_1081
SELECT !! 'cat' :: tsquery AS RESULT ;

-- 来源: 1081_file_1081
SELECT 'cat' :: tsquery @> 'cat & rat' :: tsquery AS RESULT ;

-- 来源: 1081_file_1081
SELECT 'cat' :: tsquery <@ 'cat & rat' :: tsquery AS RESULT ;

-- 来源: 1081_file_1081
SELECT get_current_ts_config ();

-- 来源: 1081_file_1081
SELECT length ( 'fat:2,4 cat:3 rat:5A' :: tsvector );

-- 来源: 1081_file_1081
SELECT numnode ( '(fat & rat) | cat' :: tsquery );

-- 来源: 1081_file_1081
SELECT plainto_tsquery ( 'english' , 'The Fat Rats' );

-- 来源: 1081_file_1081
SELECT querytree ( 'foo & ! bar' :: tsquery );

-- 来源: 1081_file_1081
SELECT setweight ( 'fat:2,4 cat:3 rat:5B' :: tsvector , 'A' );

-- 来源: 1081_file_1081
SELECT strip ( 'fat:2,4 cat:3 rat:5A' :: tsvector );

-- 来源: 1081_file_1081
SELECT to_tsquery ( 'english' , 'The & Fat & Rats' );

-- 来源: 1081_file_1081
SELECT to_tsvector ( 'english' , 'The Fat Rats' );

-- 来源: 1081_file_1081
SELECT to_tsvector_for_batch ( 'english' , 'The Fat Rats' );

-- 来源: 1081_file_1081
SELECT ts_headline ( 'x y z' , 'z' :: tsquery );

-- 来源: 1081_file_1081
SELECT ts_rank ( 'hello world' :: tsvector , 'world' :: tsquery );

-- 来源: 1081_file_1081
SELECT ts_rank_cd ( 'hello world' :: tsvector , 'world' :: tsquery );

-- 来源: 1081_file_1081
SELECT ts_rewrite ( 'a & b' :: tsquery , 'a' :: tsquery , 'foo|bar' :: tsquery );

-- 来源: 1081_file_1081
SELECT ts_rewrite ( 'world' :: tsquery , 'select ''world''::tsquery, ''hello''::tsquery' );

-- 来源: 1081_file_1081
SELECT ts_debug ( 'english' , 'The Brightest supernovaes' );

-- 来源: 1081_file_1081
SELECT ts_lexize ( 'english_stem' , 'stars' );

-- 来源: 1081_file_1081
SELECT ts_parse ( 'default' , 'foo - bar' );

-- 来源: 1081_file_1081
SELECT ts_parse ( 3722 , 'foo - bar' );

-- 来源: 1081_file_1081
SELECT ts_token_type ( 'default' );

-- 来源: 1081_file_1081
SELECT ts_token_type ( 3722 );

-- 来源: 1081_file_1081
SELECT ts_stat ( 'select ''hello world''::tsvector' );

-- 来源: 1082_JSON_JSONB
SELECT array_to_json('{{1,5},{99,100}}'::int[]);

-- 来源: 1082_JSON_JSONB
SELECT row_to_json(row(1,'foo'));

-- 来源: 1082_JSON_JSONB
SELECT json_array_element('[1,true,[1,[2,3]],null]',2);

-- 来源: 1082_JSON_JSONB
SELECT json_array_element_text('[1,true,[1,[2,3]],null]',2);

-- 来源: 1082_JSON_JSONB
SELECT json_object_field('{"a": {"b":"foo"}}','a');

-- 来源: 1082_JSON_JSONB
SELECT json_object_field_text('{"a": {"b":"foo"}}','a');

-- 来源: 1082_JSON_JSONB
SELECT json_extract_path('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', 'f4','f6');

-- 来源: 1082_JSON_JSONB
SELECT json_extract_path_op('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', ARRAY['f4','f6']);

-- 来源: 1082_JSON_JSONB
SELECT json_extract_path_text('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', 'f4','f6');

-- 来源: 1082_JSON_JSONB
SELECT json_extract_path_text_op('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', ARRAY['f4','f6']);

-- 来源: 1082_JSON_JSONB
SELECT json_array_elements('[1,true,[1,[2,3]],null]');

-- 来源: 1082_JSON_JSONB
SELECT * FROM json_array_elements_text('[1,true,[1,[2,3]],null]');

-- 来源: 1082_JSON_JSONB
SELECT json_array_length('[1,2,3,{"f1":1,"f2":[5,6]},4,null]');

-- 来源: 1082_JSON_JSONB
SELECT * FROM json_each('{"f1":[1,2,3],"f2":{"f3":1},"f4":null}');

-- 来源: 1082_JSON_JSONB
SELECT * FROM json_each_text('{"f1":[1,2,3],"f2":{"f3":1},"f4":null}');

-- 来源: 1082_JSON_JSONB
SELECT json_object_keys('{"f1":"abc","f2":{"f3":"a", "f4":"b"}, "f1":"abcd"}');

-- 来源: 1082_JSON_JSONB
SELECT jsonb_object_keys('{"f1":"abc","f2":{"f3":"a", "f4":"b"}, "f1":"abcd"}');

-- 来源: 1082_JSON_JSONB
SELECT * FROM json_populate_record(null::jpop,'{"a":"blurfl","x":43.2}');

-- 来源: 1082_JSON_JSONB
SELECT * FROM json_populate_record((1,1,null)::jpop,'{"a":"blurfl","x":43.2}');

-- 来源: 1082_JSON_JSONB
SELECT * FROM json_populate_recordset(null::jpop, '[{"a":1,"b":2},{"a":3,"b":4}]');

-- 来源: 1082_JSON_JSONB
SELECT value, json_typeof(value) FROM (values (json '123.4'), (json '"foo"'), (json 'true'), (json 'null'), (json '[1, 2, 3]'), (json '{"x":"foo", "y":123}'), (NULL::json)) AS data(value);

-- 来源: 1082_JSON_JSONB
SELECT json_build_array('a',1,'b',1.2,'c',true,'d',null,'e',json '{"x": 3, "y": [1,2,3]}','');

-- 来源: 1082_JSON_JSONB
SELECT json_build_object(1,2);

-- 来源: 1082_JSON_JSONB
SELECT jsonb_build_object('a',1,'b',1.2,'c',true,'d',null,'e',json '{"x": 3, "y": [1,2,3]}');

-- 来源: 1082_JSON_JSONB
SELECT jsonb_build_object();

-- 来源: 1082_JSON_JSONB
SELECT * FROM json_to_record('{"a":1,"b":"foo","c":"bar"}',true) AS x(a int, b text, d text);

-- 来源: 1082_JSON_JSONB
SELECT * FROM json_to_record('{"a": {"x": 1, "y": 2},"b":"foo","c":[1, 2]}') AS x(a json, b text, c int[]);

-- 来源: 1082_JSON_JSONB
SELECT * FROM json_to_recordset('[{"a":1,"b":"foo","d":false},{"a":2,"b":"bar","c":true}]',false) AS x(a int, b text, c boolean);

-- 来源: 1082_JSON_JSONB
SELECT json_object('{a,1,b,2,3,NULL,"d e f","a b c"}');

-- 来源: 1082_JSON_JSONB
SELECT json_object('{a,b,"a b c"}', '{a,1,1}');

-- 来源: 1082_JSON_JSONB
SELECT json_object('d',2,'c','name','b',true,'a',2,'a',NULL,'d',1);

-- 来源: 1082_JSON_JSONB
SELECT json_object('d',2,true,'name','b',true,'a',2,'aa', current_timestamp);

-- 来源: 1082_JSON_JSONB
SELECT json_array_append('[1, [2, 3]]', '$[1]', 4, '$[0]', false, '$[0]', null, '$[0]', current_timestamp);

-- 来源: 1082_JSON_JSONB
SELECT json_array();

-- 来源: 1082_JSON_JSONB
SELECT json_array(TRUE, FALSE, NULL, 114, 'text', current_timestamp);

-- 来源: 1082_JSON_JSONB
SELECT json_array_insert('[1, [2, 3]]', '$[1]', 4);

-- 来源: 1082_JSON_JSONB
SELECT json_array_insert('{"x": 1, "y": [1, 2]}', '$.y[0]', NULL, '$.y[0]', 123, '$.y[3]', current_timestamp);

-- 来源: 1082_JSON_JSONB
SELECT json_contains('[1, 2, {"x": 3}]', '{"x":3}');

-- 来源: 1082_JSON_JSONB
SELECT json_contains('[1, 2, {"x": 3},[1,2,3,4]]', '2','$[1]');

-- 来源: 1082_JSON_JSONB
SELECT json_contains('[1, 2, {"x": 3},[1,2,3,4]]', '1','$[1]');

-- 来源: 1082_JSON_JSONB
SELECT json_contains_path('[1, 2, {"x": 3}]', 'all', '$[0]', '$[1]', '$[2]');

-- 来源: 1082_JSON_JSONB
SELECT json_contains_path('[1, 2, {"x": 3}]', 'all', '$[0]', '$[1]', '$[6]');

-- 来源: 1082_JSON_JSONB
SELECT json_contains_path('[1, 2, {"x": 3}]', 'one', '$[0]', '$[1]', '$[5]');

-- 来源: 1082_JSON_JSONB
SELECT json_depth('[]');

-- 来源: 1082_JSON_JSONB
SELECT json_depth('{"s":1, "x":2,"y":[1]}');

-- 来源: 1082_JSON_JSONB
SELECT json_extract('[1, 2, {"x": 3}]', '$[2]');

-- 来源: 1082_JSON_JSONB
SELECT json_extract('["a", ["b", "c"], "d"]', '$[1]', '$[2]', '$[3]');

-- 来源: 1082_JSON_JSONB
SELECT json_insert('[1, [2, 3], {"a": [4, 5]}]', '$[3]', 2);

-- 来源: 1082_JSON_JSONB
SELECT json_insert('[1, [2, 3], {"a": [4, 5]}]', '$[10]', 10,'$[5]', 5);

-- 来源: 1082_JSON_JSONB
SELECT json_keys('{"x": 1, "y": 2, "z": 3}');

-- 来源: 1082_JSON_JSONB
SELECT json_keys('[1,2,3,{"name":"Tom"}]','$[3]');

-- 来源: 1082_JSON_JSONB
SELECT json_length('[1,2,3,4,5]');

-- 来源: 1082_JSON_JSONB
SELECT json_length('{"name":"Tom", "age":24, "like":"football"}');

-- 来源: 1082_JSON_JSONB
SELECT json_merge('[1, 2]','[2]');

-- 来源: 1082_JSON_JSONB
SELECT json_merge('{"b":"2"}','{"a":"1"}','[1,2]');

-- 来源: 1082_JSON_JSONB
SELECT json_quote('gauss');

-- 来源: 1082_JSON_JSONB
SELECT json_unquote('"gauss"');

-- 来源: 1082_JSON_JSONB
SELECT json_remove('[0, 1, 2, [3, 4]]', '$[0]', '$[2]');

-- 来源: 1082_JSON_JSONB
SELECT json_remove('[0, 1, 2, [3, 4]]', '$[0]', '$[0]','$[0]');

-- 来源: 1082_JSON_JSONB
SELECT json_replace('{"x": 1}', '$.x', 'true');

-- 来源: 1082_JSON_JSONB
SELECT json_replace('{"x": 1}', '$.x', true, '$.x', 123, '$.x', 'asd', '$.x', null);

-- 来源: 1082_JSON_JSONB
SELECT json_search('{"a":"abc","b":"abc"}','all','abc');

-- 来源: 1082_JSON_JSONB
SELECT json_search('{"a":"abc","b":"abc"}','one','abc');

-- 来源: 1082_JSON_JSONB
SELECT json_search('{"a":"abc","b":"a%c"}','one','a\%c');

-- 来源: 1082_JSON_JSONB
SELECT json_set('{"s":3}','$.s','d');

-- 来源: 1082_JSON_JSONB
SELECT json_set('{"s":3}','$.a','d','$.a','1');

-- 来源: 1082_JSON_JSONB
SELECT json_type('{"w":{"2":3},"2":4}');

-- 来源: 1082_JSON_JSONB
SELECT json_type('[1,2,2,3,3,4,4,4,4,4,4,4,4]');

-- 来源: 1082_JSON_JSONB
SELECT json_valid('{"name":"Tom"}');

-- 来源: 1082_JSON_JSONB
SELECT json_valid('[1,23,4,5,5]');

-- 来源: 1082_JSON_JSONB
SELECT json_valid('[1,23,4,5,5]}');

-- 来源: 1082_JSON_JSONB
SELECT * FROm classes;

-- 来源: 1082_JSON_JSONB
SELECT name, json_agg(score) score FROM classes GROUP BY name ORDER BY name;

-- 来源: 1082_JSON_JSONB
SELECT * FROM classes;

-- 来源: 1082_JSON_JSONB
SELECT json_object_agg(name, score) FROM classes GROUP BY name ORDER BY name;

-- 来源: 1082_JSON_JSONB
SELECT jsonb_contained('[1,2,3]', '[1,2,3,4]');

-- 来源: 1082_JSON_JSONB
SELECT jsonb_contains('[1,2,3,4]', '[1,2,3]');

-- 来源: 1082_JSON_JSONB
SELECT jsonb_exists('["1",2,3]', '1');

-- 来源: 1082_JSON_JSONB
SELECT jsonb_exists_all('["1","2",3]', '{1, 2}');

-- 来源: 1082_JSON_JSONB
SELECT jsonb_exists_any('["1","2",3]', '{1, 2, 4}');

-- 来源: 1082_JSON_JSONB
SELECT jsonb_cmp('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 1082_JSON_JSONB
SELECT jsonb_eq('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 1082_JSON_JSONB
SELECT jsonb_ne('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 1082_JSON_JSONB
SELECT jsonb_gt('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 1082_JSON_JSONB
SELECT jsonb_ge('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 1082_JSON_JSONB
SELECT jsonb_lt('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 1082_JSON_JSONB
SELECT jsonb_le('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 1082_JSON_JSONB
SELECT to_json('{1,5}'::text[]);

-- 来源: 1082_JSON_JSONB
SELECT to_jsonb(array[1, 2, 3, 4]);

-- 来源: 1082_JSON_JSONB
SELECT jsonb_hash('[1,2,3]');

-- 来源: 1083_HLL
SELECT hll_hash_boolean ( FALSE );

-- 来源: 1083_HLL
SELECT hll_hash_boolean ( FALSE , 10 );

-- 来源: 1083_HLL
SELECT hll_hash_smallint ( 100 :: smallint );

-- 来源: 1083_HLL
SELECT hll_hash_smallint ( 100 :: smallint , 10 );

-- 来源: 1083_HLL
SELECT hll_hash_integer ( 0 );

-- 来源: 1083_HLL
SELECT hll_hash_integer ( 0 , 10 );

-- 来源: 1083_HLL
SELECT hll_hash_bigint ( 100 :: bigint );

-- 来源: 1083_HLL
SELECT hll_hash_bigint ( 100 :: bigint , 10 );

-- 来源: 1083_HLL
SELECT hll_hash_bytea ( E '\\x' );

-- 来源: 1083_HLL
SELECT hll_hash_bytea ( E '\\x' , 10 );

-- 来源: 1083_HLL
SELECT hll_hash_text ( 'AB' );

-- 来源: 1083_HLL
SELECT hll_hash_text ( 'AB' , 10 );

-- 来源: 1083_HLL
SELECT hll_hash_any ( 1 );

-- 来源: 1083_HLL
SELECT hll_hash_any ( '08:00:2b:01:02:03' :: macaddr );

-- 来源: 1083_HLL
SELECT hll_hash_any ( 1 , 10 );

-- 来源: 1083_HLL
SELECT hll_hashval_eq ( hll_hash_integer ( 1 ), hll_hash_integer ( 1 ));

-- 来源: 1083_HLL
SELECT hll_hashval_ne ( hll_hash_integer ( 1 ), hll_hash_integer ( 1 ));

-- 来源: 1083_HLL
SELECT hll_print ( hll_empty ());

-- 来源: 1083_HLL
SELECT hll_type ( hll_empty ());

-- 来源: 1083_HLL
SELECT hll_log2m ( hll_empty ());

-- 来源: 1083_HLL
SELECT hll_log2m ( hll_empty ( 10 ));

-- 来源: 1083_HLL
SELECT hll_log2m ( hll_empty ( - 1 ));

-- 来源: 1083_HLL
SELECT hll_log2explicit ( hll_empty ());

-- 来源: 1083_HLL
SELECT hll_log2explicit ( hll_empty ( 12 , 8 ));

-- 来源: 1083_HLL
SELECT hll_log2explicit ( hll_empty ( 12 , - 1 ));

-- 来源: 1083_HLL
SELECT hll_log2sparse ( hll_empty ());

-- 来源: 1083_HLL
SELECT hll_log2sparse ( hll_empty ( 12 , 8 , 10 ));

-- 来源: 1083_HLL
SELECT hll_log2sparse ( hll_empty ( 12 , 8 , - 1 ));

-- 来源: 1083_HLL
SELECT hll_duplicatecheck ( hll_empty ());

-- 来源: 1083_HLL
SELECT hll_duplicatecheck ( hll_empty ( 12 , 8 , 10 , 1 ));

-- 来源: 1083_HLL
SELECT hll_duplicatecheck ( hll_empty ( 12 , 8 , 10 , - 1 ));

-- 来源: 1083_HLL
SELECT hll_empty ();

-- 来源: 1083_HLL
SELECT hll_empty ( 10 );

-- 来源: 1083_HLL
SELECT hll_empty ( - 1 );

-- 来源: 1083_HLL
SELECT hll_empty ( 10 , 4 );

-- 来源: 1083_HLL
SELECT hll_empty ( 10 , - 1 );

-- 来源: 1083_HLL
SELECT hll_empty ( 10 , 4 , 8 );

-- 来源: 1083_HLL
SELECT hll_empty ( 10 , 4 , - 1 );

-- 来源: 1083_HLL
SELECT hll_empty ( 10 , 4 , 8 , 0 );

-- 来源: 1083_HLL
SELECT hll_empty ( 10 , 4 , 8 , - 1 );

-- 来源: 1083_HLL
SELECT hll_add ( hll_empty (), hll_hash_integer ( 1 ));

-- 来源: 1083_HLL
SELECT hll_add_rev ( hll_hash_integer ( 1 ), hll_empty ());

-- 来源: 1083_HLL
SELECT hll_eq ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

-- 来源: 1083_HLL
SELECT hll_ne ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

-- 来源: 1083_HLL
SELECT hll_cardinality ( hll_empty () || hll_hash_integer ( 1 ));

-- 来源: 1083_HLL
SELECT hll_union ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

-- 来源: 1083_HLL
SELECT a , # c AS cardinality FROM t_a_c_hll ORDER BY a ;

-- 来源: 1083_HLL
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), 12 )) FROM t_data ;

-- 来源: 1083_HLL
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 1 )) FROM t_data ;

-- 来源: 1083_HLL
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 6 , 10 )) FROM t_data ;

-- 来源: 1083_HLL
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 6 , 10 , - 1 )) FROM t_data ;

-- 来源: 1083_HLL
SELECT # hll_union_agg ( c ) AS cardinality FROM t_a_c_hll ;

-- 来源: 1083_HLL
SELECT ( hll_empty () || hll_hash_integer ( 1 )) = ( hll_empty () || hll_hash_integer ( 1 ));

-- 来源: 1083_HLL
SELECT hll_hash_integer ( 1 ) = hll_hash_integer ( 1 );

-- 来源: 1083_HLL
SELECT ( hll_empty () || hll_hash_integer ( 1 )) <> ( hll_empty () || hll_hash_integer ( 2 ));

-- 来源: 1083_HLL
SELECT hll_hash_integer ( 1 ) <> hll_hash_integer ( 2 );

-- 来源: 1083_HLL
SELECT hll_empty () || hll_hash_integer ( 1 );

-- 来源: 1083_HLL
SELECT hll_hash_integer ( 1 ) || hll_empty ();

-- 来源: 1083_HLL
SELECT ( hll_empty () || hll_hash_integer ( 1 )) || ( hll_empty () || hll_hash_integer ( 2 ));

-- 来源: 1083_HLL
SELECT # ( hll_empty () || hll_hash_integer ( 1 ));

-- 来源: 1084_SEQUENCE
SELECT nextval ( 'seqDemo' );

-- 来源: 1084_SEQUENCE
SELECT seqDemo . nextval ;

-- 来源: 1084_SEQUENCE
SELECT nextval ( 'seq1' );

-- 来源: 1084_SEQUENCE
SELECT currval ( 'seq1' );

-- 来源: 1084_SEQUENCE
SELECT seq1 . currval seq1 ;

-- 来源: 1084_SEQUENCE
SELECT nextval ( 'seq1' );

-- 来源: 1084_SEQUENCE
SELECT lastval ();

-- 来源: 1084_SEQUENCE
SELECT nextval ( 'seqDemo' );

-- 来源: 1084_SEQUENCE
SELECT setval ( 'seqDemo' , 3 );

-- 来源: 1084_SEQUENCE
SELECT nextval ( 'seqDemo' );

-- 来源: 1084_SEQUENCE
SELECT setval ( 'seqDemo' , 5 , true );

-- 来源: 1085_file_1085
SELECT ARRAY [ 1 . 1 , 2 . 1 , 3 . 1 ]:: int [] = ARRAY [ 1 , 2 , 3 ] AS RESULT ;

-- 来源: 1085_file_1085
SELECT ARRAY [ 1 , 2 , 3 ] <> ARRAY [ 1 , 2 , 4 ] AS RESULT ;

-- 来源: 1085_file_1085
SELECT ARRAY [ 1 , 2 , 3 ] < ARRAY [ 1 , 2 , 4 ] AS RESULT ;

-- 来源: 1085_file_1085
SELECT ARRAY [ 1 , 4 , 3 ] > ARRAY [ 1 , 2 , 4 ] AS RESULT ;

-- 来源: 1085_file_1085
SELECT ARRAY [ 1 , 2 , 3 ] <= ARRAY [ 1 , 2 , 3 ] AS RESULT ;

-- 来源: 1085_file_1085
SELECT ARRAY [ 1 , 4 , 3 ] >= ARRAY [ 1 , 4 , 3 ] AS RESULT ;

-- 来源: 1085_file_1085
SELECT ARRAY [ 1 , 4 , 3 ] @> ARRAY [ 3 , 1 ] AS RESULT ;

-- 来源: 1085_file_1085
SELECT ARRAY [ 2 , 7 ] <@ ARRAY [ 1 , 7 , 4 , 2 , 6 ] AS RESULT ;

-- 来源: 1085_file_1085
SELECT ARRAY [ 1 , 4 , 3 ] && ARRAY [ 2 , 1 ] AS RESULT ;

-- 来源: 1085_file_1085
SELECT ARRAY [ 1 , 2 , 3 ] || ARRAY [ 4 , 5 , 6 ] AS RESULT ;

-- 来源: 1085_file_1085
SELECT ARRAY [ 1 , 2 , 3 ] || ARRAY [[ 4 , 5 , 6 ],[ 7 , 8 , 9 ]] AS RESULT ;

-- 来源: 1085_file_1085
SELECT 3 || ARRAY [ 4 , 5 , 6 ] AS RESULT ;

-- 来源: 1085_file_1085
SELECT ARRAY [ 4 , 5 , 6 ] || 7 AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_append ( ARRAY [ 1 , 2 ], 3 ) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_prepend ( 1 , ARRAY [ 2 , 3 ]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_cat ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 4 , 5 ]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_cat ( ARRAY [[ 1 , 2 ],[ 4 , 5 ]], ARRAY [ 6 , 7 ]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_union ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_union ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_union_distinct ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_union_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_intersect ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_intersect ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_intersect_distinct ( ARRAY [ 1 , 2 , 2 ], ARRAY [ 2 , 2 , 4 , 5 ]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_intersect_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_except ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_except ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_except ( NULL , ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_except_distinct ( ARRAY [ 1 , 2 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_except_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_except_distinct ( NULL , ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_ndims ( ARRAY [[ 1 , 2 , 3 ], [ 4 , 5 , 6 ]]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_dims ( ARRAY [[ 1 , 2 , 3 ], [ 4 , 5 , 6 ]]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_length ( array [ 1 , 2 , 3 ], 1 ) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_lower ( '[0:2]={1,2,3}' :: int [], 1 ) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_sort ( ARRAY [ 5 , 1 , 3 , 6 , 2 , 7 ]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_upper ( ARRAY [ 1 , 8 , 3 , 7 ], 1 ) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_to_string ( ARRAY [ 1 , 2 , 3 , NULL , 5 ], ',' , '*' ) AS RESULT ;

-- 来源: 1085_file_1085
SELECT array_delete(ARRAY[1,8,3,7]) AS RESULT;

-- 来源: 1085_file_1085
SELECT array_deleteidx(ARRAY[1,2,3,4,5], 1) AS RESULT;

-- 来源: 1085_file_1085
SELECT array_extendnull(ARRAY[1,8,3,7],1) AS RESULT;

-- 来源: 1085_file_1085
SELECT array_extendnull(ARRAY[1,8,3,7],2,2) AS RESULT;

-- 来源: 1085_file_1085
SELECT array_trim(ARRAY[1,8,3,7],1) AS RESULT;

-- 来源: 1085_file_1085
SELECT array_exists(ARRAY[1,8,3,7],1) AS RESULT;

-- 来源: 1085_file_1085
SELECT array_next(ARRAY[1,8,3,7],1) AS RESULT;

-- 来源: 1085_file_1085
SELECT array_prior(ARRAY[1,8,3,7],2) AS RESULT;

-- 来源: 1085_file_1085
SELECT string_to_array ( 'xx~^~yy~^~zz' , '~^~' , 'yy' ) AS RESULT ;

-- 来源: 1085_file_1085
SELECT string_to_array ( 'xx~^~yy~^~zz' , '~^~' , 'y' ) AS RESULT ;

-- 来源: 1085_file_1085
SELECT unnest ( ARRAY [ 1 , 2 ]) AS RESULT ;

-- 来源: 1085_file_1085
SELECT cardinality(array[[1, 2], [3, 4]]);

-- 来源: 1085_file_1085
SELECT array_positions(array[1, 2, 3, 1], 1) AS RESULT;

-- 来源: 1086_file_1086
SELECT int4range ( 1 , 5 ) = '[1,4]' :: int4range AS RESULT ;

-- 来源: 1086_file_1086
SELECT numrange ( 1 . 1 , 2 . 2 ) <> numrange ( 1 . 1 , 2 . 3 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT int4range ( 1 , 10 ) < int4range ( 2 , 3 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT int4range ( 1 , 10 ) > int4range ( 1 , 5 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT numrange ( 1 . 1 , 2 . 2 ) <= numrange ( 1 . 1 , 2 . 2 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT numrange ( 1 . 1 , 2 . 2 ) >= numrange ( 1 . 1 , 2 . 0 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT int4range ( 2 , 4 ) @> int4range ( 2 , 3 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT '[2011-01-01,2011-03-01)' :: tsrange @> '2011-01-10' :: timestamp AS RESULT ;

-- 来源: 1086_file_1086
SELECT int4range ( 2 , 4 ) <@ int4range ( 1 , 7 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT 42 <@ int4range ( 1 , 7 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT int8range ( 3 , 7 ) && int8range ( 4 , 12 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT int8range ( 1 , 10 ) << int8range ( 100 , 110 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT int8range ( 50 , 60 ) >> int8range ( 20 , 30 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT int8range ( 1 , 20 ) &< int8range ( 18 , 20 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT int8range ( 7 , 20 ) &> int8range ( 5 , 10 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT numrange ( 1 . 1 , 2 . 2 ) -|- numrange ( 2 . 2 , 3 . 3 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT numrange ( 5 , 15 ) + numrange ( 10 , 20 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT int8range ( 5 , 15 ) * int8range ( 10 , 20 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT int8range ( 5 , 15 ) - int8range ( 10 , 20 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT numrange ( 1 . 1 , 2 . 2 ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT numrange ( 1 . 1 , 2 . 2 , '()' ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT lower ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- 来源: 1086_file_1086
SELECT upper ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- 来源: 1086_file_1086
SELECT isempty ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- 来源: 1086_file_1086
SELECT lower_inc ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- 来源: 1086_file_1086
SELECT upper_inc ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- 来源: 1086_file_1086
SELECT lower_inf ( '(,)' :: daterange ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT upper_inf ( '(,)' :: daterange ) AS RESULT ;

-- 来源: 1086_file_1086
SELECT elem_contained_by_range ( '2' , numrange ( 1 . 1 , 2 . 2 ));

-- 来源: 1087_file_1087
SELECT sum ( a ) FROM tab ;

-- 来源: 1087_file_1087
SELECT MAX ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- 来源: 1087_file_1087
SELECT MIN ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- 来源: 1087_file_1087
SELECT AVG ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- 来源: 1087_file_1087
SELECT COUNT ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- 来源: 1087_file_1087
SELECT COUNT ( * ) FROM tpcds . inventory ;

-- 来源: 1087_file_1087
SELECT ARRAY_AGG ( sr_fee ) FROM tpcds . store_returns WHERE sr_customer_sk = 2 ;

-- 来源: 1087_file_1087
SELECT string_agg ( sr_item_sk , ',' ) FROM tpcds . store_returns WHERE sr_item_sk < 3 ;

-- 来源: 1087_file_1087
SELECT deptno , listagg ( ename , ',' ) WITHIN GROUP ( ORDER BY ename ) AS employees FROM emp GROUP BY deptno ;

-- 来源: 1087_file_1087
SELECT deptno , listagg ( mgrno , ',' ) WITHIN GROUP ( ORDER BY mgrno NULLS FIRST ) AS mgrnos FROM emp GROUP BY deptno ;

-- 来源: 1087_file_1087
SELECT job , listagg ( bonus , '($);

-- 来源: 1087_file_1087
SELECT deptno , listagg ( hiredate , ', ' ) WITHIN GROUP ( ORDER BY hiredate DESC ) AS hiredates FROM emp GROUP BY deptno ;

-- 来源: 1087_file_1087
SELECT deptno , listagg ( vacationTime , ';

-- 来源: 1087_file_1087
SELECT deptno , listagg ( job ) WITHIN GROUP ( ORDER BY job ) AS jobs FROM emp GROUP BY deptno ;

-- 来源: 1087_file_1087
SELECT deptno , mgrno , bonus , listagg ( ename , ';

-- 来源: 1087_file_1087
SELECT id , group_concat ( v separator ';

-- 来源: 1087_file_1087
SELECT id , group_concat ( id , v ) FROM t GROUP BY id ORDER BY id ASC ;

-- 来源: 1087_file_1087
SELECT id , group_concat ( v ) FROM t GROUP BY id ORDER BY id ASC ;

-- 来源: 1087_file_1087
SELECT id , group_concat ( v separator ';

-- 来源: 1087_file_1087
SELECT id , group_concat ( v separator ';

-- 来源: 1087_file_1087
SELECT id , group_concat ( hiredate separator ';

-- 来源: 1087_file_1087
SELECT id , group_concat ( v separator ';

-- 来源: 1087_file_1087
SELECT id , group_concat ( vacationt separator ';

-- 来源: 1087_file_1087
SELECT id , group_concat ( distinct v ) FROM t GROUP BY id ORDER BY id ASC ;

-- 来源: 1087_file_1087
SELECT id , group_concat ( v ORDER BY v desc ) FROM t GROUP BY id ORDER BY id ASC ;

-- 来源: 1087_file_1087
SELECT COVAR_POP ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 1087_file_1087
SELECT COVAR_SAMP ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 1087_file_1087
SELECT STDDEV_POP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 1087_file_1087
SELECT STDDEV_SAMP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 1087_file_1087
SELECT VAR_POP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 1087_file_1087
SELECT VAR_SAMP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 1087_file_1087
SELECT BIT_AND ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 1087_file_1087
SELECT BIT_OR ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 1087_file_1087
SELECT bool_and ( 100 < 2500 );

-- 来源: 1087_file_1087
SELECT bool_or ( 100 < 2500 );

-- 来源: 1087_file_1087
SELECT CORR ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 1087_file_1087
SELECT every ( 100 < 2500 );

-- 来源: 1087_file_1087
SELECT d_moy , d_fy_week_seq , rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 1087_file_1087
SELECT REGR_AVGX ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 1087_file_1087
SELECT REGR_AVGY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 1087_file_1087
SELECT REGR_COUNT ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 1087_file_1087
SELECT REGR_INTERCEPT ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 1087_file_1087
SELECT REGR_R2 ( sr_fee , sr_net_loss ) FROM store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 1087_file_1087
SELECT REGR_SLOPE ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 1087_file_1087
SELECT REGR_SXX ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 1087_file_1087
SELECT REGR_SXY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 1087_file_1087
SELECT REGR_SYY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 1087_file_1087
SELECT STDDEV ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 1087_file_1087
SELECT VARIANCE ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 1087_file_1087
SELECT * FROM pivot_func_test;

-- 来源: 1087_file_1087
SELECT id, pivot_func(val) FROM pivot_func_test GROUP BY id;

-- 来源: 1087_file_1087
SELECT CHECKSUM ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- 来源: 1087_file_1087
SELECT CHECKSUM ( inv_quantity_on_hand :: TEXT ) FROM tpcds . inventory ;

-- 来源: 1087_file_1087
SELECT CHECKSUM ( inventory :: TEXT ) FROM tpcds . inventory ;

-- 来源: 1088_file_1088
SELECT d_moy , d_fy_week_seq , rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 1088_file_1088
SELECT d_moy , d_fy_week_seq , Row_number () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 1088_file_1088
SELECT d_moy , d_fy_week_seq , dense_rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 1088_file_1088
SELECT d_moy , d_fy_week_seq , percent_rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 1088_file_1088
SELECT d_moy , d_fy_week_seq , cume_dist () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim e_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 1088_file_1088
SELECT d_moy , d_fy_week_seq , ntile ( 3 ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 1088_file_1088
SELECT d_moy , d_fy_week_seq , lag ( d_moy , 3 , null ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 1088_file_1088
SELECT d_moy , d_fy_week_seq , lead ( d_fy_week_seq , 2 ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 1088_file_1088
SELECT d_moy , d_fy_week_seq , first_value ( d_fy_week_seq ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 1088_file_1088
SELECT d_moy , d_fy_week_seq , last_value ( d_moy ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 6 ORDER BY 1 , 2 ;

-- 来源: 1088_file_1088
SELECT d_moy , d_fy_week_seq , nth_value ( d_fy_week_seq , 6 ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 6 ORDER BY 1 , 2 ;

-- 来源: 1088_file_1088
SELECT sales_group , sales_id , sales_amount , RATIO_TO_REPORT ( sales_amount ) OVER ( PARTITION BY sales_group ) FROM sales_int8 ORDER BY sales_id ;

-- 来源: 1088_file_1088
SELECT sales_group , sales_id , sales_amount , TO_CHAR ( RATIO_TO_REPORT ( sales_amount ) OVER (), '$999eeee' ) FROM sales ORDER BY sales_id ;

-- 来源: 1089_file_1089
SELECT gs_encrypt_aes128 ( 'MPPDB' , '1234@abc' );

-- 来源: 1089_file_1089
SELECT gs_decrypt_aes128 ( 'OF1g3+70oeqFfyKiWlpxfYxPnpeitNc6+7nAe02Ttt37fZF8Q+bbEYhdw/YG+0c9tHKRWM6OcTzlB3HnqvX+1d8Bflo=' , '1234@abc' );

-- 来源: 1089_file_1089
select aes_encrypt('huwei123','123456vfhex4dyu,vdaladhjsadad','1234567890123456');

-- 来源: 1089_file_1089
select aes_decrypt(aes_encrypt('huwei123','123456vfhex4dyu,vdaladhjsadad','1234567890123456'),'123456vfhex4dyu,vdaladhjsadad','1234567890123456');

-- 来源: 1089_file_1089
SELECT pg_catalog . gs_digest ( 'gaussdb' , 'sha256' );

-- 来源: 1089_file_1089
SELECT gs_password_deadline ();

-- 来源: 1089_file_1089
SELECT inet_server_addr ();

-- 来源: 1089_file_1089
SELECT inet_client_addr ();

-- 来源: 1089_file_1089
SELECT gs_encrypt('MPPDB', 'Asdf1234', 'sm4');

-- 来源: 1089_file_1089
select gs_decrypt('ZBzOmaGA4Bb+coyucJ0B8AkIShqc', 'Asdf1234', 'sm4');

-- 来源: 1089_file_1089
SELECT gs_encrypt_bytea('MPPDB', 'Asdf1234', 'sm4_ctr_sm3');

-- 来源: 1089_file_1089
select gs_decrypt_bytea('\x90e286971c2c70410def0a2814af4ac44c737926458b66271d9d1547bc937395ca018d7755672fa9dc3cdc6ec4a76001dc0e137f3bc5c8a5c51143561f1d09a848bfdebfec5e', 'Asdf1234', 'sm4_ctr_sm3');

-- 来源: 1091_file_1091
SELECT * FROM generate_series ( 2 , 4 );

-- 来源: 1091_file_1091
SELECT * FROM generate_series ( 5 , 1 , - 2 );

-- 来源: 1091_file_1091
SELECT * FROM generate_series ( 4 , 3 );

-- 来源: 1091_file_1091
SELECT current_date + s . a AS dates FROM generate_series ( 0 , 14 , 7 ) AS s ( a );

-- 来源: 1091_file_1091
SELECT * FROM generate_series ( '2008-03-01 00:00' :: timestamp , '2008-03-04 12:00' , '10 hours' );

-- 来源: 1091_file_1091
SELECT generate_subscripts ( '{NULL,1,NULL,2}' :: int [], 1 ) AS s ;

-- 来源: 1091_file_1091
SELECT * FROM unnest2 ( ARRAY [[ 1 , 2 ],[ 3 , 4 ]]);

-- 来源: 1092_file_1092
SELECT coalesce ( NULL , 'hello' );

-- 来源: 1092_file_1092
SELECT decode ( 'A' , 'A' , 1 , 'B' , 2 , 0 );

-- 来源: 1092_file_1092
SELECT nullif ( 'hello' , 'world' );

-- 来源: 1092_file_1092
SELECT nullif ( '1234' :: VARCHAR , 123 :: INT4 );

-- 来源: 1092_file_1092
SELECT nullif ( '1234' :: VARCHAR , '2012-12-24' :: DATE );

-- 来源: 1092_file_1092
SELECT nullif ( 1 :: bit , '1' :: MONEY );

-- 来源: 1092_file_1092
SELECT nvl ( 'hello' , 'world' );

-- 来源: 1092_file_1092
SELECT nvl2 ( 'hello' , 'world' , 'other' );

-- 来源: 1092_file_1092
SELECT greatest ( 1 * 2 , 2 - 3 , 4 - 1 );

-- 来源: 1092_file_1092
SELECT greatest ( 'HARRY' , 'HARRIOT' , 'HAROLD' );

-- 来源: 1092_file_1092
SELECT least ( 1 * 2 , 2 - 3 , 4 - 1 );

-- 来源: 1092_file_1092
SELECT least ( 'HARRY' , 'HARRIOT' , 'HAROLD' );

-- 来源: 1092_file_1092
SELECT * FROM student_demo WHERE LNNVL ( name = 'name1' );

-- 来源: 1092_file_1092
SELECT isnull ( null );

-- 来源: 1092_file_1092
SELECT isnull ( 1 );

-- 来源: 1092_file_1092
select if ( 2 > 3 , 'true' , 'false' );

-- 来源: 1092_file_1092
select if ( null , 'not null' , 'is null' );

-- 来源: 1092_file_1092
select ifnull ( '' , null ) is null as a ;

-- 来源: 1092_file_1092
select ifnull ( null , null ) is null as a ;

-- 来源: 1092_file_1092
select ifnull ( null , 'A' ) as a ;

-- 来源: 1093_file_1093
SELECT current_query ();

-- 来源: 1093_file_1093
SELECT current_schema ();

-- 来源: 1093_file_1093
SELECT current_schemas ( true );

-- 来源: 1093_file_1093
SELECT database ();

-- 来源: 1093_file_1093
SELECT current_user ;

-- 来源: 1093_file_1093
SELECT definer_current_user ();

-- 来源: 1093_file_1093
SELECT pg_current_sessionid ();

-- 来源: 1093_file_1093
select pg_current_sessid();

-- 来源: 1093_file_1093
SELECT pg_current_userid();

-- 来源: 1093_file_1093
select tablespace_oid_name ( 1663 );

-- 来源: 1093_file_1093
SELECT inet_client_addr ();

-- 来源: 1093_file_1093
SELECT inet_client_port ();

-- 来源: 1093_file_1093
SELECT inet_server_addr ();

-- 来源: 1093_file_1093
SELECT inet_server_port ();

-- 来源: 1093_file_1093
SELECT pg_backend_pid ();

-- 来源: 1093_file_1093
SELECT pg_conf_load_time ();

-- 来源: 1093_file_1093
SELECT pg_my_temp_schema ();

-- 来源: 1093_file_1093
SELECT pg_is_other_temp_schema ( 25356 );

-- 来源: 1093_file_1093
SELECT pg_listening_channels ();

-- 来源: 1093_file_1093
SELECT pg_postmaster_start_time ();

-- 来源: 1093_file_1093
select sessionid2pid ( sessid :: cstring ) from pv_session_stat limit 2 ;

-- 来源: 1093_file_1093
SELECT session_context ( 'USERENV' , 'CURRENT_SCHEMA' );

-- 来源: 1093_file_1093
SELECT pg_trigger_depth ();

-- 来源: 1093_file_1093
SELECT opengauss_version ();

-- 来源: 1093_file_1093
select gs_deployment ();

-- 来源: 1093_file_1093
SELECT session_user ;

-- 来源: 1093_file_1093
SELECT user ;

-- 来源: 1093_file_1093
select get_shard_oids_byname ( 'datanode1' );

-- 来源: 1093_file_1093
select getpgusername ();

-- 来源: 1093_file_1093
select getdatabaseencoding ();

-- 来源: 1093_file_1093
SELECT version ();

-- 来源: 1093_file_1093
SELECT working_version_num ();

-- 来源: 1093_file_1093
SELECT get_hostname ();

-- 来源: 1093_file_1093
SELECT get_nodename ();

-- 来源: 1093_file_1093
SELECT get_nodeinfo ( 'node_type' );

-- 来源: 1093_file_1093
SELECT get_nodeinfo ( 'node_name' );

-- 来源: 1093_file_1093
SELECT get_schema_oid ( 'public' );

-- 来源: 1093_file_1093
SELECT pgxc_parse_clog ();

-- 来源: 1093_file_1093
SELECT pgxc_parse_clog ( '-1' );

-- 来源: 1093_file_1093
SELECT pgxc_prepared_xact ();

-- 来源: 1093_file_1093
SELECT pgxc_xacts_iscommitted ( 1 );

-- 来源: 1093_file_1093
SELECT pgxc_total_memory_detail ();

-- 来源: 1093_file_1093
SELECT has_table_privilege ( 'tpcds.web_site' , 'select' );

-- 来源: 1093_file_1093
SELECT has_table_privilege ( 'omm' , 'tpcds.web_site' , 'select,INSERT WITH GRANT OPTION ' );

-- 来源: 1093_file_1093
SELECT relname FROM pg_class WHERE pg_table_is_visible ( oid );

-- 来源: 1093_file_1093
SELECT format_type (( SELECT oid FROM pg_type WHERE typname = 'varchar' ), 10 );

-- 来源: 1093_file_1093
select pg_check_authid(1);

-- 来源: 1093_file_1093
select * from pg_get_functiondef(598);

-- 来源: 1093_file_1093
select * from pg_get_indexdef(16416);

-- 来源: 1093_file_1093
select * from pg_get_indexdef(16416, true);

-- 来源: 1093_file_1093
select * from pg_get_indexdef(16416, 0, false);

-- 来源: 1093_file_1093
select * from pg_get_indexdef(16416, 1, false);

-- 来源: 1093_file_1093
select pg_check_authid(20);

-- 来源: 1093_file_1093
select * from pg_get_tabledef(16384);

-- 来源: 1093_file_1093
select * from pg_get_tabledef('t1');

-- 来源: 1093_file_1093
SELECT pg_typeof ( 33 );

-- 来源: 1093_file_1093
SELECT typlen FROM pg_type WHERE oid = pg_typeof ( 33 );

-- 来源: 1093_file_1093
SELECT collation for ( description ) FROM pg_description LIMIT 1 ;

-- 来源: 1093_file_1093
SELECT getdistributekey ( 'item' );

-- 来源: 1093_file_1093
select * from pg_get_serial_sequence('t1', 'c1');

-- 来源: 1093_file_1093
select * from pg_sequence_parameters(16420);

-- 来源: 1093_file_1093
select pgxc_get_variable_info( );

-- 来源: 1093_file_1093
select * from gs_get_index_status('public', 'index1');

-- 来源: 1093_file_1093
select * from gs_get_kernel_info();

-- 来源: 1095_file_1095
SELECT current_setting ( 'datestyle' );

-- 来源: 1095_file_1095
SELECT set_config ( 'log_statement_stats' , 'off' , false );

-- 来源: 1096_file_1096
SELECT pg_ls_dir ( './' );

-- 来源: 1096_file_1096
SELECT pg_read_file ( 'postmaster.pid' , 0 , 100 );

-- 来源: 1096_file_1096
SELECT convert_from ( pg_read_binary_file ( 'filename' ), 'UTF8' );

-- 来源: 1096_file_1096
SELECT * FROM pg_stat_file ( 'filename' );

-- 来源: 1096_file_1096
SELECT ( pg_stat_file ( 'filename' )). modification ;

-- 来源: 1096_file_1096
SELECT convert_from ( pg_read_binary_file ( 'postmaster.pid' ), 'UTF8' );

-- 来源: 1096_file_1096
SELECT * FROM pg_stat_file ( 'postmaster.pid' );

-- 来源: 1096_file_1096
SELECT ( pg_stat_file ( 'postmaster.pid' )). modification ;

-- 来源: 1097_file_1097
SELECT pid from pg_stat_activity ;

-- 来源: 1097_file_1097
SELECT pg_terminate_backend ( 140657876268816 );

-- 来源: 1098_file_1098
SELECT pg_start_backup ( 'label_goes_here' , true );

-- 来源: 1098_file_1098
SELECT * FROM pg_xlogfile_name_offset ( pg_stop_backup ());

-- 来源: 1099_file_1099
select * from pg_create_physical_replication_slot_extern ( 'uuid' , false , 'OBS;

-- 来源: 1099_file_1099
select * from pg_create_physical_replication_slot_extern ( 'uuid' , false , 'NAS;

-- 来源: 1099_file_1099
select gs_set_obs_delete_location('0/54000000');

-- 来源: 1102_file_1102
SELECT pg_column_size ( 1 );

-- 来源: 1102_file_1102
SELECT pg_database_size ( 'testdb' );

-- 来源: 1102_file_1102
select get_db_source_datasize ();

-- 来源: 1102_file_1102
SELECT datalength(1);

-- 来源: 1104_file_1104
select * from pg_create_logical_replication_slot('slot_lsn','mppdb_decoding',0);

-- 来源: 1104_file_1104
select * from pg_create_logical_replication_slot('slot_csn','mppdb_decoding',1);

-- 来源: 1104_file_1104
select * from pg_logical_slot_peek_changes('slot_lsn',NULL,4096,'skip-empty-xacts','on');

-- 来源: 1104_file_1104
select * from pg_logical_slot_peek_changes('slot_csn',NULL,4096,'skip-empty-xacts','on');

-- 来源: 1104_file_1104
select * from pg_get_replication_slots();

-- 来源: 1104_file_1104
select * from pg_get_replication_slots();

-- 来源: 1104_file_1104
select * from pg_logical_get_area_changes('0/502E418', NULL, NULL, 'sql_decoding', NULL);

-- 来源: 1104_file_1104
select * from gs_get_parallel_decode_status();

-- 来源: 1104_file_1104
select * from gs_get_slot_decoded_wal_time('replication_slot');

-- 来源: 1104_file_1104
select * from gs_logical_parallel_decode_status('replication_slot');

-- 来源: 1104_file_1104
select * from gs_logical_parallel_decode_status('replication_slot');

-- 来源: 1104_file_1104
select * from gs_logical_parallel_decode_reset_status('replication_slot');

-- 来源: 1104_file_1104
select * from gs_logical_parallel_decode_status('replication_slot');

-- 来源: 1104_file_1104
select * from gs_get_parallel_decode_thread_info();

-- 来源: 1104_file_1104
SELECT * FROM gs_get_distribute_decode_status();

-- 来源: 1104_file_1104
SELECT * FROM gs_get_distribute_decode_status_detail();

-- 来源: 1105_file_1105
select * from gs_seg_dump_page('pg_default', 1, 1024, 4157);

-- 来源: 1105_file_1105
select * from gs_seg_dump_page(16788, 1024, 0);

-- 来源: 1105_file_1105
select * from gs_seg_get_spc_location('pg_default', 1024, 4157, 0);

-- 来源: 1105_file_1105
select * from gs_seg_get_spc_location(24578,1024, 0);

-- 来源: 1105_file_1105
select * from gs_seg_get_location(4157);

-- 来源: 1105_file_1105
select * from gs_seg_get_segment_layout();

-- 来源: 1105_file_1105
select * from gs_seg_get_datafile_layout();

-- 来源: 1105_file_1105
select * from gs_seg_get_slice_layout(1,1024,0);

-- 来源: 1105_file_1105
select * from gs_seg_get_segment('pg_default', 1024, 4157);

-- 来源: 1105_file_1105
select * from gs_seg_get_segment(16768, 1024);

-- 来源: 1105_file_1105
select * from gs_seg_get_extents('pg_default', 1024, 4157);

-- 来源: 1105_file_1105
select * from gs_seg_get_extents(16768, 1024);

-- 来源: 1105_file_1105
select * from gs_seg_free_spc_remain_segment('pg_default', 1, 4159);

-- 来源: 1105_file_1105
select * from gs_seg_free_spc_remain_extent('pg_default', 1, 0, 4159);

-- 来源: 1105_file_1105
select * from gs_seg_get_datafiles();

-- 来源: 1105_file_1105
select * from gs_seg_get_spc_extents('pg_default', 1,1024, 0);

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_get_plan(16388, 16417);

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_get_bucket_statistics();

-- 来源: 1106_hashbucket
SELECT gs_redis_set_distributed_db('gaussdb');

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_hashbucket_update_segment_header(16388, 16417);

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_local_get_segment_header('mytable', '256');

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_local_update_segment_header('mytable', '4294967295,4294967295,4294967295,4294967295,....');

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_hashbucket_update_inverse_pointer('0,1,2,3,4,5,6,7,8,9,10','datanode1','datanode3');

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_hashbucket_update_inverse_pointer('0,1,2,3,4,5,6,7,8,9,10','datanode1','datanode3');

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_local_update_inverse_pointer('mytable', '4294967295,4294967295,4294967295,4294967295,....','1 2 3');

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_local_set_hashbucket_frozenxid();

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_set_hashbucket_frozenxid(16388, 16417);

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_set_nextxid('15268817');

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_set_csn('15268817');

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_check_bucket_flush('{datanode1， datanode2}');

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_show_bucketxid('1 2 3');

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_drop_bucket_files(16388, 16417);

-- 来源: 1106_hashbucket
SELECT * FROM gs_redis_local_drop_bucket_files('1 2 3', 3);

-- 来源: 1107_Undo
select * from gs_global_config where name like '%undostoragetype%';

-- 来源: 1107_Undo
select * from gs_stat_undo(true);

-- 来源: 1107_Undo
select * from gs_stat_undo(false);

-- 来源: 1107_Undo
select * from gs_undo_meta_dump_zone(-1,true);

-- 来源: 1107_Undo
select * from gs_undo_translot_dump_slot(-1,true);

-- 来源: 1107_Undo
select * from gs_undo_translot_dump_xid('15758',false);

-- 来源: 1107_Undo
select * from gs_undo_dump_record('0000000000000042');

-- 来源: 1107_Undo
select * from gs_undo_dump_xid('15779');

-- 来源: 1107_Undo
select * from gs_verify_undo_record('urp', 24, 24, 1);

-- 来源: 1107_Undo
select * from gs_verify_undo_record('zone', 0, 2, 1);

-- 来源: 1107_Undo
select * from gs_verify_undo_slot('zone', 0, 2, 1);

-- 来源: 1107_Undo
select * from gs_verify_undo_meta('all', 0, 2, 1);

-- 来源: 1108_file_1108
select table_skewness('t', 'a',5);

-- 来源: 1108_file_1108
select table_skewness('t', 'a');

-- 来源: 1108_file_1108
select table_data_skewness(row(index), 'R') from test1;

-- 来源: 1108_file_1108
select pg_stat_get_env();

-- 来源: 1108_file_1108
select locktag_decode('271b:0:0:0:0:6');

-- 来源: 1108_file_1108
select gs_parse_page_bypath('base/16603/16394', -1, 'btree', false);

-- 来源: 1108_file_1108
select gs_parse_page_bypath('base/12828/16771_vm', -1, 'vm', false);

-- 来源: 1108_file_1108
select gs_parse_page_bypath('000000000000', 0, 'clog', false);

-- 来源: 1108_file_1108
select gs_parse_page_bypath('base/12828/16777', -10, 'heap', false);

-- 来源: 1108_file_1108
select * from gs_stat_space(false);

-- 来源: 1108_file_1108
select * from gs_parse_page_bypath('base/15833/16768', 0, 'uheap', false);

-- 来源: 1108_file_1108
select * from gs_xlogdump_bylastlsn('0/4593570', -1, 'uheap');

-- 来源: 1108_file_1108
select * from gs_xlogdump_bylastlsn('0/4593570', 0, 'ubtree');

-- 来源: 1108_file_1108
SELECT query,unique_query_id,start_time,finish_time FROM dbe_perf.statement_history;

-- 来源: 1108_file_1108
SELECT query FROM dbe_perf.get_full_sql_by_parent_id_and_timestamp(536458473,'2023-06-02 17:40:59.028144+08','2023-06-02 17:40:59.032027+08');

-- 来源: 1108_file_1108
SELECT * FROM gs_index_dump_read(0, 'all');

-- 来源: 1108_file_1108
SELECT * FROM gs_index_dump_read(1, 'all');

-- 来源: 1110_file_1110
select pg_stat_get_role_name(10);

-- 来源: 1110_file_1110
select * from pg_stat_get_activity(139881386280704);

-- 来源: 1110_file_1110
select * from gs_stat_get_hotkeys_info () order by count , hash_value ;

-- 来源: 1110_file_1110
select * from gs_stat_clean_hotkeys ();

-- 来源: 1110_file_1110
select * from global_stat_get_hotkeys_info () order by count , hash_value ;

-- 来源: 1110_file_1110
select * from global_stat_clean_hotkeys ();

-- 来源: 1110_file_1110
SELECT pg_backend_pid ();

-- 来源: 1110_file_1110
SELECT pg_stat_get_backend_pid ( 1 );

-- 来源: 1110_file_1110
select * from gs_stack ( 139663481165568 );

-- 来源: 1110_file_1110
select * from gs_stack ();

-- 来源: 1110_file_1110
SELECT * FROM gs_perf_start ( 10 , 100 );

-- 来源: 1110_file_1110
SELECT * FROM gs_perf_query () WHERE overhead > 2 AND level < 10 ;

-- 来源: 1110_file_1110
SELECT * FROM gs_perf_clean ();

-- 来源: 1110_file_1110
select sessionid from pg_stat_activity where usename = 'testuser';

-- 来源: 1110_file_1110
select * from gs_session_all_settings(788861) where name = 'work_mem';

-- 来源: 1110_file_1110
select * from gs_session_all_settings() where name = 'work_mem';

-- 来源: 1110_file_1110
select * from gs_local_wal_preparse_statistics();

-- 来源: 1110_file_1110
select * from gs_hot_standby_space_info();

-- 来源: 1110_file_1110
SELECT * FROM exrto_file_read_stat();

-- 来源: 1110_file_1110
SELECT * FROM gs_exrto_recycle_info();

-- 来源: 1110_file_1110
SELECT * FROM gs_stat_get_db_conflict_all(12738);

-- 来源: 1110_file_1110
SELECT * FROM gs_redo_stat_info();

-- 来源: 1110_file_1110
SELECT * FROM gs_recovery_conflict_waitevent_info();

-- 来源: 1110_file_1110
SELECT * FROM gs_display_delay_ddl_info();

-- 来源: 1110_file_1110
SELECT * FROM gs_stat_all_partitions;

-- 来源: 1110_file_1110
SELECT * FROM gs_statio_all_partitions;

-- 来源: 1110_file_1110
SELECT * FROM gs_stat_get_partition_stats(16952);

-- 来源: 1110_file_1110
SELECT * FROM gs_stat_xact_all_partitions;

-- 来源: 1110_file_1110
SELECT * FROM gs_stat_get_xact_partition_stats(16952);

-- 来源: 1112_HashFunc
select bucketabstime ( '2011-10-01 10:10:10.112' , 1 );

-- 来源: 1112_HashFunc
select bucketbool ( true , 1 );

-- 来源: 1112_HashFunc
select bucketbool ( false , 1 );

-- 来源: 1112_HashFunc
select bucketbpchar ( 'test' , 1 );

-- 来源: 1112_HashFunc
select bucketbytea ( 'test' , 1 );

-- 来源: 1112_HashFunc
select bucketcash ( 10 :: money , 1 );

-- 来源: 1112_HashFunc
select getbucket ( 10 , 'H' );

-- 来源: 1112_HashFunc
select getbucket ( 11 , 'H' );

-- 来源: 1112_HashFunc
select getbucket ( 11 , 'R' );

-- 来源: 1112_HashFunc
select getbucket ( 12 , 'R' );

-- 来源: 1112_HashFunc
select ora_hash ( 123 );

-- 来源: 1112_HashFunc
select ora_hash ( '123' );

-- 来源: 1112_HashFunc
select ora_hash ( 'sample' );

-- 来源: 1112_HashFunc
select ora_hash ( to_date ( '2012-1-2' , 'yyyy-mm-dd' ));

-- 来源: 1112_HashFunc
select ora_hash ( 123 , 234 );

-- 来源: 1112_HashFunc
select ora_hash ( '123' , 234 );

-- 来源: 1112_HashFunc
select ora_hash ( 'sample' , 234 );

-- 来源: 1112_HashFunc
select ora_hash ( to_date ( '2012-1-2' , 'yyyy-mm-dd' ), 234 );

-- 来源: 1112_HashFunc
select hash_array ( ARRAY [[ 1 , 2 , 3 ],[ 1 , 2 , 3 ]]);

-- 来源: 1112_HashFunc
select hash_numeric ( 30 );

-- 来源: 1112_HashFunc
select hash_range ( numrange ( 1 . 1 , 2 . 2 ));

-- 来源: 1112_HashFunc
select hashbpchar ( 'hello' );

-- 来源: 1112_HashFunc
select hashbpchar ( 'hello' );

-- 来源: 1112_HashFunc
select hashchar ( 'true' );

-- 来源: 1112_HashFunc
select hashfloat4 ( 12 . 1234 );

-- 来源: 1112_HashFunc
select hashfloat8 ( 123456 . 1234 );

-- 来源: 1112_HashFunc
select hashinet ( '127.0.0.1' :: inet );

-- 来源: 1112_HashFunc
select hashint1 ( 20 );

-- 来源: 1112_HashFunc
select hashint2(20000);

-- 来源: 1121_hotkey
select * from gs_stat_get_hotkeys_info () order by count , hash_value ;

-- 来源: 1121_hotkey
select * from gs_stat_clean_hotkeys ();

-- 来源: 1122_Global SysCache
select * from gs_gsc_catalog_detail(16574, 1260);

-- 来源: 1122_Global SysCache
select * from gs_gsc_clean();

-- 来源: 1122_Global SysCache
select * from gs_gsc_dbstat_info();

-- 来源: 1123_file_1123
select * from gs_verify_data_file();

-- 来源: 1123_file_1123
select * from gs_verify_data_file(true);

-- 来源: 1123_file_1123
select * from gs_repair_file(16554,'base/16552/24745',360);

-- 来源: 1123_file_1123
select * from local_bad_block_info();

-- 来源: 1123_file_1123
select * from local_clear_bad_block_info();

-- 来源: 1123_file_1123
select * from gs_verify_and_tryrepair_page('base/16552/24745',0,false,false);

-- 来源: 1123_file_1123
select * from gs_repair_page('base/16552/24745',0,false,60);

-- 来源: 1123_file_1123
select gs_edit_page_bypath('base/15808/25075',0,16,'0x1FFF', 2, false, 'page');

-- 来源: 1123_file_1123
select gs_edit_page_bypath('base/15808/25075', 0,16,'@1231!', 8, false, 'page');

-- 来源: 1123_file_1123
select gs_edit_page_bypath('/pg_log_dir/dump/1663_15808_25075_0.editpage', 0,16,'0x1FFF', 2, true, 'page');

-- 来源: 1123_file_1123
select * from gs_repair_page_bypath('pg_log/dump/1663_15991_16767_0.editpage', 0, 'base/15991/16767', 0, 'page');

-- 来源: 1123_file_1123
select * from gs_repair_page_bypath('standby', 0, 'base/15990/16768', 0, 'page');

-- 来源: 1123_file_1123
select * from gs_repair_page_bypath('init_block', 0, 'base/15990/16768', 0, 'page');

-- 来源: 1123_file_1123
select * from gs_repair_undo_byzone(4);

-- 来源: 1123_file_1123
select * from gs_repair_undo_byzone(78);

-- 来源: 1123_file_1123
select * from gs_repair_undo_byzone(0);

-- 来源: 1123_file_1123
select * from gs_verify_urq(16387, 0, 1, 'free queue');

-- 来源: 1123_file_1123
select * from gs_verify_urq(16387, 0, 1, 'empty queue');

-- 来源: 1123_file_1123
SELECT * FROM gs_urq_dump_stat(16387, 0);

-- 来源: 1123_file_1123
SELECT gs_urq_dump_stat(17260,0);

-- 来源: 1123_file_1123
select * from gs_repair_urq(16387, 0);

-- 来源: 1123_file_1123
select * from gs_get_standby_bad_block_info();

-- 来源: 1124_XML
SELECT XMLPARSE ( DOCUMENT '<?xml version="1.0"?><book><title>Manual</title><chapter>...</chapter></book>' );

-- 来源: 1124_XML
SELECT XMLPARSE ( CONTENT 'abc<foo>bar</foo><bar>foo</bar>' );

-- 来源: 1124_XML
SELECT XMLPARSE ( CONTENT 'abc<foo>bar</foo' wellformed );

-- 来源: 1124_XML
select XMLCONCAT(('<?xml version="1.0" encoding="GB2312" standalone="no"?><bar>foo</bar>'),('<?xml version="1.0" encoding="GB2312" standalone="no" ?><bar>foo</bar>')) ;

-- 来源: 1124_XML
select XMLCONCAT('abc>');

-- 来源: 1124_XML
select XMLCONCAT(('<?xml version="1.0" encoding="GB2312" standalone="no"?><bar>foo</bar>'),('<?xml version="1.0" encoding="GB2312" standalone="no" ?><bar>foo</bar>')) ;

-- 来源: 1124_XML
select XMLCONCAT('abc>');

-- 来源: 1124_XML
SELECT xmlagg ( data ) FROM xmltest ;

-- 来源: 1124_XML
SELECT xmlagg ( data ) FROM xmltest ;

-- 来源: 1124_XML
SELECT xmlagg ( data ) FROM xmltest ;

-- 来源: 1124_XML
SELECT xmlagg ( data order by id desc ) FROM xmltest ;

-- 来源: 1124_XML
SELECT xmlelement ( name foo );

-- 来源: 1124_XML
SELECT xmlelement ( "entityescaping<>" , 'a$><&"b' );

-- 来源: 1124_XML
SELECT xmlelement ( entityescaping "entityescaping<>" , 'a$><&"b' );

-- 来源: 1124_XML
SELECT xmlelement ( noentityescaping "entityescaping<>" , 'a$><&"b' );

-- 来源: 1124_XML
SELECT xmlelement(" entityescaping <> ", '<abc/>' b);

-- 来源: 1124_XML
SELECT xmlelement(" entityescaping <> ", '<abc/>' as b);

-- 来源: 1124_XML
SELECT xmlelement(" entityescaping <> ", xml('<abc/>') b);

-- 来源: 1124_XML
SELECT xmlelement(" entityescaping <> ", xml('<abc/>') as b);

-- 来源: 1124_XML
SELECT xmlelement(" entityescaping <> ", xmlattributes('entityescaping<>' " entityescaping <> "));

-- 来源: 1124_XML
SELECT xmlelement(name " entityescaping <> ", xmlattributes(entityescaping 'entityescaping<>' " entityescaping <> "));

-- 来源: 1124_XML
SELECT xmlelement(" entityescaping <> ", xmlattributes(noentityescaping 'entityescaping<>' " entityescaping <> "));

-- 来源: 1124_XML
select getclobval ( xmlparse ( document '<a>123</a>' ));

-- 来源: 1124_XML
select getstringval(xmlparse(document '<a>123<b>456</b></a>'));

-- 来源: 1124_XML
SELECT xmlsequence(xml('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>'));

-- 来源: 1125_XMLTYPE
SELECT createxml ( '<a>123</a>' );

-- 来源: 1125_XMLTYPE
SELECT xmltype . createxml ( '<a>123</a>' );

-- 来源: 1125_XMLTYPE
select xmltype ( '<a>123<b>456</b></a>' ). extract ( '/a/b' ). getstringval ();

-- 来源: 1125_XMLTYPE
select getstringval ( extractxml ( xmltype ( '<a>123<b>456</b></a>' ), '/a/b' ));

-- 来源: 1125_XMLTYPE
SELECT getblobval ( xmltype ( '<asd/>' ), 7 );

-- 来源: 1125_XMLTYPE
select xmltype ( '<asd/>' ). getblobVal ( 7 );

-- 来源: 1125_XMLTYPE
SELECT getclobval ( xmltype ( '<a>123</a>' ));

-- 来源: 1125_XMLTYPE
SELECT xmltype ( '<a>123</a>' ). getclobval ();

-- 来源: 1125_XMLTYPE
SELECT getnumberval ( xmltype ( '<a>123</a>' ). extract ( '/a/text()' ));

-- 来源: 1125_XMLTYPE
SELECT xmltype ( '<a>123</a>' ). extract ( '/a/text()' ). getnumberval ();

-- 来源: 1125_XMLTYPE
SELECT isfragment ( xmltype ( '<a>123</a>' ));

-- 来源: 1125_XMLTYPE
SELECT xmltype ( '<a>123</a>' ). isfragment ();

-- 来源: 1125_XMLTYPE
SELECT xmltype ( '<a>123</a>' );

-- 来源: 1125_XMLTYPE
select getstringval('<a>123<b>456</b></a>');

-- 来源: 1125_XMLTYPE
select xmltype('<a>123<b>456</b></a>').getstringval();

-- 来源: 1125_XMLTYPE
select getrootelement('<a>123<b>456</b></a>');

-- 来源: 1125_XMLTYPE
select xmltype('<a>123<b>456</b></a>').getrootelement();

-- 来源: 1125_XMLTYPE
select getnamespace('<c:a xmlns:c="asd">123<d:b xmlns:d="qwe">456</d:b></c:a>');

-- 来源: 1125_XMLTYPE
select xmltype('<c:a xmlns:c="asd">123<d:b xmlns:d="qwe">456</d:b></c:a>').getnamespace();

-- 来源: 1125_XMLTYPE
select existsnode('<a>123<b>456</b></a>','/a/b');

-- 来源: 1125_XMLTYPE
select xmltype('<a>123<b>456</b></a>').existsnode('/a/b');

-- 来源: 1125_XMLTYPE
select existsnode('<a:b xmlns:a="asd">123<c>456</c></a:b>','/a:b/c','xmlns:a="asd"');

-- 来源: 1125_XMLTYPE
select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').existsnode('/a:b/c','xmlns:a="asd"');

-- 来源: 1125_XMLTYPE
select extractxml('<a>123<b>456</b></a>','/a/b');

-- 来源: 1125_XMLTYPE
select xmltype('<a>123<b>456</b></a>').extract('/a/b');

-- 来源: 1125_XMLTYPE
select xmltype('<a>123<b>456</b></a>').extractxml('/a/b');

-- 来源: 1125_XMLTYPE
select extractxml('<a:b xmlns:a="asd">123<c>456</c></a:b>','/a:b','xmlns:a="asd"');

-- 来源: 1125_XMLTYPE
select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').extract('/a:b','xmlns:a="asd"');

-- 来源: 1125_XMLTYPE
select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').extractxml('/a:b','xmlns:a="asd"');

-- 来源: 1125_XMLTYPE
SELECT xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>'));

-- 来源: 1125_XMLTYPE
SELECT unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//title/text()'))) AS title , unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//author/text()'))) AS author;

-- 来源: 1125_XMLTYPE
SELECT array_to_json(array_agg(row_to_json(t))) FROM ( SELECT unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//title/text()'))) AS title , unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinnger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//author/text()'))) AS author ) t;

-- 来源: 1126_file_1126
select * from cross_test ;

-- 来源: 1126_file_1126
select * from crosstab ( 'select group_, id, var from cross_test order by 1, 2;

-- 来源: 1126_file_1126
select * from crosstab2 ( 'select group_, id, var from cross_test order by 1, 2;

-- 来源: 1126_file_1126
select * from crosstab ( 'select group_, id, var from cross_test order by 1, 2;

-- 来源: 1127_file_1127
select uuid ();

-- 来源: 1127_file_1127
SELECT uuid_short ();

-- 来源: 1128_SQL
select gs_add_workload_rule ( 'sqlid' , 'rule for one query' , '{}' , now (), NULL , 20 , '{id=32413214}' );

-- 来源: 1128_SQL
select gs_add_workload_rule ( 'select' , 'rule for select' , '{db1, db2}' , NULL , NULL , 100 , '{tb1, tb2}' );

-- 来源: 1128_SQL
select gs_add_workload_rule ( 'resource' , 'rule for resource' , '{}' , NULL , NULL , 20 , '{cpu-80}' );

-- 来源: 1128_SQL
select gs_update_workload_rule ( 2 , 'rule for select 2' , '{db1}' , now (), NULL , 50 , '{tb1}' );

-- 来源: 1128_SQL
select gs_delete_workload_rule ( 3 );

-- 来源: 1128_SQL
select * from gs_get_workload_rule_stat ( 1 );

-- 来源: 1128_SQL
select * from gs_get_workload_rule_stat ( - 1 );

-- 来源: 1131_file_1131
SELECT 2 BETWEEN 1 AND 3 AS RESULT ;

-- 来源: 1131_file_1131
SELECT 2 >= 1 AND 2 <= 3 AS RESULT ;

-- 来源: 1131_file_1131
SELECT 2 NOT BETWEEN 1 AND 3 AS RESULT ;

-- 来源: 1131_file_1131
SELECT 2 < 1 OR 2 > 3 AS RESULT ;

-- 来源: 1131_file_1131
SELECT 2 + 2 IS NULL AS RESULT ;

-- 来源: 1131_file_1131
SELECT 2 + 2 IS NOT NULL AS RESULT ;

-- 来源: 1131_file_1131
SELECT 2 + 2 ISNULL AS RESULT ;

-- 来源: 1131_file_1131
SELECT 2 + 2 NOTNULL AS RESULT ;

-- 来源: 1131_file_1131
SELECT 2 + 2 IS DISTINCT FROM NULL AS RESULT ;

-- 来源: 1131_file_1131
SELECT 2 + 2 IS NOT DISTINCT FROM NULL AS RESULT ;

-- 来源: 1131_file_1131
select 1 <=> 1 AS RESULT ;

-- 来源: 1131_file_1131
select NULL <=> 1 AS RESULT ;

-- 来源: 1131_file_1131
select NULL <=> NULL AS RESULT ;

-- 来源: 1132_file_1132
SELECT * FROM tpcds . case_when_t1 ;

-- 来源: 1132_file_1132
SELECT CW_COL1 , CASE WHEN CW_COL1 = 1 THEN 'one' WHEN CW_COL1 = 2 THEN 'two' ELSE 'other' END FROM tpcds . case_when_t1 ORDER BY 1 ;

-- 来源: 1132_file_1132
SELECT DECODE ( 'A' , 'A' , 1 , 'B' , 2 , 0 );

-- 来源: 1132_file_1132
SELECT description , short_description , last_value , COALESCE ( description , short_description , last_value ) FROM tpcds . c_tabl ORDER BY 1 , 2 , 3 , 4 ;

-- 来源: 1132_file_1132
SELECT COALESCE ( NULL , 'Hello World' );

-- 来源: 1132_file_1132
SELECT NI_VALUE1 , NI_VALUE2 , NULLIF ( NI_VALUE1 , NI_VALUE2 ) FROM tpcds . null_if_t1 ORDER BY 1 , 2 , 3 ;

-- 来源: 1132_file_1132
SELECT NULLIF ( 'Hello' , 'Hello World' );

-- 来源: 1132_file_1132
SELECT greatest ( 9000 , 155555 , 2 . 01 );

-- 来源: 1132_file_1132
SELECT least ( 9000 , 2 );

-- 来源: 1132_file_1132
SELECT nvl ( null , 1 );

-- 来源: 1132_file_1132
SELECT nvl ( 'Hello World' , 1 );

-- 来源: 1133_file_1133
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE EXISTS ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom = store_returns . sr_reason_sk and sr_customer_sk < 10 );

-- 来源: 1133_file_1133
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk IN ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom < 10 );

-- 来源: 1133_file_1133
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk < ANY ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom < 10 );

-- 来源: 1133_file_1133
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk < all ( SELECT d_dom FROMOM tpcds . date_dim WHERE d_dom < 10 );

-- 来源: 1134_file_1134
SELECT 8000 + 500 IN ( 10000 , 9000 ) AS RESULT ;

-- 来源: 1134_file_1134
SELECT 8000 + 500 NOT IN ( 10000 , 9000 ) AS RESULT ;

-- 来源: 1134_file_1134
SELECT 8000 + 500 < SOME ( array [ 10000 , 9000 ]) AS RESULT ;

-- 来源: 1134_file_1134
SELECT 8000 + 500 < ANY ( array [ 10000 , 9000 ]) AS RESULT ;

-- 来源: 1134_file_1134
SELECT 8000 + 500 < ALL ( array [ 10000 , 9000 ]) AS RESULT ;

-- 来源: 1135_file_1135
SELECT ROW ( 1 , 2 , NULL ) < ROW ( 1 , 3 , 0 ) AS RESULT ;

-- 来源: 1135_file_1135
select ( 4 , 5 , 6 ) > ( 3 , 2 , 1 ) as result ;

-- 来源: 1135_file_1135
select ( 4 , 1 , 1 ) > ( 3 , 2 , 1 ) as result ;

-- 来源: 1135_file_1135
select ( 'test' , 'data' ) > ( 'data' , 'data' ) as result ;

-- 来源: 1135_file_1135
select ( 4 , 1 , 1 ) > ( 3 , 2 , null ) as result ;

-- 来源: 1135_file_1135
select ( null , 1 , 1 ) > ( 3 , 2 , 1 ) as result ;

-- 来源: 1135_file_1135
select ( null , 5 , 6 ) > ( null , 5 , 6 ) as result ;

-- 来源: 1135_file_1135
select ( 4 , 5 , 6 ) > ( 4 , 5 , 6 ) as result ;

-- 来源: 1135_file_1135
select ( 2 , 2 , 5 ) >= ( 2 , 2 , 3 ) as result ;

-- 来源: 1135_file_1135
select ( 2 , 2 , 1 ) <= ( 2 , 2 , 3 ) as result ;

-- 来源: 1135_file_1135
select ( 1 , 2 , 3 ) = ( 1 , 2 , 3 ) as result ;

-- 来源: 1135_file_1135
select ( 1 , 2 , 3 ) <> ( 2 , 2 , 3 ) as result ;

-- 来源: 1135_file_1135
select ( 2 , 2 , 3 ) <> ( 2 , 2 , null ) as result ;

-- 来源: 1135_file_1135
select ( null , 5 , 6 ) <> ( null , 5 , 6 ) as result ;

-- 来源: 1136_file_1136
SELECT DATE_ADD ( '2018-05-01' , INTERVAL 1 DAY );

-- 来源: 1136_file_1136
SELECT DATE_SUB ( '2018-05-01' , INTERVAL 1 YEAR );

-- 来源: 1136_file_1136
SELECT DATE '2023-01-10' - INTERVAL 1 DAY ;

-- 来源: 1136_file_1136
SELECT DATE '2023-01-10' + INTERVAL 1 MONTH ;

-- 来源: 1137_file_1137
SELECT * FROM Students WHERE rownum <= 10 ;

-- 来源: 1137_file_1137
SELECT * FROM Students WHERE rownum < 5 order by 1 ;

-- 来源: 1137_file_1137
SELECT rownum , * FROM ( SELECT * FROM Students order by 1 ) WHERE rownum <= 2 ;

-- 来源: 1137_file_1137
SELECT * FROM Students WHERE rownum > 1 ;

-- 来源: 1137_file_1137
SELECT * FROM Students ;

-- 来源: 1137_file_1137
SELECT * FROM Students ;

-- 来源: 1137_file_1137
SELECT rownum , * FROM test ;

-- 来源: 1137_file_1137
SELECT rownum , * FROM test ;

-- 来源: 1139_file_1139
SELECT text 'Origin' AS "label" , point '(0,0)' AS "value" ;

-- 来源: 1140_file_1140
SELECT 40 ! AS "40 factorial" ;

-- 来源: 1140_file_1140
SELECT CAST ( 40 AS bigint ) ! AS "40 factorial" ;

-- 来源: 1140_file_1140
SELECT text 'abc' || 'def' AS "text and unknown" ;

-- 来源: 1140_file_1140
SELECT 'abc' || 'def' AS "unspecified" ;

-- 来源: 1140_file_1140
SELECT @ '-4.5' AS "abs" ;

-- 来源: 1140_file_1140
SELECT array [ 1 , 2 ] <@ '{1,2,3}' as "is subset" ;

-- 来源: 1141_file_1141
SELECT round ( 4 , 4 );

-- 来源: 1141_file_1141
SELECT round ( CAST ( 4 AS numeric ), 4 );

-- 来源: 1141_file_1141
SELECT round ( 4 . 0 , 4 );

-- 来源: 1141_file_1141
SELECT substr ( '1234' , 3 );

-- 来源: 1141_file_1141
SELECT substr ( varchar '1234' , 3 );

-- 来源: 1141_file_1141
SELECT substr ( CAST ( varchar '1234' AS text ), 3 );

-- 来源: 1141_file_1141
SELECT substr ( 1234 , 3 );

-- 来源: 1141_file_1141
SELECT substr ( CAST ( 1234 AS text ), 3 );

-- 来源: 1142_file_1142
SELECT VS_COL1 , octet_length ( VS_COL1 ) FROM tpcds . value_storage_t1 ;

-- 来源: 1143_UNIONCASE
SELECT text 'a' AS "text" UNION SELECT 'b' ;

-- 来源: 1143_UNIONCASE
SELECT 1 . 2 AS "numeric" UNION SELECT 1 ;

-- 来源: 1143_UNIONCASE
SELECT 1 AS "real" UNION SELECT CAST ( '2.2' AS REAL );

-- 来源: 1147_file_1147
SELECT d_dow || '-' || d_dom || '-' || d_fy_week_seq AS identify_serials FROM tpcds . date_dim WHERE d_fy_week_seq = 1 ;

-- 来源: 1148_file_1148
SELECT 'a fat cat sat on a mat and ate a fat rat' :: tsvector @@ 'cat & rat' :: tsquery AS RESULT ;

-- 来源: 1148_file_1148
SELECT 'fat & cow' :: tsquery @@ 'a fat cat sat on a mat and ate a fat rat' :: tsvector AS RESULT ;

-- 来源: 1148_file_1148
SELECT to_tsvector ( 'fat cats ate fat rats' ) @@ to_tsquery ( 'fat & rat' ) AS RESULT ;

-- 来源: 1148_file_1148
SELECT 'fat cats ate fat rats' :: tsvector @@ to_tsquery ( 'fat & rat' ) AS RESULT ;

-- 来源: 1151_file_1151
SELECT id , body , title FROM tsearch . pgweb WHERE to_tsvector ( 'english' , body ) @@ to_tsquery ( 'english' , 'america' );

-- 来源: 1151_file_1151
SELECT id , body , title FROM tsearch . pgweb WHERE to_tsvector ( body ) @@ to_tsquery ( 'america' );

-- 来源: 1151_file_1151
SELECT title FROM tsearch . pgweb WHERE to_tsvector ( title || ' ' || body ) @@ to_tsquery ( 'north & america' ) ORDER BY last_mod_date DESC LIMIT 10 ;

-- 来源: 1152_file_1152
SELECT title FROM tsearch . pgweb WHERE textsearchable_index_col @@ to_tsquery ( 'north & america' ) ORDER BY last_mod_date DESC LIMIT 10 ;

-- 来源: 1153_file_1153
select c_varchar , to_tsvector ( c_varchar ) from table1 where to_tsvector ( c_text ) @@ plainto_tsquery ( '￥#@……&**' ) and to_tsvector ( c_text ) @@ plainto_tsquery ( '某公司 ' ) and c_varchar is not null order by 1 desc limit 3 ;

-- 来源: 1155_file_1155
SELECT to_tsvector ( 'english' , 'a fat cat sat on a mat - it ate a fat rats' );

-- 来源: 1156_file_1156
SELECT to_tsquery ( 'english' , 'The & Fat & Rats' );

-- 来源: 1156_file_1156
SELECT to_tsquery ( 'english' , 'Fat | Rats:AB' );

-- 来源: 1156_file_1156
SELECT to_tsquery ( 'supern:*A & star:A*B' );

-- 来源: 1156_file_1156
SELECT plainto_tsquery ( 'english' , 'The Fat Rats' );

-- 来源: 1156_file_1156
SELECT plainto_tsquery ( 'english' , 'The Fat & Rats:C' );

-- 来源: 1157_file_1157
SELECT id , title , ts_rank_cd ( to_tsvector ( body ), query ) AS rank FROM tsearch . pgweb , to_tsquery ( 'america' ) query WHERE query @@ to_tsvector ( body ) ORDER BY rank DESC LIMIT 10 ;

-- 来源: 1157_file_1157
SELECT id , title , ts_rank_cd ( to_tsvector ( body ), query , 32 /* rank/(rank+1) */ ) AS rank FROM tsearch . pgweb , to_tsquery ( 'america' ) query WHERE query @@ to_tsvector ( body ) ORDER BY rank DESC LIMIT 10 ;

-- 来源: 1157_file_1157
SELECT id , body , ts_rank_cd ( to_tsvector ( 'ngram' , body ), query ) AS rank FROM tsearch . ts_ngram , to_tsquery ( '中文' ) query WHERE query @@ to_tsvector ( body );

-- 来源: 1157_file_1157
SELECT id , body , ts_rank_cd ( to_tsvector ( 'ngram' , body ), query ) AS rank FROM tsearch . ts_ngram , to_tsquery ( '中文' ) query WHERE query @@ to_tsvector ( 'ngram' , body );

-- 来源: 1158_file_1158
SELECT ts_headline ( 'english' , 'The most common type of search is to find all documents containing given query terms and return them in order of their similarity to the query.' , to_tsquery ( 'english' , 'query & similarity' ));

-- 来源: 1158_file_1158
SELECT ts_headline ( 'english' , 'The most common type of search is to find all documents containing given query terms and return them in order of their similarity to the query.' , to_tsquery ( 'english' , 'query & similarity' ), 'StartSel = <, StopSel = >' );

-- 来源: 1161_file_1161
SELECT numnode ( plainto_tsquery ( 'the any' ));

-- 来源: 1161_file_1161
SELECT numnode(' foo & bar ' :: tsquery );

-- 来源: 1161_file_1161
SELECT querytree ( to_tsquery ( '!defined' ));

-- 来源: 1162_file_1162
SELECT ts_rewrite ( 'a & b' :: tsquery , 'a' :: tsquery , 'c' :: tsquery );

-- 来源: 1162_file_1162
SELECT ts_rewrite ( to_tsquery ( 'supernovae & crab' ), 'SELECT t, s FROM tsearch.aliases' );

-- 来源: 1162_file_1162
SELECT ts_rewrite ( to_tsquery ( 'supernovae & crab' ), 'SELECT t, s FROM tsearch.aliases' );

-- 来源: 1162_file_1162
SELECT ts_rewrite ( 'a & b' :: tsquery , 'SELECT t,s FROM tsearch.aliases WHERE ''a & b''::tsquery @> t' );

-- 来源: 1163_file_1163
SELECT * FROM ts_stat ( 'SELECT to_tsvector(''english'', sr_reason_sk) FROM tpcds.store_returns WHERE sr_customer_sk < 10' ) ORDER BY nentry DESC , ndoc DESC , word LIMIT 10 ;

-- 来源: 1163_file_1163
SELECT * FROM ts_stat ( 'SELECT to_tsvector(''english'', sr_reason_sk) FROM tpcds.store_returns WHERE sr_customer_sk < 10' , 'a' ) ORDER BY nentry DESC , ndoc DESC , word LIMIT 10 ;

-- 来源: 1164_file_1164
SELECT alias , description , token FROM ts_debug ( 'english' , 'foo-bar-beta1' );

-- 来源: 1164_file_1164
SELECT alias , description , token FROM ts_debug ( 'english' , 'http://example.com/stuff/index.html' );

-- 来源: 1167_file_1167
SELECT to_tsvector ( 'english' , 'in the list of stop words' );

-- 来源: 1167_file_1167
SELECT ts_rank_cd ( to_tsvector ( 'english' , 'in the list of stop words' ), to_tsquery ( 'list & stop' ));

-- 来源: 1167_file_1167
SELECT ts_rank_cd ( to_tsvector ( 'english' , 'list stop words' ), to_tsquery ( 'list & stop' ));

-- 来源: 1168_Simple
SELECT ts_lexize ( 'public.simple_dict' , 'YeS' );

-- 来源: 1168_Simple
SELECT ts_lexize ( 'public.simple_dict' , 'The' );

-- 来源: 1168_Simple
SELECT ts_lexize ( 'public.simple_dict' , 'YeS' );

-- 来源: 1168_Simple
SELECT ts_lexize ( 'public.simple_dict' , 'The' );

-- 来源: 1169_Synonym
SELECT * FROM ts_debug ( 'english' , 'Paris' );

-- 来源: 1169_Synonym
SELECT * FROM ts_debug ( 'english' , 'Paris' );

-- 来源: 1169_Synonym
SELECT * FROM ts_debug ( 'english' , 'paris' );

-- 来源: 1169_Synonym
SELECT * FROM ts_debug ( 'english' , 'Paris' );

-- 来源: 1169_Synonym
SELECT * FROM ts_debug ( 'english' , 'paris' );

-- 来源: 1169_Synonym
SELECT ts_lexize ( 'syn' , 'indices' );

-- 来源: 1169_Synonym
SELECT to_tsvector ( 'tst' , 'indices' );

-- 来源: 1169_Synonym
SELECT to_tsquery ( 'tst' , 'indices' );

-- 来源: 1169_Synonym
SELECT 'indexes are very useful' :: tsvector ;

-- 来源: 1169_Synonym
SELECT 'indexes are very useful' :: tsvector @@ to_tsquery ( 'tst' , 'indices' );

-- 来源: 1170_Thesaurus
SELECT plainto_tsquery ( 'russian' , 'supernova star' );

-- 来源: 1170_Thesaurus
SELECT to_tsvector ( 'russian' , 'supernova star' );

-- 来源: 1170_Thesaurus
SELECT to_tsquery ( 'russian' , '''supernova star''' );

-- 来源: 1170_Thesaurus
SELECT plainto_tsquery ( 'russian' , 'supernova star' );

-- 来源: 1171_Ispell
SELECT ts_lexize ( 'norwegian_ispell' , 'sjokoladefabrikk' );

-- 来源: 1173_file_1173
SELECT * FROM ts_debug ( 'ts_conf' , ' GaussDB, the highly scalable, SQL compliant, open source object-relational database management system, is now undergoing beta testing of the next version of our software. ' );

-- 来源: 1175_file_1175
SELECT * FROM ts_debug ( 'english' , 'a fat cat sat on a mat - it ate a fat rats' );

-- 来源: 1176_age
SELECT * FROM ts_parse ( 'default' , '123 - a number' );

-- 来源: 1176_age
SELECT * FROM ts_token_type ( 'default' );

-- 来源: 1177_file_1177
SELECT ts_lexize ( 'english_stem' , 'stars' );

-- 来源: 1177_file_1177
SELECT ts_lexize ( 'english_stem' , 'a' );

-- 来源: 1184_ABORT
SELECT * FROM customer_demographics_t1 WHERE cd_demo_sk = 1920801 ;

--查看test_db1信息。
-- 来源: 1188_ALTER DATABASE
SELECT datname,datconnlimit FROM pg_database WHERE datname = 'test_db1';

--查看test_db1信息。
-- 来源: 1188_ALTER DATABASE
SELECT t1.datname, t2.usename FROM pg_database t1, pg_user t2 WHERE t1.datname='test_db1' AND t1.datdba=t2.usesysid;

--查看test_db1信息。
-- 来源: 1188_ALTER DATABASE
SELECT t1.datname AS database, t2.spcname AS tablespace FROM pg_database t1, pg_tablespace t2 WHERE t1.dattablespace = t2.oid AND t1.datname = 'test_db1';

-- 来源: 1188_ALTER DATABASE
SELECT tablename FROM pg_tables WHERE tablename = 'test_tbl1';

--由于隔离属性的原因，该查询只能查出0条数据。
-- 来源: 1188_ALTER DATABASE
SELECT tablename FROM pg_tables WHERE tablename = 'test_tbl1';

-- 来源: 1195_ALTER GLOBAL CONFIGURATION
SELECT * FROM gs_global_config ;

-- 来源: 1195_ALTER GLOBAL CONFIGURATION
SELECT * FROM gs_global_config ;

-- 来源: 1195_ALTER GLOBAL CONFIGURATION
SELECT * FROM gs_global_config ;

--查询test1表上的索引信息。
-- 来源: 1197_ALTER INDEX
SELECT tablename,indexname,tablespace FROM pg_indexes WHERE tablename = 'test1';

--查询test1表上的索引信息。
-- 来源: 1197_ALTER INDEX
SELECT tablename,indexname,tablespace FROM pg_indexes WHERE tablename = 'test1';

--查看索引idx_test1_col1的可用性。
-- 来源: 1197_ALTER INDEX
SELECT indisusable FROM pg_index WHERE indexrelid = 'idx_test1_col1'::regclass;

--查看索引idx_test1_col1的可用性。
-- 来源: 1197_ALTER INDEX
SELECT indisusable FROM pg_index WHERE indexrelid = 'idx_test1_col1'::regclass;

--查询索引idx_test2_col1分区的名称。
-- 来源: 1197_ALTER INDEX
SELECT relname FROM pg_partition WHERE parentid = 'idx_test2_col1'::regclass;

--查询索引idx_test2_col1分区的所属表空间。
-- 来源: 1197_ALTER INDEX
SELECT t1.relname index_name, t2.spcname tablespace_name FROM pg_partition t1, pg_tablespace t2 WHERE t1.parentid = 'idx_test2_col1'::regclass AND t1.reltablespace = t2.oid;

-- 来源: 1214_ALTER SYSTEM KILL SESSION
SELECT sid , serial # , username FROM dv_sessions WHERE sid IN ( SELECT pid FROM pg_stat_activity );

--查询表信息。
-- 来源: 1215_ALTER TABLE
SELECT schemaname,tablename FROM pg_tables WHERE tablename = 'test_alt1';

-- 查看
-- 来源: 1215_ALTER TABLE
SELECT tablename, schemaname, tableowner FROM pg_tables WHERE tablename = 'test_alt1';

-- 查看。
-- 来源: 1215_ALTER TABLE
SELECT tablename, tablespace FROM pg_tables WHERE tablename = 'test_alt1';

-- 来源: 1218_ALTER TEXT SEARCH CONFIGURATION
SELECT b . cfgname , a . maptokentype , a . mapseqno , a . mapdict , c . dictname FROM pg_ts_config_map a , pg_ts_config b , pg_ts_dict c WHERE a . mapcfg = b . oid AND a . mapdict = c . oid AND b . cfgname = 'english_1' ORDER BY 1 , 2 , 3 , 4 , 5 ;

-- 来源: 1218_ALTER TEXT SEARCH CONFIGURATION
SELECT b . cfgname , a . maptokentype , a . mapseqno , a . mapdict , c . dictname FROM pg_ts_config_map a , pg_ts_config b , pg_ts_dict c WHERE a . mapcfg = b . oid AND a . mapdict = c . oid AND b . cfgname = 'english_1' ORDER BY 1 , 2 , 3 , 4 , 5 ;

-- 来源: 1233_CLUSTER
SELECT * FROM test_c1 ;

-- 来源: 1233_CLUSTER
SELECT * FROM test_c1 ;

-- 查看
-- 来源: 1233_CLUSTER
SELECT * FROM test_c2;

-- 查看
-- 来源: 1233_CLUSTER
SELECT * FROM test_c2;

-- 来源: 1235_COMMIT _ END
SELECT * FROM tpcds . customer_demographics_t2 ;

--查看数据库testdb1信息。
-- 来源: 1242_CREATE DATABASE
SELECT datname,pg_encoding_to_char(encoding) FROM pg_database WHERE datname = 'testdb1';

--查看testdb2信息。
-- 来源: 1242_CREATE DATABASE
SELECT t1.datname,t2.usename,t1.datcompatibility FROM pg_database t1,pg_user t2 WHERE t1.datname = 'testdb2' AND t1.datdba=t2.usesysid;

--查看testdb3信息。
-- 来源: 1242_CREATE DATABASE
SELECT datname,datcompatibility,dattimezone FROM pg_database WHERE datname = 'testdb3';

-- 来源: 1248_CREATE FUNCTION
SELECT * FROM func_dup_sql ( 42 );

--查询索引idx_test1信息。
-- 来源: 1252_CREATE INDEX
SELECT indexname,tablename,tablespace FROM pg_indexes WHERE indexname = 'idx_test1';

--查看索引分区信息，LOCAL索引分区数和表的分区数一致。
-- 来源: 1252_CREATE INDEX
SELECT relname FROM pg_partition WHERE parentid = 'idx_student1'::regclass;

--查看索引分区信息，GLOBAL索引分区数和表的分区数不一致。
-- 来源: 1252_CREATE INDEX
SELECT relname FROM pg_partition WHERE parentid = 'idx_student2'::regclass;

-- 来源: 1254_CREATE MASKING POLICY
SELECT * FROM tb_for_masking ;

-- 来源: 1254_CREATE MASKING POLICY
SELECT col8 FROM tb_for_masking ;

-- 来源: 1254_CREATE MASKING POLICY
SELECT col8 FROM tb_for_masking ;

-- 查询集群DN初始状态。
-- 来源: 1257_CREATE NODE
SELECT node_name, nodeis_preferred FROM pgxc_node WHERE node_type = 'D' ORDER BY 1;

-- 查询集群DN变更后状态。
-- 来源: 1257_CREATE NODE
SELECT node_name, nodeis_preferred FROM pgxc_node WHERE node_type = 'D' ORDER BY 1;

-- 来源: 1258_CREATE NODE GROUP
SELECT node_name, nodeis_preferred FROM pgxc_node WHERE node_type = 'D' ORDER BY 1;

-- 查询node group。
-- 来源: 1258_CREATE NODE GROUP
SELECT group_name, group_members FROM pgxc_group;

-- 来源: 1259_CREATE PACKAGE
SELECT emp_bonus.testpro1(1);

-- 来源: 1260_CREATE PROCEDURE
SELECT prc_add ( 2 , 3 );

-- 来源: 1260_CREATE PROCEDURE
SELECT pro_variadic ( var1 => 'hello' , VARIADIC var4 => array [ 1 , 2 , 3 , 4 ]);

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
SELECT * FROM all_data ;

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
SELECT * FROM all_data ;

-- 来源: 1267_CREATE SEQUENCE
SELECT nextval ( 'seq1' );

-- 来源: 1267_CREATE SEQUENCE
SELECT nextval ( 'seq1' );

-- 来源: 1267_CREATE SEQUENCE
SELECT * FROM test1 ;

-- 来源: 1269_CREATE SYNONYM
SELECT * FROM t1 ;

-- 来源: 1269_CREATE SYNONYM
SELECT * FROM v1 ;

-- 来源: 1269_CREATE SYNONYM
SELECT add ( 1 , 2 );

-- 来源: 1269_CREATE SYNONYM
SELECT add ( 1 . 2 , 2 . 3 );

-- 来源: 1270_CREATE TABLE
SELECT a.count,b.node_name FROM (SELECT count(*) AS count,xc_node_id FROM tablename GROUP BY xc_node_id) a, pgxc_node b WHERE a.xc_node_id=b.node_id ORDER BY a.count DESC;

-- 来源: 1270_CREATE TABLE
SELECT node_name FROM pgxc_node ;

-- 来源: 1270_CREATE TABLE
SELECT node_name , node_type , node_id FROM pgxc_node ;

-- 来源: 1270_CREATE TABLE
SELECT xc_node_id , * FROM lrt_range ;

-- 来源: 1270_CREATE TABLE
SELECT node_name , node_type , node_id FROM pgxc_node ;

-- 来源: 1270_CREATE TABLE
SELECT xc_node_id , * FROM t_news ;

-- 来源: 1270_CREATE TABLE
SELECT xc_node_id , * FROM t_news ;

-- 来源: 1270_CREATE TABLE
SELECT node_name , node_type , node_id FROM pgxc_node ;

-- 来源: 1270_CREATE TABLE
SELECT xc_node_id , * FROM t_ran1 ;

-- 查询表中col1<100的数据。
-- 来源: 1272_CREATE TABLE AS
SELECT * FROM test1 WHERE col1 < 100;

-- 来源: 1273_CREATE TABLE PARTITION
SELECT count ( * ) FROM tpcds . web_returns_p1 PARTITION ( P10 );

-- 来源: 1273_CREATE TABLE PARTITION
SELECT COUNT ( * ) FROM tpcds . web_returns_p1 PARTITION FOR ( 2450815 );

-- 来源: 1273_CREATE TABLE PARTITION
SELECT relname , boundaries , spcname FROM pg_partition p JOIN pg_tablespace t ON p . reltablespace = t . oid and p . parentid = 'tpcds.startend_pt' :: regclass ORDER BY 1 ;

-- 来源: 1273_CREATE TABLE PARTITION
SELECT COUNT ( * ) FROM tpcds . startend_pt PARTITION FOR ( 0 );

-- 来源: 1273_CREATE TABLE PARTITION
SELECT COUNT ( * ) FROM tpcds . startend_pt PARTITION ( p3 );

-- 来源: 1273_CREATE TABLE PARTITION
SELECT relname , boundaries , spcname FROM pg_partition p JOIN pg_tablespace t ON p . reltablespace = t . oid and p . parentid = 'tpcds.startend_pt' :: regclass ORDER BY 1 ;

-- 来源: 1273_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- 来源: 1273_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- 来源: 1273_CREATE TABLE PARTITION
SELECT * FROM test_list partition ( p1 );

-- 来源: 1273_CREATE TABLE PARTITION
SELECT * FROM test_list partition ( p1 );

-- 来源: 1273_CREATE TABLE PARTITION
SELECT * FROM t1 ;

-- 来源: 1273_CREATE TABLE PARTITION
SELECT * FROM test_list partition ( p2 );

-- 来源: 1273_CREATE TABLE PARTITION
SELECT * FROM test_list partition ( p2 );

-- 来源: 1273_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- 来源: 1273_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- 来源: 1273_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- 来源: 1273_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_hash' AND t1 . parttype = 'p' ;

-- 来源: 1273_CREATE TABLE PARTITION
select * from test_hash partition ( p1 );

-- 来源: 1273_CREATE TABLE PARTITION
select * from test_hash partition ( p2 );

-- 来源: 1273_CREATE TABLE PARTITION
select * from test_hash partition ( p1 );

-- 来源: 1273_CREATE TABLE PARTITION
select * from t1 ;

-- 来源: 1273_CREATE TABLE PARTITION
select * from test_hash partition ( p2 );

-- 来源: 1276_CREATE TRIGGER
SELECT * FROM test_trigger_src_tbl ;

-- 来源: 1276_CREATE TRIGGER
SELECT * FROM test_trigger_des_tbl ;

-- 来源: 1276_CREATE TRIGGER
SELECT * FROM test_trigger_src_tbl ;

-- 来源: 1276_CREATE TRIGGER
SELECT * FROM test_trigger_des_tbl ;

-- 来源: 1276_CREATE TRIGGER
SELECT * FROM test_trigger_src_tbl ;

-- 来源: 1276_CREATE TRIGGER
SELECT * FROM test_trigger_des_tbl ;

-- 来源: 1277_CREATE TYPE
SELECT ( b ). f1 FROM t1_compfoo ;

-- 来源: 1277_CREATE TYPE
SELECT * FROM t1_compfoo t1 join t2_compfoo t2 on ( t1 . b ). f1 = ( t1 . b ). f1 ;

--查看视图。
-- 来源: 1279_CREATE VIEW
SELECT * FROM test_v1;

--查看现有弱口令。
-- 来源: 1281_CREATE WEAK PASSWORD DICTIONARY
SELECT * FROM gs_global_config WHERE NAME LIKE 'weak_password';

-- 来源: 1284_DEALLOCATE
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- 来源: 1284_DEALLOCATE
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- 来源: 1284_DEALLOCATE
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- 来源: 1284_DEALLOCATE
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- 来源: 1329_EXECUTE DIRECT
SELECT * FROM pgxc_node ;

-- 来源: 1329_EXECUTE DIRECT
SELECT COUNT ( * ) FROM tpcds . customer_address ;

-- 来源: 1329_EXECUTE DIRECT
SELECT oid FROM pgxc_node where node_name = 'dn_6001_6002_6003' ;

-- 来源: 1333_EXPLAIN PLAN
SELECT * FROM plan_table;

-- 来源: 1333_EXPLAIN PLAN
SELECT * FROM plan_table ;

-- 来源: 1333_EXPLAIN PLAN
SELECT * FROM plan_table ;

-- 来源: 1349_MERGE INTO
SELECT * FROM products ORDER BY product_id ;

--查看回收站。
-- 来源: 1355_PURGE
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

-- 来源: 1355_PURGE
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

-- 来源: 1355_PURGE
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

-- 来源: 1355_PURGE
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

-- 来源: 1361_RELEASE SAVEPOINT
SELECT * FROM tpcds . table1 ;

--查询值替换插入的结果
-- 来源: 1362_REPLACE
SELECT * FROM test WHERE f1 = 1;

-- 来源: 1362_REPLACE
SELECT * FROM test WHERE f1 = 2;

-- 来源: 1362_REPLACE
SELECT * FROM test WHERE f1 = 3;

-- 来源: 1369_SAVEPOINT
SELECT * FROM table1 ;

-- 来源: 1369_SAVEPOINT
SELECT * FROM table2 ;

-- 来源: 1371_SELECT
SELECT * FROM XMLTABLE( XMLNAMESPACES('nspace1' AS "ns1", 'nspace2' AS "ns2"), -- 声明两个XML的命名空间'nspace1'和'nspace2'及对应的别名"ns1"和"ns2" '/ns1:root/*:child' -- 经row_expression从传入的数据中选取命名空间为'nspace1'的root节点，在选取其下面的所有child节点，忽略child的命名空间；其中ns1为'nspace1'的别名 PASSING xmltype( '<root xmlns="nspace1"> <child> <name>peter</name> <age>11</age> </child> <child xmlns="nspace1"> <name>qiqi</name> <age>12</age> </child> <child xmlns="nspace2"> <name>hacker</name> <age>15</age> </child> </root>') COLUMNS column FOR ORDINALITY, -- 该列为行号列 name varchar(10) path 'ns1:name', -- 从row_expression获取的每个child节点中选取命名空间为'nspace1'的name节点，并将节点中的值转换为varchar(10)返回；其中ns1为'nspace1'的别名 age int);

-- 来源: 1371_SELECT
WITH temp_t ( name , isdba ) AS ( SELECT usename , usesuper FROM pg_user ) SELECT * FROM temp_t ;

-- 来源: 1371_SELECT
SELECT DISTINCT ( r_reason_sk ) FROM tpcds . reason ;

-- 来源: 1371_SELECT
SELECT * FROM tpcds . reason LIMIT 1 ;

-- 来源: 1371_SELECT
SELECT r_reason_desc FROM tpcds . reason ORDER BY r_reason_desc ;

-- 来源: 1371_SELECT
SELECT a . usename , b . locktime FROM pg_user a , pg_user_status b WHERE a . usesysid = b . roloid ;

-- 来源: 1371_SELECT
SELECT a . usename , b . locktime , a . usesuper FROM pg_user a FULL JOIN pg_user_status b on a . usesysid = b . roloid ;

-- 来源: 1371_SELECT
SELECT r_reason_id , AVG ( r_reason_sk ) FROM tpcds . reason GROUP BY r_reason_id HAVING AVG ( r_reason_sk ) > 25 ;

-- 来源: 1371_SELECT
SELECT r_reason_id , AVG ( r_reason_sk ) FROM tpcds . reason GROUP BY CUBE ( r_reason_id , r_reason_sk );

-- 来源: 1371_SELECT
SELECT r_reason_id , AVG ( r_reason_sk ) FROM tpcds . reason GROUP BY GROUPING SETS (( r_reason_id , r_reason_sk ), r_reason_sk );

-- 来源: 1371_SELECT
SELECT r_reason_sk , tpcds . reason . r_reason_desc FROM tpcds . reason WHERE tpcds . reason . r_reason_desc LIKE 'W%' UNION SELECT r_reason_sk , tpcds . reason . r_reason_desc FROM tpcds . reason WHERE tpcds . reason . r_reason_desc LIKE 'N%' ;

-- 来源: 1371_SELECT
SELECT * FROM tpcds . reason ORDER BY NLSSORT ( r_reason_desc , 'NLS_SORT = SCHINESE_PINYIN_M' );

-- 来源: 1371_SELECT
SELECT * FROM tpcds . reason ORDER BY NLSSORT ( r_reason_desc , 'NLS_SORT = generic_m_ci' );

-- 来源: 1371_SELECT
SELECT * FROM tpcds . reason_p PARTITION ( P_05_BEFORE );

-- 来源: 1371_SELECT
SELECT * FROM tpcds . reason_p PARTITION ( P_05_BEFORE , P_15 , P_25 ) ORDER BY 1 ;

-- 来源: 1371_SELECT
SELECT COUNT ( * ), r_reason_id FROM tpcds . reason_p GROUP BY r_reason_id ;

-- 来源: 1371_SELECT
SELECT * FROM tpcds . reason GROUP BY CUBE ( r_reason_id , r_reason_sk , r_reason_desc );

-- 来源: 1371_SELECT
SELECT * FROM tpcds . reason GROUP BY GROUPING SETS (( r_reason_id , r_reason_sk ), r_reason_desc );

-- 来源: 1371_SELECT
SELECT COUNT ( * ) c , r_reason_id FROM tpcds . reason_p GROUP BY r_reason_id HAVING c > 2 ;

-- 来源: 1371_SELECT
SELECT COUNT ( * ), r_reason_id FROM tpcds . reason_p GROUP BY r_reason_id HAVING r_reason_id IN ( 'AAAAAAAABAAAAAAA' , 'AAAAAAAADAAAAAAA' );

-- 来源: 1371_SELECT
SELECT * FROM tpcds . reason_p WHERE r_reason_id = 'AAAAAAAABAAAAAAA' INTERSECT SELECT * FROM tpcds . reason_p WHERE r_reason_sk < 5 ;

-- 来源: 1371_SELECT
SELECT * FROM tpcds . reason_p WHERE r_reason_id = 'AAAAAAAABAAAAAAA' EXCEPT SELECT * FROM tpcds . reason_p WHERE r_reason_sk < 4 ;

-- 来源: 1371_SELECT
SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE t1 . sr_customer_sk = t2 . c_customer_sk ( + ) ORDER BY 1 DESC LIMIT 1 ;

-- 来源: 1371_SELECT
SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE t1 . sr_customer_sk ( + ) = t2 . c_customer_sk ORDER BY 1 DESC LIMIT 1 ;

-- 来源: 1371_SELECT
SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE t1 . sr_customer_sk = t2 . c_customer_sk ( + ) AND t2 . c_customer_sk ( + ) < 1 ORDER BY 1 LIMIT 1 ;

-- 来源: 1371_SELECT
SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE NOT ( t1 . sr_customer_sk = t2 . c_customer_sk ( + ) AND t2 . c_customer_sk ( + ) < 1 );

-- 来源: 1371_SELECT
SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE ( t1 . sr_customer_sk = t2 . c_customer_sk ( + )):: bool ;

-- 来源: 1371_SELECT
SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE t1 . sr_customer_sk ( + ) = t2 . c_customer_sk ( + );

-- 来源: 1371_SELECT
WITH RECURSIVE t1(a) as ( select 100 ), t(n) AS ( VALUES (1) UNION ALL SELECT n+1 FROM t WHERE n < (select max(a) from t1) ) SELECT sum(n) FROM t;

-- 来源: 1371_SELECT
SELECT * FROM skiplocked_astore WHERE id = 1 FOR UPDATE ;

-- 来源: 1371_SELECT
SELECT * FROM skiplocked_astore FOR UPDATE SKIP LOCKED ;

-- 来源: 1372_SELECT INTO
SELECT * INTO tpcds . reason_t1 FROM tpcds . reason WHERE r_reason_sk < 5 ;

-- 来源: 1380_START TRANSACTION
SELECT * FROM tpcds . reason ;

-- 来源: 1380_START TRANSACTION
SELECT * FROM tpcds . reason ;

-- 来源: 1380_START TRANSACTION
SELECT * FROM tpcds . reason ;

--查询tpcds.reason_t2表中的数据。
-- 来源: 1382_TIMECAPSULE TABLE
SELECT * FROM tpcds.reason_t2;

-- 来源: 1382_TIMECAPSULE TABLE
SELECT * FROM tpcds.reason_t2;

--查询tbl_test1表。
-- 来源: 1385_UPDATE
SELECT * FROM tbl_test1;

--查询tbl_test1表。
-- 来源: 1385_UPDATE
SELECT * FROM tbl_test1;

--查询。
-- 来源: 1385_UPDATE
SELECT * FROM test_grade;

--2008-08-25 Ben参加了补考,成绩为B，正常步骤需要先修改last_exam为否,然后插入2008-08-25这一天的成绩。
-- 来源: 1385_UPDATE
WITH old_exa AS ( UPDATE test_grade SET last_exam = 0 WHERE sid = 2 AND examtime = '2008-07-08' RETURNING sid, name ) INSERT INTO test_grade VALUES ( ( SELECT sid FROM old_exa ), (SELECT name FROM old_exa), 'B', '2008-08-25', 1 );

--查询。
-- 来源: 1385_UPDATE
SELECT * FROM test_grade;

-- 来源: 1396_file_1396
select $$it's an example$$;

-- 来源: 1439_file_1439
SELECT * FROM sections_t1 ;

-- 来源: 1447_file_1447
SELECT * FROM TEST_t1 ;

-- 来源: 1450_file_1450
select * from mytab ;

-- 来源: 1450_file_1450
SELECT merge_db ( 1 , 'david' );

-- 来源: 1450_file_1450
SELECT merge_db ( 1 , 'dennis' );

-- 来源: 1452_file_1452
SELECT * FROM EXAMPLE1;

-- 来源: 1452_file_1452
SELECT * FROM EXAMPLE1;

-- 来源: 1468_DBE_COMPRESSION
SELECT DBE_COMPRESSION . GET_COMPRESSION_TYPE ( 'user1' , 'test_data' , '(0,1)' , NULL );

-- 来源: 1471_DBE_ILM_ADMIN
select * from gs_adm_ilmparameters ;

-- 来源: 1477_DBE_SCHEDULER
SELECT dbe_scheduler . create_job ( 'job1' , 'PLSQL_BLOCK' , 'begin insert into test1 values(12);

-- 来源: 1477_DBE_SCHEDULER
SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 1477_DBE_SCHEDULER
select DBE_SCHEDULER.create_credential('cre_1', 'test1', '*********');

-- 来源: 1477_DBE_SCHEDULER
select DBE_SCHEDULER.create_job(job_name=>'job1', job_type=>'EXTERNAL_SCRIPT', job_action=>'/usr/bin/pwd', enabled=>true, auto_drop=>false, credential_name => 'cre_1');

-- 来源: 1477_DBE_SCHEDULER
SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 1477_DBE_SCHEDULER
SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 1480_DBE_TASK
select DBE_TASK . SUBMIT ( 'call pro_xxx();

-- 来源: 1480_DBE_TASK
select DBE_TASK . SUBMIT ( 'call pro_xxx();

-- 来源: 1489_file_1489
SELECT * FROM t1;

-- 来源: 1489_file_1489
SELECT auto_func();

-- 来源: 1490_file_1490
SELECT * FROM t2;

-- 来源: 1490_file_1490
SELECT autonomous_5(11,22);

-- 来源: 1490_file_1490
SELECT * FROM t2 ORDER BY a;

-- 来源: 1491_file_1491
SELECT * FROM t1;

-- 来源: 1492_file_1492
SELECT autonomous_33(0);

-- 来源: 1492_file_1492
SELECT * FROM t4;

-- 来源: 1493_PACKAGE
SELECT * FROM t2;

-- 来源: 1493_PACKAGE
SELECT autonomous_5(11,22);

-- 来源: 1493_PACKAGE
SELECT * FROM t2 ORDER BY a;

-- 来源: 1909_PG_REPLICATION_SLOTS
SELECT * FROM pg_replication_slots;

--在CN上执行查询。
-- 来源: 1909_PG_REPLICATION_SLOTS
SELECT * FROM pg_replication_slots;

-- 来源: 1962_PGXC_THREAD_WAIT_STATUS
SELECT * FROM pg_thread_wait_status WHERE query_id > 0 ;

-- 来源: 1962_PGXC_THREAD_WAIT_STATUS
SELECT * FROM pgxc_thread_wait_status WHERE query_id > 0 ;

-- 来源: 2120_SESSION_STAT_ACTIVITY
SELECT datname, usename, usesysid,state,pid FROM pg_stat_activity;

-- 来源: 2121_GLOBAL_SESSION_STAT_ACTIVITY
SELECT datname, usename, usesysid,state,pid FROM pg_stat_activity;

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT OID FROM PG_PROC WHERE PRONAME = 'test_debug' ;

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . turn_on ( 16389 );

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . attach ( 'datanode' , 0 );

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . next ();

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . info_locals ();

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . set_var ( 'x' , 2 );

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . print_var ( 'x' );

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . continue ();

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . continue ();

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . error_end ();

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . abort ();

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . info_code ( 16389 );

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . add_breakpoint ( 16389 , 4 );

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . info_breakpoints ();

-- 来源: 2292_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . continue ();

-- 来源: 2316_file_2316
SELECT * FROM pg_settings WHERE NAME = 'server_version' ;

-- 来源: 2316_file_2316
SELECT * FROM pg_settings ;

-- 来源: 2366_file_2366
select 0 . 1231243 as a , 0 . 1231243 :: numeric as b , 0 . 1231243 :: integer ( 10 , 3 ) as c , length ( 0 . 1242343 ) as d ;

-- 来源: 2366_file_2366
select 0 . 1231243 as a , 0 . 1231243 :: numeric as b , 0 . 1231243 :: integer ( 10 , 3 ) as c , length ( 0 . 1242343 ) as d ;

-- 来源: 2366_file_2366
select add_months ( '2018-02-28' , 3 ) from sys_dummy ;

-- 来源: 2366_file_2366
select add_months ( '2018-02-28' , 3 ) from sys_dummy ;

-- 来源: 2366_file_2366
select '' AS to_number_14 , to_number ( '34,50' , '999,99' );

-- 来源: 2366_file_2366
select '' AS to_number_14 , to_number ( '34,50' , '999,99' );

-- 来源: 2366_file_2366
select ( - 2147483648 ):: int4 / ( - 1 ):: int4 ;

-- 来源: 2366_file_2366
select ( - 2147483648 ):: int4 / ( - 1 ):: int4 ;

-- 来源: 2366_file_2366
select * from test1 where c2 > 1 ;

-- 来源: 2366_file_2366
select * from test1 where c2 > 1 ;

-- 来源: 2366_file_2366
select length ( lpad ( '123' , 0 , '*' )) from sys_dummy ;

-- 来源: 2366_file_2366
select length ( lpad ( '123' , 0 , '*' )) from sys_dummy ;

-- 来源: 2366_file_2366
select concat ( variadic NULL :: int []) is NULL ;

-- 来源: 2366_file_2366
select concat ( variadic NULL :: int []) is NULL ;

-- 来源: 2366_file_2366
select concat ( variadic NULL :: int []) is NULL ;

-- 来源: 2366_file_2366
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- 来源: 2366_file_2366
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- 来源: 2366_file_2366
select * from tab_1 where col1 is null;

-- 来源: 2366_file_2366
select * from tab_1 where col1=' ';

-- 来源: 2366_file_2366
select * from tab_1 where col1 is null;

-- 来源: 2366_file_2366
select * from tab_1 where col1=' ';

-- 来源: 2366_file_2366
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- 来源: 2366_file_2366
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- 来源: 2366_file_2366
select test(1,2);

-- 来源: 2366_file_2366
select test(1,2);

-- 来源: 2366_file_2366
SELECT power(2,3);

-- 来源: 2366_file_2366
SELECT count(*) FROM db_ind_columns;

-- 来源: 2366_file_2366
SELECT count(index_name) FROM db_ind_columns;

-- 来源: 2366_file_2366
SELECT left('abcde', 2);

-- 来源: 2366_file_2366
SELECT pg_client_encoding();

-- 来源: 2366_file_2366
SELECT power(2,3);

-- 来源: 2366_file_2366
SELECT count(*) FROM db_ind_columns;

-- 来源: 2366_file_2366
SELECT count(index_name) FROM db_ind_columns;

-- 来源: 2366_file_2366
SELECT left('abcde', 2);

-- 来源: 2366_file_2366
SELECT pg_client_encoding();

-- 来源: 2366_file_2366
select sysdate;

-- 来源: 2366_file_2366
select decode(c1,'x','0','default') from test;

-- 来源: 2366_file_2366
select (case c1 when 'x' then '0' else 'default' end) from test;

-- 来源: 2366_file_2366
select timestamp '2024-03-20 01:30:00’ at time zone 'Europe/Moscow' from dual;

-- 来源: 2366_file_2366
select timestamp '2024-03-20 01:30:00’ at time zone 'Europe/Moscow' from dual;

-- 来源: 2366_file_2366
select 'a' || null || 'b';

-- 来源: 2431_file_2431
select * from a;

-- 来源: 2431_file_2431
select * from a;

-- 来源: 2431_file_2431
select * from a;

-- 来源: 2431_file_2431
select * from a;

-- 来源: 2431_file_2431
select * from a;

-- 来源: 2431_file_2431
select * from a;

-- 来源: 2431_file_2431
select * from a;

-- 来源: 2438_file_2438
SELECT * FROM pg_user ;

-- 来源: 2438_file_2438
SELECT * FROM pg_authid ;

-- 来源: 2440_Schema
SELECT s . nspname , u . usename AS nspowner FROM pg_namespace s , pg_user u WHERE nspname = 'schema_name' AND s . nspowner = u . usesysid ;

-- 来源: 2440_Schema
SELECT * FROM pg_namespace ;

-- 来源: 2440_Schema
SELECT distinct ( tablename ), schemaname from pg_tables where schemaname = 'pg_catalog' ;

-- 来源: 2442_file_2442
SELECT * FROM public . all_data ;

-- 来源: 2442_file_2442
SELECT * FROM public . all_data ;

-- 来源: 2450_file_2450
SELECT datname FROM pg_database ;

-- 来源: 2452_file_2452
SELECT spcname FROM pg_tablespace ;

-- 来源: 2452_file_2452
SELECT PG_TABLESPACE_SIZE ( 'example' );

-- 来源: 2457_file_2457
SELECT * FROM pg_tables ;

-- 来源: 2457_file_2457
SELECT count ( * ) FROM customer_t1 ;

-- 来源: 2457_file_2457
SELECT * FROM customer_t1 ;

-- 来源: 2457_file_2457
SELECT c_customer_sk FROM customer_t1 ;

-- 来源: 2457_file_2457
SELECT DISTINCT ( c_customer_sk ) FROM customer_t1 ;

-- 来源: 2457_file_2457
SELECT * FROM customer_t1 WHERE c_customer_sk = 3869 ;

-- 来源: 2457_file_2457
SELECT * FROM customer_t1 ORDER BY c_customer_sk ;

-- 来源: 2459_file_2459
SELECT distinct ( tablename ) FROM pg_tables WHERE SCHEMANAME = 'public' AND TABLENAME LIKE 'search_table%' ;

-- 来源: 2461_schema
SELECT * FROM myschema . mytable ;

-- 来源: 2461_schema
SELECT current_schema ();

-- 来源: 2462_file_2462
SELECT * FROM tpcds . web_returns_p2 PARTITION ( P6 );

-- 来源: 2462_file_2462
SELECT * FROM tpcds . web_returns_p2 PARTITION FOR ( 35888 );

-- 来源: 2462_file_2462
SELECT * FROM tpcds . web_returns_p2 PARTITION ( P6 );

-- 来源: 2462_file_2462
SELECT * FROM tpcds . web_returns_p2 PARTITION FOR ( 35888 );

-- 来源: 2463_file_2463
SELECT RELNAME FROM PG_CLASS WHERE RELKIND = 'i' or RELKIND = 'I' ;

-- 来源: 2463_file_2463
SELECT ca_address_sk FROM tpcds . customer_address_bak WHERE ca_address_sk = 14888 ;

-- 来源: 2463_file_2463
SELECT ca_address_sk , ca_address_id FROM tpcds . customer_address_bak WHERE ca_address_sk = 5050 AND ca_street_number < 1000 ;

-- 来源: 2463_file_2463
SELECT * FROM tpcds . customer_address_bak WHERE trunc ( ca_street_number ) < 1000 ;

-- 来源: 2464_file_2464
SELECT * FROM MyView ;

-- 来源: 2466_file_2466
select job , dbname , start_date , last_date , this_date , next_date , broken , status , interval , failures , what from my_jobs ;

-- 来源: 2478_SQL
SELECT CURRENT_DATE ;

-- 来源: 2478_SQL
SELECT CURRENT_TIME ;

-- 来源: 2478_SQL
SELECT CURRENT_TIMESTAMP ( 6 );

-- 来源: 2719_Hint
select * from dbe_perf.global_plancache_status where schema_name='public' order by 1,2;

-- 来源: 2731_SQL PATCH
select * from hint_t1 t1 where t1 . a = 1 ;

-- 来源: 2731_SQL PATCH
select unique_query_id , query , query_plan from dbe_perf . statement_history where query like '%hint_t1%' ;

-- 来源: 2731_SQL PATCH
select * from dbe_sql_util . create_hint_sql_patch ( 'patch1' , 2311517824 , 'indexscan(t1)' );

-- 来源: 2731_SQL PATCH
select * from hint_t1 t1 where t1 . a = 1 ;

-- 来源: 2731_SQL PATCH
select unique_query_id , query , query_plan from dbe_perf . statement_history where query like '%hint_t1%' ;

-- 来源: 2731_SQL PATCH
select * from dbe_sql_util.drop_sql_patch('patch1');

-- 来源: 2731_SQL PATCH
select * from dbe_sql_util.create_abort_sql_patch('patch2', 2311517824);

-- 来源: 2731_SQL PATCH
select * from hint_t1 t1 where t1.a = 1;

-- 来源: 2731_SQL PATCH
select b from test_proc_patch where a = 1;

-- 来源: 2731_SQL PATCH
select unique_query_id, query, query_plan, parent_unique_sql_id from dbe_perf.statement_history where query like '%call mypro();

-- 根据parentid可以调用重载函数限制存储过程内生效
-- 来源: 2731_SQL PATCH
select * from dbe_sql_util.create_hint_sql_patch('patch1',2859505004,3460545602,'indexscan(test_proc_patch)');

-- 来源: 2731_SQL PATCH
select patch_name,unique_sql_id,parent_unique_sql_id,enable,abort,hint_string from gs_sql_patch where patch_name = 'patch1';

-- 来源: 2731_SQL PATCH
select b from test_proc_patch where a = 1;

-- 来源: 2731_SQL PATCH
select unique_query_id, query, query_plan, parent_unique_sql_id from dbe_perf.statement_history where query like '%test_proc_patch%' order by start_time;

-- 来源: 2733_GUCrewrite_rule
select * from t1 order by c2;

-- 来源: 2733_GUCrewrite_rule
select * from t2 order by c2;

-- 来源: 2733_GUCrewrite_rule
select t1.c1 from t1 where t1.c1 = (select t2.c1 from t2 where t1.c1=t2.c2) ;

-- 来源: 2733_GUCrewrite_rule
select t1.c1 from t1 where t1.c1 = (select t2.c1 from t2 where t1.c1=t2.c2) ;

-- 来源: 2743_file_2743
SELECT * FROM int_type_t1 ;

-- 来源: 2743_file_2743
SELECT * FROM int_type_t2 ;

--查询表中的数据。
-- 来源: 2743_file_2743
SELECT * FROM decimal_type_t1;

-- 来源: 2743_file_2743
SELECT * FROM numeric_type_t1 ;

-- 来源: 2743_file_2743
SELECT * FROM smallserial_type_tab ;

-- 来源: 2743_file_2743
SELECT * FROM serial_type_tab ;

-- 来源: 2743_file_2743
SELECT * FROM bigserial_type_tab ;

-- 来源: 2743_file_2743
SELECT * FROM largeserial_type_tab ;

-- 来源: 2743_file_2743
SELECT * FROM float_type_t2 ;

-- 来源: 2744_file_2744
SELECT '12.34' :: float8 :: numeric :: money ;

-- 来源: 2744_file_2744
SELECT '52093.89' :: money :: numeric :: float8 ;

-- 来源: 2745_file_2745
SELECT * FROM bool_type_t1 ;

-- 来源: 2745_file_2745
SELECT * FROM bool_type_t1 WHERE bt_col1 = 't' ;

--查询表中的数据。
-- 来源: 2746_file_2746
SELECT ct_col1, char_length(ct_col1) FROM char_type_t1;

--查询数据。
-- 来源: 2746_file_2746
SELECT ct_col1, char_length(ct_col1) FROM char_type_t2;

-- 来源: 2747_file_2747
SELECT * FROM blob_type_t1 ;

-- 来源: 2748__
SELECT * FROM date_type_tab ;

-- 来源: 2748__
SELECT * FROM time_type_tab ;

-- 来源: 2748__
SELECT * FROM day_type_tab ;

-- 来源: 2748__
SELECT * FROM year_type_tab ;

-- 来源: 2748__
SELECT * FROM date_type_tab ;

-- 来源: 2748__
SELECT * FROM date_type_tab ;

-- 来源: 2748__
SELECT time '04:05:06' ;

-- 来源: 2748__
SELECT time '04:05:06 PST' ;

-- 来源: 2748__
SELECT time with time zone '04:05:06 PST' ;

--查看数据。
-- 来源: 2748__
SELECT * FROM realtime_type_special;

-- 来源: 2748__
SELECT * FROM realtime_type_special WHERE col3 < 'infinity';

-- 来源: 2748__
SELECT * FROM realtime_type_special WHERE col3 > '-infinity';

-- 来源: 2748__
SELECT * FROM realtime_type_special WHERE col3 > 'now';

-- 来源: 2748__
SELECT * FROM realtime_type_special WHERE col3 = 'today';

-- 来源: 2748__
SELECT * FROM realtime_type_special WHERE col3 = 'tomorrow';

-- 来源: 2748__
SELECT * FROM realtime_type_special WHERE col3 > 'yesterday';

-- 来源: 2748__
SELECT TIME 'allballs';

-- 来源: 2748__
SELECT * FROM reltime_type_tab ;

-- 来源: 2751_file_2751
SELECT * FROM bit_type_t1 ;

-- 来源: 2752_file_2752
SELECT 'a fat cat sat on a mat and ate a fat rat' :: tsvector ;

-- 来源: 2752_file_2752
SELECT $$ the lexeme ' ' contains spaces $$ :: tsvector ;

-- 来源: 2752_file_2752
SELECT $$ the lexeme 'Joe''s' contains a quote $$ :: tsvector ;

-- 来源: 2752_file_2752
SELECT 'a:1 fat:2 cat:3 sat:4 on:5 a:6 mat:7 and:8 ate:9 a:10 fat:11 rat:12' :: tsvector ;

-- 来源: 2752_file_2752
SELECT 'a:1A fat:2B,4C cat:5D' :: tsvector ;

-- 来源: 2752_file_2752
SELECT 'The Fat Rats' :: tsvector ;

-- 来源: 2752_file_2752
SELECT to_tsvector ( 'english' , 'The Fat Rats' );

-- 来源: 2752_file_2752
SELECT 'fat & rat' :: tsquery ;

-- 来源: 2752_file_2752
SELECT 'fat & (rat | cat)' :: tsquery ;

-- 来源: 2752_file_2752
SELECT 'fat & rat & ! cat' :: tsquery ;

-- 来源: 2752_file_2752
SELECT 'fat:ab & cat' :: tsquery ;

-- 来源: 2752_file_2752
SELECT 'super:*' :: tsquery ;

-- 来源: 2752_file_2752
SELECT to_tsvector ( 'seriousness' ) @@ to_tsquery ( 'series:*' ) AS RESULT ;

-- 来源: 2752_file_2752
SELECT to_tsquery ( 'series:*' );

-- 来源: 2752_file_2752
SELECT to_tsquery ( 'Fat:ab & Cats' );

-- 来源: 2754_JSON_JSONB
SELECT 'null'::json;

-- 来源: 2754_JSON_JSONB
SELECT 'NULL'::jsonb;

-- 来源: 2754_JSON_JSONB
SELECT '1'::json;

-- 来源: 2754_JSON_JSONB
SELECT '-1.5'::json;

-- 来源: 2754_JSON_JSONB
SELECT '-1.5e-5'::jsonb, '-1.5e+2'::jsonb;

-- 来源: 2754_JSON_JSONB
SELECT '001'::json, '+15'::json, 'NaN'::json;

-- 来源: 2754_JSON_JSONB
SELECT 'true'::json;

-- 来源: 2754_JSON_JSONB
SELECT 'false'::jsonb;

-- 来源: 2754_JSON_JSONB
SELECT '"a"'::json;

-- 来源: 2754_JSON_JSONB
SELECT '"abc"'::jsonb;

-- 来源: 2754_JSON_JSONB
SELECT '[1, 2, "foo", null]'::json;

-- 来源: 2754_JSON_JSONB
SELECT '[]'::json;

-- 来源: 2754_JSON_JSONB
SELECT '[1, 2, "foo", null, [[]], {}]'::jsonb;

-- 来源: 2754_JSON_JSONB
SELECT '{}'::json;

-- 来源: 2754_JSON_JSONB
SELECT '{"a": 1, "b": {"a": 2, "b": null}}'::json;

-- 来源: 2754_JSON_JSONB
SELECT '{"foo": [true, "bar"], "tags": {"a": 1, "b": null}}'::jsonb;

-- 来源: 2754_JSON_JSONB
SELECT ' [1, " a ", {"a" :1 }] '::jsonb;

-- 来源: 2754_JSON_JSONB
SELECT '{"a" : 1, "a" : 2}'::jsonb;

-- 来源: 2754_JSON_JSONB
SELECT '{"aa" : 1, "b" : 2, "a" : 3}'::jsonb;

-- 来源: 2754_JSON_JSONB
SELECT '"foo"'::jsonb @> '"foo"'::jsonb;

-- 来源: 2754_JSON_JSONB
SELECT '[1, "aa", 3]'::jsonb ? 'aa';

-- 来源: 2754_JSON_JSONB
SELECT '[1, 2, 3]'::jsonb @> '[1, 3, 1]'::jsonb;

-- 来源: 2754_JSON_JSONB
SELECT '{"product": "PostgreSQL", "version": 9.4, "jsonb":true}'::jsonb @> '{"version":9.4}'::jsonb;

-- 来源: 2754_JSON_JSONB
SELECT '[1, 2, [1, 3]]'::jsonb @> '[1, 3]'::jsonb;

-- 来源: 2754_JSON_JSONB
SELECT '{"foo": {"bar": "baz"}}'::jsonb @> '{"bar": "baz"}'::jsonb;

-- 来源: 2755_HLL
SELECT hll_cardinality ( set ) FROM helloworld WHERE id = 1 ;

-- 来源: 2755_HLL
select date , hll_cardinality ( users ) from daily_uniques order by date ;

-- 来源: 2755_HLL
SELECT hll_cardinality ( hll_union_agg ( users )) FROM daily_uniques WHERE date >= '2019-02-20' :: date AND date <= '2019-02-26' :: date ;

-- 来源: 2755_HLL
SELECT date , ( # hll_union_agg ( users ) OVER two_days ) - # users AS lost_uniques FROM daily_uniques WINDOW two_days AS ( ORDER BY date ASC ROWS 1 PRECEDING );

-- 包含 。
-- 来源: 2756_file_2756
SELECT int4range(10, 20) @> 3;

-- 重叠 。
-- 来源: 2756_file_2756
SELECT numrange(11.1, 22.2) && numrange(20.0, 30.0);

-- 抽取上界 。
-- 来源: 2756_file_2756
SELECT upper(int8range(15, 25));

-- 计算交集 。
-- 来源: 2756_file_2756
SELECT int4range(10, 20) * int4range(15, 25);

-- 范围 是否为空 。
-- 来源: 2756_file_2756
SELECT isempty(numrange(1, 5));

-- 来源: 2756_file_2756
SELECT '[3,7)'::int4range;

-- 既不包括 3 也不包括 7，但是包括之间的所有点 。
-- 来源: 2756_file_2756
SELECT '(3,7)'::int4range;

-- 只包括单独一个点 4 。
-- 来源: 2756_file_2756
SELECT '[4,4]'::int4range;

-- 不包括点（并且将被标准化为 '空'） 。
-- 来源: 2756_file_2756
SELECT '[4,4)'::int4range;

-- 来源: 2756_file_2756
SELECT numrange(1.0, 14.0, '(]');

-- 如果第三个参数被忽略，则假定为 '[)'。
-- 来源: 2756_file_2756
SELECT numrange(1.0, 14.0);

-- 尽管这里指定了 '(]'，显示时该值将被转换成标准形式，因为 int8range 是一种离散范围类型（见下文）。
-- 来源: 2756_file_2756
SELECT int8range(1, 14, '(]');

-- 为一个界限使用 NULL 导致范围在那一边是无界的。
-- 来源: 2756_file_2756
SELECT numrange(NULL, 2.2);

-- 来源: 2756_file_2756
SELECT '[1.234, 5.678]'::floatrange;

-- 来源: 2756_file_2756
SELECT '[11:10, 23:00]'::timerange;

-- 来源: 2757_file_2757
SELECT oid FROM pg_class WHERE relname = 'pg_type' ;

-- 来源: 2757_file_2757
SELECT attrelid , attname , atttypid , attstattarget FROM pg_attribute WHERE attrelid = 'pg_type' :: REGCLASS ;

-- 来源: 2758_file_2758
SELECT showall ();

-- 来源: 2760_XML
SELECT * FROM xmltest ORDER BY 1;

-- 来源: 2760_XML
SELECT xmlconcat(xmlcomment('hello'), xmlelement(NAME qux, 'xml'), xmlcomment('world'));

-- 来源: 2761_XMLTYPE
SELECT * FROM xmltypetest ORDER BY 1;

-- 来源: 2763_SET
SELECT * FROM employee;

-- 来源: 2763_SET
SELECT * FROM employee;

-- 来源: 2764_aclitem
SELECT * FROM table_acl;

-- 来源: 2765_file_2765
SELECT CURRENT_ROLE ;

-- 来源: 2765_file_2765
SELECT CURRENT_SCHEMA ;

-- 来源: 2765_file_2765
SELECT CURRENT_USER ;

-- 来源: 2765_file_2765
SELECT LOCALTIMESTAMP ;

-- 来源: 2765_file_2765
SELECT SESSION_USER ;

-- 来源: 2765_file_2765
SELECT SYSDATE ;

-- 来源: 2765_file_2765
SELECT USER ;

-- 来源: 2769_file_2769
SELECT bit_length ( 'world' );

-- 来源: 2769_file_2769
SELECT btrim ( 'sring' , 'ing' );

-- 来源: 2769_file_2769
SELECT char_length ( 'hello' );

-- 来源: 2769_file_2769
select dump ( 'abc测试' );

-- 来源: 2769_file_2769
SELECT instr ( 'abcdabcdabcd' , 'bcd' , 2 , 2 );

-- 来源: 2769_file_2769
SELECT instrb ( 'abcdabcdabcd' , 'bcd' , 2 , 2 );

-- 来源: 2769_file_2769
SELECT lengthb ( 'hello' );

-- 来源: 2769_file_2769
SELECT left ( 'abcde' , 2 );

-- 来源: 2769_file_2769
SELECT length ( 'jose' , 'UTF8' );

-- 来源: 2769_file_2769
SELECT lpad ( 'hi' , 5 , 'xyza' );

-- 来源: 2769_file_2769
select lpad ( 'expr1' , 7 , '中国' );

-- 来源: 2769_file_2769
select lpad ( 'expr1' , 8 , '中国' );

-- 来源: 2769_file_2769
SELECT notlike ( 1 , 2 );

-- 来源: 2769_file_2769
SELECT notlike ( 1 , 1 );

-- 来源: 2769_file_2769
SELECT octet_length ( 'jose' );

-- 来源: 2769_file_2769
SELECT overlay ( 'hello' placing 'world' from 2 for 3 );

-- 来源: 2769_file_2769
SELECT position ( 'ing' in 'string' );

-- 来源: 2769_file_2769
SELECT pg_client_encoding ();

-- 来源: 2769_file_2769
SELECT quote_ident ( 'hello world' );

-- 来源: 2769_file_2769
SELECT quote_literal ( 'hello' );

-- 来源: 2769_file_2769
SELECT quote_literal ( E 'O\' hello ');

-- 来源: 2769_file_2769
SELECT quote_literal ( 'O\hello' );

-- 来源: 2769_file_2769
SELECT quote_literal ( NULL );

-- 来源: 2769_file_2769
SELECT quote_literal ( 42 . 5 );

-- 来源: 2769_file_2769
SELECT quote_literal ( E 'O\' 42 . 5 ');

-- 来源: 2769_file_2769
SELECT quote_literal ( 'O\42.5' );

-- 来源: 2769_file_2769
SELECT quote_nullable ( 'hello' );

-- 来源: 2769_file_2769
SELECT quote_nullable ( E 'O\' hello ');

-- 来源: 2769_file_2769
SELECT quote_nullable ( 'O\hello' );

-- 来源: 2769_file_2769
SELECT quote_nullable ( NULL );

-- 来源: 2769_file_2769
SELECT quote_nullable ( 42 . 5 );

-- 来源: 2769_file_2769
SELECT quote_nullable ( E 'O\' 42 . 5 ');

-- 来源: 2769_file_2769
SELECT quote_nullable ( 'O\42.5' );

-- 来源: 2769_file_2769
SELECT quote_nullable ( NULL );

-- 来源: 2769_file_2769
select substring_inner ( 'adcde' , 2 , 3 );

-- 来源: 2769_file_2769
SELECT substring ( 'Thomas' from 2 for 3 );

-- 来源: 2769_file_2769
select substring ( 'substrteststring' , - 5 , 5 );

-- 来源: 2769_file_2769
SELECT substring_index ( 'Test1splitTest2splitTest3splitTest4' , 'split' , 2 );

-- 来源: 2769_file_2769
SELECT substring_index ( 'Test1splitTest2splitTest3splitTest4' , 'split' , - 2 );

-- 来源: 2769_file_2769
SELECT substring ( 'Thomas' from '...$' );

-- 来源: 2769_file_2769
SELECT substring ( 'foobar' from 'o(.)b' );

-- 来源: 2769_file_2769
SELECT substring ( 'foobar' from '(o(.)b)' );

-- 来源: 2769_file_2769
SELECT substring ( 'Thomas' from '%#"o_a#"_' for '#' );

-- 来源: 2769_file_2769
SELECT rawcat ( 'ab' , 'cd' );

-- 来源: 2769_file_2769
SELECT regexp_like ( 'str' , '[ac]' );

-- 来源: 2769_file_2769
SELECT regexp_substr ( 'str' , '[ac]' );

-- 来源: 2769_file_2769
SELECT regexp_substr ( 'foobarbaz' , 'b(..)' , 3 , 2 ) AS RESULT ;

-- 来源: 2769_file_2769
SELECT regexp_count('foobarbaz','b(..)', 5) AS RESULT;

-- 来源: 2769_file_2769
SELECT regexp_instr('foobarbaz','b(..)', 1, 1, 0) AS RESULT;

-- 来源: 2769_file_2769
SELECT regexp_instr('foobarbaz','b(..)', 1, 2, 0) AS RESULT;

-- 来源: 2769_file_2769
SELECT regexp_matches ( 'foobarbequebaz' , '(bar)(beque)' );

-- 来源: 2769_file_2769
SELECT regexp_matches ( 'foobarbequebaz' , 'barbeque' );

-- 来源: 2769_file_2769
SELECT regexp_matches ( 'foobarbequebazilbarfbonk' , '(b[^b]+)(b[^b]+)' , 'g' );

-- 来源: 2769_file_2769
SELECT regexp_match('foobarbequebaz', '(bar)(beque)');

-- 来源: 2769_file_2769
SELECT (regexp_match('foobarbequebaz', 'bar.*que'))[1];

-- 来源: 2769_file_2769
SELECT regexp_match('Learning #PostgreSQL', 'R', 'c');

-- 来源: 2769_file_2769
SELECT regexp_match('hello world', 'h e l l o', 'x');

-- 来源: 2769_file_2769
SELECT regexp_split_to_array ( 'hello world' , E '\\s+' );

-- 来源: 2769_file_2769
SELECT regexp_split_to_table ( 'hello world' , E '\\s+' );

-- 来源: 2769_file_2769
SELECT repeat ( 'Pg' , 4 );

-- 来源: 2769_file_2769
SELECT replace ( 'abcdefabcdef' , 'cd' , 'XXX' );

-- 来源: 2769_file_2769
SELECT replace ( 'abcdefabcdef' , 'cd' );

-- 来源: 2769_file_2769
SELECT reverse ( 'abcde' );

-- 来源: 2769_file_2769
SELECT right ( 'abcde' , 2 );

-- 来源: 2769_file_2769
SELECT right ( 'abcde' , - 2 );

-- 来源: 2769_file_2769
SELECT rpad ( 'hi' , 5 , 'xy' );

-- 来源: 2769_file_2769
select rpad ( 'expr1' , 7 , '中国' ) || '*' ;

-- 来源: 2769_file_2769
select rpad ( 'expr1' , 8 , '中国' ) || '*' ;

-- 来源: 2769_file_2769
SELECT substr ( 'stringtest' FROM 4 );

-- 来源: 2769_file_2769
SELECT substr ( 'stringtest' , 4 );

-- 来源: 2769_file_2769
SELECT substr ( 'stringtest' , - 4 );

-- 来源: 2769_file_2769
SELECT substr ( 'stringtest' , 11 );

-- 来源: 2769_file_2769
SELECT substr ( 'teststring' FROM 5 FOR 2 );

-- 来源: 2769_file_2769
SELECT substr ( 'teststring' , 5 , 2 );

-- 来源: 2769_file_2769
SELECT substr ( 'teststring' , 5 , 10 );

-- 来源: 2769_file_2769
SELECT substrb ( 'string' , 2 );

-- 来源: 2769_file_2769
SELECT substrb ( 'string' , - 2 );

-- 来源: 2769_file_2769
SELECT substrb ( 'string' , 10 );

-- 来源: 2769_file_2769
SELECT substrb ( '数据库' , 1 );

-- 来源: 2769_file_2769
SELECT substrb ( '数据库' , 2 );

-- 来源: 2769_file_2769
SELECT substrb ( 'string' , 2 , 3 );

-- 来源: 2769_file_2769
SELECT substrb ( 'string' , 2 , 10 );

-- 来源: 2769_file_2769
SELECT substrb ( '数据库' , 4 , 3 );

-- 来源: 2769_file_2769
SELECT substrb ( '数据库' , 2 , 6 ) = ' 据' as result ;

-- 来源: 2769_file_2769
SELECT substrb ( '数据库' , 2 , 6 ) = ' 据 ' as result ;

-- 来源: 2769_file_2769
SELECT 'MPP' || 'DB' AS RESULT ;

-- 来源: 2769_file_2769
SELECT 'Value: ' || 42 AS RESULT ;

-- 来源: 2769_file_2769
SELECT split_part ( 'abc~@~def~@~ghi' , '~@~' , 2 );

-- 来源: 2769_file_2769
SELECT strpos ( 'source' , 'rc' );

-- 来源: 2769_file_2769
SELECT to_hex ( 2147483647 );

-- 来源: 2769_file_2769
SELECT translate ( '12345' , '143' , 'ax' );

-- 来源: 2769_file_2769
SELECT length ( 'abcd' );

-- 来源: 2769_file_2769
SELECT length ( '汉字abc' );

-- 来源: 2769_file_2769
SELECT lengthb ( 'Chinese' );

-- 来源: 2769_file_2769
SELECT trim ( BOTH 'x' FROM 'xTomxx' );

-- 来源: 2769_file_2769
SELECT trim ( LEADING 'x' FROM 'xTomxx' );

-- 来源: 2769_file_2769
SELECT trim ( TRAILING 'x' FROM 'xTomxx' );

-- 来源: 2769_file_2769
select to_single_byte ( 'ＡＢ１２３' );

-- 来源: 2769_file_2769
select to_multi_byte ( 'ABC123' );

-- 来源: 2769_file_2769
SELECT rtrim ( 'TRIMxxxx' , 'x' );

-- 来源: 2769_file_2769
SELECT ltrim ( 'xxxxTRIM' , 'x' );

-- 来源: 2769_file_2769
SELECT upper ( 'tom' );

-- 来源: 2769_file_2769
SELECT lower ( 'TOM' );

-- 来源: 2769_file_2769
SELECT nls_upper ( 'gro?e' );

-- 来源: 2769_file_2769
SELECT nls_upper ( 'gro?e' , 'nls_sort = XGerman' );

-- 来源: 2769_file_2769
SELECT nls_lower ( 'INDIVISIBILITY' );

-- 来源: 2769_file_2769
SELECT nls_lower ( 'INDIVISIBILITY' , 'nls_sort = XTurkish' );

-- 来源: 2769_file_2769
SELECT instr ( 'corporate floor' , 'or' , 3 );

-- 来源: 2769_file_2769
SELECT instr ( 'corporate floor' , 'or' , - 3 , 2 );

-- 来源: 2769_file_2769
SELECT initcap ( 'hi THOMAS' );

-- 来源: 2769_file_2769
SELECT ascii ( 'xyz' );

-- 来源: 2769_file_2769
SELECT ascii2 ( 'xyz' );

-- 来源: 2769_file_2769
select ascii2 ( '中xyz' );

-- 来源: 2769_file_2769
SELECT asciistr ( 'xyz中' );

-- 来源: 2769_file_2769
select unistr ( 'abc\0041\4E2D' );

-- 来源: 2769_file_2769
select vsize ( 'abc测试' );

-- 来源: 2769_file_2769
SELECT replace ( 'jack and jue' , 'j' , 'bl' );

-- 来源: 2769_file_2769
SELECT concat ( 'Hello' , ' World!' );

-- 来源: 2769_file_2769
SELECT concat ( 'Hello' , NULL );

-- 来源: 2769_file_2769
SELECT * frOm test_space WHERE c = 'a ' ;

-- 来源: 2769_file_2769
SELECT * FROM test_space WHERE c = 'a' || ' ' ;

-- 来源: 2769_file_2769
SELECT chr ( 65 );

-- 来源: 2769_file_2769
select chr ( 19968 );

-- 来源: 2769_file_2769
SELECT chr ( 65 );

-- 来源: 2769_file_2769
select chr ( 16705 );

-- 来源: 2769_file_2769
select chr ( 4259905 );

-- 来源: 2769_file_2769
SELECT nchr ( 65 );

-- 来源: 2769_file_2769
SELECT nchr ( 14989440 );

-- 来源: 2769_file_2769
SELECT nchr ( 14989440 );

-- 来源: 2769_file_2769
SELECT nchr ( 4321090 );

-- 来源: 2769_file_2769
SELECT nchr ( 14989440 );

-- 来源: 2769_file_2769
SELECT nchr ( 14989440 );

-- 来源: 2769_file_2769
SELECT regexp_substr ( '500 Hello World, Redwood Shores, CA' , ',[^,]+,' ) "REGEXPR_SUBSTR" ;

-- 来源: 2769_file_2769
SELECT regexp_replace ( 'Thomas' , '.[mN]a.' , 'M' );

-- 来源: 2769_file_2769
SELECT regexp_replace ( 'foobarbaz' , 'b(..)' , E 'X\\1Y' , 'g' ) AS RESULT ;

-- 来源: 2769_file_2769
SELECT regexp_replace('foobarbaz','b(..)', E'X\\1Y', 2, 2, 'n') AS RESULT;

-- 来源: 2769_file_2769
SELECT concat_ws ( ',' , 'ABCDE' , 2 , NULL , 22 );

-- 来源: 2769_file_2769
select * from test order by nlssort ( a , 'nls_sort=schinese_pinyin_m' );

-- 来源: 2769_file_2769
select * from test order by nlssort ( a , 'nls_sort=generic_m_ci' );

-- 来源: 2769_file_2769
SELECT convert ( 'text_in_utf8' , 'UTF8' , 'GBK' );

-- 来源: 2769_file_2769
SELECT convert_from ( 'some text' , 'GBK' );

-- 来源: 2769_file_2769
SELECT convert ( 'asdas' using 'gbk' );

-- 来源: 2769_file_2769
SELECT convert_from ( 'text_in_utf8' , 'UTF8' );

-- 来源: 2769_file_2769
SELECT convert_to ( 'some text' , 'UTF8' );

-- 来源: 2769_file_2769
SELECT 'AA_BBCC' LIKE '%A@_B%' ESCAPE '@' AS RESULT ;

-- 来源: 2769_file_2769
SELECT 'AA_BBCC' LIKE '%A@_B%' AS RESULT ;

-- 来源: 2769_file_2769
SELECT 'AA@_BBCC' LIKE '%A@_B%' AS RESULT ;

-- 来源: 2769_file_2769
SELECT regexp_like ( 'ABC' , '[A-Z]' );

-- 来源: 2769_file_2769
SELECT regexp_like ( 'ABC' , '[D-Z]' );

-- 来源: 2769_file_2769
SELECT regexp_like ( 'ABC' , '[a-z]' , 'i' );

-- 来源: 2769_file_2769
SELECT format ( 'Hello %s, %1$s' , 'World' );

-- 来源: 2769_file_2769
SELECT md5 ( 'ABC' );

-- 来源: 2769_file_2769
select sha ( 'ABC' );

-- 来源: 2769_file_2769
select sha1 ( 'ABC' );

-- 来源: 2769_file_2769
select sha2 ( 'ABC' , 224 );

-- 来源: 2769_file_2769
select sha2 ( 'ABC' , 256 );

-- 来源: 2769_file_2769
select sha2 ( 'ABC' , 0 );

-- 来源: 2769_file_2769
SELECT decode ( 'MTIzAAE=' , 'base64' );

-- 来源: 2769_file_2769
select similar_escape('\s+ab','2');

-- 来源: 2769_file_2769
select site , find_in_set ( 'wuhan' , site ) from employee ;

-- 来源: 2769_file_2769
select find_in_set ( 'ee' , 'a,ee,c' );

-- 来源: 2769_file_2769
SELECT encode ( E '123\\000\\001' , 'base64' );

-- 来源: 2769_file_2769
SELECT translate('12345','123','');

-- 来源: 2769_file_2769
SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- 来源: 2769_file_2769
SELECT * FROM logs_nchar WHERE log_id = RPAD ( TRIM ( 'FE306991300002 ' ), 16 , ' ' );

-- 来源: 2769_file_2769
SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- 来源: 2769_file_2769
SELECT * FROM logs_text WHERE log_id = 'FE306991300002 ' :: bpchar ;

-- 来源: 2769_file_2769
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- 来源: 2769_file_2769
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = 'FE306991300002 ' ;

-- 来源: 2769_file_2769
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = t2 . log_id ;

-- 来源: 2769_file_2769
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = 'FE306991300002 ' ;

-- 来源: 2769_file_2769
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id :: bpchar = t2 . log_id ;

-- 来源: 2769_file_2769
SELECT * FROM logs_varchar2 t1 , logs_char t2 WHERE t1 . log_id = 'FE306991300002 ' ;

-- 来源: 2770_file_2770
SELECT octet_length ( E 'jo\\000se' :: bytea ) AS RESULT ;

-- 来源: 2770_file_2770
SELECT overlay ( E 'Th\\000omas' :: bytea placing E '\\002\\003' :: bytea from 2 for 3 ) AS RESULT ;

-- 来源: 2770_file_2770
SELECT position ( E '\\000om' :: bytea in E 'Th\\000omas' :: bytea ) AS RESULT ;

-- 来源: 2770_file_2770
SELECT substring ( E 'Th\\000omas' :: bytea from 2 for 3 ) AS RESULT ;

-- 来源: 2770_file_2770
select substr ( E 'Th\\000omas' :: bytea , 2 , 3 ) as result ;

-- 来源: 2770_file_2770
SELECT trim ( E '\\000' :: bytea from E '\\000Tom\\000' :: bytea ) AS RESULT ;

-- 来源: 2770_file_2770
SELECT btrim ( E '\\000trim\\000' :: bytea , E '\\000' :: bytea ) AS RESULT ;

-- 来源: 2770_file_2770
SELECT get_bit ( E 'Th\\000omas' :: bytea , 45 ) AS RESULT ;

-- 来源: 2770_file_2770
SELECT get_byte ( E 'Th\\000omas' :: bytea , 4 ) AS RESULT ;

-- 来源: 2770_file_2770
SELECT set_bit ( E 'Th\\000omas' :: bytea , 45 , 0 ) AS RESULT ;

-- 来源: 2770_file_2770
SELECT set_byte ( E 'Th\\000omas' :: bytea , 4 , 64 ) AS RESULT ;

-- 来源: 2771_file_2771
SELECT B '10001' || B '011' AS RESULT ;

-- 来源: 2771_file_2771
SELECT B '10001' & B '01101' AS RESULT ;

-- 来源: 2771_file_2771
SELECT B '10001' | B '01101' AS RESULT ;

-- 来源: 2771_file_2771
SELECT B '10001' # B '01101' AS RESULT ;

-- 来源: 2771_file_2771
SELECT ~ B '10001' AS RESULT ;

-- 来源: 2771_file_2771
SELECT B '10001' << 3 AS RESULT ;

-- 来源: 2771_file_2771
SELECT B '10001' >> 2 AS RESULT ;

-- 来源: 2771_file_2771
SELECT 44 :: bit ( 10 ) AS RESULT ;

-- 来源: 2771_file_2771
SELECT 44 :: bit ( 3 ) AS RESULT ;

-- 来源: 2771_file_2771
SELECT cast ( - 44 as bit ( 12 )) AS RESULT ;

-- 来源: 2771_file_2771
SELECT '1110' :: bit ( 4 ):: integer AS RESULT ;

-- 来源: 2771_file_2771
select substring ( '10101111' :: bit ( 8 ), 2 );

-- 来源: 2772_file_2772
SELECT 'abc' LIKE 'abc' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' LIKE 'a%' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' LIKE '_b_' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' LIKE 'c' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' SIMILAR TO 'abc' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' SIMILAR TO 'a' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' SIMILAR TO '%(b|d)%' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' SIMILAR TO '(b|c)%' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' ~ 'Abc' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' ~* 'Abc' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' !~ 'Abc' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' !~* 'Abc' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' ~ '^a' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' ~ '(b|d)' AS RESULT ;

-- 来源: 2772_file_2772
SELECT 'abc' ~ '^(b|c)' AS RESULT ;

-- 来源: 2773_file_2773
SELECT 2 + 3 AS RESULT ;

-- 来源: 2773_file_2773
SELECT 2 - 3 AS RESULT ;

-- 来源: 2773_file_2773
SELECT 2 * 3 AS RESULT ;

-- 来源: 2773_file_2773
SELECT 4 / 2 AS RESULT ;

-- 来源: 2773_file_2773
SELECT 4 / 3 AS RESULT ;

-- 来源: 2773_file_2773
SELECT - 2 AS RESULT ;

-- 来源: 2773_file_2773
SELECT 5 % 4 AS RESULT ;

-- 来源: 2773_file_2773
SELECT @ - 5 . 0 AS RESULT ;

-- 来源: 2773_file_2773
SELECT 2 . 0 ^ 3 . 0 AS RESULT ;

-- 来源: 2773_file_2773
SELECT |/ 25 . 0 AS RESULT ;

-- 来源: 2773_file_2773
SELECT ||/ 27 . 0 AS RESULT ;

-- 来源: 2773_file_2773
SELECT 5 ! AS RESULT ;

-- 来源: 2773_file_2773
SELECT !! 5 AS RESULT ;

-- 来源: 2773_file_2773
SELECT 91 & 15 AS RESULT ;

-- 来源: 2773_file_2773
SELECT 32 | 3 AS RESULT ;

-- 来源: 2773_file_2773
SELECT 17 # 5 AS RESULT ;

-- 来源: 2773_file_2773
SELECT ~ 1 AS RESULT ;

-- 来源: 2773_file_2773
SELECT 1 << 4 AS RESULT ;

-- 来源: 2773_file_2773
SELECT 8 >> 2 AS RESULT ;

-- 来源: 2773_file_2773
SELECT abs ( - 17 . 4 );

-- 来源: 2773_file_2773
SELECT acos ( - 1 );

-- 来源: 2773_file_2773
SELECT asin ( 0 . 5 );

-- 来源: 2773_file_2773
SELECT atan ( 1 );

-- 来源: 2773_file_2773
SELECT atan2 ( 2 , 1 );

-- 来源: 2773_file_2773
SELECT bitand ( 127 , 63 );

-- 来源: 2773_file_2773
SELECT cbrt ( 27 . 0 );

-- 来源: 2773_file_2773
SELECT ceil ( - 42 . 8 );

-- 来源: 2773_file_2773
SELECT ceiling ( - 95 . 3 );

-- 来源: 2773_file_2773
SELECT cos ( - 3 . 1415927 );

-- 来源: 2773_file_2773
SELECT cosh ( 4 );

-- 来源: 2773_file_2773
SELECT cot ( 1 );

-- 来源: 2773_file_2773
SELECT degrees ( 0 . 5 );

-- 来源: 2773_file_2773
SELECT div ( 9 , 4 );

-- 来源: 2773_file_2773
SELECT exp ( 1 . 0 );

-- 来源: 2773_file_2773
SELECT floor ( - 42 . 8 );

-- 来源: 2773_file_2773
select int1 ( '123' );

-- 来源: 2773_file_2773
select int1 ( '1.1' );

-- 来源: 2773_file_2773
select int2 ( '1234' );

-- 来源: 2773_file_2773
select int2 ( 25 . 3 );

-- 来源: 2773_file_2773
select int4 ( '789' );

-- 来源: 2773_file_2773
select int4 ( 99 . 9 );

-- 来源: 2773_file_2773
select float4 ( '789' );

-- 来源: 2773_file_2773
select float4 ( 99 . 9 );

-- 来源: 2773_file_2773
select float8 ( '789' );

-- 来源: 2773_file_2773
select float8 ( 99 . 9 );

-- 来源: 2773_file_2773
select int16 ( '789' );

-- 来源: 2773_file_2773
select int16 ( 99 . 9 );

-- 来源: 2773_file_2773
select "numeric" ( '789' );

-- 来源: 2773_file_2773
select "numeric" ( 99 . 9 );

-- 来源: 2773_file_2773
SELECT radians ( 45 . 0 );

-- 来源: 2773_file_2773
SELECT random ();

-- 来源: 2773_file_2773
SELECT multiply ( 9 . 0 , '3.0' );

-- 来源: 2773_file_2773
SELECT multiply ( '9.0' , 3 . 0 );

-- 来源: 2773_file_2773
SELECT ln ( 2 . 0 );

-- 来源: 2773_file_2773
SELECT log ( 100 . 0 );

-- 来源: 2773_file_2773
SELECT log ( 2 . 0 , 64 . 0 );

-- 来源: 2773_file_2773
SELECT mod ( 9 , 4 );

-- 来源: 2773_file_2773
SELECT mod ( 9 , 0 );

-- 来源: 2773_file_2773
SELECT pi ();

-- 来源: 2773_file_2773
SELECT power ( 9 . 0 , 3 . 0 );

-- 来源: 2773_file_2773
SELECT remainder ( 11 , 4 );

-- 来源: 2773_file_2773
SELECT remainder ( 9 , 0 );

-- 来源: 2773_file_2773
SELECT round ( 42 . 4 );

-- 来源: 2773_file_2773
SELECT round ( 42 . 6 );

-- 来源: 2773_file_2773
SELECT round ( - 0 . 2 :: float8 );

-- 来源: 2773_file_2773
SELECT round ( 42 . 4382 , 2 );

-- 来源: 2773_file_2773
SELECT setseed ( 0 . 54823 );

-- 来源: 2773_file_2773
SELECT sign ( - 8 . 4 );

-- 来源: 2773_file_2773
SELECT sin ( 1 . 57079 );

-- 来源: 2773_file_2773
SELECT sinh ( 4 );

-- 来源: 2773_file_2773
SELECT sqrt ( 2 . 0 );

-- 来源: 2773_file_2773
SELECT tan ( 20 );

-- 来源: 2773_file_2773
SELECT tanh ( 0 . 1 );

-- 来源: 2773_file_2773
SELECT trunc ( 42 . 8 );

-- 来源: 2773_file_2773
SELECT trunc ( 42 . 4382 , 2 );

-- 来源: 2773_file_2773
SELECT width_bucket ( 5 . 35 , 0 . 024 , 10 . 06 , 5 );

-- 来源: 2773_file_2773
SELECT width_bucket ( 5 . 35 , 0 . 024 , 10 . 06 , 5 );

-- 来源: 2773_file_2773
SELECT nanvl('NaN', 1.1);

-- 来源: 2773_file_2773
SELECT numeric_eq_text(1, '1');

-- 来源: 2773_file_2773
SELECT text_eq_numeric('1', 1);

-- 来源: 2773_file_2773
SELECT bigint_eq_text(1, '1');

-- 来源: 2773_file_2773
SELECT text_eq_bigint('1', 1);

-- 来源: 2774_file_2774
SELECT date '2001-10-01' - '7' AS RESULT ;

-- 来源: 2774_file_2774
SELECT date '2001-9-28' + integer '7' AS RESULT ;

-- 来源: 2774_file_2774
SELECT date '2001-09-28' + interval '1 hour' AS RESULT ;

-- 来源: 2774_file_2774
SELECT date '2001-09-28' + time '03:00' AS RESULT ;

-- 来源: 2774_file_2774
SELECT interval '1 day' + interval '1 hour' AS RESULT ;

-- 来源: 2774_file_2774
SELECT timestamp '2001-09-28 01:00' + interval '23 hours' AS RESULT ;

-- 来源: 2774_file_2774
SELECT time '01:00' + interval '3 hours' AS RESULT ;

-- 来源: 2774_file_2774
SELECT date '2001-10-01' - date '2001-09-28' AS RESULT ;

-- 来源: 2774_file_2774
SELECT date '2001-10-01' - integer '7' AS RESULT ;

-- 来源: 2774_file_2774
SELECT date '2001-09-28' - interval '1 hour' AS RESULT ;

-- 来源: 2774_file_2774
SELECT time '05:00' - time '03:00' AS RESULT ;

-- 来源: 2774_file_2774
SELECT time '05:00' - interval '2 hours' AS RESULT ;

-- 来源: 2774_file_2774
SELECT timestamp '2001-09-28 23:00' - interval '23 hours' AS RESULT ;

-- 来源: 2774_file_2774
SELECT interval '1 day' - interval '1 hour' AS RESULT ;

-- 来源: 2774_file_2774
SELECT timestamp '2001-09-29 03:00' - timestamp '2001-09-27 12:00' AS RESULT ;

-- 来源: 2774_file_2774
SELECT 900 * interval '1 second' AS RESULT ;

-- 来源: 2774_file_2774
SELECT 21 * interval '1 day' AS RESULT ;

-- 来源: 2774_file_2774
SELECT double precision '3.5' * interval '1 hour' AS RESULT ;

-- 来源: 2774_file_2774
SELECT interval '1 hour' / double precision '1.5' AS RESULT ;

-- 来源: 2774_file_2774
SELECT age ( timestamp '2001-04-10' , timestamp '1957-06-13' );

-- 来源: 2774_file_2774
SELECT age ( timestamp '1957-06-13' );

-- 来源: 2774_file_2774
SELECT clock_timestamp ();

-- 来源: 2774_file_2774
SELECT current_date ;

-- 来源: 2774_file_2774
SELECT current_time ;

-- 来源: 2774_file_2774
SELECT current_timestamp ;

-- 来源: 2774_file_2774
SELECT current_timestamp ;

-- 来源: 2774_file_2774
SELECT current_timestamp ();

-- 来源: 2774_file_2774
SELECT current_timestamp ( 1 );

-- 来源: 2774_file_2774
SELECT current_timestamp ( 1 );

-- 来源: 2774_file_2774
SELECT pg_systimestamp ();

-- 来源: 2774_file_2774
SELECT date_part ( 'hour' , timestamp '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT date_part ( 'month' , interval '2 years 3 months' );

-- 来源: 2774_file_2774
SELECT date_trunc ( 'hour' , timestamp '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT trunc ( timestamp '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT trunc ( timestamp '2001-02-16 20:38:40' , 'hour' );

-- 来源: 2774_file_2774
SELECT round ( timestamp '2001-02-16 20:38:40' , 'hour' );

-- 来源: 2774_file_2774
SELECT daterange ( '2000-05-06' , '2000-08-08' );

-- 来源: 2774_file_2774
SELECT daterange ( '2000-05-06' , '2000-08-08' , '[]' );

-- 来源: 2774_file_2774
SELECT isfinite ( date '2001-02-16' );

-- 来源: 2774_file_2774
SELECT isfinite ( date 'infinity' );

-- 来源: 2774_file_2774
SELECT isfinite ( timestamp '2001-02-16 21:28:30' );

-- 来源: 2774_file_2774
SELECT isfinite ( timestamp 'infinity' );

-- 来源: 2774_file_2774
SELECT isfinite ( interval '4 hours' );

-- 来源: 2774_file_2774
SELECT justify_days ( interval '35 days' );

-- 来源: 2774_file_2774
SELECT JUSTIFY_HOURS ( INTERVAL '27 HOURS' );

-- 来源: 2774_file_2774
SELECT JUSTIFY_INTERVAL ( INTERVAL '1 MON -1 HOUR' );

-- 来源: 2774_file_2774
SELECT localtime AS RESULT ;

-- 来源: 2774_file_2774
SELECT localtimestamp ;

-- 来源: 2774_file_2774
SELECT maketime ( 8 , 15 , 26 . 53 );

-- 来源: 2774_file_2774
SELECT maketime ( - 888 , 15 , 26 . 53 );

-- 来源: 2774_file_2774
SELECT now ();

-- 来源: 2774_file_2774
SELECT timenow ();

-- 来源: 2774_file_2774
SELECT dbtimezone ;

-- 来源: 2774_file_2774
SELECT numtodsinterval ( 100 , 'HOUR' );

-- 来源: 2774_file_2774
SELECT numtodsinterval ( 100 , 'HOUR' );

-- 来源: 2774_file_2774
SELECT numtoyminterval ( 100 , 'MONTH' );

-- 来源: 2774_file_2774
SELECT numtodsinterval ( 100 , 'MONTH' );

-- 来源: 2774_file_2774
SELECT new_time ( '1997-10-10' , 'AST' , 'EST' );

-- 来源: 2774_file_2774
SELECT NEW_TIME ( TO_TIMESTAMP ( '10-Sep-02 14:10:10.123000' , 'DD-Mon-RR HH24:MI:SS.FF' ), 'AST' , 'PST' );

-- 来源: 2774_file_2774
SELECT SESSIONTIMEZONE ;

-- 来源: 2774_file_2774
SELECT LOWER ( SESSIONTIMEZONE );

-- 来源: 2774_file_2774
SELECT SYS_EXTRACT_UTC ( TIMESTAMP '2000-03-28 11:30:00.00' );

-- 来源: 2774_file_2774
SELECT SYS_EXTRACT_UTC ( TIMESTAMPTZ '2000-03-28 11:30:00.00 -08:00' );

-- 来源: 2774_file_2774
SELECT TZ_OFFSET ( 'US/Pacific' );

-- 来源: 2774_file_2774
SELECT TZ_OFFSET ( sessiontimezone );

-- 来源: 2774_file_2774
SELECT pg_sleep ( 10 );

-- 来源: 2774_file_2774
SELECT statement_timestamp ();

-- 来源: 2774_file_2774
SELECT sysdate ;

-- 来源: 2774_file_2774
SELECT current_sysdate ();

-- 来源: 2774_file_2774
SELECT timeofday ();

-- 来源: 2774_file_2774
SELECT transaction_timestamp ();

-- 来源: 2774_file_2774
SELECT transaction_timestamp ();

-- 来源: 2774_file_2774
SELECT add_months ( to_date ( '2017-5-29' , 'yyyy-mm-dd' ), 11 ) FROM sys_dummy ;

-- 来源: 2774_file_2774
SELECT last_day ( to_date ( '2017-01-01' , 'YYYY-MM-DD' )) AS cal_result ;

-- 来源: 2774_file_2774
SELECT months_between(to_date('2022-10-31', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

-- 来源: 2774_file_2774
SELECT months_between(to_date('2022-10-30', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

-- 来源: 2774_file_2774
SELECT months_between(to_date('2022-10-29', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

-- 来源: 2774_file_2774
SELECT next_day ( timestamp '2017-05-25 00:00:00' , 'Sunday' ) AS cal_result ;

-- 来源: 2774_file_2774
SELECT next_day ( timestamp '2024-01-17 00:00:00' , 7 . 9999999 );

-- 来源: 2774_file_2774
SELECT next_day ( timestamp '2024-01-17 00:00:00' , 7 . 9999999 );

-- 来源: 2774_file_2774
SELECT tintervalend ( '["Sep 4, 1983 23:59:12" "Oct4, 1983 23:59:12"]' );

-- 来源: 2774_file_2774
SELECT tintervalrel ( '["Sep 4, 1983 23:59:12" "Oct4, 1983 23:59:12"]' );

-- 来源: 2774_file_2774
SELECT ADDDATE ( '2018-05-01' , INTERVAL 1 DAY );

-- 来源: 2774_file_2774
SELECT ADDDATE('2018-05-01', 1);

-- 来源: 2774_file_2774
SELECT curdate ();

-- 来源: 2774_file_2774
SELECT curtime ( 3 );

-- 来源: 2774_file_2774
SELECT DATE_ADD('2018-05-01', INTERVAL 1 DAY);

-- 来源: 2774_file_2774
SELECT DATE_ADD('2018-05-01', 1);

-- 来源: 2774_file_2774
SELECT date_format('2023-10-11 12:13:14.151617','%b %c %M %m');

-- 来源: 2774_file_2774
SELECT DATE_SUB('2018-05-01', INTERVAL 1 YEAR);

-- 来源: 2774_file_2774
SELECT DATE_SUB('2023-1-1', 20);

-- 来源: 2774_file_2774
SELECT datediff('2021-11-12','2021-11-13');

-- 来源: 2774_file_2774
SELECT day('2023-01-02');

-- 来源: 2774_file_2774
SELECT dayofmonth('23-05-22');

-- 来源: 2774_file_2774
SELECT dayname('2023-10-11');

-- 来源: 2774_file_2774
SELECT dayofweek('2023-04-16');

-- 来源: 2774_file_2774
SELECT dayofyear('2000-12-31');

-- 来源: 2774_file_2774
SELECT extract(YEAR FROM '2023-10-11');

-- 来源: 2774_file_2774
SELECT extract(QUARTER FROM '2023-10-11');

-- 来源: 2774_file_2774
SELECT extract(MONTH FROM '2023-10-11');

-- 来源: 2774_file_2774
SELECT extract(WEEK FROM '2023-10-11');

-- 来源: 2774_file_2774
SELECT extract(DAY FROM '2023-10-11');

-- 来源: 2774_file_2774
SELECT extract(HOUR FROM '2023-10-11 12:13:14');

-- 来源: 2774_file_2774
SELECT from_days(36524);

-- 来源: 2774_file_2774
SELECT from_unixtime(1111885200);

-- 来源: 2774_file_2774
SELECT get_format(date, 'eur');

-- 来源: 2774_file_2774
SELECT get_format(date, 'usa');

-- 来源: 2774_file_2774
SELECT HOUR('10:10:10.1');

-- 来源: 2774_file_2774
SELECT makedate(2000, 60);

-- 来源: 2774_file_2774
SELECT MICROSECOND('2023-5-5 10:10:10.24485');

-- 来源: 2774_file_2774
SELECT MINUTE(time'10:10:10');

-- 来源: 2774_file_2774
SELECT month('2021-11-30');

-- 来源: 2774_file_2774
SELECT monthname('2023-02-28');

-- 来源: 2774_file_2774
SELECT period_add(202205, -12);

-- 来源: 2774_file_2774
SELECT period_diff('202101', '202102');

-- 来源: 2774_file_2774
SELECT SECOND('2023-5-5 10:10:10');

-- 来源: 2774_file_2774
SELECT QUARTER('2012-1-1');

-- 来源: 2774_file_2774
SELECT str_to_date('May 1, 2013','%M %d,%Y');

-- 来源: 2774_file_2774
SELECT SUBDATE('2023-1-1', 20);

-- 来源: 2774_file_2774
SELECT SUBDATE('2018-05-01', INTERVAL 1 YEAR);

-- 来源: 2774_file_2774
SELECT subtime('2000-03-01 20:59:59', '22:58');

-- 来源: 2774_file_2774
SELECT addtime('2000-03-01 20:59:59', '00:00:01');

-- 来源: 2774_file_2774
SELECT TIME_FORMAT('25:30:30', '%T|%r|%H|%h|%I|%i|%S|%f|%p|%k');

-- 来源: 2774_file_2774
SELECT time_to_sec('00:00:01');

-- 来源: 2774_file_2774
SELECT timediff(date'2022-12-30',20221229);

-- 来源: 2774_file_2774
SELECT TIMESTAMPADD(DAY,-2,'2022-07-27');

-- 来源: 2774_file_2774
SELECT to_days('2000-1-1');

-- 来源: 2774_file_2774
SELECT TO_SECONDS('2009-11-29 13:43:32');

-- 来源: 2774_file_2774
SELECT UNIX_TIMESTAMP('2022-12-22');

-- 来源: 2774_file_2774
SELECT utc_date();

-- 来源: 2774_file_2774
SELECT utc_time();

-- 来源: 2774_file_2774
SELECT utc_timestamp();

-- 来源: 2774_file_2774
SELECT week(date'2000-01-01', 1);

-- 来源: 2774_file_2774
SELECT week('2000-01-01', 2);

-- 来源: 2774_file_2774
SELECT weekday('1970-01-01 12:00:00');

-- 来源: 2774_file_2774
SELECT weekofyear('1970-05-22');

-- 来源: 2774_file_2774
SELECT year('23-05-22');

-- 来源: 2774_file_2774
SELECT yearweek(datetime'2000-01-01', 3);

-- 来源: 2774_file_2774
SELECT timestamp_diff ( 'year' , '2018-01-01' , '2020-04-01' );

-- 来源: 2774_file_2774
SELECT timestamp_diff ( 'month' , '2018-01-01' , '2020-04-01' );

-- 来源: 2774_file_2774
SELECT timestamp_diff ( 'quarter' , '2018-01-01' , '2020-04-01' );

-- 来源: 2774_file_2774
SELECT timestamp_diff ( 'week' , '2018-01-01' , '2020-04-01' );

-- 来源: 2774_file_2774
SELECT timestamp_diff ( 'day' , '2018-01-01' , '2020-04-01' );

-- 来源: 2774_file_2774
SELECT timestamp_diff ( 'hour' , '2018-01-01 10:10:10' , '2018-01-01 12:12:12' );

-- 来源: 2774_file_2774
SELECT timestamp_diff ( 'minute' , '2018-01-01 10:10:10' , '2018-01-01 12:12:12' );

-- 来源: 2774_file_2774
SELECT timestamp_diff ( 'second' , '2018-01-01 10:10:10' , '2018-01-01 10:12:12' );

-- 来源: 2774_file_2774
SELECT timestamp_diff ( 'microsecond' , '2018-01-01 10:10:10' , '2018-01-01 10:12:12' );

-- 来源: 2774_file_2774
SELECT TIMESTAMPDIFF ( YEAR , '2018-01-01' , '2020-01-01' );

-- 来源: 2774_file_2774
SELECT TIMESTAMPDIFF ( QUARTER , '2018-01-01' , '2020-01-01' );

-- 来源: 2774_file_2774
SELECT TIMESTAMPDIFF ( MONTH , '2018-01-01' , '2020-01-01' );

-- 来源: 2774_file_2774
SELECT TIMESTAMPDIFF ( WEEK , '2018-01-01' , '2020-01-01' );

-- 来源: 2774_file_2774
SELECT TIMESTAMPDIFF ( DAY , '2018-01-01' , '2020-01-01' );

-- 来源: 2774_file_2774
SELECT TIMESTAMPDIFF ( HOUR , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

-- 来源: 2774_file_2774
SELECT TIMESTAMPDIFF ( MINUTE , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

-- 来源: 2774_file_2774
SELECT TIMESTAMPDIFF ( SECOND , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

-- 来源: 2774_file_2774
SELECT TIMESTAMPDIFF ( MICROSECOND , '2020-01-01 10:10:10.000000' , '2020-01-01 10:10:10.111111' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( CENTURY FROM TIMESTAMP '2000-12-16 12:21:13' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( DAY FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( DAY FROM INTERVAL '40 days 1 minute' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( DECADE FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( DOW FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( DOY FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( EPOCH FROM TIMESTAMP WITH TIME ZONE '2001-02-16 20:38:40.12-08' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( EPOCH FROM INTERVAL '5 days 3 hours' );

-- 来源: 2774_file_2774
SELECT TIMESTAMP WITH TIME ZONE 'epoch' + 982384720 . 12 * INTERVAL '1 second' AS RESULT ;

-- 来源: 2774_file_2774
SELECT EXTRACT ( HOUR FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( ISODOW FROM TIMESTAMP '2001-02-18 20:38:40' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( ISOYEAR FROM DATE '2006-01-01' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( WEEK FROM TIMESTAMP '2006-01-01 00:00:40' );

-- 来源: 2774_file_2774
SELECT EXTRACT(ISOYEAR FROM DATE '2006-01-02');

-- 来源: 2774_file_2774
SELECT EXTRACT(WEEK FROM TIMESTAMP '2006-01-02 00:00:40');

-- 来源: 2774_file_2774
SELECT EXTRACT ( MICROSECONDS FROM TIME '17:12:28.5' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( MILLENNIUM FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( MILLISECONDS FROM TIME '17:12:28.5' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( MINUTE FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( MONTH FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( MONTH FROM INTERVAL '2 years 13 months' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( QUARTER FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( SECOND FROM TIME '17:12:28.5' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( ISOYEAR FROM DATE '2006-01-01' );

-- 来源: 2774_file_2774
SELECT EXTRACT ( WEEK FROM TIMESTAMP '2006-01-01 00:00:40' );

-- 来源: 2774_file_2774
SELECT EXTRACT(ISOYEAR FROM DATE '2006-01-02');

-- 来源: 2774_file_2774
SELECT EXTRACT(WEEK FROM TIMESTAMP '2006-01-02 00:00:40');

-- 来源: 2774_file_2774
SELECT EXTRACT ( YEAR FROM TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT date_part ( 'day' , TIMESTAMP '2001-02-16 20:38:40' );

-- 来源: 2774_file_2774
SELECT date_part ( 'hour' , INTERVAL '4 hours 3 minutes' );

-- 来源: 2775_file_2775
SELECT cash_words ( '1.23' );

-- 来源: 2775_file_2775
SELECT convert ( 12 . 5 , text );

-- 来源: 2775_file_2775
SELECT cast ( '22-oct-1997' as timestamp );

-- 来源: 2775_file_2775
SELECT cast ( '22-ocX-1997' as timestamp DEFAULT '22-oct-1997' ON CONVERSION ERROR , 'DD-Mon-YYYY' );

-- 来源: 2775_file_2775
SELECT CAST ( 12 AS UNSIGNED );

-- 来源: 2775_file_2775
SELECT hextoraw ( '7D' );

-- 来源: 2775_file_2775
SELECT numtoday ( 2 );

-- 来源: 2775_file_2775
SELECT rawtohex ( '1234567' );

-- 来源: 2775_file_2775
select rawtohex2('12\n?$\123/2');

-- 来源: 2775_file_2775
select bit2coding('1234567890');

-- 来源: 2775_file_2775
select bit4coding('1234567890');

-- 来源: 2775_file_2775
SELECT to_blob ( '0AADD343CDBBD' :: RAW ( 10 ));

-- 来源: 2775_file_2775
SELECT to_bigint ( '123364545554455' );

-- 来源: 2775_file_2775
SELECT to_binary_double ( '12345678' );

-- 来源: 2775_file_2775
SELECT to_binary_double ( '1,2,3' , '9,9,9' );

-- 来源: 2775_file_2775
SELECT to_binary_double ( 1 e2 default 12 on conversion error );

-- 来源: 2775_file_2775
SELECT to_binary_double ( 'aa' default 12 on conversion error );

-- 来源: 2775_file_2775
SELECT to_binary_double ( '12-' default 10 on conversion error , '99S' );

-- 来源: 2775_file_2775
SELECT to_binary_double ( 'aa-' default 12 on conversion error , '99S' );

-- 来源: 2775_file_2775
SELECT to_binary_float ( '12345678' );

-- 来源: 2775_file_2775
SELECT to_binary_float ( '1,2,3' , '9,9,9' );

-- 来源: 2775_file_2775
SELECT to_binary_float ( 1 e2 default 12 on conversion error );

-- 来源: 2775_file_2775
SELECT to_binary_float ( 'aa' default 12 on conversion error );

-- 来源: 2775_file_2775
SELECT to_binary_float ( '12-' default 10 on conversion error , '99S' );

-- 来源: 2775_file_2775
SELECT to_binary_float ( 'aa-' default 12 on conversion error , '99S' );

-- 来源: 2775_file_2775
SELECT to_char ( current_timestamp , 'HH12:MI:SS' );

-- 来源: 2775_file_2775
SELECT to_char ( current_timestamp , 'FMHH12:FMMI:FMSS' );

-- 来源: 2775_file_2775
SELECT to_char ( 125 . 8 :: real , '999D99' );

-- 来源: 2775_file_2775
SELECT to_char ( 1485 , '9,999' );

-- 来源: 2775_file_2775
SELECT to_char ( 1148 . 5 , '9,999.999' );

-- 来源: 2775_file_2775
SELECT to_char ( 148 . 5 , '990999.909' );

-- 来源: 2775_file_2775
SELECT to_char ( 123 , 'XXX' );

-- 来源: 2775_file_2775
SELECT to_char ( interval '15h 2m 12s' , 'HH24:MI:SS' );

-- 来源: 2775_file_2775
SELECT to_char ( 125 , '999' );

-- 来源: 2775_file_2775
select to_char ( site ) from employee ;

-- 来源: 2775_file_2775
SELECT to_char ( - 125 . 8 , '999D99S' );

-- 来源: 2775_file_2775
SELECT to_char ( '01110' );

-- 来源: 2775_file_2775
SELECT to_char ( current_timestamp , 'HH12:MI:SS' );

-- 来源: 2775_file_2775
SELECT to_nchar ( current_timestamp , 'HH12:MI:SS' );

-- 来源: 2775_file_2775
SELECT to_nchar ( current_timestamp , 'FMHH12:FMMI:FMSS' );

-- 来源: 2775_file_2775
SELECT to_nchar ( 125 . 8 :: real , '999D99' );

-- 来源: 2775_file_2775
SELECT to_nchar ( 1485 , '9,999' );

-- 来源: 2775_file_2775
SELECT to_nchar ( 1148 . 5 , '9,999.999' );

-- 来源: 2775_file_2775
SELECT to_nchar ( 148 . 5 , '990999.909' );

-- 来源: 2775_file_2775
SELECT to_nchar ( 123 , 'XXX' );

-- 来源: 2775_file_2775
SELECT to_nchar ( interval '15h 2m 12s' , 'HH24:MI:SS' );

-- 来源: 2775_file_2775
SELECT to_nchar ( 125 , '999' );

-- 来源: 2775_file_2775
SELECT to_nchar ( - 125 . 8 , '999D99S' );

-- 来源: 2775_file_2775
SELECT to_nchar ( '01110' );

-- 来源: 2775_file_2775
SELECT to_nchar ( current_timestamp , 'HH12:MI:SS' );

-- 来源: 2775_file_2775
SELECT to_clob ( 'ABCDEF' :: RAW ( 10 ));

-- 来源: 2775_file_2775
SELECT to_clob ( 'hello111' :: CHAR ( 15 ));

-- 来源: 2775_file_2775
SELECT to_clob ( 'gauss123' :: NCHAR ( 10 ));

-- 来源: 2775_file_2775
SELECT to_clob ( 'gauss234' :: VARCHAR ( 10 ));

-- 来源: 2775_file_2775
SELECT to_clob ( 'gauss345' :: VARCHAR2 ( 10 ));

-- 来源: 2775_file_2775
SELECT to_clob ( 'gauss456' :: NVARCHAR2 ( 10 ));

-- 来源: 2775_file_2775
SELECT to_clob ( 'World222!' :: TEXT );

-- 来源: 2775_file_2775
SELECT to_date ( '2015-08-14' );

-- 来源: 2775_file_2775
SELECT to_date ( '05 Dec 2000' , 'DD Mon YYYY' );

-- 来源: 2775_file_2775
SELECT to_date ( '2015-08-14' );

-- 来源: 2775_file_2775
SELECT to_date ( '05 Dec 2000' , 'DD Mon YYYY' );

-- 来源: 2775_file_2775
select to_date('12-jan-2022' default '12-apr-2022' on conversion error);

-- 来源: 2775_file_2775
select to_date('12-ja-2022' default '12-apr-2022' on conversion error);

-- 来源: 2775_file_2775
select to_date('2022-12-12' default '2022-01-01' on conversion error, 'yyyy-mm-dd');

-- 来源: 2775_file_2775
SELECT to_number ( '12,454.8-' , '99G999D9S' );

-- 来源: 2775_file_2775
SELECT to_number ( '12,454.8-' , '99G999D9S' );

-- 来源: 2775_file_2775
select to_number ( '1e2' );

-- 来源: 2775_file_2775
select to_number ( '123.456' );

-- 来源: 2775_file_2775
select to_number ( '123' , '999' );

-- 来源: 2775_file_2775
select to_number ( '123-' , '999MI' );

-- 来源: 2775_file_2775
select to_number ( '123' default '456-' on conversion error , '999MI' );

-- 来源: 2775_file_2775
SELECT to_timestamp ( 1284352323 );

-- 来源: 2775_file_2775
SELECT to_timestamp ( '12-sep-2014' );

-- 来源: 2775_file_2775
SELECT to_timestamp ( '12-Sep-10 14:10:10.123000' , 'DD-Mon-YY HH24:MI:SS.FF' );

-- 来源: 2775_file_2775
SELECT to_timestamp ( '-1' , 'SYYYY' );

-- 来源: 2775_file_2775
SELECT to_timestamp ( '98' , 'RR' );

-- 来源: 2775_file_2775
SELECT to_timestamp ( '01' , 'RR' );

-- 来源: 2775_file_2775
SELECT to_timestamp('11-Sep-11' DEFAULT '12-Sep-10 14:10:10.123000' ON CONVERSION ERROR,'DD-Mon-YY HH24:MI:SS.FF');

-- 来源: 2775_file_2775
SELECT to_timestamp('12-Sep-10 14:10:10.123000','DD-Mon-YY HH24:MI:SSXFF');

-- 来源: 2775_file_2775
SELECT to_timestamp ( '05 Dec 2000' , 'DD Mon YYYY' );

-- 来源: 2775_file_2775
SELECT to_timestamp_tz ( '05 DeX 2000' DEFAULT '05 Dec 2001' ON CONVERSION ERROR , 'DD Mon YYYY' );

-- 来源: 2775_file_2775
SELECT to_timestamp_tz ( '05 DeX 2000' DEFAULT '05 Dec 2001' ON CONVERSION ERROR , 'DD Mon YYYY' , 'nls_date_language=AMERICAN' );

-- 来源: 2775_file_2775
select to_dsinterval ( '12 1:2:3.456' );

-- 来源: 2775_file_2775
select to_dsinterval ( 'P3DT4H5M6S' );

-- 来源: 2775_file_2775
select to_yminterval ( '1-1' );

-- 来源: 2775_file_2775
select to_yminterval ( 'P13Y3M4DT4H2M5S' );

-- 来源: 2775_file_2775
select treat ( data as json ) from json_doc ;

-- 来源: 2775_file_2775
select cast ( t1 ( 1 , 2 , 3 ) as int []) result ;

-- 来源: 2775_file_2775
select indexbytableint_to_array ( pkg1 . t1 ( 1 => 1 , 2 => 2 , 3 => 3 ));

-- 来源: 2775_file_2775
SELECT convert_to_nocase ( '12345' , 'GBK' );

-- 来源: 2776_file_2776
SELECT box '((0,0),(1,1))' + point '(2.0,0)' AS RESULT ;

-- 来源: 2776_file_2776
SELECT box '((0,0),(1,1))' - point '(2.0,0)' AS RESULT ;

-- 来源: 2776_file_2776
SELECT box '((0,0),(1,1))' * point '(2.0,0)' AS RESULT ;

-- 来源: 2776_file_2776
SELECT box '((0,0),(2,2))' / point '(2.0,0)' AS RESULT ;

-- 来源: 2776_file_2776
SELECT box '((1,-1),(-1,1))' # box '((1,1),(-2,-2))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT # path '((1,0),(0,1),(-1,0))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT @-@ path '((0,0),(1,0))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT @@ circle '((0,0),10)' AS RESULT ;

-- 来源: 2776_file_2776
SELECT circle '((0,0),1)' <-> circle '((5,0),1)' AS RESULT ;

-- 来源: 2776_file_2776
SELECT box '((0,0),(1,1))' && box '((0,0),(2,2))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT circle '((0,0),1)' << circle '((5,0),1)' AS RESULT ;

-- 来源: 2776_file_2776
SELECT circle '((5,0),1)' >> circle '((0,0),1)' AS RESULT ;

-- 来源: 2776_file_2776
SELECT box '((0,0),(1,1))' &< box '((0,0),(2,2))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT box '((0,0),(3,3))' &> box '((0,0),(2,2))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT box '((0,0),(3,3))' <<| box '((3,4),(5,5))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT box '((3,4),(5,5))' |>> box '((0,0),(3,3))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT box '((0,0),(1,1))' &<| box '((0,0),(2,2))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT box '((0,0),(3,3))' |&> box '((0,0),(2,2))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT box '((0,0),(-3,-3))' <^ box '((0,0),(2,2))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT box '((0,0),(2,2))' >^ box '((0,0),(-3,-3))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT lseg '((-1,0),(1,0))' ?# box '((-2,-2),(2,2))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT ?- lseg '((-1,0),(1,0))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT point '(1,0)' ?- point '(0,0)' AS RESULT ;

-- 来源: 2776_file_2776
SELECT ?| lseg '((-1,0),(1,0))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT point '(0,1)' ?| point '(0,0)' AS RESULT ;

-- 来源: 2776_file_2776
SELECT lseg '((0,0),(0,1))' ?-| lseg '((0,0),(1,0))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT lseg '((-1,0),(1,0))' ?|| lseg '((-1,2),(1,2))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT circle '((0,0),2)' @> point '(1,1)' AS RESULT ;

-- 来源: 2776_file_2776
SELECT point '(1,1)' <@ circle '((0,0),2)' AS RESULT ;

-- 来源: 2776_file_2776
SELECT polygon '((0,0),(1,1))' ~= polygon '((1,1),(0,0))' AS RESULT ;

-- 来源: 2776_file_2776
SELECT area ( box '((0,0),(1,1))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT center ( box '((0,0),(1,2))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT diameter ( circle '((0,0),2.0)' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT height ( box '((0,0),(1,1))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT isclosed ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT isopen ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT length ( path '((-1,0),(1,0))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT npoints ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT npoints ( polygon '((1,1),(0,0))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT pclose ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT popen ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT radius ( circle '((0,0),2.0)' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT width ( box '((0,0),(1,1))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT box ( circle '((0,0),2.0)' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT box ( point '(0,0)' , point '(1,1)' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT box ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT circle ( box '((0,0),(1,1))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT circle ( point '(0,0)' , 2 . 0 ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT circle ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT lseg ( box '((-1,0),(1,0))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT lseg ( point '(-1,0)' , point '(1,0)' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT slope(point '(1,1)', point '(0,0)') AS RESULT;

-- 来源: 2776_file_2776
SELECT path ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT point ( 23 . 4 , - 44 . 5 ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT point ( box '((-1,0),(1,0))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT point ( circle '((0,0),2.0)' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT point ( lseg '((-1,0),(1,0))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT point ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT polygon ( box '((0,0),(1,1))' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT polygon ( circle '((0,0),2.0)' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT polygon ( 12 , circle '((0,0),2.0)' ) AS RESULT ;

-- 来源: 2776_file_2776
SELECT polygon ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1.5' < inet '192.168.1.6' AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1.5' <= inet '192.168.1.5' AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1.5' = inet '192.168.1.5' AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1.5' >= inet '192.168.1.5' AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1.5' > inet '192.168.1.4' AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1.5' <> inet '192.168.1.4' AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1.5' << inet '192.168.1/24' AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1/24' <<= inet '192.168.1/24' AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1/24' >> inet '192.168.1.5' AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1/24' >>= inet '192.168.1/24' AS RESULT ;

-- 来源: 2777_file_2777
SELECT ~ inet '192.168.1.6' AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1.6' & inet '10.0.0.0' AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1.6' | inet '10.0.0.0' AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1.6' + 25 AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1.43' - 36 AS RESULT ;

-- 来源: 2777_file_2777
SELECT inet '192.168.1.43' - inet '192.168.1.19' AS RESULT ;

-- 来源: 2777_file_2777
SELECT abbrev ( inet '10.1.0.0/16' ) AS RESULT ;

-- 来源: 2777_file_2777
SELECT abbrev ( cidr '10.1.0.0/16' ) AS RESULT ;

-- 来源: 2777_file_2777
SELECT broadcast ( '192.168.1.5/24' ) AS RESULT ;

-- 来源: 2777_file_2777
SELECT family ( '127.0.0.1' ) AS RESULT ;

-- 来源: 2777_file_2777
SELECT host ( '192.168.1.5/24' ) AS RESULT ;

-- 来源: 2777_file_2777
SELECT hostmask ( '192.168.23.20/30' ) AS RESULT ;

-- 来源: 2777_file_2777
SELECT masklen ( '192.168.1.5/24' ) AS RESULT ;

-- 来源: 2777_file_2777
SELECT netmask ( '192.168.1.5/24' ) AS RESULT ;

-- 来源: 2777_file_2777
SELECT network ( '192.168.1.5/24' ) AS RESULT ;

-- 来源: 2777_file_2777
SELECT set_masklen ( '192.168.1.5/24' , 16 ) AS RESULT ;

-- 来源: 2777_file_2777
SELECT set_masklen ( '192.168.1.0/24' :: cidr , 16 ) AS RESULT ;

-- 来源: 2777_file_2777
SELECT text ( inet '192.168.1.5' ) AS RESULT ;

-- 来源: 2777_file_2777
SELECT trunc ( macaddr '12:34:56:78:90:ab' ) AS RESULT ;

-- 来源: 2778_file_2778
SELECT to_tsvector ( 'fat cats ate rats' ) @@ to_tsquery ( 'cat & rat' ) AS RESULT ;

-- 来源: 2778_file_2778
SELECT to_tsvector ( 'fat cats ate rats' ) @@@ to_tsquery ( 'cat & rat' ) AS RESULT ;

-- 来源: 2778_file_2778
SELECT 'a:1 b:2' :: tsvector || 'c:1 d:2 b:3' :: tsvector AS RESULT ;

-- 来源: 2778_file_2778
SELECT 'fat | rat' :: tsquery && 'cat' :: tsquery AS RESULT ;

-- 来源: 2778_file_2778
SELECT 'fat | rat' :: tsquery || 'cat' :: tsquery AS RESULT ;

-- 来源: 2778_file_2778
SELECT !! 'cat' :: tsquery AS RESULT ;

-- 来源: 2778_file_2778
SELECT 'cat' :: tsquery @> 'cat & rat' :: tsquery AS RESULT ;

-- 来源: 2778_file_2778
SELECT 'cat' :: tsquery <@ 'cat & rat' :: tsquery AS RESULT ;

-- 来源: 2778_file_2778
SELECT get_current_ts_config ();

-- 来源: 2778_file_2778
SELECT length ( 'fat:2,4 cat:3 rat:5A' :: tsvector );

-- 来源: 2778_file_2778
SELECT numnode ( '(fat & rat) | cat' :: tsquery );

-- 来源: 2778_file_2778
SELECT plainto_tsquery ( 'english' , 'The Fat Rats' );

-- 来源: 2778_file_2778
SELECT querytree ( 'foo & ! bar' :: tsquery );

-- 来源: 2778_file_2778
SELECT setweight ( 'fat:2,4 cat:3 rat:5B' :: tsvector , 'A' );

-- 来源: 2778_file_2778
SELECT strip ( 'fat:2,4 cat:3 rat:5A' :: tsvector );

-- 来源: 2778_file_2778
SELECT to_tsquery ( 'english' , 'The & Fat & Rats' );

-- 来源: 2778_file_2778
SELECT to_tsvector ( 'english' , 'The Fat Rats' );

-- 来源: 2778_file_2778
SELECT to_tsvector_for_batch ( 'english' , 'The Fat Rats' );

-- 来源: 2778_file_2778
SELECT ts_headline ( 'x y z' , 'z' :: tsquery );

-- 来源: 2778_file_2778
SELECT ts_rank ( 'hello world' :: tsvector , 'world' :: tsquery );

-- 来源: 2778_file_2778
SELECT ts_rank_cd ( 'hello world' :: tsvector , 'world' :: tsquery );

-- 来源: 2778_file_2778
SELECT ts_rewrite ( 'a & b' :: tsquery , 'a' :: tsquery , 'foo|bar' :: tsquery );

-- 来源: 2778_file_2778
SELECT ts_rewrite ( 'world' :: tsquery , 'select ''world''::tsquery, ''hello''::tsquery' );

-- 来源: 2778_file_2778
SELECT ts_debug ( 'english' , 'The Brightest supernovaes' );

-- 来源: 2778_file_2778
SELECT ts_lexize ( 'english_stem' , 'stars' );

-- 来源: 2778_file_2778
SELECT ts_parse ( 'default' , 'foo - bar' );

-- 来源: 2778_file_2778
SELECT ts_parse ( 3722 , 'foo - bar' );

-- 来源: 2778_file_2778
SELECT ts_token_type ( 'default' );

-- 来源: 2778_file_2778
SELECT ts_token_type ( 3722 );

-- 来源: 2778_file_2778
SELECT ts_stat ( 'select ''hello world''::tsvector' );

-- 来源: 2779_JSON_JSONB
SELECT array_to_json('{{1,5},{99,100}}'::int[]);

-- 来源: 2779_JSON_JSONB
SELECT row_to_json(row(1,'foo'));

-- 来源: 2779_JSON_JSONB
SELECT json_array_element('[1,true,[1,[2,3]],null]',2);

-- 来源: 2779_JSON_JSONB
SELECT json_array_element_text('[1,true,[1,[2,3]],null]',2);

-- 来源: 2779_JSON_JSONB
SELECT json_object_field('{"a": {"b":"foo"}}','a');

-- 来源: 2779_JSON_JSONB
SELECT json_object_field_text('{"a": {"b":"foo"}}','a');

-- 来源: 2779_JSON_JSONB
SELECT json_extract_path('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', 'f4','f6');

-- 来源: 2779_JSON_JSONB
SELECT json_extract_path_op('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', ARRAY['f4','f6']);

-- 来源: 2779_JSON_JSONB
SELECT json_extract_path_text('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', 'f4','f6');

-- 来源: 2779_JSON_JSONB
SELECT json_extract_path_text_op('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', ARRAY['f4','f6']);

-- 来源: 2779_JSON_JSONB
SELECT json_array_elements('[1,true,[1,[2,3]],null]');

-- 来源: 2779_JSON_JSONB
SELECT * FROM json_array_elements_text('[1,true,[1,[2,3]],null]');

-- 来源: 2779_JSON_JSONB
SELECT json_array_length('[1,2,3,{"f1":1,"f2":[5,6]},4,null]');

-- 来源: 2779_JSON_JSONB
SELECT * FROM json_each('{"f1":[1,2,3],"f2":{"f3":1},"f4":null}');

-- 来源: 2779_JSON_JSONB
SELECT * FROM json_each_text('{"f1":[1,2,3],"f2":{"f3":1},"f4":null}');

-- 来源: 2779_JSON_JSONB
SELECT json_object_keys('{"f1":"abc","f2":{"f3":"a", "f4":"b"}, "f1":"abcd"}');

-- 来源: 2779_JSON_JSONB
SELECT jsonb_object_keys('{"f1":"abc","f2":{"f3":"a", "f4":"b"}, "f1":"abcd"}');

-- 来源: 2779_JSON_JSONB
SELECT * FROM json_populate_record(null::jpop,'{"a":"blurfl","x":43.2}');

-- 来源: 2779_JSON_JSONB
SELECT * FROM json_populate_record((1,1,null)::jpop,'{"a":"blurfl","x":43.2}');

-- 来源: 2779_JSON_JSONB
SELECT * FROM json_populate_recordset(null::jpop, '[{"a":1,"b":2},{"a":3,"b":4}]');

-- 来源: 2779_JSON_JSONB
SELECT value, json_typeof(value) FROM (values (json '123.4'), (json '"foo"'), (json 'true'), (json 'null'), (json '[1, 2, 3]'), (json '{"x":"foo", "y":123}'), (NULL::json)) AS data(value);

-- 来源: 2779_JSON_JSONB
SELECT json_build_array('a',1,'b',1.2,'c',true,'d',null,'e',json '{"x": 3, "y": [1,2,3]}','');

-- 来源: 2779_JSON_JSONB
SELECT json_build_object(1,2);

-- 来源: 2779_JSON_JSONB
SELECT jsonb_build_object('a',1,'b',1.2,'c',true,'d',null,'e',json '{"x": 3, "y": [1,2,3]}');

-- 来源: 2779_JSON_JSONB
SELECT jsonb_build_object();

-- 来源: 2779_JSON_JSONB
SELECT * FROM json_to_record('{"a":1,"b":"foo","c":"bar"}',true) AS x(a int, b text, d text);

-- 来源: 2779_JSON_JSONB
SELECT * FROM json_to_record('{"a": {"x": 1, "y": 2},"b":"foo","c":[1, 2]}') AS x(a json, b text, c int[]);

-- 来源: 2779_JSON_JSONB
SELECT * FROM json_to_recordset('[{"a":1,"b":"foo","d":false},{"a":2,"b":"bar","c":true}]',false) AS x(a int, b text, c boolean);

-- 来源: 2779_JSON_JSONB
SELECT json_object('{a,1,b,2,3,NULL,"d e f","a b c"}');

-- 来源: 2779_JSON_JSONB
SELECT json_object('{a,b,"a b c"}', '{a,1,1}');

-- 来源: 2779_JSON_JSONB
SELECT json_object('d',2,'c','name','b',true,'a',2,'a',NULL,'d',1);

-- 来源: 2779_JSON_JSONB
SELECT json_object('d',2,true,'name','b',true,'a',2,'aa', current_timestamp);

-- 来源: 2779_JSON_JSONB
SELECT json_array_append('[1, [2, 3]]', '$[1]', 4, '$[0]', false, '$[0]', null, '$[0]', current_timestamp);

-- 来源: 2779_JSON_JSONB
SELECT json_array();

-- 来源: 2779_JSON_JSONB
SELECT json_array(TRUE, FALSE, NULL, 114, 'text', current_timestamp);

-- 来源: 2779_JSON_JSONB
SELECT json_array_insert('[1, [2, 3]]', '$[1]', 4);

-- 来源: 2779_JSON_JSONB
SELECT json_array_insert('{"x": 1, "y": [1, 2]}', '$.y[0]', NULL, '$.y[0]', 123, '$.y[3]', current_timestamp);

-- 来源: 2779_JSON_JSONB
SELECT json_contains('[1, 2, {"x": 3}]', '{"x":3}');

-- 来源: 2779_JSON_JSONB
SELECT json_contains('[1, 2, {"x": 3},[1,2,3,4]]', '2','$[1]');

-- 来源: 2779_JSON_JSONB
SELECT json_contains('[1, 2, {"x": 3},[1,2,3,4]]', '1','$[1]');

-- 来源: 2779_JSON_JSONB
SELECT json_contains_path('[1, 2, {"x": 3}]', 'all', '$[0]', '$[1]', '$[2]');

-- 来源: 2779_JSON_JSONB
SELECT json_contains_path('[1, 2, {"x": 3}]', 'all', '$[0]', '$[1]', '$[6]');

-- 来源: 2779_JSON_JSONB
SELECT json_contains_path('[1, 2, {"x": 3}]', 'one', '$[0]', '$[1]', '$[5]');

-- 来源: 2779_JSON_JSONB
SELECT json_depth('[]');

-- 来源: 2779_JSON_JSONB
SELECT json_depth('{"s":1, "x":2,"y":[1]}');

-- 来源: 2779_JSON_JSONB
SELECT json_extract('[1, 2, {"x": 3}]', '$[2]');

-- 来源: 2779_JSON_JSONB
SELECT json_extract('["a", ["b", "c"], "d"]', '$[1]', '$[2]', '$[3]');

-- 来源: 2779_JSON_JSONB
SELECT json_insert('[1, [2, 3], {"a": [4, 5]}]', '$[3]', 2);

-- 来源: 2779_JSON_JSONB
SELECT json_insert('[1, [2, 3], {"a": [4, 5]}]', '$[10]', 10,'$[5]', 5);

-- 来源: 2779_JSON_JSONB
SELECT json_keys('{"x": 1, "y": 2, "z": 3}');

-- 来源: 2779_JSON_JSONB
SELECT json_keys('[1,2,3,{"name":"Tom"}]','$[3]');

-- 来源: 2779_JSON_JSONB
SELECT json_length('[1,2,3,4,5]');

-- 来源: 2779_JSON_JSONB
SELECT json_length('{"name":"Tom", "age":24, "like":"football"}');

-- 来源: 2779_JSON_JSONB
SELECT json_merge('[1, 2]','[2]');

-- 来源: 2779_JSON_JSONB
SELECT json_merge('{"b":"2"}','{"a":"1"}','[1,2]');

-- 来源: 2779_JSON_JSONB
SELECT json_quote('gauss');

-- 来源: 2779_JSON_JSONB
SELECT json_unquote('"gauss"');

-- 来源: 2779_JSON_JSONB
SELECT json_remove('[0, 1, 2, [3, 4]]', '$[0]', '$[2]');

-- 来源: 2779_JSON_JSONB
SELECT json_remove('[0, 1, 2, [3, 4]]', '$[0]', '$[0]','$[0]');

-- 来源: 2779_JSON_JSONB
SELECT json_replace('{"x": 1}', '$.x', 'true');

-- 来源: 2779_JSON_JSONB
SELECT json_replace('{"x": 1}', '$.x', true, '$.x', 123, '$.x', 'asd', '$.x', null);

-- 来源: 2779_JSON_JSONB
SELECT json_search('{"a":"abc","b":"abc"}','all','abc');

-- 来源: 2779_JSON_JSONB
SELECT json_search('{"a":"abc","b":"abc"}','one','abc');

-- 来源: 2779_JSON_JSONB
SELECT json_search('{"a":"abc","b":"a%c"}','one','a\%c');

-- 来源: 2779_JSON_JSONB
SELECT json_set('{"s":3}','$.s','d');

-- 来源: 2779_JSON_JSONB
SELECT json_set('{"s":3}','$.a','d','$.a','1');

-- 来源: 2779_JSON_JSONB
SELECT json_type('{"w":{"2":3},"2":4}');

-- 来源: 2779_JSON_JSONB
SELECT json_type('[1,2,2,3,3,4,4,4,4,4,4,4,4]');

-- 来源: 2779_JSON_JSONB
SELECT json_valid('{"name":"Tom"}');

-- 来源: 2779_JSON_JSONB
SELECT json_valid('[1,23,4,5,5]');

-- 来源: 2779_JSON_JSONB
SELECT json_valid('[1,23,4,5,5]}');

-- 来源: 2779_JSON_JSONB
SELECT * FROM classes;

-- 来源: 2779_JSON_JSONB
SELECT name, json_agg(score) score FROM classes GROUP BY name ORDER BY name;

-- 来源: 2779_JSON_JSONB
SELECT * FROM classes;

-- 来源: 2779_JSON_JSONB
SELECT json_object_agg(name, score) FROM classes GROUP BY name ORDER BY name;

-- 来源: 2779_JSON_JSONB
SELECT jsonb_contained('[1,2,3]', '[1,2,3,4]');

-- 来源: 2779_JSON_JSONB
SELECT jsonb_contains('[1,2,3,4]', '[1,2,3]');

-- 来源: 2779_JSON_JSONB
SELECT jsonb_exists('["1",2,3]', '1');

-- 来源: 2779_JSON_JSONB
SELECT jsonb_exists_all('["1","2",3]', '{1, 2}');

-- 来源: 2779_JSON_JSONB
SELECT jsonb_exists_any('["1","2",3]', '{1, 2, 4}');

-- 来源: 2779_JSON_JSONB
SELECT jsonb_cmp('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 2779_JSON_JSONB
SELECT jsonb_eq('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 2779_JSON_JSONB
SELECT jsonb_ne('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 2779_JSON_JSONB
SELECT jsonb_gt('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 2779_JSON_JSONB
SELECT jsonb_ge('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 2779_JSON_JSONB
SELECT jsonb_lt('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 2779_JSON_JSONB
SELECT jsonb_le('["a", "b"]', '{"a":1, "b":2}');

-- 来源: 2779_JSON_JSONB
SELECT to_json('{1,5}'::text[]);

-- 来源: 2779_JSON_JSONB
SELECT to_jsonb(array[1, 2, 3, 4]);

-- 来源: 2779_JSON_JSONB
SELECT jsonb_hash('[1,2,3]');

-- 来源: 2780_HLL
SELECT hll_hash_boolean ( FALSE );

-- 来源: 2780_HLL
SELECT hll_hash_boolean ( FALSE , 10 );

-- 来源: 2780_HLL
SELECT hll_hash_smallint ( 100 :: smallint );

-- 来源: 2780_HLL
SELECT hll_hash_smallint ( 100 :: smallint , 10 );

-- 来源: 2780_HLL
SELECT hll_hash_integer ( 0 );

-- 来源: 2780_HLL
SELECT hll_hash_integer ( 0 , 10 );

-- 来源: 2780_HLL
SELECT hll_hash_bigint ( 100 :: bigint );

-- 来源: 2780_HLL
SELECT hll_hash_bigint ( 100 :: bigint , 10 );

-- 来源: 2780_HLL
SELECT hll_hash_bytea ( E '\\x' );

-- 来源: 2780_HLL
SELECT hll_hash_bytea ( E '\\x' , 10 );

-- 来源: 2780_HLL
SELECT hll_hash_text ( 'AB' );

-- 来源: 2780_HLL
SELECT hll_hash_text ( 'AB' , 10 );

-- 来源: 2780_HLL
SELECT hll_hash_any ( 1 );

-- 来源: 2780_HLL
SELECT hll_hash_any ( '08:00:2b:01:02:03' :: macaddr );

-- 来源: 2780_HLL
SELECT hll_hash_any ( 1 , 10 );

-- 来源: 2780_HLL
SELECT hll_hashval_eq ( hll_hash_integer ( 1 ), hll_hash_integer ( 1 ));

-- 来源: 2780_HLL
SELECT hll_hashval_ne ( hll_hash_integer ( 1 ), hll_hash_integer ( 1 ));

-- 来源: 2780_HLL
SELECT hll_print ( hll_empty ());

-- 来源: 2780_HLL
SELECT hll_type ( hll_empty ());

-- 来源: 2780_HLL
SELECT hll_log2m ( hll_empty ());

-- 来源: 2780_HLL
SELECT hll_log2m ( hll_empty ( 10 ));

-- 来源: 2780_HLL
SELECT hll_log2m ( hll_empty ( - 1 ));

-- 来源: 2780_HLL
SELECT hll_log2explicit ( hll_empty ());

-- 来源: 2780_HLL
SELECT hll_log2explicit ( hll_empty ( 12 , 8 ));

-- 来源: 2780_HLL
SELECT hll_log2explicit ( hll_empty ( 12 , - 1 ));

-- 来源: 2780_HLL
SELECT hll_log2sparse ( hll_empty ());

-- 来源: 2780_HLL
SELECT hll_log2sparse ( hll_empty ( 12 , 8 , 10 ));

-- 来源: 2780_HLL
SELECT hll_log2sparse ( hll_empty ( 12 , 8 , - 1 ));

-- 来源: 2780_HLL
SELECT hll_duplicatecheck ( hll_empty ());

-- 来源: 2780_HLL
SELECT hll_duplicatecheck ( hll_empty ( 12 , 8 , 10 , 1 ));

-- 来源: 2780_HLL
SELECT hll_duplicatecheck ( hll_empty ( 12 , 8 , 10 , - 1 ));

-- 来源: 2780_HLL
SELECT hll_empty ();

-- 来源: 2780_HLL
SELECT hll_empty ( 10 );

-- 来源: 2780_HLL
SELECT hll_empty ( - 1 );

-- 来源: 2780_HLL
SELECT hll_empty ( 10 , 4 );

-- 来源: 2780_HLL
SELECT hll_empty ( 10 , - 1 );

-- 来源: 2780_HLL
SELECT hll_empty ( 10 , 4 , 8 );

-- 来源: 2780_HLL
SELECT hll_empty ( 10 , 4 , - 1 );

-- 来源: 2780_HLL
SELECT hll_empty ( 10 , 4 , 8 , 0 );

-- 来源: 2780_HLL
SELECT hll_empty ( 10 , 4 , 8 , - 1 );

-- 来源: 2780_HLL
SELECT hll_add ( hll_empty (), hll_hash_integer ( 1 ));

-- 来源: 2780_HLL
SELECT hll_add_rev ( hll_hash_integer ( 1 ), hll_empty ());

-- 来源: 2780_HLL
SELECT hll_eq ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

-- 来源: 2780_HLL
SELECT hll_ne ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

-- 来源: 2780_HLL
SELECT hll_cardinality ( hll_empty () || hll_hash_integer ( 1 ));

-- 来源: 2780_HLL
SELECT hll_union ( hll_add ( hll_empty (), hll_hash_integer ( 1 )), hll_add ( hll_empty (), hll_hash_integer ( 2 )));

-- 来源: 2780_HLL
SELECT a , # c AS cardinality FROM t_a_c_hll ORDER BY a ;

-- 来源: 2780_HLL
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), 12 )) FROM t_data ;

-- 来源: 2780_HLL
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 1 )) FROM t_data ;

-- 来源: 2780_HLL
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 6 , 10 )) FROM t_data ;

-- 来源: 2780_HLL
SELECT hll_cardinality ( hll_add_agg ( hll_hash_text ( c ), NULL , 6 , 10 , - 1 )) FROM t_data ;

-- 来源: 2780_HLL
SELECT # hll_union_agg ( c ) AS cardinality FROM t_a_c_hll ;

-- 来源: 2780_HLL
SELECT ( hll_empty () || hll_hash_integer ( 1 )) = ( hll_empty () || hll_hash_integer ( 1 ));

-- 来源: 2780_HLL
SELECT hll_hash_integer ( 1 ) = hll_hash_integer ( 1 );

-- 来源: 2780_HLL
SELECT ( hll_empty () || hll_hash_integer ( 1 )) <> ( hll_empty () || hll_hash_integer ( 2 ));

-- 来源: 2780_HLL
SELECT hll_hash_integer ( 1 ) <> hll_hash_integer ( 2 );

-- 来源: 2780_HLL
SELECT hll_empty () || hll_hash_integer ( 1 );

-- 来源: 2780_HLL
SELECT hll_hash_integer ( 1 ) || hll_empty ();

-- 来源: 2780_HLL
SELECT ( hll_empty () || hll_hash_integer ( 1 )) || ( hll_empty () || hll_hash_integer ( 2 ));

-- 来源: 2780_HLL
SELECT # ( hll_empty () || hll_hash_integer ( 1 ));

-- 来源: 2781_SEQUENCE
SELECT nextval ( 'seqDemo' );

-- 来源: 2781_SEQUENCE
SELECT seqDemo . nextval ;

-- 来源: 2781_SEQUENCE
SELECT nextval ( 'seq1' );

-- 来源: 2781_SEQUENCE
SELECT currval ( 'seq1' );

-- 来源: 2781_SEQUENCE
SELECT seq1 . currval ;

-- 来源: 2781_SEQUENCE
SELECT nextval ( 'seq1' );

-- 来源: 2781_SEQUENCE
SELECT lastval ();

-- 来源: 2781_SEQUENCE
SELECT nextval ( 'seqDemo' );

-- 来源: 2781_SEQUENCE
SELECT setval ( 'seqDemo' , 5 );

-- 来源: 2781_SEQUENCE
SELECT nextval ( 'seqDemo' );

-- 来源: 2781_SEQUENCE
SELECT setval ( 'seqDemo' , 5 , true );

-- 来源: 2781_SEQUENCE
SELECT last_insert_id ( 100 );

-- 来源: 2781_SEQUENCE
SELECT last_insert_id ();

-- 来源: 2782_file_2782
SELECT ARRAY [ 1 . 1 , 2 . 1 , 3 . 1 ]:: int [] = ARRAY [ 1 , 2 , 3 ] AS RESULT ;

-- 来源: 2782_file_2782
SELECT ARRAY [ 1 , 2 , 3 ] <> ARRAY [ 1 , 2 , 4 ] AS RESULT ;

-- 来源: 2782_file_2782
SELECT ARRAY [ 1 , 2 , 3 ] < ARRAY [ 1 , 2 , 4 ] AS RESULT ;

-- 来源: 2782_file_2782
SELECT ARRAY [ 1 , 4 , 3 ] > ARRAY [ 1 , 2 , 4 ] AS RESULT ;

-- 来源: 2782_file_2782
SELECT ARRAY [ 1 , 2 , 3 ] <= ARRAY [ 1 , 2 , 3 ] AS RESULT ;

-- 来源: 2782_file_2782
SELECT ARRAY [ 1 , 4 , 3 ] >= ARRAY [ 1 , 4 , 3 ] AS RESULT ;

-- 来源: 2782_file_2782
SELECT ARRAY [ 1 , 4 , 3 ] @> ARRAY [ 3 , 1 ] AS RESULT ;

-- 来源: 2782_file_2782
SELECT ARRAY [ 2 , 7 ] <@ ARRAY [ 1 , 7 , 4 , 2 , 6 ] AS RESULT ;

-- 来源: 2782_file_2782
SELECT ARRAY [ 1 , 4 , 3 ] && ARRAY [ 2 , 1 ] AS RESULT ;

-- 来源: 2782_file_2782
SELECT ARRAY [ 1 , 2 , 3 ] || ARRAY [ 4 , 5 , 6 ] AS RESULT ;

-- 来源: 2782_file_2782
SELECT ARRAY [ 1 , 2 , 3 ] || ARRAY [[ 4 , 5 , 6 ],[ 7 , 8 , 9 ]] AS RESULT ;

-- 来源: 2782_file_2782
SELECT 3 || ARRAY [ 4 , 5 , 6 ] AS RESULT ;

-- 来源: 2782_file_2782
SELECT ARRAY [ 4 , 5 , 6 ] || 7 AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_append ( ARRAY [ 1 , 2 ], 3 ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_prepend ( 1 , ARRAY [ 2 , 3 ]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_cat ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 4 , 5 ]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_cat ( ARRAY [[ 1 , 2 ],[ 4 , 5 ]], ARRAY [ 6 , 7 ]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_union ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_union ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_union_distinct ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_union_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_intersect ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_intersect ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_intersect_distinct ( ARRAY [ 1 , 2 , 2 ], ARRAY [ 2 , 2 , 4 , 5 ]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_intersect_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_except ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_except ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_except ( NULL , ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_except_distinct ( ARRAY [ 1 , 2 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_except_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_except_distinct ( NULL , ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_ndims ( ARRAY [[ 1 , 2 , 3 ], [ 4 , 5 , 6 ]]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_dims ( ARRAY [[ 1 , 2 , 3 ], [ 4 , 5 , 6 ]]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_length ( array [ 1 , 2 , 3 ], 1 ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_length ( array [[ 1 , 2 , 3 ],[ 4 , 5 , 6 ]], 2 ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_lower ( '[0:2]={1,2,3}' :: int [], 1 ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_upper ( ARRAY [ 1 , 8 , 3 , 7 ], 1 ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_to_string ( ARRAY [ 1 , 2 , 3 , NULL , 5 ], ',' , '*' ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT array_delete(ARRAY[1,8,3,7]) AS RESULT;

-- 来源: 2782_file_2782
SELECT array_deleteidx(ARRAY[1,2,3,4,5], 1) AS RESULT;

-- 来源: 2782_file_2782
SELECT array_extendnull(ARRAY[1,8,3,7],1) AS RESULT;

-- 来源: 2782_file_2782
SELECT array_extendnull(ARRAY[1,8,3,7],2,2) AS RESULT;

-- 来源: 2782_file_2782
SELECT array_trim(ARRAY[1,8,3,7],1) AS RESULT;

-- 来源: 2782_file_2782
SELECT array_exists(ARRAY[1,8,3,7],1) AS RESULT;

-- 来源: 2782_file_2782
SELECT array_next(ARRAY[1,8,3,7],1) AS RESULT;

-- 来源: 2782_file_2782
SELECT array_prior(ARRAY[1,8,3,7],2) AS RESULT;

-- 来源: 2782_file_2782
SELECT string_to_array ( 'xx~^~yy~^~zz' , '~^~' , 'yy' ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT string_to_array ( 'xx~^~yy~^~zz' , '~^~' , 'y' ) AS RESULT ;

-- 来源: 2782_file_2782
SELECT unnest ( ARRAY [ 1 , 2 ]) AS RESULT ;

-- 来源: 2782_file_2782
SELECT cardinality(array[[1, 2], [3, 4]]);

-- 来源: 2782_file_2782
SELECT array_positions(array[1, 2, 3, 1], 1) AS RESULT;

-- 来源: 2783_file_2783
SELECT int4range ( 1 , 5 ) = '[1,4]' :: int4range AS RESULT ;

-- 来源: 2783_file_2783
SELECT numrange ( 1 . 1 , 2 . 2 ) <> numrange ( 1 . 1 , 2 . 3 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT int4range ( 1 , 10 ) < int4range ( 2 , 3 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT int4range ( 1 , 10 ) > int4range ( 1 , 5 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT numrange ( 1 . 1 , 2 . 2 ) <= numrange ( 1 . 1 , 2 . 2 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT numrange ( 1 . 1 , 2 . 2 ) >= numrange ( 1 . 1 , 2 . 0 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT int4range ( 2 , 4 ) @> int4range ( 2 , 3 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT '[2011-01-01,2011-03-01)' :: tsrange @> '2011-01-10' :: timestamp AS RESULT ;

-- 来源: 2783_file_2783
SELECT int4range ( 2 , 4 ) <@ int4range ( 1 , 7 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT 42 <@ int4range ( 1 , 7 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT int8range ( 3 , 7 ) && int8range ( 4 , 12 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT int8range ( 1 , 10 ) << int8range ( 100 , 110 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT int8range ( 50 , 60 ) >> int8range ( 20 , 30 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT int8range ( 1 , 20 ) &< int8range ( 18 , 20 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT int8range ( 7 , 20 ) &> int8range ( 5 , 10 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT numrange ( 1 . 1 , 2 . 2 ) -|- numrange ( 2 . 2 , 3 . 3 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT numrange ( 5 , 15 ) + numrange ( 10 , 20 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT int8range ( 5 , 15 ) * int8range ( 10 , 20 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT int8range ( 5 , 15 ) - int8range ( 10 , 20 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT numrange ( 1 . 1 , 2 . 2 ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT numrange ( 1 . 1 , 2 . 2 , '()' ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT lower ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- 来源: 2783_file_2783
SELECT upper ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- 来源: 2783_file_2783
SELECT isempty ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- 来源: 2783_file_2783
SELECT lower_inc ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- 来源: 2783_file_2783
SELECT upper_inc ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

-- 来源: 2783_file_2783
SELECT lower_inf ( '(,)' :: daterange ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT upper_inf ( '(,)' :: daterange ) AS RESULT ;

-- 来源: 2783_file_2783
SELECT elem_contained_by_range ( '2' , numrange ( 1 . 1 , 2 . 2 ));

-- 来源: 2784_file_2784
SELECT sum ( a ) FROM tab ;

-- 来源: 2784_file_2784
SELECT MAX ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- 来源: 2784_file_2784
SELECT MIN ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- 来源: 2784_file_2784
SELECT AVG ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- 来源: 2784_file_2784
SELECT COUNT ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- 来源: 2784_file_2784
SELECT COUNT ( * ) FROM tpcds . inventory ;

-- 来源: 2784_file_2784
SELECT ARRAY_AGG ( sr_fee ) FROM tpcds . store_returns WHERE sr_customer_sk = 2 ;

-- 来源: 2784_file_2784
SELECT string_agg ( sr_item_sk , ',' ) FROM tpcds . store_returns WHERE sr_item_sk < 3 ;

-- 来源: 2784_file_2784
SELECT deptno , listagg ( ename , ',' ) WITHIN GROUP ( ORDER BY ename ) AS employees FROM emp GROUP BY deptno ;

-- 来源: 2784_file_2784
SELECT deptno , listagg ( mgrno , ',' ) WITHIN GROUP ( ORDER BY mgrno NULLS FIRST ) AS mgrnos FROM emp GROUP BY deptno ;

-- 来源: 2784_file_2784
SELECT job , listagg ( bonus , '($);

-- 来源: 2784_file_2784
SELECT deptno , listagg ( hiredate , ', ' ) WITHIN GROUP ( ORDER BY hiredate DESC ) AS hiredates FROM emp GROUP BY deptno ;

-- 来源: 2784_file_2784
SELECT deptno , listagg ( vacationTime , ';

-- 来源: 2784_file_2784
SELECT deptno , listagg ( job ) WITHIN GROUP ( ORDER BY job ) AS jobs FROM emp GROUP BY deptno ;

-- 来源: 2784_file_2784
SELECT deptno , mgrno , bonus , listagg ( ename , ';

-- 来源: 2784_file_2784
SELECT id , group_concat ( v separator ';

-- 来源: 2784_file_2784
SELECT id , group_concat ( id , v ) FROM t GROUP BY id ORDER BY id ASC ;

-- 来源: 2784_file_2784
SELECT id , group_concat ( v ) FROM t GROUP BY id ORDER BY id ASC ;

-- 来源: 2784_file_2784
SELECT id , group_concat ( v separator ';

-- 来源: 2784_file_2784
SELECT id , group_concat ( v separator ';

-- 来源: 2784_file_2784
SELECT id , group_concat ( hiredate separator ';

-- 来源: 2784_file_2784
SELECT id , group_concat ( v separator ';

-- 来源: 2784_file_2784
SELECT id , group_concat ( vacationt separator ';

-- 来源: 2784_file_2784
SELECT id , group_concat ( distinct v ) FROM t GROUP BY id ORDER BY id ASC ;

-- 来源: 2784_file_2784
SELECT id , group_concat ( v ORDER BY v desc ) FROM t GROUP BY id ORDER BY id ASC ;

-- 来源: 2784_file_2784
SELECT COVAR_POP ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 2784_file_2784
SELECT COVAR_SAMP ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 2784_file_2784
SELECT STDDEV_POP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 2784_file_2784
SELECT STDDEV_SAMP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 2784_file_2784
SELECT VAR_POP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 2784_file_2784
SELECT VAR_SAMP ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 2784_file_2784
SELECT BIT_AND ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 2784_file_2784
SELECT BIT_OR ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 2784_file_2784
SELECT bool_and ( 100 < 2500 );

-- 来源: 2784_file_2784
SELECT bool_or ( 100 < 2500 );

-- 来源: 2784_file_2784
SELECT CORR ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 2784_file_2784
SELECT every ( 100 < 2500 );

-- 来源: 2784_file_2784
SELECT REGR_AVGX ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 2784_file_2784
SELECT REGR_AVGY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 2784_file_2784
SELECT REGR_COUNT ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 2784_file_2784
SELECT REGR_INTERCEPT ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 2784_file_2784
SELECT REGR_R2 ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 2784_file_2784
SELECT REGR_SLOPE ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 2784_file_2784
SELECT REGR_SXX ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 2784_file_2784
SELECT REGR_SXY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 2784_file_2784
SELECT REGR_SYY ( sr_fee , sr_net_loss ) FROM tpcds . store_returns WHERE sr_customer_sk < 1000 ;

-- 来源: 2784_file_2784
SELECT STDDEV ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 2784_file_2784
SELECT VARIANCE ( inv_quantity_on_hand ) FROM tpcds . inventory WHERE inv_warehouse_sk = 1 ;

-- 来源: 2784_file_2784
SELECT CHECKSUM ( inv_quantity_on_hand ) FROM tpcds . inventory ;

-- 来源: 2784_file_2784
SELECT CHECKSUM ( inv_quantity_on_hand :: TEXT ) FROM tpcds . inventory ;

-- 来源: 2784_file_2784
SELECT CHECKSUM ( inventory :: TEXT ) FROM tpcds . inventory ;

-- 来源: 2784_file_2784
SELECT percentile_cont(0) WITHIN GROUP (ORDER BY value) FROM (VALUES (1),(2)) v(value);

-- 来源: 2784_file_2784
SELECT mode() WITHIN GROUP (ORDER BY value) FROM (values(1, 'a'), (2, 'b'), (2, 'c')) v(value, tag);

-- 来源: 2784_file_2784
SELECT mode() WITHIN GROUP (ORDER BY tag) FROM (values(1, 'a'), (2, 'b'), (2, 'c')) v(value, tag);

-- 来源: 2784_file_2784
SELECT * FROM pivot_func_test;

-- 来源: 2784_file_2784
SELECT id, pivot_func(val) FROM pivot_func_test GROUP BY id;

-- 来源: 2785_file_2785
SELECT d_moy , d_fy_week_seq , rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 2785_file_2785
SELECT d_moy , d_fy_week_seq , Row_number () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 2785_file_2785
SELECT d_moy , d_fy_week_seq , dense_rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 2785_file_2785
SELECT d_moy , d_fy_week_seq , percent_rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 2785_file_2785
SELECT d_moy , d_fy_week_seq , cume_dist () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim e_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 2785_file_2785
SELECT d_moy , d_fy_week_seq , ntile ( 3 ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 2785_file_2785
SELECT d_moy , d_fy_week_seq , lag ( d_moy , 3 , null ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 2785_file_2785
SELECT d_moy, d_fy_week_seq, lead(d_fy_week_seq,2) OVER(PARTITION BY d_moy ORDER BY d_fy_week_seq) FROM tpcds.date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1,2;

-- 来源: 2785_file_2785
SELECT d_moy , d_fy_week_seq , first_value ( d_fy_week_seq ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

-- 来源: 2785_file_2785
SELECT d_moy , d_fy_week_seq , last_value ( d_moy ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 6 ORDER BY 1 , 2 ;

-- 来源: 2785_file_2785
SELECT sales_group , sales_id , sales_amount , RATIO_TO_REPORT ( sales_amount ) OVER ( PARTITION BY sales_group ) FROM sales_int8 ORDER BY sales_id ;

-- 来源: 2785_file_2785
SELECT sales_group , sales_id , sales_amount , TO_CHAR ( RATIO_TO_REPORT ( sales_amount ) OVER (), '$999eeee' ) FROM sales ORDER BY sales_id ;

-- 来源: 2785_file_2785
SELECT d_moy , d_fy_week_seq , nth_value ( d_fy_week_seq , 6 ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 6 ORDER BY 1 , 2 ;

-- 来源: 2786_file_2786
SELECT gs_encrypt_aes128 ( 'MPPDB' , 'Asdf1234' );

-- 来源: 2786_file_2786
SELECT gs_encrypt('MPPDB', 'Asdf1234', 'sm4');

-- 来源: 2786_file_2786
SELECT gs_encrypt_bytea('MPPDB', 'Asdf1234', 'sm4_ctr_sm3');

-- 来源: 2786_file_2786
SELECT gs_decrypt_aes128 ( 'gwditQLQG8NhFw4OuoKhhQJoXojhFlYkjeG0aYdSCtLCnIUgkNwvYI04KbuhmcGZp8jWizBdR1vU9CspjuzI0lbz12A=' , '1234' );

-- 来源: 2786_file_2786
select gs_decrypt('ZBzOmaGA4Bb+coyucJ0B8AkIShqc', 'Asdf1234', 'sm4');

-- 来源: 2786_file_2786
select gs_decrypt_bytea('\x90e286971c2c70410def0a2814af4ac44c737926458b66271d9d1547bc937395ca018d7755672fa9dc3cdc6ec4a76001dc0e137f3bc5c8a5c51143561f1d09a848bfdebfec5e', 'Asdf1234', 'sm4_ctr_sm3');

-- 来源: 2786_file_2786
select aes_encrypt('huwei123','123456vfhex4dyu,vdaladhjsadad','1234567890123456');

-- 来源: 2786_file_2786
select aes_decrypt(aes_encrypt('huwei123','123456vfhex4dyu,vdaladhjsadad','1234567890123456'),'123456vfhex4dyu,vdaladhjsadad','1234567890123456');

-- 来源: 2786_file_2786
SELECT pg_catalog . gs_digest ( 'gaussdb' , 'sha256' );

-- 来源: 2786_file_2786
SELECT gs_password_deadline ();

-- 来源: 2786_file_2786
SELECT inet_server_addr ();

-- 来源: 2786_file_2786
SELECT inet_client_addr ();

-- 来源: 2788_file_2788
SELECT * FROM generate_series ( 2 , 4 );

-- 来源: 2788_file_2788
SELECT * FROM generate_series ( 5 , 1 , - 2 );

-- 来源: 2788_file_2788
SELECT * FROM generate_series ( 4 , 3 );

-- 来源: 2788_file_2788
SELECT current_date + s . a AS dates FROM generate_series ( 0 , 14 , 7 ) AS s ( a );

-- 来源: 2788_file_2788
SELECT * FROM generate_series ( '2008-03-01 00:00' :: timestamp , '2008-03-04 12:00' , '10 hours' );

-- 来源: 2788_file_2788
SELECT generate_subscripts ( '{NULL,1,NULL,2}' :: int [], 1 ) AS s ;

-- 来源: 2788_file_2788
SELECT * FROM unnest2 ( ARRAY [[ 1 , 2 ],[ 3 , 4 ]]);

-- 来源: 2789_file_2789
SELECT coalesce ( NULL , 'hello' );

-- 来源: 2789_file_2789
SELECT decode ( 'A' , 'A' , 1 , 'B' , 2 , 0 );

-- 来源: 2789_file_2789
SELECT nullif ( 'hello' , 'world' );

-- 来源: 2789_file_2789
SELECT nullif ( '1234' :: VARCHAR , 123 :: INT4 );

-- 来源: 2789_file_2789
SELECT nullif ( '1234' :: VARCHAR , '2012-12-24' :: DATE );

-- 来源: 2789_file_2789
SELECT nullif ( 1 :: bit , '1' :: MONEY );

-- 来源: 2789_file_2789
SELECT nvl ( 'hello' , 'world' );

-- 来源: 2789_file_2789
SELECT nvl2 ( 'hello' , 'world' , 'other' );

-- 来源: 2789_file_2789
SELECT greatest ( 1 * 2 , 2 - 3 , 4 - 1 );

-- 来源: 2789_file_2789
SELECT greatest ( 'HARRY' , 'HARRIOT' , 'HAROLD' );

-- 来源: 2789_file_2789
SELECT least ( 1 * 2 , 2 - 3 , 4 - 1 );

-- 来源: 2789_file_2789
SELECT least ( 'HARRY' , 'HARRIOT' , 'HAROLD' );

-- 来源: 2789_file_2789
SELECT * FROM student_demo WHERE LNNVL ( name = 'name1' );

-- 来源: 2789_file_2789
SELECT isnull ( null );

-- 来源: 2789_file_2789
SELECT isnull ( 1 );

-- 来源: 2789_file_2789
select if ( 2 > 3 , 'true' , 'false' );

-- 来源: 2789_file_2789
select if ( null , 'not null' , 'is null' );

-- 来源: 2789_file_2789
select ifnull ( '' , null ) is null as a ;

-- 来源: 2789_file_2789
select ifnull ( null , null ) is null as a ;

-- 来源: 2789_file_2789
select ifnull ( null , 'A' ) as a ;

-- 来源: 2790_file_2790
SELECT current_query ();

-- 来源: 2790_file_2790
SELECT current_schema ();

-- 来源: 2790_file_2790
SELECT current_schemas ( true );

-- 来源: 2790_file_2790
SELECT database ();

-- 来源: 2790_file_2790
SELECT current_user ;

-- 来源: 2790_file_2790
SELECT definer_current_user ();

-- 来源: 2790_file_2790
SELECT pg_current_sessionid ();

-- 来源: 2790_file_2790
select pg_current_sessid();

-- 来源: 2790_file_2790
SELECT pg_current_userid();

-- 来源: 2790_file_2790
SELECT working_version_num ();

-- 来源: 2790_file_2790
select tablespace_oid_name ( 1663 );

-- 来源: 2790_file_2790
SELECT inet_client_addr ();

-- 来源: 2790_file_2790
SELECT inet_client_port ();

-- 来源: 2790_file_2790
SELECT inet_server_addr ();

-- 来源: 2790_file_2790
SELECT inet_server_port ();

-- 来源: 2790_file_2790
SELECT pg_backend_pid ();

-- 来源: 2790_file_2790
SELECT pg_conf_load_time ();

-- 来源: 2790_file_2790
SELECT pg_my_temp_schema ();

-- 来源: 2790_file_2790
SELECT pg_is_other_temp_schema ( 25356 );

-- 来源: 2790_file_2790
SELECT pg_listening_channels ();

-- 来源: 2790_file_2790
SELECT pg_postmaster_start_time ();

-- 来源: 2790_file_2790
select * from pg_get_ruledef(24828);

-- 来源: 2790_file_2790
select sessionid2pid ( sessid :: cstring ) from gs_session_stat limit 2 ;

-- 来源: 2790_file_2790
SELECT session_context ( 'USERENV' , 'CURRENT_SCHEMA' );

-- 来源: 2790_file_2790
SELECT pg_trigger_depth ();

-- 来源: 2790_file_2790
SELECT session_user ;

-- 来源: 2790_file_2790
SELECT user ;

-- 来源: 2790_file_2790
select getpgusername ();

-- 来源: 2790_file_2790
select getdatabaseencoding ();

-- 来源: 2790_file_2790
select version();

-- 来源: 2790_file_2790
select opengauss_version ();

-- 来源: 2790_file_2790
select gs_deployment ();

-- 来源: 2790_file_2790
SELECT get_hostname ();

-- 来源: 2790_file_2790
SELECT get_nodename ();

-- 来源: 2790_file_2790
SELECT get_nodeinfo ( 'node_type' );

-- 来源: 2790_file_2790
SELECT get_nodeinfo ( 'node_name' );

-- 来源: 2790_file_2790
SELECT get_schema_oid ( 'public' );

-- 来源: 2790_file_2790
SELECT has_table_privilege ( 'tpcds.web_site' , 'select' );

-- 来源: 2790_file_2790
SELECT has_table_privilege ( 'omm' , 'tpcds.web_site' , 'select,INSERT WITH GRANT OPTION ' );

-- 来源: 2790_file_2790
SELECT relname FROM pg_class WHERE pg_table_is_visible ( oid );

-- 来源: 2790_file_2790
SELECT format_type (( SELECT oid FROM pg_type WHERE typname = 'varchar' ), 10 );

-- 来源: 2790_file_2790
select pg_check_authid(1);

-- 来源: 2790_file_2790
select * from pg_get_functiondef(598);

-- 来源: 2790_file_2790
select * from pg_get_indexdef(16416);

-- 来源: 2790_file_2790
select oid from pg_class where relname = 'index_sales';

-- 来源: 2790_file_2790
select * from pg_get_indexdef(24632, true);

-- 来源: 2790_file_2790
select * from pg_get_indexdef(24632, false);

-- 来源: 2790_file_2790
select * from pg_get_indexdef(16416, 0, false);

-- 来源: 2790_file_2790
select * from pg_get_indexdef(16416, 1, false);

-- 来源: 2790_file_2790
select pg_check_authid(20);

-- 来源: 2790_file_2790
select * from pg_get_tabledef(16384);

-- 来源: 2790_file_2790
select * from pg_get_tabledef('t1');

-- 来源: 2790_file_2790
SELECT pg_typeof ( 33 );

-- 来源: 2790_file_2790
SELECT typlen FROM pg_type WHERE oid = pg_typeof ( 33 );

-- 来源: 2790_file_2790
SELECT collation for ( description ) FROM pg_description LIMIT 1 ;

-- 来源: 2790_file_2790
select * from pg_get_serial_sequence('t1', 'c1');

-- 来源: 2790_file_2790
select * from pg_sequence_parameters(16420);

-- 来源: 2790_file_2790
select * from gs_get_kernel_info();

-- 来源: 2792_file_2792
SELECT current_setting ( 'datestyle' );

-- 来源: 2792_file_2792
SELECT set_config ( 'log_statement_stats' , 'off' , false );

-- 来源: 2793_file_2793
SELECT pg_ls_dir ( './' );

-- 来源: 2793_file_2793
SELECT pg_read_file ( 'postmaster.pid' , 0 , 100 );

-- 来源: 2793_file_2793
SELECT convert_from ( pg_read_binary_file ( 'filename' ), 'UTF8' );

-- 来源: 2793_file_2793
SELECT * FROM pg_stat_file ( 'filename' );

-- 来源: 2793_file_2793
SELECT ( pg_stat_file ( 'filename' )). modification ;

-- 来源: 2793_file_2793
SELECT convert_from ( pg_read_binary_file ( 'postmaster.pid' ), 'UTF8' );

-- 来源: 2793_file_2793
SELECT * FROM pg_stat_file ( 'postmaster.pid' );

-- 来源: 2793_file_2793
SELECT ( pg_stat_file ( 'postmaster.pid' )). modification ;

-- 来源: 2794_file_2794
SELECT pid from pg_stat_activity ;

-- 来源: 2794_file_2794
SELECT pg_terminate_backend ( 140657876268816 );

-- 来源: 2795_file_2795
SELECT pg_start_backup ( 'label_goes_here' );

-- 来源: 2795_file_2795
SELECT * FROM pg_xlogfile_name_offset ( pg_stop_backup ());

-- 来源: 2796_file_2796
select * from pg_create_physical_replication_slot_extern ( 'uuid' , false , 'OBS;

-- 来源: 2796_file_2796
select * from pg_create_physical_replication_slot_extern ( 'uuid' , false , 'NAS;

-- 来源: 2796_file_2796
select gs_set_obs_delete_location('0/54000000');

-- 来源: 2799_file_2799
SELECT pg_column_size ( 1 );

-- 来源: 2799_file_2799
SELECT pg_database_size ( 'testdb' );

-- 来源: 2799_file_2799
select get_db_source_datasize ();

-- 来源: 2799_file_2799
SELECT datalength(1);

-- 来源: 2801_file_2801
select * from pg_create_logical_replication_slot('slot_lsn','mppdb_decoding',0);

-- 来源: 2801_file_2801
select * from pg_create_logical_replication_slot('slot_csn','mppdb_decoding',1);

-- 来源: 2801_file_2801
select * from pg_logical_slot_peek_changes('slot_lsn',NULL,4096,'skip-empty-xacts','on');

-- 来源: 2801_file_2801
select * from pg_logical_slot_peek_changes('slot_csn',NULL,4096,'skip-empty-xacts','on');

-- 来源: 2801_file_2801
SELECT pg_current_xlog_location();

-- 来源: 2801_file_2801
SELECT * FROM pg_logical_get_area_changes('0/5ECBCD48', NULL, NULL, 'sql_decoding', NULL);

-- 来源: 2801_file_2801
SELECT pg_current_xlog_location();

-- 来源: 2801_file_2801
SELECT * FROM pg_logical_get_area_changes('0/5F62CFE8', NULL, NULL, 'sql_decoding', NULL, 'skip-generated-columns', 'on');

-- 来源: 2801_file_2801
select * from pg_get_replication_slots();

-- 来源: 2801_file_2801
select * from gs_get_parallel_decode_status();

-- 来源: 2801_file_2801
select * from gs_get_slot_decoded_wal_time('replication_slot');

-- 来源: 2801_file_2801
select * from gs_logical_parallel_decode_status('replication_slot');

-- 来源: 2801_file_2801
select * from gs_logical_parallel_decode_status('replication_slot');

-- 来源: 2801_file_2801
select * from gs_logical_parallel_decode_reset_status('replication_slot');

-- 来源: 2801_file_2801
select * from gs_logical_parallel_decode_status('replication_slot');

-- 来源: 2801_file_2801
select * from gs_logical_decode_start_observe('replication_slot',20,5);

-- 来源: 2801_file_2801
select * from gs_logical_decode_start_observe('replication_slot',20,5);

-- 来源: 2801_file_2801
select * from gs_logical_decode_stop_observe('replication_slot');

-- 来源: 2801_file_2801
select * from gs_logical_decode_stop_observe('replication_slot');

-- 来源: 2801_file_2801
select * from gs_logical_decode_observe_data('replication_slot');

-- 来源: 2801_file_2801
select * from gs_logical_decode_observe('replication_slot');

-- 来源: 2801_file_2801
select * from gs_logical_decode_observe_status('replication_slot');

-- 来源: 2801_file_2801
select * from gs_logical_decode_observe_status('replication_slo');

-- 来源: 2801_file_2801
select * from gs_logical_decode_stop_observe('replication_slot');

-- 来源: 2801_file_2801
select * from gs_logical_decode_observe_status('replication_slot');

-- 来源: 2801_file_2801
select * from gs_get_parallel_decode_thread_info();

-- 来源: 2802_file_2802
select * from gs_seg_dump_page('pg_default', 1, 1024, 4157);

-- 来源: 2802_file_2802
select * from gs_seg_dump_page(16788, 1024, 0);

-- 来源: 2802_file_2802
select * from gs_seg_get_spc_location('pg_default', 1024, 4157, 0);

-- 来源: 2802_file_2802
select * from gs_seg_get_spc_location(24578,1024,0);

-- 来源: 2802_file_2802
select * from gs_seg_get_location(4157);

-- 来源: 2802_file_2802
select * from gs_seg_get_segment_layout();

-- 来源: 2802_file_2802
select * from gs_seg_get_datafile_layout();

-- 来源: 2802_file_2802
select * from gs_seg_get_slice_layout(1,1024, 0);

-- 来源: 2802_file_2802
select * from gs_seg_get_segment('pg_default', 1024, 4157);

-- 来源: 2802_file_2802
select * from gs_seg_get_segment(16768, 1024);

-- 来源: 2802_file_2802
select * from gs_seg_get_extents('pg_default', 1024, 4157);

-- 来源: 2802_file_2802
select * from gs_seg_get_extents(16768, 1024);

-- 来源: 2802_file_2802
select * from gs_seg_free_spc_remain_segment('pg_default', 1, 4159);

-- 来源: 2802_file_2802
select * from gs_seg_free_spc_remain_extent('pg_default', 1, 0, 4159);

-- 来源: 2802_file_2802
select * from gs_seg_get_datafiles();

-- 来源: 2802_file_2802
select * from gs_seg_get_spc_extents('pg_default', 1,1024, 0);

-- 来源: 2804_file_2804
select pg_stat_get_env();

-- 来源: 2804_file_2804
select gs_parse_page_bypath('base/16603/16394', -1, 'btree', false);

-- 来源: 2804_file_2804
select gs_parse_page_bypath('base/12828/16771_vm', -1, 'vm', false);

-- 来源: 2804_file_2804
select gs_parse_page_bypath('000000000000', 0, 'clog', false);

-- 来源: 2804_file_2804
select gs_parse_page_bypath('base/12828/16777', -10, 'heap', false);

-- 来源: 2804_file_2804
select * from gs_stat_space(false);

-- 来源: 2804_file_2804
select * from gs_index_dump_read(0, 'all');

-- 来源: 2804_file_2804
select * from gs_index_dump_read(1, 'all');

-- 来源: 2804_file_2804
select * from gs_parse_page_bypath('base/15833/16768', 0, 'uheap', false);

-- 来源: 2804_file_2804
select * from gs_xlogdump_bylastlsn('0/4593570', -1, 'uheap');

-- 来源: 2804_file_2804
select * from gs_xlogdump_bylastlsn('0/4593570', 0, 'ubtree');

-- 来源: 2804_file_2804
SELECT query,unique_query_id,start_time,finish_time FROM dbe_perf.statement_history;

-- 来源: 2804_file_2804
SELECT query FROM dbe_perf.get_full_sql_by_parent_id_and_timestamp(536458473,'2023-06-02 17:40:59.028144+08','2023-06-02 17:40:59.032027+08');

-- 来源: 2805_Undo
select * from gs_global_config where name like '%undostoragetype%';

-- 来源: 2805_Undo
select * from gs_stat_undo(true);

-- 来源: 2805_Undo
select * from gs_stat_undo(false);

-- 来源: 2805_Undo
select * from gs_undo_meta_dump_zone(-1,true);

-- 来源: 2805_Undo
select * from gs_undo_translot_dump_slot(-1,true);

-- 来源: 2805_Undo
select * from gs_undo_translot_dump_xid('15758',false);

-- 来源: 2805_Undo
select * from gs_undo_dump_record('0000000000000042');

-- 来源: 2805_Undo
select * from gs_undo_dump_xid('15779');

-- 来源: 2805_Undo
select * from gs_verify_undo_record('urp', 24, 24, 1);

-- 来源: 2805_Undo
select * from gs_verify_undo_record('zone', 0, 2, 1);

-- 来源: 2805_Undo
select * from gs_verify_undo_slot('zone', 0, 2, 1);

-- 来源: 2805_Undo
select * from gs_verify_undo_meta('all', 0, 2, 1);

-- 来源: 2808_file_2808
select pg_stat_get_role_name(10);

-- 来源: 2808_file_2808
select * from pg_stat_get_activity(139881386280704);

-- 来源: 2808_file_2808
select * from gs_stack ( 139663481165568 );

-- 来源: 2808_file_2808
select * from gs_stack ();

-- 来源: 2808_file_2808
SELECT * FROM gs_perf_start ( 10 , 100 );

-- 来源: 2808_file_2808
SELECT * FROM gs_perf_query () WHERE overhead > 2 AND level < 10 ;

-- 来源: 2808_file_2808
SELECT * FROM gs_perf_clean ();

-- 来源: 2808_file_2808
select sessionid from pg_stat_activity where usename = 'testuser';

-- 来源: 2808_file_2808
select * from gs_session_all_settings(788861) where name = 'work_mem';

-- 来源: 2808_file_2808
select * from gs_session_all_settings() where name = 'work_mem';

-- 来源: 2808_file_2808
select * from gs_local_wal_preparse_statistics();

-- 来源: 2808_file_2808
SELECT * FROM GS_WLM_RESPOOL_CPU_INFO ();

-- 来源: 2808_file_2808
SELECT * FROM GS_WLM_RESPOOL_CONNECTION_INFO ();

-- 来源: 2808_file_2808
SELECT * FROM GS_WLM_RESPOOL_MEMORY_INFO ();

-- 来源: 2808_file_2808
SELECT * FROM GS_WLM_RESPOOL_CONCURRENCY_INFO();

-- 来源: 2808_file_2808
SELECT * FROM GS_WLM_RESPOOL_IO_INFO();

-- 来源: 2808_file_2808
SELECT * FROM GS_WLM_USER_SPACE_INFO ();

-- 来源: 2808_file_2808
SELECT * FROM GS_WLM_SESSION_IO_INFO ();

-- 来源: 2808_file_2808
SELECT * FROM GS_WLM_SESSION_MEMORY_INFO ();

-- 来源: 2808_file_2808
select * from gs_hot_standby_space_info();

-- 来源: 2808_file_2808
SELECT * FROM exrto_file_read_stat();

-- 来源: 2808_file_2808
SELECT * FROM gs_exrto_recycle_info();

-- 来源: 2808_file_2808
SELECT * FROM gs_stat_get_db_conflict_all(12738);

-- 来源: 2808_file_2808
SELECT * FROM gs_redo_stat_info();

-- 来源: 2808_file_2808
SELECT * FROM gs_recovery_conflict_waitevent_info();

-- 来源: 2808_file_2808
SELECT * FROM gs_display_delay_ddl_info();

-- 来源: 2808_file_2808
SELECT * FROM gs_stat_all_partitions;

-- 来源: 2808_file_2808
SELECT * FROM gs_statio_all_partitions;

-- 来源: 2808_file_2808
SELECT * FROM gs_stat_get_partition_stats(16952);

-- 来源: 2808_file_2808
SELECT * FROM gs_stat_xact_all_partitions;

-- 来源: 2808_file_2808
SELECT * FROM gs_stat_get_xact_partition_stats(16952);

-- 来源: 2809_file_2809
select pg_get_triggerdef(oid) from pg_trigger;

-- 来源: 2809_file_2809
select pg_get_triggerdef(oid,true) from pg_trigger;

-- 来源: 2809_file_2809
select pg_get_triggerdef(oid,false) from pg_trigger;

-- 来源: 2810_HashFunc
select ora_hash ( 123 );

-- 来源: 2810_HashFunc
select ora_hash ( '123' );

-- 来源: 2810_HashFunc
select ora_hash ( 'sample' );

-- 来源: 2810_HashFunc
select ora_hash ( to_date ( '2012-1-2' , 'yyyy-mm-dd' ));

-- 来源: 2810_HashFunc
select ora_hash ( 123 , 234 );

-- 来源: 2810_HashFunc
select ora_hash ( '123' , 234 );

-- 来源: 2810_HashFunc
select ora_hash ( 'sample' , 234 );

-- 来源: 2810_HashFunc
select ora_hash ( to_date ( '2012-1-2' , 'yyyy-mm-dd' ), 234 );

-- 来源: 2810_HashFunc
select hash_array ( ARRAY [[ 1 , 2 , 3 ],[ 1 , 2 , 3 ]]);

-- 来源: 2810_HashFunc
select hash_numeric ( 30 );

-- 来源: 2810_HashFunc
select hash_range ( numrange ( 1 . 1 , 2 . 2 ));

-- 来源: 2810_HashFunc
select hashbpchar ( 'hello' );

-- 来源: 2810_HashFunc
select hashbpchar ( 'hello' );

-- 来源: 2810_HashFunc
select hashchar ( 'true' );

-- 来源: 2810_HashFunc
select hashfloat4 ( 12 . 1234 );

-- 来源: 2810_HashFunc
select hashfloat8 ( 123456 . 1234 );

-- 来源: 2810_HashFunc
select hashinet ( '127.0.0.1' :: inet );

-- 来源: 2810_HashFunc
select hashint1 ( 20 );

-- 来源: 2810_HashFunc
select hashint2(20000);

-- 来源: 2812_file_2812
select * from pg_get_gtt_relstats(74069);

-- 来源: 2812_file_2812
select * from pg_get_gtt_statistics(74069,1,''::text);

-- 来源: 2812_file_2812
select * from pg_gtt_attached_pid(74069);

-- 来源: 2812_file_2812
select * from pg_list_gtt_relfrozenxids();

-- 来源: 2816_file_2816
select * , sys_connect_by_path ( name , '-' ) from connect_table start with id = 1 connect by prior id = pid ;

-- 来源: 2816_file_2816
select * , connect_by_root ( name ) from connect_table start with id = 1 connect by prior id = pid ;

-- 来源: 2819_Global SysCache
select * from gs_gsc_catalog_detail(16574, 1260);

-- 来源: 2819_Global SysCache
select * from gs_gsc_clean();

-- 来源: 2819_Global SysCache
select * from gs_gsc_dbstat_info();

-- 来源: 2820_file_2820
select * from gs_verify_data_file();

-- 来源: 2820_file_2820
select * from gs_verify_data_file(true);

-- 来源: 2820_file_2820
select * from gs_repair_file(16554,'base/16552/24745',360);

-- 来源: 2820_file_2820
select * from local_bad_block_info();

-- 来源: 2820_file_2820
select * from local_clear_bad_block_info();

-- 来源: 2820_file_2820
select * from gs_verify_and_tryrepair_page('base/16552/24745',0,false,false);

-- 来源: 2820_file_2820
select * from gs_repair_page('base/16552/24745',0,false,60);

-- 来源: 2820_file_2820
select gs_edit_page_bypath('base/15808/25075',0,16,'0x1FFF', 2, false, 'page');

-- 来源: 2820_file_2820
select gs_edit_page_bypath('base/15808/25075', 0,16,'@1231!', 8, false, 'page');

-- 来源: 2820_file_2820
select gs_edit_page_bypath('/pg_log_dir/dump/1663_15808_25075_0.editpage', 0,16,'0x1FFF', 2, true, 'page');

-- 来源: 2820_file_2820
select * from gs_repair_page_bypath('pg_log/dump/1663_15991_16767_0.editpage', 0, 'base/15991/16767', 0, 'page');

-- 来源: 2820_file_2820
select * from gs_repair_page_bypath('standby', 0, 'base/15990/16768', 0, 'page');

-- 来源: 2820_file_2820
select * from gs_repair_page_bypath('init_block', 0, 'base/15990/16768', 0, 'page');

-- 来源: 2820_file_2820
select * from gs_repair_undo_byzone(4);

-- 来源: 2820_file_2820
select * from gs_repair_undo_byzone(78);

-- 来源: 2820_file_2820
select * from gs_repair_undo_byzone(0);

-- 来源: 2820_file_2820
select * from gs_verify_urq(16387, 0, 1, 'free queue');

-- 来源: 2820_file_2820
select * from gs_verify_urq(16387, 0, 1, 'empty queue');

-- 来源: 2820_file_2820
SELECT * FROM gs_urq_dump_stat(16387, 0);

-- 来源: 2820_file_2820
SELECT gs_urq_dump_stat(17260,0);

-- 来源: 2820_file_2820
select * from gs_repair_urq(16387, 0);

-- 来源: 2820_file_2820
select * from gs_get_standby_bad_block_info();

-- 来源: 2821_XML
SELECT XMLPARSE ( DOCUMENT '<?xml version="1.0"?><book><title>Manual</title><chapter>...</chapter></book>' );

-- 来源: 2821_XML
SELECT XMLPARSE ( CONTENT 'abc<foo>bar</foo><bar>foo</bar>' );

-- 来源: 2821_XML
SELECT XMLPARSE ( CONTENT 'abc<foo>bar</foo' wellformed );

-- 来源: 2821_XML
SELECT XMLSERIALIZE ( CONTENT 'good' AS CHAR ( 10 ));

-- 来源: 2821_XML
SELECT xmlserialize ( DOCUMENT '<head>bad</head>' as text );

-- 来源: 2821_XML
SELECT xmlcomment ( 'hello' );

-- 来源: 2821_XML
select XMLCONCAT(('<?xml version="1.0" encoding="GB2312" standalone="no"?><bar>foo</bar>'),('<?xml version="1.0" encoding="GB2312" standalone="no" ?><bar>foo</bar>')) ;

-- 来源: 2821_XML
select XMLCONCAT('abc>');

-- 来源: 2821_XML
select XMLCONCAT(('<?xml version="1.0" encoding="GB2312" standalone="no"?><bar>foo</bar>'),('<?xml version="1.0" encoding="GB2312" standalone="no" ?><bar>foo</bar>')) ;

-- 来源: 2821_XML
select XMLCONCAT('abc>');

-- 来源: 2821_XML
SELECT xmlelement ( name foo );

-- 来源: 2821_XML
SELECT xmlelement ( "entityescaping<>" , 'a$><&"b' );

-- 来源: 2821_XML
SELECT xmlelement ( entityescaping "entityescaping<>" , 'a$><&"b' );

-- 来源: 2821_XML
SELECT xmlelement ( noentityescaping "entityescaping<>" , 'a$><&"b' );

-- 来源: 2821_XML
SELECT xmlelement(" entityescaping <> ", '<abc/>' b);

-- 来源: 2821_XML
SELECT xmlelement(" entityescaping <> ", '<abc/>' as b);

-- 来源: 2821_XML
SELECT xmlelement(" entityescaping <> ", xml('<abc/>') b);

-- 来源: 2821_XML
SELECT xmlelement(" entityescaping <> ", xml('<abc/>') as b);

-- 来源: 2821_XML
SELECT xmlelement(" entityescaping <> ", xmlattributes('entityescaping<>' " entityescaping <> "));

-- 来源: 2821_XML
SELECT xmlelement(name " entityescaping <> ", xmlattributes(entityescaping 'entityescaping<>' " entityescaping <> "));

-- 来源: 2821_XML
SELECT xmlelement(" entityescaping <> ", xmlattributes(noentityescaping 'entityescaping<>' " entityescaping <> "));

-- 来源: 2821_XML
SELECT xmlforest ( 'abc' AS foo , 123 AS bar );

-- 来源: 2821_XML
SELECT xmlpi ( name php , 'echo "hello world";

-- 来源: 2821_XML
SELECT xmlroot ( '<?xml version="1.1"?><content>abc</content>' , version '1.0' , standalone yes );

-- 来源: 2821_XML
SELECT xmlagg ( data ) FROM xmltest ;

-- 来源: 2821_XML
SELECT xmlagg ( data ) FROM xmltest ;

-- 来源: 2821_XML
SELECT xmlagg ( data ) FROM xmltest ;

-- 来源: 2821_XML
SELECT xmlagg ( data order by id desc ) FROM xmltest ;

-- 来源: 2821_XML
SELECT xmlexists ( '//town[text() = ''Toronto'']' PASSING BY REF '<towns><town>Toronto</town><town>Ottawa</town></towns>' );

-- 来源: 2821_XML
SELECT xml_is_well_formed ( '<>' );

-- 来源: 2821_XML
SELECT xml_is_well_formed_document ( '<pg:foo xmlns:pg="http://postgresql.org/stuff">bar</pg:foo>' );

-- 来源: 2821_XML
select xml_is_well_formed_content ( 'k' );

-- 来源: 2821_XML
SELECT xpath ( '/my:a/text()' , '<my:a xmlns:my="http://example.com">test</my:a>' , ARRAY [ ARRAY [ 'my' , 'http://example.com' ]]);

-- 来源: 2821_XML
SELECT xpath_exists ( '/my:a/text()' , '<my:a xmlns:my="http://example.com">test</my:a>' , ARRAY [ ARRAY [ 'my' , 'http://example.com' ]]);

-- 来源: 2821_XML
SELECT query_to_xml ( 'SELECT * FROM testxmlschema.test1' , false , false , '' );

-- 来源: 2821_XML
SELECT query_to_xmlschema ( 'SELECT * FROM testxmlschema.test1' , false , false , '' );

-- 来源: 2821_XML
SELECT query_to_xml_and_xmlschema ( 'SELECT * FROM testxmlschema.test1' , true , true , '' );

-- 来源: 2821_XML
SELECT cursor_to_xml ( 'xc' :: refcursor , 5 , false , true , '' );

-- 来源: 2821_XML
SELECT cursor_to_xmlschema ( 'xc' :: refcursor , true , false , '' );

-- 来源: 2821_XML
SELECT schema_to_xml ( 'testxmlschema' , false , true , '' );

-- 来源: 2821_XML
SELECT schema_to_xmlschema ( 'testxmlschema' , false , true , '' );

-- 来源: 2821_XML
SELECT schema_to_xml_and_xmlschema ( 'testxmlschema' , true , true , 'foo' );

-- 来源: 2821_XML
SELECT database_to_xml ( true , true , 'test' );

-- 来源: 2821_XML
SELECT database_to_xmlschema ( true , true , 'test' );

-- 来源: 2821_XML
SELECT database_to_xml_and_xmlschema ( true , true , 'test' );

-- 来源: 2821_XML
SELECT table_to_xml ( 'testxmlschema.test1' , false , false , '' );

-- 来源: 2821_XML
SELECT table_to_xmlschema ( 'testxmlschema.test1' , false , false , '' );

-- 来源: 2821_XML
SELECT table_to_xml_and_xmlschema ( 'testxmlschema.test1' , false , false , '' );

-- 来源: 2821_XML
SELECT getclobval ( xmlparse ( document '<a>123</a>' ));

-- 来源: 2821_XML
SELECT getstringval(xmlparse(document '<a>123<b>456</b></a>'));

-- 来源: 2821_XML
SELECT xmlsequence(xml('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>'));

-- 来源: 2822_XMLTYPE
SELECT createxml ( '<a>123</a>' );

-- 来源: 2822_XMLTYPE
SELECT xmltype . createxml ( '<a>123</a>' );

-- 来源: 2822_XMLTYPE
select xmltype ( '<a>123<b>456</b></a>' ). extract ( '/a/b' ). getstringval ();

-- 来源: 2822_XMLTYPE
select getstringval ( extractxml ( xmltype ( '<a>123<b>456</b></a>' ), '/a/b' ));

-- 来源: 2822_XMLTYPE
SELECT getblobval ( xmltype ( '<asd/>' ), 7 );

-- 来源: 2822_XMLTYPE
select xmltype ( '<asd/>' ). getblobVal ( 7 );

-- 来源: 2822_XMLTYPE
SELECT getclobval ( xmltype ( '<a>123</a>' ));

-- 来源: 2822_XMLTYPE
SELECT xmltype ( '<a>123</a>' ). getclobval ();

-- 来源: 2822_XMLTYPE
SELECT getnumberval ( xmltype ( '<a>123</a>' ). extract ( '/a/text()' ));

-- 来源: 2822_XMLTYPE
SELECT xmltype ( '<a>123</a>' ). extract ( '/a/text()' ). getnumberval ();

-- 来源: 2822_XMLTYPE
SELECT isfragment ( xmltype ( '<a>123</a>' ));

-- 来源: 2822_XMLTYPE
SELECT xmltype ( '<a>123</a>' ). isfragment ();

-- 来源: 2822_XMLTYPE
SELECT xmltype ( '<a>123</a>' );

-- 来源: 2822_XMLTYPE
select getstringval('<a>123<b>456</b></a>');

-- 来源: 2822_XMLTYPE
select xmltype('<a>123<b>456</b></a>').getstringval();

-- 来源: 2822_XMLTYPE
select getrootelement('<a>123<b>456</b></a>');

-- 来源: 2822_XMLTYPE
select xmltype('<a>123<b>456</b></a>').getrootelement();

-- 来源: 2822_XMLTYPE
select getnamespace('<c:a xmlns:c="asd">123<d:b xmlns:d="qwe">456</d:b></c:a>');

-- 来源: 2822_XMLTYPE
select xmltype('<c:a xmlns:c="asd">123<d:b xmlns:d="qwe">456</d:b></c:a>').getnamespace();

-- 来源: 2822_XMLTYPE
select existsnode('<a>123<b>456</b></a>','/a/b');

-- 来源: 2822_XMLTYPE
select xmltype('<a>123<b>456</b></a>').existsnode('/a/b');

-- 来源: 2822_XMLTYPE
select existsnode('<a:b xmlns:a="asd">123<c>456</c></a:b>','/a:b/c','xmlns:a="asd"');

-- 来源: 2822_XMLTYPE
select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').existsnode('/a:b/c','xmlns:a="asd"');

-- 来源: 2822_XMLTYPE
select extractxml('<a>123<b>456</b></a>','/a/b');

-- 来源: 2822_XMLTYPE
select xmltype('<a>123<b>456</b></a>').extract('/a/b');

-- 来源: 2822_XMLTYPE
select xmltype('<a>123<b>456</b></a>').extractxml('/a/b');

-- 来源: 2822_XMLTYPE
select extractxml('<a:b xmlns:a="asd">123<c>456</c></a:b>','/a:b','xmlns:a="asd"');

-- 来源: 2822_XMLTYPE
select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').extract('/a:b','xmlns:a="asd"');

-- 来源: 2822_XMLTYPE
select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').extractxml('/a:b','xmlns:a="asd"');

-- 来源: 2822_XMLTYPE
SELECT xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>'));

-- 来源: 2822_XMLTYPE
SELECT unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//title/text()'))) AS title , unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//author/text()'))) AS author;

-- 来源: 2822_XMLTYPE
SELECT array_to_json(array_agg(row_to_json(t))) FROM ( SELECT unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//title/text()'))) AS title , unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinnger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//author/text()'))) AS author ) t;

-- 来源: 2823_Global Plsql Cache
select * from gs_glc_memory_detail where type='func' or type='pkg';

-- 来源: 2823_Global Plsql Cache
select invalidate_plsql_object('public','f3','function');

-- 来源: 2823_Global Plsql Cache
select invalidate_plsql_object();

-- 来源: 2824_file_2824
select * from cross_test ;

-- 来源: 2824_file_2824
select * from crosstab ( 'select group_, id, var from cross_test order by 1, 2;

-- 来源: 2824_file_2824
select * from crosstab2 ( 'select group_, id, var from cross_test order by 1, 2;

-- 来源: 2824_file_2824
select * from crosstab ( 'select group_, id, var from cross_test order by 1, 2;

-- 来源: 2825_file_2825
select uuid ();

-- 来源: 2825_file_2825
SELECT uuid_short ();

-- 来源: 2826_SQL
select gs_add_workload_rule ( 'sqlid' , 'rule for one query' , '' , now (), '' , 20 , '{id=32413214}' );

-- 来源: 2826_SQL
select gs_add_workload_rule ( 'select' , 'rule for select' , '{db1, db2}' , '' , '' , 100 , '{tb1, tb2}' );

-- 来源: 2826_SQL
select gs_add_workload_rule ( 'resource' , 'rule for resource' , '{}' , '' , '' , 20 , '{cpu-80}' );

-- 来源: 2826_SQL
select gs_update_workload_rule ( 2 , 'rule for select 2' , '{db1}' , now (), '' , 50 , '{tb1}' );

-- 来源: 2826_SQL
select gs_delete_workload_rule ( 3 );

-- 来源: 2826_SQL
select * from gs_get_workload_rule_stat ( 1 );

-- 来源: 2826_SQL
select * from gs_get_workload_rule_stat ( - 1 );

-- 来源: 2829_file_2829
SELECT 2 BETWEEN 1 AND 3 AS RESULT ;

-- 来源: 2829_file_2829
SELECT 2 >= 1 AND 2 <= 3 AS RESULT ;

-- 来源: 2829_file_2829
SELECT 2 NOT BETWEEN 1 AND 3 AS RESULT ;

-- 来源: 2829_file_2829
SELECT 2 < 1 OR 2 > 3 AS RESULT ;

-- 来源: 2829_file_2829
SELECT 2 + 2 IS NULL AS RESULT ;

-- 来源: 2829_file_2829
SELECT 2 + 2 IS NOT NULL AS RESULT ;

-- 来源: 2829_file_2829
SELECT 2 + 2 ISNULL AS RESULT ;

-- 来源: 2829_file_2829
SELECT 2 + 2 NOTNULL AS RESULT ;

-- 来源: 2829_file_2829
SELECT 2 + 2 IS DISTINCT FROM NULL AS RESULT ;

-- 来源: 2829_file_2829
SELECT 2 + 2 IS NOT DISTINCT FROM NULL AS RESULT ;

-- 来源: 2829_file_2829
select 1 <=> 1 AS RESULT ;

-- 来源: 2829_file_2829
select NULL <=> 1 AS RESULT ;

-- 来源: 2829_file_2829
select NULL <=> NULL AS RESULT ;

-- 来源: 2830_file_2830
SELECT * FROM tpcds . case_when_t1 ;

-- 来源: 2830_file_2830
SELECT CW_COL1 , CASE WHEN CW_COL1 = 1 THEN 'one' WHEN CW_COL1 = 2 THEN 'two' ELSE 'other' END FROM tpcds . case_when_t1 ORDER BY 1 ;

-- 来源: 2830_file_2830
SELECT DECODE ( 'A' , 'A' , 1 , 'B' , 2 , 0 );

-- 来源: 2830_file_2830
SELECT description , short_description , last_value , COALESCE ( description , short_description , last_value ) FROM tpcds . c_tabl ORDER BY 1 , 2 , 3 , 4 ;

-- 来源: 2830_file_2830
SELECT COALESCE ( NULL , 'Hello World' );

-- 来源: 2830_file_2830
SELECT NI_VALUE1 , NI_VALUE2 , NULLIF ( NI_VALUE1 , NI_VALUE2 ) FROM tpcds . null_if_t1 ORDER BY 1 , 2 , 3 ;

-- 来源: 2830_file_2830
SELECT NULLIF ( 'Hello' , 'Hello World' );

-- 来源: 2830_file_2830
SELECT greatest ( 9000 , 155555 , 2 . 01 );

-- 来源: 2830_file_2830
SELECT least ( 9000 , 2 );

-- 来源: 2830_file_2830
SELECT nvl ( null , 1 );

-- 来源: 2830_file_2830
SELECT nvl ( 'Hello World' , 1 );

-- 来源: 2831_file_2831
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE EXISTS ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom = store_returns . sr_reason_sk and sr_customer_sk < 10 );

-- 来源: 2831_file_2831
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk IN ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom < 10 );

-- 来源: 2831_file_2831
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk < ANY ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom < 10 );

-- 来源: 2831_file_2831
SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk < all ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom < 10 );

-- 来源: 2832_file_2832
SELECT 8000 + 500 IN ( 10000 , 9000 ) AS RESULT ;

-- 来源: 2832_file_2832
SELECT 8000 + 500 NOT IN ( 10000 , 9000 ) AS RESULT ;

-- 来源: 2832_file_2832
SELECT 8000 + 500 < SOME ( array [ 10000 , 9000 ]) AS RESULT ;

-- 来源: 2832_file_2832
SELECT 8000 + 500 < ANY ( array [ 10000 , 9000 ]) AS RESULT ;

-- 来源: 2832_file_2832
SELECT 8000 + 500 < ALL ( array [ 10000 , 9000 ]) AS RESULT ;

-- 来源: 2833_file_2833
SELECT ROW ( 1 , 2 , NULL ) < ROW ( 1 , 3 , 0 ) AS RESULT ;

-- 来源: 2833_file_2833
select ( 4 , 5 , 6 ) > ( 3 , 2 , 1 ) as result ;

-- 来源: 2833_file_2833
select ( 4 , 1 , 1 ) > ( 3 , 2 , 1 ) as result ;

-- 来源: 2833_file_2833
select ( 'test' , 'data' ) > ( 'data' , 'data' ) as result ;

-- 来源: 2833_file_2833
select ( 4 , 1 , 1 ) > ( 3 , 2 , null ) as result ;

-- 来源: 2833_file_2833
select ( null , 1 , 1 ) > ( 3 , 2 , 1 ) as result ;

-- 来源: 2833_file_2833
select ( null , 5 , 6 ) > ( null , 5 , 6 ) as result ;

-- 来源: 2833_file_2833
select ( 4 , 5 , 6 ) > ( 4 , 5 , 6 ) as result ;

-- 来源: 2833_file_2833
select ( 2 , 2 , 5 ) >= ( 2 , 2 , 3 ) as result ;

-- 来源: 2833_file_2833
select ( 2 , 2 , 1 ) <= ( 2 , 2 , 3 ) as result ;

-- 来源: 2833_file_2833
select ( 1 , 2 , 3 ) = ( 1 , 2 , 3 ) as result ;

-- 来源: 2833_file_2833
select ( 1 , 2 , 3 ) <> ( 2 , 2 , 3 ) as result ;

-- 来源: 2833_file_2833
select ( 2 , 2 , 3 ) <> ( 2 , 2 , null ) as result ;

-- 来源: 2833_file_2833
select ( null , 5 , 6 ) <> ( null , 5 , 6 ) as result ;

-- 来源: 2834_file_2834
SELECT DATE_ADD ( '2018-05-01' , INTERVAL 1 DAY );

-- 来源: 2834_file_2834
SELECT DATE_SUB ( '2018-05-01' , INTERVAL 1 YEAR );

-- 来源: 2834_file_2834
SELECT DATE '2023-01-10' - INTERVAL 1 DAY ;

-- 来源: 2834_file_2834
SELECT DATE '2023-01-10' + INTERVAL 1 MONTH ;

-- 来源: 2835_file_2835
SELECT * FROM Students WHERE rownum <= 10 ;

-- 来源: 2835_file_2835
SELECT * FROM Students WHERE rownum < 5 order by 1 ;

-- 来源: 2835_file_2835
SELECT rownum , * FROM ( SELECT * FROM Students order by 1 ) WHERE rownum <= 2 ;

-- 来源: 2835_file_2835
SELECT * FROM Students WHERE rownum > 1 ;

-- 来源: 2835_file_2835
SELECT * FROM Students ;

-- 来源: 2835_file_2835
SELECT * FROM Students ;

-- 来源: 2837_file_2837
SELECT text 'Origin' AS "label" , point '(0,0)' AS "value" ;

-- 来源: 2838_file_2838
SELECT 40 ! AS "40 factorial" ;

-- 来源: 2838_file_2838
SELECT CAST ( 40 AS bigint ) ! AS "40 factorial" ;

-- 来源: 2838_file_2838
SELECT text 'abc' || 'def' AS "text and unknown" ;

-- 来源: 2838_file_2838
SELECT 'abc' || 'def' AS "unspecified" ;

-- 来源: 2838_file_2838
SELECT @ '-4.5' AS "abs" ;

-- 来源: 2838_file_2838
SELECT array [ 1 , 2 ] <@ '{1,2,3}' as "is subset" ;

-- 来源: 2839_file_2839
SELECT round ( 4 , 4 );

-- 来源: 2839_file_2839
SELECT round ( CAST ( 4 AS numeric ), 4 );

-- 来源: 2839_file_2839
SELECT round ( 4 . 0 , 4 );

-- 来源: 2839_file_2839
SELECT substr ( '1234' , 3 );

-- 来源: 2839_file_2839
SELECT substr ( varchar '1234' , 3 );

-- 来源: 2839_file_2839
SELECT substr ( CAST ( varchar '1234' AS text ), 3 );

-- 来源: 2839_file_2839
SELECT substr ( 1234 , 3 );

-- 来源: 2839_file_2839
SELECT substr ( CAST ( 1234 AS text ), 3 );

-- 来源: 2840_file_2840
SELECT VS_COL1 , octet_length ( VS_COL1 ) FROM tpcds . value_storage_t1 ;

-- 来源: 2841_UNIONCASE
SELECT text 'a' AS "text" UNION SELECT 'b' ;

-- 来源: 2841_UNIONCASE
SELECT 1 . 2 AS "numeric" UNION SELECT 1 ;

-- 来源: 2841_UNIONCASE
SELECT 1 AS "real" UNION SELECT CAST ( '2.2' AS REAL );

-- 来源: 2845_file_2845
SELECT d_dow || '-' || d_dom || '-' || d_fy_week_seq AS identify_serials FROM tpcds . date_dim WHERE d_fy_week_seq = 1 ;

-- 来源: 2846_file_2846
SELECT 'a fat cat sat on a mat and ate a fat rat' :: tsvector @@ 'cat & rat' :: tsquery AS RESULT ;

-- 来源: 2846_file_2846
SELECT 'fat & cow' :: tsquery @@ 'a fat cat sat on a mat and ate a fat rat' :: tsvector AS RESULT ;

-- 来源: 2846_file_2846
SELECT to_tsvector ( 'fat cats ate fat rats' ) @@ to_tsquery ( 'fat & rat' ) AS RESULT ;

-- 来源: 2846_file_2846
SELECT 'fat cats ate fat rats' :: tsvector @@ to_tsquery ( 'fat & rat' ) AS RESULT ;

-- 来源: 2849_file_2849
SELECT id , body , title FROM tsearch . pgweb WHERE to_tsvector ( 'english' , body ) @@ to_tsquery ( 'english' , 'america' );

-- 来源: 2849_file_2849
SELECT id , body , title FROM tsearch . pgweb WHERE to_tsvector ( body ) @@ to_tsquery ( 'america' );

-- 来源: 2849_file_2849
SELECT title FROM tsearch . pgweb WHERE to_tsvector ( title || ' ' || body ) @@ to_tsquery ( 'north & america' ) ORDER BY last_mod_date DESC LIMIT 10 ;

-- 来源: 2850_file_2850
SELECT title FROM tsearch . pgweb WHERE textsearchable_index_col @@ to_tsquery ( 'north & america' ) ORDER BY last_mod_date DESC LIMIT 10 ;

-- 来源: 2851_file_2851
select c_varchar , to_tsvector ( c_varchar ) from table1 where to_tsvector ( c_text ) @@ plainto_tsquery ( '￥#@……&**' ) and to_tsvector ( c_text ) @@ plainto_tsquery ( '某公司 ' ) and c_varchar is not null order by 1 desc limit 3 ;

-- 来源: 2853_file_2853
SELECT to_tsvector ( 'english' , 'a fat cat sat on a mat - it ate a fat rats' );

-- 来源: 2854_file_2854
SELECT to_tsquery ( 'english' , 'The & Fat & Rats' );

-- 来源: 2854_file_2854
SELECT to_tsquery ( 'english' , 'Fat | Rats:AB' );

-- 来源: 2854_file_2854
SELECT to_tsquery ( 'supern:*A & star:A*B' );

-- 来源: 2854_file_2854
SELECT plainto_tsquery ( 'english' , 'The Fat Rats' );

-- 来源: 2854_file_2854
SELECT plainto_tsquery ( 'english' , 'The Fat & Rats:C' );

-- 来源: 2855_file_2855
SELECT id , title , ts_rank_cd ( to_tsvector ( body ), query ) AS rank FROM tsearch . pgweb , to_tsquery ( 'america' ) query WHERE query @@ to_tsvector ( body ) ORDER BY rank DESC LIMIT 10 ;

-- 来源: 2855_file_2855
SELECT id , title , ts_rank_cd ( to_tsvector ( body ), query , 32 /* rank/(rank+1) */ ) AS rank FROM tsearch . pgweb , to_tsquery ( 'america' ) query WHERE query @@ to_tsvector ( body ) ORDER BY rank DESC LIMIT 10 ;

-- 来源: 2855_file_2855
SELECT id , body , ts_rank_cd ( to_tsvector ( 'ngram' , body ), query ) AS rank FROM tsearch . ts_ngram , to_tsquery ( '中文' ) query WHERE query @@ to_tsvector ( body );

-- 来源: 2855_file_2855
SELECT id , body , ts_rank_cd ( to_tsvector ( 'ngram' , body ), query ) AS rank FROM tsearch . ts_ngram , to_tsquery ( '中文' ) query WHERE query @@ to_tsvector ( 'ngram' , body );

-- 来源: 2856_file_2856
SELECT ts_headline ( 'english' , 'The most common type of search is to find all documents containing given query terms and return them in order of their similarity to the query.' , to_tsquery ( 'english' , 'query & similarity' ));

-- 来源: 2856_file_2856
SELECT ts_headline ( 'english' , 'The most common type of search is to find all documents containing given query terms and return them in order of their similarity to the query.' , to_tsquery ( 'english' , 'query & similarity' ), 'StartSel = <, StopSel = >' );

-- 来源: 2859_file_2859
SELECT numnode ( plainto_tsquery ( 'the any' ));

-- 来源: 2859_file_2859
SELECT numnode(' foo & bar ' :: tsquery );

-- 来源: 2859_file_2859
SELECT querytree ( to_tsquery ( '!defined' ));

-- 来源: 2860_file_2860
SELECT ts_rewrite ( 'a & b' :: tsquery , 'a' :: tsquery , 'c' :: tsquery );

-- 来源: 2860_file_2860
SELECT ts_rewrite ( to_tsquery ( 'supernovae & crab' ), 'SELECT t, s FROM tsearch.aliases' );

-- 来源: 2860_file_2860
SELECT ts_rewrite ( to_tsquery ( 'supernovae & crab' ), 'SELECT t, s FROM tsearch.aliases' );

-- 来源: 2860_file_2860
SELECT ts_rewrite ( 'a & b' :: tsquery , 'SELECT t,s FROM tsearch.aliases WHERE ''a & b''::tsquery @> t' );

-- 来源: 2861_file_2861
SELECT * FROM ts_stat ( 'SELECT to_tsvector(''english'', sr_reason_sk) FROM tpcds.store_returns WHERE sr_customer_sk < 10' ) ORDER BY nentry DESC , ndoc DESC , word LIMIT 10 ;

-- 来源: 2861_file_2861
SELECT * FROM ts_stat ( 'SELECT to_tsvector(''english'', sr_reason_sk) FROM tpcds.store_returns WHERE sr_customer_sk < 10' , 'a' ) ORDER BY nentry DESC , ndoc DESC , word LIMIT 10 ;

-- 来源: 2862_file_2862
SELECT alias , description , token FROM ts_debug ( 'english' , 'foo-bar-beta1' );

-- 来源: 2862_file_2862
SELECT alias , description , token FROM ts_debug ( 'english' , 'http://example.com/stuff/index.html' );

-- 来源: 2865_file_2865
SELECT to_tsvector ( 'english' , 'in the list of stop words' );

-- 来源: 2865_file_2865
SELECT ts_rank_cd ( to_tsvector ( 'english' , 'in the list of stop words' ), to_tsquery ( 'list & stop' ));

-- 来源: 2865_file_2865
SELECT ts_rank_cd ( to_tsvector ( 'english' , 'list stop words' ), to_tsquery ( 'list & stop' ));

-- 来源: 2866_Simple
SELECT ts_lexize ( 'public.simple_dict' , 'YeS' );

-- 来源: 2866_Simple
SELECT ts_lexize ( 'public.simple_dict' , 'The' );

-- 来源: 2866_Simple
SELECT ts_lexize ( 'public.simple_dict' , 'YeS' );

-- 来源: 2866_Simple
SELECT ts_lexize ( 'public.simple_dict' , 'The' );

-- 来源: 2867_Synonym
SELECT * FROM ts_debug ( 'english' , 'Paris' );

-- 来源: 2867_Synonym
SELECT * FROM ts_debug ( 'english' , 'Paris' );

-- 来源: 2867_Synonym
SELECT * FROM ts_debug ( 'english' , 'paris' );

-- 来源: 2867_Synonym
SELECT * FROM ts_debug ( 'english' , 'Paris' );

-- 来源: 2867_Synonym
SELECT * FROM ts_debug ( 'english' , 'paris' );

-- 来源: 2867_Synonym
SELECT ts_lexize ( 'syn' , 'indices' );

-- 来源: 2867_Synonym
SELECT to_tsvector ( 'tst' , 'indices' );

-- 来源: 2867_Synonym
SELECT to_tsquery ( 'tst' , 'indices' );

-- 来源: 2867_Synonym
SELECT 'indexes are very useful' :: tsvector ;

-- 来源: 2867_Synonym
SELECT 'indexes are very useful' :: tsvector @@ to_tsquery ( 'tst' , 'indices' );

-- 来源: 2868_Thesaurus
SELECT plainto_tsquery ( 'russian' , 'supernova star' );

-- 来源: 2868_Thesaurus
SELECT to_tsvector ( 'russian' , 'supernova star' );

-- 来源: 2868_Thesaurus
SELECT to_tsquery ( 'russian' , '''supernova star''' );

-- 来源: 2868_Thesaurus
SELECT plainto_tsquery ( 'russian' , 'supernova star' );

-- 来源: 2869_Ispell
SELECT ts_lexize ( 'norwegian_ispell' , 'sjokoladefabrikk' );

-- 来源: 2871_file_2871
SELECT * FROM ts_debug ( 'ts_conf' , ' GaussDB, the highly scalable, SQL compliant, open source object-relational database management system, is now undergoing beta testing of the next version of our software. ' );

-- 来源: 2873_file_2873
SELECT * FROM ts_debug ( 'english' , 'a fat cat sat on a mat - it ate a fat rats' );

-- 来源: 2874_file_2874
SELECT * FROM ts_parse ( 'default' , '123 - a number' );

-- 来源: 2874_file_2874
SELECT * FROM ts_token_type ( 'default' );

-- 来源: 2875_file_2875
SELECT ts_lexize ( 'english_stem' , 'stars' );

-- 来源: 2875_file_2875
SELECT ts_lexize ( 'english_stem' , 'a' );

--查询数据。
-- 来源: 2882_ABORT
SELECT * FROM customer_demographics_t1 WHERE cd_demo_sk = 1920801;

--查看test_db1信息。
-- 来源: 2885_ALTER DATABASE
SELECT datname,datconnlimit FROM pg_database WHERE datname = 'test_db1';

--查看test_db1信息。
-- 来源: 2885_ALTER DATABASE
SELECT t1.datname, t2.usename FROM pg_database t1, pg_user t2 WHERE t1.datname='test_db1' AND t1.datdba=t2.usesysid;

--查看test_db1信息。
-- 来源: 2885_ALTER DATABASE
SELECT t1.datname AS database, t2.spcname AS tablespace FROM pg_database t1, pg_tablespace t2 WHERE t1.dattablespace = t2.oid AND t1.datname = 'test_db1';

-- 来源: 2885_ALTER DATABASE
SELECT tablename FROM pg_tables WHERE tablename = 'test_tbl1';

--由于隔离属性的原因，该查询只能查出0条数据。
-- 来源: 2885_ALTER DATABASE
SELECT tablename FROM pg_tables WHERE tablename = 'test_tbl1';

-- 来源: 2894_ALTER GLOBAL CONFIGURATION
SELECT * FROM gs_global_config ;

-- 来源: 2894_ALTER GLOBAL CONFIGURATION
SELECT * FROM gs_global_config ;

-- 来源: 2894_ALTER GLOBAL CONFIGURATION
SELECT * FROM gs_global_config ;

--查询test1表上的索引信息。
-- 来源: 2896_ALTER INDEX
SELECT tablename,indexname,tablespace FROM pg_indexes WHERE tablename = 'test1';

--查询test1表上的索引信息。
-- 来源: 2896_ALTER INDEX
SELECT tablename,indexname,tablespace FROM pg_indexes WHERE tablename = 'test1';

--查看索引idx_test1_col1的可用性。
-- 来源: 2896_ALTER INDEX
SELECT indisusable FROM pg_index WHERE indexrelid = 'idx_test1_col1'::regclass;

--查看索引idx_test1_col1的可用性。
-- 来源: 2896_ALTER INDEX
SELECT indisusable FROM pg_index WHERE indexrelid = 'idx_test1_col1'::regclass;

--查询索引idx_test2_col1分区的名称。
-- 来源: 2896_ALTER INDEX
SELECT relname FROM pg_partition WHERE parentid = 'idx_test2_col1'::regclass;

--查询索引idx_test2_col1分区的所属表空间。
-- 来源: 2896_ALTER INDEX
SELECT t1.relname index_name, t2.spcname tablespace_name FROM pg_partition t1, pg_tablespace t2 WHERE t1.parentid = 'idx_test2_col1'::regclass AND t1.reltablespace = t2.oid;

-- 来源: 2915_ALTER SYSTEM KILL SESSION
SELECT sa.sessionid AS sid,0::integer AS serial#,ad.rolname AS username FROM pg_stat_get_activity(NULL) AS sa LEFT JOIN pg_authid ad ON(sa.usesysid = ad.oid)WHERE sa.application_name <> 'JobScheduler';

--查询表信息。
-- 来源: 2916_ALTER TABLE
SELECT schemaname,tablename FROM pg_tables WHERE tablename = 'test_alt1';

-- 查看
-- 来源: 2916_ALTER TABLE
SELECT tablename, schemaname, tableowner FROM pg_tables WHERE tablename = 'test_alt1';

-- 查看。
-- 来源: 2916_ALTER TABLE
SELECT tablename, tablespace FROM pg_tables WHERE tablename = 'test_alt1';

--查询文本搜索配置相关信息。
-- 来源: 2920_ALTER TEXT SEARCH CONFIGURATION
SELECT b.cfgname,a.maptokentype,a.mapseqno,a.mapdict,c.dictname FROM pg_ts_config_map a,pg_ts_config b, pg_ts_dict c WHERE a.mapcfg=b.oid AND a.mapdict=c.oid AND b.cfgname='english_1' ORDER BY 1,2,3,4,5;

--查询文本搜索配置相关信息。
-- 来源: 2920_ALTER TEXT SEARCH CONFIGURATION
SELECT b.cfgname,a.maptokentype,a.mapseqno,a.mapdict,c.dictname FROM pg_ts_config_map a,pg_ts_config b, pg_ts_dict c WHERE a.mapcfg=b.oid AND a.mapdict=c.oid AND b.cfgname='english_1' ORDER BY 1,2,3,4,5;

-- 来源: 2935_CLUSTER
SELECT * FROM test_c1 ;

-- 来源: 2935_CLUSTER
SELECT * FROM test_c1 ;

-- 查看
-- 来源: 2935_CLUSTER
SELECT * FROM test_c2;

-- 查看
-- 来源: 2935_CLUSTER
SELECT * FROM test_c2;

--查询数据。
-- 来源: 2937_COMMIT _ END
SELECT * FROM tpcds. customer_demographics_t2;

-- 执行聚合函数
-- 来源: 2940_CREATE AGGREGATE
SELECT sum(a) FROM test_sum;

--查看数据库testdb1信息。
-- 来源: 2944_CREATE DATABASE
SELECT datname,pg_encoding_to_char(encoding) FROM pg_database WHERE datname = 'testdb1';

--查看testdb2信息。
-- 来源: 2944_CREATE DATABASE
SELECT t1.datname,t2.usename,t1.datcompatibility FROM pg_database t1,pg_user t2 WHERE t1.datname = 'testdb2' AND t1.datdba=t2.usesysid;

--查看testdb3信息。
-- 来源: 2944_CREATE DATABASE
SELECT datname,datcompatibility,dattimezone FROM pg_database WHERE datname = 'testdb3';

-- 调用func_dup_sql函数
-- 来源: 2951_CREATE FUNCTION
SELECT * FROM func_dup_sql(42);

--查询索引idx_test1信息。
-- 来源: 2955_CREATE INDEX
SELECT indexname,tablename,tablespace FROM pg_indexes WHERE indexname = 'idx_test1';

--查看索引分区信息，发现LOC索引分区数和表的分区数一致。
-- 来源: 2955_CREATE INDEX
SELECT relname FROM pg_partition WHERE parentid = 'idx_student1'::regclass;

--查看索引分区信息，发现GLOBAL索引分区数和表的分区数不一致。
-- 来源: 2955_CREATE INDEX
SELECT relname FROM pg_partition WHERE parentid = 'idx_student2'::regclass;

-- 来源: 2957_CREATE MASKING POLICY
SELECT * FROM tb_for_masking ;

-- 来源: 2957_CREATE MASKING POLICY
SELECT col8 FROM tb_for_masking ;

-- 来源: 2957_CREATE MASKING POLICY
SELECT col8 FROM tb_for_masking ;

-- 来源: 2962_CREATE PACKAGE
SELECT emp_bonus.testpro1(1);

-- 来源: 2963_CREATE PROCEDURE
SELECT prc_add ( 2 , 3 );

-- 来源: 2963_CREATE PROCEDURE
SELECT pro_variadic ( var1 => 'hello' , VARIADIC var4 => array [ 1 , 2 , 3 , 4 ]);

--当前用户执行SELECT操作
-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
SELECT * FROM all_data;

--切换至alice用户执行SELECT操作。
-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
SELECT * FROM all_data;

-- 来源: 2972_CREATE SEQUENCE
SELECT nextval ( 'seq1' );

-- 来源: 2972_CREATE SEQUENCE
SELECT nextval ( 'seq1' );

-- 来源: 2972_CREATE SEQUENCE
SELECT * FROM test1 ;

--使用同义词t1。
-- 来源: 2975_CREATE SYNONYM
SELECT * FROM t1;

--使用同义词v1。
-- 来源: 2975_CREATE SYNONYM
SELECT * FROM v1;

--使用同义词add。
-- 来源: 2975_CREATE SYNONYM
SELECT add(1,2);

-- 来源: 2975_CREATE SYNONYM
SELECT add(1.2,2.3);

-- 查询表中col1<100的数据。
-- 来源: 2977_CREATE TABLE AS
SELECT * FROM test1 WHERE col1 < 100;

--查询分区P10的行数。
-- 来源: 2978_CREATE TABLE PARTITION
SELECT count(*) FROM tpcds. web_returns_p1 PARTITION (P10);

--查询分区P1的行数。
-- 来源: 2978_CREATE TABLE PARTITION
SELECT COUNT(*) FROM tpcds. web_returns_p1 PARTITION FOR (2450815);

-- 查看分区表信息
-- 来源: 2978_CREATE TABLE PARTITION
SELECT relname, boundaries, spcname FROM pg_partition p JOIN pg_tablespace t ON p.reltablespace=t.oid and p.parentid='tpcds.startend_pt'::regclass ORDER BY 1;

-- 来源: 2978_CREATE TABLE PARTITION
SELECT COUNT(*) FROM tpcds.startend_pt PARTITION FOR (0);

-- 来源: 2978_CREATE TABLE PARTITION
SELECT COUNT(*) FROM tpcds.startend_pt PARTITION (p3);

-- 查看分区情形
-- 来源: 2978_CREATE TABLE PARTITION
SELECT relname, boundaries, spcname FROM pg_partition p JOIN pg_tablespace t ON p.reltablespace=t.oid and p.parentid='tpcds.startend_pt'::regclass ORDER BY 1;

-- 来源: 2978_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'sales' AND t1 . parttype = 'p' ;

-- 来源: 2978_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'sales' AND t1 . parttype = 'p' ;

-- 来源: 2978_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- 来源: 2978_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- 来源: 2978_CREATE TABLE PARTITION
select * from test_list partition ( p1 );

-- 来源: 2978_CREATE TABLE PARTITION
select * from test_list partition ( p1 );

-- 来源: 2978_CREATE TABLE PARTITION
select * from t1 ;

-- 来源: 2978_CREATE TABLE PARTITION
select * from test_list partition ( p2 );

-- 来源: 2978_CREATE TABLE PARTITION
select * from test_list partition ( p2 );

-- 来源: 2978_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- 来源: 2978_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- 来源: 2978_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

-- 来源: 2978_CREATE TABLE PARTITION
SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_hash' AND t1 . parttype = 'p' ;

-- 来源: 2978_CREATE TABLE PARTITION
select * from test_hash partition ( p1 );

-- 来源: 2978_CREATE TABLE PARTITION
select * from test_hash partition ( p2 );

-- 来源: 2978_CREATE TABLE PARTITION
select * from test_hash partition ( p1 );

-- 来源: 2978_CREATE TABLE PARTITION
select * from t1 ;

-- 来源: 2978_CREATE TABLE PARTITION
select * from test_hash partition ( p2 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_hash ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_range ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_hash ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_range ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from hash_list ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from hash_hash ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from hash_range ;

--指定分区查询数据
-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list partition (p_201901);

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list subpartition (p_201901_a);

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list partition for ('201902');

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list subpartition for ('201902','1');

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select *from range_list;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from range_list;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list partition ( p_201901 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list partition ( p_201901 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list partition ( p_201902 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list partition ( p_201902 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201901_a );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201901_a );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201901_b );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201901_b );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201902_a );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201902_a );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201902_b );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201902_b );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201901_a );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201901_b );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201901_a );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201901_b );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201901_c );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list partition ( p_201901 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201902_a );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201902_b );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201902_a );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201902_b );

-- 来源: 2980_CREATE TABLE SUBPARTITION
select * from list_list subpartition ( p_201902_c );

-- 来源: 2983_CREATE TRIGGER
SELECT * FROM test_trigger_src_tbl;

-- 来源: 2983_CREATE TRIGGER
SELECT * FROM test_trigger_des_tbl;

-- 来源: 2983_CREATE TRIGGER
SELECT * FROM test_trigger_src_tbl;

-- 来源: 2983_CREATE TRIGGER
SELECT * FROM test_trigger_des_tbl;

-- 来源: 2983_CREATE TRIGGER
SELECT * FROM test_trigger_src_tbl;

-- 来源: 2983_CREATE TRIGGER
SELECT * FROM test_trigger_des_tbl;

-- 来源: 2984_CREATE TYPE
SELECT (b).f1 FROM t1_compfoo;

-- 来源: 2984_CREATE TYPE
SELECT * FROM t1_compfoo t1 join t2_compfoo t2 on (t1.b).f1=(t1.b).f1;

--查看视图。
-- 来源: 2987_CREATE VIEW
SELECT * FROM test_v1;

--查看现有弱口令。
-- 来源: 2988_CREATE WEAK PASSWORD DICTIONARY
SELECT * FROM gs_global_config WHERE NAME LIKE 'weak_password';

-- 来源: 2991_DEALLOCATE
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- 来源: 2991_DEALLOCATE
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- 来源: 2991_DEALLOCATE
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- 来源: 2991_DEALLOCATE
SELECT name , statement , parameter_types FROM pg_prepared_statements ;

-- 来源: 3044_EXPLAIN PLAN
SELECT * FROM plan_table;

-- 查询更新后的结果
-- 来源: 3060_MERGE INTO
SELECT * FROM products ORDER BY product_id;

-- 来源: 3063_PREDICT BY
SELECT id, PREDICT BY price_model (FEATURES size,lot) FROM houses;

--查看回收站
-- 来源: 3066_PURGE
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

-- 来源: 3066_PURGE
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

-- 来源: 3066_PURGE
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

-- 来源: 3066_PURGE
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

--查询表的内容，会同时看到3和4。
-- 来源: 3072_RELEASE SAVEPOINT
SELECT * FROM tpcds. table1;

--查询值替换插入的结果
-- 来源: 3073_REPLACE
SELECT * FROM test WHERE f1 = 1;

-- 来源: 3073_REPLACE
SELECT * FROM test WHERE f1 = 2;

-- 来源: 3073_REPLACE
SELECT * FROM test WHERE f1 = 3;

--查询表的内容，会同时看到1和3,不能看到2，因为2被回滚。
-- 来源: 3080_SAVEPOINT
SELECT * FROM table1;

--查询表的内容，会同时看到3和4。
-- 来源: 3080_SAVEPOINT
SELECT * FROM table2;

-- 来源: 3082_SELECT
SELECT * FROM XMLTABLE( XMLNAMESPACES('nspace1' AS "ns1", 'nspace2' AS "ns2"), -- 声明两个XML的命名空间'nspace1'和'nspace2'及对应的别名"ns1"和"ns2" '/ns1:root/*:child' -- 经row_expression从传入的数据中选取命名空间为'nspace1'的root节点，在选取其下面的所有child节点，忽略child的命名空间；其中ns1为'nspace1'的别名 PASSING xmltype( '<root xmlns="nspace1"> <child> <name>peter</name> <age>11</age> </child> <child xmlns="nspace1"> <name>qiqi</name> <age>12</age> </child> <child xmlns="nspace2"> <name>hacker</name> <age>15</age> </child> </root>') COLUMNS column FOR ORDINALITY, -- 该列为行号列 name varchar(10) path 'ns1:name', -- 从row_expression获取的每个child节点中选取命名空间为'nspace1'的name节点，并将节点中的值转换为varchar(10)返回；其中ns1为'nspace1'的别名 age int);

-- 来源: 3082_SELECT
SELECT * FROM TEST START WITH id = 1 CONNECT BY prior id = fatherid ORDER SIBLINGS BY id DESC;

--先通过子查询得到一张临时表temp_t，然后查询表temp_t中的所有数据。
-- 来源: 3082_SELECT
WITH temp_t(name,isdba) AS (SELECT usename,usesuper FROM pg_user) SELECT * FROM temp_t;

--查询 tpcds. reason表的所有r_reason_sk记录，且去除重复。
-- 来源: 3082_SELECT
SELECT DISTINCT(r_reason_sk) FROM tpcds. reason;

--LIMIT子句示例：获取表中一条记录。
-- 来源: 3082_SELECT
SELECT * FROM tpcds. reason LIMIT 1;

--查询所有记录，且按字母升序排列。
-- 来源: 3082_SELECT
SELECT r_reason_desc FROM tpcds. reason ORDER BY r_reason_desc;

--通过表别名，从pg_user和pg_user_status这两张表中获取数据。
-- 来源: 3082_SELECT
SELECT a.usename,b.locktime FROM pg_user a,pg_user_status b WHERE a.usesysid=b.roloid;

--FULL JOIN子句示例：将pg_user和pg_user_status这两张表的数据进行全连接显示，即数据的合集。
-- 来源: 3082_SELECT
SELECT a.usename,b.locktime,a.usesuper FROM pg_user a FULL JOIN pg_user_status b ON a.usesysid=b.roloid;

--GROUP BY子句示例：根据查询条件过滤，并对结果进行分组。
-- 来源: 3082_SELECT
SELECT r_reason_id, AVG(r_reason_sk) FROM tpcds. reason GROUP BY r_reason_id HAVING AVG(r_reason_sk) > 25;

--GROUP BY CUBE子句示例：根据查询条件过滤，并对结果进行分组汇总。
-- 来源: 3082_SELECT
SELECT r_reason_id,AVG(r_reason_sk) FROM tpcds. reason GROUP BY CUBE(r_reason_id,r_reason_sk);

--GROUP BY GROUPING SETS子句示例:根据查询条件过滤，并对结果进行分组汇总。
-- 来源: 3082_SELECT
SELECT r_reason_id,AVG(r_reason_sk) FROM tpcds. reason GROUP BY GROUPING SETS((r_reason_id,r_reason_sk),r_reason_sk);

--UNION子句示例：将表 tpcds. reason里r_reason_desc字段中的内容以W开头和以N开头的进行合并。
-- 来源: 3082_SELECT
SELECT r_reason_sk, tpcds. reason.r_reason_desc FROM tpcds. reason WHERE tpcds. reason.r_reason_desc LIKE 'W%' UNION SELECT r_reason_sk, tpcds. reason.r_reason_desc FROM tpcds. reason WHERE tpcds. reason.r_reason_desc LIKE 'N%';

--NLS_SORT子句示例：中文拼音排序。
-- 来源: 3082_SELECT
SELECT * FROM tpcds. reason ORDER BY NLSSORT( r_reason_desc, 'NLS_SORT = SCHINESE_PINYIN_M');

--不区分大小写排序（可选，仅支持纯英文不区分大小写排序）:
-- 来源: 3082_SELECT
SELECT * FROM tpcds. reason ORDER BY NLSSORT( r_reason_desc, 'NLS_SORT = generic_m_ci');

--PARTITION子句示例：从 tpcds. reason_p的表分区P_05_BEFORE中获取数据。
-- 来源: 3082_SELECT
SELECT * FROM tpcds. reason_p PARTITION (P_05_BEFORE);

--PARTITION子句指定多分区示例：从 tpcds. reason_p的表分区P_05_BEFORE，P_15，P_25中获取数据。
-- 来源: 3082_SELECT
SELECT * FROM tpcds. reason_p PARTITION (P_05_BEFORE, P_15, P_25) ORDER BY 1;

--GROUP BY子句示例：按r_reason_id分组统计 tpcds. reason_p表中的记录数。
-- 来源: 3082_SELECT
SELECT COUNT(*),r_reason_id FROM tpcds. reason_p GROUP BY r_reason_id;

--GROUP BY CUBE子句示例：根据查询条件过滤，并对查询结果分组汇总。
-- 来源: 3082_SELECT
SELECT * FROM tpcds. reason GROUP BY CUBE (r_reason_id,r_reason_sk,r_reason_desc);

--GROUP BY GROUPING SETS子句示例：根据查询条件过滤，并对查询结果分组汇总。
-- 来源: 3082_SELECT
SELECT * FROM tpcds. reason GROUP BY GROUPING SETS ((r_reason_id,r_reason_sk),r_reason_desc);

--HAVING子句示例：按r_reason_id分组统计 tpcds. reason_p表中的记录，并只显示r_reason_id个数大于2的信息。
-- 来源: 3082_SELECT
SELECT COUNT(*) c,r_reason_id FROM tpcds. reason_p GROUP BY r_reason_id HAVING c>2;

--IN子句示例：按r_reason_id分组统计 tpcds. reason_p表中的r_reason_id个数，并只显示r_reason_id值为 AAAAAAAABAAAAAAA或AAAAAAAADAAAAAAA的个数。
-- 来源: 3082_SELECT
SELECT COUNT(*),r_reason_id FROM tpcds. reason_p GROUP BY r_reason_id HAVING r_reason_id IN('AAAAAAAABAAAAAAA','AAAAAAAADAAAAAAA');

--INTERSECT子句示例：查询r_reason_id等于AAAAAAAABAAAAAAA，并且r_reason_sk小于5的信息。
-- 来源: 3082_SELECT
SELECT * FROM tpcds. reason_p WHERE r_reason_id='AAAAAAAABAAAAAAA' INTERSECT SELECT * FROM tpcds. reason_p WHERE r_reason_sk<5;

--EXCEPT子句示例：查询r_reason_id等于AAAAAAAABAAAAAAA，并且去除r_reason_sk小于4的信息。
-- 来源: 3082_SELECT
SELECT * FROM tpcds. reason_p WHERE r_reason_id='AAAAAAAABAAAAAAA' EXCEPT SELECT * FROM tpcds. reason_p WHERE r_reason_sk<4;

--通过在where子句中指定"(+)"来实现左连接。
-- 来源: 3082_SELECT
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE t1.sr_customer_sk = t2.c_customer_sk(+) ORDER BY 1 DESC LIMIT 1;

--通过在where子句中指定"(+)"来实现右连接。
-- 来源: 3082_SELECT
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE t1.sr_customer_sk(+) = t2.c_customer_sk ORDER BY 1 DESC LIMIT 1;

--通过在where子句中指定"(+)"来实现左连接，并且增加连接条件。
-- 来源: 3082_SELECT
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE t1.sr_customer_sk = t2.c_customer_sk(+) AND t2.c_customer_sk(+) < 1 ORDER BY 1 LIMIT 1;

--不支持在where子句中指定"(+)"的同时使用内层嵌套AND/OR的表达式。
-- 来源: 3082_SELECT
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE NOT(t1.sr_customer_sk = t2.c_customer_sk(+) AND t2.c_customer_sk(+) < 1);

--where子句在不支持表达式宏指定"(+)"会报错。
-- 来源: 3082_SELECT
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE (t1.sr_customer_sk = t2.c_customer_sk(+))::bool;

--where子句在表达式的两边都指定"(+)"会报错。
-- 来源: 3082_SELECT
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE t1.sr_customer_sk(+) = t2.c_customer_sk(+);

-- 来源: 3082_SELECT
SELECT * FROM tpcds.time_table;

--2021-04-25 17:50:22.311176应该使用tpcds.time_table中第四条snaptime字段值
-- 来源: 3082_SELECT
SELECT * FROM tpcds.time_table TIMECAPSULE TIMESTAMP to_timestamp('2021-04-25 17:50:22.311176','YYYY-MM-DD HH24:MI:SS.FF');

--107330 csn应该使用tpcds.time_table中第四条snapcsn字段值
-- 来源: 3082_SELECT
SELECT * FROM tpcds.time_table TIMECAPSULE CSN 107330;

--WITH RECURSIVE查询示例：计算从1到100的累加值。
-- 来源: 3082_SELECT
WITH RECURSIVE t1(a) AS ( SELECT 100 ), t(n) AS ( VALUES (1) UNION ALL SELECT n+1 FROM t WHERE n < (SELECT max(a) FROM t1) ) SELECT sum(n) FROM t;

-- 来源: 3082_SELECT
SELECT * FROM p1;

-- 来源: 3082_SELECT
SELECT * FROM p1 UNPIVOT(score FOR class IN(math, phy));

-- 来源: 3082_SELECT
SELECT * FROM p2;

-- 来源: 3082_SELECT
SELECT * FROM p2 PIVOT(max(score) FOR class IN ('MATH', 'PHY'));

-- 来源: 3082_SELECT
SELECT * FROM skiplocked_astore WHERE id = 1 FOR UPDATE;

--STEP 3:session2 使用SKIP LOCKED会跳过被锁行，仅返回加锁成功的行
-- 来源: 3082_SELECT
SELECT * FROM skiplocked_astore FOR UPDATE SKIP LOCKED;

--将 tpcds. reason表中r_reason_sk小于5的值加入到新建表中。
-- 来源: 3083_SELECT INTO
SELECT * INTO tpcds. reason_t1 FROM tpcds. reason WHERE r_reason_sk < 5;

--查看数据
-- 来源: 3091_SHRINK
SELECT * FROM row_compression;

-- 来源: 3093_SNAPSHOT
SELECT * FROM DB4AISHOT(s1@1.0);

-- 来源: 3094_START TRANSACTION
SELECT * FROM tpcds. reason;

-- 来源: 3094_START TRANSACTION
SELECT * FROM tpcds. reason;

-- 来源: 3094_START TRANSACTION
SELECT * FROM tpcds. reason;

--查询tpcds.reason_t2表中的数据
-- 来源: 3096_TIMECAPSULE TABLE
SELECT * FROM tpcds.reason_t2;

-- 来源: 3096_TIMECAPSULE TABLE
SELECT * FROM tpcds.reason_t2;

--查询tbl_test1表。
-- 来源: 3099_UPDATE
SELECT * FROM tbl_test1;

--查询tbl_test1表。
-- 来源: 3099_UPDATE
SELECT * FROM tbl_test1;

--查询。
-- 来源: 3099_UPDATE
SELECT * FROM test_grade;

--2008-08-25 Ben参加了补考,成绩为B，正常步骤需要先修改last_exam为否,然后插入2008-08-25这一天的成绩。
-- 来源: 3099_UPDATE
WITH old_exam AS ( UPDATE test_grade SET last_exam = 0 WHERE sid = 2 AND examtime = '2008-07-08' RETURNING sid, name ) INSERT INTO test_grade VALUES ( ( SELECT sid FROM old_exam ), ( SELECT name FROM old_exam ), 'B', '2008-08-25', 1 );

--查询。
-- 来源: 3099_UPDATE
SELECT * FROM test_grade;

-- 来源: 3111_file_3111
select $$it's an example$$;

-- 来源: 3145_file_3145
SELECT check_test();

-- 来源: 3149_file_3149
SELECT * FROM sections_t1 ;

-- 来源: 3157_file_3157
SELECT * FROM hdfs_t1 ;

-- 来源: 3160_file_3160
select * from mytab ;

-- 来源: 3160_file_3160
SELECT merge_db ( 1 , 'david' );

-- 来源: 3160_file_3160
SELECT merge_db ( 1 , 'dennis' );

-- 来源: 3178_DBE_COMPRESSION
SELECT DBE_COMPRESSION . GET_COMPRESSION_TYPE ( 'user1' , 'test_data' , '(0,1)' , NULL );

-- 来源: 3180_DBE_HEAT_MAP
SELECT * from DBE_HEAT_MAP . ROW_HEAT_MAP ( owner => 'heat_map_data' , segment_name => 'heat_map_table' , partition_name => NULL , ctid => '(0,1)' );

-- 来源: 3181_DBE_ILM
SELECT ORDER_ID , DBE_COMPRESSION . GET_COMPRESSION_TYPE ( 'ilm_data' , 'order_table' , ctid :: text , NULL ) FROM ILM_DATA . ORDER_TABLE ;

-- 来源: 3181_DBE_ILM
SELECT ORDER_ID , DBE_HEAT_MAP . ROW_HEAT_MAP ( 'ilm_data' , 'order_table' , NULL , ctid :: text ) FROM ILM_DATA . ORDER_TABLE ;

-- 来源: 3181_DBE_ILM
SELECT ORDER_ID , DBE_COMPRESSION . GET_COMPRESSION_TYPE ( 'ilm_data' , 'order_table' , ctid :: text , NULL ) FROM ILM_DATA . ORDER_TABLE ;

-- 来源: 3182_DBE_ILM_ADMIN
select * from gs_adm_ilmparameters ;

-- 来源: 3186_DBE_PROFILER
SELECT dbe_profiler . pl_start_profiling ( '123' );

-- 来源: 3186_DBE_PROFILER
SELECT * FROM dbe_profiler . pl_profiling_functions ORDER BY run_id , funcoid ;

-- 来源: 3186_DBE_PROFILER
SELECT * FROM dbe_profiler . pl_profiling_details WHERE funcoid = 16770 ORDER BY run_id , funcoid , line # ;

-- 来源: 3186_DBE_PROFILER
SELECT * FROM dbe_profiler . pl_profiling_callgraph ORDER BY run_id , stack ;

-- 来源: 3186_DBE_PROFILER
SELECT step_name , loops_count FROM dbe_profiler . pl_profiling_trackinfo WHERE funcoid = 16770 ;

-- 来源: 3186_DBE_PROFILER
SELECT dbe_profiler . pl_clear_profiling ( '' );

-- 来源: 3186_DBE_PROFILER
SELECT step_name , loops_count FROM dbe_profiler . pl_profiling_trackinfo WHERE funcoid = 16770 ;

-- 来源: 3186_DBE_PROFILER
SELECT dbe_profiler . pl_start_profiling ( '100' );

-- 来源: 3186_DBE_PROFILER
SELECT * FROM dbe_profiler . pl_profiling_functions ORDER BY run_id , funcoid ;

-- 来源: 3186_DBE_PROFILER
SELECT * FROM dbe_profiler . pl_profiling_details ORDER BY run_id , funcoid , line # ;

-- 来源: 3186_DBE_PROFILER
SELECT * FROM dbe_profiler . pl_profiling_callgraph ORDER BY run_id , stack ;

-- 来源: 3186_DBE_PROFILER
SELECT * FROM dbe_profiler . pl_profiling_trackinfo ORDER BY run_id , funcoid ;

-- 来源: 3186_DBE_PROFILER
SELECT dbe_profiler . pl_start_profiling ( '101' );

-- 来源: 3186_DBE_PROFILER
SELECT * FROM dbe_profiler . pl_profiling_functions ORDER BY run_id , funcoid ;

-- 来源: 3186_DBE_PROFILER
SELECT * FROM dbe_profiler . pl_profiling_details ORDER BY run_id , funcoid , line # ;

-- 来源: 3186_DBE_PROFILER
SELECT * FROM dbe_profiler . pl_profiling_callgraph ORDER BY run_id , stack ;

-- 来源: 3186_DBE_PROFILER
SELECT * FROM dbe_profiler . pl_profiling_trackinfo ORDER BY run_id , funcoid ;

-- 来源: 3186_DBE_PROFILER
SELECT dbe_profiler . pl_clear_profiling ( '' );

-- 来源: 3186_DBE_PROFILER
SELECT * FROM dbe_profiler . pl_profiling_functions ;

-- 来源: 3189_DBE_SCHEDULER
SELECT dbe_scheduler . create_job ( 'job1' , 'PLSQL_BLOCK' , 'begin insert into test1 values(12);

-- 来源: 3189_DBE_SCHEDULER
SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 3189_DBE_SCHEDULER
select DBE_SCHEDULER.create_credential('cre_1', 'test1', '*********');

-- 来源: 3189_DBE_SCHEDULER
select DBE_SCHEDULER.create_job(job_name=>'job1', job_type=>'EXTERNAL_SCRIPT', job_action=>'/usr/bin/pwd', enabled=>true, auto_drop=>false, credential_name => 'cre_1');

-- 来源: 3189_DBE_SCHEDULER
SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 3189_DBE_SCHEDULER
SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 3192_DBE_STATS
SELECT relname,instr(reloptions::text,'stat_state=locked',1,1) <> 0 as exist_lock FROM PG_CLASS WHERE relname='t1' and relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_lock');

-- 来源: 3192_DBE_STATS
SELECT relname,instr(reloptions::text,'stat_state=locked',1,1) <> 0 as exist_lock FROM PG_CLASS WHERE relname='upart_table';

-- 来源: 3192_DBE_STATS
SELECT relname,instr(reloptions::text,'stat_state=locked',1,1) <> 0 as exist_lock FROM PG_PARTITION WHERE parentid='upart_table'::REGCLASS;

-- 来源: 3192_DBE_STATS
SELECT staattnum,stastate FROM PG_STATISTIC WHERE starelid='t1'::REGCLASS;

-- 查看历史表
-- 来源: 3192_DBE_STATS
SELECT relname,reltuples FROM GS_TABLESTATS_HISTORY WHERE relname='t1';

-- 查看当前系统表中的统计信息
-- 来源: 3192_DBE_STATS
SELECT relname,reltuples FROM PG_CLASS WHERE relname='t1' AND relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_restore');

-- 来源: 3192_DBE_STATS
SELECT relname,reltuples FROM PG_CLASS WHERE relname='t1' AND relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_restore');

-- 查看历史表里的统计信息
-- 来源: 3192_DBE_STATS
SELECT staattnum,stadistinct,stakind1,stanumbers1,stavalues1 FROM GS_STATISTIC_HISTORY WHERE starelid='t1'::REGCLASS ORDER BY statimestamp;

-- 查询当前系统表中的统计信息
-- 来源: 3192_DBE_STATS
SELECT staattnum,stadistinct,stakind1,stanumbers1,stavalues1 FROM PG_STATISTIC WHERE starelid='t1'::REGCLASS;

-- 来源: 3192_DBE_STATS
SELECT staattnum,stadistinct,stakind1,stanumbers1,stavalues1 FROM PG_STATISTIC WHERE starelid='t1'::REGCLASS;

-- 来源: 3192_DBE_STATS
SELECT relname,reltuples FROM GS_TABLESTATS_HISTORY WHERE relname='t1';

-- 来源: 3192_DBE_STATS
SELECT relname,reltuples FROM PG_CLASS WHERE relname='t1' AND relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_restore');

-- 来源: 3192_DBE_STATS
SELECT relname,reltuples FROM PG_CLASS WHERE relname='t1' AND relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_restore');

-- 查看历史表
-- 来源: 3192_DBE_STATS
SELECT relname,reltuples FROM GS_TABLESTATS_HISTORY WHERE relname='t1';

-- 来源: 3192_DBE_STATS
SELECT relname,reltuples FROM GS_TABLESTATS_HISTORY WHERE relname='t1';

-- 来源: 3193_DBE_TASK
SELECT DBE_TASK . SUBMIT ( 'call pro_xxx();

-- 来源: 3193_DBE_TASK
SELECT DBE_TASK . SUBMIT ( 'call pro_xxx();

-- 来源: 3202_file_3202
SELECT * FROM t1;

-- 来源: 3202_file_3202
SELECT ok.id,ok.a FROM autonomous_out() AS ok(id INT,a DATE);

-- 来源: 3203_file_3203
SELECT * FROM t2;

-- 来源: 3203_file_3203
SELECT autonomous_5(11,22);

-- 来源: 3203_file_3203
SELECT * FROM t2 ORDER BY a;

-- 来源: 3204_file_3204
SELECT * FROM t1;

-- 来源: 3205_file_3205
SELECT autonomous_33(0);

-- 来源: 3205_file_3205
SELECT * FROM t4;

-- 来源: 3206_Package
SELECT * FROM t2;

-- 来源: 3206_Package
SELECT autonomous_5(11,22);

-- 来源: 3206_Package
SELECT * FROM t2 ORDER BY a;

-- 来源: 3626_PG_REPLICATION_SLOTS
SELECT * FROM pg_replication_slots;

-- 来源: 3806_SESSION_STAT_ACTIVITY
SELECT datname, usename, usesysid,state,pid FROM pg_stat_activity;

-- 来源: 3807_GLOBAL_SESSION_STAT_ACTIVITY
SELECT datname, usename, usesysid,state,pid FROM pg_stat_activity;

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT OID FROM PG_PROC WHERE PRONAME = 'test_debug' ;

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . turn_on ( 16389 );

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . attach ( 'datanode' , 0 );

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . next ();

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . info_locals ();

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . set_var ( 'x' , 2 );

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . print_var ( 'x' );

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . continue ();

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . continue ();

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . error_end ();

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . abort ();

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . info_code ( 16389 );

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . add_breakpoint ( 16389 , 4 );

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . info_breakpoints ();

-- 来源: 3929_DBE_PLDEBUGGER Schema
SELECT * FROM DBE_PLDEBUGGER . continue ();

-- 来源: 3977_file_3977
SELECT * FROM pg_settings WHERE NAME = 'server_version' ;

-- 来源: 3977_file_3977
SELECT * FROM pg_settings ;

-- 来源: 4027_file_4027
select 0 . 1231243 as a , 0 . 1231243 :: numeric as b , 0 . 1231243 :: integer ( 10 , 3 ) as c , length ( 0 . 1242343 ) as d ;

-- 来源: 4027_file_4027
select 0 . 1231243 as a , 0 . 1231243 :: numeric as b , 0 . 1231243 :: integer ( 10 , 3 ) as c , length ( 0 . 1242343 ) as d ;

-- 来源: 4027_file_4027
select add_months ( '2018-02-28' , 3 ) from sys_dummy ;

-- 来源: 4027_file_4027
select add_months ( '2018-02-28' , 3 ) from sys_dummy ;

-- 来源: 4027_file_4027
select '' AS to_number_14 , to_number ( '34,50' , '999,99' );

-- 来源: 4027_file_4027
select '' AS to_number_14 , to_number ( '34,50' , '999,99' );

-- 来源: 4027_file_4027
select ( - 2147483648 ):: int4 / ( - 1 ):: int4 ;

-- 来源: 4027_file_4027
select ( - 2147483648 ):: int4 / ( - 1 ):: int4 ;

-- 来源: 4027_file_4027
select * from test1 where c2 > 1 ;

-- 来源: 4027_file_4027
select * from test1 where c2 > 1 ;

-- 来源: 4027_file_4027
select length ( lpad ( '123' , 0 , '*' )) from sys_dummy ;

-- 来源: 4027_file_4027
select length ( lpad ( '123' , 0 , '*' )) from sys_dummy ;

-- 来源: 4027_file_4027
select concat ( variadic NULL :: int []) is NULL ;

-- 来源: 4027_file_4027
select concat ( variadic NULL :: int []) is NULL ;

-- 来源: 4027_file_4027
select concat ( variadic NULL :: int []) is NULL ;

-- 来源: 4027_file_4027
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- 来源: 4027_file_4027
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- 来源: 4027_file_4027
select r, r is null as isnull, r is not null as isnotnull from (values (1,row(1,2)), (1,row(null,null)), (1,null), (null,row(1,2)), (null,row(null,null)), (null,null) ) r(a,b);

-- 来源: 4027_file_4027
select r, r is null as isnull, r is not null as isnotnull from (values (1,row(1,2)), (1,row(null,null)), (1,null), (null,row(1,2)), (null,row(null,null)), (null,null) ) r(a,b);

-- 来源: 4027_file_4027
select * from tab_1 where col1 is null;

-- 来源: 4027_file_4027
select * from tab_1 where col1=' ';

-- 来源: 4027_file_4027
select * from tab_1 where col1 is null;

-- 来源: 4027_file_4027
select * from tab_1 where col1=' ';

-- 来源: 4027_file_4027
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- 来源: 4027_file_4027
select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

-- 来源: 4027_file_4027
select test(1,2);

-- 来源: 4027_file_4027
select test(1,2);

-- 来源: 4027_file_4027
select power(2,3);

-- 来源: 4027_file_4027
select count(*) from db_ind_columns;

-- 来源: 4027_file_4027
select count(index_name) from db_ind_columns;

-- 来源: 4027_file_4027
SELECT left('abcde', 2);

-- 来源: 4027_file_4027
SELECT pg_client_encoding();

-- 来源: 4027_file_4027
select power(2,3);

-- 来源: 4027_file_4027
select count(*) from db_ind_columns;

-- 来源: 4027_file_4027
select count(index_name) from db_ind_columns;

-- 来源: 4027_file_4027
SELECT left('abcde', 2);

-- 来源: 4027_file_4027
SELECT pg_client_encoding();

-- 来源: 4027_file_4027
select sysdate;

-- 来源: 4027_file_4027
select decode(c1,'x','0','default') from test;

-- 来源: 4027_file_4027
select (case c1 when 'x' then '0' else 'default' end) from test;

-- 来源: 4027_file_4027
select timestamp '2024-03-20 01:30:00’ at time zone 'Europe/Moscow' from dual;

-- 来源: 4027_file_4027
select timestamp '2024-03-20 01:30:00’ at time zone 'Europe/Moscow' from dual;

-- 来源: 4027_file_4027
select 'a' || null || 'b';

-- 来源: 4152_key
select * from gs_stat_get_hotkeys_info () order by count , hash_value ;

-- 来源: 4152_key
select * from global_stat_get_hotkeys_info () order by count , hash_value ;

-- 查询物化视图结果
-- 来源: 4273_file_4273
SELECT * FROM mv;

-- 查询物化视图结果
-- 来源: 4273_file_4273
SELECT * FROM mv;

-- 查询物化视图结果
-- 来源: 4277_file_4277
SELECT * FROM mv;

-- 查询物化视图结果
-- 来源: 4277_file_4277
select * from mv;

-- 从加密表中查询数据
-- 来源: 4280_gsql
select * from creditcard_info where name = 'joe';

-- 从系统表中查询主密钥信息
-- 来源: 4280_gsql
SELECT * FROM gs_client_global_keys;

-- 从系统表中查询列密钥信息
-- 来源: 4280_gsql
SELECT column_key_name,column_key_distributed_id ,global_key_id,key_owner FROM gs_column_keys;

-- 来源: 4283__
SELECT f_encrypt_in_sql ( 'Avi' , '1234567890123456' );

-- 来源: 4283__
SELECT f_encrypt_in_plpgsql ( 'Avi' , val2 => '1234567890123456' );

-- 来源: 4284_file_4284
SELECT relname,reloptions FROM pg_class WHERE relname = 't1';

-- 来源: 4284_file_4284
SELECT * FROM t1;

-- 来源: 4286_file_4286
SELECT * FROM gs_security_label ;

-- 来源: 4286_file_4286
SELECT * FROM gs_security_label;

-- 来源: 4287_file_4287
SELECT * FROM pg_seclabels ;

-- 来源: 4287_file_4287
SELECT * FROM pg_seclabels;

--查询t1_hash分区类型
-- 来源: 4303_file_4303
SELECT relname, parttype FROM pg_class WHERE relname = 't1_hash';

--查询t1_hash分区类型
-- 来源: 4304_file_4304
SELECT oid, relname, parttype FROM pg_class WHERE relname = 't1_hash';

--查询t1_hash的分区信息
-- 来源: 4304_file_4304
SELECT oid, relname, parttype, parentid FROM pg_partition WHERE parentid = 16685;

-- 来源: 4307_file_4307
SELECT * FROM range_sales PARTITION (p1);

-- 来源: 4307_file_4307
SELECT * FROM range_sales PARTITION (p2);

-- 来源: 4307_file_4307
SELECT * FROM range_sales PARTITION (p3);

-- 查看分区表信息
-- 来源: 4307_file_4307
SELECT relname, boundaries, spcname FROM pg_partition p JOIN pg_tablespace t ON p.reltablespace=t.oid and p.parentid='tpcds.startend_pt'::regclass ORDER BY 1;

-- 来源: 4313_DQL_DML
SELECT * FROM list_02 ORDER BY data;

-- 查询分区p_list_2数据
-- 来源: 4313_DQL_DML
SELECT * FROM list_02 PARTITION (p_list_2) ORDER BY data;

-- 查询(100)所对应的分区的数据，即分区p_list_
-- 来源: 4313_DQL_DML
SELECT * FROM list_02 PARTITION FOR (100) ORDER BY data;

-- 来源: 4320_file_4320
SELECT RELNAME FROM PG_CLASS WHERE RELKIND='i' or RELKIND='I';

-- 来源: 4322_file_4322
select relname, parttype, relpages, reltuples from pg_partition where parentid=(select oid from pg_class where relname='t1_range_int') order by relname;

-- 来源: 4322_file_4322
select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_stats where tablename='t1_range_int' order by tablename, partitionname, attname;

-- 来源: 4322_file_4322
select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_ext_stats where tablename='t1_range_int' order by tablename,partitionname,attname;

-- 来源: 4322_file_4322
select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_stats where tablename='t1_range_int_index' order by tablename,partitionname,attname;

-- 来源: 4323_file_4323
select relname, relpages, reltuples from pg_partition where relname in ('id11', 'id22', 'max_id1');

-- 来源: 4323_file_4323
select * from pg_stats where tablename ='only_fisrt_part' and partitionname ='id11';

-- 来源: 4365_init_td
SELECT * FROM pg_thread_wait_status;

-- 来源: 4387_file_4387
select * from gs_global_config where name like '%undostoragetype%';

-- 来源: 4393_file_4393
select int8in(xidout(next_csn)) from gs_get_next_xid_csn();

-- 来源: 4393_file_4393
select now();

-- 来源: 4393_file_4393
SELECT * FROM flashtest;

-- 来源: 4393_file_4393
SELECT * FROM flashtest TIMECAPSULE CSN 79351682;

-- 来源: 4393_file_4393
SELECT * FROM flashtest;

-- 来源: 4393_file_4393
SELECT * FROM flashtest TIMECAPSULE TIMESTAMP '2023-09-13 19:35:26.011986';

-- 来源: 4393_file_4393
SELECT * FROM flashtest;

-- 来源: 4393_file_4393
SELECT * FROM flashtest TIMECAPSULE TIMESTAMP to_timestamp ('2023-09-13 19:35:26.011986', 'YYYY-MM-DD HH24:MI:SS.FF');

-- 来源: 4393_file_4393
SELECT * FROM flashtest AS ft TIMECAPSULE CSN 79351682;

-- 来源: 4394_file_4394
select int8in(xidout(next_csn)) from gs_get_next_xid_csn();

-- 来源: 4394_file_4394
select now();

-- 来源: 4394_file_4394
SELECT * FROM flashtest;

-- 来源: 4394_file_4394
SELECT * FROM flashtest;

-- 来源: 4394_file_4394
SELECT * FROM flashtest;

-- 来源: 4394_file_4394
select now();

-- 来源: 4394_file_4394
SELECT * FROM flashtest;

-- 来源: 4394_file_4394
SELECT * FROM flashtest;

-- 来源: 4394_file_4394
select now();

-- 来源: 4394_file_4394
SELECT * FROM flashtest;

-- 来源: 4394_file_4394
SELECT * FROM flashtest;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4395_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4395_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4395_DROP_TRUNCATE
select * from flashtest_rename;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4395_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4395_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4407_file_4407
SELECT a.oid, a.relname FROM pg_class a inner join pg_namespace b on a.relnamespace = b.oid WHERE (a.relname = 'gsilmpolicy_seq' OR a.relname = 'gsilmtask_seq') AND b.nspname = 'public';

-- 来源: 4407_file_4407
SELECT * FROM gs_my_ilmpolicies;

-- 来源: 4407_file_4407
SELECT * FROM gs_my_ilmdatamovementpolicies;

-- 来源: 4407_file_4407
SELECT * FROM gs_my_ilmobjects;

-- 来源: 4407_file_4407
SELECT * FROM gs_my_ilmtasks;

-- 来源: 4407_file_4407
SELECT * FROM gs_my_ilmevaluationdetails;

-- 来源: 4407_file_4407
SELECT * FROM gs_my_ilmresults;

-- 来源: 4407_file_4407
SELECT * FROM gs_adm_ilmparameters;

-- 来源: 4409_TIPS
SELECT * FROM gs_adm_ilmresults ORDER BY task_id desc;

-- 来源: 4409_TIPS
SELECT * from DBE_HEAT_MAP . ROW_HEAT_MAP ( owner => 'heat_map_data' , segment_name => 'heat_map_table' , partition_name => NULL , ctid => '(0,1)' );

-- 来源: 4409_TIPS
SELECT * FROM GS_ADM_ILMPARAMETERS;

-- 来源: 4409_TIPS
SELECT * FROM GS_ADM_ILMPOLICIES;

-- 来源: 4409_TIPS
SELECT * FROM GS_MY_ILMPOLICIES;

-- 来源: 4409_TIPS
SELECT * FROM GS_ADM_ILMDATAMOVEMENTPOLICIES;

-- 来源: 4409_TIPS
SELECT * FROM GS_MY_ILMDATAMOVEMENTPOLICIES;

-- 来源: 4409_TIPS
SELECT * FROM GS_ADM_ILMOBJECTS;

-- 来源: 4409_TIPS
SELECT * FROM GS_MY_ILMOBJECTS;

-- 来源: 4409_TIPS
SELECT * FROM GS_ADM_ILMTASKS;

-- 来源: 4409_TIPS
SELECT * FROM GS_MY_ILMTASKS;

-- 来源: 4409_TIPS
SELECT * FROM GS_ADM_ILMEVALUATIONDETAILS;

-- 来源: 4409_TIPS
SELECT * FROM GS_MY_ILMEVALUATIONDETAILS;

-- 来源: 4409_TIPS
SELECT * FROM GS_ADM_ILMRESULTS;

-- 来源: 4409_TIPS
SELECT * FROM GS_MY_ILMRESULTS;

-- 来源: 4433_query
select "table", "column" from gs_index_advise('SELECT c_discount from bmsql_customer where c_w_id = 10');

-- 来源: 4433_query
select "table", "column" from gs_index_advise('select name, age, sex from t1 where age >= 18 and age < 35 and sex = ' 'f ' ';

-- 来源: 4433_query
select "table", "column", "indextype" from gs_index_advise('select name, age, sex from range_table where age = 20;

-- 来源: 4434_file_4434
select * from hypopg_create_index('create index on bmsql_customer(c_w_id)');

-- 来源: 4434_file_4434
select * from hypopg_display_index();

-- 来源: 4434_file_4434
select * from hypopg_estimate_size(329729);

-- 来源: 4434_file_4434
select * from hypopg_drop_index(329726);

-- 来源: 4434_file_4434
select * from hypopg_reset_index();

--查询物化视图结果。
-- 来源: 4493_file_4493
SELECT * FROM mv;

--查询物化视图结果。
-- 来源: 4493_file_4493
SELECT * FROM mv;

--查询物化视图结果。
-- 来源: 4497_file_4497
SELECT * FROM mv;

--查询物化视图结果。
-- 来源: 4497_file_4497
select * from mv;

-- 从加密表中查询数据
-- 来源: 4500_gsql
select * from creditcard_info where name = 'joe';

-- 从系统表中查询主密钥信息
-- 来源: 4500_gsql
SELECT * FROM gs_client_global_keys;

-- 从系统表中查询列密钥信息
-- 来源: 4500_gsql
SELECT column_key_name,column_key_distributed_id ,global_key_id,key_owner FROM gs_column_keys;

-- 来源: 4504__
SELECT f_encrypt_in_sql ( 'Avi' , '1234567890123456' );

-- 来源: 4504__
SELECT f_encrypt_in_plpgsql ( 'Avi' , val2 => '1234567890123456' );

-- 来源: 4507_gsql
select id,credit from contacts where credit > 10000;

-- 来源: 4507_gsql
select id,credit from contacts where credit < 10000;

-- 来源: 4507_gsql
select id,credit from contacts where credit >= 8000;

-- 来源: 4507_gsql
select id,credit from contacts where credit <= 8000;

-- 来源: 4507_gsql
select id,credit from contacts order by credit;

-- 来源: 4507_gsql
select id,credit from contacts order by credit DESC;

-- 来源: 4507_gsql
select credit*2 from contacts limit 1;

-- 来源: 4507_gsql
select sum(credit) from contacts;

-- 来源: 4507_gsql
select case when credit > 9000 then name end from contacts;

-- 来源: 4507_gsql
select credit::text, credit::int from contacts offset 1 limit 1;

-- 来源: 4507_gsql
select credit from contacts where name like 'zhang%';

-- 来源: 4511_file_4511
SELECT relname,reloptions FROM pg_class WHERE relname = 't1';

-- 来源: 4511_file_4511
SELECT * FROM t1;

-- 来源: 4513_file_4513
SELECT * FROM gs_security_label ;

-- 来源: 4513_file_4513
SELECT * FROM gs_security_label;

-- 来源: 4514_file_4514
SELECT * FROM pg_seclabels ;

-- 来源: 4514_file_4514
SELECT * FROM pg_seclabels;

-- 来源: 4522_DDL
SELECT * FROM pg_create_logical_replication_slot ( 'slot1' , 'mppdb_decoding' );

-- 来源: 4522_DDL
SELECT data FROM pg_logical_slot_peek_changes ( 'ldp_ddl_replica_slot' , NULL , NULL , 'enable-ddl-decoding' , 'true' , 'enable-ddl-json-format' , 'false' ) WHERE data not like 'BEGIN%' AND data not like 'COMMIT%' AND data not like '%dbe_pldeveloper.gs_source%' ;

-- 来源: 4522_DDL
SELECT * FROM pg_drop_replication_slot ( 'slot1' );

--查询t1_hash分区类型
-- 来源: 4531_file_4531
SELECT relname, parttype FROM pg_class WHERE relname = 't1_hash';

--查询t1_sub_rr分区类型
-- 来源: 4531_file_4531
SELECT relname, parttype FROM pg_class WHERE relname = 't1_sub_rr';

--查询t1_hash分区类型
-- 来源: 4532_file_4532
SELECT oid, relname, parttype FROM pg_class WHERE relname = 't1_hash';

--查询t1_hash的分区信息
-- 来源: 4532_file_4532
SELECT oid, relname, parttype, parentid FROM pg_partition WHERE parentid = 16685;

-- 来源: 4535_file_4535
SELECT * FROM range_sales PARTITION (p1);

-- 来源: 4535_file_4535
SELECT * FROM range_sales PARTITION (p2);

-- 来源: 4535_file_4535
SELECT * FROM range_sales PARTITION (p3);

-- 查看分区表信息
-- 来源: 4535_file_4535
SELECT relname, boundaries, spcname FROM pg_partition p JOIN pg_tablespace t ON p.reltablespace=t.oid and p.parentid='tpcds.startend_pt'::regclass ORDER BY 1;

-- 来源: 4543_DQL_DML
SELECT * FROM list_list_02 ORDER BY data;

-- 查询分区p_list_4数据
-- 来源: 4543_DQL_DML
SELECT * FROM list_list_02 PARTITION (p_list_4) ORDER BY data;

-- 查询(100, 100)所对应的二级分区的数据，即二级分区p_list_4_subpartdefault1，这个分区是在p_list_4下自动创建的一个分区范围定义为DEFAULT的分区
-- 来源: 4543_DQL_DML
SELECT * FROM list_list_02 SUBPARTITION FOR(100, 100) ORDER BY data;

-- 查询分区p_list_2 数据
-- 来源: 4543_DQL_DML
SELECT * FROM list_list_02 PARTITION (p_list_2) ORDER BY data;

-- 查询(0, 100)所对应的二级分区的数据，即二级分区p_list_2_
-- 来源: 4543_DQL_DML
SELECT * FROM list_list_02 SUBPARTITION FOR (0, 100) ORDER BY data;

-- 来源: 4555_file_4555
SELECT RELNAME FROM PG_CLASS WHERE RELKIND='i' or RELKIND='I';

-- 来源: 4557_file_4557
select relname, parttype, relpages, reltuples from pg_partition where parentid=(select oid from pg_class where relname='t1_range_int') order by relname;

-- 来源: 4557_file_4557
select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_stats where tablename='t1_range_int' order by tablename, partitionname, attname;

-- 来源: 4557_file_4557
select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_ext_stats where tablename='t1_range_int' order by tablename,partitionname,attname;

-- 来源: 4557_file_4557
select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_stats where tablename='t1_range_int_index' order by tablename,partitionname,attname;

-- 来源: 4558_file_4558
select relname, relpages, reltuples from pg_partition where relname in ('id11', 'id22', 'max_id1');

-- 来源: 4558_file_4558
select * from pg_stats where tablename ='only_fisrt_part' and partitionname ='id11';

-- 来源: 4625_init_td
select * from pg_thread_wait_status;

-- 来源: 4647_file_4647
select * from gs_global_config where name like '%undostoragetype%';

-- 来源: 4653_file_4653
select int8in(xidout(next_csn)) from gs_get_next_xid_csn();

-- 来源: 4653_file_4653
select now();

-- 来源: 4653_file_4653
SELECT * FROM flashtest;

-- 来源: 4653_file_4653
SELECT * FROM flashtest TIMECAPSULE CSN 79351682;

-- 来源: 4653_file_4653
SELECT * FROM flashtest;

-- 来源: 4653_file_4653
SELECT * FROM flashtest TIMECAPSULE TIMESTAMP '2023-09-13 19:35:26.011986';

-- 来源: 4653_file_4653
SELECT * FROM flashtest;

-- 来源: 4653_file_4653
SELECT * FROM flashtest TIMECAPSULE TIMESTAMP to_timestamp ('2023-09-13 19:35:26.011986', 'YYYY-MM-DD HH24:MI:SS.FF');

-- 来源: 4653_file_4653
SELECT * FROM flashtest AS ft TIMECAPSULE CSN 79351682;

-- 来源: 4654_file_4654
select int8in(xidout(next_csn)) from gs_get_next_xid_csn();

-- 来源: 4654_file_4654
select now();

-- 来源: 4654_file_4654
SELECT * FROM flashtest;

-- 来源: 4654_file_4654
SELECT * FROM flashtest;

-- 来源: 4654_file_4654
SELECT * FROM flashtest;

-- 来源: 4654_file_4654
select now();

-- 来源: 4654_file_4654
SELECT * FROM flashtest;

-- 来源: 4654_file_4654
SELECT * FROM flashtest;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4655_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4655_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4655_DROP_TRUNCATE
select * from flashtest_rename;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4655_DROP_TRUNCATE
select * from flashtest;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4655_DROP_TRUNCATE
select * from gs_recyclebin;

-- 来源: 4667_file_4667
SELECT a.oid, a.relname from pg_class a inner join pg_namespace b on a.relnamespace = b.oid WHERE (a.relname = 'gsilmpolicy_seq' OR a.relname = 'gsilmtask_seq') AND b.nspname = 'public';

-- 来源: 4667_file_4667
SELECT * FROM gs_my_ilmpolicies;

-- 来源: 4667_file_4667
SELECT * FROM gs_my_ilmdatamovementpolicies;

-- 来源: 4667_file_4667
SELECT * FROM gs_my_ilmobjects;

-- 来源: 4667_file_4667
SELECT * FROM gs_my_ilmtasks;

-- 来源: 4667_file_4667
SELECT * FROM gs_my_ilmevaluationdetails;

-- 来源: 4667_file_4667
SELECT * FROM gs_my_ilmresults;

-- 来源: 4667_file_4667
SELECT * FROM gs_adm_ilmparameters;

-- 来源: 4669_TIPS
SELECT * FROM gs_adm_ilmresults ORDER BY task_id desc;

-- 来源: 4669_TIPS
SELECT * from DBE_HEAT_MAP . ROW_HEAT_MAP ( owner => 'heat_map_data' , segment_name => 'heat_map_table' , partition_name => NULL , ctid => '(0,1)' );

-- 来源: 4669_TIPS
SELECT * FROM GS_ADM_ILMPARAMETERS;

-- 来源: 4669_TIPS
SELECT * FROM GS_ADM_ILMPOLICIES;

-- 来源: 4669_TIPS
SELECT * FROM GS_MY_ILMPOLICIES;

-- 来源: 4669_TIPS
SELECT * FROM GS_ADM_ILMDATAMOVEMENTPOLICIES;

-- 来源: 4669_TIPS
SELECT * FROM GS_MY_ILMDATAMOVEMENTPOLICIES;

-- 来源: 4669_TIPS
SELECT * FROM GS_ADM_ILMOBJECTS;

-- 来源: 4669_TIPS
SELECT * FROM GS_MY_ILMOBJECTS;

-- 来源: 4669_TIPS
SELECT * FROM GS_ADM_ILMTASKS;

-- 来源: 4669_TIPS
SELECT * FROM GS_MY_ILMTASKS;

-- 来源: 4669_TIPS
SELECT * FROM GS_ADM_ILMEVALUATIONDETAILS;

-- 来源: 4669_TIPS
SELECT * FROM GS_MY_ILMEVALUATIONDETAILS;

-- 来源: 4669_TIPS
SELECT * FROM GS_ADM_ILMRESULTS;

-- 来源: 4669_TIPS
SELECT * FROM GS_MY_ILMRESULTS;

-- 来源: 4693_query
select "table", "column" from gs_index_advise('select name, age, sex from t1 where age >= 18 and age < 35 and sex = ' 'f ' ';

-- 来源: 4693_query
select "table", "column", "indextype" from gs_index_advise('select name, age, sex from range_table where age = 20;

-- 来源: 4694_file_4694
select * from hypopg_create_index('create index on bmsql_customer(c_w_id)');

-- 来源: 4694_file_4694
select * from hypopg_display_index();

-- 来源: 4733_DB4AI
select gs_explain_model('iris_classification_model');

-- 来源: 4733_DB4AI
SELECT id , PREDICT BY iris_classification (FEATURES sepal_length,sepal_width,petal_length,petal_width ) as " PREDICT" FROM tb_iris limit 3;

-- 来源: 4733_DB4AI
select gs_explain_model("ecoli_svmc");

-- 来源: 4733_DB4AI
select id, PREDICT BY patient_logistic_regression (FEATURES second_attack,treatment) FROM patients;

-- 来源: 4733_DB4AI
select id, PREDICT BY patient_linear_regression (FEATURES second_attack) FROM patients;

-------------------------------------------------------------------------------------------------------------------------------------
-- 来源: 4733_DB4AI
select id, PREDICT BY patient_linear_regression (FEATURES 1,second_attack,treatment) FROM patients;

-- 来源: 5279_drop user
select d.datname,s.classid,s.objid from pg_roles r join pg_shdepend s on r.oid=s.refobjid join pg_database d on s.dbid=d.oid where rolname=' test1 ';

-- 来源: 5295_file_5295
select gs_create_log_tables();

-- 来源: 5778_gsql
select * from : foo ;

-- 来源: 5778_gsql
select * from HR . areaS TEST ;

-- 来源: 5779_file_5779
SELECT * FROM HR . areaS ;

-- 来源: 5779_file_5779
SELECT * FROM HR . areaS ;

-- 来源: 5779_file_5779
SELECT * FROM HR . areaS ;

-- 来源: 5779_file_5779
SELECT * FROM HR . areaS ;

-------------------------------------------------- 每个表的重分布执行时间（redis_progress_detail）： 由于该表由重分布线程创建记录，当重分布异常退出或者session连接异常时可能导致记录的时间不准确，只能作为参考，需要获取准确时间需要通过日志进行读取； 当用户表在pgxc_redistb中的redistributed字段为'y'时，用户再修改表名，该表中的table_name不会再进行更新。
-- 来源: 5824_gs_expand
select * from redis_progress_detail;

-- 来源: 5891_gsql
select * from : foo ;

-- 来源: 5891_gsql
select * from HR . areaS TEST ;

-- 来源: 5892_file_5892
SELECT * FROM HR . areaS ;

-- 来源: 5892_file_5892
SELECT * FROM HR . areaS ;

-- 来源: 5892_file_5892
SELECT * FROM HR . areaS ;

-- 来源: 5892_file_5892
SELECT * FROM HR . areaS ;

-- 来源: 733_file_733
select * from a;

-- 来源: 733_file_733
select * from a;

-- 来源: 733_file_733
select * from a;

-- 来源: 733_file_733
select * from a;

-- 来源: 733_file_733
select * from a;

-- 来源: 733_file_733
select * from a;

-- 来源: 733_file_733
select * from a;

-- 来源: 740_file_740
SELECT * FROM pg_user ;

-- 来源: 740_file_740
SELECT * FROM pg_authid ;

-- 来源: 742_Schema
SELECT s . nspname , u . usename AS nspowner FROM pg_namespace s , pg_user u WHERE nspname = 'schema_name' AND s . nspowner = u . usesysid ;

-- 来源: 742_Schema
SELECT * FROM pg_namespace ;

-- 来源: 742_Schema
SELECT distinct ( tablename ), schemaname from pg_tables where schemaname = 'pg_catalog' ;

-- 来源: 744_file_744
SELECT * FROM public . all_data ;

-- 来源: 744_file_744
SELECT * FROM public . all_data ;

-- 来源: 752_file_752
SELECT datname FROM pg_database ;

-- 来源: 754_file_754
SELECT spcname FROM pg_tablespace ;

-- 来源: 754_file_754
SELECT PG_TABLESPACE_SIZE ( 'fastspace' );

-- 来源: 759_file_759
SELECT * FROM pg_tables ;

-- 来源: 759_file_759
SELECT count ( * ) FROM customer_t1 ;

-- 来源: 759_file_759
SELECT * FROM customer_t1 ;

-- 来源: 759_file_759
SELECT c_customer_sk FROM customer_t1 ;

-- 来源: 759_file_759
SELECT DISTINCT ( c_customer_sk ) FROM customer_t1 ;

-- 来源: 759_file_759
SELECT * FROM customer_t1 WHERE c_customer_sk = 3869 ;

-- 来源: 759_file_759
SELECT * FROM customer_t1 ORDER BY c_customer_sk ;

-- 来源: 761_file_761
SELECT distinct ( tablename ) FROM pg_tables WHERE SCHEMANAME = 'public' AND TABLENAME LIKE 'search_table%' ;

-- 来源: 763_schema
SELECT * FROM myschema . mytable ;

-- 来源: 763_schema
SELECT current_schema ();

-- 来源: 764_file_764
SELECT * FROM tpcds . web_returns_p2 PARTITION ( P6 );

-- 来源: 764_file_764
SELECT * FROM tpcds . web_returns_p2 PARTITION FOR ( 35888 );

-- 来源: 764_file_764
SELECT * FROM tpcds . web_returns_p2 PARTITION ( P6 );

-- 来源: 764_file_764
SELECT * FROM tpcds . web_returns_p2 PARTITION FOR ( 35888 );

-- 来源: 765_file_765
SELECT RELNAME FROM PG_CLASS WHERE RELKIND = 'i' ;

-- 来源: 765_file_765
SELECT ca_address_sk FROM tpcds . customer_address_bak WHERE ca_address_sk = 14888 ;

-- 来源: 765_file_765
SELECT ca_address_sk , ca_address_id FROM tpcds . customer_address_bak WHERE ca_address_sk = 5050 AND ca_street_number < 1000 ;

-- 来源: 765_file_765
SELECT * FROM tpcds . customer_address_bak WHERE trunc ( ca_street_number ) < 1000 ;

-- 来源: 766_file_766
SELECT * FROM MyView ;

-- 来源: 766_file_766
SELECT * FROM my_views ;

-- 来源: 766_file_766
SELECT * FROM adm_views ;

-- 来源: 767_file_767
SELECT SETVAL ( 'newSeq1' , 10000 );

-- 来源: 768_file_768
select job , dbname , start_date , last_date , this_date , next_date , broken , status , interval , failures , what from my_jobs ;

-- 来源: 780_SQL
SELECT CURRENT_DATE ;

-- 来源: 780_SQL
SELECT CURRENT_TIME ;

-- 来源: 780_SQL
SELECT CURRENT_TIMESTAMP ( 6 );

-- 来源: 995_file_995
select table_skewness ( 'inventory' );

-- 来源: 995_file_995
select table_skewness ( 'inventory' );

