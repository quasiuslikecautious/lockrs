use std::net::TcpListener;

use lockrs_server::run;

/// rfc: https://www.rfc-editor.org/rfc/rfc6749#section-4
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").expect("Failed to bind to port");
    let addr = listener.local_addr().unwrap();
    tracing::info!("listening at {}", addr);
    println!("listening at {}", addr);
    run(listener, None).await.expect("Failed to bind address.");
}
