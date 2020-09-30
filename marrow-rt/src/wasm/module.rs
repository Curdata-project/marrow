use crate::Error;
use core::fmt::{self, Debug};
use wasmi::Module;

pub struct WasmModule {
    pub(crate) module: Module,
    pub(crate) name: &'static str,
}

impl Debug for WasmModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[WasmModule name: {}]", self.name)
    }
}

impl WasmModule {
    pub fn from_bytes<B: AsRef<[u8]>>(name: &'static str, bytes: B) -> Result<WasmModule, Error> {
        let module = Module::from_buffer(bytes)?;
        Ok(WasmModule { module, name })
    }
}
