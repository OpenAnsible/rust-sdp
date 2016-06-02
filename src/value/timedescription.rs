
#[derive(Debug)]
pub struct TimeDescription {
    t: usize,          // time the session is active
    r: Option<usize>   // zero or more repeat times
}