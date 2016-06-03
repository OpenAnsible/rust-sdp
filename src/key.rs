use std::fmt;
use std::str::FromStr;
use std::string::ToString;
// use std::error::Error;
use error::Error;

// SDP Key
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Key {
    // Session Description
    v,
    o,
    s,
    i,
    u,
    e,
    p,
    c,
    b,
    z,
    k,
    a,
    // Time Description
    t,
    r,
    // Media Description
    m,
}

impl ToString for Key {
    fn to_string(&self) -> String {
        match *self {
            Key::v => "v".to_string(),
            Key::o => "o".to_string(),
            Key::s => "s".to_string(),
            Key::i => "i".to_string(),
            Key::u => "u".to_string(),
            Key::e => "e".to_string(),
            Key::p => "p".to_string(),
            Key::c => "c".to_string(),
            Key::b => "b".to_string(),
            Key::z => "z".to_string(),
            Key::k => "k".to_string(),
            Key::a => "a".to_string(),
            Key::t => "t".to_string(),
            Key::r => "r".to_string(),
            Key::m => "m".to_string(),
        }
    }
}

impl FromStr for Key {
    type Err = Error;
    fn from_str(s: &str) -> Result<Key, Error> {
        match s.to_lowercase().as_ref() {
            "v" => Ok(Key::v),
            "o" => Ok(Key::o),
            "s" => Ok(Key::s),
            "i" => Ok(Key::i),
            "u" => Ok(Key::u),
            "e" => Ok(Key::e),
            "p" => Ok(Key::p),
            "c" => Ok(Key::c),
            "b" => Ok(Key::b),
            "z" => Ok(Key::z),
            "k" => Ok(Key::k),
            "a" => Ok(Key::a),
            "t" => Ok(Key::t),
            "r" => Ok(Key::r),
            "m" => Ok(Key::m),
            _   => Err(Error::Key)
        }
    }
}
