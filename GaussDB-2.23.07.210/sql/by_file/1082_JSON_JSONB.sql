-- 来源: 1082_JSON_JSONB.txt
-- SQL 数量: 106

SELECT array_to_json('{{1,5},{99,100}}'::int[]);

SELECT row_to_json(row(1,'foo'));

SELECT json_array_element('[1,true,[1,[2,3]],null]',2);

SELECT json_array_element_text('[1,true,[1,[2,3]],null]',2);

SELECT json_object_field('{"a": {"b":"foo"}}','a');

SELECT json_object_field_text('{"a": {"b":"foo"}}','a');

SELECT json_extract_path('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', 'f4','f6');

SELECT json_extract_path_op('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', ARRAY['f4','f6']);

SELECT json_extract_path_text('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', 'f4','f6');

SELECT json_extract_path_text_op('{"f2":{"f3":1},"f4":{"f5":99,"f6":"stringy"}}', ARRAY['f4','f6']);

SELECT json_array_elements('[1,true,[1,[2,3]],null]');

SELECT * FROM json_array_elements_text('[1,true,[1,[2,3]],null]');

SELECT json_array_length('[1,2,3,{"f1":1,"f2":[5,6]},4,null]');

SELECT * FROM json_each('{"f1":[1,2,3],"f2":{"f3":1},"f4":null}');

SELECT * FROM json_each_text('{"f1":[1,2,3],"f2":{"f3":1},"f4":null}');

SELECT json_object_keys('{"f1":"abc","f2":{"f3":"a", "f4":"b"}, "f1":"abcd"}');

SELECT jsonb_object_keys('{"f1":"abc","f2":{"f3":"a", "f4":"b"}, "f1":"abcd"}');

CREATE TYPE jpop AS (a text, b int, c bool);

SELECT * FROM json_populate_record(null::jpop,'{"a":"blurfl","x":43.2}');

SELECT * FROM json_populate_record((1,1,null)::jpop,'{"a":"blurfl","x":43.2}');

DROP TYPE jpop;

CREATE TYPE jpop AS (a text, b int, c bool);

SELECT * FROM json_populate_recordset(null::jpop, '[{"a":1,"b":2},{"a":3,"b":4}]');

DROP TYPE jpop;

SELECT value, json_typeof(value) FROM (values (json '123.4'), (json '"foo"'), (json 'true'), (json 'null'), (json '[1, 2, 3]'), (json '{"x":"foo", "y":123}'), (NULL::json)) AS data(value);

SELECT json_build_array('a',1,'b',1.2,'c',true,'d',null,'e',json '{"x": 3, "y": [1,2,3]}','');

SELECT json_build_object(1,2);

SELECT jsonb_build_object('a',1,'b',1.2,'c',true,'d',null,'e',json '{"x": 3, "y": [1,2,3]}');

SELECT jsonb_build_object();

SELECT * FROM json_to_record('{"a":1,"b":"foo","c":"bar"}',true) AS x(a int, b text, d text);

SELECT * FROM json_to_record('{"a": {"x": 1, "y": 2},"b":"foo","c":[1, 2]}') AS x(a json, b text, c int[]);

SELECT * FROM json_to_recordset('[{"a":1,"b":"foo","d":false},{"a":2,"b":"bar","c":true}]',false) AS x(a int, b text, c boolean);

SELECT json_object('{a,1,b,2,3,NULL,"d e f","a b c"}');

SELECT json_object('{a,b,"a b c"}', '{a,1,1}');

SELECT json_object('d',2,'c','name','b',true,'a',2,'a',NULL,'d',1);

SELECT json_object('d',2,true,'name','b',true,'a',2,'aa', current_timestamp);

SELECT json_array_append('[1, [2, 3]]', '$[1]', 4, '$[0]', false, '$[0]', null, '$[0]', current_timestamp);

SELECT json_array();

SELECT json_array(TRUE, FALSE, NULL, 114, 'text', current_timestamp);

SELECT json_array_insert('[1, [2, 3]]', '$[1]', 4);

SELECT json_array_insert('{"x": 1, "y": [1, 2]}', '$.y[0]', NULL, '$.y[0]', 123, '$.y[3]', current_timestamp);

SELECT json_contains('[1, 2, {"x": 3}]', '{"x":3}');

SELECT json_contains('[1, 2, {"x": 3},[1,2,3,4]]', '2','$[1]');

SELECT json_contains('[1, 2, {"x": 3},[1,2,3,4]]', '1','$[1]');

