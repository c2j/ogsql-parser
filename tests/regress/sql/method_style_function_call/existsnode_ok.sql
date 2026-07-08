-- description: method-style existsnode() with extra args should parse without warnings
-- nowarn: R010
SELECT xmltype('<a>123<b>456</b></a>').existsnode('/a/b');
