use std::{collections::HashMap, fs};

use diesel::{SqliteConnection, pg};

use crate::settings::{Config, AuthConfig, CtfConfig, ProtocolConfig};


pub type DieselConnection = pg::PgConnection;
pub fn get_config() -> Config {
    let config = fs::read_to_string("./config.json").unwrap();
    let config = serde_json::from_str(&config).unwrap();
    config
}