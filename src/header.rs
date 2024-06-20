use core::fmt;
use std::str::FromStr;

use itertools::Itertools;

use crate::error::ServerError;

pub struct Header {
    pub key: String,
    pub value: String,
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

impl FromStr for Header {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let key = parts
            .next()
            .ok_or(ServerError::IncorrectHeaderError)?
            .to_string();
        let value = parts
            .next()
            .ok_or(ServerError::IncorrectHeaderError)?
            .to_string();
        Ok(Header { key, value })
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
