use super::entities::Config;

pub trait ConfigRepo: Send + Sync {
    type ConfigRepoError: std::error::Error + Send + Sync;

    fn get_config(&self) -> Result<Config, Self::ConfigRepoError>;
    fn save_config(&mut self, config: &Config) -> Result<(), Self::ConfigRepoError>;
}
