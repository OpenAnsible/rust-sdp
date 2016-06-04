

// https://tools.ietf.org/html/rfc4566#section-5.13

// a=<attribute>
// a=<attribute>:<value>

#[derive(Debug, Clone)]
pub enum AttrFlag {
    RecvOnly,
    SendRecv,
    FmtP,
    RtcpMux,
    RtcpFb,
    RtpMap,
    Extmap,
    IcePwd,
    IceUfrag,
    Setup,
    Ssrc,   // SSRC
    Mid,
    Msid

}

#[derive(Debug, Clone)]
pub struct Attr {
    flag : AttrFlag,
    value: String
}

