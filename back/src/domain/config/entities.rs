use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct DatabaseConfig {
    pub database_url: String,
}

impl PartialEq for DatabaseConfig {
    fn eq(&self, other: &Self) -> bool {
        self.database_url == other.database_url
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct AuthConfig {
    pub password: String,
}

impl PartialEq for AuthConfig {
    fn eq(&self, other: &Self) -> bool {
        self.password == other.password
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct ProtocolConfig {
    pub protocol: String,
    pub team_token: String,
    pub checksys_host: String,
    pub checksys_port: u32,
}

impl PartialEq for ProtocolConfig {
    fn eq(&self, other: &Self) -> bool {
        self.protocol == other.protocol
            && self.team_token == other.team_token
            && self.checksys_host == other.checksys_host
            && self.checksys_port == other.checksys_port
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct CtfConfig {
    pub protocol: ProtocolConfig,
    pub flag_format: String,
    pub flag_lifetime: u32,
    pub submit_period: u32,
    pub waiting_period: u32,
    pub submit_flag_limit: u32,
    pub teams: HashMap<String, String>,
}

impl PartialEq for CtfConfig {
    fn eq(&self, other: &Self) -> bool {
        self.protocol == other.protocol
            && self.flag_format == other.flag_format
            && self.flag_lifetime == other.flag_lifetime
            && self.submit_period == other.submit_period
            && self.waiting_period == other.waiting_period
            && self.submit_flag_limit == other.submit_flag_limit
            && self.teams == other.teams
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct Config {
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub ctf: CtfConfig,
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.database == other.database && self.auth == other.auth && self.ctf == other.ctf
    }
}

impl Config {
    #[cfg(test)]
    pub fn test_config() -> Self {
        Config {
            database: DatabaseConfig {
                database_url: "postgres://user:password@localhost/db".to_string(),
            },
            auth: AuthConfig {
                password: "password".to_string(),
            },
            ctf: CtfConfig {
                protocol: ProtocolConfig {
                    protocol: "http".to_string(),
                    team_token: "team_token".to_string(),
                    checksys_host: "localhost".to_string(),
                    checksys_port: 8080,
                },
                flag_format: "flag{format}".to_string(),
                flag_lifetime: 3600,
                submit_period: 300,
                waiting_period: 300,
                submit_flag_limit: 5,
                teams: HashMap::new(),
            },
        }
    }
}
