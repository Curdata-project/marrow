use crate::{
    Error, ExternalsBuilder, Module, ModuleRef, NativeFunc, RuntimeValue, StartFunctionName,
    UnionModule, UnionRef,
};
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use core::cell::RefCell;
use wasmi::{ImportsBuilder, ModuleInstance};

pub struct Runtime {
    pub(crate) modules: BTreeMap<&'static str, ModuleRef>,
    pub(crate) external: RefCell<ExternalsBuilder>,
    pub(crate) offset: usize,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            modules: BTreeMap::new(),
            external: RefCell::new(ExternalsBuilder::default()),
            offset: 0,
        }
    }

    pub fn get(&self, name: &str) -> Option<&ModuleRef> {
        self.modules.get(name)
    }

    /// invoke export function.
    pub fn invoke_export(
        &self,
        module_name: &str,
        func_name: &str,
        args: &[RuntimeValue],
    ) -> Result<Option<RuntimeValue>, Error> {
        let module = self.modules.get(module_name).unwrap();
        let mut external = self.external.try_borrow_mut().unwrap();
        module.invoke_export(func_name, args, &mut *external)
    }

    fn run_wasm(
        &mut self,
        start: StartFunctionName,
        deps: &[&str],
        module: &wasmi::Module,
    ) -> Result<wasmi::ModuleRef, Error> {
        let mut imports = ImportsBuilder::default();
        for dep in deps {
            if let Some(m) = self.get(dep) {
                imports.push_resolver(*dep, m);
            } else {
                return Err(Error::NoRuningModule);
            }
        }

        let ins = ModuleInstance::new(module, &imports).unwrap();
        let mut external = self.external.try_borrow_mut().unwrap();

        match start {
            StartFunctionName::Function(v) => {
                let refs = ins.assert_no_start();
                // TODO: deal execute error. consider exit this machine.
                refs.invoke_export(v, &[], &mut *external).unwrap();
                Ok(refs)
            }
            StartFunctionName::Section => {
                // TODO: deal execute error. consider exit this machine.
                let refs = ins.run_start(&mut *external).unwrap();
                Ok(refs)
            }
            StartFunctionName::NoStart => {
                let refs = ins.assert_no_start();
                Ok(refs)
            }
        }
    }

    fn run_native(&mut self, funcs: &'static [NativeFunc]) -> usize {
        let offset = self.offset;
        self.offset = offset + funcs.len();
        offset
    }

    pub fn run(
        &mut self,
        name: &'static str,
        _module: Module,
        start: StartFunctionName,
        deps: &[&str],
    ) -> Result<(), Error> {
        let module = match &_module.module {
            UnionModule::WasmModule { module } => {
                let refs = self.run_wasm(start, deps, &module)?;
                UnionRef::WasmRef { refs }
            }
            UnionModule::NativeModule { funcs } => {
                let offset = self.run_native(funcs);
                UnionRef::NativeRef { offset }
            }
        };

        let refs = ModuleRef {
            module: Rc::new(_module),
            union: module,
        };

        self.modules.insert(name, refs);

        Ok(())
    }
}
