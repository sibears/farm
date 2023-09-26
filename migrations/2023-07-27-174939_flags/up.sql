-- Your SQL goes here
CREATE TABLE IF NOT EXISTS flags (
    id SERIAL PRIMARY KEY NOT NULL,
    flag TEXT NOT NULL,
    sploit TEXT,
    team TEXT,
    time TIMESTAMP NOT NULL,
    status TEXT NOT NULL,
    checksystem_response TEXT
);