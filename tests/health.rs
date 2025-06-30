use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use tower::util::ServiceExt; 
use std::sync::Arc;
use signalstashrs::app_state::AppState;
use signalstashrs::redis::RedisStore;

async fn test_app_state() -> Arc<AppState> {
    let redis = RedisStore::new("redis://localhost:6379").await.unwrap();
    Arc::new(AppState { sensor_datum_prefix: "test-prefix".to_string(), redis: Arc::new(redis) })
}

#[tokio::test]
async fn healthz_returns_200() {
    let app = signalstashrs::routes::health::routes(test_app_state().await);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "ok");
}

#[tokio::test]
async fn readyz_returns_200() {
    let app = signalstashrs::routes::health::routes(test_app_state().await);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "ready");
}

#[tokio::test]
async fn startz_returns_200() {
    let app = signalstashrs::routes::health::routes(test_app_state().await);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/startz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "started");
}
