use std::process::{Command};
use std::fs;

pub fn spawn_job(commit: &str) -> std::io::Result<()> {
    let log_path = to_log_path(&commit);
    let parent_dir = parent_path(&commit);
    match fs::create_dir_all(parent_dir) {
        Err(e) => Err(e),
        Ok(_) => match Command::new("./supervisor.sh").args(&[commit, &log_path]).spawn() {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}

pub fn read_job_log(commit: &str) -> std::io::Result<String> {
    fs::read_to_string(&to_log_path(&commit))
}

fn to_log_path(commit: &str) -> String {
    let mut path = String::from(commit);
    if path.len() > 2 { path.insert(2, '/') };
    String::from("jobs/") + &path
}

fn parent_path(commit: &str) -> String {
    match commit.len() {
        0 ... 2 => String::from("jobs/"),
        _ => format!("jobs/{}", &commit[..2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        assert_eq!(to_log_path("abc"), "jobs/ab/c");
    }

    #[test]
    fn short() {
        assert_eq!(to_log_path("ab"), "jobs/ab");
    }
}
