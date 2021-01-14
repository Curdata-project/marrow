#[link(wasm_import_module = "mw_rt")]
extern "C" {
    fn __callback_u32(index: usize, result: u32);
    fn __callback_i32(index: usize, result: i32);
    fn __callback_u64(index: usize, result: u64);
    fn __callback_i64(index: usize, result: i64);
    fn __callback_isize(index: usize, result: isize);
    fn __callback_usize(index: usize, result: usize);
    fn __callback_bytes(index: usize, ptr: *const u8, length: usize);
    fn __callback_null(index: usize);
}

pub fn callback_null(index: usize, _result: ()) {
    unsafe { __callback_null(index) }
}

pub fn callback_u8(index: usize, result: u8) {
    unsafe { __callback_u32(index, result.into()) }
}

pub fn callback_u16(index: usize, result: u16) {
    unsafe { __callback_u32(index, result.into()) }
}

pub fn callback_u32(index: usize, result: u32) {
    unsafe { __callback_u32(index, result) }
}

pub fn callback_u64(index: usize, result: u64) {
    unsafe { __callback_u64(index, result) }
}

pub fn callback_i8(index: usize, result: i8) {
    unsafe { __callback_i32(index, result.into()) }
}

pub fn callback_i16(index: usize, result: i16) {
    unsafe { __callback_i32(index, result.into()) }
}

pub fn callback_i32(index: usize, result: i32) {
    unsafe { __callback_i32(index, result) }
}

pub fn callback_i64(index: usize, result: i64) {
    unsafe { __callback_i64(index, result) }
}

pub fn callback_isize(index: usize, result: isize) {
    unsafe { __callback_isize(index, result) }
}

pub fn callback_usize(index: usize, result: usize) {
    unsafe { __callback_usize(index, result) }
}

pub fn callback_bytes(index: usize, result: &[u8]) {
    let ptr = result.as_ptr();
    let len = result.len();
    unsafe { __callback_bytes(index, ptr, len) }
}

pub fn callback_bytes_vec(index: usize, result: alloc::vec::Vec<u8>) {
    let ptr = result.as_ptr();
    let len = result.len();
    unsafe { __callback_bytes(index, ptr, len) }
}
