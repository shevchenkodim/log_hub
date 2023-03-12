CREATE OR REPLACE FUNCTION generate_unique_string(param varchar, table_name varchar, length int) RETURNS varchar as
$$
DECLARE
    resp_string varchar := random_string(length);
BEGIN
    DECLARE
        c int := 0;
    BEGIN
        EXECUTE format('SELECT COUNT(*) FROM %s WHERE %s = $1', table_name, param) INTO c USING resp_string;
        WHILE (c) > 0 LOOP
                resp_string := random_string(length);
            END LOOP;
    END;
    RETURN resp_string;
END
$$ LANGUAGE plpgsql;
