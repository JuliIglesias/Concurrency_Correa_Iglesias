use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::env;
use std::time::Instant;
use data_structures::{Queue, blocking_queue, lock_free_queue};

fn parse_args() -> (usize, usize, usize) {
    let args: Vec<String> = env::args().collect();

    let mut producers = None;
    let mut consumers = None;
    let mut items = None;

    let mut iter = args.iter().skip(1); // Saltar el primer argumento (nombre del programa)
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--producers" => producers = iter.next().and_then(|s| s.parse().ok()),
            "--consumers" => consumers = iter.next().and_then(|s| s.parse().ok()),
            "--items" => items = iter.next().and_then(|s| s.parse().ok()),
            _ => {}
        }
    }

    if let (Some(p), Some(c), Some(i)) = (producers, consumers, items) {
        (p, c, i)
    } else {
        eprintln!("Uso: cargo run -- --producers <N> --consumers <N> --items <N>");
        std::process::exit(1);
    }
}

fn main() {
    let (num_producers, num_consumers, items_per_producer) = parse_args();

    println!("--- Blocking Queue ---");
    let start_blocking = Instant::now();
    let queue = Arc::new(blocking_queue::BlockingQueue::new());
    run_test(queue, num_producers, num_consumers, items_per_producer);
    println!("Tiempo: {:?}\n", start_blocking.elapsed());

    println!("--- Lock-Free Queue ---");
    let start_lock_free = Instant::now();
    let queue = Arc::new(lock_free_queue::LockFreeQueue::new());
    run_test(queue, num_producers, num_consumers, items_per_producer);
    println!("Tiempo: {:?}", start_lock_free.elapsed());
}

fn run_test(
    queue: Arc<dyn Queue<usize>>,
    num_producers: usize,
    num_consumers: usize,
    items_per_producer: usize,
) {
    let consumed = Arc::new(AtomicUsize::new(0));
    let total = num_producers * items_per_producer;

    let mut handles = vec![];

    // productores
    for i in 0..num_producers {
        let q = Arc::clone(&queue);
        handles.push(thread::spawn(move || {
            for j in 0..items_per_producer {
                q.enqueue(i * items_per_producer + j);
            }
        }));
    }

    // consumidores
    for _ in 0..num_consumers {
        let q = Arc::clone(&queue);
        let c = Arc::clone(&consumed);
        handles.push(thread::spawn(move || {
            while c.load(Ordering::Relaxed) < total {
                if let Some(_) = q.dequeue() {
                    c.fetch_add(1, Ordering::Relaxed);
                }
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Total consumidos: {}", consumed.load(Ordering::Relaxed));
}