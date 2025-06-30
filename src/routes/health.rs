use std::sync::Arc;
use axum::{routing::get, Router, extract::State, response::{IntoResponse, Response}};
use crate::app_state::AppState;
use crate::error_utils::log_and_response;

/// Returns a new `Router` containing endpoints for health-checking and startup synchronization.
///
/// This includes:
///
/// * `/healthz`: Returns "ok" if the application is still alive.
/// * `/readyz`: Returns "ready" if the application is ready to receive traffic.
/// * `/startz`: Returns "ok" if the application has finished starting up.
///
/// These endpoints are intended to be used by load balancers, service meshes, or other external systems
/// to determine if the application is still alive and ready to receive traffic, and when it has finished
/// starting up.
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .with_state(state)
        .route("/startz", get(startz))
}

/// Returns "ok" if the application is still alive.
///
/// This is intended to be used by load balancers, service meshes, or other external systems to determine
/// if the application is still alive. The application should return a success response (200) if it is
/// healthy, and a failure response (500) if it is not.
async fn healthz() -> &'static str {
    "ok"
}

/// Returns "ready" if the application is ready to receive traffic.
///
/// This is intended to be used by a load balancer or service mesh to determine if the application is ready
/// to receive traffic. The application should return a success response (200) if it is ready, and a
/// failure response (500) if it is not.
/// 
/// External dependencies should be checked before returning "ready".
async fn readyz(State(state): State<Arc<AppState>>) -> axum::response::Response {
    match state.redis.check_connectivity().await {
        Ok(()) => "ready".into_response(),
        Err(e) => log_and_response("Redis connectivity check failed in readyz", e),
    }
}

/// Returns "started" if the application has started successfully.
///
/// This is intended to be used by the application itself to determine if it has started
/// successfully. The application should return a success response (200) if it has started,
/// and a failure response (500) if it has not.
async fn startz() -> &'static str {
    "started"
}
