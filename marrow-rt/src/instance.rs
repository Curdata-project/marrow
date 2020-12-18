use crate::{Module, Type};

/// Define `Instance` behavior.
pub trait Instance {
    /// create instance from module.
    fn from_module<M>(module: &M) -> Self
    where
        M: Module,
        Self: Sized;

    /// get value from linear memory.
    fn get_value(&self, index: usize) -> Type<'_>;

    /// invoke function by name.
    fn invoke_func(&self, name: &'_ str, args: &[Type<'_>]) -> Type<'_>;

    /// add host function to instance.
    fn add_func<'a>(&mut self, name: &'_ str, func: fn(&[Type<'a>]) -> Option<Type<'a>>);
}
