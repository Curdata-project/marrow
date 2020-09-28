use crate::Error;
use wasmi::{
    Externals, FuncRef, GlobalDescriptor, GlobalRef, MemoryDescriptor, MemoryRef,
    ModuleImportResolver, ModuleRef, RuntimeValue, Signature, TableDescriptor, TableRef,
};

#[derive(Clone, Debug)]
pub struct WasmModuleRef {
    pub(crate) refs: ModuleRef,
}

impl WasmModuleRef {
    pub fn invoke_export_wasm<E: Externals>(
        &self,
        func_name: &str,
        args: &[RuntimeValue],
        external: &mut E,
    ) -> Result<Option<RuntimeValue>, Error> {
        Ok(self.refs.invoke_export(func_name, args, external)?)
    }
}

impl ModuleImportResolver for WasmModuleRef {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &Signature,
    ) -> Result<FuncRef, wasmi::Error> {
        self.refs.resolve_func(field_name, _signature)
    }

    fn resolve_global(
        &self,
        field_name: &str,
        _global_type: &GlobalDescriptor,
    ) -> Result<GlobalRef, wasmi::Error> {
        self.refs.resolve_global(field_name, _global_type)
    }

    fn resolve_memory(
        &self,
        field_name: &str,
        _memory_type: &MemoryDescriptor,
    ) -> Result<MemoryRef, wasmi::Error> {
        self.refs.resolve_memory(field_name, _memory_type)
    }

    fn resolve_table(
        &self,
        field_name: &str,
        _table_type: &TableDescriptor,
    ) -> Result<TableRef, wasmi::Error> {
        self.refs.resolve_table(field_name, _table_type)
    }
}
