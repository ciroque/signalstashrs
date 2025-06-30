use axum::{routing::post, Router};
use crate::app_state::AppState;
use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ingest", post(ingest))
        .with_state(state)
}

async fn ingest(State(state): State<Arc<AppState>>) -> axum::response::Response {
    if let Err(_) = state.redis.check_connectivity().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Redis unavailable").into_response();
    }
    "ingest endpoint".into_response()
}
