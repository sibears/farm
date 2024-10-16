use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct AuthConfig {
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct ProtocolConfig {
    pub protocol: String,
    pub team_token: String,
    pub checksys_host: String,
    pub checksys_port: u32,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct CtfConfig {
    pub protocol: ProtocolConfig,
    pub flag_format: String,
    pub flag_lifetime: u32,
    pub submit_period: u32,
    pub submit_flag_limit: u32,
    pub teams: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub ctf: CtfConfig,
}