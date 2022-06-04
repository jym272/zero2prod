//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    //Arrange
    spawn_app();
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    //healthy -> length of response body should be equal to "Healthy"
    assert_eq!(Some(7), response.content_length());
}
fn spawn_app() {
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    //port 0: random port
    let server = zero2prod::run("127.0.0.1:0").expect("Failed to start server.");
    let _ = tokio::spawn(server);
}
