#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

// use mw_std::debug;
use alloc::boxed::Box;
use core::cell::RefCell;
use mw_rt::actor::Actor;
use alloc::vec::Vec;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[mw_rt::actor::actor]
pub struct MyActor {}

#[async_trait::async_trait]
impl Actor for MyActor {
    fn new() -> Self {
        MyActor {}
    }

    async fn init(&mut self) {}
}

#[mw_rt::actor::expose]
impl MyActor {
    #[mw_rt::actor::method]
    pub fn return_int(&mut self, _bytes1: &[u8]) -> usize {
        1
    }

    pub fn return_bytes(&mut self) -> &[u8] {
        &[0u8; 10]
    }

    #[mw_rt::actor::method]
    pub async fn async_return_bytes(&mut self, _t: u8, _bytes1: &[u8], _bytes2: &[u8]) -> u8 {
        1
    }

    #[mw_rt::actor::method]
    pub async fn async_return_bytes_vec(&mut self) -> Vec<u8> {
        Vec::new()
    }
}
