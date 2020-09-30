use crate::{
    Error, ModuleName, NativeInstance, NativeModule, NativeModuleRef, RuntimeValue,
    StartFunctionName, WasmModule, WasmModuleRef,
};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::cell::RefCell;
use wasmi::{Externals, ImportsBuilder, ModuleInstance, RuntimeArgs, Trap};

pub struct RuntimeExternal {
    pub(crate) native_mods: BTreeMap<&'static str, NativeInstance>,
    pub(crate) native_refs: BTreeMap<usize, NativeModuleRef>,
}

impl Default for RuntimeExternal {
    fn default() -> Self {
        RuntimeExternal {
            native_mods: BTreeMap::new(),
            native_refs: BTreeMap::new(),
        }
    }
}

pub struct Runtime {
    pub(crate) offset: usize,
    pub(crate) external: RefCell<RuntimeExternal>,
    pub(crate) wasm_refs: BTreeMap<&'static str, WasmModuleRef>,
}

impl Default for Runtime {
    fn default() -> Self {
        let external = RefCell::new(RuntimeExternal::default());
        Runtime {
            wasm_refs: BTreeMap::new(),
            offset: 0,
            external,
        }
    }
}

impl Runtime {
    pub fn invoke_export(
        &self,
        module: &str,
        func_name: &str,
        args: &[RuntimeValue],
    ) -> Result<Option<RuntimeValue>, Error> {
        // let external = self.external.try_borrow()?;
        let mut_external = &mut *self.external.try_borrow_mut()?;
        let module = self.wasm_refs.get(module).unwrap();
        Ok(module.refs.invoke_export(func_name, args, mut_external)?)
    }

    pub fn run_native(&mut self, module: NativeModule) -> Result<(), Error> {
        // create native
        let external = self.external.get_mut();
        external
            .native_mods
            .insert(module.name, module.alloc_module());
        Ok(())
    }

    pub fn run_wasm(
        &mut self,
        module: WasmModule,
        start: StartFunctionName,
        deps: &[ModuleName],
    ) -> Result<(), Error> {
        let mut imports_builder = ImportsBuilder::default();
        let mut offset_indexs = Vec::new();
        let external = self.external.get_mut();
        let native_refs = &mut external.native_refs;
        for dep_name in deps {
            match dep_name {
                ModuleName::Wasm(n) => {
                    let m = self.wasm_refs.get(n).unwrap();
                    imports_builder.push_resolver(module.name, m);
                }
                ModuleName::Native(n) => {
                    let nm = external.native_mods.get(n).unwrap();
                    log::info!("current offset is {}", self.offset);
                    offset_indexs.push((self.offset, nm.instance.name));
                    let native_ref = NativeModuleRef {
                        module: nm.clone(),
                        refs: None,
                        offset: self.offset,
                    };
                    native_refs.insert(self.offset, native_ref);
                    self.offset = nm.offset() + self.offset;
                }
            }
        }
        for (offset_index, name) in offset_indexs.clone() {
            let native_ref = external.native_refs.get(&offset_index).unwrap();
            log::info!("load index {}, name: {}", offset_index, name);
            imports_builder.push_resolver(name, native_ref);
        }
        let ins = ModuleInstance::new(&module.module, &imports_builder)?;
        let refs = match start {
            StartFunctionName::Function(v) => {
                let refs = ins.assert_no_start();
                refs.invoke_export(v, &[], external)?;
                refs
            }
            StartFunctionName::Section => ins.run_start(external)?,
            StartFunctionName::NoStart => ins.assert_no_start(),
        };
        self.wasm_refs
            .insert(module.name, WasmModuleRef { refs: refs.clone() });
        for (offset_index, _name) in offset_indexs {
            let native_ref = external.native_refs.get_mut(&offset_index).unwrap();
            native_ref.refs = Some(refs.clone());
        }
        Ok(())
    }
}

impl Externals for RuntimeExternal {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        log::info!("call index: {}", index);
        let mut r = self.native_refs.split_off(&index);
        // TODO: process index.
        let (mindex, module) = r.pop_last().unwrap();
        log::info!("call funcs index is {}", mindex);
        let real_index = index - module.offset;
        let f = module.module.instance.funcs[real_index].func;
        // Insert back
        let result = f(&module.refs.as_ref().unwrap(), args.as_ref());
        self.native_refs.insert(index, module);
        self.native_refs.append(&mut r);
        Ok(result?)
    }
}
