use wasmtime::*;
use libloading::{Library, Symbol};

fn execute_wasm() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::from_file(&engine, "./libs/lib1.wasm").unwrap();
    let instance = Instance::new(&mut store, &module, &[]).unwrap();
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add").unwrap();
    println!("Result: {}", add.call(&mut store, (1, 2)).unwrap());
}

fn execute_so() {
    unsafe {
        // Load the shared library
        let lib = Library::new("./libs/liblib1.so").expect("Failed to load library");
        // Get a function symbol from the library
        let func: Symbol<unsafe extern "C" fn(i32, i32) -> i32> = lib.get(b"add").expect("Failed to get symbol");
        // Call the function
        println!("{:?}", func(1, 2));
        // Unload the library
        drop(lib);
    }
}

fn main() {
    execute_so();
    execute_wasm();
}
