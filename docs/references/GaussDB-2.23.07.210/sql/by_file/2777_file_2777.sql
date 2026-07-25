-- 来源: 2777_file_2777.txt
-- SQL 数量: 29

SELECT inet '192.168.1.5' < inet '192.168.1.6' AS RESULT ;

SELECT inet '192.168.1.5' <= inet '192.168.1.5' AS RESULT ;

SELECT inet '192.168.1.5' = inet '192.168.1.5' AS RESULT ;

SELECT inet '192.168.1.5' >= inet '192.168.1.5' AS RESULT ;

SELECT inet '192.168.1.5' > inet '192.168.1.4' AS RESULT ;

SELECT inet '192.168.1.5' <> inet '192.168.1.4' AS RESULT ;

SELECT inet '192.168.1.5' << inet '192.168.1/24' AS RESULT ;

SELECT inet '192.168.1/24' <<= inet '192.168.1/24' AS RESULT ;

SELECT inet '192.168.1/24' >> inet '192.168.1.5' AS RESULT ;

SELECT inet '192.168.1/24' >>= inet '192.168.1/24' AS RESULT ;

SELECT ~ inet '192.168.1.6' AS RESULT ;

SELECT inet '192.168.1.6' & inet '10.0.0.0' AS RESULT ;

SELECT inet '192.168.1.6' | inet '10.0.0.0' AS RESULT ;

SELECT inet '192.168.1.6' + 25 AS RESULT ;

SELECT inet '192.168.1.43' - 36 AS RESULT ;

SELECT inet '192.168.1.43' - inet '192.168.1.19' AS RESULT ;

SELECT abbrev ( inet '10.1.0.0/16' ) AS RESULT ;

SELECT abbrev ( cidr '10.1.0.0/16' ) AS RESULT ;

SELECT broadcast ( '192.168.1.5/24' ) AS RESULT ;

SELECT family ( '127.0.0.1' ) AS RESULT ;

SELECT host ( '192.168.1.5/24' ) AS RESULT ;

SELECT hostmask ( '192.168.23.20/30' ) AS RESULT ;

SELECT masklen ( '192.168.1.5/24' ) AS RESULT ;

SELECT netmask ( '192.168.1.5/24' ) AS RESULT ;

SELECT network ( '192.168.1.5/24' ) AS RESULT ;

SELECT set_masklen ( '192.168.1.5/24' , 16 ) AS RESULT ;

SELECT set_masklen ( '192.168.1.0/24' :: cidr , 16 ) AS RESULT ;

SELECT text ( inet '192.168.1.5' ) AS RESULT ;

SELECT trunc ( macaddr '12:34:56:78:90:ab' ) AS RESULT ;

