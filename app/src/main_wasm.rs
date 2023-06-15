use wasm_runner::{WasmRuntime};

pub fn main() {
	let file_path = "./libs/lib1.wasm";
	let value: i32 = 2;
	let mut runtime = WasmRuntime::<i32>::create(file_path, "handler", value);

	let add = runtime.exec::<(), u32>();
	println!("Result: {}", add.call(&mut runtime.store, ()).unwrap());

	runtime.reload();

	let add2 = runtime.exec::<(), u32>();
	println!("Result: {}", add2.call(&mut runtime.store, ()).unwrap());
}
