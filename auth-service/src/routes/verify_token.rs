use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email},
    utils::auth::validate_token,
};

pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let token = request.token;
    let claims = match validate_token(&token, state.banned_token_store).await {
        Ok(claims) => claims,
        Err(_) => return Err(AuthAPIError::InvalidToken),
    };
    let email = claims.sub;

    let user_store = &state.user_store.read().await;

    if user_store
        .get_user(&Email::parse(email).map_err(|_| AuthAPIError::InvalidToken)?)
        .await
        .is_err()
    {
        return Err(AuthAPIError::InvalidToken);
    }
    Ok("Token is valid")
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
