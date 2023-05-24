use libloading::{Library, Symbol};

fn main() {
    unsafe {
        // Load the shared library
        let lib = Library::new("libs/liblib1.so").expect("Failed to load library");
        // Get a function symbol from the library
        let func: Symbol<unsafe extern "C" fn(usize, usize) -> usize> = lib.get(b"add").expect("Failed to get symbol");
        // Call the function
        println!("{:?}", func(1 as usize, 2 as usize));
        // Unload the library
        drop(lib);
    }

    unsafe {
        // Load the shared library
        let lib = Library::new("libs/liblib1.so").expect("Failed to load library");
        // Get a function symbol from the library
        let func: Symbol<unsafe extern "C" fn()> = lib.get(b"foo").expect("Failed to get symbol");
        // Call the function
        func();
        // Unload the library
        drop(lib);
    }

    unsafe {
        // Load the shared library
        let lib = Library::new("libs/liblib1.so").expect("Failed to load library");
        // Get a function symbol from the library
        let func: Symbol<unsafe extern "C" fn() -> String> = lib.get(b"bar").expect("Failed to get symbol");
        // Call the function
        let ret = func();
        println!("{}", ret);
        // Unload the library
        drop(lib);
    }
}