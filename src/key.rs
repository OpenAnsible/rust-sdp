use std::fmt;
use std::str::FromStr;
use std::string::ToString;

// SDP Key
#[derive(Debug, Clone)]
pub enum Key {
    // Session Description
    pub v,
    pub o,
    pub s,
    pub i,
    pub u,
    pub e,
    pub p,
    pub c,
    pub b,
    pub z,
    pub k,
    pub a,
    // Time Description
    pub t,
    pub r,
    // Media Description
    pub m,
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
    fn from_str(s: &str) -> Option<Key> {
        match s.to_lowercase().as_ref() {
            "v" => Some(Key::v),
            "o" => Some(Key::o),
            "s" => Some(Key::s),
            "i" => Some(Key::i),
            "u" => Some(Key::u),
            "e" => Some(Key::e),
            "p" => Some(Key::p),
            "c" => Some(Key::c),
            "b" => Some(Key::b),
            "z" => Some(Key::z),
            "k" => Some(Key::k),
            "a" => Some(Key::a),
            "t" => Some(Key::t),
            "r" => Some(Key::r),
            "m" => Some(Key::m),
            _   => None
        }
    }
}
