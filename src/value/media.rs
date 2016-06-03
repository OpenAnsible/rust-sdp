extern crate regex;


use std::fmt;
use std::str::FromStr;
use std::string::ToString;

use regex::Regex;

use error::Error;


/**
    
    media: Media Type           // https://tools.ietf.org/html/rfc4566#section-8.2.1
    port : RTP Port             // number of ports : port+1
                                
    proto: Transport Protocols  // https://tools.ietf.org/html/rfc4566#section-8.2.2
                                // UDP/TLS/RTP/AVP/SAVP/SAVPF
                                  *  udp: denotes an unspecified protocol running over UDP.
                                  *  RTP/AVP: denotes RTP [19] used under the RTP Profile for Audio
                                     and Video Conferences with Minimal Control [20] running over
                                     UDP.
                                  *  RTP/SAVP: denotes the Secure Real-time Transport Protocol [23]
                                     running over UDP.
    fmt  :                      is a media format description.  

                                If the <proto> sub-field is "RTP/AVP" or "RTP/SAVP" the <fmt>
                                sub-fields contain RTP payload type numbers.  When a list of
                                payload type numbers is given, this implies that all of these
                                payload formats MAY be used in the session, but the first of these
                                formats SHOULD be used as the default format for the session.  For
                                dynamic payload type assignments the "a=rtpmap:" attribute (see
                                Section 6) SHOULD be used to map from an RTP payload type number
                                to a media encoding name that identifies the payload format.  The
                                "a=fmtp:"  attribute MAY be used to specify format parameters (see
                                Section 6).

                                If the <proto> sub-field is "udp" the <fmt> sub-fields MUST
                                reference a media type describing the format under the "audio",
                                "video", "text", "application", or "message" top-level media
                                types.  The media type registration SHOULD define the packet
                                format for use with UDP transport.

                                For media using other transport protocols, the <fmt> field is
                                protocol specific.  Rules for interpretation of the <fmt> sub-
                                field MUST be defined when registering new protocols (see Section
                                8.2.2).

RTP Profile Names:
    http://www.iana.org/assignments/rtp-parameters/rtp-parameters.xhtml#rtp-parameters-7

    RTP/AVP     [RFC3550]
    RTP/SAVP    [RFC3711]
    RTP/AVPF    [RFC4585]
    RTP/SAVPF   [RFC5124]


RTP/AVP:   RTP audio video profile
           https://en.wikipedia.org/wiki/RTP_audio_video_profile
           https://tools.ietf.org/html/rfc5391

RTP/SAVPF: Extended Secure RTP Profile for Real-time Transport Control Protocol (RTCP)-Based Feedback (RTP/SAVPF)
           https://tools.ietf.org/html/rfc5124
**/

#[derive(Debug)]
pub enum Media {
    // https://tools.ietf.org/html/rfc4566#section-8.2.1
    Video,
    Audio,
    Text,
    Message,
    Application
}



#[derive(Debug)]
pub enum Protocol {
    Rtp(RtpProfile),
    Tls,
    Udp
    // Unregistered(String)
}

// 协议特征
#[derive(Debug)]
pub enum RtpProfile {
    Avp,
    Savp,
    Avpf,
    Savpf
}

#[derive(Debug)]
pub struct MediaInfo {
    media: Media,
    port : usize,
    port_num : Option<usize>,
    protocols: Vec<Protocol>,
    fmt      : usize,
    extension: Option<String>  //  9 0 8
}


impl ToString for Media {
    fn to_string(&self) -> String {
        match *self {
            Media::Video   => "video".to_string(),
            Media::Audio   => "audio".to_string(),
            Media::Text    => "text".to_string(),
            Media::Message => "message".to_string(),
            Media::Application => "application".to_string()
        }
    }
}


impl FromStr for Media {
    type Err = Error;
    fn from_str(s: &str) -> Result<Media, Error> {
        match s.to_lowercase() {
            "video" => Media::Video,
            "audio" => Media::Audio,
            "text"  => Media::Text,
            "message"     => Media::Message,
            "application" => Media::Application,
            _             => Err(Error::MediaInfo)
        }
    }
}


impl ToString for MediaInfo {
    fn to_string(&self) -> String {
        self.media.to_string() + self.port.to_string().as_ref() + match self.port_num {
            Some(pn) => "/".to_string() + pn.to_string().as_ref(),
            None     => ""
        } //+ 
        // TODO: Join Protocols, FMT, Extension
    }
}


impl FromStr for MediaInfo {
    type Err = Error;
    fn from_str(s: &str) -> Result<MediaInfo, Error> {
        // m=<media> <port> <proto> <fmt>
        // m=audio 9 UDP/TLS/RTP/SAVPF 109 9 0 8

        // m=<media> <port>/<number of ports> <proto> <fmt>
        // m=video 49170/2 RTP/AVP 31
        let re  = match Regex::new(r"(\S+)\s(\d+)(/)(\d+)\s(\S+)\s(\d+)\s(\S+)") {
            Ok(re) => re,
            Err(e) => {
                println!("[Regex] {:?}", e);
                return Err(Error::Origin);
            }
        };
        let cap = re.captures(s).unwrap();
        // TODO
        Err(Error::MediaInfo)
    }
}


