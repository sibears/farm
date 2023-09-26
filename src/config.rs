use std::collections::HashMap;

use diesel::{SqliteConnection, pg};

use crate::settings::{Config, AuthConfig, CtfConfig, ProtocolConfig};


pub type DieselConnection = pg::PgConnection;
// TODO: Зарефакторить конфиги в удобную структуру для редактирования, мб добавить макросы для этого
pub fn get_config() -> Config {
    Config::new(
        AuthConfig::new(
           "sibears1cool"
        ),
        CtfConfig::new(
            ProtocolConfig::new(
                "forcad_http", 
                "3e74875c9a2d2eb0", 
                "forkad.docker.localhost", 
                80
            ),
            r"\w{31}=",
            5*60,
            5,
            100,
            HashMap::from([
                ("First", "first.docker.localhost"),
                ("Second", "second.docker.localhost")
            ])
        )
    )
}