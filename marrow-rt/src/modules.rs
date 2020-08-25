use wasmi::ModuleRef;

use alloc::collections::BTreeMap;

pub struct Modules {
    pub wasm: BTreeMap<[u64; 4], ModuleRef>,
    pub max_cache: usize,
}

impl Modules {
    pub fn new() -> Self {
        Modules {
            wasm: BTreeMap::new(),
            max_cache: 20,
        }
    }

    pub fn load() {}
}
