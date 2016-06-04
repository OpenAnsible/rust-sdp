

mod version;
mod base;
mod origin;
mod connection;
mod attr;
mod bandwidth;
mod uri;



use std::fmt;
use std::str::FromStr;
use std::string::ToString;

pub use self::base::{ NetType, AddrType };
pub use self::version::{ ProtocolVersion, SessionVersion };

use self::uri::{ Url as Uri, ParseError };

use self::origin::Origin;
use self::bandwidth::Bandwidth;
use self::connection::ConnectionInfo;



use key::Key;

// https://github.com/niax/rust-email
pub type Email       = Option<String>;
pub type Phone       = Option<String>;

pub type TimeZone    = Option<String>; // z=<adjustment time> <offset> <adjustment time> <offset> ....
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

// #[derive(Debug)]
// pub struct MediaDescription {
//     // Media description, if present
//     m: String, // media name and transport address
//     i: Option<String>, // media title
//     c: Option<ConnectionInfomation>, // connection information -- optional if included at session level
//     b: Option<usize>,  // zero or more bandwidth information lines
//     k: Option<String>, // encryption key
//     a: Option<Attrs>,  // zero or more session attribute lines
// }

// SDP Value
#[derive(Debug, Clone)]
pub enum Value {
    // Session Description
    v(ProtocolVersion),  // protocol version
    o(Origin),           // originator and session identifier
    s(String),           // session name
    i(String),           // session information
    u(Uri),              // URI of description
    e(Email),            // email address
    p(Phone),            // phone number
    c(Option<ConnectionInfo>),   // connection information -- not required if included in all media
    b(Option<Bandwidth>),// zero or more bandwidth information lines
    z(TimeZone),         // time zone adjustments
    k(Option<String>),   // encryption key, https://tools.ietf.org/html/rfc4566#section-5.12
    a(Attr),             // zero or more session attribute lines
    // Time Description
    t(usize),           // time the session is active
    r(Option<usize>),   // zero or more repeat times
    // Media Description
    m(Media),           // media name and transport address
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match *self {
            Value::v(ref pv)     => pv.to_string(),
            Value::o(ref origin) => origin.to_string(),
            Value::s(ref sname)  => sname.to_string(),
            Value::i(ref sinfo)  => sinfo.to_string(),
            Value::u(ref uri)    => uri.to_string(),
            Value::e(ref email)  => match *email {
                Some(ref e)      => e.to_string(),
                None             => "".to_string()
            },
            Value::p(ref p)      => match *p {
                Some(ref p)      => p.to_string(),
                None             => "".to_string()
            },
            Value::c(ref c)      => match *c {
                Some(ref c)      => c.to_string(),
                None             => "".to_string()
            },
            Value::b(ref b)      => match *b {
                Some(ref b)      => b.to_string(),
                None             => "".to_string()
            },
            Value::z(ref z)      => match *z {
                Some(ref z)      => z.to_string(),
                None             => "".to_string()
            },
            Value::k(ref k)      => match *k {
                Some(ref k)      => k.to_string(),
                None             => "".to_string()
            },
            Value::a(ref a)      => match *a {
                Some(ref a)      => a.to_string(),
                None             => "".to_string()
            },
            Value::t(ref t)      => t.to_string(),
            Value::r(ref r)      => match *r {
                Some(ref r)      => r.to_string(),
                None             => "".to_string()
            },
            Value::m(ref m)      => match *m {
                Some(ref m)      => m.to_string(),
                None             => "".to_string()
            },
        }
    }
}

impl Value {
    pub fn from_str(v: &str, key: Key) -> Option<Value> {
        match key {
            Key::v => match ProtocolVersion::from_str(v) {
                Ok(pversion) => Some(Value::v(pversion)),
                Err(_)       => None,
            },
            Key::o => match Origin::from_str(v) {
                Ok(origin) => Some(Value::o(origin)),
                Err(_)     => None
            },
            Key::s => Some(Value::s(v.to_string())),
            Key::i => Some(Value::i(v.to_string())),
            Key::u => match Uri::parse(v) {
            // Key::u => match Uri::from_str(v) {
                Ok(uri)        => Some(Value::u(uri)),
                ParseError     => None
            },
            Key::e => Some(Value::e( Some(v.to_string()) )),
            Key::p => Some(Value::p( Some(v.to_string()) )),
            Key::c => match ConnectionInfo::from_str(v) {
                Ok(connection_info) => Some( Value::c( Some(connection_info) ) ),
                Err(_) => None
            },
            Key::b => match Bandwidth::from_str(v) {
                Ok(bandwidth) => Some( Value::b( Some(bandwidth) ) ),
                Err(_)        => None
            },
            Key::z => Some(Value::z( Some(v.to_string()) )),
            Key::k => Some(Value::k( Some(v.to_string()) )),
            Key::a => Some(Value::a( Some(v.to_string()) )),
            Key::t => {
                // Some(Value::t( Some(v.to_string()) ))
                Some(Value::t(0usize))
            },
            Key::r => {
                // Some(Value::r( Some(v.to_string()) ))
                Some(Value::r( Some(0usize) ))
            },
            Key::m => {
                Some(Value::m( Some(v.to_string()) ))
            }
        }
    }
}
// impl Value {

// }