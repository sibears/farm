use std::borrow::Cow;

use serde::Serialize;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct ApiError {
    pub details: Cow<'static, str>,
}

impl ApiError {
    pub fn new<S>(details: S) -> Self 
        where S: Into<Cow<'static, str>>
    {
        ApiError { 
            details: details.into()
         }
    }
}

#[derive(Debug)]
pub enum BasicAuthError {
    BadCount,
    Invalid
} 