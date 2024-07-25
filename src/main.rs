use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialise our configuration reader
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Returns a io::Error if we fail to bind the address
    // otherwise call .await on our server
    let listener = TcpListener::bind(address).expect("Expected to bind port for TcpListener");
    run(listener, connection_pool)?.await
}
