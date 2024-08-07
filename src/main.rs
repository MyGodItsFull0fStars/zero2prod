use std::net::TcpListener;

use sqlx::{PgPool, Pool, Postgres};

use zero2prod::configuration::{get_configuration, Settings};
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Initialise our configuration reader
    let configuration: Settings = get_configuration().expect("Failed to read configuration");
    let connection_pool: Pool<Postgres> =
        PgPool::connect(&configuration.database.connection_string())
            .await
            .expect("Failed to connect to Postgres");

    // Here we choose to bind explicitly to localhost, 127.0.0.1, for security
    // reasons. This binding may cause issues in some environments. For example,
    // it causes connectivity issues running in WSL2, where you cannot reach the
    // server when it is bound to WSL2's localhost interface. As a workaround,
    // you can choose to bind to all interfaces, 0.0.0.0, instead, but be aware
    // of the security implications when you expose the server on all interfaces.
    let address: String = format!("127.0.0.1:{}", configuration.application_port);
    // Returns a io::Error if we fail to bind the address
    // otherwise call .await on our server
    let listener: TcpListener =
        TcpListener::bind(address).expect("Expected to bind port for TcpListener");
    run(listener, connection_pool)?.await?;

    Ok(())
}
