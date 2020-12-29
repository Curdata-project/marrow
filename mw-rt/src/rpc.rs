
#[link(wasm_import_module = "mw_rt")]
extern "C" {
    fn __callback_u32(result: u32);
    fn __callback_i32(result: i32);
    fn __callback_u64(result: u64);
    fn __callback_i64(result: i64);
    fn __callback_isize(result: isize);
    fn __callback_usize(result: usize);
    fn __callback_bytes(ptr: *const u8, length: usize);
}

pub fn callback_u8(result: u8) {
    unsafe { __callback_u32(result.into()) }
}

pub fn callback_u16(result: u16) {
    unsafe { __callback_u32(result.into()) }
}

pub fn callback_u32(result: u32) {
    unsafe { __callback_u32(result) }
}

pub fn callback_u64(result: u64) {
    unsafe { __callback_u64(result) }
}

pub fn callback_i8(result: i8) {
    unsafe { __callback_i32(result.into()) }
}

pub fn callback_i16(result: i16) {
    unsafe { __callback_i32(result.into()) }
}

pub fn callback_i32(result: i32) {
    unsafe { __callback_i32(result) }
}

pub fn callback_i64(result: i64) {
    unsafe { __callback_i64(result) }
}

pub fn callback_isize(result: isize) {
    unsafe { __callback_isize(result) }
}

pub fn callback_usize(result: usize) {
    unsafe { __callback_usize(result) }
}

pub fn callback_bytes(result: &[u8]) {
    let ptr = result.as_ptr();
    let len = result.len();
    unsafe { __callback_bytes(ptr, len) }
}

