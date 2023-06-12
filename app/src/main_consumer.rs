// use std::thread::{spawn};
use cli::{get_input};

pub fn main() {

	loop {
		let text = get_input();
		println!("> {text}");
	}
	// spawn(|| {

	// });

}
