-- 来源: 4313_DQL_DML.txt
-- SQL 数量: 6

SELECT * FROM list_02 ORDER BY data;

-- 查询分区p_list_2数据
SELECT * FROM list_02 PARTITION (p_list_2) ORDER BY data;

-- 查询(100)所对应的分区的数据，即分区p_list_
SELECT * FROM list_02 PARTITION FOR (100) ORDER BY data;

DELETE FROM list_02 PARTITION (p_list_5);

-- 指定分区p_list_7插入数据，由于数据不符合该分区约束，插入报错
INSERT INTO list_02 PARTITION (p_list_7) VALUES(null, 'cherry', 'cherry data');

-- 将分区值100所属分区，即分区p_list_4的数据进行更新
UPDATE list_02 PARTITION FOR (100) SET data = '';

