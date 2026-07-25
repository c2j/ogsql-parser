-- 来源: 761_file_761.txt
-- SQL 数量: 6

CREATE TABLE public.search_table_t1(a int) distribute by hash(a);

CREATE TABLE public.search_table_t2(b int) distribute by hash(b);

CREATE TABLE public.search_table_t3(c int) distribute by hash(c);

CREATE TABLE public.search_table_t4(d int) distribute by hash(d);

CREATE TABLE public.search_table_t5(e int) distribute by hash(e);

SELECT distinct ( tablename ) FROM pg_tables WHERE SCHEMANAME = 'public' AND TABLENAME LIKE 'search_table%' ;

