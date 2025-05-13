use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use crate::Queue;

struct Node<T> {
    value: Option<T>,
    next: AtomicPtr<Node<T>>,
}

pub struct LockFreeQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> LockFreeQueue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node {
            value: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }
}

impl<T: Send> Queue<T> for LockFreeQueue<T> {
    fn enqueue(&self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value: Some(value),
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        loop {
            let cur_tail = self.tail.load(Ordering::Acquire);
            let cur_tail_next = unsafe { (*cur_tail).next.load(Ordering::Acquire) };

            if cur_tail == self.tail.load(Ordering::Acquire) {
                if !cur_tail_next.is_null() {
                    let _ = self.tail.compare_exchange(cur_tail, cur_tail_next, Ordering::AcqRel, Ordering::Relaxed);
                } else if unsafe { (*cur_tail).next.compare_exchange(ptr::null_mut(), new_node, Ordering::AcqRel, Ordering::Relaxed) }.is_ok() {
                    let _ = self.tail.compare_exchange(cur_tail, new_node, Ordering::AcqRel, Ordering::Relaxed);
                    return;
                }
            }
        }
    }

    fn dequeue(&self) -> Option<T> {
        loop {
            let cur_head = self.head.load(Ordering::Acquire);
            let cur_tail = self.tail.load(Ordering::Acquire);
            let cur_head_next = unsafe { (*cur_head).next.load(Ordering::Acquire) };

            if cur_head_next.is_null() {
                return None; // cola vacía
            }

            if cur_head == cur_tail {
                let _ = self.tail.compare_exchange(cur_tail, cur_head_next, Ordering::AcqRel, Ordering::Relaxed);
            } else {
                if self.head.compare_exchange(cur_head, cur_head_next, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
                    let value = unsafe {
                        let value = (*cur_head_next).value.take();
                        drop(Box::from_raw(cur_head)); // liberar el nodo anterior
                        value
                    };
                    return value;
                }
            }
        }
    }
}

impl<T> Drop for LockFreeQueue<T> {
    fn drop(&mut self) {
        unsafe {
            let mut curr = self.head.load(Ordering::Relaxed);
            while !curr.is_null() {
                let next = (*curr).next.load(Ordering::Relaxed);
                drop(Box::from_raw(curr));
                curr = next;
            }
        }
    }
}
