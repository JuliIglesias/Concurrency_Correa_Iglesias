use std::sync::Arc;
use std::thread;
use philosophers_may_3rd::Table;
use philosophers_may_3rd::Philosopher;

fn main() {
    let table_for_five = Arc::new(Table::new(5));

    let philosophers = vec![
        Philosopher::new(0, Arc::clone(&table_for_five), 5),
        Philosopher::new(1, Arc::clone(&table_for_five), 5),
        Philosopher::new(2, Arc::clone(&table_for_five), 5),
        Philosopher::new(3, Arc::clone(&table_for_five), 5),
        Philosopher::new(4, Arc::clone(&table_for_five), 5),
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|philosopher| {
            thread::spawn(move || {
                loop {
                    philosopher.eat();
                    philosopher.think();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
