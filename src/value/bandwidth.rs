use std::fmt;
use std::str::FromStr;
use std::string::ToString;
// https://tools.ietf.org/html/rfc4566#section-5.8

#[derive(Debug, Clone)]
pub struct Bandwidth {
    // b=<bwtype>:<bandwidth>
    // b=X-YZ:128
    // Use of the "X-" prefix is NOT RECOMMENDED: instead new modifiers
    // SHOULD be registered with IANA in the standard namespace. 
    pub ct: String,
    pub bandwidth: usize
}

impl Bandwidth {
    pub fn new (ct: String, bandwidth: usize) -> Bandwidth {
        Bandwidth { ct: ct, bandwidth: bandwidth }
    }
}

impl ToString for Bandwidth {
    fn to_string(&self) -> String {
        let bw = "b=".to_string() + self.ct.as_ref() + ":" + self.bandwidth as str;
        bw
    }
}

impl FromStr for NetType {
    fn from_str(s: &str) -> Result<Bandwidth, &'static str> {
        let parts = s.splitn(2, '=').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err("parse fail.");
        }
        let key   = parts[0];
        let value = parts[1];

        match key {
            "v" => {
                if let Ok(v) = FromStr::from_str(value) {
                    println!("v => {}", v);
                    Some(SdpLine::ProtocolVersion(v))
                } else {
                    None
                }
            },
            "o" => {
                if let Some(o) = parse_origin(value) {
                    //println!("o => {}", o);
                    Some(SdpLine::Origin(o))
                } else {
                    None
                }
            },
            _ => None
        }
    }
}