SELECT json_contains_path('[1, 2, {"x": 3}]', 'all', '$[0]', '$[1]', '$[2]');

SELECT json_contains_path('[1, 2, {"x": 3}]', 'all', '$[0]', '$[1]', '$[6]');

SELECT json_contains_path('[1, 2, {"x": 3}]', 'one', '$[0]', '$[1]', '$[5]');

SELECT json_depth('[]');

SELECT json_depth('{"s":1, "x":2,"y":[1]}');

SELECT json_extract('[1, 2, {"x": 3}]', '$[2]');

SELECT json_extract('["a", ["b", "c"], "d"]', '$[1]', '$[2]', '$[3]');

SELECT json_insert('[1, [2, 3], {"a": [4, 5]}]', '$[3]', 2);

SELECT json_insert('[1, [2, 3], {"a": [4, 5]}]', '$[10]', 10,'$[5]', 5);

SELECT json_keys('{"x": 1, "y": 2, "z": 3}');

SELECT json_keys('[1,2,3,{"name":"Tom"}]','$[3]');

SELECT json_length('[1,2,3,4,5]');

SELECT json_length('{"name":"Tom", "age":24, "like":"football"}');

SELECT json_merge('[1, 2]','[2]');

SELECT json_merge('{"b":"2"}','{"a":"1"}','[1,2]');

SELECT json_quote('gauss');

SELECT json_unquote('"gauss"');

SELECT json_remove('[0, 1, 2, [3, 4]]', '$[0]', '$[2]');

SELECT json_remove('[0, 1, 2, [3, 4]]', '$[0]', '$[0]','$[0]');

SELECT json_replace('{"x": 1}', '$.x', 'true');

SELECT json_replace('{"x": 1}', '$.x', true, '$.x', 123, '$.x', 'asd', '$.x', null);

SELECT json_search('{"a":"abc","b":"abc"}','all','abc');

SELECT json_search('{"a":"abc","b":"abc"}','one','abc');

SELECT json_search('{"a":"abc","b":"a%c"}','one','a\%c');

SELECT json_set('{"s":3}','$.s','d');

SELECT json_set('{"s":3}','$.a','d','$.a','1');

SELECT json_type('{"w":{"2":3},"2":4}');

SELECT json_type('[1,2,2,3,3,4,4,4,4,4,4,4,4]');

SELECT json_valid('{"name":"Tom"}');

SELECT json_valid('[1,23,4,5,5]');

SELECT json_valid('[1,23,4,5,5]}');

CREATE TABLE classes(name varchar, score int);

INSERT INTO classes VALUES('A',2);

INSERT INTO classes VALUES('A',3);

INSERT INTO classes VALUES('D',5);

INSERT INTO classes VALUES('D',null);

SELECT * FROm classes;

SELECT name, json_agg(score) score FROM classes GROUP BY name ORDER BY name;

DROP TABLE classes;

CREATE TABLE classes(name varchar, score int);

INSERT INTO classes VALUES('A',2);

INSERT INTO classes VALUES('A',3);

INSERT INTO classes VALUES('D',5);

INSERT INTO classes VALUES('D',null);

SELECT * FROM classes;

SELECT json_object_agg(name, score) FROM classes GROUP BY name ORDER BY name;

DROP TABLE classes;

SELECT jsonb_contained('[1,2,3]', '[1,2,3,4]');

SELECT jsonb_contains('[1,2,3,4]', '[1,2,3]');

SELECT jsonb_exists('["1",2,3]', '1');

SELECT jsonb_exists_all('["1","2",3]', '{1, 2}');

SELECT jsonb_exists_any('["1","2",3]', '{1, 2, 4}');

SELECT jsonb_cmp('["a", "b"]', '{"a":1, "b":2}');

SELECT jsonb_eq('["a", "b"]', '{"a":1, "b":2}');

SELECT jsonb_ne('["a", "b"]', '{"a":1, "b":2}');

SELECT jsonb_gt('["a", "b"]', '{"a":1, "b":2}');

SELECT jsonb_ge('["a", "b"]', '{"a":1, "b":2}');

SELECT jsonb_lt('["a", "b"]', '{"a":1, "b":2}');

SELECT jsonb_le('["a", "b"]', '{"a":1, "b":2}');

SELECT to_json('{1,5}'::text[]);

SELECT to_jsonb(array[1, 2, 3, 4]);

SELECT jsonb_hash('[1,2,3]');

