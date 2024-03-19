use schemars::JsonSchema;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, JsonSchema)]
pub struct StatusStatistic(pub HashMap<String, u64>);
