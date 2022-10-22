use startup::Application;

mod api;
mod context;
mod server;
mod startup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let application = Application::build().await?;
    application.run_until_stopped().await?;
    Ok(())
}
