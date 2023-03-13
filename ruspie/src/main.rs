use startup::Application;

mod api;
mod context;
mod server;
mod startup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    dotenvy::dotenv().ok();
    let application = Application::build().await?;
    application.run_until_stopped().await?;
    Ok(())
}
