use redis::aio::{ConnectionManager, MultiplexedConnection};
use redis::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct RedisStore {
    client: Arc<Client>,
}

impl RedisStore {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let client = Client::open(url)?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub async fn get_connection_manager(&self) -> anyhow::Result<MultiplexedConnection> {
        let conn = self.client.clone().get_multiplexed_tokio_connection().await?;
        Ok(conn)
    }

    pub async fn check_connectivity(&self) -> anyhow::Result<()> {
        let mut conn = self.get_connection_manager().await?;
        let pong: String = redis::cmd("PING").query_async(&mut conn).await?;
        if pong == "PONG" {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Unexpected PING response: {}", pong))
        }
    }
}
