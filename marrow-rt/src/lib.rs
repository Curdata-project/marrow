#![no_std]
#![feature(map_first_last)]

extern crate alloc;

pub use wasmi::RuntimeValue;
pub use wasmi::ValueType;

mod error;
pub use error::Error;

mod wasm;
pub use wasm::{WasmModule, WasmModuleRef};

mod native;
pub use native::{NativeFunc, NativeInstance, NativeModule, NativeModuleRef};

mod runtime;
pub use runtime::{ModuleName, Runtime, StartFunctionName};
