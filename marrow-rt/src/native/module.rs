use super::NativeFunc;
use crate::{Error, NativeInstance};
// use wasmi::{FuncInstance, FuncRef, ModuleImportResolver, Signature};
use alloc::rc::Rc;

#[derive(Debug)]
pub struct NativeModule {
    pub(crate) funcs: &'static [NativeFunc],
    pub(crate) name: &'static str,
}

impl NativeModule {
    pub fn from_funcs(
        name: &'static str,
        funcs: &'static [NativeFunc],
    ) -> Result<NativeModule, Error> {
        Ok(NativeModule { funcs, name })
    }

    pub fn offset(&self) -> usize {
        self.funcs.len()
    }

    pub(crate) fn alloc_module(self) -> NativeInstance {
        NativeInstance {
            instance: Rc::new(self),
        }
    }
}
