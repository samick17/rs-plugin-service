// use so_runner;
// use wasm_runner;

// fn main() {
//     so_runner::execute_so("./libs/liblib1.so", "handler");
//     wasm_runner::execute_wasm("./libs/lib1.wasm", "handler");
// }
// 
// use std::sync::mpsc::{channel};
// use std::thread::{spawn};

// use cli::{get_input};
// use queue::{Queue};

// fn main() {
//     let (tx, rx) = channel::<String>();
//     let mut queue: Queue<String> = Queue::new();
//     let tx_clone = tx.clone();
//     spawn(move || {
//         loop {
//             let text = get_input();
//             println!("> {}", text);
//             tx_clone.send(text).unwrap();
//         }
//     });
//     while let Ok(msg) = rx.recv() {
//         queue.enqueue(msg.clone());
//         println!("Recv: {}", msg);
//         println!("Size: {}", queue.len());
//         println!("{:?}", queue.slice());
//     }
// }

use std::{time::Duration};

use notify::{RecursiveMode, Watcher};
use notify::event::{EventKind, CreateKind, ModifyKind, RenameMode, MetadataKind};
use notify_debouncer_full::new_debouncer;
// use tempfile::tempdir;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // setup debouncer
    let (tx, rx) = std::sync::mpsc::channel();

    // no specific tickrate, max debounce time 1 seconds
    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx)?;

    let file_path = Path::new("./public");
    debouncer
        .watcher()
        .watch(file_path, RecursiveMode::Recursive)?;

    debouncer
        .cache()
        .add_root(file_path, RecursiveMode::Recursive);

    // print all events and errors
    for result in rx {
        match result {
            Ok(events) => events.iter().for_each(|event| {
                println!("{event:?}");
                if let EventKind::Create(CreateKind::File) = event.kind {
                    println!("[Watcher] Create File");
                    println!("{:?}", event.paths);
                } else if let EventKind::Modify(ModifyKind::Metadata(MetadataKind::Any)) = event.kind {
                    println!("[Watcher] Modify Metadata");
                    println!("{:?}", event.paths);
                } else if let EventKind::Modify(ModifyKind::Name(RenameMode::Both)) = event.kind {
                    println!("[Watcher] Name Changed: Both");
                    println!("{:?}", event.paths);
                } else if let EventKind::Modify(ModifyKind::Name(RenameMode::Any)) = event.kind {
                    println!("[Watcher] Name Changed: Any");
                }/* else if let EventKind::Modify(ModifyKind::Metadata(MetadataKind::Any)) = event.kind {
                    println!("[Watcher] Modify Metadata");
                } else if let EventKind::Modify(ModifyKind::Name(RenameMode::Both)) = event.kind {
                    println!("[Watcher] Name Changed: Both");
                } else if let EventKind::Modify(ModifyKind::Name(RenameMode::Any)) = event.kind {
                    println!("[Watcher] Name Changed: Any");
                }*/
            }),
            Err(errors) => errors.iter().for_each(|error| {
                println!("{error:?}")
            }),
        }
        println!();
    }

    Ok(())
}

