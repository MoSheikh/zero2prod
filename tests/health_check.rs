mod utils;
use function_name::named;

#[named]
#[tokio::test]
async fn health_check_exists() {
    let test_server = utils::init_test(function_name!()).await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/healthz", test_server.app_address))
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
