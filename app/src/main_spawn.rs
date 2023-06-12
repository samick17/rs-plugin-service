use std::process::{Command};
use std::thread;
use std::time::{Duration};

fn main() {
    let mut child = Command::new("ping")
    .arg("localhost")
    .spawn()
    .expect("Failed to spawn child process");

    thread::sleep(Duration::from_secs(3));
    child.kill();
    child.wait().unwrap();
}
