//! execute sql

use alloc::rc::Rc;
use core::cell::RefCell;
use core::ffi::c_void;
use core::future::Future;
use core::option::Option::Some;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

///
unsafe extern "C" fn hook_ptr_size<F>(user_data: *mut c_void, ptr: *const u8, size: usize)
where
    F: FnMut(*const u8, usize),
{
    //这里将闭包的数据指针强转为函数指针，并传入参数
    (*(user_data as *mut F))(ptr, size)
}

unsafe extern "C" fn hook_number<F>(user_data: *mut c_void, result: i32)
where
    F: FnMut(i32),
{
    (*(user_data as *mut F))(result)
}

/// judgment table exists or not
pub fn sql_operate_callback<F>(bytes:&[u8], mut f: F)
where
    F: FnMut(i32),
{
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _sql_operate_callback(
            ptr: *const u8,
            size: usize,
            cb: unsafe extern "C" fn(*mut c_void, i32),
            user_data: *mut c_void,
        );
    }
    let user_data = &mut f as *mut _ as *mut c_void;

    unsafe {
        _sql_operate_callback(bytes.as_ptr(), bytes.len(), hook_number::<F>, user_data);
    }
}

/// 封装调用的js接口，用来create table,update,delete,modify
pub fn sql_run_callback<F>(bytes: &[u8], mut f: F)
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

    // let bytes = s.as_bytes();

    // 调用提供的C-ABI接口
    unsafe {
        _sql_run_callback(bytes.as_ptr(), bytes.len(), hook_ptr_size::<F>, user_data);
    };
}

/// 查询用接口，
pub fn sql_query_callback<F>(bytes:&[u8], mut f: F)
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

    // let bytes = s.as_bytes();

    // 调用提供的C-ABI接口
    unsafe {
        _sql_query_callback(bytes.as_ptr(), bytes.len(), hook_ptr_size::<F>, user_data);
    };
}

#[derive(Debug, Clone)]
pub struct RunSqlResult {
    inner: Rc<RefCell<RunInner>>,
}

#[derive(Debug, Clone, Default)]
struct RunInner {
    ptr: Option<*const u8>,
    size: Option<usize>,
    task: Option<Waker>,
}

#[derive(Debug, Clone)]
pub struct OperateSqlResult {
    inner: Rc<RefCell<OperateInner>>,
}

#[derive(Debug, Clone, Default)]
struct OperateInner {
    result: Option<i32>,
    task: Option<Waker>,
}

impl OperateSqlResult {
    pub fn default() -> Self {
        OperateSqlResult {
            inner: Rc::new(RefCell::new(Default::default())),
        }
    }
}

impl RunSqlResult {
    pub fn default() -> Self {
        RunSqlResult {
            inner: Rc::new(RefCell::new(Default::default())),
        }
    }
}

impl Future for RunSqlResult {
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

impl Future for OperateSqlResult {
    type Output = i32;

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

/// ty:0 update/create/modify
/// ty:1 query
pub fn sql_execute(bytes: &[u8], ty: u8) -> RunSqlResult {
    let result = RunSqlResult::default();
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
        0 => sql_run_callback(bytes, closure),
        1 => sql_query_callback(bytes, closure),
        _ => (),
    };

    result.clone()
}

pub fn sql_table_exist(bytes:&[u8]) -> OperateSqlResult {
    let result = OperateSqlResult::default();
    let mut inner = result.inner.borrow_mut();

    sql_operate_callback(bytes, move |r: i32| {
        inner.result = Some(r);

        let task_op = inner.task.as_ref();
        if task_op.is_some() {
            task_op.unwrap().wake_by_ref();
        };
    });

    result.clone()
}

#[no_mangle]
pub extern "C" fn call_sql_operate_callback_fn(
    result: i32,
    cb: unsafe extern "C" fn(*mut c_void, i32),
    user_data: *mut c_void,
) {
    unsafe { cb(user_data, result) }
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
