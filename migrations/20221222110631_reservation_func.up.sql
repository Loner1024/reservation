CREATE OR REPLACE FUNCTION rsvp.query(user_id text, rid text, during TSTZRANGE)
    RETURNS TABLE
            (
                LIKE rsvp.reservations
            )
AS
$$
BEGIN
    IF uid IS NULL AND rid IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE during && during;
    ELSIF uid IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE rid = rid AND during @> timespan;
    ELSIF rid IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE uid = uid AND during @> timespan;
    ELSE
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE uid = uid AND rid = rid AND during @> timespan;
    END IF;
END;

$$ LANGUAGE plpgsql;
