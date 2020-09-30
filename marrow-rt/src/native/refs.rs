use crate::{Error, NativeInstance};
use wasmi::{FuncInstance, FuncRef, ModuleImportResolver, ModuleRef, Signature};

#[derive(Clone, Debug)]
pub struct NativeModuleRef {
    pub(crate) module: NativeInstance,
    pub(crate) refs: Option<ModuleRef>,
    pub(crate) offset: usize,
}

impl NativeModuleRef {
    fn resolve_func_index(&self, field_name: &str) -> Result<usize, Error> {
        let funcs = self.module.instance.funcs;
        match funcs.binary_search_by(|func| func.name.cmp(field_name)) {
            Ok(offset) => Ok(offset),
            Err(_) => Err(Error::NoMethod),
        }
    }
}

impl ModuleImportResolver for NativeModuleRef {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &Signature,
    ) -> Result<FuncRef, wasmi::Error> {
        let i = self.resolve_func_index(field_name)?;
        log::info!(
            "resolve funcs is {}, offset is: {}, i is: {}",
            field_name,
            self.offset,
            i
        );
        Ok(FuncInstance::alloc_host(
            _signature.clone(),
            i + self.offset,
        ))
    }
}
