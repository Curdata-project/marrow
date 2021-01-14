use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;
use core::ffi::c_void;
use core::future::Future;
use core::option::Option::Some;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

unsafe extern "C" fn hook<F>(user_data: *mut c_void, ptr: *const u8, size: usize)
where
    F: FnMut(*const u8, usize),
{
    (*(user_data as *mut F))(ptr, size)
}

fn http_request_callback<F>(bytes: &[u8], mut f: F)
where
    F: FnMut(*const u8, usize),
{
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _http_request_callback(
            ptr: *const u8,
            size: usize,
            //这里定义的是回调函数，传下去的hook指针和hook的数据指针，最后在里边执行
            cb: unsafe extern "C" fn(*mut c_void, *const u8, usize),
            user_data: *mut c_void,
        );
    }

    let user_data = &mut f as *mut _ as *mut c_void;

    // 调用提供的C-ABI接口
    unsafe {
        _http_request_callback(bytes.as_ptr(), bytes.len(), hook::<F>, user_data);
    };
}

#[derive(Debug, Clone)]
pub struct HttpRequestResult {
    inner: Rc<RefCell<Inner>>,
}

#[derive(Debug, Clone, Default)]
struct Inner {
    task: Option<Waker>,
    v: Option<Vec<u8>>,
}

impl HttpRequestResult {
    pub fn default() -> Self {
        HttpRequestResult {
            inner: Rc::new(RefCell::new(Default::default())),
        }
    }
}

impl Future for HttpRequestResult {
    type Output = alloc::vec::Vec<u8>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.borrow_mut();

        if inner.v.is_some() {
            return Poll::Ready(inner.v.clone().unwrap());
        }

        inner.task = Some(cx.waker().clone());
        Poll::Pending
    }
}

pub fn http_request(bytes: &[u8]) -> HttpRequestResult {
    let result = HttpRequestResult::default();
    let mut inner = result.inner.borrow_mut();

    http_request_callback(bytes, move |ptr: *const u8, size: usize| {
        let v = unsafe { alloc::slice::from_raw_parts(ptr, size).to_vec() };

        inner.v = Some(v);

        let task_op = inner.task.as_ref();
        if task_op.is_some() {
            task_op.unwrap().wake_by_ref();
        };
    });

    result
}

#[no_mangle]
pub extern "C" fn call_http_request_callback_fn(
    ptr: *const u8,
    size: usize,
    cb: unsafe extern "C" fn(*mut c_void, *const u8, usize),
    user_data: *mut c_void,
) {
    unsafe { cb(user_data, ptr, size) }
}
