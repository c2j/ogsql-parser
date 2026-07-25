-- 来源: 1053_file_1053.txt
-- SQL 数量: 6

SELECT point(1.1, 2.2);

SELECT lseg(point(1.1, 2.2), point(3.3, 4.4));

SELECT box(point(1.1, 2.2), point(3.3, 4.4));

SELECT path(polygon '((0,0),(1,1),(2,0))');

SELECT polygon(box '((0,0),(1,1))');

SELECT circle(point(0,0),1);

