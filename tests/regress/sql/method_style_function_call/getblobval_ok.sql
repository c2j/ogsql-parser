-- description: method-style getblobval() with numeric arg should parse without warnings
-- nowarn: R010
SELECT xmltype('<asd/>').getblobval(7);
