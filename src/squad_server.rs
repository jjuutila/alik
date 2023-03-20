use crate::config::ServerConfig;
use std::io::Result;
use std::process::{Child, Command, Output};
use tracing::info;

pub fn start_server(server_config: &ServerConfig) -> Result<Child> {
    info!(
        "Running start script: {:?}",
        server_config.start_batch_file_path
    );

    // We need to use .spawn because .output waits for the process to end
    Command::new(&server_config.start_batch_file_path).spawn()
}

pub fn stop_server(server_config: &ServerConfig) -> Result<Output> {
    info!(
        "Running stop script: {:?}",
        server_config.stop_batch_file_path
    );

    Command::new(&server_config.stop_batch_file_path).output()
}

pub fn restart_server(server_config: &ServerConfig) -> Result<Child> {
    stop_server(server_config)?;
    start_server(server_config)
}
