use once_cell::sync::Lazy;
use prometheus::{IntGaugeVec, opts};
use crate::models::flag::{Flag, FlagStatus};

pub static FLAG_COUNTER: Lazy<IntGaugeVec> = Lazy::new(|| {
    IntGaugeVec::new(opts!("flag_accepted_counter", "Count of accepted flags"), &["type"])
        .expect("Could not create FLAG_ACCEPTED_COUNTER")
});

pub fn update_metrics(flags: &[Flag]) {
    for flag in flags {
        if flag.status != FlagStatus::QUEUED.to_string() {
            FLAG_COUNTER.with_label_values(&[&flag.status]).inc();
            FLAG_COUNTER.with_label_values(&[FlagStatus::QUEUED.to_string().as_str()]).dec();
        }
    }
}