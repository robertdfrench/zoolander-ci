use std::process::{Command};

pub fn spawn_job(commit: &str, log_path: &str) -> std::io::Result<()> {
    match Command::new("./supervisor.sh").args(&[commit, log_path]).spawn() {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}
