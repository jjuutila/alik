mod config;
mod discord_client;

use tracing::error;
use tracing_subscriber::{FmtSubscriber};

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let config = config::parse_config().expect("Failed to load config");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    let subscriber = FmtSubscriber::builder()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let mut client = discord_client::create_discord_client(config)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
