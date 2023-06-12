use std::thread::{spawn};
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};
use std::sync::mpsc::{channel, Sender};
use std::collections::{HashMap};
use std::net::{TcpListener, TcpStream};

use cli::{get_input};

fn handle_client(id: i32, stream: TcpStream, tx_signal: Sender::<i32>) {
	let mut stream_read = stream.try_clone().unwrap();
	spawn(move || {
		println!("[new conn]: {id}");
		let mut buffer = [0; 1024];
		loop {
			match stream_read.read(&mut buffer) {
				Ok(0) => {
					println!("Disconnected!");
					tx_signal.send(id).unwrap();
					return;
				},
				Ok(size) => {
					let temp_buffer = buffer[..size].to_vec();
					let received = String::from_utf8_lossy(&temp_buffer);
	    			println!("[Received] {size}: {}", received);
				},
				Err(_) => {
					println!("Error!!");
					return;
				},
			}
		}
	});
}

pub fn main() {
	let connections = Arc::new(Mutex::new(HashMap::<i32, TcpStream>::new()));
	
	let (tx_signal, rx_signal) = channel();

	let connections_cloned = connections.clone();
	spawn(move || {
		loop {
			let text = get_input();
			println!("> {text}");
			for (_, mut stream) in connections_cloned.lock().unwrap().iter() {
				stream.write(text.as_bytes()).unwrap();
			}
		}
	});

	let connections_cloned2 = connections.clone();
	spawn(move || {
		loop {
			let conn_id: i32 = rx_signal.recv().unwrap();
			// let conn_id = received.clone();
			println!("Revv!!!{conn_id}");
			connections_cloned2.lock().unwrap().remove(&conn_id);
			println!("[Count] {}", connections_cloned2.lock().unwrap().len());
		}
	});

	let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address");

	let mut serial_id = 1;
    // accept connections and process them serially
    for stream in listener.incoming() {
    	match stream {
    		Ok(stream) => {
    			let new_id = serial_id;
    			connections.lock().unwrap().insert(new_id, stream.try_clone().unwrap());
		        handle_client(new_id, stream, tx_signal.clone());
		        serial_id += 1;
    		},
    		Err(_) => {

    		},
    	}
    }
}
