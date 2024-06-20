use http_server_starter_rust::request::handle_request;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_request(stream)
                .await
                .expect("should not fail for now");
        });
    }
}
