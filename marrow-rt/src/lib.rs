pub use wasmi::ValueType;

extern crate alloc;

mod modules;
pub use modules::Modules;

mod wasm_module;
pub use wasm_module::WasmModule;
