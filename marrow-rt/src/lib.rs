#![no_std]
#![feature(map_first_last)]

pub use wasmi::RuntimeValue;
pub use wasmi::ValueType;

extern crate alloc;

mod native;
pub(crate) use native::ExternalsBuilder;
pub use native::{NativeFunc, NativeModule, NativeModuleRef};

mod wasm;
pub use wasm::{StartFunctionName, WasmModule, WasmModuleBuilder, WasmModuleRef};
