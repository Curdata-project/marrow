use crate::{Error, NativeFunc};
use core::{fmt, fmt::Debug};

pub(crate) enum UnionModule {
    NativeModule { funcs: &'static [NativeFunc] },
    WasmModule { module: wasmi::Module },
}

impl Debug for UnionModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "...")
    }
}

#[derive(Debug)]
pub struct Module {
    pub(crate) name: &'static str,
    pub(crate) module: UnionModule,
}

impl Module {
    pub fn from_buffer<B: AsRef<[u8]>>(name: &'static str, buffer: B) -> Result<Module, Error> {
        let module = wasmi::Module::from_buffer(buffer).unwrap();
        let union_module = UnionModule::WasmModule { module };
        Ok(Module {
            name,
            module: union_module,
        })
    }

    pub fn from_funcs(name: &'static str, funcs: &'static [NativeFunc]) -> Result<Module, Error> {
        let module = UnionModule::NativeModule { funcs };
        Ok(Module { name, module })
    }
}
