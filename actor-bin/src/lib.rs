#![no_std]

extern crate alloc;
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wstd::fs;
use wstd::debug;
use wstd::runtime;

#[marrow::main]
async fn main() {
    let _r = fs::read_file("./test.txt").await;
    debug::println("ok");
}

