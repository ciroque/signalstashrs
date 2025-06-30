use axum::{routing::post, Router};
use crate::app_state::AppState;
use std::sync::Arc;
use crate::error_utils::log_and_response;

use axum::{extract::State, http::StatusCode, response::IntoResponse};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ingest", post(ingest))
        .with_state(state)
}

async fn ingest(State(state): State<Arc<AppState>>) -> axum::response::Response {
    if let Err(e) = state.redis.check_connectivity().await {
        return log_and_response("Redis connectivity check failed in ingest", e);
    }
    "ingest endpoint".into_response()
}
