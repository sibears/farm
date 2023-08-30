-- Your SQL goes here
CREATE TABLE IF NOT EXISTS flags (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    flag TEXT NOT NULL,
    sploit TEXT,
    team TEXT,
    time DATETIME NOT NULL,
    status TEXT NOT NULL,
    checksystem_response TEXT
);