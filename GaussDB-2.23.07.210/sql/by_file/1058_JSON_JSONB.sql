-- 来源: 1058_JSON_JSONB.txt
-- SQL 数量: 16

SELECT 'null'::json;

SELECT 'NULL'::jsonb;

SELECT '1'::json;

SELECT '-1.5'::json;

SELECT '-1.5e-5'::jsonb, '-1.5e+2'::jsonb;

SELECT '001'::json, '+15'::json, 'NaN'::json;

SELECT 'true'::json;

SELECT '"a"'::json;

SELECT '[1, 2, "foo", null]'::json;

SELECT '[]'::json;

SELECT '[1, 2, "foo", null, [[]], {}]'::jsonb;

SELECT '{}'::json;

SELECT '{"foo": [true, "bar"], "tags": {"a": 1, "b": null}}'::jsonb;

SELECT ' [1, " a ", {"a" :1 }] '::jsonb;

SELECT '{"a" : 1, "a" : 2}'::jsonb;

SELECT '{"aa" : 1, "b" : 2, "a" : 3}'::jsonb;

