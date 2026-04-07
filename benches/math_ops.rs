use core::f64;
use divan::Bencher;
use logprob::LogProb;
use num_traits::Float;

fn main() {
    divan::main();
}
const N: usize = 100;

fn p_and_q<T: Float + From<u16>>() -> (Vec<LogProb<T>>, Vec<LogProb<T>>) {
    let log_p: Vec<_> = (0..N)
        .map(|i| LogProb::new(-(<T as From<u16>>::from((i + 2) as u16))).unwrap())
        .collect();
    let log_q: Vec<_> = (0..N)
        .map(|i| LogProb::new(-(<T as From<u16>>::from(i as u16))).unwrap())
        .collect();
    (log_p, log_q)
}

#[divan::bench(types=[f64,f32])]
fn subtract_op<T: Float + From<u16> + Sync>(bencher: Bencher) {
    let (p, q) = p_and_q::<T>();
    bencher.bench(|| {
        for (x, y) in p.iter().zip(&q) {
            _ = divan::black_box(*x) - divan::black_box(*y);
        }
    });
}

#[divan::bench(types=[f64,f32])]
fn unchecked_sub<T: Float + From<u16> + Sync>(bencher: Bencher) {
    let (p, q) = p_and_q::<T>();
    bencher.bench(|| {
        for (x, y) in p.iter().zip(&q) {
            unsafe {
                _ = divan::black_box(x).unchecked_sub(*divan::black_box(y));
            }
        }
    });
}

#[divan::bench(types=[f64,f32])]
fn saturating_sub<T: Float + From<u16> + Sync>(bencher: Bencher) {
    let (p, q) = p_and_q::<T>();
    bencher.bench(|| {
        for (x, y) in p.iter().zip(&q) {
            _ = divan::black_box(x).saturating_sub(*divan::black_box(y));
        }
    });
}

#[divan::bench(types=[f64,f32])]
fn try_sub<T: Float + From<u16> + Sync>(bencher: Bencher) {
    let (p, q) = p_and_q::<T>();
    bencher.bench(|| {
        for (x, y) in p.iter().zip(&q) {
            _ = divan::black_box(x).try_sub(*divan::black_box(y)).unwrap();
        }
    });
}

#[divan::bench(types=[f64,f32])]
fn checked_sub<T: Float + From<u16> + Sync>(bencher: Bencher) {
    let (p, q) = p_and_q::<T>();
    bencher.bench(|| {
        for (x, y) in p.iter().zip(&q) {
            _ = divan::black_box(x)
                .checked_sub(*divan::black_box(y))
                .unwrap();
        }
    });
}
