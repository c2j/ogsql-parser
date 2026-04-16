-- 来源: 1124_XML.txt
-- SQL 数量: 42

SELECT XMLPARSE ( DOCUMENT '<?xml version="1.0"?><book><title>Manual</title><chapter>...</chapter></book>' );

SELECT XMLPARSE ( CONTENT 'abc<foo>bar</foo><bar>foo</bar>' );

SELECT XMLPARSE ( CONTENT 'abc<foo>bar</foo' wellformed );

set xmloption=content;

select XMLCONCAT(('<?xml version="1.0" encoding="GB2312" standalone="no"?><bar>foo</bar>'),('<?xml version="1.0" encoding="GB2312" standalone="no" ?><bar>foo</bar>')) ;

select XMLCONCAT('abc>');

set a_format_version='10c';

set a_format_dev_version=s2;

set xmloption=content;

select XMLCONCAT(('<?xml version="1.0" encoding="GB2312" standalone="no"?><bar>foo</bar>'),('<?xml version="1.0" encoding="GB2312" standalone="no" ?><bar>foo</bar>')) ;

select XMLCONCAT('abc>');

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

set a_format_version = '10c' ;

set a_format_dev_version = 's4' ;

declare xmldata xml ;

select getclobval ( xmlparse ( document '<a>123</a>' ));

set a_format_version='10c';

set a_format_dev_version='s4';

declare xmldata xml;

select getstringval(xmlparse(document '<a>123<b>456</b></a>'));

SELECT xmlsequence(xml('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>'));

