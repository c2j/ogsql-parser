-- 类别: CURSOR
-- SQL 数量: 24

-- 来源: 1335_FETCH
FETCH FORWARD 3 FROM cursor1 ;

-- 来源: 1335_FETCH
CLOSE cursor1 ;

-- 来源: 1335_FETCH
FETCH FORWARD 2 FROM cursor2 ;

-- 来源: 1335_FETCH
CLOSE cursor2 ;

-- 来源: 1335_FETCH
FETCH FORWARD 2 FROM cursor1 ;

-- 来源: 1335_FETCH
FETCH FORWARD 1 FROM cursor1 ;

-- 来源: 1335_FETCH
CLOSE cursor1 ;

-- 来源: 1350_MOVE
FETCH 4 FROM cursor1 ;

-- 来源: 1350_MOVE
CLOSE cursor1 ;

-- 来源: 1367_ROLLBACK TO SAVEPOINT
FETCH 1 FROM foo ;

-- 来源: 1367_ROLLBACK TO SAVEPOINT
FETCH 1 FROM foo ;

-- 来源: 1489_file_1489
fetch "<unnamed portal 1>";

--抓取头3行到游标cursor1里。
-- 来源: 3046_FETCH
FETCH FORWARD 3 FROM cursor1;

--关闭游标并提交事务。
-- 来源: 3046_FETCH
CLOSE cursor1;

--抓取头2行到游标cursor2里。
-- 来源: 3046_FETCH
FETCH FORWARD 2 FROM cursor2;

--关闭游标并提交事务。
-- 来源: 3046_FETCH
CLOSE cursor2;

--抓取头2行到游标cursor1里。
-- 来源: 3046_FETCH
FETCH FORWARD 2 FROM cursor1;

--抓取下一行到游标cursor1里。
-- 来源: 3046_FETCH
FETCH FORWARD 1 FROM cursor1;

--关闭游标。
-- 来源: 3046_FETCH
CLOSE cursor1;

--抓取游标cursor1的前4行。
-- 来源: 3061_MOVE
FETCH 4 FROM cursor1;

--关闭游标。
-- 来源: 3061_MOVE
CLOSE cursor1;

cursor c1 is
select * from t1;

-- 来源: 3078_ROLLBACK TO SAVEPOINT
FETCH 1 FROM foo;

-- 来源: 3078_ROLLBACK TO SAVEPOINT
FETCH 1 FROM foo;

-- 来源: 3202_file_3202
fetch "<unnamed portal 1>";
