#![no_std]
#![feature(map_first_last)]

// use alloc level.
extern crate alloc;

pub use wasmi::RuntimeValue;
pub use wasmi::ValueType;

mod runtime;
pub use runtime::{Runtime, StartFunctionName};

mod module;
pub use module::{Module, ModuleRef, NativeFunc};

mod error;
pub use error::Error;
