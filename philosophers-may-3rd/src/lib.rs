use std::sync::{Arc, Condvar, Mutex};
use std::thread::sleep;
use std::time::Duration;

struct Table {
    forks: Vec<bool>,
}

impl Table {
    fn new(forks_amount: u32) -> Table {
        if forks_amount < 2 {
            panic!("Amount must be greater than 1");
        }

        Table {
            forks: vec![false; forks_amount as usize],
        }
    }

    fn change_forks_state(&mut self, left_fork_index: u32, right_fork_index: u32) {
        if left_fork_index < self.forks.len() as u32 || right_fork_index > self.forks.len() as u32 {
            panic!("Forks indexes out of bounds");
        }

        self.forks[left_fork_index as usize] = !self.forks[right_fork_index as usize];
        self.forks[right_fork_index as usize] = !self.forks[left_fork_index as usize];
    }
}

struct Philosopher {
    id: u32,
    table: Arc<(Mutex<Table>, Condvar)>,
    left_fork_index: u32,
    right_fork_index: u32,
}

impl Philosopher {
    fn new(id: u32, table: Arc<(Mutex<Table>, Condvar)>, forks_length: u32) -> Philosopher {
        let left_fork_index = if id == 0 { forks_length - 1 } else { id - 1 };
        let right_fork_index = if id + 1 == forks_length { 0 } else { id + 1 };

        Philosopher {
            id,
            table,
            left_fork_index,
            right_fork_index
        }
    }

    fn think() {
        sleep(Duration::new(5, 0));
    }

    fn eat(&mut self) {
        let locked_table = self.table.0.lock().unwrap();

        if !locked_table.forks[self.left_fork_index as usize] || !locked_table.forks[self.right_fork_index as usize] {
            self.table.1.wait(locked_table);
        }
        
        
    }
}
