use std::thread;
use tokio::task;

/// Calcula una porción de la serie de Leibniz (función auxiliar sugerida)
pub fn leibniz_pi_partial(start: usize, count: usize) -> f64 {
    (start..start + count)
        .map(|k| {
            let k = k as f64;
            (-1.0f64).powf(k) / (2.0 * k + 1.0)
        })
        .sum::<f64>() * 4.0
}

/// Cálculo concurrente de Pi usando threads estándar
pub fn calc_pi_threads(tasks: usize, terms: usize) -> f64 {
    let chunk = terms / tasks;
    let mut handles = Vec::with_capacity(tasks);
    for i in 0..tasks {
        let start = i * chunk;
        let count = if i == tasks - 1 { terms - start } else { chunk };
        handles.push(thread::spawn(move || leibniz_pi_partial(start, count)));
    }
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

/// Cálculo concurrente de Pi usando async/await con Tokio
pub async fn calc_pi_async(tasks: usize, terms: usize) -> f64 {
    let chunk = terms / tasks;
    let mut handles = Vec::with_capacity(tasks);
    for i in 0..tasks {
        let start = i * chunk;
        let count = if i == tasks - 1 { terms - start } else { chunk };
        handles.push(task::spawn_blocking(move || leibniz_pi_partial(start, count)));
    }
    let mut sum = 0.0;
    for h in handles {
        sum += h.await.unwrap();
    }
    sum
}
