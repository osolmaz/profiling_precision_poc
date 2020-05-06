use std::time::Instant;

use criterion::black_box;

const N_ITER: usize = 1_000_000;

fn main() {
    let mut durations = Vec::new();

    for _i in 0..N_ITER {
        let now = Instant::now();
        black_box(0);
        durations.push(now.elapsed().as_nanos());
    }

    let tmp: u128 = durations.iter().sum();
    let mean_duration_1: f64 = tmp as f64 / N_ITER as f64;

    //
    let now = Instant::now();

    for _i in 0..N_ITER {
        black_box(0);
    }

    let total_duration = now.elapsed().as_nanos();
    let mean_duration_2: f64 = total_duration as f64 / N_ITER as f64;

    println!(
        "Mean runtime, calls interleaved with logging: {} ns",
        mean_duration_1
    );
    println!(
        "Mean runtime, calls NOT interleaved with logging: {} ns",
        mean_duration_2
    );
    println!(
        "Difference: {}, {}%",
        mean_duration_2 - mean_duration_1,
        100. * (mean_duration_2 - mean_duration_1) / mean_duration_1
    );
}
