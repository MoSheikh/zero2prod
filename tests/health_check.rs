use std::net::TcpListener;

use reqwest;

use zero2prod::run;

const HOST: &str = "127.0.0.1";

#[tokio::test]
async fn health_check_exists() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/healthz"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(
        response
            .text()
            .await
            .expect("Could not parse response text."),
        "OK"
    );
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address.");
    let _ = tokio::spawn(server);

    format!("http://{HOST}:{port}")
}
