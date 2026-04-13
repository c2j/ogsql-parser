-- 来源: 1258_CREATE NODE GROUP.txt
-- SQL 数量: 4

SELECT node_name, nodeis_preferred FROM pgxc_node WHERE node_type = 'D' ORDER BY 1;

-- 创建node group，用上一步中查询到的真实节点名称替换dn_6001_6002_6003。
CREATE NODE GROUP test_group WITH ( dn_6001_6002_6003 );

-- 查询node group。
SELECT group_name, group_members FROM pgxc_group;

-- 删除node group。
DROP NODE GROUP test_group;

