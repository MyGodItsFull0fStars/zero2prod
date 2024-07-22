use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Returns a io::Error if we fail to bind the address
    // otherwise call .await on our server
    let listener =
        TcpListener::bind("127.0.0.1:8000").expect("Expected to bind port for TcpListener");
    run(listener)?.await
}
