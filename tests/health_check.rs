mod utils;

#[tokio::test]
async fn health_check_exists() {
    let address = utils::spawn_app();
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
