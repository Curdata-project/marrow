use crate::{Instance};
use alloc::collections::BTreeMap;

pub struct Host<'a> {
    instances: BTreeMap<&'a str, &'a dyn Instance>,
}

impl<'a> Host<'a> {
    pub fn new() -> Self {
        Host {
            instances: BTreeMap::new(),
        }
    }

    pub fn get_instance(&self, name: &str) -> Option<&dyn Instance> {
        self.instances.get(name).map(|i| *i)
    }

    pub fn add_instance<I>(&mut self, name: &'a str, instance: &'a I)
    where
        I: Instance,
    {
        self.instances.insert(name, instance);
    }
}
