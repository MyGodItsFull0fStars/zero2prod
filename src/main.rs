use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialise our configuration reader
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Returns a io::Error if we fail to bind the address
    // otherwise call .await on our server
    let listener = TcpListener::bind(address).expect("Expected to bind port for TcpListener");
    run(listener)?.await
}
