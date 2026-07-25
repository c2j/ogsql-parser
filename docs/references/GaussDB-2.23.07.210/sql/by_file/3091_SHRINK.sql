-- 来源: 3091_SHRINK.txt
-- SQL 数量: 5

CREATE TABLE row_compression ( id int ) WITH (compresstype=2, compress_chunk_size = 512, compress_level = 1);

--插入数据
INSERT INTO row_compression SELECT generate_series(1,1000);

--查看数据
SELECT * FROM row_compression;

--shrink整理
SHRINK TABLE row_compression;

--删除表
DROP TABLE row_compression;

