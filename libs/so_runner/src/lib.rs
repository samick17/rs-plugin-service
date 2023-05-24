use libloading::{Library, Symbol};

pub fn execute_so(file_path: &str, fn_name: &str) {
    unsafe {
        // Load the shared library
        let lib = Library::new(file_path).expect("Failed to load library");
        // Get a function symbol from the library
        let func: Symbol<unsafe extern "C" fn() -> u32> = lib.get(fn_name.as_bytes()).expect("Failed to get symbol");
        // Call the function
        println!("Result: {:?}", func());
        // Unload the library
        drop(lib);
    }
}
