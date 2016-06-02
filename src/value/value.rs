
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

mod version;
mod origin;
mod connection;
mod attrs;
mod bandwidth;


use self::version::{ ProtocolVersion, SessionVersion };
// use self::origin::Origin;
// use self::connection::ConnectionInfomation as ConnectionInfo;
// use self::attrs::Attrs;
// use self::bandwidth::Bandwidth;


use key::Key;

pub type Origin      = String;
pub type SessionName = String;
pub type SessionInfo = Option<String>;
pub type Uri         = Option<String>;
pub type Email       = Option<String>;
pub type Phone       = Option<String>;
pub type ConnectionInfo = Option<String>;

pub type TimeZone    = Option<String>; // z=<adjustment time> <offset> <adjustment time> <offset> ....
pub type Bandwidth   = Option<String>;
pub type Attr        = Option<String>;

pub type Media       = Option<String>; // m=<media> <port> <proto> <fmt> m=video 49170/2 RTP/AVP 31

/*

Session description
    v=  (protocol version)
    o=  (originator and session identifier)
    s=  (session name)
    i=* (session information)
    u=* (URI of description)
    e=* (email address)
    p=* (phone number)
    c=* (connection information -- not required if included in all media)
    b=* (zero or more bandwidth information lines)
    One or more time descriptions ("t=" and "r=" lines; see below)
    z=* (time zone adjustments)
    k=* (encryption key)
    a=* (zero or more session attribute lines)
    Zero or more media descriptions

Time description
    t=  (time the session is active)
    r=* (zero or more repeat times)

Media description, if present
    m=  (media name and transport address)
    i=* (media title)
    c=* (connection information -- optional if included at session level)
    b=* (zero or more bandwidth information lines)
    k=* (encryption key)
    a=* (zero or more media attribute lines)
*/

// SDP Value
#[derive(Debug, Clone)]
pub enum Value {
    // Session Description
    pub v(ProtocolVersion),  // protocol version
    pub o(Origin),           // originator and session identifier
    pub s(SessionName),      // session name
    pub i(SessionInfo),      // session information
    pub u(Uri),              // URI of description
    pub e(Email),            // email address
    pub p(Phone),            // phone number
    pub c(ConnectionInfo),   // connection information -- not required if included in all media
    pub b(Bandwidth),        // zero or more bandwidth information lines
    pub z(TimeZone),         // time zone adjustments
    pub k(Option<String>),   // encryption key
    pub a(Attr),             // zero or more session attribute lines
    // Time Description
    pub t(usize),           // time the session is active
    pub r(usize),           // zero or more repeat times
    // Media Description
    pub m(Media),           // media name and transport address
}

impl FromStr for Value {
    fn from_str(v: &str, key: Key) -> Option<Value> {
        match key {
            Key::v => {
                
            }
        }
    }
}
// impl Value {

// }