use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

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

    pub fn enqueue(&self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value: Some(value),
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        loop {
            let cur_tail = self.tail.load(Ordering::Acquire);
            let cur_tail_next = unsafe { (*cur_tail).next.load(Ordering::Acquire) };

            if (cur_tail == self.tail.load(Ordering::Acquire)) {
                if unsafe { (*cur_tail).next.compare_exchange(ptr::null_mut(), new_node, Ordering::AcqRel, Ordering::Relaxed) }.is_ok() {
                    let _ = self.tail.compare_exchange(cur_tail, new_node, Ordering::AcqRel, Ordering::Relaxed);
                    return;
                }
            } else {
                let _ = self.tail.compare_exchange(cur_tail, cur_tail_next, Ordering::AcqRel, Ordering::Relaxed);
            }
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };

            if next.is_null() {
                return None; // cola vacía
            }

            if head == tail {
                let _ = self.tail.compare_exchange(tail, next, Ordering::AcqRel, Ordering::Relaxed);
            } else {
                if self.head.compare_exchange(head, next, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
                    let value = unsafe {
                        let value = (*next).value.take();
                        Box::from_raw(head); // liberar el nodo anterior
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
