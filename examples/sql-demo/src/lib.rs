#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

mod proto;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[mw_rt::async_main]
async fn main() {
    let mut message = proto::sql_demo::Sql::default();
    let str = "select * from test_db from id = $1";
    message.sql = Cow::Borrowed(str);
    let param = String::from("123456qwer7890").as_bytes().to_vec();
    message.params.push(Cow::Owned(param));
    let mut out: Vec<u8> = Vec::new();

    let serialize_result = quick_protobuf::serialize_into_slice(&message, out.as_mut_slice());
    if serialize_result.as_ref().is_err() {
        return;
    }
    sql_fun(out.as_slice()).await;
}

async fn sql_fun(bytes: &[u8]) {
    let result = mw_std::sql::sql_execute(bytes, 0);
    mw_std::debug::println(&alloc::format!("{:?}", result));
}
