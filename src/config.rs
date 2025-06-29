use std::env;
use tracing::Level;

pub struct Settings {
    pub bind_address: String,
    pub log_level: Level,
    pub redis_url: String,
}

impl Settings {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        let log_level = env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "INFO".into())
            .parse()?;
        let bind_address = env::var("BIND_ADDRESS")
            .unwrap_or_else(|_| "0.0.0.0:20120".into());
        
        let redis_url = env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://localhost:6379".into());

        Ok(Self { bind_address, log_level, redis_url })
    }
}
