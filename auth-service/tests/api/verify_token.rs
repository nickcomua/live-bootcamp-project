use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_200_valid_token() {
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

    let response = app.post_verify_token(&serde_json::json!({
        "token": auth_cookie.value().to_string()
    }))
    .await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    let response = app.post_verify_token(&serde_json::json!({
        "token": "invalid"
    }))
    .await;

    assert_eq!(response.status(), 401);
}
#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let response = app.post_verify_token(&serde_json::json!({})).await;
    assert_eq!(response.status(), 422);
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
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

    let response = app.post_verify_token(&serde_json::json!({
        "token": auth_cookie.value().to_string()
    }))
    .await;

    assert_eq!(response.status().as_u16(), 200);

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

    let response = app.post_verify_token(&serde_json::json!({
        "token": auth_cookie.value().to_string()
    }))
    .await;

    assert_eq!(response.status(), 401);
}