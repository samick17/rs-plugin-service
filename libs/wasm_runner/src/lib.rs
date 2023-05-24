use wasmtime::*;

pub fn execute_wasm(file_path: &str, fn_name: &str) {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::from_file(&engine, file_path).unwrap();
    let instance = Instance::new(&mut store, &module, &[]).unwrap();
    let add = instance.get_typed_func::<(), u32>(&mut store, fn_name).unwrap();
    println!("Result: {}", add.call(&mut store, ()).unwrap());
}
