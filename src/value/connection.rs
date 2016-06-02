use std::fmt;
use std::str::FromStr;
use std::string::ToString;
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// Unicast: 单播, Broadcast: 广播, Multicast: 组播

#[derive(Debug, Clone)]
pub enum NetType {
    In,  // Internet
}

#[derive(Debug, Clone)]
pub enum Ttl {
    Some(u8), // MUST be in the range 0-255
    None
}


#[derive(Debug, Clone)]
pub struct ConnectionInfomation {
    
    // Ipv4  <base multicast address>[/<ttl>]/<number of addresses>
    // One address c=<nettype> <addrtype> <connection-address>
    // c=IN IP4 0.0.0.0/127
    // Multicast addresses
    // c=IN IP4 224.2.1.1/127/2
    // c=IN IP4 224.2.1.1/127
    // c=IN IP4 224.2.1.2/127
    
    // Ipv6
    // c=IN IP6 FF15::101/3
    // c=IN IP6 FF15::101
    // c=IN IP6 FF15::102
    // c=IN IP6 FF15::103

    pub nettype : NetType,
    // addrtype: AddrType,
    pub address : IpAddr,
    // IPv4 multicast connection address MUST also have a time to live (TTL) value present in addition to the multicast address.
    // TTL values MUST be in the range 0-255
    // IPv6 multicast does not use TTL scoping, and hence the TTL value MUST NOT be present for IPv6 multicast.
    pub ttl     : Ttl,
    pub number_of_addresses: Option<u8>
}


impl Ttl {
    pub fn new (ttl: u8) -> Ttl {
        Ttl::Some(ttl)
    }
    pub fn ttl(&self) -> Option<u8> {
        match *self {
            Ttl::Some(ref ttl) => Some(ttl.as_ref()),
            Ttl::None          => None
        }
    }
}


impl FromStr for NetType {
    fn from_str(s: &str) -> Result<NetType, &'static str> {
        match s.to_lowercase().as_ref() {
            "internet" | "in" => Ok(NetType::In),
            _                 => Err("parse net type fail.")
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


impl ConnectionInfomation {
    pub fn new()   -> Result<ConnectionInfomation, &'static str> {
        Err("TODO.")
    }
    pub fn parse() -> Result<ConnectionInfomation, &'static str> {
        Err("TODO.")
    }
    // pub fn is_multicast(&self) -> bool {
    //     false
    // }
    // pub fn is_broadcast(&self) -> bool {
    //     false
    // }


}

impl ToString for ConnectionInfomation {
    fn to_string(&self) -> String {
        "TODO".to_string()
    }
}
impl FromStr for ConnectionInfomation {
    fn from_str(s: &str) -> Result<ConnectionInfomation, &'static str> {
        Err("TODO.")
    }
}

