use std::net::TcpListener;

use zero2prod::config::Settings;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = Settings::get();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.app.port))
        .unwrap_or_else(|e| panic!("Could not bind to port {}: {e}", settings.app.port));

    run(listener, &settings)?.await
}
