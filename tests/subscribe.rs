mod utils;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = utils::spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{address}/subscriptions"))
        .form(&[("name", "john doe"), ("email", "jdoe@email.com")])
        .send()
        .await
        .expect("Failed to execute request");

    let status = response.status().as_u16();
    assert_eq!(status, 200);
}
