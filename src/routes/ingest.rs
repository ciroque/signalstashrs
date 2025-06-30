use axum::{routing::post, Router};
use crate::app_state::AppState;
use std::sync::Arc;
use crate::error_utils::log_and_response;
use chrono::Utc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, response::Response, http::Request};
use crate::sensor::{SensorData, Domain};
use axum::http::header::CONTENT_TYPE;
use axum::body::Bytes;
use prost::Message;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ingest", post(ingest))
        .with_state(state)
}

async fn ingest(State(state): State<Arc<AppState>>, body: Bytes) -> Response {
    // Check content-type
    // (axum does not enforce this for us, so we check manually)
    // Accept only application/x-protobuf
    // To check content-type, you need access to the request headers.
    // This requires changing the handler signature to accept the full Request.
    // We'll extract the content-type header from the request.
    //
    // Handler signature update:
    // async fn ingest(State(state): State<Arc<AppState>>, req: Request<Body>, body: Bytes) -> Response
    // For now, let's just add the import for Request and leave a note for the next step.
    
    tracing::debug!("Received {} bytes", body.len());
    
    // Parse protobuf body
    let sensor_data = SensorData::decode(body.as_ref());
    let sensor_data = match sensor_data {
        Ok(msg) => msg,
        Err(e) => return log_and_response("Failed to decode protobuf in ingest", e),
    };

    let device_id = match std::str::from_utf8(&sensor_data.device_id) {
        Ok(s) => s.to_owned(),
        Err(e) => return log_and_response("Invalid UTF-8 in device_id in ingest", e),
    };

    let domain: &'static str = match Domain::from_i32(sensor_data.domain) {
        Some(Domain::SoundPressureLevel) => "SPL",
        Some(Domain::Unspecified) => "UNSPECIFIED",
        None => "UNKNOWN",
    };

    let key = format!("ts:{}:{}", device_id, domain);
    
    // TODO(steve): PUT THIS BACK
    // let timestamp = sensor_data.timestamp;
    let timestamp = chrono::Utc::now().timestamp();
    
    
    let datum = sensor_data.datum;

    let mut conn = match state.redis.get_connection_manager().await {
        Ok(conn) => conn,
        Err(e) => return log_and_response("Failed to get Redis connection in ingest", e),
    };

    let res: redis::RedisResult<()> = redis::cmd("TS.ADD")
        .arg(&key)
        .arg(timestamp)
        .arg(datum)
        .arg("LABELS")
        .arg("device_id").arg(device_id)
        .arg("domain").arg(domain)
        .query_async(&mut conn)
        .await;
    if let Err(e) = res {
        return log_and_response("Failed to write to RedisTimeSeries in ingest", e);
    }

    StatusCode::NO_CONTENT.into_response()
}
