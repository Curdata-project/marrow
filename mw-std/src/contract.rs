//! contract operate

use alloc::rc::Rc;
use core::cell::RefCell;
use core::ffi::c_void;
use core::future::Future;
use core::option::Option::Some;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

// sync func
pub fn do_load(bytes: &[u8]) -> u32 {
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _load_contract(ptr: *const u8, size: usize) -> u32;
    }

    unsafe { _load_contract(bytes.as_ptr(), bytes.len()) }
}

unsafe extern "C" fn hook_number<F>(user_data: *mut c_void, result: u32)
where
    F: FnMut(u32),
{
    (*(user_data as *mut F))(result)
}

pub fn load_callback<F>(bytes: &[u8], mut f: F)
where
    F: FnMut(u32),
{
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _load_contract_callback(
            ptr: *const u8,
            size: usize,
            cb: unsafe extern "C" fn(*mut c_void, u32),
            user_data: *mut c_void,
        );
    }

    let user_data = &mut f as *mut _ as *mut c_void;

    unsafe {
        _load_contract_callback(bytes.as_ptr(), bytes.len(), hook_number::<F>, user_data);
    }
}

#[derive(Debug, Clone)]
pub struct NumberResult {
    inner: Rc<RefCell<NumberInner>>,
}

#[derive(Debug, Clone, Default)]
struct NumberInner {
    result: Option<u32>,
    task: Option<Waker>,
}

impl Future for NumberResult {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.borrow_mut();

        if inner.result.is_some() {
            let v = inner.result.unwrap();
            return Poll::Ready(v);
        }

        inner.task = Some(cx.waker().clone());
        Poll::Pending
    }
}

impl NumberResult {
    fn default() -> Self {
        NumberResult {
            inner: Rc::new(RefCell::new(NumberInner::default())),
        }
    }
}

// async func
pub fn loda(bytes: &[u8]) -> NumberResult {
    let result = NumberResult::default();
    let mut inner = result.inner.borrow_mut();

    load_callback(bytes, |result: u32| {
        inner.result = Some(result);

        let task_op = inner.task.as_ref();
        if task_op.is_some() {
            task_op.unwrap().wake_by_ref();
        };
    });

    result.clone()
}

/// sync get contract
pub fn get_by_id(id: i32) -> i32 {
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _get_contract_by_id(id: i32) -> i32;
    }

    unsafe { _get_contract_by_id(id) }
}
