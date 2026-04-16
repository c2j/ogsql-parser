-- 类别: DML_MERGE
-- SQL 数量: 6

-- 来源: 1349_MERGE INTO
MERGE INTO products p USING newproducts np ON ( p . product_id = np . product_id ) WHEN MATCHED THEN UPDATE SET p . product_name = np . product_name , p . category = np . category WHERE p . product_name != 'play gym' WHEN NOT MATCHED THEN INSERT VALUES ( np . product_id , np . product_name , np . category ) WHERE np . category = 'books' ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
MERGE INTO range_list partition (p_201901) p USING newrange_list partition (p_201901) np ON p.month_code= np.month_code WHEN MATCHED THEN UPDATE SET dept_code = np.dept_code, user_no = np.user_no, sales_amt = np.sales_amt WHEN NOT MATCHED THEN INSERT VALUES (np.month_code, np.dept_code, np.user_no, np.sales_amt);

-- 来源: 2980_CREATE TABLE SUBPARTITION
MERGE INTO range_list partition for ('201901') p USING newrange_list partition for ('201901') np ON p.month_code= np.month_code WHEN MATCHED THEN UPDATE SET dept_code = np.dept_code, user_no = np.user_no, sales_amt = np.sales_amt WHEN NOT MATCHED THEN INSERT VALUES (np.month_code, np.dept_code, np.user_no, np.sales_amt);

-- 来源: 2980_CREATE TABLE SUBPARTITION
MERGE INTO range_list subpartition (p_201901_a) p USING newrange_list subpartition (p_201901_a) np ON p.month_code= np.month_code WHEN MATCHED THEN UPDATE SET dept_code = np.dept_code, user_no = np.user_no, sales_amt = np.sales_amt WHEN NOT MATCHED THEN INSERT VALUES (np.month_code, np.dept_code, np.user_no, np.sales_amt);

-- 来源: 2980_CREATE TABLE SUBPARTITION
MERGE INTO range_list subpartition for ('201901', '1') p USING newrange_list subpartition for ('201901', '1') np ON p.month_code= np.month_code WHEN MATCHED THEN UPDATE SET dept_code = np.dept_code, user_no = np.user_no, sales_amt = np.sales_amt WHEN NOT MATCHED THEN INSERT VALUES (np.month_code, np.dept_code, np.user_no, np.sales_amt);

-- 进行MERGE INTO操作
-- 来源: 3060_MERGE INTO
MERGE INTO products p USING newproducts np ON (p.product_id = np.product_id) WHEN MATCHED THEN UPDATE SET p.product_name = np.product_name, p.category = np.category WHERE p.product_name != 'play gym' WHEN NOT MATCHED THEN INSERT VALUES (np.product_id, np.product_name, np.category) WHERE np.category = 'books';

