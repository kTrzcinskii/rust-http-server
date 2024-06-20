use std::sync::Arc;

use http_server_starter_rust::{config::Config, request::handle_request};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let config = Config::load();
    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
    let config_arc = Arc::new(config);
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let cloned_arc = Arc::clone(&config_arc);
        tokio::spawn(async move {
            handle_request(stream, cloned_arc)
                .await
                .expect("should not fail for now");
        });
    }
}
