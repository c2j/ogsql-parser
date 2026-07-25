-- 来源: 2936_COMMENT.txt
-- SQL 数量: 8

CREATE TABLE emp( empno varchar(7), ename varchar(50), job varchar(50), mgr varchar(7), deptno int );

--表添加注释
COMMENT ON TABLE emp IS '部门表';

--字段添加注释
COMMENT ON COLUMN emp.empno IS '员工编号';

COMMENT ON COLUMN emp.ename IS '员工姓名';

COMMENT ON COLUMN emp.job IS '职务';

COMMENT ON COLUMN emp.mgr IS '上司编号';

COMMENT ON COLUMN emp.deptno IS '部门编号';

--删除
DROP TABLE emp;

