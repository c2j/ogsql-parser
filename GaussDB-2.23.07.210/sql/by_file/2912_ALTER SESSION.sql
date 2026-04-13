-- 来源: 2912_ALTER SESSION.txt
-- SQL 数量: 14

CREATE SCHEMA ds;

--设置模式搜索路径。
SET SEARCH_PATH TO ds, public;

--设置日期时间风格为传统的POSTGRES风格（日在月前）。
SET DATESTYLE TO postgres, dmy;

--设置当前会话的字符编码为UTF8。
ALTER SESSION SET NAMES 'UTF8';

--设置时区为加州伯克利。
SET TIME ZONE 'PST8PDT';

--设置时区为意大利。
SET TIME ZONE 'Europe/Rome';

--设置当前模式。
ALTER SESSION SET CURRENT_SCHEMA TO tpcds;

--设置XML OPTION为DOCUMENT。
ALTER SESSION SET XML OPTION DOCUMENT;

--创建角色joe，并设置会话的角色为joe。
CREATE ROLE joe WITH PASSWORD ' ******** ';

ALTER SESSION SET SESSION AUTHORIZATION joe PASSWORD ' ******** ';

--删除ds模式。
DROP SCHEMA ds;

--删除joe。
DROP ROLE joe;

--开启事务,设置事务级别
START TRANSACTION;

ALTER SESSION SET TRANSACTION READ ONLY;

