use crate::application::config::service::ConfigService;
use crate::application::flags::service::FlagService;
use crate::application::sending::service::SendingService;
use crate::infrastructure::config::file_repository::FileConfigRepo;
use crate::infrastructure::flags::postgres_repository::PostgresFlagRepo;

pub type ConcreteConfigService = ConfigService<FileConfigRepo>;
pub type ConcreteFlagService = FlagService<PostgresFlagRepo, FileConfigRepo>;
pub type ConcreteSendingService = SendingService<PostgresFlagRepo, FileConfigRepo>;
