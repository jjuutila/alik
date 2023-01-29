mod commands;

use tracing::{error, info};

use serenity::{
    async_trait,
    model::{
        application::interaction::{Interaction, InteractionResponseType},
        event::ResumedEvent,
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
    Error,
};

use crate::{config::DiscordConfig, squad_server::SquadServer};

struct Handler {
    guild_id: u64,
    squad_server: SquadServer,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        let result =
            GuildId::set_application_commands(&GuildId(self.guild_id), &ctx.http, |commands| {
                commands
                    .create_application_command(|command| {
                        command
                            .name("start_server")
                            .description("Starts the server")
                    })
                    .create_application_command(|command| {
                        command.name("stop_server").description("Stops the server")
                    })
                    .create_application_command(|command| {
                        command
                            .name("restart_server")
                            .description("Restarts the server")
                    })
            })
            .await;

        match result {
            Ok(_) => info!("Commands added successfully"),
            Err(why) => error!("Adding commands failed: {:?}", why),
        }
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "start_server" => match self.squad_server.start_server() {
                    Ok(_) => {
                        info!("Server starting");
                        String::from("Server starting")
                    }
                    Err(e) => {
                        error!("Error starting the server: '{}'", e);
                        format!("Error starting the server: '{}'", e)
                    }
                },
                "stop_server" => match self.squad_server.stop_server() {
                    Ok(_) => {
                        info!("Server stopped");
                        String::from("Server stopped")
                    }
                    Err(e) => {
                        error!("Error stopping the server: '{}'", e);
                        format!("Error stopping the server: '{}'", e)
                    }
                },
                "restart_server" => match self.squad_server.restart_server() {
                    Ok(_) => {
                        info!("Server restarting");
                        String::from("Server restarting")
                    }
                    Err(e) => {
                        error!("Error restarting the server: '{}'", e);
                        format!("Error restarting the server: '{}'", e)
                    }
                },
                _ => "Command not implemented".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

pub async fn create_discord_client(
    config: DiscordConfig,
    squad_server: SquadServer,
) -> Result<Client, Error> {
    let DiscordConfig {
        discord_token,
        application_id,
        guild_id,
    } = config;

    let handler = Handler {
        guild_id,
        squad_server,
    };

    Client::builder(discord_token, GatewayIntents::empty())
        .application_id(application_id)
        .event_handler(handler)
        .await
}
