use crate::domain::flags::entities::{Flag, SaveFlag};

use super::entities::FlagStatus;

pub trait FlagRepo: Send + Sync {
    type FlagRepoError;

    fn get(&self, id: i32) -> Result<Flag, Self::FlagRepoError>;
    fn get_all(&self) -> Result<Vec<Flag>, Self::FlagRepoError>;
    fn get_all_by_status(&self, flag_status: FlagStatus) -> Result<Vec<Flag>, Self::FlagRepoError>;
    fn save(&self, flag: &SaveFlag) -> Result<usize, Self::FlagRepoError>;
    fn save_all(&self, flag: &[SaveFlag]) -> Result<usize, Self::FlagRepoError>;
    fn delete(&self, id: i32) -> Result<usize, Self::FlagRepoError>;
    fn delete_all(&self, flags: &[Flag]) -> Result<usize, Self::FlagRepoError>;
    fn update(&self, flag: &Flag) -> Result<usize, Self::FlagRepoError>;
    fn update_all(&self, flags: &[Flag]) -> Result<usize, Self::FlagRepoError>;
    fn get_limit(&self, limit: u32) -> Result<Vec<Flag>, Self::FlagRepoError>;
    fn get_last_id(&self) -> Result<i32, Self::FlagRepoError>;
    fn get_limit_by_status(
        &self,
        flag_status: FlagStatus,
        limit: u32,
    ) -> Result<Vec<Flag>, Self::FlagRepoError>;
}
