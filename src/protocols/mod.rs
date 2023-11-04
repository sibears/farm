use std::collections::HashMap;
use once_cell::sync::Lazy;
use phf::phf_map;

use crate::{models::flag::Flag, settings::ProtocolConfig};

use self::forcad_http::ForсAdHttp;
use crate::protocols::ructf_tcp::RuCtfTcp;

pub mod forcad_http;
pub mod ructf_tcp;

pub trait ProtocolHandler: Send + Sync {
    fn send_flags(&self, queue_flags: Vec<Flag>, config: &ProtocolConfig) -> Vec<Flag>;
}

pub static mut PROTOCOLS: phf::Map<&str, &dyn ProtocolHandler> = phf_map!{
    "forсad_http" => &ForсAdHttp,
    "ructf_tcp" => &RuCtfTcp
};