pub mod inmemory_repository;
pub mod postgres_repository;
pub use inmemory_repository::*;
pub use postgres_repository::*;

#[cfg(test)]
mod tests {
    use crate::domain::flags::{FlagRepo, FlagStatus, SaveFlag};
    use crate::infrastructure::flags::InMemoryFlagRepository;
    use rstest::*;

    #[fixture]
    fn repository() -> impl FlagRepo {
        let repo = InMemoryFlagRepository::new();
        return repo;
    }

    #[rstest]
    #[tokio::test]
    async fn test_save_get_flag(mut repository: impl FlagRepo) {
        let flag = SaveFlag {
            flag: "test_flag".to_string(),
            sploit: Some("test_sploit".to_string()),
            team: Some("test_team".to_string()),
            created_time: chrono::Utc::now().naive_utc(),
            status: FlagStatus::QUEUED,
            checksystem_response: None,
        };
        repository.save(&flag).await.unwrap();
        let last_id = repository.get_last_id().await.unwrap();
        let flag_from_db = repository.get(last_id).await.expect("Flag should be found");
        let all_flags = repository.get_all().await.unwrap();
        let flag_filter = all_flags.iter().find(|f| f.id == last_id).unwrap();
        assert_eq!(&flag_from_db, flag_filter);
        assert_eq!(flag_from_db, flag);
    }
}
