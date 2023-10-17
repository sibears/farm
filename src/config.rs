use std::{collections::HashMap, fs};

use diesel::pg;

use crate::settings::{Config, AuthConfig, CtfConfig, ProtocolConfig};


pub type DieselConnection = pg::PgConnection;
pub fn get_config() -> Config {
    let config = fs::read_to_string("./config.json");
    let config: Config = match config {
        Ok(conf) => serde_json::from_str(&conf).unwrap(),
        Err(_) => {
            serde_json::from_str(&std::env::var("FARM_CONFIG").unwrap()).unwrap()
        },
    };
    config
}