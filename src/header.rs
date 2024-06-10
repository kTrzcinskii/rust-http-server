use core::fmt;

use itertools::Itertools;

pub struct Header {
    key: String,
    value: String,
}

pub enum ResponseHeaderType {
    ContentType,
    ContentLength,
}

pub enum RequestHeaderType {
    Host,
    UserAgent,
    Accept,
}

impl fmt::Display for ResponseHeaderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseHeaderType::ContentType => write!(f, "Content-Type"),
            ResponseHeaderType::ContentLength => write!(f, "Content-Length"),
        }
    }
}

impl fmt::Display for RequestHeaderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestHeaderType::Host => write!(f, "Host"),
            RequestHeaderType::UserAgent => write!(f, "User-Agent"),
            RequestHeaderType::Accept => write!(f, "Accept"),
        }
    }
}

impl Header {
    pub fn new<T: fmt::Display>(header_type: T, header_value: &str) -> Self {
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
