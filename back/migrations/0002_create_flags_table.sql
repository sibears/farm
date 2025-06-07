set enable_parallel_hash=on;

CREATE TABLE IF NOT EXISTS flags (
    id SERIAL PRIMARY KEY NOT NULL,
    flag TEXT NOT NULL,
    sploit TEXT,
    team TEXT,
    created_time TIMESTAMP NOT NULL,
    start_waiting_time TIMESTAMP,
    status flag_status NOT NULL DEFAULT 'queued',
    checksystem_response TEXT
);