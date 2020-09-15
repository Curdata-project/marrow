#![no_std]
#![feature(map_first_last)]

pub use wasmi::RuntimeValue;
pub use wasmi::ValueType;

extern crate alloc;

// declare trait of host;
pub mod host;

pub mod module;
pub mod wasm;

mod native;
pub use self::native::{ExternalsBuilder, NativeFunc, NativeModule, NativeModuleRef};
