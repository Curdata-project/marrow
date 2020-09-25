use super::UnionModule;
use crate::{Error, Module, RuntimeValue};
use alloc::rc::Rc;
use wasmi::{
    Externals, FuncInstance, FuncRef, GlobalDescriptor, GlobalRef, MemoryDescriptor, MemoryRef,
    ModuleImportResolver, Signature, TableDescriptor, TableRef,
};

#[derive(Debug, Clone)]
pub enum UnionRef {
    WasmRef {
        refs: wasmi::ModuleRef,
    },
    NativeRef {
        offset: usize,
        refs: wasmi::ModuleRef,
    },
}

#[derive(Debug, Clone)]
pub struct ModuleRef {
    pub(crate) module: Rc<Module>,
    pub(crate) union: UnionRef,
}

impl ModuleRef {
    // reslove index by field name.
    fn resolve_func_index(&self, field_name: &str) -> Result<usize, Error> {
        if let UnionModule::NativeModule { funcs } = self.module.module {
            // get i of field name.
            match funcs.binary_search_by(|func| func.name.cmp(field_name)) {
                Ok(offset) => Ok(offset),
                Err(_) => Err(Error::NoMethod),
            }
        } else {
            Err(Error::ModuleTypeError)
        }
    }

    pub fn invoke_export_wasm<E: Externals>(
        &self,
        func_name: &str,
        args: &[RuntimeValue],
        external: &mut E,
    ) -> Result<Option<RuntimeValue>, Error> {
        match &self.union {
            UnionRef::WasmRef { refs } => Ok(refs.invoke_export(func_name, args, external)?),
            _ => Err(Error::NoMethod),
        }
    }

    pub fn invoke_export_native(
        &self,
        index: usize,
        args: &[RuntimeValue],
    ) -> Result<Option<RuntimeValue>, Error> {
        if let UnionRef::NativeRef { offset: _, refs } = &self.union {
            if let UnionModule::NativeModule { funcs } = self.module.module {
                let func = funcs[index].func;
                Ok(func(&refs, args))
            } else {
                Err(Error::ModuleTypeError)
            }
        } else {
            Err(Error::ModuleTypeError)
        }
    }

    pub fn get_native_offset(&self) -> usize {
        10
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
            UnionRef::NativeRef { offset, refs: _ } => {
                let i = self.resolve_func_index(field_name)?;
                FuncInstance::alloc_host(_signature.clone(), i + offset)
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
            UnionRef::NativeRef { offset: _, refs: _ } => Err(Error::ModuleTypeError)?,
        }
    }

    fn resolve_memory(
        &self,
        field_name: &str,
        _memory_type: &MemoryDescriptor,
    ) -> Result<MemoryRef, wasmi::Error> {
        match &self.union {
            UnionRef::WasmRef { refs } => Ok(refs.resolve_memory(field_name, _memory_type)?),
            UnionRef::NativeRef { offset: _, refs: _ } => Err(Error::ModuleTypeError)?,
        }
    }

    fn resolve_table(
        &self,
        field_name: &str,
        _table_type: &TableDescriptor,
    ) -> Result<TableRef, wasmi::Error> {
        match &self.union {
            UnionRef::WasmRef { refs } => Ok(refs.resolve_table(field_name, _table_type)?),
            UnionRef::NativeRef { offset: _, refs: _ } => Err(Error::ModuleTypeError)?,
        }
    }
}
