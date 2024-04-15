use super::ProtocolHandler;
use crate::models::flag::{Flag, FlagStatus};
use crate::settings::ProtocolConfig;
use log::info;
use std::collections::HashMap;
use std::ffi::CStr;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct RuCtfTcp;

impl ProtocolHandler for RuCtfTcp {
    fn send_flags(&self, queue_flags: Vec<Flag>, config: &ProtocolConfig) -> Vec<Flag> {
        let responses: HashMap<String, Vec<&str>> = HashMap::from([
            (
                FlagStatus::QUEUED.to_string(),
                vec![
                    "timeout",
                    "game not started",
                    "try again later",
                    "game over",
                    "is not up",
                    "no such flag",
                ],
            ),
            (
                FlagStatus::ACCEPTED.to_string(),
                vec!["accepted", "congrat"],
            ),
            (
                FlagStatus::REJECTED.to_string(),
                vec![
                    "bad",
                    "wrong",
                    "expired",
                    "unknown",
                    "your own",
                    "too old",
                    "not in database",
                    "already submitted",
                    "invalid flag",
                ],
            ),
        ]);
        match TcpStream::connect(format!("{}:{}", config.checksys_host, config.checksys_port)) {
            Ok(mut stream) => {
                let mut buf: [u8; 4096] = [0; 4096];
                match stream.read(&mut buf) {
                    Ok(_) => {
                        let welcome_msg = std::str::from_utf8(&buf).unwrap().to_lowercase();
                        if !welcome_msg.contains("enter your flags") {
                            return Vec::new();
                        }
                        let mut updated_flags: Vec<Flag> = Vec::new();
                        for mut flag in queue_flags {
                            stream.write((flag.flag).as_bytes()).unwrap();
                            buf.fill(0);
                            match stream.read(&mut buf) {
                                Ok(_) => {
                                    let answ =
                                        CStr::from_bytes_until_nul(&buf).unwrap().to_string_lossy();
                                    let msg =
                                        answ.replace(format!("[{}] ", flag.flag).as_str(), "");
                                    flag.checksystem_response = Some(msg.clone().into());
                                    for (status, key_words) in &responses {
                                        if key_words
                                            .iter()
                                            .any(|word| msg.to_lowercase().contains(word))
                                        {
                                            flag.status = status.clone().into();
                                            break;
                                        }
                                    }
                                    updated_flags.push(flag);
                                }
                                Err(e) => error!("Failed to receive data from check system: {}", e),
                            }
                        }
                        info!("Checksys response: {:?}", updated_flags);
                        return updated_flags;
                    }
                    Err(e) => error!("Failed to receive data from check system: {}", e),
                }
            }
            Err(e) => error!("Failed to connect to check system: {}", e),
        }
        Vec::new()
    }
}
