use crate::application::config::ConfigService;
use crate::application::flags::FlagService;
use crate::application::sending::SendingService;
use crate::infrastructure::config::FileConfigRepo;
use crate::infrastructure::flags::PostgresFlagRepo;

pub type ConcreteConfigService = ConfigService<FileConfigRepo>;
pub type ConcreteFlagService = FlagService<PostgresFlagRepo, FileConfigRepo>;
pub type ConcreteSendingService = SendingService<PostgresFlagRepo, FileConfigRepo>;
