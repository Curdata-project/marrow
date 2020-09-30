use marrow_rt::{Error, NativeFunc, NativeModule, RuntimeValue};
use std::convert::TryInto;
use wasmi::{ExternVal, ModuleRef};

pub fn print(module: &ModuleRef, args: &[RuntimeValue]) -> Result<Option<RuntimeValue>, Error> {
    if let ExternVal::Memory(memory) = module.export_by_name("memory").unwrap() {
        let ptr = args[0].try_into().unwrap();
        let size: u32 = args[1].try_into().unwrap();
        let data = memory.get(ptr, size.try_into().unwrap()).unwrap();
        let s = String::from_utf8(data).unwrap();
        println!("{}", s);
    }
    Ok(None)
}

static FUNCS: &[NativeFunc] = &[NativeFunc {
    name: "print",
    func: print,
}];

pub fn entry() -> NativeModule {
    NativeModule::from_funcs("prints", FUNCS).unwrap()
}
