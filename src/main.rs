mod config;
mod discord_client;
mod squad_server;

use std::fs;
use tracing::error;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let config_string = fs::read_to_string("config.ini").expect("Could not read config file");
    let config = config::parse_config(config_string).expect("Failed to parse config");

    let subscriber = FmtSubscriber::builder().finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let mut client = discord_client::create_discord_client(config)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
