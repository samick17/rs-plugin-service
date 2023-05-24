#[no_mangle]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[no_mangle]
pub fn foo() {
    println!("foo");
}

#[no_mangle]
pub fn bar() -> String {
    String::from("asdfg")
}

#[no_mangle]
pub fn handler() -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
