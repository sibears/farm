extern crate diesel;

use diesel::r2d2::{Pool, PooledConnection};
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::config::DieselConnection;

pub struct DbConn {
    pub master: PooledConnection<ConnectionManager<DieselConnection>>,
}

pub struct DbCollection {
    pub db_conn_pool: Pool<ConnectionManager<DieselConnection>>,
}

pub fn init_db(database_url: String) -> DbCollection {
    let manager = ConnectionManager::<DieselConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let conn = &mut pool.get().unwrap();

    run_migrations(conn);
    DbCollection { db_conn_pool: pool }
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations(conn: &mut DieselConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}