-- 来源: 2821_XML.txt
-- SQL 数量: 73

SELECT XMLPARSE ( DOCUMENT '<?xml version="1.0"?><book><title>Manual</title><chapter>...</chapter></book>' );

SELECT XMLPARSE ( CONTENT 'abc<foo>bar</foo><bar>foo</bar>' );

SELECT XMLPARSE ( CONTENT 'abc<foo>bar</foo' wellformed );

SELECT XMLSERIALIZE ( CONTENT 'good' AS CHAR ( 10 ));

SELECT xmlserialize ( DOCUMENT '<head>bad</head>' as text );

SELECT xmlcomment ( 'hello' );

set xmloption=content;

select XMLCONCAT(('<?xml version="1.0" encoding="GB2312" standalone="no"?><bar>foo</bar>'),('<?xml version="1.0" encoding="GB2312" standalone="no" ?><bar>foo</bar>')) ;

select XMLCONCAT('abc>');

set a_format_version='10c';

set a_format_dev_version=s2;

set xmloption=content;

select XMLCONCAT(('<?xml version="1.0" encoding="GB2312" standalone="no"?><bar>foo</bar>'),('<?xml version="1.0" encoding="GB2312" standalone="no" ?><bar>foo</bar>')) ;

select XMLCONCAT('abc>');

SELECT xmlelement ( name foo );

SELECT xmlelement ( "entityescaping<>" , 'a$><&"b' );

SELECT xmlelement ( entityescaping "entityescaping<>" , 'a$><&"b' );

SELECT xmlelement ( noentityescaping "entityescaping<>" , 'a$><&"b' );

SELECT xmlelement(" entityescaping <> ", '<abc/>' b);

SELECT xmlelement(" entityescaping <> ", '<abc/>' as b);

SELECT xmlelement(" entityescaping <> ", xml('<abc/>') b);

SELECT xmlelement(" entityescaping <> ", xml('<abc/>') as b);

SELECT xmlelement(" entityescaping <> ", xmlattributes('entityescaping<>' " entityescaping <> "));

SELECT xmlelement(name " entityescaping <> ", xmlattributes(entityescaping 'entityescaping<>' " entityescaping <> "));

SELECT xmlelement(" entityescaping <> ", xmlattributes(noentityescaping 'entityescaping<>' " entityescaping <> "));

SELECT xmlforest ( 'abc' AS foo , 123 AS bar );

SELECT xmlpi ( name php , 'echo "hello world";

SELECT xmlroot ( '<?xml version="1.1"?><content>abc</content>' , version '1.0' , standalone yes );

CREATE TABLE xmltest ( id int , data xml );

INSERT INTO xmltest VALUES ( 1 , '<value>one</value>' );

INSERT INTO xmltest VALUES ( 2 , '<value>two</value>' );

SELECT xmlagg ( data ) FROM xmltest ;

set xmloption = document ;

SELECT xmlagg ( data ) FROM xmltest ;

DELETE FROM XMLTEST ;

INSERT INTO xmltest VALUES ( 1 , '<?xml version="1.0" encoding="GBK"?><value>one</value>' );

INSERT INTO xmltest VALUES ( 2 , '<?xml version="1.0" encoding="GBK"?><value>two</value>' );

SELECT xmlagg ( data ) FROM xmltest ;

SELECT xmlagg ( data order by id desc ) FROM xmltest ;

SELECT xmlexists ( '//town[text() = ''Toronto'']' PASSING BY REF '<towns><town>Toronto</town><town>Ottawa</town></towns>' );

SELECT xml_is_well_formed ( '<>' );

SELECT xml_is_well_formed_document ( '<pg:foo xmlns:pg="http://postgresql.org/stuff">bar</pg:foo>' );

select xml_is_well_formed_content ( 'k' );

SELECT xpath ( '/my:a/text()' , '<my:a xmlns:my="http://example.com">test</my:a>' , ARRAY [ ARRAY [ 'my' , 'http://example.com' ]]);

SELECT xpath_exists ( '/my:a/text()' , '<my:a xmlns:my="http://example.com">test</my:a>' , ARRAY [ ARRAY [ 'my' , 'http://example.com' ]]);

CREATE SCHEMA testxmlschema ;

CREATE TABLE testxmlschema . test1 ( a int , b text );

INSERT INTO testxmlschema . test1 VALUES ( 1 , 'one' ), ( 2 , 'two' ), ( - 1 , null );

create database test ;

SELECT query_to_xml ( 'SELECT * FROM testxmlschema.test1' , false , false , '' );

SELECT query_to_xmlschema ( 'SELECT * FROM testxmlschema.test1' , false , false , '' );

SELECT query_to_xml_and_xmlschema ( 'SELECT * FROM testxmlschema.test1' , true , true , '' );

CURSOR xc WITH HOLD FOR SELECT * FROM testxmlschema . test1 ORDER BY 1 , 2 ;

SELECT cursor_to_xml ( 'xc' :: refcursor , 5 , false , true , '' );

SELECT cursor_to_xmlschema ( 'xc' :: refcursor , true , false , '' );

SELECT schema_to_xml ( 'testxmlschema' , false , true , '' );

SELECT schema_to_xmlschema ( 'testxmlschema' , false , true , '' );

SELECT schema_to_xml_and_xmlschema ( 'testxmlschema' , true , true , 'foo' );

SELECT database_to_xml ( true , true , 'test' );

SELECT database_to_xmlschema ( true , true , 'test' );

SELECT database_to_xml_and_xmlschema ( true , true , 'test' );

SELECT table_to_xml ( 'testxmlschema.test1' , false , false , '' );

SELECT table_to_xmlschema ( 'testxmlschema.test1' , false , false , '' );

SELECT table_to_xml_and_xmlschema ( 'testxmlschema.test1' , false , false , '' );

SET a_format_version = '10c' ;

SET a_format_dev_version = 's4' ;

DECLARE xmldata xml ;

SELECT getclobval ( xmlparse ( document '<a>123</a>' ));

SET a_format_version='10c';

SET a_format_dev_version='s4';

DECLARE xmldata xml;

SELECT getstringval(xmlparse(document '<a>123<b>456</b></a>'));

SELECT xmlsequence(xml('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>'));

