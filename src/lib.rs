extern crate regex;

use std::fmt;
use std::str::FromStr;
use std::string::ToString;
use std::collections::BTreeMap;

use regex::Regex;

mod key;
mod value;
mod error;

pub use key::Key;
pub use value::Value;
use error::Error;

pub static MEDIA_TYPE: &'static str = "application/sdp";
pub static SDP_DEMO  : &'static str = "v=0
o=mozilla...THIS_IS_SDPARTA-46.0.1 5381835512098962904 0 IN IP4 0.0.0.0
s=-
t=0 0
a=fingerprint:sha-256 A4:96:14:F7:93:2B:BD:A3:D0:F5:F9:1A:46:BD:91:9B:EB:77:12:6E:C7:58:25:45:BB:40:49:03:9F:46:99:10
a=group:BUNDLE sdparta_0 sdparta_1
a=ice-options:trickle
a=msid-semantic:WMS *
m=audio 9 UDP/TLS/RTP/SAVPF 109 9 0 8
c=IN IP4 0.0.0.0
a=sendrecv
a=extmap:1 urn:ietf:params:rtp-hdrext:ssrc-audio-level
a=fmtp:109 maxplaybackrate=48000;stereo=1
a=ice-pwd:2974489cf3e430f30d355d3c99518041
a=ice-ufrag:a8ab487e
a=mid:sdparta_0
a=msid:{91781776-0364-e749-b472-dae4dacaf595} {80b8e956-15b9-7c4d-944a-0f87a78fbbcf}
a=rtcp-mux
a=rtpmap:109 opus/48000/2
a=rtpmap:9 G722/8000/1
a=rtpmap:0 PCMU/8000
a=rtpmap:8 PCMA/8000
a=setup:actpass
a=ssrc:1677787430 cname:{77117ace-c218-7c4e-a4b6-6573734c32cb}
m=video 9 UDP/TLS/RTP/SAVPF 120 126 97
c=IN IP4 0.0.0.0
a=recvonly
a=fmtp:120 max-fs=12288;max-fr=60
a=fmtp:126 profile-level-id=42e01f;level-asymmetry-allowed=1;packetization-mode=1
a=fmtp:97 profile-level-id=42e01f;level-asymmetry-allowed=1
a=ice-pwd:2974489cf3e430f30d355d3c99518041
a=ice-ufrag:a8ab487e
a=mid:sdparta_1
a=rtcp-fb:120 nack
a=rtcp-fb:120 nack pli
a=rtcp-fb:120 ccm fir
a=rtcp-fb:126 nack
a=rtcp-fb:126 nack pli
a=rtcp-fb:126 ccm fir
a=rtcp-fb:97 nack
a=rtcp-fb:97 nack pli
a=rtcp-fb:97 ccm fir
a=rtcp-mux
a=rtpmap:120 VP8/90000
a=rtpmap:126 H264/90000
a=rtpmap:97 H264/90000
a=setup:actpass
a=ssrc:157736891 cname:{77117ace-c218-7c4e-a4b6-6573734c32cb}";


// The free text string SHOULD be in the ISO-10646 character set with UTF-8 encoding, 
// or alternatively in ISO-8859-1 
// or other encodings if the appropriate session-level "a=charset" attribute is set.

#[derive(Debug, Clone)]
pub struct SessionDescription {
    lines: BTreeMap<Key, Value>
}

impl SessionDescription {
    pub fn new() -> SessionDescription {
        SessionDescription { lines: BTreeMap::new() }
    }
    pub fn get(&self, k: &Key) -> Option<&Value> {
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
            let line = k.to_string() + "=" + v.to_string().as_ref() + "\r\n";
            s.push_str( line.as_ref() );
        }
        s
    }
}

impl FromStr for SessionDescription {
    type Err = Error;
    fn from_str(sdp: &str) -> Result<SessionDescription, Error> {
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
                    Ok(key) => {
                        match Value::from_str(&sdp_value, key.clone()) {
                            Some(value) => {
                                // println!("Success: {:?}", value);
                                session_description.insert(key.clone(), value.clone());
                            },
                            None        => {
                                println!("Invalid Line: {}", &line);
                            }
                        };
                    },
                    Err(_)  => {
                        println!("Invalid Line: {}", &line);
                    }
                };
            }
        }
        Ok(session_description)
    }
}



// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let sdp_string = SDP_DEMO;
//         let s = SessionDescription::from_str(sdp_string);
//         println!("{:?}", s );
//     }
// }

fn main (){
        let sdp_string = SDP_DEMO;
        println!("{}", sdp_string);
        println!("=============================================");
        let s = SessionDescription::from_str(sdp_string);
        match s {
            Ok(s)  => {
                println!("{}", s.to_string() );
            },
            Err(_) => {
                println!("parse error.");
            }
        }
        
    }