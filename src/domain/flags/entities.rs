use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

use crate::schema::flags;

#[derive(
    Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, AsChangeset, JsonSchema, Clone,
)]
#[diesel(primary_key(id))]
#[diesel(table_name = flags)]
pub struct Flag {
    pub id: i32,
    pub flag: String,
    pub sploit: Option<String>,
    pub team: Option<String>,
    pub time: NaiveDateTime,
    pub status: FlagStatus,
    pub checksystem_response: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, PartialEq, Debug, JsonSchema)]
#[diesel(table_name = flags)]
pub struct NewFlag {
    pub flag: String,
    pub sploit: Option<String>,
    pub team: Option<String>,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize, PartialEq, JsonSchema, Clone, Display, EnumIter, EnumString)]
#[ExistingTypePath = "crate::schema::sql_types::FlagStatus"]
pub enum FlagStatus {
    QUEUED,
    SKIPPED,
    ACCEPTED,
    REJECTED,
}
