-- 来源: 2754_JSON_JSONB.txt
-- SQL 数量: 25

SELECT 'null'::json;

SELECT 'NULL'::jsonb;

SELECT '1'::json;

SELECT '-1.5'::json;

SELECT '-1.5e-5'::jsonb, '-1.5e+2'::jsonb;

SELECT '001'::json, '+15'::json, 'NaN'::json;

SELECT 'true'::json;

SELECT 'false'::jsonb;

SELECT '"a"'::json;

SELECT '"abc"'::jsonb;

SELECT '[1, 2, "foo", null]'::json;

SELECT '[]'::json;

SELECT '[1, 2, "foo", null, [[]], {}]'::jsonb;

SELECT '{}'::json;

SELECT '{"a": 1, "b": {"a": 2, "b": null}}'::json;

SELECT '{"foo": [true, "bar"], "tags": {"a": 1, "b": null}}'::jsonb;

SELECT ' [1, " a ", {"a" :1 }] '::jsonb;

SELECT '{"a" : 1, "a" : 2}'::jsonb;

SELECT '{"aa" : 1, "b" : 2, "a" : 3}'::jsonb;

SELECT '"foo"'::jsonb @> '"foo"'::jsonb;

SELECT '[1, "aa", 3]'::jsonb ? 'aa';

SELECT '[1, 2, 3]'::jsonb @> '[1, 3, 1]'::jsonb;

SELECT '{"product": "PostgreSQL", "version": 9.4, "jsonb":true}'::jsonb @> '{"version":9.4}'::jsonb;

SELECT '[1, 2, [1, 3]]'::jsonb @> '[1, 3]'::jsonb;

SELECT '{"foo": {"bar": "baz"}}'::jsonb @> '{"bar": "baz"}'::jsonb;

