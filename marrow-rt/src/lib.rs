#![no_std]
#![feature(map_first_last)]

pub use wasmi::RuntimeValue;
pub use wasmi::ValueType;

extern crate alloc;

mod native;
pub use native::{ExternalsBuilder, NativeFunc, NativeModule, NativeModuleRef};

mod wasm;
pub use wasm::{StartFunctionName, WasmModule, WasmModuleBuilder, WasmModuleRef};
