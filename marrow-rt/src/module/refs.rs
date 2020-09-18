use crate::{Error, ExternalsBuilder, Module, RuntimeValue, UnionModule};
use alloc::rc::Rc;
use wasmi::{
    FuncInstance, FuncRef, GlobalDescriptor, GlobalRef, MemoryDescriptor, MemoryRef,
    ModuleImportResolver, Signature, TableDescriptor, TableRef,
};

/// Start function name for wasm module.
pub enum StartFunctionName {
    /// Use special function as start function.
    Function(&'static str),
    /// Use module's start function. Inital module failed if no start function in module.
    Section,
    /// Don't use module's start function. Even if module has start function.
    NoStart,
}

#[derive(Debug)]
pub enum UnionRef {
    WasmRef { refs: wasmi::ModuleRef },
    NativeRef { offset: usize },
}

#[derive(Debug)]
pub struct ModuleRef {
    pub(crate) module: Rc<Module>,
    pub(crate) union: UnionRef,
}

impl ModuleRef {
    fn resolve_func_index(&self, field_name: &str) -> Result<usize, Error> {
        let mut i = 0;
        let funcs = match self.module.module {
            UnionModule::NativeModule { funcs } => funcs,
            _ => return Err(Error::ModuleTypeError),
        };
        let func = &funcs[i];
        let funcs_len = funcs.len();
        while func.name != field_name && i <= funcs_len {
            i += 1;
        }
        // check `i` is resloveable.
        if i > funcs_len {
            Err(Error::NoMethod)
        } else {
            Ok(i)
        }
    }

    pub fn invoke_export(
        &self,
        func_name: &str,
        args: &[RuntimeValue],
        external: &mut ExternalsBuilder,
    ) -> Result<Option<RuntimeValue>, Error> {
        match &self.union {
            UnionRef::WasmRef { refs } => Ok(refs.invoke_export(func_name, args, external)?),
            _ => Err(Error::NoMethod),
        }
    }
}

impl ModuleImportResolver for ModuleRef {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &Signature,
    ) -> Result<FuncRef, wasmi::Error> {
        let func_ref = match &self.union {
            UnionRef::WasmRef { refs } => refs.resolve_func(field_name, _signature)?,
            UnionRef::NativeRef { offset: _ } => {
                let i = self.resolve_func_index(field_name)?;
                FuncInstance::alloc_host(_signature.clone(), i)
            }
        };
        Ok(func_ref)
    }

    fn resolve_global(
        &self,
        field_name: &str,
        _global_type: &GlobalDescriptor,
    ) -> Result<GlobalRef, wasmi::Error> {
        match &self.union {
            UnionRef::WasmRef { refs } => Ok(refs.resolve_global(field_name, _global_type)?),
            UnionRef::NativeRef { offset: _ } => Err(Error::ModuleTypeError)?,
        }
    }

    fn resolve_memory(
        &self,
        field_name: &str,
        _memory_type: &MemoryDescriptor,
    ) -> Result<MemoryRef, wasmi::Error> {
        match &self.union {
            UnionRef::WasmRef { refs } => Ok(refs.resolve_memory(field_name, _memory_type)?),
            UnionRef::NativeRef { offset: _ } => Err(Error::ModuleTypeError)?,
        }
    }

    fn resolve_table(
        &self,
        field_name: &str,
        _table_type: &TableDescriptor,
    ) -> Result<TableRef, wasmi::Error> {
        match &self.union {
            UnionRef::WasmRef { refs } => Ok(refs.resolve_table(field_name, _table_type)?),
            UnionRef::NativeRef { offset: _ } => Err(Error::ModuleTypeError)?,
        }
    }
}
