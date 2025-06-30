use crate::redis::RedisStore;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub redis: Arc<RedisStore>,
    pub sensor_datum_prefix: String,
}
