mod context;
mod writer;

use context::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Application::build(None).await?;
    app.run_until_stopped().await?;
    Ok(())
}
