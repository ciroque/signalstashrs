use redis::Client;
use redis::aio::MultiplexedConnection;
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
        let conn = self
            .client
            .clone()
            .get_multiplexed_tokio_connection()
            .await?;
        Ok(conn)
    }

    pub async fn check_connectivity(&self) -> anyhow::Result<()> {
        let mut conn = self.get_connection_manager().await?;
        let pong: String = redis::cmd(crate::consts::redis::PING_CMD)
            .query_async(&mut conn)
            .await?;
        if pong == crate::consts::redis::PONG_CMD {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Unexpected PING response: {}", pong))
        }
    }
}
