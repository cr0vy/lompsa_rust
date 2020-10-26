-- Your SQL goes here
CREATE TABLE events (
    event_id INTEGER NOT NULL PRIMARY KEY,
    event_date VARCHAR NOT NULL,
    event_desc VARCHAR NOT NULL,
    event_from VARCHAR NOT NULL,
    event_to VARCHAR NOT NULL,
    event_acc VARCHAR NOT NULL,
    event_sum INTEGER NOT NULL
);
