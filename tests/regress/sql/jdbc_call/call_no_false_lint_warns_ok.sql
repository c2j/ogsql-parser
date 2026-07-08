-- description: CALL with various args should not trigger any existing linter false positives
-- nowarn: R001,R002,R003,R004,R005,R006,R007,R008,R009,R010
CALL pkg_xxx.proc_yyy(__XML_PARAM_id__, __XML_PARAM_VARCHAR_name__);
