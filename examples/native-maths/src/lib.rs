use marrow_rt::{Error, NativeFunc, NativeModule, RuntimeValue};
use wasmi::ModuleRef;

pub fn add(_module: &ModuleRef, _args: &[RuntimeValue]) -> Result<Option<RuntimeValue>, Error> {
    Ok(Some(RuntimeValue::I32(1)))
}

pub fn mul(_module: &ModuleRef, _args: &[RuntimeValue]) -> Result<Option<RuntimeValue>, Error> {
    Ok(Some(RuntimeValue::I32(1)))
}

static FUNCS: &[NativeFunc] = &[
    NativeFunc {
        name: "add",
        func: add,
    },
    NativeFunc {
        name: "mul",
        func: mul,
    },
];

pub fn entry() -> NativeModule {
    NativeModule::from_funcs("maths", FUNCS).unwrap()
}
