use std::net::TcpListener;

use http_server_starter_rust::request::handle_request;

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
