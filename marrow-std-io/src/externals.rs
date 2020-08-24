use wasmi::{
    Externals, RuntimeValue, RuntimeArgs, Trap, MemoryRef,
};

pub struct IoExternals {
    mem: MemoryRef,
}

impl IoExternals {
    pub fn new(mem: MemoryRef) -> Self {
        IoExternals {
            mem,
        }
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
                let a: i32 = args.nth_checked(0)?;
                // get buffer from memony.
                
                Ok(None)
            }
            _ => panic!("Unimplemented function at {}", index),
        }
    }
}
