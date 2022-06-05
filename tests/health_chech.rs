//! tests/health_check.rs
use zero2prod::spawn_app;

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    //Arrange
    let address = spawn_app();
    let url = format!("{}/health_check", address);
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(url.as_str())
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    //healthy -> length of response body should be equal to "Healthy"
    assert_eq!(Some(7), response.content_length());
}
