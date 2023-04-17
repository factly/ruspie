mod context;
mod startup;

use crate::startup::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let application = Application::new(std::time::Duration::from_secs(10));
    application.run().await
}
