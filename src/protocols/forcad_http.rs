use reqwest::header::HeaderMap;
use std::{collections::HashMap, vec};

use crate::{
    models::flag::{Flag, FlagStatus},
    settings::ProtocolConfig,
};
use serde_json::Value;

use super::ProtocolHandler;

pub struct ForcAdHttp;

impl ProtocolHandler for ForcAdHttp {
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

        let client = reqwest::blocking::Client::new();
        let url = format!(
            "http://{}:{}/flags",
            config.checksys_host, config.checksys_port
        );
        let mut headers = HeaderMap::new();
        headers.insert("X-Team-Token", config.team_token.parse().unwrap());
        let flag_str: Vec<String> = queue_flags
            .iter()
            .map(|item| item.flag.to_owned())
            .collect();

        let result = client.put(url).headers(headers).json(&flag_str).send();
        if result.is_err() {
            error!("Http put error: {:?}", result.unwrap_err());
            return Vec::new();
        }
        let result = match result.unwrap().json::<Value>() {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e.to_string());
                return queue_flags;
            }
        };
        info!("Checksys response: {:?}", &result.to_string());
        if !result["error"].is_null() {
            error!("Response error: {:?}", result["error"]);
            return Vec::new();
        }
        let mut updated_flags: Vec<Flag> = Vec::new();
        for item in result.as_array().unwrap() {
            let mut item = item.as_object().unwrap().to_owned();
            let flag_template = format!("[{}]", &item["flag"]);
            item["msg"] = json!(item["msg"]
                .as_str()
                .unwrap()
                .replace(flag_template.as_str(), ""));
            let mut old_flag: Flag = queue_flags
                .iter()
                .find(|x| x.flag == item["flag"].as_str().unwrap())
                .unwrap()
                .clone();
            let tmp = item["msg"].as_str().unwrap().to_string();
            old_flag.checksystem_response = Some(tmp.into());
            for (status, key_words) in &responses {
                let lowercase_response = old_flag
                    .checksystem_response
                    .to_owned()
                    .unwrap()
                    .to_lowercase();
                if key_words
                    .iter()
                    .any(|word| lowercase_response.contains(word))
                {
                    old_flag.status = status.clone().into();
                    break;
                }
            }
            updated_flags.push(old_flag);
        }
        info!("Updated: {:?}", updated_flags);
        updated_flags
    }
}
