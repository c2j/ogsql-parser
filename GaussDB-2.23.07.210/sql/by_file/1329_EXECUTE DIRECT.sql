-- 来源: 1329_EXECUTE DIRECT.txt
-- SQL 数量: 10

SELECT * FROM pgxc_node ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . customer_address ( ca_address_sk INTEGER NOT NULL , ca_address_id CHARACTER ( 16 ) NOT NULL );

EXECUTE DIRECT ON ( dn_6001_6002 ) 'select count(*) from tpcds.customer_address' ;

SELECT COUNT ( * ) FROM tpcds . customer_address ;

SELECT oid FROM pgxc_node where node_name = 'dn_6001_6002_6003' ;

SET enable_direct_standby_datanodes = on ;

EXECUTE DIRECT ON ( 16385 , 16386 , 16384 ) 'SELECT * FROM gs_get_listen_address_ext_info();

DROP TABLE tpcds . customer_address ;

DROP SCHEMA tpcds ;

