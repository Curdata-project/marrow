use std::convert::TryInto;
use wasmi::{Externals, MemoryRef, RuntimeArgs, RuntimeValue, Trap};

pub struct IoExternals {
    mem: MemoryRef,
}

impl IoExternals {
    pub fn new(mem: MemoryRef) -> Self {
        IoExternals { mem }
    }
}

const FUNC_PRINTLN_STDOUT_INDEX: usize = 0;

impl Externals for IoExternals {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            FUNC_PRINTLN_STDOUT_INDEX => {
                let ptr: i32 = args.nth_checked(0)?;
                let size: u32 = args.nth_checked(0)?;
                // get buffer from memony.
                let data = self
                    .mem
                    .get(ptr.try_into().unwrap(), size.try_into().unwrap())
                    .unwrap();
                let s = String::from_utf8(data).unwrap();
                println!("{}", s);
                Ok(None)
            }
            _ => panic!("Unimplemented function at {}", index),
        }
    }
}

// impl ModuleImportResolver for IoExternals {
//     fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
//         let index =
//     }
// }
