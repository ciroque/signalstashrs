use signalstashrs::application::Application;

mod config;

/* <<<<<<<<<<<<<<  ✨ Windsurf Command ⭐ >>>>>>>>>>>>>>>> */
/// The main entry point for the application.
///
/// This function is marked as `#[tokio::main]`, which means it will be called by
/// the Tokio runtime once it has been initialized. It will then create a new
/// `Application` instance, build it, and then run it.
///
/// The `main` function will return an error if either the `Application` instance
/// cannot be built or if the `run` method fails.
/* <<<<<<<<<<  1c28239e-db5d-4158-afb6-f85aef1b4f7b  >>>>>>>>>>> */
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Application::build().await?.run().await?;
    Ok(())
}
