-- description: CALL inside CREATE PROCEDURE body should parse
CREATE OR REPLACE PROCEDURE prc_wrapper(p_id INTEGER)
LANGUAGE plpgsql
AS $$
BEGIN
    CALL pkg_inventory.check_stock(p_id, 100);
    CALL pkg_order.create_order(p_id, 0, 1);
END;
$$;
