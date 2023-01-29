use crate::config::ServerConfig;
use std::io::Result;
use std::process::{Child, Command, Output};
use tracing::info;

pub struct SquadServer {
    pub config: ServerConfig,
}

impl SquadServer {
    pub fn start_server(&self) -> Result<Child> {
        info!(
            "Running start script: {:?}",
            self.config.start_batch_file_path
        );

        // We need to use .spawn because .output waits for the process to end
        Command::new(&self.config.start_batch_file_path).spawn()
    }

    pub fn stop_server(&self) -> Result<Output> {
        info!(
            "Running stop script: {:?}",
            self.config.stop_batch_file_path
        );

        Command::new(&self.config.stop_batch_file_path).output()
    }

    pub fn restart_server(&self) -> Result<Child> {
        self.stop_server()?;
        self.start_server()
    }
}
