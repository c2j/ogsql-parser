-- 来源: 2946_CREATE DATA SOURCE.txt
-- SQL 数量: 8

CREATE DATA SOURCE ds_test1;

--创建一个Data Source对象，含TYPE信息，VERSION为NULL。
CREATE DATA SOURCE ds_test2 TYPE 'MPPDB' VERSION NULL;

--创建一个Data Source对象，仅含OPTIONS。
CREATE DATA SOURCE ds_test3 OPTIONS (dsn ' GaussDB ', encoding 'utf8');

--创建一个Data Source对象，含TYPE, VERSION, OPTIONS。
CREATE DATA SOURCE ds_test4 TYPE 'unknown' VERSION '11.2.3' OPTIONS (dsn ' GaussDB ', username 'userid', password '********', encoding '');

--删除Data Source对象。
DROP DATA SOURCE ds_test1;

DROP DATA SOURCE ds_test2;

DROP DATA SOURCE ds_test3;

DROP DATA SOURCE ds_test4;

