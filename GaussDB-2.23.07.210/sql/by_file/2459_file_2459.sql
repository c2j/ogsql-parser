-- 来源: 2459_file_2459.txt
-- SQL 数量: 6

CREATE TABLE public.search_table_t1(a int);

CREATE TABLE public.search_table_t2(b int);

CREATE TABLE public.search_table_t3(c int);

CREATE TABLE public.search_table_t4(d int);

CREATE TABLE public.search_table_t5(e int);

SELECT distinct ( tablename ) FROM pg_tables WHERE SCHEMANAME = 'public' AND TABLENAME LIKE 'search_table%' ;

