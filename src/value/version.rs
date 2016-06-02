use std::fmt;
use std::str::FromStr;
use std::string::ToString;

#[derive(Clone, Debug)]
pub enum ProtocolVersion {
    pub V0
}

#[derive(Clone, Debug)]
pub enum SessionVersion {
    pub V0
}

impl ProtocolVersion {
    pub fn to_usize(&self) -> usize {
        match *self {
            ProtocolVersion::V0 => 0
        }
    }
    pub fn from_usize(n: usize) -> Option<ProtocolVersion> {
        match n {
            0 => Some(ProtocolVersion::V0),
            _ => None
        }
    }
}

impl FromStr for ProtocolVersion {
    fn from_str(pv: &str) -> Option<ProtocolVersion> {
        match usize::from_str(pv) {
            Ok(i)  => ProtocolVersion::from_usize(i),
            Err(_) => None
        }
    }
}

impl SessionVersion {
    pub fn to_usize(&self) -> usize {
        match *self {
            SessionVersion::V0 => 0
        }
    }
    pub fn from_usize(n: usize) -> Option<SessionVersion> {
        match n {
            0 => Some(SessionVersion::V0),
            _ => None
        }
    }
}
impl FromStr for SessionVersion {
    fn from_str(sv: &str) -> Option<SessionVersion> {
        match usize::from_str(sv) {
            Ok(i)  => SessionVersion::from_usize(i),
            Err(_) => None
        }
    }
}