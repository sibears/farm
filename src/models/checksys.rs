use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct ForkadResponse {
    pub flag: Option<String>,
    pub msg: Option<String>,
    pub error: Option<String>,
}
