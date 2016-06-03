use std::fmt;
use std::str::FromStr;
use std::string::ToString;

pub enum Error {
    Key,
    Value,
    ProtocolVersion,
    Origin,
    SessionVersion,
    SessionName,
    SessionId,
    AddrType,
    NetType,
    IpAddress,
    Bandwidth,
    Uri,
    ConnectionInfo,
    MediaInfo
}

