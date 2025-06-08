use crate::domain::flags::entities::{Flag, SaveFlag};

use super::entities::FlagStatus;

#[async_trait]
pub trait FlagRepo: Send + Sync {
    type FlagRepoError: std::error::Error + Send + Sync;

    async fn get(&self, id: i32) -> Result<Flag, Self::FlagRepoError>;
    async fn get_all(&self) -> Result<Vec<Flag>, Self::FlagRepoError>;
    async fn get_all_by_status(
        &self,
        flag_status: FlagStatus,
    ) -> Result<Vec<Flag>, Self::FlagRepoError>;
    async fn save(&self, flag: &SaveFlag) -> Result<usize, Self::FlagRepoError>;
    async fn save_all(&self, flag: &[SaveFlag]) -> Result<usize, Self::FlagRepoError>;
    async fn delete(&self, id: i32) -> Result<usize, Self::FlagRepoError>;
    async fn delete_all(&self, flags: &[Flag]) -> Result<usize, Self::FlagRepoError>;
    async fn update(&self, flag: &Flag) -> Result<usize, Self::FlagRepoError>;
    async fn update_all(&self, flags: &[Flag]) -> Result<usize, Self::FlagRepoError>;
    async fn get_limit(&self, limit: u32) -> Result<Vec<Flag>, Self::FlagRepoError>;
    async fn get_limit_with_offset_from_start(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Flag>, Self::FlagRepoError>;
    async fn get_limit_with_offset_from_end(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Flag>, Self::FlagRepoError>;
    async fn get_last_id(&self) -> Result<i32, Self::FlagRepoError>;
    async fn get_limit_by_status(
        &self,
        flag_status: FlagStatus,
        limit: u32,
    ) -> Result<Vec<Flag>, Self::FlagRepoError>;
    async fn get_all_by_id(&self, ids: &[i32]) -> Result<Vec<Flag>, Self::FlagRepoError>;
    async fn get_total_flags(&self) -> Result<i64, Self::FlagRepoError>;
    async fn get_total_flags_by_status(
        &self,
        flag_status: FlagStatus,
    ) -> Result<i64, Self::FlagRepoError>;
}
