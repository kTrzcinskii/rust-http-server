use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::{
    error::ServerError,
    response::{send_response, send_response_to_echo, send_response_to_user_agent, ServerResponse},
};

pub fn handle_request(mut stream: TcpStream) -> Result<(), ServerError> {
    let buf_read = BufReader::new(&mut stream);

    // Remember that its spliiting even on "\n" only
    // So we have to options
    // 1) Remember about it (as it only matter in body pare i guess) and join those lines toghether
    // 2) Rewrite it and manually split on "\r\n"
    // For now 1) is used
    let mut requests_segments_iter = buf_read.lines();

    let request_line = requests_segments_iter
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

    let headers_lines: Vec<String> = requests_segments_iter
        .take_while(|line| match line {
            Ok(ref l) => !l.is_empty(),
            Err(_) => false,
        })
        .filter_map(Result::ok)
        .collect();
    match request_path {
        echo_path if echo_path.starts_with("/echo/") => send_response_to_echo(stream, echo_path)?,
        "/user-agent" => send_response_to_user_agent(stream, headers_lines)?,
        "/" => send_response(stream, ServerResponse::Ok, vec![], "")?,
        _ => send_response(stream, ServerResponse::NotFound, vec![], "")?,
    }

    Ok(())
}
