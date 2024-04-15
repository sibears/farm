use std::fs;

use crate::repos::flag::PostgresFlagRepo;
use diesel::pg;

use crate::settings::Config;

pub type DieselConnection = pg::PgConnection;
pub type DbFlagRepo = PostgresFlagRepo;
pub fn get_config() -> Config {
    let config = fs::read_to_string("./config.json");
    let config: Config = match config {
        Ok(conf) => serde_json::from_str(&conf).unwrap(),
        Err(_) => serde_json::from_str(&std::env::var("FARM_CONFIG").unwrap()).unwrap(),
    };
    config
}
