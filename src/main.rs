use std::net::TcpListener;

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use zero2prod::config::Settings;
use zero2prod::pool::create_pool;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    set_global_default(
        Registry::default()
            .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
            .with(JsonStorageLayer)
            .with(BunyanFormattingLayer::new(
                "zero2prod".into(),
                std::io::stdout,
            )),
    )
    .expect("Failed to set subscriber");

    let settings = Settings::get();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.app.port))?;
    let pool = create_pool(&settings.database);

    match run(listener, pool)?.await {
        Ok(_) => Ok(()),
        Err(e) => panic!("Server encountered error:\n{e}"),
    }
}
