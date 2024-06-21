use std::path::Path;

use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{
    config,
    error::ServerError,
    header::{Header, HeaderType},
    request::RequestContent,
    utils,
};

pub enum ServerResponse {
    Ok,
    NotFound,
    Created,
}

impl ServerResponse {
    pub fn get_status_line(&self) -> &'static str {
        match self {
            ServerResponse::Ok => "HTTP/1.1 200 OK",
            ServerResponse::NotFound => "HTTP/1.1 404 Not Found",
            ServerResponse::Created => "HTTP/1.1 201 Created",
        }
    }
}

pub async fn send_response(
    mut stream: TcpStream,
    response: ServerResponse,
    mut headers: Vec<Header>,
    body: &str,
    content: &RequestContent,
) -> Result<(), ServerError> {
    let h = content.headers.iter().find(|h| {
        h.key == HeaderType::AcceptEncoding.to_string()
            && config::ACCEPTED_ENCODINGS.contains(&h.value.as_str())
    });

    if let Some(encoding_type) = h {
        let encoding_header =
            Header::new(HeaderType::ContentEncoding, encoding_type.value.as_str());
        headers.push(encoding_header);
        // TODO: encode body here
    }

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
    headers.push(Header::new(HeaderType::ContentType, "text/plain"));
    headers.push(Header::new(
        HeaderType::ContentLength,
        &message.len().to_string(),
    ));

    send_response(stream, ServerResponse::Ok, headers, message, &content).await
}

pub async fn send_response_to_user_agent(
    stream: TcpStream,
    content: RequestContent,
) -> Result<(), ServerError> {
    let message = content
        .headers
        .iter()
        .find(|h| h.key == HeaderType::UserAgent.to_string())
        .and_then(|h| Option::Some(h.value.as_str()))
        .unwrap_or("");

    let mut headers: Vec<Header> = Vec::new();
    headers.push(Header::new(HeaderType::ContentType, "text/plain"));
    headers.push(Header::new(
        HeaderType::ContentLength,
        &message.len().to_string(),
    ));
    send_response(stream, ServerResponse::Ok, headers, &message, &content).await
}

pub async fn send_response_to_files(
    stream: TcpStream,
    content: RequestContent,
    dir_path: &str,
) -> Result<(), ServerError> {
    let file_name = utils::extract_filename_from_request_path(&content.path)?;

    let path = Path::new(dir_path).join(file_name);
    if fs::metadata(path.clone()).await.is_err() {
        return send_response(stream, ServerResponse::NotFound, vec![], "", &content).await;
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
        HeaderType::ContentType,
        "application/octet-stream",
    ));
    headers.push(Header::new(
        HeaderType::ContentLength,
        &file_buf.len().to_string(),
    ));

    send_response(
        stream,
        ServerResponse::Ok,
        headers,
        &String::from_utf8(file_buf).expect("For now just assuming that file is utf8"),
        &content,
    )
    .await
}

pub async fn send_response_to_post_file(
    stream: TcpStream,
    content: RequestContent,
    dir_path: &str,
) -> Result<(), ServerError> {
    let file_name = utils::extract_filename_from_request_path(&content.path)?;
    let file_path = Path::new(dir_path).join(file_name);
    let mut file = File::create(file_path)
        .await
        .map_err(|_| ServerError::FileCreatingError)?;
    file.write_all(content.body.as_bytes())
        .await
        .map_err(|_| ServerError::FileWritingError)?;
    send_response(stream, ServerResponse::Created, vec![], "", &content).await
}
