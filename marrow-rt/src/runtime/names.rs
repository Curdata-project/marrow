/// Start function name for wasm module.
pub enum StartFunctionName {
    /// Use special function as start function.
    Function(&'static str),
    /// Use module's start function. Inital module failed if no start function in module.
    Section,
    /// Don't use module's start function. Even if module has start function.
    NoStart,
}

pub enum ModuleName {
    Native(&'static str),
    Wasm(&'static str),
}
