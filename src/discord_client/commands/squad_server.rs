use std::io::Error;
use std::process::{Output, Command };
use tracing::{info};


pub fn start_server(start_batch_file_path: &str) -> Result<Output, Error> {
    info!("Starting server");

    Command::new(start_batch_file_path).output()
}
