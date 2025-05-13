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
        self.condvar.notify_all(); // despierta a todos los threads
    }

    fn dequeue(&self) -> Option<T> {
        let mut q = self.queue.lock().unwrap();

        while q.len() == 0 {
            // espera a que haya elementos en la cola
            q = self.condvar.wait(q).unwrap();
        }

        q.pop_front()
    }
}
