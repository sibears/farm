use crate::{application::flags::service::FlagService, domain::flags::entities::FlagStatus};
use rocket_prometheus::{
    prometheus::{Gauge as PromGauge, Opts},
    PrometheusMetrics,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct FlagMetricsService {
    accepted: PromGauge,
    rejected: PromGauge,
    skipped: PromGauge,
    queued: PromGauge,
    waiting: PromGauge,
}

impl FlagMetricsService {
    /// Создаём Gauges и регистрируем их в реестре rocket_prometheus
    pub fn new(prometheus: &PrometheusMetrics) -> Self {
        // Создание и регистрация Gauge для каждого статуса.
        let accepted_opts = Opts::new("flags_accepted", "Number of ACCEPTED flags");
        let accepted = PromGauge::with_opts(accepted_opts).unwrap();
        prometheus
            .registry()
            .register(Box::new(accepted.clone()))
            .unwrap();

        let rejected_opts = Opts::new("flags_rejected", "Number of REJECTED flags");
        let rejected = PromGauge::with_opts(rejected_opts).unwrap();
        prometheus
            .registry()
            .register(Box::new(rejected.clone()))
            .unwrap();

        let skipped_opts = Opts::new("flags_skipped", "Number of SKIPPED flags");
        let skipped = PromGauge::with_opts(skipped_opts).unwrap();
        prometheus
            .registry()
            .register(Box::new(skipped.clone()))
            .unwrap();

        let queued_opts = Opts::new("flags_queued", "Number of QUEUED flags");
        let queued = PromGauge::with_opts(queued_opts).unwrap();
        prometheus
            .registry()
            .register(Box::new(queued.clone()))
            .unwrap();

        let waiting_opts = Opts::new("flags_waiting", "Number of WAITING flags");
        let waiting = PromGauge::with_opts(waiting_opts).unwrap();
        prometheus
            .registry()
            .register(Box::new(waiting.clone()))
            .unwrap();

        Self {
            accepted,
            rejected,
            skipped,
            queued,
            waiting,
        }
    }

    /// Обновляет значения метрик на основе данных из FlagService.
    pub fn update_flags_count(&self, flag_service: &Arc<FlagService>) {
        // Удобная функция для обновления значения конкретного Gauge
        fn update_gauge(flag_service: &FlagService, status: FlagStatus, gauge: &PromGauge) {
            match flag_service.get_total_flags_by_status(status) {
                Ok(count) => gauge.set(count as f64),
                Err(_) => gauge.set(0.0),
            }
        }

        update_gauge(flag_service, FlagStatus::ACCEPTED, &self.accepted);
        update_gauge(flag_service, FlagStatus::REJECTED, &self.rejected);
        update_gauge(flag_service, FlagStatus::SKIPPED, &self.skipped);
        update_gauge(flag_service, FlagStatus::QUEUED, &self.queued);
        update_gauge(flag_service, FlagStatus::WAITING, &self.waiting);
    }
}
