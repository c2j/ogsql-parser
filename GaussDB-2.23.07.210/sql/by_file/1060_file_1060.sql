-- 来源: 1060_file_1060.txt
-- SQL 数量: 15

CREATE TABLE reservation (room int, during tsrange);

INSERT INTO reservation VALUES (1108, '[2010-01-01 14:30, 2010-01-01 15:30)');

-- 包含 。
SELECT int4range(10, 20) @> 3;

-- 判断是否重叠
SELECT numrange(11.1, 22.2) && numrange(20.0, 30.0);

-- 抽取上界 。
SELECT upper(int8range(15, 25));

-- 计算交集 。
SELECT int4range(10, 20) * int4range(15, 25);

-- 判断范围是否为空 。
SELECT isempty(numrange(1, 5));

SELECT '[3,7)'::int4range;

-- 既不包括 3 也不包括 7，但是包括之间的所有点 。
SELECT '(3,7)'::int4range;

-- 只包括单独一个点 4 。
SELECT '[4,4]'::int4range;

-- 不包括点（并且将被标准化为 '空'） 。
SELECT '[4,4)'::int4range;

SELECT numrange(1.0, 14.0, '(]');

-- 如果第三个参数被忽略，则假定为 '[)'。
SELECT numrange(1.0, 14.0);

-- 尽管这里指定了 '(]'，显示时该值将被转换成标准形式，因为 int8range 是一种离散范围类型（见下文）。
SELECT int8range(1, 14, '(]');

-- 为一个界限使用 NULL 导致范围在那一边是无界的。
SELECT numrange(NULL, 2.2);

