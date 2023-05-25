// use so_runner;
// use wasm_runner;

// fn main() {
//     so_runner::execute_so("./libs/liblib1.so", "handler");
//     wasm_runner::execute_wasm("./libs/lib1.wasm", "handler");
// }
use std::sync::mpsc::{channel};
use std::thread::{spawn};

use cli::{get_input};
use queue::{Queue};

fn main() {
    let (tx, rx) = channel::<String>();
    let mut queue: Queue<String> = Queue::new();
    let tx_clone = tx.clone();
    spawn(move || {
        loop {
            let text = get_input();
            println!("> {}", text);
            tx_clone.send(String::from("asdfg")).unwrap();
        }
    });

    while let Ok(msg) = rx.recv() {
        queue.enqueue(msg.clone());
        println!("Recv: {}", msg);
        println!("Size: {}", queue.len());
    }
    

    // queue.enqueue(10);
    // queue.enqueue(20);
    // queue.enqueue(30);
    // queue.enqueue(30);

    // while !queue.is_empty() {
    //     println!("Dequeued item: {:?}", queue.dequeue());
    // }
}

