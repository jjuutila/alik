use super::helpers;
use crate::config::ServerConfigMap;
use crate::squad_server::start_server;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub const NAME: &str = "start_server";

pub fn run(server_configs: &ServerConfigMap, options: &[CommandDataOption]) -> String {
    match helpers::find_server_config(server_configs, options) {
        Some(sc) => match start_server(sc) {
            Ok(_) => String::from("Server starting"),
            Err(e) => format!("Error starting the server: '{}'", e),
        },
        None => String::from("Server not found"),
    }
}

pub fn register<'a>(
    command: &'a mut CreateApplicationCommand,
    server_configs: &ServerConfigMap,
) -> &'a mut CreateApplicationCommand {
    helpers::register(command, NAME, "Start a server", server_configs)
}
