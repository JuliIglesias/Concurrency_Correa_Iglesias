mod io_tasks;
mod pi_calc;

use std::env;
use std::time::Instant;

struct Args {
    mode: String,
    task: String,
    number_of_tasks: usize,
    terms: usize,
    millis: u64,
}

fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(format!("Use: {} [thread|async] [io|pi] [options]", args[0]));
    }

    // Obtenemos directamente los valores como strings
    let mode = args[1].to_lowercase();
    let task = args[2].to_lowercase();

    // Validación básica
    if mode != "thread" && mode != "async" {
        return Err("Mode must be 'thread' or 'async'".to_string());
    }

    if task != "io" && task != "pi" {
        return Err("Task type must be 'io' ro 'pi'.".to_string());
    }

    // Valores por defecto
    let mut number_of_tasks = 10;
    let mut terms = 10000;
    let mut millis = 100;

    // Parsear opciones adicionales
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "-n" | "--number-of-tasks" => {
                if i + 1 < args.len() {
                    i += 1;
                    number_of_tasks = args[i]
                        .parse::<usize>()
                        .map_err(|_| "Number of tasks must be a positive integer.".to_string())?;
                }
            }
            "-t" | "--terms" => {
                if i + 1 < args.len() {
                    i += 1;
                    terms = args[i]
                        .parse::<usize>()
                        .map_err(|_| "Number of terms must be a positive integer.".to_string())?;
                }
            }
            "-m" | "--millis" => {
                if i + 1 < args.len() {
                    i += 1;
                    millis = args[i]
                        .parse::<u64>()
                        .map_err(|_| "Milliseconds must be a positive integer.".to_string())?;
                }
            }
            _ => return Err(format!("Unknown option: {}", args[i])),
        }
        i += 1;
    }

    Ok(Args {
        mode,
        task,
        number_of_tasks,
        terms,
        millis,
    })
}

#[tokio::main]
async fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("Error: {}", error);
            eprintln!("Use: cargo run [thread|async] [io|pi] [options]");
            eprintln!("Options:");
            eprintln!("  -n, --number-of-tasks N    Number of concurrent (default: 10)");
            eprintln!("  -t, --terms N              Number of terms for Leibniz terms (default: 10000)");
            eprintln!("  -m, --millis N             Milliseconds of wait for IO tasks (default: 100)");
            std::process::exit(1);
        }
    };

    let start = Instant::now();
    match (args.mode.as_str(), args.task.as_str()) {
        ("thread", "io") => {
            io_tasks::simulate_io_threads(args.number_of_tasks, args.millis);
        }
        ("async", "io") => {
            io_tasks::simulate_io_async(args.number_of_tasks, args.millis).await;
        }
        ("thread", "pi") => {
            let pi = pi_calc::calc_pi_threads(args.number_of_tasks, args.terms);
            println!("Pi ≈ {}", pi);
        }
        ("async", "pi") => {
            let pi = pi_calc::calc_pi_async(args.number_of_tasks, args.terms).await;
            println!("Pi ≈ {}", pi);
        }
        _ => {
            eprintln!("Invalid arguments");
            std::process::exit(1);
        }
    }
    let elapsed = start.elapsed();
    println!("Execution time: {:.2?}", elapsed);
}
