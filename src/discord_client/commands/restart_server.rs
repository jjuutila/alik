use crate::config::ServerConfigMap;
use crate::squad_server::restart_server;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use super::helpers;

pub const NAME: &str = "restart_server";

pub fn run(server_configs: &ServerConfigMap, options: &[CommandDataOption]) -> String {
    match helpers::find_server_config(server_configs, options) {
        Some(sc) => match restart_server(sc) {
            Ok(_) => String::from("Server restarting"),
            Err(e) => format!("Error restarting the server: '{}'", e),
        },
        None => String::from("Server not found"),
    }
}

pub fn register<'a>(
    command: &'a mut CreateApplicationCommand,
    server_configs: &ServerConfigMap,
) -> &'a mut CreateApplicationCommand {
    helpers::register(command, NAME, "Restarts a server", server_configs)
}
