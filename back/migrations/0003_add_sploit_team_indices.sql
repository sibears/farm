-- Indices for Grafana PostgreSQL panels (sploit x team matrix, per-team/per-sploit
-- aggregates). GROUP BY / time-range filters would otherwise full-scan the flags
-- table as the flag count grows during a CTF.
CREATE INDEX IF NOT EXISTS idx_flags_sploit ON flags (sploit);
CREATE INDEX IF NOT EXISTS idx_flags_team ON flags (team);
