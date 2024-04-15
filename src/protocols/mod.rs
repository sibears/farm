use phf::phf_map;

use crate::{models::flag::Flag, settings::ProtocolConfig};

use self::ctf01d_http::Ctf01dHttp;
use self::forcad_http::ForcAdHttp;
use self::ructf_tcp::RuCtfTcp;

pub mod ctf01d_http;
pub mod forcad_http;
pub mod ructf_tcp;

pub trait ProtocolHandler: Send + Sync {
    fn send_flags(&self, queue_flags: Vec<Flag>, config: &ProtocolConfig) -> Vec<Flag>;
}

pub static PROTOCOLS: phf::Map<&str, &dyn ProtocolHandler> = phf_map! {
    "forcad_http" => &ForcAdHttp,
    "ructf_tcp" => &RuCtfTcp,
    "ctf01d_http" => &Ctf01dHttp
};
