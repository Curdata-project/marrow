mod module;
pub use module::Module;
pub(crate) use module::UnionModule;

mod refs;
pub use refs::ModuleRef;
pub(crate) use refs::UnionRef;

mod func;
pub use func::NativeFunc;
