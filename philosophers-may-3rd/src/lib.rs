use std::sync::{Arc, Condvar, Mutex};
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

pub struct Table {
    forks: Mutex<Vec<bool>>,
    cond: Condvar,
}

impl Table {
    pub fn new(forks_amount: u32) -> Table {
        if forks_amount < 2 {
            panic!("Amount must be greater than 1");
        }

        Table {
            forks: Mutex::from(vec![true; forks_amount as usize]),
            cond: Condvar::new(),
        }
    }

    pub fn pick_up_forks(&self, left_fork_index: u32, right_fork_index: u32) {
        let mut locked_forks_availability = self.forks.lock().unwrap();
        
        while !locked_forks_availability[left_fork_index as usize] || !locked_forks_availability[right_fork_index as usize] {
            println!("Either fork {} or {} is unavailable, philosopher {} can not eat", left_fork_index, right_fork_index, left_fork_index);
            locked_forks_availability = self.cond.wait(locked_forks_availability).unwrap();
        }

        locked_forks_availability[left_fork_index as usize] = false;
        locked_forks_availability[right_fork_index as usize] = false;
    }

    pub fn drop_forks(&self, left_fork_index: u32, right_fork_index: u32) {
        let mut locked_forks_availability = self.forks.lock().unwrap();

        locked_forks_availability[left_fork_index as usize] = true;
        locked_forks_availability[right_fork_index as usize] = true;
        
        self.cond.notify_all();
    }
}

pub struct Philosopher {
    id: u32,
    table: Arc<Table>,
    left_fork_index: u32,
    right_fork_index: u32,
}

impl Philosopher {
    pub fn new(id: u32, table: Arc<Table>, forks_length: u32) -> Philosopher {
        let left_fork_index = id;
        let right_fork_index = if id + 1 == forks_length { 0 } else { id + 1 };

        Philosopher {
            id,
            table,
            left_fork_index,
            right_fork_index
        }
    }

    pub fn think(&self) {
        let mut rng = rand::rng();
        let sleep_duration = rng.random_range(1..=10); // Genera un número aleatorio entre 1 y 10 segundos
        
        println!("philosopher {} thinks for {} seconds", self.id, sleep_duration);
        sleep(Duration::new(sleep_duration, 0));
    }

    pub fn eat(&self) {
        self.table.pick_up_forks(self.left_fork_index, self.right_fork_index);

        let mut rng = rand::rng();
        let sleep_duration = rng.random_range(1..=10); // Genera un número aleatorio entre 1 y 10 segundos
        
        println!("philosopher {} eats for {} seconds", self.id, sleep_duration);
        sleep(Duration::new(sleep_duration, 0));
        
        self.table.drop_forks(self.left_fork_index, self.right_fork_index);
    }
}
