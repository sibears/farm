use crate::domain::config::{Config, ConfigRepo, ConfigRepoError};

pub struct InMemoryConfigRepository {
    config: Config,
}

impl InMemoryConfigRepository {
    pub fn new(config: &Config) -> Self {
        InMemoryConfigRepository {
            config: config.clone(),
        }
    }
}

impl ConfigRepo for InMemoryConfigRepository {
    fn get_config(&self) -> Result<Config, ConfigRepoError> {
        Ok(self.config.clone())
    }

    fn save_config(&mut self, config: &Config) -> Result<(), ConfigRepoError> {
        self.config = config.clone();
        Ok(())
    }
}
