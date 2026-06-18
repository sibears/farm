#!/bin/sh
set -e

# Creates the read-only role Grafana uses for the PostgreSQL datasource.
#
# Runs ONLY on a fresh Postgres data volume (docker-entrypoint-initdb.d
# semantics). On an existing volume it is skipped — run the same statements
# manually then (see docs/plans/.../Risks for the fallback GRANT).
#
# The `flags` table is created later by the backend migrator, so we cannot
# GRANT on it here. Instead we grant on any existing tables plus ALTER DEFAULT
# PRIVILEGES so future tables created by the owner are readable by grafana_ro.

: "${POSTGRES_USER:=postgres}"
: "${GRAFANA_RO_PASSWORD:=sibears_ro}"

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
  CREATE ROLE grafana_ro LOGIN PASSWORD '${GRAFANA_RO_PASSWORD}' NOSUPERUSER NOCREATEDB NOCREATEROLE;
  GRANT CONNECT ON DATABASE ${POSTGRES_DB} TO grafana_ro;
  GRANT USAGE ON SCHEMA public TO grafana_ro;
  GRANT SELECT ON ALL TABLES IN SCHEMA public TO grafana_ro;
  ALTER DEFAULT PRIVILEGES FOR ROLE ${POSTGRES_USER} IN SCHEMA public
    GRANT SELECT ON TABLES TO grafana_ro;
EOSQL
