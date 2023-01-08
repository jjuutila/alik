mod commands;

use tracing::{error, info};

use serenity::{
    async_trait,
    model::{
        event::ResumedEvent,
        gateway::Ready,
        id::GuildId,
        application::interaction::{Interaction, InteractionResponseType},
    },
    prelude::*,
    Error,
};

use crate::config::{DiscordConfig, BotConfig};

struct Handler {
    config: BotConfig,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        let result = GuildId::set_application_commands(
            &GuildId(self.config.guild_id),
            &ctx.http,
            |commands| {
                commands.create_application_command(|command| {
                    command
                        .name("start_server")
                        .description("Starts the server")
                })
            },
        )
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
                "start_server" => {
                    match commands::squad_server::run_script(&self.config.start_batch_file_path) {
                        Ok(_) => String::from("Server started"),
                        Err(e) => {
                            error!("Error starting the server: '{}'", e);
                            format!("Error starting the server: '{}'", e)
                        },
                    }
                }
                _ => "not implemented :(".to_string(),
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

pub async fn create_discord_client(config: (DiscordConfig, BotConfig)) -> Result<Client, Error> {
    let DiscordConfig { discord_token, application_id} = config.0;

    let handler = Handler { config: config.1 };

    Client::builder(discord_token, GatewayIntents::empty())
        .application_id(application_id)
        .event_handler(handler)
        .await
}
