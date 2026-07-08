-- description: Qualified star (t.*) with DISTINCT should skip R011
-- nowarn: R011
SELECT DISTINCT t.*, t.a FROM t ORDER BY b;
