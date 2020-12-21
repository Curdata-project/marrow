//! execute sql

use alloc::rc::Rc;
use core::cell::RefCell;
use core::ffi::c_void;
use core::future::Future;
use core::option::Option::Some;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

///
unsafe extern "C" fn hook<F>(user_data: *mut c_void, ptr: *const u8, size: usize)
where
    F: FnMut(*const u8, usize),
{
    //这里将闭包的数据指针强转为函数指针，并传入参数
    (*(user_data as *mut F))(ptr, size)
}

/// 封装调用的js接口，用来create table,update,delete,modify
pub fn sql_run_callback<F>(s: &str, mut f: F)
where
    F: FnMut(*const u8, usize),
{
    // 外部C-ABI接口
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _sql_run_callback(
            ptr: *const u8,
            size: usize,
            //这里定义的是回调函数，传下去的hook指针和hook的数据指针，最后在里边执行
            cb: unsafe extern "C" fn(*mut c_void, *const u8, usize),
            user_data: *mut c_void,
        );
    }

    let user_data = &mut f as *mut _ as *mut c_void;

    let bytes = s.as_bytes();

    // 调用提供的C-ABI接口
    unsafe {
        _sql_run_callback(bytes.as_ptr(), bytes.len(), hook::<F>, user_data);
    };
}

/// 查询用接口，
pub fn sql_query_callback<F>(s: &str, mut f: F)
where
    F: FnMut(*const u8, usize),
{
    // 外部C-ABI接口
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _sql_query_callback(
            ptr: *const u8,
            size: usize,
            //这里定义的是回调函数，传下去的hook指针和hook的数据指针，最后在里边执行
            cb: unsafe extern "C" fn(*mut c_void, *const u8, usize),
            user_data: *mut c_void,
        );
    }

    let user_data = &mut f as *mut _ as *mut c_void;

    let bytes = s.as_bytes();

    // 调用提供的C-ABI接口
    unsafe {
        _sql_query_callback(bytes.as_ptr(), bytes.len(), hook::<F>, user_data);
    };
}

#[derive(Debug, Clone)]
pub struct SqlResult {
    inner: Rc<RefCell<Inner>>,
}

#[derive(Debug, Clone, Default)]
struct Inner {
    ptr: Option<*const u8>,
    size: Option<usize>,
    task: Option<Waker>,
}

impl SqlResult {
    pub fn default() -> Self {
        SqlResult {
            inner: Rc::new(RefCell::new(Default::default())),
        }
    }
}

impl Future for SqlResult {
    type Output = alloc::vec::Vec<u8>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.borrow_mut();

        if inner.ptr.is_some() && inner.size.is_some() {
            let v = unsafe {
                alloc::slice::from_raw_parts(inner.ptr.unwrap(), inner.size.unwrap()).to_vec()
            };
            return Poll::Ready(v);
        }

        inner.task = Some(cx.waker().clone());
        Poll::Pending
    }
}

/// ty:0 update/create/modify
/// ty:1 query
pub fn sql_execute(s: &str, ty: u8) -> SqlResult {
    let result = SqlResult::default();
    let mut inner = result.inner.borrow_mut();

    let closure = move |ptr: *const u8, size: usize| {
        inner.ptr = Some(ptr);
        inner.size = Some(size);

        let task_op = inner.task.as_ref();
        if task_op.is_some() {
            task_op.unwrap().wake_by_ref();
        };
    };

    match ty {
        0 => sql_run_callback(s, closure),
        1 => sql_query_callback(s, closure),
        _ => (),
    };

    result.clone()
}

#[no_mangle]
pub extern "C" fn call_sql_callback_fn(
    ptr: *const u8,
    size: usize,
    cb: unsafe extern "C" fn(*mut c_void, *const u8, usize),
    user_data: *mut c_void,
) {
    unsafe { cb(user_data, ptr, size) }
}
