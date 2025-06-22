use std::net::TcpListener;

use zero2prod::config::Settings;
use zero2prod::pool::create_pool;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let settings = Settings::get();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.app.port))?;
    let pool = create_pool(&settings.database);

    match run(listener, pool)?.await {
        Ok(_) => Ok(()),
        Err(e) => panic!("Server encountered error:\n{e}"),
    }
}
