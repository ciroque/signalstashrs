use axum::{routing::get, Router};

/* <<<<<<<<<<<<<<  ✨ Windsurf Command ⭐ >>>>>>>>>>>>>>>> */
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
/* <<<<<<<<<<  05b697a8-da86-4735-9207-43674aa82560  >>>>>>>>>>> */
pub fn routes() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/startz", get(startz))
}

/* <<<<<<<<<<<<<<  ✨ Windsurf Command ⭐ >>>>>>>>>>>>>>>> */
/// Returns "ok" if the application is still alive.
///
/// This is intended to be used by load balancers, service meshes, or other external systems to determine
/// if the application is still alive. The application should return a success response (200) if it is
/// healthy, and a failure response (500) if it is not.
/* <<<<<<<<<<  72f1ca22-afaf-4779-9080-0d53041085a9  >>>>>>>>>>> */
async fn healthz() -> &'static str {
    "ok"
}

/* <<<<<<<<<<<<<<  ✨ Windsurf Command ⭐ >>>>>>>>>>>>>>>> */
/// Returns "ready" if the application is ready to receive traffic.
///
/// This is intended to be used by a load balancer or service mesh to determine if the application is ready
/// to receive traffic. The application should return a success response (200) if it is ready, and a
/// failure response (500) if it is not.
/// 
/// External dependencies should be checked before returning "ready".
/* <<<<<<<<<<  48af5623-50e6-41d4-b7cb-48a558ed1911  >>>>>>>>>>> */
async fn readyz() -> &'static str {
    // TODO(steve): check dependencies
    "ready"
}

/* <<<<<<<<<<<<<<  ✨ Windsurf Command ⭐ >>>>>>>>>>>>>>>> */
/// Returns "started" if the application has started successfully.
///
/// This is intended to be used by the application itself to determine if it has started
/// successfully. The application should return a success response (200) if it has started,
/// and a failure response (500) if it has not.
/* <<<<<<<<<<  a5eda83e-462f-40eb-a576-1b23595f88e6  >>>>>>>>>>> */
async fn startz() -> &'static str {
    "started"
}
