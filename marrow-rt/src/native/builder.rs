use wasmi::{Externals, ModuleRef, RuntimeArgs, RuntimeValue, Trap};

use super::NativeModuleRef;

use alloc::collections::BTreeMap;

pub struct ExternalsBuilder<'a> {
    pub(crate) tree: BTreeMap<usize, &'a NativeModuleRef>,
    pub(crate) instance: ModuleRef,
    pub(crate) end: usize,
}

impl<'a> ExternalsBuilder<'a> {
    pub fn new(instance: ModuleRef) -> Self {
        ExternalsBuilder {
            tree: BTreeMap::new(),
            end: 0,
            instance,
        }
    }
    pub fn with_resolver(mut self, resolver: &'a NativeModuleRef) -> Self {
        self.tree.insert(resolver.offset, resolver);
        self.end = resolver.offset;
        self
    }
}

impl<'a> Externals for ExternalsBuilder<'a> {
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
        Ok(f(&self.instance, args.as_ref()))
    }
}
