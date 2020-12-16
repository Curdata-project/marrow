#![no_std]

use wstd::fs;
use wstd::debug;

#[no_mangle]
pub extern "C" fn main() {
    fs::read_file_callback("./test.txt", || {
        debug::println("read file success");
    });
}
