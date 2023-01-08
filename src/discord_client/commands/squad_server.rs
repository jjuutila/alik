use std::io::Error;
use std::process::{Output, Command };
use tracing::{info};

pub fn run_script(file_path: &str) -> Result<Output, Error> {
    info!("Running script: {:?}", file_path);

    Command::new(file_path).output()
}
