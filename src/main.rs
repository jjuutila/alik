mod config;
mod discord_client;
mod squad_server;

use squad_server::SquadServer;
use tracing::error;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let config = config::parse_config().expect("Failed to load config");

    let subscriber = FmtSubscriber::builder().finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let squad_server = SquadServer { config: config.1 };

    let mut client = discord_client::create_discord_client(config.0, squad_server)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
