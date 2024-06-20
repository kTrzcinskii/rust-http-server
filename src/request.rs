use std::sync::Arc;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpStream,
};

use crate::{
    config::Config,
    error::ServerError,
    response::{
        send_response, send_response_to_echo, send_response_to_files, send_response_to_user_agent,
        ServerResponse,
    },
};

pub async fn handle_request(mut stream: TcpStream, config: Arc<Config>) -> Result<(), ServerError> {
    let buf_read = BufReader::new(&mut stream);

    // Remember that its spliiting even on "\n" only
    // So we have to options
    // 1) Remember about it (as it only matter in body pare i guess) and join those lines toghether
    // 2) Rewrite it and manually split on "\r\n"
    // For now 1) is used
    let mut requests_segments_iter = buf_read.lines();

    let request_line = requests_segments_iter
        .next_line()
        .await
        .map_err(|_| ServerError::IncorrectRequestFormatError)?
        .ok_or_else(|| ServerError::IncorrectRequestFormatError)?;

    let mut request_line_iterator = request_line.split(" ");
    let _request_method = request_line_iterator
        .next()
        .ok_or_else(|| ServerError::IncorrectRequestFormatError)?;
    let request_path = request_line_iterator
        .next()
        .ok_or_else(|| ServerError::IncorrectRequestFormatError)?;

    let mut headers_lines: Vec<String> = Vec::new();
    while let Some(line) = requests_segments_iter
        .next_line()
        .await
        .map_err(|_| ServerError::IncorrectRequestFormatError)?
    {
        if line.is_empty() {
            break;
        }
        headers_lines.push(line);
    }

    match request_path {
        echo_path if echo_path.starts_with("/echo/") => {
            send_response_to_echo(stream, echo_path).await?
        }
        files_path if files_path.starts_with("/files/") => {
            send_response_to_files(stream, files_path, config.get_files_directory()).await?
        }
        "/user-agent" => send_response_to_user_agent(stream, headers_lines).await?,
        "/" => send_response(stream, ServerResponse::Ok, vec![], "").await?,
        _ => send_response(stream, ServerResponse::NotFound, vec![], "").await?,
    }

    Ok(())
}
