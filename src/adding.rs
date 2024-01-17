use std::borrow::Borrow;

use super::LogProb;
use crate::errors::ProbabilitiesSumToGreaterThanOne;
pub use num_traits::Float;

pub trait Ln2: Sized {
    const LN_2: Self;
    const ZERO: Self;
    const NEG_INFINITY: Self;
}

impl Ln2 for f32 {
    const LN_2: Self = std::f32::consts::LN_2;
    const ZERO: Self = 0.0;
    const NEG_INFINITY: Self = std::f32::NEG_INFINITY;
}
impl Ln2 for f64 {
    const LN_2: Self = std::f64::consts::LN_2;
    const ZERO: Self = 0.0;
    const NEG_INFINITY: Self = std::f64::NEG_INFINITY;
}

impl<T: Float + Ln2> LogProb<T> {
    fn add_log_prob_internal(x: T, y: T) -> T {
        if x > y {
            x + (y - x).exp().ln_1p()
        } else if x < y {
            y + (x - y).exp().ln_1p()
        } else {
            x + T::LN_2
        }
    }
    /// Add log probabilities safely.
    #[inline(always)]
    pub fn add_log_prob(
        &self,
        y: LogProb<T>,
    ) -> Result<LogProb<T>, ProbabilitiesSumToGreaterThanOne> {
        let y = y.0;
        Ok(LogProb::new(Self::add_log_prob_internal(self.0, y))?)
    }

    /// Add log probabilities but clamping at 0.0.
    #[inline(always)]
    pub fn add_log_prob_clamped(&self, y: LogProb<T>) -> LogProb<T> {
        match self.add_log_prob(y) {
            Ok(x) => x,
            Err(_err) => LogProb(T::ZERO),
        }
    }

    /// Add log probabilities and return a float (so can be greater than 0.0).
    #[inline(always)]
    pub fn add_log_prob_float(&self, y: LogProb<T>) -> T {
        Self::add_log_prob_internal(self.0, y.0)
    }
}

fn log_sum_exp_allocate_inner<
    T: Float + Ln2 + std::iter::Sum,
    L: Borrow<LogProb<T>>,
    I: Iterator<Item = L>,
>(
    iterable: I,
) -> T {
    let mut max = LogProb(T::NEG_INFINITY);
    let v: Vec<_> = iterable
        .map(|x| {
            let x = x.borrow();
            if x > &max {
                max = *x;
            }
            *x
        })
        .collect();
    log_sum_exp_inner(&v, max)
}

fn log_sum_exp_inner<T: Float + std::iter::Sum + Ln2, L: Borrow<LogProb<T>>>(
    val: &[L],
    max: LogProb<T>,
) -> T {
    val.iter()
        .map(|x| (x.borrow().0 - max.0).exp())
        .sum::<T>()
        .ln()
        + max.0
}

pub fn log_sum_exp<T: Float + std::iter::Sum + Ln2, L: Borrow<LogProb<T>> + Ord>(
    val: &[L],
) -> Result<LogProb<T>, ProbabilitiesSumToGreaterThanOne> {
    match val.iter().max() {
        Some(max) => Ok(LogProb::new(log_sum_exp_inner(val, *max.borrow()))?),
        None => Ok(LogProb(T::NEG_INFINITY)),
    }
}

pub fn log_sum_exp_clamped<T: Float + std::iter::Sum + Ln2, L: Borrow<LogProb<T>> + Ord>(
    val: &[L],
) -> LogProb<T> {
    match val.iter().max() {
        Some(max) => match LogProb::new(log_sum_exp_inner(val, *max.borrow())) {
            Ok(x) => x,
            Err(_) => LogProb(T::ZERO),
        },
        None => LogProb(T::NEG_INFINITY),
    }
}

pub fn log_sum_exp_float<T: Float + std::iter::Sum + Ln2, L: Borrow<LogProb<T>> + Ord>(
    val: &[L],
) -> T {
    match val.iter().max() {
        Some(max) => log_sum_exp_inner(val, *max.borrow()),
        None => T::NEG_INFINITY,
    }
}

pub trait LogSumExp: Iterator {
    fn log_sum_exp<T: Float + Ln2, L: Borrow<LogProb<T>>>(
        mut self,
    ) -> Result<LogProb<T>, ProbabilitiesSumToGreaterThanOne>
    where
        Self: Sized,
        Self: Iterator<Item = L>,
    {
        match self.next() {
            Some(first) => self.try_fold(*first.borrow(), |acc, x| (*x.borrow()).add_log_prob(acc)),
            None => Ok(LogProb(T::ZERO)),
        }
    }

    fn log_sum_exp_float<T: Float + Ln2, L: Borrow<LogProb<T>>>(mut self) -> T
    where
        Self: Sized,
        Self: Iterator<Item = L>,
    {
        match self.next() {
            Some(x) => {
                let first: T = (*x.borrow()).into_inner();
                self.fold(first, |acc, x| {
                    LogProb::<T>::add_log_prob_internal(x.borrow().0, acc)
                })
            }
            None => T::NEG_INFINITY,
        }
    }

    fn log_sum_exp_clamped<T: Float + Ln2, L: Borrow<LogProb<T>>>(mut self) -> LogProb<T>
    where
        Self: Sized,
        Self: Iterator<Item = L>,
    {
        match self.next() {
            Some(first) => {
                match self.try_fold(*first.borrow(), |acc, x| x.borrow().add_log_prob(acc)) {
                    Ok(x) => x,
                    Err(_) => LogProb(T::ZERO),
                }
            }
            None => LogProb(T::NEG_INFINITY),
        }
    }

    fn log_sum_exp_allocate<T: Float + Ln2 + std::iter::Sum, L: Borrow<LogProb<T>>>(
        self,
    ) -> Result<LogProb<T>, ProbabilitiesSumToGreaterThanOne>
    where
        Self: Sized,
        Self: Iterator<Item = L>,
    {
        Ok(LogProb::new(log_sum_exp_allocate_inner(self))?)
    }

    fn log_sum_exp_clamped_allocate<T: Float + Ln2 + std::iter::Sum, L: Borrow<LogProb<T>>>(
        self,
    ) -> LogProb<T>
    where
        Self: Sized,
        Self: Iterator<Item = L>,
    {
        match LogProb::new(log_sum_exp_allocate_inner(self)) {
            Ok(x) => x,
            Err(_) => LogProb(T::ZERO),
        }
    }

    fn log_sum_exp_float_allocate<T: Float + Ln2 + std::iter::Sum, L: Borrow<LogProb<T>>>(self) -> T
    where
        Self: Sized,
        Self: Iterator<Item = L>,
    {
        log_sum_exp_allocate_inner(self)
    }
}

impl<I: ?Sized> LogSumExp for I where I: Iterator {}
