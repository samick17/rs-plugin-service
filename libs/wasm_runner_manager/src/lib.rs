use std::collections::HashMap;
use std::cell::RefCell;
use wasm_runner;

// Example for multiple wasm runtimes management
// Use default hosted-state: i32 in sample code.
pub struct WasmManager {
    runtimes: HashMap<String, RefCell<wasm_runner::WasmRuntime<i32>>>,
}

impl WasmManager {
    pub fn load_plugin(&mut self, name: &str, file_path: &str, fn_name: &str) {
        let runtime = wasm_runner::WasmRuntime::<i32>::create(file_path, fn_name, 0);
        self.runtimes.insert(name.to_owned(), RefCell::new(runtime));
    }

    // Example for execute plugin implementation
    pub fn exec_plugin(&mut self, name: &str) {
        match self.runtimes.get(name) {
            Some(plugin) => {
                let mut runtime = plugin.borrow_mut();
                let add = runtime.exec::<(), u32>();
                let result = add.call(&mut runtime.store, ()).unwrap();
                println!("[Exec] Result: {:?}", result);
            },
            None => {
                println!("None!");
            },
        }
    }
}

pub fn create_manager() -> WasmManager {
    WasmManager{
        runtimes: HashMap::new(),
    }
}
