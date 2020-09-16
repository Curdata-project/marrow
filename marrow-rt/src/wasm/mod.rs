mod module;
pub use module::WasmModule;

mod refs;
pub use refs::WasmModuleRef;

mod builder;
pub use builder::{StartFunctionName, WasmModuleBuilder};
