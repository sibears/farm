pub mod file_repository;
pub mod inmemory_repository;

pub use file_repository::*;
pub use inmemory_repository::*;

#[cfg(test)]
mod tests {
    use crate::domain::config::{
        AuthConfig, Config, ConfigRepo, CtfConfig, DatabaseConfig, ProtocolConfig,
    };
    use crate::infrastructure::config::InMemoryConfigRepository;
    use rstest::*;
    use std::collections::HashMap;

    #[fixture]
    fn repository() -> impl ConfigRepo {
        let config = Config {
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
                teams: [
                    ("team1".to_string(), "127.0.0.1".to_string()),
                    ("team2".to_string(), "127.0.1.1".to_string()),
                ]
                .iter()
                .cloned()
                .collect(),
            },
        };
        InMemoryConfigRepository::new(&config)
    }

    #[rstest]
    #[tokio::test]
    async fn test_config_repository(mut repository: impl ConfigRepo) {
        let mut config = repository.get_config().unwrap();
        config.auth.password = "updated_password".to_string();
        repository
            .save_config(&config)
            .expect("Failed to update config");
        let updated_config = repository.get_config().unwrap();
        assert_eq!(
            updated_config.auth.password, config.auth.password,
            "Config password should match the updated value"
        );
        assert_eq!(
            updated_config.auth.password, "updated_password",
            "Config password should be updated"
        );
    }
}
