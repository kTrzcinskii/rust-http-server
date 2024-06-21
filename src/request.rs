use std::{str::FromStr, sync::Arc};

use tokio::{
    io::{AsyncReadExt, BufReader},
    net::TcpStream,
    time,
};

use crate::{
    config::Config,
    error::ServerError,
    header::Header,
    response::{
        send_response, send_response_to_echo, send_response_to_files, send_response_to_post_file,
        send_response_to_user_agent, ServerResponse,
    },
};

#[derive(PartialEq)]
pub enum RequestMethod {
    Get,
    Post,
}

impl FromStr for RequestMethod {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(RequestMethod::Get),
            "POST" => Ok(RequestMethod::Post),
            _ => Err(ServerError::IncorrectHttpMethodError),
        }
    }
}

pub struct RequestContent {
    pub path: String,
    pub method: RequestMethod,
    pub headers: Vec<Header>,
    pub body: String,
}

#[derive(PartialEq)]
enum RequestParsignStage {
    RequestLine,
    Headers,
    RequestBody,
}

impl RequestContent {
    pub async fn parse_request(mut buf: BufReader<&mut TcpStream>) -> Result<Self, ServerError> {
        let mut buffer = Vec::new();
        let mut current_stage = RequestParsignStage::RequestLine;

        let mut path = String::new();
        let mut method = RequestMethod::Get;
        let mut headers: Vec<Header> = vec![];

        let read_timeout = time::Duration::from_millis(500);

        loop {
            // we need to fix blocking when there is no more data in tcp stream to read
            // https://users.rust-lang.org/t/how-can-set-a-timeout-when-reading-data-using-tokio-bufreader/39347

            // instead of just throwing error, we should check if its not the timeout error
            // https://stackoverflow.com/questions/33557375/how-do-i-prevent-tcpstream-from-blocking-on-a-read

            let mut temp_buffer: [u8; 1024] = [0; 1024];
            let read_count_res = time::timeout(read_timeout, buf.read(&mut temp_buffer)).await;

            if read_count_res.is_err() {
                // timeout - we assume there is no more data to read
                break;
            }

            let read_count = read_count_res
                .unwrap() // at this point we know its not an error
                .map_err(|_| ServerError::TcpStreamReadingError)?;

            if read_count == 0 {
                break; // all data read
            }

            buffer.extend_from_slice(&temp_buffer[..read_count]);

            if current_stage == RequestParsignStage::RequestBody {
                continue;
            }

            while let Some(index) = buffer.windows(2).position(|p| p == b"\r\n") {
                // get content to "\r\n"
                let part = buffer.drain(..index).collect::<Vec<u8>>();
                // remove "\r\n"
                buffer.drain(..2);
                let part_str = String::from_utf8(part).unwrap();
                match current_stage {
                    RequestParsignStage::RequestLine => {
                        let mut parts = part_str.split(" ");
                        method = RequestMethod::from_str(parts.next().unwrap_or(""))?;

                        path = parts
                            .next()
                            .ok_or(ServerError::IncorrectRequestLineError)?
                            .to_string();

                        current_stage = RequestParsignStage::Headers;
                    }
                    RequestParsignStage::Headers => {
                        // end of headers
                        if part_str.is_empty() {
                            current_stage = RequestParsignStage::RequestBody;
                            break;
                        }
                        let header = Header::from_str(&part_str)?;
                        headers.push(header);
                    }
                    // DEAD CODE
                    RequestParsignStage::RequestBody => {
                        panic!("should never reach it")
                    }
                }
            }
        }

        let body = String::from_utf8(buffer).unwrap();

        Ok(RequestContent {
            path,
            method,
            headers,
            body,
        })
    }
}

pub async fn handle_request(mut stream: TcpStream, config: Arc<Config>) -> Result<(), ServerError> {
    let buf_read = BufReader::new(&mut stream);

    let content = RequestContent::parse_request(buf_read).await?;

    match content.method {
        RequestMethod::Get => match content.path.as_str() {
            echo_path if echo_path.starts_with("/echo/") => {
                send_response_to_echo(stream, content).await?
            }
            files_path if files_path.starts_with("/files/") => {
                send_response_to_files(stream, content, config.get_files_directory()).await?
            }
            "/user-agent" => send_response_to_user_agent(stream, content).await?,
            "/" => send_response(stream, ServerResponse::Ok, vec![], "").await?,
            _ => send_response(stream, ServerResponse::NotFound, vec![], "").await?,
        },
        RequestMethod::Post => match content.path.as_str() {
            files_path if files_path.starts_with("/files/") => {
                send_response_to_post_file(stream, content, config.get_files_directory()).await?
            }
            _ => send_response(stream, ServerResponse::NotFound, vec![], "").await?,
        },
    }

    Ok(())
}
