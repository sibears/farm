use std::borrow::Cow;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
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