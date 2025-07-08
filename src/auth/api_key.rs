use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use rand::RngCore;
use redis::AsyncCommands;
use std::sync::Arc;
use tracing::warn;

use crate::app_state::AppState;

pub const AUTH_HEADER: &str = "Authorization";
pub const AUTH_SCHEME: &str = "SignalStash";
pub const API_KEY_PREFIX: &str = "api_key:";
pub const API_ADMIN_KEY_PREFIX: &str = "api_admin_key:";
pub const ALL_ADMIN_KEYS: &str = "all_admin_keys";

pub const API_KEY_FORMAT_PREFIX: &str = "sk-sigstash-";
pub const ADMIN_KEY_FORMAT_PREFIX: &str = "sk-sigstash-admin-";

/// Extract API key from the Authorization header
/// Format should be: "SignalStash {key}"
fn extract_api_key_from_header(req: &Request<Body>) -> Result<&str, StatusCode> {
    // Extract API key from Authorization header
    let auth_header = match req.headers().get(AUTH_HEADER) {
        Some(header) => header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Parse the header value to extract the API key
    match auth_header.strip_prefix(&format!("{} ", AUTH_SCHEME)) {
        Some(key) => Ok(key),
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn validate_api_key(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = extract_api_key_from_header(&req)?;

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

pub async fn validate_admin_api_key(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = extract_api_key_from_header(&req)?;

    // Get Redis connection
    let mut conn = state
        .redis
        .get_connection_manager()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if Admin API key exists in Redis
    let key_exists: bool = conn
        .exists(format!("{}{}", API_ADMIN_KEY_PREFIX, api_key))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if key_exists {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Generates a secure API key with the given prefix followed by base64-encoded random data
pub fn generate_api_key(prefix: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut random_bytes = [0u8; 48]; // 384 bits of entropy
    rng.fill_bytes(&mut random_bytes);

    // Encode as base64 and remove padding characters
    let random_part = base64::encode_config(&random_bytes, base64::URL_SAFE_NO_PAD);

    // Combine prefix and random data
    format!("{}{}", prefix, random_part)
}

/// Creates a new admin API key and stores it in Redis
pub async fn create_admin_api_key(state: Arc<AppState>) -> Result<String, StatusCode> {
    // Generate a secure key using the admin prefix
    let admin_key = generate_api_key(ADMIN_KEY_FORMAT_PREFIX);

    // Get Redis connection
    let mut conn = state
        .redis
        .get_connection_manager()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Store admin key with "admin" as the value
    let redis_key = format!("{}{}", API_ADMIN_KEY_PREFIX, admin_key);
    conn.set::<_, _, ()>(&redis_key, "admin")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Add to set of all admin keys for tracking
    conn.sadd::<_, _, ()>(ALL_ADMIN_KEYS, &admin_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(admin_key)
}

/// Checks if any admin API keys exist in Redis
pub async fn admin_keys_exist(state: Arc<AppState>) -> Result<bool, StatusCode> {
    // Get Redis connection
    let mut conn = state
        .redis
        .get_connection_manager()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if any admin keys exist by checking the cardinality of the set
    let admin_keys_count: usize = conn
        .scard(ALL_ADMIN_KEYS)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(admin_keys_count > 0)
}

/// Bootstraps an admin API key if none exists
/// Returns a tuple with a boolean (indicating if a new key was created) and optionally the new key
pub async fn bootstrap_admin_key(
    state: Arc<AppState>,
) -> Result<(bool, Option<String>), StatusCode> {
    // Check if any admin keys exist
    let admin_exists = admin_keys_exist(state.clone()).await?;

    if !admin_exists {
        // No admin keys exist, create one
        let admin_key = create_admin_api_key(state).await?;

        // Log the key prominently
        warn!(
            "╔══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗"
        );
        warn!(
            "║                                              INITIAL ADMIN API KEY GENERATED                                              ║"
        );
        warn!(
            "╠══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╣"
        );
        warn!(
            "║ Use the following key to access the API key management endpoints:                                                         ║"
        );
        warn!(
            "║ Authorization: {} {}                                                      ║",
            AUTH_SCHEME, admin_key
        );
        warn!(
            "║                                                                                                                           ║"
        );
        warn!(
            "║ IMPORTANT: Store this key securely! It will not be shown again.                                                           ║"
        );
        warn!(
            "╚══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╝"
        );

        Ok((true, Some(admin_key)))
    } else {
        // Admin keys already exist
        Ok((false, None))
    }
}
