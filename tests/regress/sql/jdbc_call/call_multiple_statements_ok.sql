-- description: multiple CALL statements with mixed argument styles should parse
CALL proc_a();
CALL proc_b(1, 2);
CALL proc_c(p1 => 'a', p2 := 3);
