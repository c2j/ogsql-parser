-- 来源: 4623_Ustore.txt
-- SQL 数量: 4

CREATE TABLE ustore_table(a INT PRIMARY KEY, b CHAR (20)) WITH (STORAGE_TYPE=USTORE);

drop table ustore_table;

CREATE INDEX UB_tree_index ON test(a);

drop index ub_tree_index;

