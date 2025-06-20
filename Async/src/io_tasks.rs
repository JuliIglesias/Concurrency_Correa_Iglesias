//! Simulación de tareas de I/O para TP7

use std::thread;
use std::time::Duration;
use tokio::{self, time as tokio_time};

/// Simula tareas de I/O usando threads estándar
pub fn simulate_io_threads(tasks: usize, millis: u64) {
    let mut handles = Vec::with_capacity(tasks);
    for _ in 0..tasks {
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(millis));
        }));
    }
    for h in handles {
        let _ = h.join();
    }
}

/// Simula tareas de I/O usando async/await con Tokio
pub async fn simulate_io_async(tasks: usize, millis: u64) {
    let mut handles = Vec::with_capacity(tasks);
    for _ in 0..tasks {
        handles.push(tokio::spawn(async move {
            tokio_time::sleep(Duration::from_millis(millis)).await;
        }));
    }
    for h in handles {
        let _ = h.await;
    }
}
