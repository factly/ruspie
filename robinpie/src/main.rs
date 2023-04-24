mod context;
mod writer;

use mongodb::options::ClientOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mongo_uri: String =
        std::env::var("MONGO_URI").unwrap_or("mongodb://localhost:27017".to_string());
    let client_options = ClientOptions::parse(mongo_uri).await?;
    let client = mongodb::Client::with_options(client_options)?;
    let db = client.database("robinpie");
    println!("Connected to database: {}", db.name());
    Ok(())
}
