use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

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
        "/" => send_response(stream, ServerResponse::Ok)?,
        _ => send_response(stream, ServerResponse::NotFound)?,
    }

    Ok(())
}

fn send_response(mut stream: TcpStream, response: ServerResponse) -> Result<(), ServerError> {
    let status_line = response.get_status_line();
    let response_message = format!("{status_line}\r\n\r\n");
    stream
        .write_all(response_message.as_bytes())
        .map_err(|_| ServerError::WriteResponseError)?;
    Ok(())
}
