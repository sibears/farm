extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
use dotenv::dotenv;
use rocket::Data;

use std::env;
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::{PooledConnection, Pool};

use crate::{settings::{Config, DatabaseConfig}, config::DieselConnection};


pub struct DbConn {
    pub master: PooledConnection<ConnectionManager<DieselConnection>>
}

pub struct DbCollection {
    pub db_conn_pool: Pool<ConnectionManager<DieselConnection>>,
}

pub fn init_sqlite_db(config: &DatabaseConfig) -> DbCollection {
    let manager = ConnectionManager::<DieselConnection>::new(config.database_url.as_ref());
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    DbCollection { db_conn_pool: pool }
}
