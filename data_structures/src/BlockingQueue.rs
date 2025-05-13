use std::collections::VecDeque;
use std::sync::{Mutex, Condvar};

pub struct BlockingQueue<T> {
    queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
}

impl<T> BlockingQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        }
    }

    pub fn enqueue(&self, value: T) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(value);
        self.condvar.notify_one(); // despierta a un consumidor
    }

    pub fn dequeue(&self) -> T {
        let mut q = self.queue.lock().unwrap();
        loop {
            if let Some(val) = q.pop_front() {
                return val;
            }
            q = self.condvar.wait(q).unwrap();
        }
    }
}
