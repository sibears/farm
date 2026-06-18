use crate::{
    application::flags::FlagService,
    domain::{
        config::ConfigRepo,
        flags::{Flag, FlagRepo, FlagStatus, SaveFlag},
    },
};
use rocket_prometheus::{
    prometheus::{CounterVec, Gauge as PromGauge, Opts},
    PrometheusMetrics,
};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// Максимум уникальных значений на одну метку (`sploit`/`team`) до схлопывания
/// новых значений в `"other"`. Защищает Prometheus от раздувания кардинальности
/// при свободных клиентских строках.
const LABEL_CARDINALITY_CAP: usize = 200;

#[derive(Clone)]
pub struct FlagMetricsService {
    accepted: PromGauge,
    rejected: PromGauge,
    skipped: PromGauge,
    queued: PromGauge,
    waiting: PromGauge,
    /// Событийный счётчик «получено флагов» с разбивкой по сплойту и команде.
    received: CounterVec,
    /// Событийный счётчик «обработано флагов» по сплойту, команде и результату.
    processed: CounterVec,
    /// Множества «виденных» значений меток для капа кардинальности (раздельные).
    seen_sploits: Arc<Mutex<HashSet<String>>>,
    seen_teams: Arc<Mutex<HashSet<String>>>,
}

impl FlagMetricsService {
    /// Создаём Gauges и Counters и регистрируем их в реестре rocket_prometheus
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

        // Размеченные событийные счётчики.
        let received = CounterVec::new(
            Opts::new(
                "flags_received_total",
                "Total flags received per sploit/team",
            ),
            &["sploit", "team"],
        )
        .unwrap();
        prometheus
            .registry()
            .register(Box::new(received.clone()))
            .unwrap();

        let processed = CounterVec::new(
            Opts::new(
                "flags_processed_total",
                "Total flags resolved per sploit/team/result",
            ),
            &["sploit", "team", "result"],
        )
        .unwrap();
        prometheus
            .registry()
            .register(Box::new(processed.clone()))
            .unwrap();

