use wasmi::{
    Error, Externals, FuncInstance, FuncRef, ModuleImportResolver, RuntimeArgs, RuntimeValue,
    Signature, Trap,
};

use alloc::format;

/// WebAssembly native module's func
///
/// Each `NativeFunc` persent a function define for host.
pub struct NativeFunc {
    pub name: &'static str,
    pub func: fn(&[RuntimeValue]) -> Option<RuntimeValue>,
}

pub struct NativeModule {
    funcs: &'static [NativeFunc],
    offset: usize,
}

impl NativeModule {
    pub fn new(funcs: &'static [NativeFunc], offset: usize) -> Self {
        NativeModule {
            funcs,
            offset: offset + funcs.len(),
        }
    }
}

impl ModuleImportResolver for NativeModule {
    fn resolve_func(&self, field_name: &str, _signature: &Signature) -> Result<FuncRef, Error> {
        let mut i = 0;
        let func = &self.funcs[i];
        let funcs_len = self.funcs.len();
        while func.name != field_name && i <= funcs_len {
            i += 1;
        }
        if i > funcs_len {
            Err(Error::Instantiation(format!("id is exceed.")))
        } else {
            let func_ref = FuncInstance::alloc_host(_signature.clone(), i);
            Ok(func_ref)
        }
    }
}

impl Externals for NativeModule {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        let real_index = index - self.offset;
        let f = self.funcs[real_index].func;
        Ok(f(args.as_ref()))
    }
}
