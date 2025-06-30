use axum::{Router};
use std::net::SocketAddr;
use tracing::{info};
use crate::routes;

use crate::config::Settings;

pub struct Application {
    settings: Settings,
    router: Router,
}

impl Application {
/* <<<<<<<<<<<<<<  ✨ Windsurf Command ⭐ >>>>>>>>>>>>>>>> */
    /// Builds a new instance of `Application`.
    ///
    /// This method will return an error if the `Settings` cannot be built from the environment.
    /// It will also initialize the global tracing subscriber with the configured log level.
    ///
    /// After building the settings and initializing the tracing subscriber, it will construct a
    /// new `Router` instance with the routes from `health` and `ingest` merged into it.
    ///
    /// # Errors
    ///
    /// This method will return an error if the settings cannot be built from the environment.
    ///
    /// # Examples
    ///
    /// 
/* <<<<<<<<<<  165ce97d-5003-4ce2-9acb-ed4b0508ceab  >>>>>>>>>>> */
    pub async fn build() -> anyhow::Result<Self> {
        let settings = Settings::from_env()?;

        tracing_subscriber::fmt()
            .with_max_level(settings.log_level)
            .with_target(false)
            .compact()
            .init();

        let router = Router::new()
            .merge(routes::health::routes())
            .merge(routes::ingest::routes());
        
        Ok(Self { settings, router })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let addr: SocketAddr = self.settings.bind_address.parse()?;
        let tcp_listener = tokio::net::TcpListener::bind(addr).await?;
        info!("Starting server on http://{}", addr);

        axum::serve(tcp_listener, self.router).await?;
        Ok(())
    }
}
