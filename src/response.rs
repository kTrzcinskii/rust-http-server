use std::{io::Write, net::TcpStream};

use crate::{
    error::ServerError,
    header::{Header, HeaderType},
};

pub enum ServerResponse {
    Ok,
    NotFound,
}

impl ServerResponse {
    pub fn get_status_line(&self) -> &'static str {
        match self {
            ServerResponse::Ok => "HTTP/1.1 200 OK",
            ServerResponse::NotFound => "HTTP/1.1 404 Not Found",
        }
    }
}

pub fn send_response(
    mut stream: TcpStream,
    response: ServerResponse,
    headers: Vec<Header>,
    body: &str,
) -> Result<(), ServerError> {
    let status_line = response.get_status_line();
    let headers_str = Header::combine_headers(headers);
    let response_message = format!("{status_line}\r\n{headers_str}\r\n{body}");
    stream
        .write_all(response_message.as_bytes())
        .map_err(|_| ServerError::WriteResponseError)?;
    Ok(())
}

pub fn send_response_to_echo(stream: TcpStream, echo_path: &str) -> Result<(), ServerError> {
    const ECHO_LEN: usize = 6; // "/echo/"
    let message = &echo_path[ECHO_LEN..];
    let mut headers: Vec<Header> = Vec::new();
    headers.push(Header::new(HeaderType::ContentType, "text/plain"));
    headers.push(Header::new(
        HeaderType::ContentLength,
        &message.len().to_string(),
    ));
    send_response(stream, ServerResponse::Ok, headers, message)
}