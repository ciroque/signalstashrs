use axum::{http::StatusCode, response::IntoResponse};
use tracing::error;
use uuid::Uuid;

/// Logs the error with a correlation ID and returns a generic error response with the correlation ID.
pub fn log_and_response<E: std::fmt::Display>(context: &str, err: E) -> axum::response::Response {
    let correlation_id = Uuid::new_v4();
    error!(correlation_id = %correlation_id, error = %err, "{context}");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("internal error (correlation id: {correlation_id})"),
    )
        .into_response()
}
