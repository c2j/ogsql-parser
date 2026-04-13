-- 来源: 2851_file_2851.txt
-- SQL 数量: 8

create table table1 ( c_int int , c_bigint bigint , c_varchar varchar , c_text text ) with ( orientation = row , storage_type = ASTORE );

create text search configuration ts_conf_1 ( parser = POUND );

create text search configuration ts_conf_2 ( parser = POUND ) with ( split_flag = '%' );

set default_text_search_config = 'ts_conf_1' ;

create index idx1 on table1 using gin ( to_tsvector ( c_text ));

set default_text_search_config = 'ts_conf_2' ;

create index idx2 on table1 using gin ( to_tsvector ( c_text ));

select c_varchar , to_tsvector ( c_varchar ) from table1 where to_tsvector ( c_text ) @@ plainto_tsquery ( '￥#@……&**' ) and to_tsvector ( c_text ) @@ plainto_tsquery ( '某公司 ' ) and c_varchar is not null order by 1 desc limit 3 ;

