--keyword using
CREATE TABLE float_type_t2 ( FT_COL1 INTEGER , FT_COL2 FLOAT4 , FT_COL3 FLOAT8 , FT_COL4 FLOAT ( 3 ), FT_COL5 BINARY_DOUBLE , FT_COL6 DECIMAL ( 10 , 4 ), FT_COL7 INTEGER  using index pctfree 10 initrans 2 maxtrans 255 );


-- table comment
comment on table a.b is 'simplest table';

-- local
create index idx on t1 ( c2 ) local with(fillfactor=90);

create type bf.t1 is table of csbs ;

drop package body if exists bf.myjob;

MERGE INTO products p USING newproducts np ON ( p . product_id = np . product_id ) WHEN MATCHED THEN UPDATE SET p . product_name = np . product_name , p . category = np . category WHERE p . product_name != 'play gym' WHEN NOT MATCHED THEN INSERT VALUES ( np . product_id , np . product_name , np . category ) WHERE np . category = 'books' ;


ALTER TABLE PAR_FUND_PRE_LIQUIDATION
 ADD CONSTRAINT PK_PAR_FUND_PRE_LIQUIDATION PRIMARY KEY (FUND_CODE)
 USING index
 /*TABLESPACE BIGFUND_IND*/
 PCTFREE 10
 INITRANS 2
 MAXTRANS 255
 /*STORAGE
 (
   INITIAL 64K
   NEXT 1M
   MINEXTENTS 1
   MAXEXTENTS UNLIMITED
 )*/;


 ALTER TABLE PAR_FUND_PRE_LIQUIDATION
  ADD CONSTRAINT PK_PAR_FUND_PRE_LIQUIDATION PRIMARY KEY (FUND_CODE)
  USING index
  /*TABLESPACE BIGFUND_IND*/
  PCTFREE 10
  INITRANS 2
  MAXTRANS 255
  /*STORAGE
  (
    INITIAL 64K
    NEXT 1M
    MINEXTENTS 1
    MAXEXTENTS UNLIMITED
  )*/;



create index ind1 on t1 (part_id)
   INITRANS 2
   MAXTRANS 255;

  create table t1(
      id varchar(2) not null,
      code varchar(1)
  )
  pctfree 10
  initrans 1
  maxtrans 255;

  create index ind1 on t1 (part_id)
     INITRANS 2
     MAXTRANS 255;

create table t2(
        id varchar(2) not null,
        code varchar(1),
        CONSTRAINT PK_A PRIMARY Key (id)
        using index pctfree 10 initrans 2 maxtrans 255
    ) nocompress
    pctfree 10
    initrans 1
    maxtrans 255;
