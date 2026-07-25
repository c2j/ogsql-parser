-- 来源: 1257_CREATE NODE.txt
-- SQL 数量: 7

CREATE NODE datanode1 WITH( TYPE = datanode, PREFERRED = false );

CREATE NODE datanode2 WITH( TYPE = datanode, PREFERRED = false );

-- 查询集群DN初始状态。
SELECT node_name, nodeis_preferred FROM pgxc_node WHERE node_type = 'D' ORDER BY 1;

-- 将datanode1设为preferred DN。
ALTER NODE datanode1 WITH(preferred = true);

-- 查询集群DN变更后状态。
SELECT node_name, nodeis_preferred FROM pgxc_node WHERE node_type = 'D' ORDER BY 1;

-- 删除集群节点。
DROP NODE datanode1;

DROP NODE datanode2;

