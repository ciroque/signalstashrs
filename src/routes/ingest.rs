use axum::{routing::post, Router};

pub fn routes() -> Router {
    Router::new().route("/ingest", post(ingest))
}

async fn ingest() -> &'static str {
    "ingest endpoint"
}
