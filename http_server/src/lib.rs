use std::time::Instant;

pub fn leibniz_approximation(n: u64) -> (f64, f64) {
    let now = Instant::now();
    let mut result: f64 = 0.0;

    for i in 0..=n {
        result += leibniz_term(i);
    }

    let total_duration: f64 = now.elapsed().as_secs_f64();

    (result, total_duration)
}

fn leibniz_term(n: u64) -> f64 {
    let numerator: f64 = 4.0 * if n % 2 == 0 { 1.0 } else { -1.0 };
    let dividend: f64 = 2.0 * n as f64 + 1.0;

    numerator / dividend
}
