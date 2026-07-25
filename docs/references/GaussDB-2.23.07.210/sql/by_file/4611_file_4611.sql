-- 来源: 4611_file_4611.txt
-- SQL 数量: 5

CREATE TABLE TEST(a int);

CREATE TABLE TEST1(a int) with(orientation=row, storage_type=ustore);

CREATE TABLE TEST2(a int) with(orientation=row, storage_type=astore);

create table test4(a int) with(orientation=row);

show enable_default_ustore_table;

