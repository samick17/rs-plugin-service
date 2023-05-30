use std::sync::mpsc::{channel};
use std::thread::{spawn};

use cli::{get_input};
use queue::{Queue};

pub fn main() {
    let (tx, rx) = channel::<String>();
    let mut queue: Queue<String> = Queue::new();
    let tx_clone = tx.clone();
    spawn(move || {
        loop {
            let text = get_input();
            println!("> {}", text);
            tx_clone.send(text).unwrap();
        }
    });
    while let Ok(msg) = rx.recv() {
        queue.enqueue(msg.clone());
        println!("Recv: {}", msg);
        println!("Size: {}", queue.len());
        println!("{:?}", queue.slice());
    }
}
