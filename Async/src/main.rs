mod io_tasks;
mod pi_calc;

use clap::{Parser, ValueEnum};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Modo de ejecución: thread o async
    #[arg(value_enum)]
    mode: Mode,
    /// Tipo de tarea: io o pi
    #[arg(value_enum)]
    task: TaskType,
    /// Número de tareas concurrentes
    #[arg(short, long, default_value_t = 10)]
    tasks: usize,
    /// Cantidad de términos para el cálculo de Pi (solo para task=pi)
    #[arg(short, long, default_value_t = 10000)]
    terms: usize,
    /// Milisegundos de espera por tarea (solo para task=io)
    #[arg(short, long, default_value_t = 100)]
    millis: u64,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode {
    Thread,
    Async,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum TaskType {
    Io,
    Pi,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let start = Instant::now();
    match (args.mode, args.task) {
        (Mode::Thread, TaskType::Io) => {
            io_tasks::simulate_io_threads(args.tasks, args.millis);
        }
        (Mode::Async, TaskType::Io) => {
            io_tasks::simulate_io_async(args.tasks, args.millis).await;
        }
        (Mode::Thread, TaskType::Pi) => {
            let pi = pi_calc::calc_pi_threads(args.tasks, args.terms);
            println!("Pi ≈ {}", pi);
        }
        (Mode::Async, TaskType::Pi) => {
            let pi = pi_calc::calc_pi_async(args.tasks, args.terms).await;
            println!("Pi ≈ {}", pi);
        }
    }
    let elapsed = start.elapsed();
    println!("Tiempo de ejecución: {:.2?}", elapsed);
}

