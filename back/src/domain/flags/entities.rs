use chrono::NaiveDateTime;
use regex::Regex;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
use utoipa::ToSchema;

#[derive(FromForm)]
pub struct FlagsQuery {
    #[field(default = 20)]
    pub limit: u32,
    #[field(default = 0)]
    pub offset: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ToSchema)]
pub struct Flag {
    pub id: i32,
    pub flag: String,
    pub sploit: Option<String>,
    pub team: Option<String>,
    pub created_time: NaiveDateTime,
    pub start_waiting_time: Option<NaiveDateTime>,
    pub status: FlagStatus,
    pub checksystem_response: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ToSchema)]
pub struct NewFlag {
    pub flag: String,
    pub sploit: Option<String>,
    pub team: Option<String>,
}

impl NewFlag {
    pub fn match_regex(&self, regex: &Regex) -> bool {
        regex.is_match(&self.flag)
    }
}

pub struct SaveFlag {
    pub flag: String,
    pub sploit: Option<String>,
    pub team: Option<String>,
    pub created_time: NaiveDateTime,
    pub status: FlagStatus,
    pub checksystem_response: Option<String>,
}

impl From<&NewFlag> for SaveFlag {
    fn from(new_flag: &NewFlag) -> Self {
        SaveFlag {
            flag: new_flag.flag.clone(),
            sploit: new_flag.sploit.clone(),
            team: new_flag.team.clone(),
            created_time: chrono::Utc::now().naive_utc(),
            status: FlagStatus::default_status(),
            checksystem_response: None,
        }
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Clone,
    Copy,
    Display,
    EnumIter,
    EnumString,
    ToSchema,
    sqlx::Type,
)]
#[sqlx(type_name = "flag_status", rename_all = "lowercase")]
pub enum FlagStatus {
    QUEUED,
    WAITING,
    SKIPPED,
    ACCEPTED,
    REJECTED,
}

impl FlagStatus {
    pub fn default_status() -> Self {
        FlagStatus::QUEUED
    }
}
