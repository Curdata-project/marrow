#![no_std]

pub use wasmi::RuntimeValue;
pub use wasmi::ValueType;

extern crate alloc;

// declare trait of host;
pub mod host;

pub mod native;
