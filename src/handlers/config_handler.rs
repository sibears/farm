use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind, Config as NotifyConfig};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use std::path::Path;
use crate::settings::Config;
use crate::config::get_config; // Обновите путь импорта в соответствии с вашим проектом

pub fn watch_config_file(config: Arc<Config>, file_path: &'static str) {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, NotifyConfig::default()).unwrap();

    watcher.watch(Path::new(file_path), RecursiveMode::NonRecursive).unwrap();

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(event) => {
                    match event.unwrap().kind {
                        EventKind::Modify(_) | EventKind::Create(_) => {
                            println!("Config file changed, reloading...");
                            let new_config = get_config(file_path);
                            {
                                let mut db_config = config.database.lock().unwrap();
                                *db_config = new_config.database.lock().unwrap().clone();
                            }
                            {
                                let mut auth_config = config.auth.lock().unwrap();
                                *auth_config = new_config.auth.lock().unwrap().clone();
                            }
                            {
                                let mut ctf_config = config.ctf.lock().unwrap();
                                *ctf_config = new_config.ctf.lock().unwrap().clone();
                            }
                        }
                        _ => (),
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}