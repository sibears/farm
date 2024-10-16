-- Your SQL goes here
set enable_parallel_hash=on;

CREATE TYPE flag_status AS ENUM ('QUEUED', 'SKIPPED', 'ACCEPTED', 'REJECTED');

CREATE TABLE IF NOT EXISTS flags (
    id SERIAL PRIMARY KEY NOT NULL,
    flag TEXT NOT NULL,
    sploit TEXT,
    team TEXT,
    time TIMESTAMP NOT NULL,
    status flag_status NOT NULL,
    checksystem_response TEXT
);