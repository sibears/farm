extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use r2d2_diesel::ConnectionManager;
use r2d2::{PooledConnection, Pool};

use crate::config::DieselConnection;


pub struct DbConn {
    pub master: PooledConnection<ConnectionManager<DieselConnection>>
}

pub struct DbCollection {
    pub db_conn_pool: Pool<ConnectionManager<DieselConnection>>,
}

pub fn init_db(database_url: String) -> DbCollection {
    let manager = ConnectionManager::<DieselConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    DbCollection { db_conn_pool: pool }
}
