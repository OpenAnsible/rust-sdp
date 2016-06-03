
// RFC
// https://tools.ietf.org/html/rfc822
// https://tools.ietf.org/html/rfc2822
// https://tools.ietf.org/html/rfc5322

// Python 
// j.doe@example.com (Jane Doe)
// re.compile(r"(?P<email>[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)\s\((.*?)\)")

// Jane Doe <j.doe@example.com>
// re.compile(r"(?P<uname>.*?)\s<(?P<email>[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)>")

// SDP Email&Phone Line
// https://tools.ietf.org/html/rfc4566#section-5.6
// Phone
// +1 617 555-6011

#[derive(Debug)]
pub struct Email {

}

#[derive(Debug)]
pub struct Phone {

}