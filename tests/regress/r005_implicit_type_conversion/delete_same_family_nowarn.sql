-- description: DELETE WHERE varchar col vs string literal is same-family, should NOT trigger R005
-- schema: t.status=character varying(20)
-- nowarn: R005
DELETE FROM t WHERE status = 'inactive';
