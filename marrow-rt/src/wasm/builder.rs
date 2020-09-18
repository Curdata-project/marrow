use super::{WasmModule, WasmModuleRef};
use crate::{ExternalsBuilder, NativeModuleRef};
use alloc::vec::Vec;
use core::cell::RefCell;
use wasmi::{ImportsBuilder, ModuleInstance};

/// `Webassembly` module builder.
pub struct WasmModuleBuilder<'a> {
    pub(crate) permissions: Vec<&'a str>,
    pub(crate) module: Option<&'a WasmModule>,
    pub(crate) start: StartFunctionName,
    pub(crate) wasm_module: Vec<(&'a str, &'a mut WasmModuleRef)>,
    pub(crate) native_module: Vec<(&'a str, &'a mut NativeModuleRef)>,
}

impl<'a> Default for WasmModuleBuilder<'a> {
    fn default() -> Self {
        WasmModuleBuilder {
            permissions: Vec::new(),
            start: StartFunctionName::NoStart,
            module: None,
            wasm_module: Vec::new(),
            native_module: Vec::new(),
        }
    }
}

/// Add `NativeModule` for this `wasm` module.
///
/// These methods can use chain call and use mutable borrowed self
impl<'a> WasmModuleBuilder<'a> {
    /// Chain call.
    pub fn with_native_module(mut self, name: &'a str, resolver: &'a mut NativeModuleRef) -> Self {
        self.push_native_module(name, resolver);
        self
    }
    /// Use Mutable borrowed self.
    pub fn push_native_module(&mut self, name: &'a str, resolver: &'a mut NativeModuleRef) {
        // self.module = Some(module);
        self.native_module.push((name, resolver));
    }
}

/// Add `WasmModule` for this `wasm` module.
impl<'a> WasmModuleBuilder<'a> {
    /// Chain call.
    pub fn with_wasm_module(mut self, name: &'a str, resolver: &'a mut WasmModuleRef) -> Self {
        self.push_wasm_module(name, resolver);
        self
    }

    /// Use Mutable borrowed self.
    pub fn push_wasm_module(&mut self, name: &'a str, resolver: &'a mut WasmModuleRef) {
        //self.module = Some(module);
        self.wasm_module.push((name, resolver));
    }
}

/// Add WasmModule define for this `wasm` module.
impl<'a> WasmModuleBuilder<'a> {
    /// Chain call.
    pub fn with_module(mut self, module: &'a WasmModule) -> Self {
        self.push_module(module);
        self
    }

    /// Use Mutable borrowed self.
    pub fn push_module(&mut self, module: &'a WasmModule) {
        self.module = Some(module);
    }
}

/// Add permission for this `wasm` module.
impl<'a> WasmModuleBuilder<'a> {
    /// Chain call.
    pub fn with_permission(mut self, permission: &'a str) -> Self {
        self.push_permission(permission);
        self
    }

    /// Use Mutable borrowed self.
    pub fn push_permission(&mut self, permission: &'a str) {
        self.permissions.push(permission);
    }
}

/// Set start function for `wasm` module.
impl<'a> WasmModuleBuilder<'a> {
    /// Chain call.
    /// See [StartFunctionName](./enum.StartFunctionName.html) for detail.
    pub fn with_start(mut self, start: StartFunctionName) -> Self {
        self.push_start(start);
        self
    }

    /// Use Mutable borrowed self.
    /// See [StartFunctionName](./enum.StartFunctionName.html) for detail.
    pub fn push_start(&mut self, start: StartFunctionName) {
        self.start = start;
    }
}

/// Builder
impl<'a> WasmModuleBuilder<'a> {
    pub fn build(self) -> WasmModuleRef {
        let mut external = ExternalsBuilder::default();
        let mut imports = ImportsBuilder::default();

        // parse id.

        // build external.
        for (name, reslover) in self.native_module {
            reslover.permission.push(false);
            external.push_resolver(reslover.clone());

            log::info!(
                "NativeModule {} offset is: {} at external.",
                name,
                external.end
            );
            reslover.offset = external.end;
            imports.push_resolver(name, reslover);
        }

        // build imports.
        for (name, resolver) in self.wasm_module {
            imports.push_resolver(name, resolver);
        }

        // TODO: trow error.
        let module = self.module.unwrap();
        let ins = ModuleInstance::new(&module.module, &imports).unwrap();
        match self.start {
            StartFunctionName::Function(v) => {
                let refs = ins.assert_no_start();
                external.push_module(refs.clone());
                // TODO: deal execute error. consider exit this machine.
                refs.invoke_export(v, &[], &mut external).unwrap();
                WasmModuleRef {
                    refs,
                    external: RefCell::new(external),
                }
            }
            StartFunctionName::Section => {
                // TODO: deal execute error. consider exit this machine.
                let refs = ins.run_start(&mut external).unwrap();
                external.push_module(refs.clone());
                WasmModuleRef {
                    refs,
                    external: RefCell::new(external),
                }
            }
            StartFunctionName::NoStart => {
                let refs = ins.assert_no_start();
                WasmModuleRef {
                    refs,
                    external: RefCell::new(external),
                }
            }
        }
    }

    // fn build_import_builder(self) -> ImportsBuilder {
    //     let mut imports = ImportsBuilder::default();
    //     let natives = &self.native_module;
    //     for (name, resolver) in natives.into_iter() {
    //         imports.push_resolver(name, resolver);
    //     }
    //     imports
    // }
}
