-- 来源: 4554_file_4554.txt
-- SQL 数量: 5

--INSERT常量，执行FastPath优化
explain insert into fastpath_t1 values (0, 'test_insert');

--INSERT带参数/简单表达式，执行FastPath优化
prepare insert_t1 as insert into fastpath_t1 values($1 + 1 + $2, $2);

explain execute insert_t1(10, '0');

--INSERT为子查询，无法执行FastPath优化，走标准执行器模块
create table test_1(col1 int, col3 text);

explain insert into fastpath_t1 select * from test_1;

