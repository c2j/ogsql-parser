-- description: without schema, R005 should not warn (avoids false positives on legitimate col=literal, issue #240)
-- nowarn: R005
SELECT * FROM t WHERE age = '30';
