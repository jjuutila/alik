use crate::config::ServerConfigMap;
use crate::squad_server::stop_server;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use super::helpers;

pub const NAME: &str = "stop_server";

pub fn run(server_configs: &ServerConfigMap, options: &[CommandDataOption]) -> String {
    match helpers::find_server_config(server_configs, options) {
        Some(sc) => match stop_server(sc) {
            Ok(_) => String::from("Server stopped"),
            Err(e) => format!("Error stopping the server: '{}'", e),
        },
        None => String::from("Server not found"),
    }
}

pub fn register<'a>(
    command: &'a mut CreateApplicationCommand,
    server_configs: &ServerConfigMap,
) -> &'a mut CreateApplicationCommand {
    helpers::register(command, NAME, "Stops a server", server_configs)
}
