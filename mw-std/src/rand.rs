//! generate random number

use alloc::rc::Rc;
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

pub fn gen_rand32_callback<F>(mut f: F)
where
    F: FnMut(*const u8, usize),
{
    // 外部C-ABI接口
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _gen_rand32_callback(
            //这里定义的是回调函数，传下去的hook指针和hook的数据指针，最后在里边执行
            cb: unsafe extern "C" fn(*mut c_void, *const u8, usize),
            user_data: *mut c_void,
        );
    }

    let user_data = &mut f as *mut _ as *mut c_void;

    // 调用提供的C-ABI接口
    unsafe {
        _gen_rand32_callback(hook::<F>, user_data);
    };
}

#[derive(Debug, Clone)]
pub struct Rand32Result {
    inner: Rc<RefCell<Inner>>,
}

#[derive(Debug, Clone, Default)]
struct Inner {
    ptr: Option<*const u8>,
    size: Option<usize>,
    task: Option<Waker>,
}

impl Rand32Result {
    pub fn default() -> Self {
        Rand32Result {
            inner: Rc::new(RefCell::new(Default::default())),
        }
    }
}

impl Future for Rand32Result {
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

pub fn gen_rand32() -> Rand32Result {
    let result = Rand32Result::default();
    let mut inner = result.inner.borrow_mut();

    gen_rand32_callback(move |ptr: *const u8, size: usize| {
        inner.ptr = Some(ptr);
        inner.size = Some(size);

        let task_op = inner.task.as_ref();
        if task_op.is_some() {
            task_op.unwrap().wake_by_ref();
        };
    });

    result
}

#[no_mangle]
pub extern "C" fn call_gen_rand32_callback_fn(
    ptr: *const u8,
    size: usize,
    cb: unsafe extern "C" fn(*mut c_void, *const u8, usize),
    user_data: *mut c_void,
) {
    unsafe { cb(user_data, ptr, size) }
}
