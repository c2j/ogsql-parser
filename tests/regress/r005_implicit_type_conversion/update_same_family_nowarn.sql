-- description: UPDATE WHERE varchar col vs string literal is same-family, should NOT trigger R005
-- schema: t.status=varchar(20)
-- nowarn: R005
UPDATE t SET status = 'active' WHERE status = 'inactive';
