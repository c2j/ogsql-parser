-- 来源: 4433_query.txt
-- SQL 数量: 3

select "table", "column" from gs_index_advise('SELECT c_discount from bmsql_customer where c_w_id = 10');

select "table", "column" from gs_index_advise('select name, age, sex from t1 where age >= 18 and age < 35 and sex = ' 'f ' ';

select "table", "column", "indextype" from gs_index_advise('select name, age, sex from range_table where age = 20;

