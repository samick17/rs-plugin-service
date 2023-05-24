use so_runner;
use wasm_runner;

fn main() {
    so_runner::execute_so("./libs/liblib1.so", "handler");
    wasm_runner::execute_wasm("./libs/lib1.wasm", "handler");
}
