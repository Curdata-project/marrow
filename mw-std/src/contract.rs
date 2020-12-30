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

unsafe extern "C" fn hook_ptr_size<F>(user_data: *mut c_void, ptr: *const u8, size: usize)
where
    F: FnMut(*const u8, usize),
{
    //这里将闭包的数据指针强转为函数指针，并传入参数
    (*(user_data as *mut F))(ptr, size)
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

pub fn list_callback<F>(mut f: F)
where
    F: FnMut(*const u8, usize),
{
    // 外部C-ABI接口
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _list_contract_callback(
            cb: unsafe extern "C" fn(*mut c_void, *const u8, usize),
            user_data: *mut c_void,
        );
    }

    let user_data = &mut f as *mut _ as *mut c_void;

    // 调用提供的C-ABI接口
    unsafe {
        _list_contract_callback(hook_ptr_size::<F>, user_data);
    };
}

pub fn run_callback<F>(bytes: &[u8], mut f: F)
where
    F: FnMut(*const u8, usize),
{
    // 外部C-ABI接口
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _run_contract_callback(
            ptr: *const u8,
            size: usize,
            cb: unsafe extern "C" fn(*mut c_void, *const u8, usize),
            user_data: *mut c_void,
        );
    }

    let user_data = &mut f as *mut _ as *mut c_void;

    // 调用提供的C-ABI接口
    unsafe {
        _run_contract_callback(bytes.as_ptr(), bytes.len(), hook_ptr_size::<F>, user_data);
    };
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

#[derive(Debug, Clone)]
pub struct OtherResult {
    inner: Rc<RefCell<OtherInner>>,
}

#[derive(Debug, Clone, Default)]
struct OtherInner {
    ptr: Option<*const u8>,
    size: Option<usize>,
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

impl Future for OtherResult {
    type Output = alloc::vec::Vec<u8>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.borrow_mut();

        if inner.ptr.is_some() {
            let v = unsafe {
                alloc::slice::from_raw_parts(inner.ptr.unwrap(), inner.size.unwrap()).to_vec()
            };
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

impl OtherResult {
    fn default() -> Self {
        OtherResult {
            inner: Rc::new(RefCell::new(OtherInner::default())),
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

/// get list constract
pub fn list() -> OtherResult {
    let result = OtherResult::default();
    let mut inner = result.inner.borrow_mut();

    list_callback(|ptr: *const u8, size: usize| {
        inner.ptr = Some(ptr);
        inner.size = Some(size);

        let task_op = inner.task.as_ref();
        if task_op.is_some() {
            task_op.unwrap().wake_by_ref();
        };
    });

    result.clone()
}

pub fn run(bytes: &[u8]) -> OtherResult {
    let result = OtherResult::default();
    let mut inner = result.inner.borrow_mut();

    run_callback(bytes, |ptr: *const u8, size: usize| {
        inner.ptr = Some(ptr);
        inner.size = Some(size);

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

#[no_mangle]
pub extern "C" fn call_number_callback_fn(
    result: i32,
    cb: unsafe extern "C" fn(*mut c_void, i32),
    user_data: *mut c_void,
) {
    unsafe { cb(user_data, result) }
}

#[no_mangle]
pub extern "C" fn call_other_callback_fn(
    ptr: *const u8,
    size: usize,
    cb: unsafe extern "C" fn(*mut c_void, *const u8, usize),
    user_data: *mut c_void,
) {
    unsafe { cb(user_data, ptr, size) }
}
