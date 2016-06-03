use std::fmt;
use std::str::FromStr;
use std::string::ToString;

use error::Error;

#[derive(Debug, Clone)]
pub enum NetType {
    In,  // Internet
}

#[derive(Debug, Clone)]
pub enum AddrType {
	Ip4,
	Ip6
}


impl FromStr for NetType {
	type Err = Error;
    fn from_str(s: &str) -> Result<NetType, Error> {
        match s.to_lowercase().as_ref() {
            "internet" | "in" => Ok(NetType::In),
            _                 => Err(Error::NetType)
        }
    }
}
impl ToString for NetType {
    fn to_string(&self) -> String {
        match *self {
            NetType::In => "IN".to_string()
        }
    }
}

impl FromStr for AddrType {
	type Err = Error;
    fn from_str(s: &str) -> Result<AddrType, Error> {
        match s.to_lowercase().as_ref() {
            "ip4" | "ipv4" => Ok(AddrType::Ip4),
            "ip6" | "ipv6" => Ok(AddrType::Ip6),
            _              => Err(Error::AddrType)
        }
    }
}
impl ToString for AddrType {
    fn to_string(&self) -> String {
        match *self {
            AddrType::Ip4 => "IP4".to_string(),
            AddrType::Ip6 => "IP6".to_string()
        }
    }
}