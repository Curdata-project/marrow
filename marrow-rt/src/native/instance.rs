use crate::NativeModule;
use alloc::rc::Rc;

#[derive(Clone, Debug)]
pub struct NativeInstance {
    pub(crate) instance: Rc<NativeModule>,
}

impl NativeInstance {
    pub fn offset(&self) -> usize {
        self.instance.funcs.len()
    }
}
