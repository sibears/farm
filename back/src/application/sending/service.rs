use crate::application::config::ConfigService;
use crate::application::flags::FlagService;
use crate::domain::config::ConfigRepo;
use crate::domain::flags::{Flag, FlagRepo, FlagStatus};
use crate::domain::sending::SendingServiceError;
use sqlx::types::chrono;
use std::sync::Arc;
use tokio::time::Duration;

pub struct SendingService<T: FlagRepo, C: ConfigRepo> {
    flag_service: Arc<FlagService<T, C>>,
    config_service: Arc<ConfigService<C>>,
}

impl<T: FlagRepo, C: ConfigRepo> SendingService<T, C> {
    pub fn new(
        flag_service: Arc<FlagService<T, C>>,
        config_service: Arc<ConfigService<C>>,
    ) -> Self {
        SendingService {
            flag_service,
            config_service,
        }
    }

    pub async fn get_flags_for_senders(&self) -> Result<Vec<Flag>, SendingServiceError> {
        let mut flags = self.flag_service.next_send_flags().await?;
        flags.iter_mut().for_each(|item| {
            item.status = FlagStatus::WAITING;
            item.start_waiting_time = Some(chrono::Utc::now().naive_utc());
        });
        self.flag_service.update_all_flags(&flags).await?;
        Ok(flags)
    }

    pub async fn update_waiting_flags(&self) -> Result<(), SendingServiceError> {
        let config = self.config_service.get_config().unwrap();
        let duraction = config.ctf.waiting_period;
        let mut flags = self.flag_service.get_waiting_flags().await?;
        flags.iter_mut().for_each(|item| {
            if item.start_waiting_time.unwrap() + Duration::new(duraction.into(), 0)
                < chrono::Utc::now().naive_utc()
            {
                item.status = FlagStatus::QUEUED;
                item.start_waiting_time = None;
                info!("Flag with id {} status changed to QUEUED", item.id);
            }
        });

        self.flag_service.update_all_flags(&flags).await?;
        Ok(())
    }

    pub async fn update_flags_from_sending(
        &self,
        flags: &[Flag],
    ) -> Result<Vec<Flag>, SendingServiceError> {
        let ids = flags.iter().map(|flag| flag.id).collect::<Vec<i32>>();
        let original_flags = self.flag_service.get_flags(&ids).await?;

        // Создаем вектор для обновленных флагов
        let flags_to_update: Vec<Flag> = original_flags
            .into_iter()
            .filter(|f| f.status == FlagStatus::WAITING)
            .map(|mut flag| {
                if let Some(sending_flag) = flags.iter().find(|f| f.id == flag.id) {
                    flag.status = sending_flag.status;
                    flag.checksystem_response = sending_flag.checksystem_response.clone();
                    flag.start_waiting_time = None;
                }
                flag
            })
            .collect();

        // Обновляем только если есть флаги для обновления
        if !flags_to_update.is_empty() {
            self.flag_service.update_all_flags(&flags_to_update).await?;
        }

        Ok(flags_to_update)
    }
}

#[cfg(test)]
mod tests {
    use super::SendingService;
    use crate::application::config::ConfigService;
    use crate::application::flags::FlagService;
    use crate::domain::config::Config;
    use crate::domain::flags::{Flag, FlagRepo, FlagStatus, SaveFlag};
    use crate::infrastructure::config::InMemoryConfigRepository;
    use crate::infrastructure::flags::InMemoryFlagRepository;
    use sqlx::types::chrono;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    type Repo = Arc<RwLock<InMemoryFlagRepository>>;

    fn build() -> (
        Repo,
        SendingService<InMemoryFlagRepository, InMemoryConfigRepository>,
    ) {
        let repo: Repo = Arc::new(RwLock::new(InMemoryFlagRepository::new()));
        let config = Config::test_config();
        let config_repo = Arc::new(InMemoryConfigRepository::new(&config));
        let config_service = Arc::new(ConfigService::new(config_repo));
        let flag_service = Arc::new(FlagService::new(repo.clone(), config_service.clone()));
        let sending = SendingService::new(flag_service, config_service);
        (repo, sending)
    }

    async fn seed(repo: &Repo, status: FlagStatus) -> i32 {
        let mut guard = repo.write().await;
        guard
            .save(&[SaveFlag {
                flag: format!("flag_{status}"),
                sploit: Some("s".to_string()),
                team: Some("t".to_string()),
                created_time: chrono::Utc::now().naive_utc(),
                status,
                checksystem_response: None,
            }])
            .await
            .unwrap();
        guard.get_last_id().await.unwrap()
    }

    fn incoming(id: i32, status: FlagStatus) -> Flag {
        Flag {
            id,
            flag: "x".to_string(),
            sploit: Some("s".to_string()),
            team: Some("t".to_string()),
            created_time: chrono::Utc::now().naive_utc(),
            start_waiting_time: None,
            status,
            checksystem_response: Some("ok".to_string()),
        }
    }

    #[tokio::test]
    async fn returns_only_waiting_flags_resolved() {
        let (repo, sending) = build();
        let id = seed(&repo, FlagStatus::WAITING).await;

        let resolved = sending
            .update_flags_from_sending(&[incoming(id, FlagStatus::ACCEPTED)])
            .await
            .unwrap();

        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].status, FlagStatus::ACCEPTED);
    }

    #[tokio::test]
    async fn non_waiting_flags_are_not_resolved() {
        let (repo, sending) = build();
        let id = seed(&repo, FlagStatus::QUEUED).await;

        let resolved = sending
            .update_flags_from_sending(&[incoming(id, FlagStatus::ACCEPTED)])
            .await
            .unwrap();

        assert!(resolved.is_empty());
    }
}
