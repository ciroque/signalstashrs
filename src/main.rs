use signalstashrs::application::Application;

mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Application::build().await?.run().await?;
    Ok(())
}
