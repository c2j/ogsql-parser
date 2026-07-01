-- description: literal = literal (no column reference) should NOT trigger R005
-- nowarn: R005
SELECT * FROM t WHERE 1 = '1';
