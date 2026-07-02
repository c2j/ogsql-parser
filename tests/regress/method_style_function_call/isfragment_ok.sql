-- description: method-style isfragment() should parse without warnings
-- nowarn: R010
SELECT xmltype('<a>123</a>').isfragment();
