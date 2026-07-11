-- description: CALL with JDBC ? placeholders should parse without warnings
-- nowarn: R001
CALL fnc_com_getday(?, ?, ?, ?);
