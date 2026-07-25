-- 来源: 3978_file_3978.txt
-- SQL 数量: 22

ALTER DATABASE dbname SET paraname TO value ;

ALTER USER username SET paraname TO value ;

SET paraname TO value ;

SHOW hot_standby ;

SHOW authentication_timeout ;

SHOW explain_perf_mode ;

ALTER DATABASE postgres SET explain_perf_mode TO pretty ;

ALTER USER omm SET explain_perf_mode TO pretty ;

SET explain_perf_mode TO pretty ;

SHOW explain_perf_mode ;

SHOW max_connections ;

\ q 修改 GaussDB 数据库主节点的最大连接数。 gs_guc set -Z datanode -N all -I all -c "max_connections = 800" 重启 数据库 。 gs_om -t stop && gs_om -t start 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看最大连接数。 1 2 3 4

SHOW max_connections ;

SHOW authentication_timeout ;

\ q 修改数据库主节点的客户端认证最长时间。 gs_guc reload -Z datanode -N all -I all -c "authentication_timeout = 59s" 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看客户端认证的最长时间。 1 2 3 4

SHOW authentication_timeout ;

SHOW max_connections ;

\ q 修改 GaussDB 数据库节点的最大连接数。 gs_guc set -Z datanode -N all -I all -c "max_connections = 500" 重启 数据库 。 gs_om -t stop gs_om -t start 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看最大连接数。 1 2 3 4

SHOW max_connections ;

SHOW authentication_timeout ;

\ q 修改 GaussDB 数据库节点的客户端认证最长时间。 gs_guc reload -Z datanode -N all -I all -c "authentication_timeout = 30s" 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看客户端认证的最长时间。 1 2 3 4

SHOW authentication_timeout ;

