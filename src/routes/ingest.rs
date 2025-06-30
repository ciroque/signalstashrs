use axum::{routing::post, Router};
use crate::app_state::AppState;
use std::sync::Arc;
use crate::error_utils::log_and_response;

use axum::{extract::State, http::StatusCode, response::IntoResponse, response::Response, http::Request};
use crate::sensor::{SensorData, Domain};
use axum::body::Bytes;
use prost::Message;

use crate::consts::redis::{REDIS_LABEL_DEVICE_ID, REDIS_LABEL_DOMAIN, REDIS_CMD_TS_ADD, REDIS_LABELS_LABEL};
use crate::consts::errors::{ERR_DECODE_PROTOBUF, ERR_INVALID_UTF8_DEVICE_ID, ERR_REDIS_CONN, ERR_REDIS_WRITE};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(crate::consts::routes::INGEST_PATH, post(ingest))
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
        Err(e) => return log_and_response(ERR_DECODE_PROTOBUF, e),
    };

    let device_id = match std::str::from_utf8(&sensor_data.device_id) {
        Ok(s) => s.to_owned(),
        Err(e) => return log_and_response(ERR_INVALID_UTF8_DEVICE_ID, e),
    };

    let domain = Domain::from_i32(sensor_data.domain)
        .map(|d| d.as_str_name())
        .unwrap_or("UNKNOWN");
    
    let key = format!("{}:{}:{}", state.sensor_datum_prefix, device_id, domain);
    
    // TODO(steve): PUT THIS BACK
    // let timestamp = sensor_data.timestamp;
    let timestamp = chrono::Utc::now().timestamp();
    
    
    let datum = sensor_data.datum;

    let mut conn = match state.redis.get_connection_manager().await {
        Ok(conn) => conn,
        Err(e) => return log_and_response(ERR_REDIS_CONN, e),
    };

    let res: redis::RedisResult<()> = redis::cmd(REDIS_CMD_TS_ADD)
        .arg(&key)
        .arg(timestamp)
        .arg(datum)
        .arg(REDIS_LABELS_LABEL)
        .arg(REDIS_LABEL_DEVICE_ID).arg(&device_id)
        .arg(REDIS_LABEL_DOMAIN).arg(domain)
        .query_async(&mut conn)
        .await;
    if let Err(e) = res {
        return log_and_response(ERR_REDIS_WRITE, e);
    }

    StatusCode::NO_CONTENT.into_response()
}
