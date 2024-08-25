use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use notify::{RecommendedWatcher, DebouncedEvent, Watcher, channel};
use crate::settings::Config;
use crate::handlers::config_handler::load_config_from_file;

pub fn watch_config_file(config: Arc<Mutex<Config>>, file_path: &str) {
    let mut watcher = notify::recommended_watcher(|res| {
        
    });
    watcher.watch(file_path, RecursiveMode::NonRecursive).unwrap();

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(DebouncedEvent::Write(_)) | Ok(DebouncedEvent::Create(_)) => {
                    println!("Config file changed, reloading...");
                    let new_config = load_config_from_file(file_path);
                    let mut config_lock = config.lock().unwrap();
                    *config_lock = new_config;
                }
                Err(e) => println!("watch error: {:?}", e),
                _ => (),
            }
        }
    });
}