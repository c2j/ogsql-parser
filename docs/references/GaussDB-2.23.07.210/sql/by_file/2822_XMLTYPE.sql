-- 来源: 2822_XMLTYPE.txt
-- SQL 数量: 37

SELECT createxml ( '<a>123</a>' );

SELECT xmltype . createxml ( '<a>123</a>' );

select xmltype ( '<a>123<b>456</b></a>' ). extract ( '/a/b' ). getstringval ();

select getstringval ( extractxml ( xmltype ( '<a>123<b>456</b></a>' ), '/a/b' ));

declare a xmltype ;

declare xmltype_clob clob ;

declare xmltype_blob blob ;

SELECT getblobval ( xmltype ( '<asd/>' ), 7 );

select xmltype ( '<asd/>' ). getblobVal ( 7 );

SELECT getclobval ( xmltype ( '<a>123</a>' ));

SELECT xmltype ( '<a>123</a>' ). getclobval ();

SELECT getnumberval ( xmltype ( '<a>123</a>' ). extract ( '/a/text()' ));

SELECT xmltype ( '<a>123</a>' ). extract ( '/a/text()' ). getnumberval ();

SELECT isfragment ( xmltype ( '<a>123</a>' ));

SELECT xmltype ( '<a>123</a>' ). isfragment ();

SELECT xmltype ( '<a>123</a>' );

declare xmltype_clob clob ;

declare xmltype_blob blob ;

select getstringval('<a>123<b>456</b></a>');

select xmltype('<a>123<b>456</b></a>').getstringval();

select getrootelement('<a>123<b>456</b></a>');

select xmltype('<a>123<b>456</b></a>').getrootelement();

select getnamespace('<c:a xmlns:c="asd">123<d:b xmlns:d="qwe">456</d:b></c:a>');

select xmltype('<c:a xmlns:c="asd">123<d:b xmlns:d="qwe">456</d:b></c:a>').getnamespace();

select existsnode('<a>123<b>456</b></a>','/a/b');

select xmltype('<a>123<b>456</b></a>').existsnode('/a/b');

select existsnode('<a:b xmlns:a="asd">123<c>456</c></a:b>','/a:b/c','xmlns:a="asd"');

select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').existsnode('/a:b/c','xmlns:a="asd"');

select extractxml('<a>123<b>456</b></a>','/a/b');

select xmltype('<a>123<b>456</b></a>').extract('/a/b');

select xmltype('<a>123<b>456</b></a>').extractxml('/a/b');

select extractxml('<a:b xmlns:a="asd">123<c>456</c></a:b>','/a:b','xmlns:a="asd"');

select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').extract('/a:b','xmlns:a="asd"');

select xmltype('<a:b xmlns:a="asd">123<c>456</c></a:b>').extractxml('/a:b','xmlns:a="asd"');

SELECT xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>'));

SELECT unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//title/text()'))) AS title , unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//author/text()'))) AS author;

SELECT array_to_json(array_agg(row_to_json(t))) FROM ( SELECT unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//title/text()'))) AS title , unnest(xmlsequence(xmltype('<books><book><title>The Catcher in the Rye</title><author>J.D. Salinnger</author><year>1951</year></book><book><title>1984</title><author>George Orwell</author><year>1949</year></book><book><title>The Hitchhiker''s Guide to the Galaxy</title><author>Douglas Adams</author><year>1979</year></book></books>').extract('//author/text()'))) AS author ) t;

