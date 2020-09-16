use wasmi::{Externals, ModuleRef, RuntimeArgs, RuntimeValue, Trap};

use super::NativeModuleRef;

use alloc::collections::BTreeMap;

pub struct ExternalsBuilder {
    pub(crate) tree: BTreeMap<usize, NativeModuleRef>,
    pub(crate) instance: Option<ModuleRef>,
    pub(crate) end: usize,
}

impl Default for ExternalsBuilder {
    fn default() -> Self {
        ExternalsBuilder {
            tree: BTreeMap::new(),
            end: 0,
            instance: None,
        }
    }
}

impl ExternalsBuilder {
    pub fn with_resolver(mut self, resolver: NativeModuleRef) -> Self {
        self.push_resolver(resolver);
        self
    }

    pub fn push_resolver(&mut self, resolver: NativeModuleRef) {
        self.end = resolver.offset;
        self.tree.insert(resolver.offset, resolver);
    }

    pub fn with_module(mut self, module: ModuleRef) -> Self {
        self.instance = Some(module);
        self
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
