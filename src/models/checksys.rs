use std::borrow::Cow;

use serde::Serialize;
use serde::Deserialize;


#[derive(Serialize, Deserialize, Debug)]
pub struct ForkadResponse {
    pub flag: Option<Cow<'static, str>>,
    pub msg: Option<Cow<'static, str>>,
    pub error: Option<Cow<'static, str>>
}