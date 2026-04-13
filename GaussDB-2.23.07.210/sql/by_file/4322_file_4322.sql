-- 来源: 4322_file_4322.txt
-- SQL 数量: 11

create table t1_range_int ( c1 int, c2 int, c3 int, c4 int ) partition by range(c1) ( partition range_p00 values less than(10), partition range_p01 values less than(20), partition range_p02 values less than(30), partition range_p03 values less than(40), partition range_p04 values less than(50) );

insert into t1_range_int select v,v,v,v from generate_series(0, 49) as v;

analyze t1_range_int with all;

select relname, parttype, relpages, reltuples from pg_partition where parentid=(select oid from pg_class where relname='t1_range_int') order by relname;

select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_stats where tablename='t1_range_int' order by tablename, partitionname, attname;

ALTER TABLE t1_range_int ADD STATISTICS ((c2, c3));

analyze t1_range_int with all;

select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_ext_stats where tablename='t1_range_int' order by tablename,partitionname,attname;

create index t1_range_int_index on t1_range_int(text(c1)) local;

analyze t1_range_int with all;

select schemaname,tablename,partitionname,subpartitionname,attname,inherited,null_frac,avg_width,n_distinct,n_dndistinct,most_common_vals,most_common_freqs,histogram_bounds from pg_stats where tablename='t1_range_int_index' order by tablename,partitionname,attname;

