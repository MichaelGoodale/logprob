use logprob::{LogProb, LogSumExp};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

fn get_big_vector(n: u32) -> Vec<LogProb<f64>> {
    let mut rng = ChaCha8Rng::seed_from_u64(1);
    let n_float: f64 = n.into();
    (0..n)
        .map(|_| LogProb::from_raw_prob(rng.gen::<f64>() / n_float).unwrap())
        .collect()
}

fn get_big_vector_overflow(n: u32) -> Vec<LogProb<f64>> {
    let mut rng = ChaCha8Rng::seed_from_u64(1);
    (0..n)
        .map(|_| LogProb::from_raw_prob(rng.gen::<f64>()).unwrap())
        .collect()
}

const SIZES: &[u32] = &[0, 1, 2, 5, 10, 20, 50, 100, 1000, 10_000];

#[divan::bench(args = SIZES)]
fn allocate_log_exp(n: u32) -> LogProb<f64> {
    divan::black_box(get_big_vector(n).into_iter())
        .log_sum_exp()
        .unwrap()
}

#[divan::bench(args = SIZES)]
fn dont_allocate_log_exp(n: u32) -> LogProb<f64> {
    divan::black_box(get_big_vector(n).into_iter())
        .log_sum_exp_no_alloc()
        .unwrap()
}

#[divan::bench(args = SIZES)]
fn allocate_log_exp_clamped(n: u32) -> LogProb<f64> {
    divan::black_box(get_big_vector_overflow(n).into_iter()).log_sum_exp_clamped()
}

#[divan::bench(args = SIZES)]
fn dont_allocate_log_exp_clamped(n: u32) -> LogProb<f64> {
    divan::black_box(get_big_vector_overflow(n).into_iter()).log_sum_exp_clamped_no_alloc()
}
