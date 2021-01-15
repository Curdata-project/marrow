//! load byte code

use alloc::rc::Rc;
use core::cell::RefCell;
use core::ffi::c_void;
use core::future::Future;
use core::option::Option::Some;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

unsafe extern "C" fn hook_number<F>(user_data: *mut c_void, result: i32)
where
    F: FnMut(i32),
{
    (*(user_data as *mut F))(result)
}

pub fn load_callback<F>(bytes: &[u8], mut f: F)
where
    F: FnMut(i32),
{
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _load_callback(
            ptr: *const u8,
            size: usize,
            cb: unsafe extern "C" fn(*mut c_void, i32),
            user_data: *mut c_void,
        );
    }
    let user_data = &mut f as *mut _ as *mut c_void;

    unsafe {
        _load_callback(bytes.as_ptr(), bytes.len(), hook_number::<F>, user_data);
    }
}

#[derive(Debug, Clone)]
pub struct LoadResult {
    inner: Rc<RefCell<Inner>>,
}
#[derive(Debug, Clone, Default)]
struct Inner {
    result: Option<i32>,
    task: Option<Waker>,
}

impl LoadResult {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Inner::default())),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Instance {
    pub handle: Option<i32>,
}

impl Instance {
    pub fn run(self, args: &[u8]) -> i32 {
        #[link(wasm_import_module = "wstd")]
        extern "C" {
            fn _load_run(index: i32, ptr: *const u8, size: usize) -> i32;
        };

        unsafe { _load_run(self.handle.unwrap(), args.as_ptr(), args.len()) }
    }
}

impl Future for LoadResult {
    type Output = Instance;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.borrow_mut();

        if inner.result.is_some() {
            let v = inner.result.unwrap();
            return Poll::Ready(Instance { handle: Some(v) });
        }

        inner.task = Some(cx.waker().clone());
        Poll::Pending
    }
}

pub fn loader(bytes: &[u8]) -> LoadResult {
    let result = LoadResult::default();
    let mut inner = result.inner.borrow_mut();

    load_callback(bytes, move |result: i32| {
        inner.result = Some(result);

        let task_op = inner.task.as_ref();
        if task_op.is_some() {
            task_op.unwrap().wake_by_ref();
        };
    });

    result.clone()
}

#[no_mangle]
pub extern "C" fn call_loader_callback_fn(
    result: i32,
    cb: unsafe extern "C" fn(*mut c_void, i32),
    user_data: *mut c_void,
) {
    unsafe { cb(user_data, result) }
}
