// use so_runner;
// use wasm_runner;

// fn main() {
//     so_runner::execute_so("./libs/liblib1.so", "handler");
//     wasm_runner::execute_wasm("./libs/lib1.wasm", "handler");
// }
use std::io::{stdin};
use std::sync::mpsc::{channel};
use std::thread::{spawn};

pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { items: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn dequeue(&mut self) -> T {
        self.items.remove(0)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}

fn main() {
    let (tx, rx) = channel::<String>();
    let mut queue: Queue<String> = Queue::new();
    let tx_clone = tx.clone();
    spawn(move || {
        loop {
            let text = get_input();
            println!("> {}", text);
            tx_clone.send(String::from("asdfg"));
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

