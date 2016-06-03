extern crate regex;


use std::fmt;
use std::str::FromStr;
use std::string::ToString;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use regex::Regex;

use super::{ NetType, AddrType };
use super::{ ProtocolVersion, SessionVersion };

use error::Error;

// Unicast: 单播, Broadcast: 广播, Multicast: 组播


#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    // Ipv4  <base multicast address>[/<ttl>]/<number of addresses>
    // One address c=<nettype> <addrtype> <connection-address>
    // c=IN IP4 0.0.0.0/127
    pub nettype : NetType,
    pub addrtype: AddrType,
    pub address : IpAddr,
    pub ttl     : u8,
    pub num     : Option<u8>
}

impl ToString for ConnectionInfo {
    fn to_string(&self) -> String {
        let connection_info = self.nettype.to_string() + " "
                            + self.addrtype.to_string().as_ref() + " "
                            + self.address.to_string().as_ref() + "/"
                            + self.ttl.to_string().as_ref();
        match self.num {
            Some(num) => connection_info + "/" + num.to_string().as_ref(),
            None      => connection_info
        }
    }
}
impl FromStr for ConnectionInfo {
    type Err = Error;
    fn from_str(s: &str) -> Result<ConnectionInfo, Error> {
        // c=IN IP4 0.0.0.0/127/3
        let re  = match Regex::new(r"(IN)\s(IP\d)\s(\d+.\d+.\d+.\d+)"){
            Ok(re) => re,
            Err(e) => {
                println!("[Regex] {:?}", e);
                return Err(Error::ConnectionInfo);
            }
        };
        let cap = re.captures(s).unwrap();
        let nettype = match cap.at(1) {
            Some(nettype) => {
                match NetType::from_str(nettype) {
                    Ok(nettype) => nettype,
                    Err(_)      => return Err(Error::NetType)
                }
            },
            None             => return Err(Error::NetType)
        };
        let addrtype = match cap.at(2) {
            Some(addrtype)   => match AddrType::from_str(addrtype) {
                Ok(addrtype) => addrtype,
                Err(_)       => return Err(Error::AddrType)
            },
            None             => return Err(Error::AddrType)
        };
        let address = match cap.at(3) {
            Some(address)    => {
                match IpAddr::from_str(address) {
                    Ok(address) => address,
                    Err(_)      => return Err(Error::IpAddress)
                }
            },
            None                => return Err(Error::IpAddress)
        };
        // check addrtype <-> address
        match addrtype {
            AddrType::Ip4 => {
                match address {
                    IpAddr::V6(_) => return Err(Error::AddrType),
                    IpAddr::V4(_) => {  }
                };
            },
            AddrType::Ip6 => {
                match address {
                    IpAddr::V4(_) => return Err(Error::AddrType),
                    IpAddr::V6(_) => {  }
                };
            }
        }
        // parse ttl, number of address
        let v: Vec<&str> = s.split('/').collect();
        let mut ttl = 60u8;
        let mut num = None;

        if v.len() ==  2 {
            ttl = match u8::from_str(v[1]){
                Ok(i)  => i,
                Err(_) => {
                    println!("WARN: TTL Must be 0-255");
                    60u8
                }
            };
        } else if v.len() == 3 {
            ttl = match u8::from_str(v[1]){
                Ok(i)  => i,
                Err(_) => {
                    println!("WARN: TTL Must be 0-255");
                    60u8
                }
            };
            num = match u8::from_str(v[2]){
                Ok(i)  => Some(i),
                Err(_) => {
                    println!("WARN: TTL Must be 0-255");
                    None
                }
            };
        }
        Ok(ConnectionInfo{
            nettype : nettype,
            addrtype: addrtype,
            address : address,
            ttl: ttl,
            num: num
        })
    }
}

