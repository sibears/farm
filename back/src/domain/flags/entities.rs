use chrono::NaiveDateTime;
use diesel::sql_types::Text;
use diesel_enum::DbEnum;
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
use utoipa::ToSchema;
use rocket::form::FromForm;


use crate::domain::flags::errors::FlagStatusError;
use crate::schema::flags;

#[derive(FromForm)]
pub struct FlagsQuery {
    #[field(default = 20)]
    pub limit: u32,
    #[field(default = 0)]
    pub offset: u32,
}

#[derive(
    Queryable,
    Insertable,
    Serialize,
    Deserialize,
    PartialEq,
    Debug,
    AsChangeset,
    JsonSchema,
    Clone,
    ToSchema,
)]
#[diesel(primary_key(id))]
#[diesel(table_name = flags)]
pub struct Flag {
    pub id: i32,
    pub flag: String,
    pub sploit: Option<String>,
    pub team: Option<String>,
    #[schema(value_type = String, format = "date-time")]
    pub created_time: NaiveDateTime,
    #[schema(value_type = Option<String>, format = "date-time")]
    pub start_waiting_time: Option<NaiveDateTime>,
    pub status: FlagStatus,
    pub checksystem_response: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, JsonSchema, Clone, ToSchema)]
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

#[derive(Insertable)]
#[diesel(table_name = flags)]
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
    JsonSchema,
    Clone,
    Display,
    EnumIter,
    EnumString,
    AsExpression,
    FromSqlRow,
    DbEnum,
    ToSchema,
)]
#[diesel(sql_type = Text)]
#[diesel_enum(error_fn = FlagStatusError::not_found)]
#[diesel_enum(error_type = FlagStatusError)]
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
