use std::fmt;
use std::str::FromStr;
use std::string::ToString;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// SDP Media Type: application/sdp

#[derive(Clone, Debug)]
pub struct Origin {
    pub username: String,
    pub session_id: String,
    pub session_version: i64,
    // pub nettype   // IN( IANA Registered, Meas `Internet` ) 
    // pub addrtype  // IP4 | IP6
    pub ip_address: IpAddr,
}

#[derive(Clone, Debug)]
pub struct Connection {
    pub address: IpAddr,
    pub ttl    : Option<u8>,
    pub num_addresses: Option<u8>
}

#[derive(Debug)]
pub struct Session {
    name : String,               // session name : mandatory with at least one UTF-8-encoded character
    title: Option<String>  // session title or short information
}

// Time description
#[derive(Debug)]
pub struct Time {
    active: Option<usize>,   // time the session is active
    repeat: Option<usize>    // zero or more repeat times
}


pub type Attr  = String;
pub type Attrs = Vec<Attr>;

#[derive(Debug)]
pub struct Media {
    // m:  media name and transport address
    name    : String,
    address : IpAddr,
    title   : Option<String>,
    connection: Option<Connection>,
    bandwidth : Option<usize>,
    ekey      : Option<String>,
    attrs     : Option<Attrs>
}

#[derive(Debug)]
pub struct SessionDescription {
    version: usize,  // protocol version number, currently only 0
    origin : String, // originator and session identifier : username, id, version number, network address
    session: Session, 
    uri    : Option<String>,  // URI of description
    email  : Option<String>,
    phone  : Option<String>,
    connection: Option<Connection>, // connection information—not required if included in all media
    bandwidth : Option<usize>,  // zero or more bandwidth information lines
    time   : Time,              // One or more time descriptions ("t=" and "r=" lines; see below)
    timezone: Option<String>,   // time zone adjustments
    ekey    : Option<String>,   // encryption key
    attrs   : Option<Attrs>,    // zero or more session attribute lines
    media   : Option<>
}


impl SessionDescription {
    pub fn new () -> SessionDescription {
        SessionDescription {
            ..
        }
    }
}

impl ToString for SessionDescription {
    fn to_string(&self) -> String {
        String::new()
    }
}

impl FromStr for SessionDescription {
    fn from_str(s: &str) -> Result<SessionDescription, &'static str> {
        if s == "" {
            Err("SDP parse error.")
        } else {
            Ok(SessionDescription {
                ..
            })
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
