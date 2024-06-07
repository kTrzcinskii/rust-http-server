// TODO: split it into several files

use core::fmt;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use itertools::Itertools;

#[derive(Debug)]
enum ServerError {
    IncorrectRequestFormatError,
    WriteResponseError,
}

enum ServerResponse {
    Ok,
    NotFound,
}

impl ServerResponse {
    fn get_status_line(&self) -> &'static str {
        match self {
            ServerResponse::Ok => "HTTP/1.1 200 OK",
            ServerResponse::NotFound => "HTTP/1.1 404 Not Found",
        }
    }
}

struct Header {
    key: String,
    value: String,
}

enum HeaderType {
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
    fn new(header_type: HeaderType, header_value: &str) -> Self {
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
    fn combine_headers(headers: Vec<Header>) -> String {
        let mut response = headers
            .into_iter()
            .map(|header| header.to_string())
            .join("\r\n");
        response.push_str("\r\n");
        response
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    server_loop(listener);
}

fn server_loop(listener: TcpListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // TODO: better error handling here
                handle_request(stream).expect("should not fail for now");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_request(mut stream: TcpStream) -> Result<(), ServerError> {
    let buf_read = BufReader::new(&mut stream);
    let request_line = buf_read
        .lines()
        .next()
        .ok_or_else(|| ServerError::IncorrectRequestFormatError)?
        .map_err(|_| ServerError::IncorrectRequestFormatError)?;

    let mut request_line_iterator = request_line.split(" ");
    let _request_method = request_line_iterator
        .next()
        .ok_or_else(|| ServerError::IncorrectRequestFormatError)?;
    let request_path = request_line_iterator
        .next()
        .ok_or_else(|| ServerError::IncorrectRequestFormatError)?;

    match request_path {
        echo_path if echo_path.starts_with("/echo/") => send_response_to_echo(stream, echo_path)?,
        "/" => send_response(stream, ServerResponse::Ok, vec![])?,
        _ => send_response(stream, ServerResponse::NotFound, vec![])?,
    }

    Ok(())
}

fn send_response(
    mut stream: TcpStream,
    response: ServerResponse,
    headers: Vec<Header>,
) -> Result<(), ServerError> {
    let status_line = response.get_status_line();
    let headers_str = Header::combine_headers(headers);
    let response_message = format!("{status_line}\r\n{headers_str}\r\n");
    stream
        .write_all(response_message.as_bytes())
        .map_err(|_| ServerError::WriteResponseError)?;
    Ok(())
}

fn send_response_to_echo(stream: TcpStream, echo_path: &str) -> Result<(), ServerError> {
    const ECHO_LEN: usize = 6; // "/echo/"
    let message = &echo_path[ECHO_LEN..];
    let mut headers: Vec<Header> = Vec::new();
    headers.push(Header::new(HeaderType::ContentType, "text/plain"));
    headers.push(Header::new(
        HeaderType::ContentLength,
        &message.len().to_string(),
    ));
    send_response(stream, ServerResponse::Ok, headers)
}
