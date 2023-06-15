// mod main_mp;
// mod main_runner;
// mod main_watcher;
// mod main_spider;
// mod main_spawn;
mod main_broker;
mod main_producer;
mod main_consumer;
// mod main_wasm;

use std::env;


fn main() {
    // main_mp::main();
    // main_runner::main();
    // main_watcher::main();
    // main_spider::main();
    // main_spawn::main();
    // main_wasm::main();
    let role = env::var("ROLE").unwrap_or(String::from("default"));
    match role.as_str() {
        "broker" => {
            println!("[Broker]");
            main_broker::main();
        },
        "producer" => {
            println!("[Producer]");
            main_producer::main();
        },
        "consumer" => {
            println!("[Consumer]");
            main_consumer::main();
        },
        _ => {
            println!("No role specified.");
        },
    }
}
