use phf::phf_map;

use crate::{models::flag::Flag, settings::ProtocolConfig};

use self::forcad_http::ForcAdHttp;
use self::ructf_tcp::RuCtfTcp;
use self::ctf01d_http::Ctf01dHttp;

pub mod forcad_http;
pub mod ructf_tcp;
pub mod ctf01d_http;

pub trait ProtocolHandler: Send + Sync {
    fn send_flags(&self, queue_flags: Vec<Flag>, config: &ProtocolConfig) -> Vec<Flag>;
}

pub static PROTOCOLS: phf::Map<&str, &dyn ProtocolHandler> = phf_map!{
    "forcad_http" => &ForcAdHttp,
    "ructf_tcp" => &RuCtfTcp,
    "ctf01d_http" => &Ctf01dHttp
};
