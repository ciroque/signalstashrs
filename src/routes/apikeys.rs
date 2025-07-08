use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::app_state::AppState;

const API_KEY_PREFIX: &str = "api_key:";
const ALL_API_KEYS: &str = "all_api_keys";

#[derive(Serialize)]
struct ApiKey {
    key: String,
    user_id: String,
}

#[derive(Deserialize)]
struct CreateApiKeyRequest {
    user_id: String,
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/keys", get(list_keys))
        .route("/api/keys", post(create_key))
        .route("/api/keys/:key", delete(revoke_key))
        .with_state(state)
}


async fn create_key(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<Json<ApiKey>, StatusCode> {
    // Generate a new API key with our custom format
    let key = crate::auth::generate_api_key();
    
    // Get Redis connection
    let mut conn = state
        .redis
        .get_connection_manager()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Store API key with user ID as value
    let redis_key = format!("{}{}", API_KEY_PREFIX, key);
    conn.set::<_, _, ()>(&redis_key, &payload.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Add to set of all API keys for tracking
    conn.sadd::<_, _, ()>(ALL_API_KEYS, &key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiKey {
        key,
        user_id: payload.user_id,
    }))
}

async fn list_keys(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ApiKey>>, StatusCode> {
    // Get Redis connection
    let mut conn = state
        .redis
        .get_connection_manager()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get all API keys
    let keys: Vec<String> = conn
        .smembers(ALL_API_KEYS)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut api_keys = Vec::new();
    
    // For each key, get the associated user ID
    for key in keys {
        let redis_key = format!("{}{}", API_KEY_PREFIX, key);
        let user_id: String = conn
            .get(&redis_key)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
        api_keys.push(ApiKey { key, user_id });
    }

    Ok(Json(api_keys))
}

async fn revoke_key(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Get Redis connection
    let mut conn = state
        .redis
        .get_connection_manager()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Remove the API key
    let redis_key = format!("{}{}", API_KEY_PREFIX, key);
    let exists: bool = conn
        .exists(&redis_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !exists {
        return Err(StatusCode::NOT_FOUND);
    }

    // Delete the key
    conn.del::<_, ()>(&redis_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Remove from set of all API keys
    conn.srem::<_, _, ()>(ALL_API_KEYS, &key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
