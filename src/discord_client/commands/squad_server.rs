use std::io::Error;
use std::process::{Child, Command};

// cd "C:\servers\squad\_trainingserver\"
// start SquadServer.exe Port=8000 QueryPort=49000 FIXEDMAXPLAYERS=80 FIXEDMAXTICKRATE=300 RANDOM=NONE -log -fullcrashdump

pub fn start_server(start_batch_file_path: &str) -> Result<Child, Error> {
    Command::new(start_batch_file_path).spawn()
}
