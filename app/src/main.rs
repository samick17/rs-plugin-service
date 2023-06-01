// mod main_mp;
// mod main_runner;
// mod main_watcher;
// mod main_spider;
use std::process::{Command};
use std::thread;
use std::time::{Duration};

fn main() {
    // main_mp::main();
    // main_runner::main();
    // main_watcher::main();
    // main_spider::main();
    let mut child = Command::new("ping")
    .arg("localhost")
    .spawn()
    .expect("Failed to spawn child process");

    thread::sleep(Duration::from_secs(3));
    child.kill();
    child.wait().unwrap();
}
