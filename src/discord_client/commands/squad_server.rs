use std::io::Result;
use std::process::{Child, Command};
use tracing::info;

pub fn run_script(file_path: &str) -> Result<Child> {
    info!("Running script: {:?}", file_path);

    // We need to use .spawn because .output waits for the process to end
    Command::new(file_path).spawn()
}
