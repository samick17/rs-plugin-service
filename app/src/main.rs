use wasmtime::*;
use anyhow::Result;

fn main() -> Result<()> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::from_file(&engine, "./libs/lib1.wasm")?;
    let instance = Instance::new(&mut store, &module, &[])?;
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    println!("Result: {}", add.call(&mut store, (1, 2)).unwrap());
    Ok(())
}

// use libloading::{Library, Symbol};

// fn main() {
//     unsafe {
//         // Load the shared library
//         let lib = Library::new("libs/liblib1.so").expect("Failed to load library");
//         // Get a function symbol from the library
//         let func: Symbol<unsafe extern "C" fn(usize, usize) -> usize> = lib.get(b"add").expect("Failed to get symbol");
//         // Call the function
//         println!("{:?}", func(1 as usize, 2 as usize));
//         // Unload the library
//         drop(lib);
//     }

//     unsafe {
//         // Load the shared library
//         let lib = Library::new("libs/liblib1.so").expect("Failed to load library");
//         // Get a function symbol from the library
//         let func: Symbol<unsafe extern "C" fn()> = lib.get(b"foo").expect("Failed to get symbol");
//         // Call the function
//         func();
//         // Unload the library
//         drop(lib);
//     }

//     unsafe {
//         // Load the shared library
//         let lib = Library::new("libs/liblib1.so").expect("Failed to load library");
//         // Get a function symbol from the library
//         let func: Symbol<unsafe extern "C" fn() -> String> = lib.get(b"bar").expect("Failed to get symbol");
//         // Call the function
//         let ret = func();
//         println!("{}", ret);
//         // Unload the library
//         drop(lib);
//     }
// }