use crate::config::DbFlagRepo;
use crate::middleware::metrics::{update_metrics, FLAG_COUNTER};
use crate::settings::ProtocolConfig;
use crate::{
    models::flag::Flag,
    repos::flag::FlagRepo,
    settings::Config,
};
use chrono::{Duration, Utc};
use std::sync::Arc;
use std::thread;
use crate::db::connection::init_db;

pub fn flag_handler(config: Arc<Config>) {
    let database_url = config.database.lock().unwrap().database_url.to_string();
    let db_pool = init_db(database_url);
    let flag_repo = DbFlagRepo::new();
    let mut db_conn = db_pool.get().unwrap();
    let flags = flag_repo.find_all(&mut db_conn).unwrap();
    for item in flags {
        FLAG_COUNTER.with_label_values(&[&item.status]).inc();
    }
    loop {
        let lock_ctf_config = config.ctf.lock().unwrap();
        let flag_lifetime = lock_ctf_config.flag_lifetime;
        let submit_period = lock_ctf_config.submit_period;
        let submit_flag_limit = lock_ctf_config.submit_flag_limit;
        let protocol_config = ProtocolConfig::new(
            lock_ctf_config.protocol.protocol.to_owned(),
            lock_ctf_config.protocol.team_token.to_owned(),
            lock_ctf_config.protocol.checksys_host.to_owned(),
            lock_ctf_config.protocol.checksys_port.to_owned(),
        );
        drop(lock_ctf_config);

        let submit_start_time = Utc::now().naive_local();
        let flag_lifetime = Duration::seconds(flag_lifetime as i64);
        let submit_period = Duration::seconds(submit_period as i64);
        let skip_time = submit_start_time.checked_sub_signed(flag_lifetime).unwrap();

        flag_repo.skip_flags(&mut db_conn, skip_time).unwrap();

        let queue_flags = flag_repo.get_limit(&mut db_conn, submit_flag_limit as i64).unwrap();
        info!("Queue flags: {:?}", queue_flags);
        if queue_flags.len() > 0 {
            let updated_flags = submit_flags(queue_flags, protocol_config);
            if updated_flags.len() > 0 {
                flag_repo.update_status(&mut db_conn, updated_flags.as_slice()).unwrap();
            }
            update_metrics(updated_flags.as_slice());
        }

        let submit_spent = Utc::now().naive_local() - submit_start_time;
        if submit_period > submit_spent {
            thread::sleep((submit_period - submit_spent).to_std().unwrap());
        }
    }
}

fn submit_flags(queue_flags: Vec<Flag>, protocol_config: ProtocolConfig) -> Vec<Flag> {
    info!("protocol: {}", protocol_config.protocol);
    let handler = protocol_config.get_protocol_handler();
    handler.send_flags(queue_flags, &protocol_config)
}
