use super::{Config, ConfigRepoError};

pub trait ConfigRepo: Send + Sync {
    fn get_config(&self) -> Result<Config, ConfigRepoError>;
    fn save_config(&mut self, config: &Config) -> Result<(), ConfigRepoError>;
}
