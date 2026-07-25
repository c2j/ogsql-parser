-- 来源: 4693_query.txt
-- SQL 数量: 2

select "table", "column" from gs_index_advise('select name, age, sex from t1 where age >= 18 and age < 35 and sex = ' 'f ' ';

select "table", "column", "indextype" from gs_index_advise('select name, age, sex from range_table where age = 20;

