use super::{WasmModule, WasmModuleRef};
use crate::{ExternalsBuilder, NativeModuleRef};
use alloc::vec::Vec;
use core::cell::RefCell;
use wasmi::{ImportsBuilder, ModuleInstance};

/// Start function name for wasm module.
pub enum StartFunctionName {
    /// Use special function as start function.
    Function(&'static str),
    /// Use module's start function. Inital module failed if no start function in module.
    Section,
    /// Don't use module's start function. Even if module has start function.
    NoStart,
}

/// `Webassembly` module builder.
pub struct WasmModuleBuilder<'a> {
    pub(crate) permissions: Vec<&'static str>,
    pub(crate) module: Option<&'a WasmModule>,
    pub(crate) start: StartFunctionName,
    // this field must not none when inital module.
    pub(crate) externals_builder: ExternalsBuilder,
    pub(crate) imports_builder: ImportsBuilder<'a>,
}

impl<'a> Default for WasmModuleBuilder<'a> {
    fn default() -> Self {
        WasmModuleBuilder {
            permissions: Vec::new(),
            start: StartFunctionName::NoStart,
            externals_builder: ExternalsBuilder::default(),
            imports_builder: ImportsBuilder::default(),
            module: None,
        }
    }
}

/// Add `NativeModule` for this `wasm` module.
///
/// These methods can use chain call and use mutable borrowed self
impl<'a> WasmModuleBuilder<'a> {
    /// Chain call.
    pub fn with_native_module(mut self, name: &str, resolver: &'a NativeModuleRef) -> Self {
        self.push_native_module(name, resolver);
        self
    }
    /// Use Mutable borrowed self.
    pub fn push_native_module(&mut self, name: &str, resolver: &'a NativeModuleRef) {
        // self.module = Some(module);
        // Test has external?, if exists inital.
        self.imports_builder.push_resolver(name, resolver);
        self.externals_builder.push_resolver(resolver.clone());
    }
}

/// Add `NativeModule` for this `wasm` module.
impl<'a> WasmModuleBuilder<'a> {
    /// Chain call.
    pub fn with_wasm_module(mut self, name: &str, resolver: &'a WasmModuleRef) -> Self {
        self.push_wasm_module(name, resolver);
        self
    }

    /// Use Mutable borrowed self.
    pub fn push_wasm_module(&mut self, name: &str, resolver: &'a WasmModuleRef) {
        //self.module = Some(module);
        self.imports_builder.push_resolver(name, resolver);
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
    pub fn with_permission(mut self, permission: &'static str) -> Self {
        self.push_permission(permission);
        self
    }

    /// Use Mutable borrowed self.
    pub fn push_permission(&mut self, permission: &'static str) {
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

/// Builder。
impl<'a> WasmModuleBuilder<'a> {
    pub fn build(self) -> WasmModuleRef {
        let module = self.module.unwrap();

        // check format of permission.
        // for x in &self.permissions {
        //     let splited = x.split(".");
        // }

        // TODO: deal load error.
        let mut external = self.externals_builder;
        let ins = ModuleInstance::new(&module.module, &self.imports_builder).unwrap();
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
}
