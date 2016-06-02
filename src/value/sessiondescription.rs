

#[derive(Debug)]
pub struct SessionDescription {
    v: ProtocolVersion,  // protocol version number, currently only 0, For example: v=0
    o: String,           // originator and session identifier : username, id, version number, network address
    s: String,           // session name : mandatory with at least one UTF-8-encoded character
    i: Option<String>,   // session title or short information
    u: Option<String>,   // URI of description
    e: Option<String>,   // Email-address, For example: e=j.doe@example.com (Jane Doe) | e=Jane Doe <j.doe@example.com>
    p: Option<String>,   // Phone-number, For example: p=+1 617 555-6011
    c: Option<ConnectionInfomation>, // connection information—not required if included in all media
    b: Option<usize>,    // zero or more bandwidth information lines
    // One or more time descriptions ("t=" and "r=" lines; see below)
    z: Option<String>,   // time zone adjustments
    k: Option<String>,   // encryption key
    a: Option<Attrs>,    // zero or more session attribute lines
    // Zero or more media descriptions
}