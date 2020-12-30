//! load byte code

/// load byte code return number
/// 0 success
/// 1 fail
#[no_mangle]
pub fn loader(bytes: &[u8]) -> i32 {
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _loader(ptr: *const u8, size: usize) -> i32;
    }

    unsafe { _loader(bytes.as_ptr(), bytes.len()) }
}
