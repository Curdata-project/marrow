//! time operate

// get timestamp
pub fn get_timestamp() -> i64 {
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _get_timestamp() -> i64;
    }
    unsafe { _get_timestamp() }
}
