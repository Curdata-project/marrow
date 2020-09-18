#![no_std]
#![feature(map_first_last)]

pub use wasmi::RuntimeValue;
pub use wasmi::ValueType;

extern crate alloc;

mod native;
pub(crate) use native::ExternalsBuilder;
pub use native::NativeFunc;
// pub use native::{NativeFunc, NativeModule, NativeModuleRef};

// mod wasm;
// pub use wasm::{StartFunctionName, WasmModule, WasmModuleBuilder, WasmModuleRef};

mod runtime;
pub use runtime::Runtime;

mod module;
pub use module::{Module, ModuleRef, StartFunctionName};
pub(crate) use module::{UnionModule, UnionRef};

use core::fmt;

#[derive(Debug)]
pub enum Error {
    NoRuningModule,
    NoMethod,
    ModuleTypeError,
    InnerError(wasmi::Error),
}

impl From<wasmi::Error> for Error {
    fn from(e: wasmi::Error) -> Error {
        Error::InnerError(e)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl wasmi::HostError for Error {}
