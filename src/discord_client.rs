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

use crate::config::{AlikConfig, DiscordConfig, ServerConfigMap};

struct Handler {
    guild_id: u64,
    server_configs: ServerConfigMap,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        let result =
            GuildId::set_application_commands(&GuildId(self.guild_id), &ctx.http, |commands| {
                commands
                    .create_application_command(|command| {
                        commands::start_server::register(command, &self.server_configs)
                    })
                    .create_application_command(|command| {
                        commands::stop_server::register(command, &self.server_configs)
                    })
                    .create_application_command(|command| {
                        commands::restart_server::register(command, &self.server_configs)
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
                commands::start_server::NAME => {
                    commands::start_server::run(&self.server_configs, &command.data.options)
                }
                commands::stop_server::NAME => {
                    commands::stop_server::run(&self.server_configs, &command.data.options)
                }
                commands::restart_server::NAME => {
                    commands::restart_server::run(&self.server_configs, &command.data.options)
                }
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

pub async fn create_discord_client(config: AlikConfig) -> Result<Client, Error> {
    let DiscordConfig {
        discord_token,
        application_id,
        guild_id,
    } = config.discord;

    let handler = Handler {
        guild_id,
        server_configs: config.servers,
    };

    Client::builder(discord_token, GatewayIntents::empty())
        .application_id(application_id)
        .event_handler(handler)
        .await
}
