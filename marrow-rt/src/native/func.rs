use crate::Error;
use core::fmt::{self, Debug};
use wasmi::{ModuleRef, RuntimeValue};

/// WebAssembly native module's func.
///
/// Each `NativeFunc` persent a function define for host.
pub struct NativeFunc {
    pub name: &'static str,
    pub func: fn(&ModuleRef, &[RuntimeValue]) -> Result<Option<RuntimeValue>, Error>,
}

impl Debug for NativeFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Func name: {}]", self.name)
    }
}
