-- 来源: 2921_ALTER TEXT SEARCH DICTIONARY.txt
-- SQL 数量: 5

CREATE TEXT SEARCH DICTIONARY my_dict ( TEMPLATE = Simple );

--更改Simple类型字典，将非停用词设置为已识别，其他参数保持不变。
ALTER TEXT SEARCH DICTIONARY my_dict ( Accept = true );

--更改Simple类型字典，重置Accept参数。
ALTER TEXT SEARCH DICTIONARY my_dict ( Accept );

--更新词典定义，不实际更改任何内容。
ALTER TEXT SEARCH DICTIONARY my_dict ( dummy );

--删除字典my_dict。
DROP TEXT SEARCH DICTIONARY my_dict;

