
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

use connection::{ 
	IpAddr, Ipv4Addr, Ipv6Addr, 
	NetType, Ttl, ConnectionInfomation
}


#[derive(Clone, Debug)]
pub struct Origin {
    pub username  : String, // username MUST NOT contain spaces
    pub session_id: String,
    pub session_version: SessionVersion,
    // pub nettype   // IN( IANA Registered, Meas `Internet` ) 
    // pub addrtype  // IP4 | IP6
    pub ip_address: IpAddr,
}