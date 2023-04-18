mod context;
mod startup;

use crate::startup::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let interval = std::env::var("PRE_FETCH_INTERVAL")
        .unwrap_or_else(|_| "60".to_string())
        .parse::<u64>()
        .unwrap();
    let application = Application::new(std::time::Duration::from_secs(interval));
    application.run().await
}
