use auth_service::ErrorResponse;
use serde_json::json;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let response = app.post_login(&json!({})).await;

    assert_eq!(response.status(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let response = app
        .post_login(&json!({
            "email": "invalid",
            "password": "invalid"
        }))
        .await;

    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.     
    let app = TestApp::new().await;

    let response = app
        .post_login(&json!({
            "email": "invalid@example.com",
            "password": "password123"
        }))
        .await;

    assert_eq!(response.status(), 401);
    assert_eq!(
        response.json::<ErrorResponse>().await.unwrap().error,
        "Incorrect credentials"
    );
}