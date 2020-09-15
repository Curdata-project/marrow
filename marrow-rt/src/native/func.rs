use wasmi::{ModuleRef, RuntimeValue};

/// WebAssembly native module's func
///
/// Each `NativeFunc` persent a function define for host.
pub struct NativeFunc {
    pub name: &'static str,
    pub func: fn(&ModuleRef, &[RuntimeValue]) -> Option<RuntimeValue>,
}
