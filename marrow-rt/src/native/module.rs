use super::NativeFunc;
use alloc::rc::Rc;

#[derive(Debug)]
pub struct NativeModule {
    pub name: &'static str,
    pub(crate) funcs: &'static [NativeFunc],
}

impl NativeModule {
    pub fn new(name: &'static str, funcs: &'static [NativeFunc]) -> Self {
        NativeModule { name, funcs }
    }

    pub(crate) fn alloc_module(self) -> Rc<NativeModule> {
        Rc::new(self)
    }
}
