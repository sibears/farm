use dotenv::dotenv;
use std::{collections::HashMap, env, sync::Mutex};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::protocols::{ProtocolHandler, PROTOCOLS};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct DatabaseConfig {
    pub database_url: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AuthConfig {
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ProtocolConfig {
    pub protocol: String,
    pub team_token: String,
    pub checksys_host: String,
    pub checksys_port: u32,
}

impl ProtocolConfig {
    pub fn get_protocol_handler(&self) -> &dyn ProtocolHandler {
        *PROTOCOLS.get(&self.protocol).unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CtfConfig {
    pub protocol: ProtocolConfig,
    pub flag_format: String,
    pub flag_lifetime: u32,
    pub submit_period: u32,
    pub submit_flag_limit: u32,
    pub teams: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Config {
    pub database: Mutex<DatabaseConfig>,
    pub auth: Mutex<AuthConfig>,
    pub ctf: Mutex<CtfConfig>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RawConfig {
    pub auth: AuthConfig,
    pub ctf: CtfConfig,
}

impl Config {
    pub fn new(auth_config: AuthConfig, ctf_config: CtfConfig) -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Config {
            database: Mutex::new(DatabaseConfig {
                database_url: database_url.into(),
            }),
            auth: Mutex::new(auth_config),
            ctf: Mutex::new(ctf_config),
        }
    }
}

impl DatabaseConfig {
    pub fn copy(&mut self, another: &DatabaseConfig) {
        self.database_url = another.database_url.to_owned();
    }
}

impl AuthConfig {
    pub fn new(password: String) -> Self {
        AuthConfig { password }
    }

    pub fn copy(&mut self, another: &AuthConfig) {
        self.password = another.password.to_owned();
    }
}

impl CtfConfig {
    pub fn new(
        proto_config: ProtocolConfig,
        flag_format: String,
        flag_lifetime: u32,
        submit_period: u32,
        submit_flag_limit: u32,
        teams: HashMap<String, String>,
    ) -> Self {
        CtfConfig {
            protocol: proto_config,
            flag_format,
            flag_lifetime,
            submit_period,
            submit_flag_limit,
            teams: teams
                .into_iter()
                .map(|item| (item.0.into(), item.1.into()))
                .collect(),
        }
    }

    pub fn copy(&mut self, another: &CtfConfig) {
        self.flag_format = another.flag_format.to_owned();
        self.flag_lifetime = another.flag_lifetime.to_owned();
        self.submit_flag_limit = another.submit_flag_limit;
        self.teams = another.teams.to_owned();
        self.submit_period = self.submit_period;
        self.protocol.copy(&another.protocol);
    }
}

impl ProtocolConfig {
    pub fn new(
        protocol: String,
        team_token: String,
        checksys_host: String,
        checksys_port: u32,
    ) -> Self {
        ProtocolConfig {
            protocol: protocol.into(),
            team_token: team_token.into(),
            checksys_host: checksys_host.into(),
            checksys_port,
        }
    }

    pub fn copy(&mut self, another: &ProtocolConfig) {
        self.protocol = another.protocol.to_owned();
        self.team_token = another.team_token.to_owned();
        self.checksys_host = another.checksys_host.to_owned();
        self.checksys_port = another.checksys_port;
    }
}
