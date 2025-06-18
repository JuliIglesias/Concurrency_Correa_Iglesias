pub mod blocking_queue;
pub mod lock_free_queue;

pub trait Queue<T>: Send + Sync {
    fn enqueue(&self, item: T);
    fn dequeue(&self) -> Option<T>;
}
