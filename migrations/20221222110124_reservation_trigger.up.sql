CREATE TABLE rsvp.reservation_changes
(
    id             SERIAL                       NOT NULL,
    reservation_id uuid                         NOT NULL,
    op             rsvp.reservation_update_type NOT NULL
);

-- trigger rsvp.reservations
CREATE
    OR REPLACE FUNCTION rsvp.reservation_trigger() RETURNS TRIGGER AS
$$
BEGIN
    IF
        TG_OP = 'INSERT' THEN
        INSERT INTO rsvp.reservation_changes (reservation_id, op) VALUES (NEW.id, 'create');
    ELSIF
        TG_OP = 'UPDATE' THEN
        IF OLD.status <> NEW.status THEN
            INSERT INTO rsvp.reservation_changes (reservation_id, op) VALUES (NEW.id, 'update');
        END IF;
    ELSIF
        TG_OP = 'DELETE' THEN
        INSERT INTO rsvp.reservation_changes (reservation_id, op) VALUES (NEW.id, 'delete');
    END IF;
    NOTIFY reservation_update;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER reservation_trigger
    AFTER INSERT OR
        UPDATE OR
        DELETE
    ON rsvp.reservations
    FOR EACH ROW
EXECUTE PROCEDURE rsvp.reservation_trigger();
