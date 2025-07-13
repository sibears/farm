pub mod service;
pub use service::*;

#[cfg(test)]
mod tests {
    use crate::application::config::ConfigService;
    use crate::domain::config::{Config, ConfigRepo};
    use crate::infrastructure::config::InMemoryConfigRepository;
    use rstest::*;
    use std::sync::Arc;

    #[fixture]
    fn service() -> ConfigService<InMemoryConfigRepository> {
        let config = Config::test_config();
        let repo = Arc::new(InMemoryConfigRepository::new(&config));
        let service = ConfigService::new(repo);
        return service;
    }

    #[rstest]
    #[tokio::test]
    async fn test_config_service(service: ConfigService<InMemoryConfigRepository>) {
        let config = service.get_config().unwrap();
        let test_config = Config::test_config();
        assert_eq!(config, test_config);
    }
}
