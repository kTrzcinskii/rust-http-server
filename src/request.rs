use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::{
    error::ServerError,
    response::{send_response, send_response_to_echo, ServerResponse},
};

pub fn handle_request(mut stream: TcpStream) -> Result<(), ServerError> {
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
        "/" => send_response(stream, ServerResponse::Ok, vec![], "")?,
        _ => send_response(stream, ServerResponse::NotFound, vec![], "")?,
    }

    Ok(())
}
