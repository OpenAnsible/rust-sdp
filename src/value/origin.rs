extern crate regex;


use std::fmt;
use std::str::FromStr;
use std::string::ToString;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use regex::Regex;

use super::{ NetType, AddrType };
use super::{ ProtocolVersion, SessionVersion };
use error::Error;

// o=<username> <sess-id> <sess-version> <nettype> <addrtype> <unicast-address>
// o=mozilla...THIS_IS_SDPARTA-46.0.1 5381835512098962904 0 IN IP4 0.0.0.0

#[derive(Clone, Debug)]
pub struct Origin {
    pub username  : String,   // username MUST NOT contain spaces
    pub session_id: String,
    pub session_version: SessionVersion,
    pub nettype : NetType,   // IN( IANA Registered, Meas `Internet` ) 
    pub addrtype: AddrType,  // IP4 | IP6
    pub address : IpAddr,
}


impl ToString for Origin {
    fn to_string(&self) -> String {
        let origin = "o=".to_string() 
                   + self.username.as_ref() + " " 
                   + self.session_id.as_ref() + " "
                   + self.session_version.to_string().as_ref()  + " "
                   + self.nettype.to_string().as_ref() + " "
                   + self.addrtype.to_string().as_ref() + " "
                   + self.address.to_string().as_ref();
        origin
    }
}
impl FromStr for Origin {
    type Err = Error;
    fn from_str(s: &str) -> Result<Origin, Error> {
        let re  = match Regex::new(r"(\S+)\s(\S+)\s(\d+)\s(IN)\s(IP\d)\s(\d+\.\d+\.\d+\.\d+)") {
            Ok(re) => re,
            Err(e) => {
                println!("[Regex] {:?}", e);
                return Err(Error::Origin);
            }
        };
        let cap = re.captures(s).unwrap();
        let username = match cap.at(1) {
            Some(username) => username.to_string(),
            None           => return Err(Error::SessionName)
        };
        let session_id = match cap.at(2) {
            Some(session_id) => session_id.to_string(),
            None             => return Err(Error::SessionId)
        };
        let session_version = match cap.at(3) {
            Some(session_version)   => match SessionVersion::from_str(session_version) {
                Ok(session_version) => session_version,
                Err(_)              => return Err(Error::SessionVersion)
            },
            None                    => return Err(Error::SessionVersion)
        };
        let nettype = match cap.at(4) {
            Some(nettype) => {
                match NetType::from_str(nettype) {
                    Ok(nettype) => nettype,
                    Err(_)      => return Err(Error::NetType)
                }
            },
            None                => return Err(Error::NetType)
        };
        let addrtype = match cap.at(5) {
            Some(addrtype) => match AddrType::from_str(addrtype) {
                Ok(addrtype) => addrtype,
                Err(_)       => return Err(Error::AddrType)
            },
            None             => return Err(Error::AddrType)
        };
        let address = match cap.at(6) {
            Some(address) => {
                match IpAddr::from_str(address) {
                    Ok(address) => address,
                    Err(e)      => return Err(Error::IpAddress)
                }
            },
            None                => return Err(Error::IpAddress)
        };
        // check addrtype <-> address
        match addrtype {
            AddrType::Ip4 => {
                match address {
                    IpAddr::V6(_) => return Err(Error::AddrType),
                    IpAddr::V4(_) => { }
                };
            },
            AddrType::Ip6 => {
                match address {
                    IpAddr::V4(_) => return Err(Error::AddrType),
                    IpAddr::V6(_) => { }
                };
            }
        }
        Ok(Origin {
            username: username,
            session_id     : session_id,
            session_version: session_version,
            nettype : nettype,
            addrtype: addrtype,
            address : address
        })
    }
}