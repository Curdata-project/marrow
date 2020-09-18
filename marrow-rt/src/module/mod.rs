mod module;
pub use module::Module;
pub(crate) use module::UnionModule;

mod refs;
pub(crate) use refs::UnionRef;
pub use refs::{ModuleRef, StartFunctionName};
