CREATE TYPE encounter_status AS ENUM (
    'SCHEDULED',
    'DONE',
    'CANCELLED'
);

CREATE TABLE encounter (
    id TEXT NOT NULL PRIMARY KEY,
    type TEXT NOT NULL,
    name TEXT NOT NULL,
    patient_id TEXT NOT NULL,
    program TEXT NOT NULL,
    start_datetime TIMESTAMP NOT NULL,
    end_datetime TIMESTAMP,
    status encounter_status
)