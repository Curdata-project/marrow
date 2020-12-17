use core::cell::{RefCell, Cell};
use alloc::collections::VecDeque;
use alloc::rc::Rc;
use alloc::boxed::Box;
use core::future::Future;

struct Queue {
    //task队列
    tasks: RefCell<VecDeque<Rc<crate::task::Task>>>,
    //可变属性
    is_spinning: Cell<bool>,
}

impl Queue {
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

pub struct Runtime {
    queue: Queue,
}


impl Runtime {
    pub fn new() -> Self {
        let queue = Queue {
            is_spinning: Cell::new(false),
            tasks: RefCell::new(VecDeque::new()),
        };

        Self {
            queue,
        }
    }

    pub fn spawn<F>(self,future: F) where F: Future<Output = ()> + 'static,
    {
        crate::task::Task::spawn(Box::pin(future),self);
    }

    pub(crate) fn push_task(&self, task: Rc<crate::task::Task>) {
        self.queue.tasks.borrow_mut().push_back(task);

        if !self.queue.is_spinning.replace(true) { self.queue.run_all() }
    }
}


