-- Your SQL goes here
set enable_parallel_hash=on;

CREATE TYPE flag_status AS ENUM ('queued', 'skipped', 'accepted', 'rejected');

CREATE TABLE IF NOT EXISTS flags (
    id SERIAL PRIMARY KEY NOT NULL,
    flag TEXT NOT NULL,
    sploit TEXT,
    team TEXT,
    created_time TIMESTAMP NOT NULL,
    start_waiting_time TIMESTAMP,
    status TEXT NOT NULL,
    checksystem_response TEXT
);
