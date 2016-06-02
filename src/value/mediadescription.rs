

#[derive(Debug)]
pub struct MediaDescription {
    // Media description, if present
    m: String, // media name and transport address
    i: Option<String>, // media title
    c: Option<ConnectionInfomation>, // connection information -- optional if included at session level
    b: Option<usize>,  // zero or more bandwidth information lines
    k: Option<String>, // encryption key
    a: Option<Attrs>,  // zero or more session attribute lines
}