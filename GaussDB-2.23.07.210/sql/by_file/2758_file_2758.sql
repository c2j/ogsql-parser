-- 来源: 2758_file_2758.txt
-- SQL 数量: 6

create table t1 ( a int );

insert into t1 values ( 1 ),( 2 );

CREATE OR REPLACE FUNCTION showall () RETURNS SETOF record AS $$ SELECT count ( * ) from t1 ;

SELECT showall ();

DROP FUNCTION showall ();

drop table t1 ;

