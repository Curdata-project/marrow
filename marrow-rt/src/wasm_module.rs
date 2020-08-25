use alloc::vec::Vec;

pub struct WasmModule<ID> {
    pub id: ID,
    pub code: Vec<u8>,
    pub name: String,
    pub version: String,
    pub deps: Vec<String>,
}
