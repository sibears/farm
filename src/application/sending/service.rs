use crate::application::config::service::ConfigService;
use crate::application::flags::service::FlagService;
use crate::domain::flags::entities::{Flag, FlagStatus};
use std::sync::Arc;

pub struct SendingService {
    flag_service: Arc<FlagService>,
    config_service: Arc<ConfigService>,
}

impl SendingService {
    pub fn new(flag_service: Arc<FlagService>, config_service: Arc<ConfigService>) -> Self {
        SendingService {
            flag_service,
            config_service,
        }
    }

    pub fn get_flags_for_senders(&self) -> Result<Vec<Flag>, diesel::result::Error> {
        let mut flags = self.flag_service.next_send_flags()?;
        flags.iter_mut().for_each(|item| {
            item.status = FlagStatus::WAITING;
            item.start_waiting_time = Some(chrono::Utc::now().naive_utc());
        });
        self.flag_service.update_all_flags(&flags)?;
        Ok(flags)
    }

    pub fn update_waiting_flags(&self) -> Result<(), diesel::result::Error> {
        let config = self.config_service.get_config().unwrap();
        let duraction = config.ctf.waiting_period;
        let mut flags = self.flag_service.get_waiting_flags()?;
        flags.iter_mut().for_each(|item| {
            if item.start_waiting_time.unwrap() + chrono::Duration::seconds(duraction.into())
                < chrono::Utc::now().naive_utc()
            {
                item.status = FlagStatus::QUEUED;
                item.start_waiting_time = None;
                info!("Flag with id {} status changed to QUEUED", item.id);
            }
        });

        self.flag_service.update_all_flags(&flags)?;
        Ok(())
    }

    pub fn update_flags_from_sending(&self, flags: &[Flag]) -> Result<(), diesel::result::Error> {
        let original_flags = self.flag_service.get_full_flags(flags)?;

        // Создаем вектор для обновленных флагов
        let flags_to_update: Vec<Flag> = original_flags
            .into_iter()
            .filter(|f| f.status == FlagStatus::WAITING)
            .map(|mut flag| {
                if let Some(sending_flag) = flags.iter().find(|f| f.id == flag.id) {
                    flag.status = sending_flag.status.clone();
                    flag.checksystem_response = sending_flag.checksystem_response.clone();
                    flag.start_waiting_time = None;
                }
                flag
            })
            .collect();

        // Обновляем только если есть флаги для обновления
        if !flags_to_update.is_empty() {
            self.flag_service.update_all_flags(&flags_to_update)?;
        }

        Ok(())
    }
}
