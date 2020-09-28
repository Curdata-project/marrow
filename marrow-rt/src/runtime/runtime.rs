// use crate::module::{UnionModule, UnionRef};
use crate::{
    Error, ModuleName, NativeInstance, NativeModule, NativeModuleRef, RuntimeValue,
    StartFunctionName, WasmModuleRef,
};
use alloc::collections::BTreeMap;
// use alloc::rc::Rc;
// // use core::cell::RefCell;
use alloc::vec::Vec;
use wasmi::{Externals, ImportsBuilder, ModuleInstance, RuntimeArgs, Trap};

pub struct Runtime {
    pub(crate) offset: usize,
    pub(crate) wasm_refs: BTreeMap<&'static str, WasmModuleRef>,
    pub(crate) native_mods: BTreeMap<&'static str, NativeInstance>,
    pub(crate) native_refs: BTreeMap<usize, NativeModuleRef>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            offset: 0,
            wasm_refs: BTreeMap::new(),
            native_mods: BTreeMap::new(),
            native_refs: BTreeMap::new(),
        }
    }

    pub fn run_native(&mut self, name: &'static str, module: NativeModule) -> Result<(), Error> {
        // create native
        self.native_mods.insert(name, module.alloc_module());
        Ok(())
    }

    pub fn run_wasm(
        &mut self,
        module: &wasmi::Module,
        name: &'static str,
        start: StartFunctionName,
        deps: &[ModuleName],
    ) -> Result<(), Error> {
        let mut imports_builder = ImportsBuilder::default();
        let mut offset_indexs = Vec::new();
        for dep_name in deps {
            match dep_name {
                ModuleName::Wasm(n) => {
                    let m = self.wasm_refs.get(n).unwrap();
                    imports_builder.push_resolver(name, m);
                }
                ModuleName::Native(n) => {
                    let nm = self.native_mods.get(n).unwrap();
                    self.offset = nm.offset() + self.offset;
                    offset_indexs.push(self.offset);
                    let native_ref = NativeModuleRef {
                        module: nm.clone(),
                        refs: None,
                        offset: self.offset,
                    };
                    self.native_refs.insert(self.offset, native_ref);
                }
            }
        }
        let ins = ModuleInstance::new(module, &imports_builder)?;
        let refs = match start {
            StartFunctionName::Function(v) => {
                let refs = ins.assert_no_start();
                refs.invoke_export(v, &[], self)?;
                refs
            }
            StartFunctionName::Section => ins.run_start(self)?,
            StartFunctionName::NoStart => ins.assert_no_start(),
        };
        self.wasm_refs
            .insert(name, WasmModuleRef { refs: refs.clone() });
        for offset_index in offset_indexs {
            let native_ref = self.native_refs.get_mut(&offset_index).unwrap();
            native_ref.refs = Some(refs.clone());
        }
        Ok(())
    }
}

impl Externals for Runtime {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        let mut r = self.native_refs.split_off(&index);
        // // TODO: process index.
        let (mindex, module) = self.native_refs.pop_last().unwrap();
        log::info!("call funcs index is {}", mindex);
        let real_index = index - module.offset;
        let f = module.module.instance.funcs[real_index].func;
        // // Insert back
        let res = f(&module.refs.as_ref().unwrap(), args.as_ref());
        self.native_refs.insert(index, module);
        self.native_refs.append(&mut r);
        Ok(res)
        // Ok(None)
    }
}

//     pub fn run(
//         &mut self,
//         name: &'static str,
//         m: Module,
//         start: Option<StartFunctionName>,
//         deps: &[&str],
//     ) -> Result<(), Error> {
//         match &m.module {
//             UnionModule::WasmModule { module } => {
//                 let refs = self.run_wasm(&module, start.unwrap(), deps)?;
//                 let r = ModuleRef {
//                     module: Rc::new(m),
//                     union: UnionRef::WasmRef { refs },
//                 };
//                 self.refs.insert(name, r);
//             }
//             UnionModule::NativeModule { funcs } => self.run_native(funcs)?,
//         }
//         Ok(())
//     }
// }
