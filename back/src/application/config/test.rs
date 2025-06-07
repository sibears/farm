use crate::domain::config::entities::Config;
use crate::domain::config::repository::ConfigRepo;

pub struct MockConfigRepo {
    should_fail: bool,
    config: Config,
}

impl MockConfigRepo {
    pub fn new(should_fail: bool, config: Config) -> Self {
        Self {
            should_fail,
            config,
        }
    }
}

impl ConfigRepo for MockConfigRepo {
    type ConfigRepoError = std::io::Error;

    fn get_config(&self) -> Result<Config, Self::ConfigRepoError> {
        if self.should_fail {
            Err(std::io::Error::other("Mock error"))
        } else {
            Ok(self.config.clone())
        }
    }

    fn save_config(&mut self, config: &Config) -> Result<(), Self::ConfigRepoError> {
        if self.should_fail {
            Err(std::io::Error::other("Mock error"))
        } else {
            self.config = config.clone();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::config::service::ConfigService;
    use crate::domain::config::entities::{
        AuthConfig, Config, CtfConfig, DatabaseConfig, ProtocolConfig,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    fn create_test_config() -> Config {
        Config {
            database: DatabaseConfig {
                database_url: "test_url".to_string(),
            },
            auth: AuthConfig {
                password: "test_password".to_string(),
            },
            ctf: CtfConfig {
                protocol: ProtocolConfig {
                    protocol: "test_protocol".to_string(),
                    team_token: "test_token".to_string(),
                    checksys_host: "test_host".to_string(),
                    checksys_port: 80,
                },
                flag_format: r"\w{31}=".to_string(),
                flag_lifetime: 300,
                submit_period: 5,
                submit_flag_limit: 100,
                teams: HashMap::new(),
                waiting_period: 10,
            },
        }
    }

    #[test]
    fn test_config_service() {
        let config = create_test_config();
        let mock_repo = Arc::new(MockConfigRepo::new(false, config.clone()));
        let service = ConfigService::new(mock_repo);

        let result = service.get_config().unwrap();
        assert_eq!(result.database.database_url, "test_url");
        assert_eq!(result.ctf.flag_format, r"\w{31}=");
    }
}
