use core::fmt;

use itertools::Itertools;

pub struct Header {
    key: String,
    value: String,
}

pub enum HeaderType {
    ContentType,
    ContentLength,
}

impl fmt::Display for HeaderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeaderType::ContentType => write!(f, "Content-Type"),
            HeaderType::ContentLength => write!(f, "Content-Length"),
        }
    }
}

impl Header {
    pub fn new(header_type: HeaderType, header_value: &str) -> Self {
        Header {
            key: header_type.to_string(),
            value: String::from(header_value),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}

impl Header {
    pub fn combine_headers(headers: Vec<Header>) -> String {
        let mut response = headers
            .into_iter()
            .map(|header| header.to_string())
            .join("\r\n");
        response.push_str("\r\n");
        response
    }
}
