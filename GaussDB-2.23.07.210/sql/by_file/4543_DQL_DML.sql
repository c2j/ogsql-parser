-- 来源: 4543_DQL_DML.txt
-- SQL 数量: 8

SELECT * FROM list_list_02 ORDER BY data;

-- 查询分区p_list_4数据
SELECT * FROM list_list_02 PARTITION (p_list_4) ORDER BY data;

-- 查询(100, 100)所对应的二级分区的数据，即二级分区p_list_4_subpartdefault1，这个分区是在p_list_4下自动创建的一个分区范围定义为DEFAULT的分区
SELECT * FROM list_list_02 SUBPARTITION FOR(100, 100) ORDER BY data;

-- 查询分区p_list_2 数据
SELECT * FROM list_list_02 PARTITION (p_list_2) ORDER BY data;

-- 查询(0, 100)所对应的二级分区的数据，即二级分区p_list_2_
SELECT * FROM list_list_02 SUBPARTITION FOR (0, 100) ORDER BY data;

DELETE FROM list_list_02 PARTITION (p_list_5);

-- 指定分区p_list_7_1插入数据，由于数据不符合该分区约束，插入报错
INSERT INTO list_list_02 SUBPARTITION (p_list_7_1) VALUES(null, 'cherry', 'cherry data');

-- 将一级分区值100所属分区的数据进行更新
UPDATE list_list_02 PARTITION FOR (100) SET id = 1;