        Self {
            accepted,
            rejected,
            skipped,
            queued,
            waiting,
            received,
            processed,
            seen_sploits: Arc::new(Mutex::new(HashSet::new())),
            seen_teams: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Нормализует значение метки: пусто/None → `"unknown"`; при превышении капа
    /// уникальных значений новые схлопываются в `"other"`.
    fn normalize_label(value: Option<&str>, seen: &Arc<Mutex<HashSet<String>>>) -> String {
        let value = match value.map(str::trim).filter(|s| !s.is_empty()) {
            Some(v) => v.to_string(),
            None => return "unknown".to_string(),
        };

        let mut seen = seen.lock().unwrap();
        if seen.contains(&value) {
            value
        } else if seen.len() < LABEL_CARDINALITY_CAP {
            seen.insert(value.clone());
            value
        } else {
            "other".to_string()
        }
    }

    /// Инкрементирует счётчик «получено» по фактически вставленным флагам.
    pub fn record_received(&self, flags: &[SaveFlag]) {
        for flag in flags {
            let sploit = Self::normalize_label(flag.sploit.as_deref(), &self.seen_sploits);
            let team = Self::normalize_label(flag.team.as_deref(), &self.seen_teams);
            self.received.with_label_values(&[&sploit, &team]).inc();
        }
    }

    /// Инкрементирует счётчик «обработано» по терминально-резолвнутым флагам.
    pub fn record_processed(&self, flags: &[Flag]) {
        for flag in flags {
            let sploit = Self::normalize_label(flag.sploit.as_deref(), &self.seen_sploits);
            let team = Self::normalize_label(flag.team.as_deref(), &self.seen_teams);
            // Display у FlagStatus (strum) отдаёт ВЕРХНИЙ регистр — приводим явно.
            let result = flag.status.to_string().to_lowercase();
            self.processed
                .with_label_values(&[&sploit, &team, &result])
                .inc();
        }
    }

    /// Обновляет значения метрик на основе данных из FlagService.
    pub async fn update_flags_count<T: FlagRepo, C: ConfigRepo>(
        &self,
        flag_service: &Arc<FlagService<T, C>>,
    ) {
        // Удобная функция для обновления значения конкретного Gauge
        async fn update_gauge<T: FlagRepo, C: ConfigRepo>(
            flag_service: &FlagService<T, C>,
            status: FlagStatus,
            gauge: &PromGauge,
        ) {
            match flag_service.get_total_flags_by_status(status).await {
                Ok(count) => gauge.set(count as f64),
                Err(_) => gauge.set(0.0),
            }
        }

        update_gauge(flag_service, FlagStatus::ACCEPTED, &self.accepted).await;
        update_gauge(flag_service, FlagStatus::REJECTED, &self.rejected).await;
        update_gauge(flag_service, FlagStatus::SKIPPED, &self.skipped).await;
        update_gauge(flag_service, FlagStatus::QUEUED, &self.queued).await;
        update_gauge(flag_service, FlagStatus::WAITING, &self.waiting).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn metrics() -> FlagMetricsService {
        let prometheus = PrometheusMetrics::new();
        FlagMetricsService::new(&prometheus)
    }

    fn save_flag(flag: &str, sploit: Option<&str>, team: Option<&str>) -> SaveFlag {
        SaveFlag {
            flag: flag.to_string(),
            sploit: sploit.map(String::from),
            team: team.map(String::from),
            created_time: Utc::now().naive_utc(),
            status: FlagStatus::QUEUED,
            checksystem_response: None,
        }
    }

    fn flag(id: i32, sploit: Option<&str>, team: Option<&str>, status: FlagStatus) -> Flag {
        Flag {
            id,
            flag: format!("flag_{id}"),
            sploit: sploit.map(String::from),
            team: team.map(String::from),
            created_time: Utc::now().naive_utc(),
            start_waiting_time: None,
            status,
            checksystem_response: None,
        }
    }

    #[test]
    fn record_received_increments_per_label_pair() {
        let metrics = metrics();
        metrics.record_received(&[
            save_flag("a", Some("sploit1"), Some("team1")),
            save_flag("b", Some("sploit1"), Some("team1")),
        ]);
        assert_eq!(
            metrics
                .received
                .with_label_values(&["sploit1", "team1"])
                .get(),
            2.0
        );
    }

    #[test]
    fn record_received_uses_unknown_for_empty_sploit() {
        // Covers AE3
        let metrics = metrics();
        metrics.record_received(&[save_flag("a", None, Some("team1"))]);
        assert_eq!(
            metrics
                .received
                .with_label_values(&["unknown", "team1"])
                .get(),
            1.0
        );
    }

    #[test]
    fn record_received_collapses_overflow_to_other() {
        // Covers AE4
        let metrics = metrics();
        for i in 0..=LABEL_CARDINALITY_CAP {
            metrics.record_received(&[save_flag("f", Some(&format!("sploit{i}")), Some("team1"))]);
        }
        // Значение сверх капа учитывается под "other".
        assert!(
            metrics
                .received
                .with_label_values(&["other", "team1"])
                .get()
                >= 1.0
        );
    }

    #[test]
    fn record_processed_uses_lowercase_result() {
        // Covers AE2
        let metrics = metrics();
        metrics.record_processed(&[
            flag(1, Some("s"), Some("t"), FlagStatus::ACCEPTED),
            flag(2, Some("s"), Some("t"), FlagStatus::REJECTED),
        ]);
        assert_eq!(
            metrics
                .processed
                .with_label_values(&["s", "t", "accepted"])
                .get(),
            1.0
        );
        assert_eq!(
            metrics
                .processed
                .with_label_values(&["s", "t", "rejected"])
                .get(),
            1.0
        );
    }

    #[test]
    fn sploit_and_team_caps_are_independent() {
        let metrics = metrics();
        // Заполняем кап по sploit.
        for i in 0..LABEL_CARDINALITY_CAP {
            metrics.record_received(&[save_flag("f", Some(&format!("s{i}")), Some("team1"))]);
        }
        // Новый team всё ещё принимается (его кап не заполнен), sploit → other.
        metrics.record_received(&[save_flag("f", Some("s_overflow"), Some("team_new"))]);
        assert!(
            metrics
                .received
                .with_label_values(&["other", "team_new"])
                .get()
                >= 1.0
        );
    }
}
