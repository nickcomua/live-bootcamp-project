use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    ErrorResponse, domain::{Email, EmailValidationError}, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME
};
use serde_json::json;

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

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled(
) -> Result<(), EmailValidationError> {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email.clone(),
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    assert!(app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email)?)
        .await
        .is_ok());
    Ok(())
}
