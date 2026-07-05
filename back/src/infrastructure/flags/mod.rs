pub mod inmemory_repository;
pub mod postgres_repository;
pub use inmemory_repository::*;
pub use postgres_repository::*;

#[cfg(test)]
mod tests {
    use crate::domain::flags::{Flag, FlagRepo, FlagRepoError, FlagStatus, SaveFlag};
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
        repository.save(std::slice::from_ref(&flag)).await.unwrap();
        let last_id = repository.get_last_id().await.unwrap();
        let flag_from_db = repository
            .get(&[last_id])
            .await
            .expect("Flag should be found")
            .into_iter()
            .next()
            .expect("Flag should be found");
        let all_flags = repository.get_all().await.unwrap();
        let flag_filter = all_flags.iter().find(|f| f.id == last_id).unwrap();
        assert_eq!(&flag_from_db, flag_filter);
        assert_eq!(&flag_from_db.flag, &flag.flag);
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_returns_not_found_when_any_id_is_missing(mut repository: impl FlagRepo) {
        let flag = SaveFlag {
            flag: "test_flag".to_string(),
            sploit: Some("test_sploit".to_string()),
            team: Some("test_team".to_string()),
            created_time: chrono::Utc::now().naive_utc(),
            status: FlagStatus::QUEUED,
            checksystem_response: None,
        };
        repository.save(&[flag]).await.unwrap();

        let existing_id = repository.get_last_id().await.unwrap();
        let missing_id = existing_id + 1;
        let err = repository
            .get(&[existing_id, missing_id])
            .await
            .expect_err("Missing id should return NotFound");

        assert!(matches!(err, FlagRepoError::NotFound(id) if id == missing_id));
    }

    #[rstest]
    #[tokio::test]
    async fn test_save_ignores_duplicate_flags(mut repository: impl FlagRepo) {
        let flag = SaveFlag {
            flag: "duplicate_flag".to_string(),
            sploit: Some("test_sploit".to_string()),
            team: Some("test_team".to_string()),
            created_time: chrono::Utc::now().naive_utc(),
            status: FlagStatus::QUEUED,
            checksystem_response: None,
        };

        let first_save_count = repository
            .save(&[flag.clone(), flag.clone()])
            .await
            .unwrap()
            .len();
        let second_save_count = repository
            .save(std::slice::from_ref(&flag))
            .await
            .unwrap()
            .len();
        let all_flags = repository.get_all().await.unwrap();

        assert_eq!(first_save_count, 1);
        assert_eq!(second_save_count, 0);
        assert_eq!(all_flags.len(), 1);
        assert_eq!(all_flags[0].flag, flag.flag);
    }

    #[rstest]
    #[tokio::test]
    async fn test_update_changes_status(mut repository: impl FlagRepo) {
        let flag = SaveFlag {
            flag: "update_flag".to_string(),
            sploit: Some("s".to_string()),
            team: Some("t".to_string()),
            created_time: chrono::Utc::now().naive_utc(),
            status: FlagStatus::WAITING,
            checksystem_response: None,
        };
        repository.save(std::slice::from_ref(&flag)).await.unwrap();
        let id = repository.get_last_id().await.unwrap();
        let mut stored = repository
            .get(&[id])
            .await
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        stored.status = FlagStatus::ACCEPTED;

        let updated = repository
            .update(std::slice::from_ref(&stored))
            .await
            .unwrap();
        let after = repository
            .get(&[id])
            .await
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        assert_eq!(updated, 1);
        assert_eq!(after.status, FlagStatus::ACCEPTED);
    }

    #[rstest]
    #[tokio::test]
    async fn test_update_missing_id_returns_not_found(mut repository: impl FlagRepo) {
        let flag = Flag {
            id: 999,
            flag: "ghost".to_string(),
            sploit: None,
            team: None,
            created_time: chrono::Utc::now().naive_utc(),
            start_waiting_time: None,
            status: FlagStatus::ACCEPTED,
            checksystem_response: None,
        };
        let err = repository
            .update(std::slice::from_ref(&flag))
            .await
            .expect_err("Missing id should return NotFound");
        assert!(matches!(err, FlagRepoError::NotFound(id) if id == 999));
    }
}
