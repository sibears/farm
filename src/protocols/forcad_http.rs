use std::{borrow::Cow, collections::HashMap, vec, ops::Deref};

use lazy_static::lazy_static;
use reqwest::header::HeaderMap;

use crate::{models::{flag::{Flag, FlagStatus}, checksys::ForkadResponse}, settings::{Config, ProtocolConfig}};
use rocket::{serde::json::Json, log::private::debug};
use serde_json::Value;

use super::{ProtocolHandler, PROTOCOLS};

pub struct ForcAdHttp;

impl ProtocolHandler for ForcAdHttp {
    fn send_flags(&self, queue_flags: Vec<Flag>, config: &ProtocolConfig) -> Vec<Flag> {

        let responses: HashMap<String, Vec<&str>> = HashMap::from([
            (FlagStatus::QUEUED.to_string(), 
                vec!["timeout", 
                "game not started", 
                "try again later", 
                "game over", 
                "is not up", 
                "no such flag"]),
            (FlagStatus::ACCEPTED.to_string(),
                vec!["accepted", "congrat"]),
            (FlagStatus::REJECTED.to_string(),
                vec!["bad", "wrong", "expired", "unknown", "your own",
                "too old", "not in database", "already submitted", "invalid flag"])
        ]);
    
        let client = reqwest::blocking::Client::new();
        let url = format!("http://{}:{}/flags", config.checksys_host, config.checksys_port);
        let mut headers = HeaderMap::new();
        headers.insert("X-Team-Token", config.team_token.parse().unwrap());
        let flag_str: Vec<Cow<'static, str>> = queue_flags.iter().map(|item| item.flag.to_owned()).collect();
    
        let result = client.put(url)
            .headers(headers)
            .json(&flag_str)
            .send();
        if result.is_err() {
            return Vec::new();
        }
        let result = match result.unwrap().json::<Value>() {
            Ok(v) => v,
            Err(e) => {
                debug!("{}", e.to_string());
                return queue_flags;
            }
        };
        debug!("Checksys response: {:?}", &result.to_string());
        if !result["error"].is_null() {
            error!("{:?}", result["error"]);
            return Vec::new();
        }
        let mut updated_flags: Vec<Flag> = Vec::new();
        for item in result.as_array().unwrap() {
            debug!("item: {}", item);
            let mut item = item.as_object().unwrap().to_owned();
            item["msg"] = json!(item["msg"].as_str().unwrap()[34..]);
            let mut old_flag: Flag = queue_flags.iter().find(|x| x.flag == item["flag"].as_str().unwrap()).unwrap().clone();
            let tmp = item["msg"].as_str().unwrap().to_string();
            old_flag.checksystem_response = Some(tmp.into());
            for (status, key_words) in &responses {
                if key_words.iter().any(|word| old_flag.checksystem_response.to_owned().unwrap().contains(word)) {
                    old_flag.status = status.clone().into();
                    break;
                }
            }
            updated_flags.push(old_flag);
        }
        updated_flags
    }
}