use zero2prod::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    //Act
    let body = "name=jorge%20clavijo&email=jym272%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    let status_code = response.status().as_u16();
    assert_eq!(200, status_code);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    //Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=jorge%20clavijo", "missing the email"),
        ("email=jym272%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        //Act
        let response = client
            .post(&format!("{}/subscriptions", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        //Assert
        let status_code = response.status().as_u16();
        assert_eq!(
            400, status_code,
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
