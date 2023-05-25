use std::io::{stdin};

pub fn get_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
