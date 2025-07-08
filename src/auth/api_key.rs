use axum::{
    extract::State,
    http::StatusCode,
    middleware::Next,
    response::Response,
    body::Body,
};
use redis::AsyncCommands;
use std::sync::Arc;

use crate::app_state::AppState;

pub const AUTH_HEADER: &str = "Authorization";
pub const AUTH_SCHEME: &str = "SignalStash";
pub const API_KEY_PREFIX: &str = "api_key:";

pub async fn validate_api_key(
    State(state): State<Arc<AppState>>,
    req: axum::http::Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract API key from Authorization header
    let auth_header = match req.headers().get(AUTH_HEADER) {
        Some(header) => header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Parse the header value to extract the API key
    // Format should be: "SignalStash sk-sigstash-{key}"
    let api_key = match auth_header.strip_prefix(&format!("{} ", AUTH_SCHEME)) {
        Some(key) => key,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Get Redis connection
    let mut conn = state
        .redis
        .get_connection_manager()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if API key exists in Redis
    let key_exists: bool = conn
        .exists(format!("{}{}", API_KEY_PREFIX, api_key))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if key_exists {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
