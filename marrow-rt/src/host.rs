use wasmi::{Module, ModuleInstance, ModuleRef};

use alloc::vec::Vec;

pub struct Host {
    modules: Vec<Module>,
}

impl Host {
    // pub fn from_bytes<B: AsRef<[u8]>>(bytes: &B) -> Self {
    //     let
    // }
}
