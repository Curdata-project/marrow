use core::cell::{RefCell, Cell};
use alloc::collections::VecDeque;
use alloc::rc::Rc;

struct QueueState {
    //task队列
    tasks: RefCell<VecDeque<Rc<crate::task::Task>>>,
    //可变属性
    is_spinning: Cell<bool>,
}

impl QueueState {
    fn run_all(&self) {
        //false就panic
        debug_assert!(self.is_spinning.get());

        //循环跑队列
        loop {
            let task = match self.tasks.borrow_mut().pop_front() {
                Some(task) => task,
                None => break,
            };
            task.run();
        }
        //空了就不给跑了
        self.is_spinning.set(false);
    }
}

pub struct Queue {
    state: Rc<QueueState>,
}

unsafe impl Sync for Queue {}

impl Queue {
    pub(crate) fn push_task(&self, task: Rc<crate::task::Task>) {
        self.state.tasks.borrow_mut().push_back(task);

        if !self.state.is_spinning.replace(true) {
            self.state.run_all();
        }
    }
}

impl Queue {
    fn new() -> Self {
        let state = Rc::new(QueueState {
            is_spinning: Cell::new(false),
            tasks: RefCell::new(VecDeque::new()),
        });

        Self {
            state,
        }
    }
}




lazy_static!{
    // pub(crate) static QUEUE: Queue = Queue::new();
    pub static ref QUEUE: Queue = Queue::new();
}

//全局队列
//no_std下无法使用
// thread_local! {
//     pub(crate) static QUEUE: Queue = Queue::new();
// }