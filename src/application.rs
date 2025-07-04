use crate::app_state::AppState;
use crate::redis::RedisStore;
use crate::routes;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

use crate::config::Settings;

pub struct Application {
    settings: Settings,
    router: Router,
}

impl Application {
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
    pub async fn build() -> anyhow::Result<Self> {
        let env = std::env::vars().collect();
        let settings = Settings::from_env_vars(&env)?;

        tracing_subscriber::fmt()
            .with_max_level(settings.log_level)
            .with_target(false)
            .compact()
            .init();

        let redis = RedisStore::new(&settings.redis_url).await?;

        let sensor_datum_prefix = settings.sensor_datum_prefix.clone();
        let state = Arc::new(AppState {
            redis: Arc::new(redis),
            sensor_datum_prefix,
        });

        let router = Router::new()
            .merge(routes::health::routes(state.clone()))
            .merge(routes::ingest::routes(state.clone()));

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
