use startup::Application;

mod api;
mod auth;
mod context;
mod server;
mod startup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let application = Application::build().await?;
    application.run_until_stopped().await?;
    Ok(())
}
