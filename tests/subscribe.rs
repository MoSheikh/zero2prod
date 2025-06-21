mod utils;
use function_name::named;

use zero2prod::models::Subscription;

#[named]
#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let test_server = utils::init_test(function_name!()).await;
    let client = reqwest::Client::new();

    const NAME: &str = "john doe";
    const EMAIL: &str = "jd@email.com";

    let response = client
        .post(format!("{}/subscribe", test_server.app_address))
        .form(&[("name", NAME), ("email", EMAIL)])
        .send()
        .await
        .expect("Subscribe request failed");

    assert!(response.status().is_success());

    let created_subscription = response
        .json::<Subscription>()
        .await
        .expect("Could not parse created subscription");

    assert_eq!(created_subscription.name, NAME);
    assert_eq!(created_subscription.email, EMAIL);

    let response = client
        .post(format!("{}/subscriptions", test_server.app_address))
        .form(&[("name", NAME), ("email", EMAIL)])
        .send()
        .await
        .expect("Failed to request subscriptions");

    assert!(response.status().is_success());

    let queried_subscription = response
        .json::<Subscription>()
        .await
        .expect("Could not parse subscription from query result");

    assert_eq!(queried_subscription, created_subscription);
}
