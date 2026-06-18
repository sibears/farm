use crate::domain::flags::{Flag, FlagRepo, FlagRepoError, FlagStatus, SaveFlag};
use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::Arc;

pub struct InMemoryFlagRepository {
    flags: Vec<Flag>,
}

impl InMemoryFlagRepository {
    pub fn new() -> Self {
        InMemoryFlagRepository { flags: Vec::new() }
    }
}

impl Default for InMemoryFlagRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FlagRepo for InMemoryFlagRepository {
    async fn get(&self, ids: &[i32]) -> Result<Vec<Flag>, FlagRepoError> {
        let requested_ids = ids.iter().copied().collect::<HashSet<i32>>();
        let flags = self
            .flags
            .iter()
            .filter(|flag| requested_ids.contains(&flag.id))
            .cloned()
            .collect::<Vec<Flag>>();

        let found_ids = flags.iter().map(|flag| flag.id).collect::<HashSet<i32>>();
        if let Some(missing_id) = ids.iter().copied().find(|id| !found_ids.contains(id)) {
            return Err(FlagRepoError::NotFound(missing_id));
        }

        Ok(flags)
    }

    async fn get_all(&self) -> Result<Arc<[Flag]>, FlagRepoError> {
        Ok(self.flags.clone().into())
    }
    async fn get_by_status(&self, _flag_status: FlagStatus) -> Result<Vec<Flag>, FlagRepoError> {
        todo!()
    }

    async fn save(&mut self, flags: &[SaveFlag]) -> Result<Vec<SaveFlag>, FlagRepoError> {
        let mut inserted = Vec::new();

        for flag in flags {
            if self.flags.iter().any(|stored| stored.flag == flag.flag) {
                continue;
            }

            let mut stored_flag = Flag::from(flag);
            stored_flag.id = self.flags.last().map_or(1, |stored| stored.id + 1);
            self.flags.push(stored_flag);
            inserted.push(flag.clone());
        }

        Ok(inserted)
    }

    async fn delete(&mut self, _flags: &[i32]) -> Result<usize, FlagRepoError> {
        todo!()
    }

    async fn update(&mut self, _flags: &[Flag]) -> Result<usize, FlagRepoError> {
        todo!()
    }

    async fn get_limit(&self, _limit: u32) -> Result<Vec<Flag>, FlagRepoError> {
        todo!()
    }

    async fn get_limit_with_offset_from_start(
        &self,
        _limit: u32,
        _offset: u32,
    ) -> Result<Vec<Flag>, FlagRepoError> {
        todo!()
    }

    async fn get_limit_with_offset_from_end(
        &self,
        _limit: u32,
        _offset: u32,
    ) -> Result<Vec<Flag>, FlagRepoError> {
        todo!()
    }

    async fn get_last_id(&self) -> Result<i32, FlagRepoError> {
        self.flags
            .last()
            .map(|flag| flag.id)
            .ok_or(FlagRepoError::NotFound(0))
    }

    async fn get_limit_by_status(
        &self,
        _flag_status: FlagStatus,
        _limit: u32,
    ) -> Result<Vec<Flag>, FlagRepoError> {
        todo!()
    }

    async fn get_total_flags(&self) -> Result<i64, FlagRepoError> {
        todo!()
    }

    async fn get_total_flags_by_status(
        &self,
        _flag_status: FlagStatus,
    ) -> Result<i64, FlagRepoError> {
        todo!()
    }
}
