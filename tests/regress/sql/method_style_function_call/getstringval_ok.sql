-- description: method-style getstringval() should parse without warnings
-- nowarn: R010
SELECT xmltype('<a>123<b>456</b></a>').getstringval();
