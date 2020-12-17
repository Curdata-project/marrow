#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use wstd::fs;
use wstd::debug;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[marrow_rt::main]
async fn main() {
    let _r = fs::read_file("./test.txt").await;
    debug::println("ok");
}

