-- 来源: 1268_CREATE SERVER.txt
-- SQL 数量: 4

CREATE SERVER my_server FOREIGN DATA WRAPPER file_fdw ;

DROP SERVER my_server ;

CREATE SERVER server_remote FOREIGN DATA WRAPPER GC_FDW OPTIONS ( address '10.146.187.231:8000,10.180.157.130:8000' , dbname 'test' , username 'test' , password '********' );

DROP SERVER server_remote ;

