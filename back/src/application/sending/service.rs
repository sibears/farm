use crate::application::config::service::ConfigService;
use crate::application::flags::service::FlagService;
use crate::domain::config::repository::ConfigRepo;
use crate::domain::flags::entities::{Flag, FlagStatus};
use crate::domain::flags::repository::FlagRepo;
use std::sync::Arc;

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

    pub async fn get_flags_for_senders(&self) -> Result<Vec<Flag>, T::FlagRepoError> {
        let mut flags = self.flag_service.next_send_flags().await?;
        flags.iter_mut().for_each(|item| {
            item.status = FlagStatus::WAITING;
            item.start_waiting_time = Some(chrono::Utc::now().naive_utc());
        });
        self.flag_service.update_all_flags(&flags).await?;
        Ok(flags)
    }

    pub async fn update_waiting_flags(&self) -> Result<(), T::FlagRepoError> {
        let config = self.config_service.get_config().unwrap();
        let duraction = config.ctf.waiting_period;
        let mut flags = self.flag_service.get_waiting_flags().await?;
        flags.iter_mut().for_each(|item| {
            if item.start_waiting_time.unwrap() + chrono::Duration::seconds(duraction.into())
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

    pub async fn update_flags_from_sending(&self, flags: &[Flag]) -> Result<(), T::FlagRepoError> {
        let original_flags = self.flag_service.get_full_flags(flags).await?;

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

        Ok(())
    }
}
