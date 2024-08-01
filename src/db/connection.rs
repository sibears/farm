extern crate diesel;

use std::borrow::BorrowMut;

use diesel::r2d2::{Pool, PooledConnection};
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::config::DbPool;


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn init_db(database_url: String) -> DbPool {
    let manager = ConnectionManager::<diesel::pg::PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}


