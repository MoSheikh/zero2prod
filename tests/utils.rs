use std::net::TcpListener;
use testcontainers_modules::testcontainers::{ContainerAsync, ImageExt};
use testcontainers_modules::{postgres, testcontainers::runners::AsyncRunner};

use tokio::task::JoinHandle;
use zero2prod::{
    config::{AppSettings, DbSettings, Settings},
    run,
};

use diesel_migrations::*;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct TestServer {
    pub app_address: String,
    _handle: JoinHandle<Result<(), std::io::Error>>,
    _db: ContainerAsync<postgres::Postgres>,
}

pub async fn init_test_db(test_name: &str) -> (ContainerAsync<postgres::Postgres>, String) {
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
    let pool = zero2prod::pool::create_pool(&DbSettings {
        url: url.clone(),
        pool_size: 1,
    });
    let conn = pool.get().await.unwrap();
    match conn
        .interact(|conn| match conn.run_pending_migrations(MIGRATIONS) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        })
        .await
    {
        Ok(res) => match res {
            Ok(_) => (container, url),
            Err(e) => panic!("Could not apply migrations to database: {e}"),
        },
        Err(e) => panic!("Could not apply migrations to database: {e}"),
    }
}

pub async fn init_test(test_name: &str) -> TestServer {
    let (db_container, db_url) = init_test_db(test_name).await;
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();

    let settings = Settings {
        database: DbSettings {
            url: db_url,
            pool_size: 1,
        },
        app: AppSettings { port },
    };

    let server = run(listener, &settings).expect("Failed to bind address.");
    let handle = tokio::spawn(server);

    TestServer {
        app_address: format!("http://127.0.0.1:{port}"),
        _db: db_container,
        _handle: handle,
    }
}
