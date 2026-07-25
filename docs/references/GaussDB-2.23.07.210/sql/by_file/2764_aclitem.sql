-- 来源: 2764_aclitem.txt
-- SQL 数量: 4

CREATE TABLE table_acl (id int,priv aclitem,privs aclitem[]);

INSERT INTO table_acl VALUES (1,'user1=arw/omm','{omm=d/user2,omm=w/omm}');

INSERT INTO table_acl VALUES (2,'user1=aw/omm','{omm=d/user2}');

SELECT * FROM table_acl;

