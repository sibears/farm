use chrono::NaiveDateTime;
use crate::domain::flags::entities::{Flag, NewFlag};

pub trait FlagRepo {
    type FlagRepoError;

    fn get_all(&self) -> Result<Vec<Flag>, Self::FlagRepoError>;
    fn get_by_id(&self, id: i32) -> Result<Flag, Self::FlagRepoError>;
    fn save(&self, flag: &NewFlag) -> Result<usize, Self::FlagRepoError>;
    fn save_all(&self, flag: &[NewFlag]) -> Result<usize, Self::FlagRepoError>;
    fn delete_by_id(&self, id: i32) -> Result<usize, Self::FlagRepoError>;
    fn update(&self, flag: &Flag) -> Result<usize, Self::FlagRepoError>;
    fn skip_by_time(&self, skip_time: NaiveDateTime) -> Result<usize, Self::FlagRepoError>;
    fn get_limit(&self, limit: i64) -> Result<Vec<Flag>, Self::FlagRepoError>;
    fn update_all(&self, flags: &[Flag]) -> Result<usize, Self::FlagRepoError>;
    // fn skip_duplicate(&self, flags: Vec<NewFlag>) -> Result<Vec<NewFlag>, Self::ReposError>;
    fn get_last_id(&self) -> Result<i32, Self::FlagRepoError>;
}