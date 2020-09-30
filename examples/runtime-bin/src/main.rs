use marrow_rt::{ModuleName, Runtime, StartFunctionName, WasmModule};
use std::fs::read;

fn main() {
    env_logger::init();
    let mut runtime = Runtime::default();
    runtime.run_native(native_maths::entry()).unwrap();
    runtime.run_native(native_prints::entry()).unwrap();
    let data =
        read("examples/wasm-add-print/target/wasm32-unknown-unknown/release/wasm_add_print.wasm")
            .unwrap();
    let module = WasmModule::from_bytes("hello", data).unwrap();
    runtime
        .run_wasm(
            module,
            StartFunctionName::NoStart,
            &[ModuleName::Native("prints"), ModuleName::Native("maths")],
        )
        .unwrap();
    runtime.invoke_export("hello", "entry", &[]).unwrap();
    let r2 = runtime.invoke_export("hello", "entry2", &[]).unwrap();
    println!("{:#?}", r2);
}
