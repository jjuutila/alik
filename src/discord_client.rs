mod commands;

use commands::meta::*;
use std::{collections::HashSet, sync::Arc};
use tracing::{error, info};

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{
        event::ResumedEvent,
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::ApplicationCommandInteractionDataOptionValue, Interaction,
            InteractionResponseType,
        },
    },
    prelude::*,
    Error,
};

use crate::config::Config;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[group]
#[commands(ping)]
struct General;

struct Handler {
    guild_id: u64,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        let result =
            GuildId::set_application_commands(&GuildId(self.guild_id), &ctx.http, |commands| {
                commands.create_application_command(|command| {
                    command
                        .name("start_server")
                        .description("Starts the server")
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
                "start_server" => "Hey, I'm alive!".to_string(),
                "id" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option")
                        .resolved
                        .as_ref()
                        .expect("Expected user object");

                    if let ApplicationCommandInteractionDataOptionValue::User(user, _member) =
                        options
                    {
                        format!("{}'s id is {}", user.tag(), user.id)
                    } else {
                        "Please provide a valid user".to_string()
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

pub async fn create_discord_client(config: Config) -> Result<Client, Error> {
    let http = Http::new_with_token(&config.discord_token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(err) => panic!("Could not access application info: {:?}", err),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("~"))
        .group(&GENERAL_GROUP);

    let handler = Handler {
        guild_id: config.guild_id,
    };

    Client::builder(&config.discord_token)
        .framework(framework)
        .application_id(config.application_id)
        .event_handler(handler)
        .await
}
