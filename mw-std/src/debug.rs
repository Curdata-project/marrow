pub fn println(s: &str) {
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn print(data: *const u8, length: usize);
    }

    let bytes = s.as_bytes();
    unsafe { print(bytes.as_ptr(), bytes.len()) }
}
