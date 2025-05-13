use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::env;
use std::time::Instant;
use data_structures::{BlockingQueue, lockfree_queue};

trait Queue<T>: Send + Sync {
    fn enqueue(&self, item: T);
    fn dequeue(&self) -> Option<T>;
}

fn parse_args() -> (usize, usize, usize, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Uso: {} <producers> <consumers> <items_per_producer> <mode>", args[0]);
        std::process::exit(1);
    }

    let producers = args[1].parse().unwrap();
    let consumers = args[2].parse().unwrap();
    let items = args[3].parse().unwrap();
    let mode = args[4].clone();

    (producers, consumers, items, mode)
}

fn main() {
    let (num_producers, num_consumers, items_per_producer, mode) = parse_args();
    let total_items = num_producers * items_per_producer;

    let start = Instant::now();

    match mode.as_str() {
        "blocking" => run_test::<blocking_queue::BlockingQueue<usize>>(num_producers, num_consumers, items_per_producer),
        "lockfree" => run_test::<lockfree_queue::LockFreeQueue<usize>>(num_producers, num_consumers, items_per_producer),
        _ => {
            eprintln!("Modo desconocido: usa 'blocking' o 'lockfree'");
            std::process::exit(1);
        }
    }

    let duration = start.elapsed();
    println!("Tiempo total: {:?}", duration);
}

fn run_test<Q: Queue<usize> + 'static>(
    num_producers: usize,
    num_consumers: usize,
    items_per_producer: usize,
) {
    let queue = Arc::new(Q::new());
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

    println!("Consumidos: {}", consumed.load(Ordering::Relaxed));
}
