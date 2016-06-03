extern crate regex;

use std::fmt;
use std::str::FromStr;
use std::string::ToString;

use regex::Regex;

use error::Error;

// https://tools.ietf.org/html/rfc4566#section-5.8

#[derive(Debug, Clone)]
pub struct Bandwidth {
    pub prefix: String,
    pub ct    : String,
    pub size  : usize
}


impl ToString for Bandwidth {
    fn to_string(&self) -> String {
        self.prefix.clone() + "-" + self.ct.as_ref() + ":" + self.size.to_string().as_ref()
    }
}

impl FromStr for Bandwidth {
    type Err = Error;
    fn from_str(s: &str) -> Result<Bandwidth, Error> {
        // b=<bwtype>:<bandwidth>
        // b=X-YZ:128
        // Use of the "X-" prefix is NOT RECOMMENDED: instead new modifiers
        // SHOULD be registered with IANA in the standard namespace. 
        let re  = match Regex::new(r"(\S+)-(\S+):(\d+)") {
            Ok(re) => re,
            Err(e) => {
                println!("[Regex] {:?}", e);
                return Err(Error::Bandwidth);
            }
        };
        let cap = re.captures(s).unwrap();
        let prefix = cap.at(0);
        let ct     = cap.at(1);

        let size: usize = match cap.at(2) {
            Some(size) => {
                match usize::from_str(size){
                    Ok(size) => size,
                    Err(_)   => {
                        println!("ERROR: bandwidth.size parse fail. {:?}", size);
                        return Err(Error::Bandwidth);
                    }
                }
            },
            None       => {
                println!("ERROR: bandwidth.size parse fail. ");
                return Err(Error::Bandwidth);
            }
        };
        
        if prefix.is_some() && ct.is_some() {
            Ok(Bandwidth {
                prefix: prefix.unwrap().to_string(),
                ct    : ct.unwrap().to_string(),
                size  : size
            })
        } else {
            return Err(Error::Bandwidth);
        }
    }
}