# Core Reservation

- Feature Name: core-reservation
- Start Date: 2022-12-21

## Summary

A core reservation service that solves the problem of reserving a resource for a period of time. We Leverage postgres
EXCLUDE constraints to ensure that only one reservat ion can be made for a given resource at a given time.
One paragraph explanation of the feature.

## Motivation

We need a common solution for various reservation requirements: 1) calendar booking; 2) hotel/room booking;3) meeting
room booking; 4 parking lot booking; 5) etc. Repeatedly build ing features for these requirements 1S a waste
of time and resources. We should have a common solution that can be used by all teams.

## Guide-level explanation

## Service interface

We would use gRPC as a service interface. Be low is the proto definition:

```protobuf
syntax = 'proto3';

enum ReservationStatus {
  UNKNOWN = 0;
  PENDING = 1;
  CONFIRMED = 2;
  BLOCKED = 3;
}

message Reservation {
  string id = 1;
  string user_id = 2;
  ReservationStatus status = 3;

  string resource_id = 4;
  google.protobuf.Timestamp start = 5;
  google.protobuf.Timestamp end = 6;

  string note = 7;
}

message ReserveRequest {
  Reservation reservation = 1;
}

message ReserveResponse {
  Reservation reservation = 1;
}

message UpdateRequest {
  string note = 1;
}

message UpdateResponse {
  Reservation reservation = 1;
}

message ConfirmRequest {
  string id = 1;
}

message ConfirmResponse {
  Reservation reservation = 1;
}

message CancelRequest {
  string id = 1;
}

message CancelResponse {
  Reservation reservation = 1;
}

message GetRequest {
  string id = 1;
}

message GetResponse {
  Reservation reservation = 1;
}

message QueryRequest {
  string resource_id = 1;
  string user_id = 2;
  ReservationStatus status = 3;
  google.protobuf.Timestamp start = 4;
  google. protobuf.Timestamp end = 5;
}

message ListenRequest {}

enum ReservationUpdateType {
  UNKNOWN = 0;
  CREATE = 1;
  UPDATE = 2;
  DELETE = 3;
}

message ListenResponse {
  ReservationUpdateType op = 1;
  Reservation reservation = 2;
}

service ReservationService {
  rpc Reserve(ReserveRequest) returns (ReserveResponse);
  rpc Confirm(ConfirmRequest) returns (ConfirmResponse);
  rpc Update(UpdateRequest) returns (UpdateResponse);
  rpc Cancel(CancelRequest) returns (CancelResponse);
  rpc Get(GetRequest) returns (GetResponse);
  rpc Query(QueryRequest) returns (stream Reservation);
  rpc Listen(ListenRequest) returns (stream Reservation);
}
```

## Database schema

```postgresql
CREATE SCHEMA rsvp;
CREATE TYPE rsvp.reservation_status AS ENUM
    ('unknown','pending', 'confirmed', 'blocked');
CREATE TYPE rsvp.reservation_update_type AS ENUM
    ('unknown','create', 'update', 'delete');
CREATE TABLE rsvp.reservations
(
    id          uuid                    NOT NULL DEFAULT uuid_generate_v4(),
    user_id     varchar(64)             NOT NULL,
    status      rsvp.reservation_status NOT NULL DEFAULT 'pending',
    resource_id varchar(64)             NOT NULL,
    timespan    tstzrange               NOT NULL,
    note        text,
    CONSTRAINT reservations_pkey PRIMARY KEY (id),
    CONSTRAINT reservations_conflict EXCLUDE USING gist (resource_id WITH =, timespan WITH &&)
);
CREATE INDEX reservation_resource_id_idx ON rsvp.reservations (resource_id);
CREATE INDEX reservation_user_id_idx ON rsvp.reservations (user_id);
-- query
CREATE OR REPLACE FUNCTION rsvp.query(user_id text, rid text, during: TSTZRANGE) RETURNS TABLE rsvp.reservations AS
$$

$$ LANGUAGE plpgsql;

CREATE TABLE rsvp.reservation_changes
(
    id             SERIAL                  NOT NULL,
    reservation_id uuid                    NOT NULL,
    op             reservation_update_type NOT NULL
);

-- trigger rsvp.reservations
CREATE OR REPLACE FUNCTION rsvp.reservation_trigger() RETURNS TRIGGER AS
$$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO rsvp.reservation_changes (reservation_id, op) VALUES (NEW.id, 'create');
    ELSIF TG_OP = 'UPDATE' THEN
        IF OLD.status <> NEW.status THEN
            INSERT INTO rsvp.reservation_changes (reservation_id, op) VALUES (NEW.id, 'update');
        END IF;
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO rsvp.reservation_changes (reservation_id, op) VALUES (NEW.id, 'delete');
    END IF;
    RETURN NULL;
END
$$
    LANGUAGE plpgsql;
CREATE TRIGGER reservation_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON rsvp.reservations
    FOR EACH ROW
EXECUTE PROCEDURE rsvp.reservation_trigger();
```

## Reference-level explanation

[reference-level-explanation]: #reference-level-explanation

This is the technical portion of the RFC. Explain the design in sufficient detail that:

- Its interaction with other features is clear.
- It is reasonably clear how the feature would be implemented.
- Corner cases are dissected by example.

The section should return to the examples given in the previous section, and explain more fully how the detailed
proposal makes those examples work.

## Drawbacks

[drawbacks]: #drawbacks

Why should we *not* do this?

## Rationale and alternatives

[rationale-and-alternatives]: #rationale-and-alternatives

- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?
- If this is a language proposal, could this be done in a library or macro instead? Does the proposed change make Rust
  code easier or harder to read, understand, and maintain?

## Prior art

[prior-art]: #prior-art

Discuss prior art, both the good and the bad, in relation to this proposal.
A few examples of what this can include are:

- For language, library, cargo, tools, and compiler proposals: Does this feature exist in other programming languages
  and what experience have their community had?
- For community proposals: Is this done by some other community and what were their experiences with it?
- For other teams: What lessons can we learn from what other communities have done here?
- Papers: Are there any published papers or great posts that discuss this? If you have some relevant papers to refer to,
  this can serve as a more detailed theoretical background.

This section is intended to encourage you as an author to think about the lessons from other languages, provide readers
of your RFC with a fuller picture.
If there is no prior art, that is fine - your ideas are interesting to us whether they are brand new or if it is an
adaptation from other languages.

Note that while precedent set by other languages is some motivation, it does not on its own motivate an RFC.
Please also take into consideration that rust sometimes intentionally diverges from common language features.

## Unresolved questions

[unresolved-questions]: #unresolved-questions

- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of
  the solution that comes out of this RFC?

## Future possibilities

TBD
