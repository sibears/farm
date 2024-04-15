use reqwest::StatusCode;

use crate::{
    models::flag::{Flag, FlagStatus},
    settings::ProtocolConfig,
};

use super::ProtocolHandler;

pub struct Ctf01dHttp;

impl ProtocolHandler for Ctf01dHttp {
    fn send_flags(&self, queue_flags: Vec<Flag>, config: &ProtocolConfig) -> Vec<Flag> {
        let client = reqwest::blocking::Client::new();
        let url = format!(
            "http://{}:{}/flag",
            config.checksys_host, config.checksys_port
        );
        let flag_str: Vec<String> = queue_flags
            .iter()
            .map(|item| item.flag.to_owned())
            .collect();

        let mut updated_flags: Vec<Flag> = Vec::new();
        for flag in flag_str {
            let result = client
                .get(&url)
                .query(&[("teamid", &config.team_token), ("flag", &flag)])
                .send();

            let result = match result {
                Ok(x) => x,
                Err(e) => {
                    error!("Http put error: {}", e.to_string());
                    continue;
                }
            };
            let result_status = result.status();
            let result_body = match result.text() {
                Ok(v) => v,
                Err(e) => {
                    error!("{}", e.to_string());
                    continue;
                }
            };
            info!("Checksys response: {:?}", &result_body);

            let mut old_flag: Flag = queue_flags.iter().find(|x| x.flag == flag).unwrap().clone();
            old_flag.checksystem_response = Some(result_body.clone().into());

            match result_status {
                StatusCode::OK => old_flag.status = FlagStatus::ACCEPTED.to_string().into(),
                StatusCode::BAD_REQUEST => {
                    old_flag.status = FlagStatus::REJECTED.to_string().into()
                }
                StatusCode::FORBIDDEN => old_flag.status = FlagStatus::REJECTED.to_string().into(),
                _ => error!(
                    "Result error: {:?} {:?}",
                    result_status, old_flag.checksystem_response
                ),
            }

            if result_body.contains("service is dead") {
                old_flag.status = FlagStatus::QUEUED.to_string().into();
            }
            updated_flags.push(old_flag);
        }
        info!("Updated: {:?}", updated_flags);
        updated_flags
    }
}
