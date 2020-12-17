use core::cell::{RefCell, Cell};
use core::pin::Pin;
use alloc::boxed::Box;
use core::future::Future;
use alloc::rc::Rc;
use core::task::{Waker, RawWaker, RawWakerVTable, Poll, Context};
use core::mem::ManuallyDrop;
use crate::runtime::Runtime;

pub struct Task {
    inner: RefCell<Option<Inner>>,

    is_queued: Cell<bool>,

    runtime:RefCell<Runtime>
}

struct Inner {
    future: Pin<Box<dyn Future<Output = ()> + 'static>>,
    waker: Waker,
}


impl Task {
    pub fn spawn(future: Pin<Box<dyn Future<Output = ()> + 'static>>, runtime: Runtime) {
        //构建task
        let this = Rc::new(Self {
            inner: RefCell::new(None),
            is_queued: Cell::new(false),
            runtime: RefCell::new(runtime),
        });

        //构建wake
        //通过将task转换成RawWaker，再把RawWaker转换成waker
        let waker = unsafe { Waker::from_raw(Task::into_raw_waker(Rc::clone(&this))) };

        //保存future和wake到task中，对应
        *this.inner.borrow_mut() = Some(Inner { future, waker });

        //唤醒
        Task::wake_by_ref(&this);
    }


    //入队
    fn wake_by_ref(this: &Rc<Self>) {
        //如果为true，就返回
        if this.is_queued.replace(true) {
            return;
        }

       let runtime = this.runtime.borrow_mut();
        runtime.push_task(Rc::clone(this))
    }

    //waker是RawWaker的包装，调用的wake其实就是RawWaker的虚拟表中的定义wake
    //自定义了一系列唤醒的行为
    //raw_wake raw_wake_by_ref都是入队
    unsafe fn into_raw_waker(this: Rc<Self>) -> RawWaker {

        unsafe fn raw_clone(ptr: *const ()) -> RawWaker {
            let ptr = ManuallyDrop::new(Rc::from_raw(ptr as *const Task));
            Task::into_raw_waker((*ptr).clone())
        }

        unsafe fn raw_wake(ptr: *const ()) {
            let ptr = Rc::from_raw(ptr as *const Task);
            Task::wake_by_ref(&ptr);
        }

        unsafe fn raw_wake_by_ref(ptr: *const ()) {
            let ptr = ManuallyDrop::new(Rc::from_raw(ptr as *const Task));
            Task::wake_by_ref(&ptr);
        }

        unsafe fn raw_drop(ptr: *const ()) {
            drop(Rc::from_raw(ptr as *const Task));
        }

        //虚拟函数表指针，用于定义RawWaker的行为
        //
        const VTABLE: RawWakerVTable =
            RawWakerVTable::new(raw_clone, raw_wake, raw_wake_by_ref, raw_drop);

        RawWaker::new(Rc::into_raw(this) as *const (), &VTABLE)
    }

    pub fn run(&self) {
        let mut borrow = self.inner.borrow_mut();

        //判断inner是否不为空
        let inner = match borrow.as_mut() {
            Some(inner) => {
                inner
            },
            None => return,
        };

        //可以入队标志
        self.is_queued.set(false);

        //创建轮询机，注入waker
        let poll = {
            let mut cx = Context::from_waker(&inner.waker);
            inner.future.as_mut().poll(&mut cx)
        };

        //如果返回的是空，修改值
        if let Poll::Ready(()) = poll {
            *borrow = None;
        }
    }
}

