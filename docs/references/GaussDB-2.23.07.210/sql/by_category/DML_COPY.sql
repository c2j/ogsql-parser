-- 类别: DML_COPY
-- SQL 数量: 24

-- 来源: 1237_COPY
COPY tpcds . ship_mode TO '/home/omm/ds_ship_mode.dat' ;

-- 来源: 1237_COPY
COPY tpcds . ship_mode TO STDOUT ;

-- 来源: 1237_COPY
COPY tpcds . ship_mode TO STDOUT WITH ( delimiter ',' , encoding 'utf8' );

-- 来源: 1237_COPY
COPY tpcds . ship_mode TO STDOUT WITH ( format 'CSV' , force_quote ( SM_SHIP_MODE_SK ));

-- 来源: 1237_COPY
COPY tpcds . ship_mode_t1 FROM STDIN ;

-- 来源: 1237_COPY
COPY tpcds . ship_mode_t1 FROM '/home/omm/ds_ship_mode.dat' ;

-- 来源: 1237_COPY
COPY tpcds . ship_mode_t1 FROM '/home/omm/ds_ship_mode.dat' TRANSFORM ( SM_TYPE AS LEFT ( SM_TYPE , 10 ));

-- 来源: 1237_COPY
COPY tpcds . ship_mode_t1 FROM '/home/omm/ds_ship_mode.dat' WITH ( format 'text' , delimiter E '\t' , ignore_extra_data 'true' , noescaping 'true' );

-- 来源: 1237_COPY
COPY tpcds . ship_mode_t1 FROM '/home/omm/ds_ship_mode.dat' FIXED FORMATTER ( SM_SHIP_MODE_SK ( 0 , 2 ), SM_SHIP_MODE_ID ( 2 , 16 ), SM_TYPE ( 18 , 30 ), SM_CODE ( 50 , 10 ), SM_CARRIER ( 61 , 20 ), SM_CONTRACT ( 82 , 20 )) header ignore_extra_data ;

--将 tpcds. ship_mode中的数据拷贝到/home/ omm /ds_ship_mode.dat文件中。
-- 来源: 2939_COPY
COPY tpcds. ship_mode TO '/home/ omm /ds_ship_mode.dat';

--将 tpcds. ship_mode 输出到STDOUT。
-- 来源: 2939_COPY
COPY tpcds. ship_mode TO STDOUT;

--将 tpcds. ship_mode 的数据输出到STDOUT，使用参数如下：分隔符为','(delimiter ',')，编码格式为UTF8(encoding 'utf8')。
-- 来源: 2939_COPY
COPY tpcds. ship_mode TO STDOUT WITH (delimiter ',', encoding 'utf8');

--将 tpcds. ship_mode 的数据输出到STDOUT，使用参数如下：导入格式为CSV（format 'CSV'），引号包围SM_SHIP_MODE_SK字段的导出内容(force_quote(SM_SHIP_MODE_SK))。
-- 来源: 2939_COPY
COPY tpcds. ship_mode TO STDOUT WITH (format 'CSV', force_quote(SM_SHIP_MODE_SK));

--从STDIN拷贝数据到表 tpcds. ship_mode_t1。
-- 来源: 2939_COPY
COPY tpcds. ship_mode_t1 FROM STDIN;

--从/home/ omm /ds_ship_mode.dat文件拷贝数据到表 tpcds. ship_mode_t1。
-- 来源: 2939_COPY
COPY tpcds. ship_mode_t1 FROM '/home/ omm /ds_ship_mode.dat';

--从/home/ omm /ds_ship_mode.dat文件拷贝数据到表 tpcds. ship_mode_t1，应用TRANSFORM表达式转换，取SM_TYPE列左边10个字符插入到表中。
-- 来源: 2939_COPY
COPY tpcds. ship_mode_t1 FROM '/home/ omm /ds_ship_mode.dat' TRANSFORM (SM_TYPE AS LEFT(SM_TYPE, 10));

--从/home/ omm /ds_ship_mode.dat文件拷贝数据到表 tpcds. ship_mode_t1，使用参数如下：导入格式为TEXT（format 'text'），分隔符为'\t'（delimiter E'\t'），忽略多余列（ignore_extra_data 'true'），不指定转义（noescaping 'true'）。
-- 来源: 2939_COPY
COPY tpcds. ship_mode_t1 FROM '/home/ omm /ds_ship_mode.dat' WITH(format 'text', delimiter E'\t', ignore_extra_data 'true', noescaping 'true');

--从/home/ omm /ds_ship_mode.dat文件拷贝数据到表 tpcds. ship_mode_t1，使用参数如下：导入格式为FIXED（FIXED），指定定长格式（FORMATTER(SM_SHIP_MODE_SK(0, 2), SM_SHIP_MODE_ID(2,16), SM_TYPE(18,30), SM_CODE(50,10), SM_CARRIER(61,20), SM_CONTRACT(82,20))），忽略多余列（ignore_extra_data），有数据头（header）。
-- 来源: 2939_COPY
COPY tpcds. ship_mode_t1 FROM '/home/ omm /ds_ship_mode.dat' FIXED FORMATTER(SM_SHIP_MODE_SK(0, 2), SM_SHIP_MODE_ID(2,16), SM_TYPE(18,30), SM_CODE(50,10), SM_CARRIER(61,20), SM_CONTRACT(82,20)) header ignore_extra_data;

-- 来源: 5778_gsql
copy HR . areaS from STDIN ;

-- 来源: 5839_gs_rescue
copy original from '/data2/file01';

-- 来源: 5839_gs_rescue
copy amend from '/data2/file02';

-- 来源: 5891_gsql
copy HR . areaS from STDIN ;

-- 来源: 5949_gs_rescue
copy original from '/data2/file01';

-- 来源: 5949_gs_rescue
copy amend from '/data2/file02';

