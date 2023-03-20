use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
    },
};

use crate::config::{ServerConfig, ServerConfigMap};

pub fn register<'a, D: ToString>(
    command: &'a mut CreateApplicationCommand,
    command_name: D,
    command_description: D,
    server_configs: &ServerConfigMap,
) -> &'a mut CreateApplicationCommand {
    command
        .name(command_name)
        .description(command_description)
        .create_option(|option| {
            option
                .name("server_name")
                .description("Server name")
                .kind(CommandOptionType::String)
                .required(true);

            server_configs.into_iter().for_each(|(server_name, _)| {
                option.add_string_choice(server_name, server_name);
            });

            return option;
        })
}

pub fn find_server_config<'a>(
    server_configs: &'a ServerConfigMap,
    options: &[CommandDataOption],
) -> Option<&'a ServerConfig> {
    let option_value = options.first().and_then(|o| o.resolved.as_ref());

    if let Some(CommandDataOptionValue::String(server_name)) = option_value {
        server_configs.get(server_name)
    } else {
        None
    }
}
