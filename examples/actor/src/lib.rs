#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

// use mw_std::debug;
use core::cell::RefCell;
use alloc::boxed::Box;
use mw_rt::actor::Actor;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct MyActor {}

struct MyActorWrapper {
    actor: RefCell<MyActor>,
    lastest_bytes_length: RefCell<usize>,
}

unsafe impl Sync for MyActorWrapper {}

#[async_trait::async_trait]
impl Actor for MyActor {
    fn new() -> Self {
        MyActor {}
    }

    async fn init(&mut self) {

    }
}

impl MyActor {
    pub fn return_int(&mut self) -> usize {
        1
    }

    pub fn return_bytes(&mut self) -> &[u8] {
        &[0u8; 10]
    }

    // pub async fn async_return_int(&mut self) -> usize {
    //     1
    // }

    // pub async fn async_return_bytes(&mut self) -> &[u8] {
    //     &[0u8; 10]
    // }
}

#[no_mangle]
pub extern "C" fn rpc_actor_return_int() -> usize {
    ACTOR.actor.borrow_mut().return_int()
}


#[no_mangle]
pub extern "C" fn rpc_actor_return_bytes() -> *const u8 {
    let mut actor = ACTOR.actor.borrow_mut();
    let bytes = actor.return_bytes();
    *ACTOR.lastest_bytes_length.borrow_mut() = bytes.len();
    bytes.as_ptr()
}


#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref ACTOR: MyActorWrapper = MyActorWrapper {
        actor: RefCell::new(MyActor::new()),
        lastest_bytes_length: RefCell::new(0),
    };
}

#[no_mangle]
pub extern "C" fn __get_lastest_bytes_length() -> usize {
    *ACTOR.lastest_bytes_length.borrow()
}

#[mw_rt::async_main]
async fn main() {
    ACTOR.actor.borrow_mut().init().await;
}
