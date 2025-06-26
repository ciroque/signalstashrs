use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::{info, Level};

use crate::config::Settings;

pub struct Application {
    settings: Settings,
    router: Router,
}

impl Application {
    pub async fn build() -> anyhow::Result<Self> {
        tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .with_target(false)
            .compact()
            .init();

        let settings = Settings::from_env()?;

        let router = Router::new()
            .route("/healthz", get(Self::healthz))
            .route("/readyz", get(Self::readyz))
            .route("/startz", get(Self::startz));

        Ok(Self { settings, router })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let addr: SocketAddr = self.settings.bind_address.parse()?;
        let tcp_listener = tokio::net::TcpListener::bind(addr).await?;
        info!("Starting server on http://{}", addr);

        axum::serve(tcp_listener, self.router).await?;
        Ok(())
    }

    async fn healthz() -> &'static str {
        "ok"
    }
    async fn readyz() -> &'static str {
        "ok"
    }
    async fn startz() -> &'static str {
        "ok"
    }
}
