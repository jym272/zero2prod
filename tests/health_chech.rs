//! tests/health_check.rs
use std::net::TcpListener;

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    //Arrange
    let port = spawn_app();
    let url = format!("http://localhost:{}/health_check", port);
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
fn spawn_app() -> String {
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    //port 0: random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to start server.");
    let _ = tokio::spawn(server);
    //it's dropped when the tokio runtime is over, new runtime at the
    //beginning of each test
    port.to_string()
}
