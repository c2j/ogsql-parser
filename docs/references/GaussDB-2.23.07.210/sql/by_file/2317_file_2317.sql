-- 来源: 2317_file_2317.txt
-- SQL 数量: 18

SET paraname TO value ;

SHOW hot_standby ;

SHOW authentication_timeout ;

SHOW explain_perf_mode ;

SET explain_perf_mode TO pretty ;

SHOW explain_perf_mode ;

SHOW max_connections ;

\ q 修改 集群 所有CN 的最大连接数。 gs_guc set -Z coordinator -N all -I all -c "max_connections = 800" 重启 集群 。 gs_om -t stop && gs_om -t start 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看最大连接数。 1 2 3 4

SHOW max_connections ;

SHOW authentication_timeout ;

\ q 修改 集群 所有CN 的客户端认证最长时间。 gs_guc reload -Z coordinator -N all -I all -c "authentication_timeout = 59s" 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看客户端认证的最长时间。 1 2 3 4

SHOW authentication_timeout ;

SHOW max_connections ;

\ q 修改 集群 所有 CN和DN 的最大连接数。 gs_guc set -Z coordinator -Z datanode -N all -I all -c "max_connections = 500" 重启 集群 。 gs_om -t stop gs_om -t start 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看最大连接数。 1 2 3 4

SHOW max_connections ;

SHOW authentication_timeout ;

\ q 修改 集群 所有 CN和DN 的客户端认证最长时间。 gs_guc reload -Z coordinator -Z datanode -N all -I all -c "authentication_timeout = 30s" 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看客户端认证的最长时间。 1 2 3 4

SHOW authentication_timeout ;

