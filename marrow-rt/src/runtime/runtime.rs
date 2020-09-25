use crate::module::{UnionModule, UnionRef};
use crate::{Error, Module, ModuleRef, NativeFunc, RuntimeValue, StartFunctionName};
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
// use core::cell::RefCell;
use wasmi::{ImportsBuilder, ModuleInstance};

pub struct Runtime {
    pub(crate) offset: usize,
    pub(crate) refs: BTreeMap<&'static str, ModuleRef>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            offset: 0,
            refs: BTreeMap::new(),
        }
    }

    fn run_wasm(
        &mut self,
        module: &wasmi::Module,
        start: StartFunctionName,
        deps: &[&str],
    ) -> Result<wasmi::ModuleRef, Error> {
        let mut imports_builder = ImportsBuilder::default();
        for dep in deps {
            if let Some(refs) = self.refs.get(dep) {
                // if is native, set offsets.
                if let UnionRef::NativeRef {
                    offset: _,
                    refs: inner_refs,
                } = &refs.union
                {
                    // build native-wasm pair offset.
                    let new_offset = self.offset + refs.get_native_offset();
                    let new_refs = ModuleRef {
                        module: refs.module.clone(),
                        union: UnionRef::NativeRef {
                            offset: new_offset,
                            refs: inner_refs.clone(),
                        },
                    };
                    imports_builder.push_resolver(*dep, &new_refs);
                } else {
                    imports_builder.push_resolver(*dep, refs);
                }
            } else {
                return Err(Error::NoRuningModule);
            }
        }
        let ins = ModuleInstance::new(module, &imports_builder)?;
        let refs = match start {
            StartFunctionName::Function(v) => {
                let refs = ins.assert_no_start();
                // TODO: deal execute error. consider exit this machine.
                refs.invoke_export(v, &[], &mut wasmi::NopExternals)?;
                refs
            }
            StartFunctionName::Section => {
                // TODO: deal execute error. consider exit this machine.
                ins.run_start(&mut wasmi::NopExternals)?
            }
            StartFunctionName::NoStart => ins.assert_no_start(),
        };
        Ok(refs)
    }

    fn run_native(&mut self, _funcs: &'static [NativeFunc]) -> Result<(), Error> {
        // create native
        Ok(())
    }

    pub fn run(
        &mut self,
        name: &'static str,
        m: Module,
        start: Option<StartFunctionName>,
        deps: &[&str],
    ) -> Result<(), Error> {
        match &m.module {
            UnionModule::WasmModule { module } => {
                let refs = self.run_wasm(&module, start.unwrap(), deps)?;
                let r = ModuleRef {
                    module: Rc::new(m),
                    union: UnionRef::WasmRef { refs },
                };
                self.refs.insert(name, r);
            }
            UnionModule::NativeModule { funcs } => self.run_native(funcs)?,
        }
        Ok(())
    }
}
