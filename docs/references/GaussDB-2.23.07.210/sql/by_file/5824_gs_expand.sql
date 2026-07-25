-- 来源: 5824_gs_expand.txt
-- SQL 数量: 1

-------------------------------------------------- 每个表的重分布执行时间（redis_progress_detail）： 由于该表由重分布线程创建记录，当重分布异常退出或者session连接异常时可能导致记录的时间不准确，只能作为参考，需要获取准确时间需要通过日志进行读取； 当用户表在pgxc_redistb中的redistributed字段为'y'时，用户再修改表名，该表中的table_name不会再进行更新。
select * from redis_progress_detail;

