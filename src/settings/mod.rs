use std::{borrow::Cow, env, collections::HashMap, sync::Mutex};
use dotenv::dotenv;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::protocols::{ProtocolHandler, forcad_http::ForcAdHttp, PROTOCOLS};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct DatabaseConfig {
    pub database_url: Cow<'static, str>
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AuthConfig {
    pub password: Cow<'static, str>
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ProtocolConfig {
    pub protocol: Cow<'static, str>,
    pub team_token: Cow<'static, str>,
    pub checksys_host: Cow<'static, str>,
    pub checksys_port: u32
}

impl ProtocolConfig {
    pub fn get_protocol_handler(&self) -> &dyn ProtocolHandler {
        unsafe {
            *PROTOCOLS.get(&self.protocol).unwrap()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CtfConfig {
    pub protocol: ProtocolConfig,
    pub flag_format: Cow<'static, str>,
    pub flag_lifetime: u32,
    pub submit_period: u32,
    pub submit_flag_limit: u32,
    // TODO: Заменить вектор на мапу, чтобы можно было вставлять названия комманд
    pub teams: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Config {
    pub database: Mutex<DatabaseConfig>,
    pub auth: Mutex<AuthConfig>,
    pub ctf: Mutex<CtfConfig>,
}


#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RawConfig {
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub ctf: CtfConfig,
}

impl Config {
    pub fn new(auth_config: AuthConfig, ctf_config: CtfConfig) -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        Config { 
            database: Mutex::new(DatabaseConfig { 
                database_url: database_url.into()
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
    pub fn new<S>(password: S) -> Self 
        where S: Into<Cow<'static, str>> 
    { 
        AuthConfig { password: password.into() }
    }

    pub fn copy(&mut self, another: &AuthConfig) {
        self.password = another.password.to_owned();
    }
}

impl CtfConfig {
    pub fn new<S>(
        proto_config: ProtocolConfig, 
        flag_format: S,
        flag_lifetime: u32,
        submit_period: u32,
        submit_flag_limit: u32,
        teams: HashMap<S, S>
    ) -> Self 
        where S: Into<Cow<'static, str>> 
    { 
        CtfConfig { 
            protocol: proto_config, 
            flag_format: flag_format.into(), 
            flag_lifetime: flag_lifetime, 
            submit_period: submit_period, 
            submit_flag_limit: submit_flag_limit, 
            teams: teams.into_iter().map(|item| (item.0.into(), item.1.into())).collect()
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
    pub fn new<S>(
        protocol: S,
        team_token: S,
        checksys_host: S,
        checksys_port: u32
    ) -> Self 
        where S: Into<Cow<'static, str>> 
    { 
        ProtocolConfig { 
            protocol: protocol.into(), 
            team_token: team_token.into(), 
            checksys_host: checksys_host.into(), 
            checksys_port: checksys_port 
        }
    }

    pub fn copy(&mut self, another: &ProtocolConfig) {
        self.protocol = another.protocol.to_owned();
        self.team_token = another.team_token.to_owned();
        self.checksys_host = another.checksys_host.to_owned();
        self.checksys_port = another.checksys_port;
    }
}