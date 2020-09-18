use wasmi::Module;

/// Explain `Webassembly` of modules.
pub struct WasmModule {
    pub(crate) module: Module,
    pub name: &'static str,
}

impl WasmModule {
    pub fn from_buffer<B: AsRef<[u8]>>(name: &'static str, buffer: B) -> Self {
        let module = Module::from_buffer(buffer).unwrap();
        WasmModule { module, name }
    }
}
