use std::fmt;
use std::str::FromStr;
use std::string::ToString;
use std::collections::BTreeMap;

mod key;
mod value;

use key::Key;
use value::Value;

pub static MEDIA_TYPE: &'static str = "application/sdp";


// The free text string SHOULD be in the ISO-10646 character set with UTF-8 encoding, 
// or alternatively in ISO-8859-1 
// or other encodings if the appropriate session-level "a=charset" attribute is set.


#[derive(Debug)]
pub struct SessionDescription {
    lines: BTreeMap<Key, Value>
}

impl SessionDescription {
    pub fn new() -> SessionDescription {
        SessionDescription { lines: BTreeMap::new() }
    }
    // pub fn addLine(&mut self, k: &str, v: &str) -> Result<(), &'static str> {
    // }
    pub fn get(&self, k: Key) -> Option<&Value> {
        self.lines.get(k)
    }
    pub fn insert(&mut self, k: Key, v: Value) -> Option<Value> {
        if self.lines.contains_key(&k) {
            self.remove(&k);
        }
        self.lines.insert(k, v)
    }
    pub fn remove(&mut self, k: &Key) -> Option<Value> {
        self.lines.remove(k)
    }
    pub fn keys(&self) -> Vec<Key> {
        self.lines.keys().cloned().collect::<Vec<Key>>()
    }
    pub fn values(&self) -> Vec<Value> {
        self.lines.values().cloned().collect::<Vec<Value>>()
    }
    pub fn len(&self) -> usize {
        self.lines.len()
    }
    pub fn clear(&mut self) {
        self.lines.clear();
    }

}

impl ToString for SessionDescription {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for (k, v) in &self.lines {
            s.push_str( k.to_string() + "=" + v.to_string().as_ref() + "\r\n" )
        }
        s
    }
}

impl FromStr for SessionDescription {
    fn from_str(sdp: &str) -> Result<SessionDescription, &'static str> {
        let mut session_description = SessionDescription::new();

        for line in sdp.lines() {
            let key_value = line.splitn(2, '=').collect::<Vec<&str>>();
            if key_value.len() != 2 {
                // Ignored
                println!("Invalid Line: {}", &line);
            } else {
                let sdp_key   = key_value[0];
                let sdp_value = key_value[1];
                match Key::from_str(sdp_key) {
                    Some(key) => match Value::from_str(&sdp_value, key) {
                        Some(value) => {
                            session_description.insert(key, value);
                        },
                        None        => {
                            println!("Invalid Line: {}", &line);
                        }
                    },
                    None      => {
                        println!("Invalid Line: {}", &line);
                    }
                }
            }
        }
        Ok(session_description)
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
