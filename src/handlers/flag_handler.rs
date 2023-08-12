use std::{time::Duration, thread};

use chrono::Utc;

use crate::settings::Config;

pub fn flag_handler(config: Config) {
    loop {
        let submit_start_time = Utc::now().naive_local();
        
        thread::sleep(Duration::from_millis(100));
    }
}