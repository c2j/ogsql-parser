-- 来源: 5891_gsql.txt
-- SQL 数量: 8

\ set foo bar 要引用变量的值，在变量前面加冒号。例如查看变量的值： 1

\ echo : foo bar 这种变量的引用方法适用于规则的SQL语句和除\copy、\ef、\help、\sf、\!以外的元命令。 gsql预定义了一些特殊变量，同时也规划了变量的取值。为了保证和后续版本最大限度地兼容，请避免以其他目的使用这些变量。所有特殊变量见 表2 。 所有特殊变量都由大写字母、数字和下划线组成。 要查看特殊变量的默认值，请使用元命令 \echo : varname （例如\echo :DBNAME）。 表2 特殊变量设置 变量 设置方法 变量说明 DBNAME \set DBNAME dbname 当前连接的数据库的名称。每次连接数据库时都会被重新设置。 ECHO \set ECHO all | queries 如果设置为all，只显示查询信息。等效于使用gsql连接数据库时指定-a参数。 如果设置为queries，显示命令行和查询信息。等效于使用gsql连接数据库时指定-e参数。 ECHO_HIDDEN \set ECHO_HIDDEN on | off | noexec 当使用元命令查询数据库信息（例如\dg）时，此变量的取值决定了查询的行为： 设置为on，先显示元命令实际调用的查询语句，然后显示查询结果。等效于使用gsql连接数据库时指定-E参数。 设置为off，则只显示查询结果。 设置为noexec，则只显示查询信息，不执行查询操作。 ENCODING \set ENCODING encoding 当前客户端的字符集编码。 FETCH_COUNT \set FETCH_COUNT variable 如果该变量的值为大于0的整数，假设为n，则执行SELECT语句时每次从结果集中取n行到缓存并显示到屏幕。 如果不设置此变量，或设置的值小于等于0，则执行SELECT语句时一次性把结果都取到缓存。 说明： 设置合理的变量值，将减少内存使用量。一般来说，设为100到1000之间的值比较合理。 HISTCONTROL \set HISTCONTROL ignorespace | ignoredups | ignoreboth | none ignorespace：以空格开始的行将不会写入历史列表。 ignoredups：与以前历史记录里匹配的行不会写入历史记录。 ignoreboth、none或者其他值：所有以交互模式读入的行都被保存到历史列表。 说明： none表示不设置HISTCONTROL。 HISTFILE \set HISTFILE filename 此文件用于存储历史名列表。缺省值是~/.bash_history。 HISTSIZE \set HISTSIZE size 保存在历史命令里命令的个数。缺省值是500。 HOST \set HOST hostname 已连接的数据库主机名称。 IGNOREEOF \set IGNOREEOF variable 若设置此变量为数值，假设为10，则在gsql中输入的前9次EOF字符（通常是Ctrl+C）都会被忽略，在第10次按Ctrl+C才能退出gsql程序。 若设置此变量为非数值，则缺省为10。 若删除此变量，则向交互的gsql会话发送一个EOF终止应用。 LASTOID \set LASTOID oid 最后影响的oid值，即为从一条INSERT或lo_import命令返回的值。此变量只保证在下一条SQL语句的结果显示之前有效。 ON_ERROR_ROLLBACK \set ON_ERROR_ROLLBACK on | interactive | off 如果是on，当一个事务块里的语句产生错误的时候，这个错误将被忽略而事务继续。 如果是interactive，这样的错误只是在交互的会话里忽略。 如果是off（缺省），事务块里一个语句生成的错误将会回滚整个事务。on_error_rollback-on模式是通过在一个事务块的每个命令前隐含地发出一个SAVEPOINT的方式来工作的，在发生错误的时候回滚到该事务块。 ON_ERROR_STOP \set ON_ERROR_STOP on | off on：命令执行错误时会立即停止，在交互模式下，gsql会立即返回已执行命令的结果。 off（缺省）：命令执行错误时将会跳过错误继续执行。 PORT \set PORT port 正连接数据库的端口号。 USER \set USER username 当前用于连接的数据库用户。 VERBOSITY \set VERBOSITY terse | default | verbose 这个选项可以设置为值terse、default、verbose之一以控制错误报告的冗余行。 terse：仅返回严重且主要的错误文本以及文本位置（一般适合于单行错误信息）。 default：返回严重且主要的错误文本及其位置，还包括详细的错误细节、错误提示（可能会跨越多行）。 verbose：返回所有的错误信息。 SQL代换 像元命令的参数一样，gsql变量的一个关键特性是可以把gsql变量替换成正规的SQL语句。此外，gsql还提供为变量更换新的别名或其他标识符等功能。使用SQL代换方式替换一个变量的值可在变量前加冒号。例如： 1 2 3 4 5 6 7 8

\ set foo 'HR.areaS'

select * from : foo ;

\ set PROMPT2 TEST

select * from HR . areaS TEST ;

\ set PROMPT3 '>>>>'

copy HR . areaS from STDIN ;

