use std::net::TcpListener;
use std::sync::LazyLock;
use testcontainers_modules::testcontainers::{ContainerAsync, ImageExt};
use testcontainers_modules::{postgres, testcontainers::runners::AsyncRunner};

use tokio::task::JoinHandle;
use zero2prod::{
    config::DbSettings,
    pool::{Pool, create_pool},
    run, telemetry,
};

use diesel_migrations::*;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct TestServer {
    pub app_address: String,
    _handle: JoinHandle<Result<(), std::io::Error>>,
    _db: ContainerAsync<postgres::Postgres>,
}

static TRACING: LazyLock<()> = LazyLock::new(|| match std::env::var("TEST_LOG").is_ok() {
    true => telemetry::init("test", "debug", std::io::stdout),
    false => telemetry::init("test", "debug", std::io::sink),
});

pub async fn init_test_db(test_name: &str) -> (ContainerAsync<postgres::Postgres>, Pool) {
    let container = postgres::Postgres::default()
        .with_db_name(test_name)
        .with_user("test")
        .with_password("test")
        .with_container_name(format!("pg_{test_name}"))
        .start()
        .await
        .expect("Could not start postgres container");

    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let url = format!("postgres://test:test@localhost:{port}/{test_name}");
    let pool = create_pool(&DbSettings { url, pool_size: 16 });
    let conn = pool.get().await.unwrap();
    let migration_result = conn
        .interact(|conn| match conn.run_pending_migrations(MIGRATIONS) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        })
        .await;

    match migration_result {
        Ok(res) => match res {
            Ok(_) => (container, pool),
            Err(e) => panic!("Could not apply migrations to database: {e}"),
        },
        Err(e) => panic!("Could not apply migrations to database: {e}"),
    }
}

pub async fn init_test(test_name: &str) -> TestServer {
    LazyLock::force(&TRACING);

    let (db_container, pool) = init_test_db(test_name).await;
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener, pool).expect("Failed to bind address.");
    let handle = tokio::spawn(server);

    TestServer {
        app_address: format!("http://127.0.0.1:{port}"),
        _db: db_container,
        _handle: handle,
    }
}
