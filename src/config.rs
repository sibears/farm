use std::fs;

use crate::repos::flag::PostgresFlagRepo;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::pg;
use diesel::r2d2::PooledConnection;

use crate::settings::Config;

pub type DbConnection = pg::PgConnection;
pub type DbFlagRepo = PostgresFlagRepo;
pub type DbPool = Pool<ConnectionManager<DbConnection>>;
pub type DbPooled = PooledConnection<ConnectionManager<DbConnection>>;


pub fn get_config(path: &str) -> Config {
    let config = fs::read_to_string(path);
    let config: Config = match config {
        Ok(conf) => serde_json::from_str(&conf).unwrap(),
        Err(_) => serde_json::from_str(&std::env::var("FARM_CONFIG").unwrap()).unwrap(),
    };
    config
}