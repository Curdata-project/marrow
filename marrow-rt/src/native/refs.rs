use super::NativeModule;
use alloc::format;
use alloc::rc::Rc;
use bitvec::prelude::*;
use wasmi::{Error, FuncInstance, FuncRef, ModuleImportResolver, Signature};

/// Reference of `NativeModule`.
#[derive(Clone, Debug)]
pub struct NativeModuleRef {
    pub(crate) native: Rc<NativeModule>,
    pub(crate) offset: usize,
    pub(crate) permission: BitVec<LocalBits, u64>,
}

impl NativeModuleRef {
    /// Create reference form `NativeModule`.
    pub fn new(loaded_module: NativeModule, offset: usize) -> NativeModuleRef {
        NativeModuleRef {
            native: loaded_module.alloc_module(),
            offset,
            permission: BitVec::new(),
        }
    }

    /// Get begin index of these module.
    pub fn begin_index(&self) -> usize {
        self.offset
    }

    /// Get end index of these module.
    pub fn end_index(&self) -> usize {
        self.offset + self.native.funcs.len()
    }
}

impl ModuleImportResolver for NativeModuleRef {
    fn resolve_func(&self, field_name: &str, _signature: &Signature) -> Result<FuncRef, Error> {
        let mut i = 0;
        let func = &self.native.funcs[i];
        let funcs_len = self.native.funcs.len();
        while func.name != field_name && i <= funcs_len {
            i += 1;
        }
        // check `i` is resloveable.
        if i > funcs_len {
            Err(Error::Instantiation(format!("id is exceed.")))
        } else {
            let func_ref = FuncInstance::alloc_host(_signature.clone(), i);
            Ok(func_ref)
        }
    }
}