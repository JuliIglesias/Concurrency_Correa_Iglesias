use std::collections::VecDeque;
use std::sync::{Mutex, Condvar};
use crate::Queue;

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
}

impl<T: Send> Queue<T> for BlockingQueue<T> {
    fn enqueue(&self, value: T) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(value);
        self.condvar.notify_one(); // despierta a un consumidor
    }

    fn dequeue(&self) -> Option<T> {
        let mut q = self.queue.lock().unwrap();
        loop {
            if let Some(val) = q.pop_front() {
                return Some(val);
            }
            q = self.condvar.wait(q).unwrap();
        }
    }
}
