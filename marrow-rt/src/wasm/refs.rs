use crate::ExternalsBuilder;
use wasmi::{
    Error, FuncRef, GlobalDescriptor, GlobalRef, MemoryDescriptor, MemoryRef, ModuleImportResolver,
    ModuleRef, RuntimeValue, Signature, TableDescriptor, TableRef,
};

use core::cell::RefCell;

/// Reference of WasmModule.
pub struct WasmModuleRef {
    pub(crate) refs: ModuleRef,
    pub(crate) external: RefCell<ExternalsBuilder>,
}

impl WasmModuleRef {
    /// invoke export function.
    pub fn invoke_export(
        &self,
        func_name: &str,
        args: &[RuntimeValue],
    ) -> Result<Option<RuntimeValue>, Error> {
        let mut external = self.external.try_borrow_mut().unwrap();
        self.refs.invoke_export(func_name, args, &mut *external)
    }
}

impl ModuleImportResolver for WasmModuleRef {
    fn resolve_func(&self, field_name: &str, _signature: &Signature) -> Result<FuncRef, Error> {
        self.refs.resolve_func(field_name, _signature)
    }

    fn resolve_global(
        &self,
        field_name: &str,
        _global_type: &GlobalDescriptor,
    ) -> Result<GlobalRef, Error> {
        self.refs.resolve_global(field_name, _global_type)
    }

    fn resolve_memory(
        &self,
        field_name: &str,
        _memory_type: &MemoryDescriptor,
    ) -> Result<MemoryRef, Error> {
        self.refs.resolve_memory(field_name, _memory_type)
    }

    fn resolve_table(
        &self,
        field_name: &str,
        _table_type: &TableDescriptor,
    ) -> Result<TableRef, Error> {
        self.refs.resolve_table(field_name, _table_type)
    }
}
