// mod main_mp;
// mod main_runner;
// mod main_watcher;
// mod main_spider;
// mod main_spawn;
// mod main_wasm;
mod main_broker;
mod main_producer;
mod main_consumer;

use std::env;

// use wasm_runner_manager::{create_manager};


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
    // let mut mgr = create_manager();
    // let plugin_name = "plugin1";
    // mgr.load_plugin(plugin_name, "./libs/lib1.wasm", "handler");
    // mgr.exec_plugin(plugin_name);
}
