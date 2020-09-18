use super::NativeModuleRef;
use alloc::collections::BTreeMap;
use bitvec::prelude::*;
use wasmi::{Externals, ModuleRef, RuntimeArgs, RuntimeValue, Trap};

#[derive(Debug)]
pub struct ExternalsBuilder {
    pub(crate) tree: BTreeMap<usize, NativeModuleRef>,
    pub(crate) instance: Option<ModuleRef>,
    pub(crate) end: usize,
    pub(crate) permission: BitVec<LocalBits, u64>,
}

impl Default for ExternalsBuilder {
    fn default() -> Self {
        ExternalsBuilder {
            tree: BTreeMap::new(),
            end: 0,
            instance: None,
            permission: BitVec::new(),
        }
    }
}

impl ExternalsBuilder {
    pub fn push_resolver(&mut self, mut resolver: NativeModuleRef) {
        log::info!(
            "NativeModule {} offset is: {} at external.",
            resolver.native.name,
            self.end,
        );
        resolver.offset = self.end;
        self.end = resolver.offset + self.end;
        self.tree.insert(resolver.offset, resolver);
    }

    pub(crate) fn push_module(&mut self, module: ModuleRef) {
        self.instance = Some(module);
    }
}

impl Externals for ExternalsBuilder {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        // find module.
        if index > self.end {
            // Error and close these machine.
            panic!("Unimplemented function at {}", index);
        }

        // check permission.
        let mut r = self.tree.split_off(&index);
        // TODO: process index.
        let (mindex, module) = self.tree.pop_last().unwrap();
        log::info!("call funcs index is {}", mindex);
        let real_index = index - module.offset;
        let f = module.native.funcs[real_index].func;
        // Insert back
        self.tree.insert(index, module);
        self.tree.append(&mut r);
        Ok(f(&self.instance.as_ref().unwrap(), args.as_ref()))
    }
}