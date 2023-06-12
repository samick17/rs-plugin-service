use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::{spawn};

use cli::{get_input};

pub fn main() {

	let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

	let mut stream_write = stream.try_clone().unwrap();

	spawn(move|| {
		let mut buffer = [0; 1024];
		loop {
			match stream.read(&mut buffer) {
				Ok(0) => {
					println!("Disconnected!");
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
    // stream.write(&[1])?;
    // stream.read(&mut [0; 128])?;

	loop {
		let text = get_input();
		println!("> {text}");
		stream_write.write(text.as_bytes()).unwrap();
	}
	// spawn(|| {

	// });

}
