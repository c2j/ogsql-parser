-- 来源: 5892_file_5892.txt
-- SQL 数量: 17

\ l List of databases Name | Owner | Encoding | Collate | Ctype | Access privileges ----------------+----------+-----------+---------+-------+----------------------- human_resource | omm | SQL_ASCII | C | C | postgres | omm | SQL_ASCII | C | C | template0 | omm | SQL_ASCII | C | C | = c / omm + | | | | | omm = CTc / omm template1 | omm | SQL_ASCII | C | C | = c / omm + | | | | | omm = CTc / omm human_staff | omm | SQL_ASCII | C | C | ( 5 rows ) 更多gsql元命令请参见 元命令参考 。 示例 以把一个查询分成多行输入为例。注意提示符的变化： 1 2 3 4

CREATE TABLE HR . areaS ( gaussdb ( # area_ID NUMBER , gaussdb ( # area_NAME VARCHAR2 ( 25 ) gaussdb -# ) tablespace EXAMPLE ;

\ d HR . areaS Table "hr.areas" Column | Type | Modifiers -----------+-----------------------+----------- area_id | numeric | not null area_name | character varying ( 25 ) | 向HR.areaS表插入四行数据： 1 2 3 4 5 6 7

INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 1 , 'Europe' );

INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 2 , 'Americas' );

INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 3 , 'Asia' );

INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 4 , 'Middle East and Africa' );

\ set PROMPT1 '%n@%m %~%R%#' omm @ [ local ]

查看表： 1 2 3 4 5 6 7 8 omm @ [ local ]

SELECT * FROM HR . areaS ;

\ pset border 2 Border style is 2 . omm @ [ local ]

SELECT * FROM HR . areaS ;

\ pset border 0 Border style is 0 . omm @ [ local ]

SELECT * FROM HR . areaS ;

\ a \ t \ x Output format is unaligned . Showing only tuples . Expanded display is on . omm @ [ local ]

SELECT * FROM HR . areaS ;

父主题： gsql

