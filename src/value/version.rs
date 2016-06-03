use std::fmt;
use std::str::FromStr;
use std::string::ToString;
use error::Error;


#[derive(Clone, Debug)]
pub enum ProtocolVersion {
    V0
}

#[derive(Clone, Debug)]
pub enum SessionVersion {
    V0
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
    type Err = Error;
    fn from_str(s: &str) -> Result<ProtocolVersion, Error> {
        match usize::from_str(s) {
            Ok(i)  => match ProtocolVersion::from_usize(i) {
                Some(i) => Ok(i),
                None    => return Err(Error::ProtocolVersion)
            },
            Err(_) => Err(Error::ProtocolVersion)
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
    type Err = Error;
    fn from_str(s: &str) -> Result<SessionVersion, Error> {
        match usize::from_str(s) {
            Ok(i)  => match SessionVersion::from_usize(i) {
                Some(i) => Ok(i),
                None    => return Err(Error::SessionVersion)
            },
            Err(_) => Err(Error::SessionVersion)
        }
    }
}


impl ToString for ProtocolVersion {
    fn to_string(&self) -> String {
        match *self {
            ProtocolVersion::V0 => "0".to_string()
        }
    }
}

impl ToString for SessionVersion {
    fn to_string(&self) -> String {
        match *self {
            SessionVersion::V0 => "0".to_string()
        }
    }
}