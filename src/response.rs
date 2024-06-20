use std::path::Path;

use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{
    error::ServerError,
    header::{Header, RequestHeaderType, ResponseHeaderType},
    request::RequestContent,
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

pub async fn send_response(
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
        .await
        .map_err(|_| ServerError::WriteResponseError)?;
    Ok(())
}

pub async fn send_response_to_echo(
    stream: TcpStream,
    content: RequestContent,
) -> Result<(), ServerError> {
    const ECHO_LEN: usize = 6; // "/echo/"
    let message = &content.path[ECHO_LEN..];
    let mut headers: Vec<Header> = Vec::new();
    headers.push(Header::new(ResponseHeaderType::ContentType, "text/plain"));
    headers.push(Header::new(
        ResponseHeaderType::ContentLength,
        &message.len().to_string(),
    ));

    send_response(stream, ServerResponse::Ok, headers, message).await
}

pub async fn send_response_to_user_agent(
    stream: TcpStream,
    content: RequestContent,
) -> Result<(), ServerError> {
    let message = content
        .headers
        .iter()
        .find(|h| h.key == RequestHeaderType::UserAgent.to_string())
        .and_then(|h| Option::Some(h.value.as_str()))
        .unwrap_or("");

    let mut headers: Vec<Header> = Vec::new();
    headers.push(Header::new(ResponseHeaderType::ContentType, "text/plain"));
    headers.push(Header::new(
        ResponseHeaderType::ContentLength,
        &message.len().to_string(),
    ));
    send_response(stream, ServerResponse::Ok, headers, &message).await
}

pub async fn send_response_to_files(
    stream: TcpStream,
    content: RequestContent,
    dir_path: &str,
) -> Result<(), ServerError> {
    // it shoudl be some const in `header.rs`
    const FILES_PATH_PREFIX_LEN: usize = "/files/".len();

    let file_name = &content.path[FILES_PATH_PREFIX_LEN..];

    let path = Path::new(dir_path).join(file_name);
    if fs::metadata(path.clone()).await.is_err() {
        return send_response(stream, ServerResponse::NotFound, vec![], "").await;
    }

    let mut file = File::open(path)
        .await
        .map_err(|_| ServerError::FileReadingError)?;

    let mut file_buf = vec![];
    file.read_to_end(&mut file_buf)
        .await
        .map_err(|_| ServerError::FileReadingError)?;

    let mut headers: Vec<Header> = Vec::new();
    headers.push(Header::new(
        ResponseHeaderType::ContentType,
        "application/octet-stream",
    ));
    headers.push(Header::new(
        ResponseHeaderType::ContentLength,
        &file_buf.len().to_string(),
    ));

    send_response(
        stream,
        ServerResponse::Ok,
        headers,
        &String::from_utf8(file_buf).expect("For now just assuming that file is utf8"),
    )
    .await
}

pub async fn send_response_to_post_file(
    stream: TcpStream,
    content: RequestContent,
    dir_path: &str,
) -> Result<(), ServerError> {
    todo!()
}
