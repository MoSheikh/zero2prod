use std::net::TcpListener;

use zero2prod::config::Settings;
use zero2prod::pool::create_pool;
use zero2prod::run;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    telemetry::init("zero2prod", "info", std::io::stdout);

    let settings = Settings::get();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.app.port))?;
    let pool = create_pool(&settings.database);

    match run(listener, pool)?.await {
        Ok(_) => Ok(()),
        Err(e) => panic!("Server encountered error:\n{e}"),
    }
}
