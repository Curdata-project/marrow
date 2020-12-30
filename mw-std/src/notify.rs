//! transfer js callback

pub fn notify_ptr_size(index: usize, data: &[u8]) {
    let ptr = data.as_ptr();
    let size = data.len();

    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _callback_ptr_size(index: usize, ptr: *const u8, size: usize);
    }

    unsafe {
        _callback_ptr_size(index, ptr, size);
    }
}

pub fn notify_number(index: usize, number: usize) {
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _callback_number(index: usize, number: usize);
    }

    unsafe {
        _callback_number(index, number);
    }
}
